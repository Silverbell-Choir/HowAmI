use std::{
    io::{self, Read},
    path::PathBuf,
    process::{Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct CapturedOutput {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub timed_out: bool,
}

impl CapturedOutput {
    pub fn success(&self) -> bool {
        !self.timed_out && self.status.success()
    }
}

pub fn run_capture(command: &mut Command, timeout: Duration) -> io::Result<CapturedOutput> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn()?;

    let stdout = child.stdout.take().ok_or_else(|| {
        io::Error::new(io::ErrorKind::Other, "failed to capture child stdout")
    })?;
    let stderr = child.stderr.take().ok_or_else(|| {
        io::Error::new(io::ErrorKind::Other, "failed to capture child stderr")
    })?;

    let stdout_reader = thread::spawn(move || read_all(stdout));
    let stderr_reader = thread::spawn(move || read_all(stderr));

    let deadline = Instant::now() + timeout;
    let (status, timed_out) = loop {
        if let Some(status) = child.try_wait()? {
            break (status, false);
        }

        if Instant::now() >= deadline {
            let _ = child.kill();
            let status = child.wait()?;
            break (status, true);
        }

        thread::sleep(Duration::from_millis(25));
    };

    Ok(CapturedOutput {
        status,
        stdout: join_reader(stdout_reader)?,
        stderr: join_reader(stderr_reader)?,
        timed_out,
    })
}

fn read_all<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn join_reader(
    handle: thread::JoinHandle<io::Result<Vec<u8>>>,
) -> io::Result<Vec<u8>> {
    handle.join().map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "command output reader thread panicked")
    })?
}

#[cfg(target_os = "windows")]
pub fn windows_powershell() -> io::Result<PathBuf> {
    let root = std::env::var_os("SystemRoot")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "SystemRoot is unavailable"))?;
    let path = root
        .join("System32")
        .join("WindowsPowerShell")
        .join("v1.0")
        .join("powershell.exe");
    require_file(path)
}

#[cfg(target_os = "macos")]
pub fn macos_program(name: &str) -> Option<PathBuf> {
    let path = match name {
        "id" => "/usr/bin/id",
        "osascript" => "/usr/bin/osascript",
        "sw_vers" => "/usr/bin/sw_vers",
        "uname" => "/usr/bin/uname",
        "system_profiler" => "/usr/sbin/system_profiler",
        "systemextensionsctl" => "/usr/bin/systemextensionsctl",
        _ => return None,
    };
    existing_file(path)
}

#[cfg(target_os = "linux")]
pub fn linux_program(name: &str) -> Option<PathBuf> {
    const TRUSTED_DIRS: &[&str] = &["/usr/bin", "/bin", "/usr/sbin", "/sbin"];
    TRUSTED_DIRS
        .iter()
        .map(|dir| std::path::Path::new(dir).join(name))
        .find(|path| path.is_file())
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn existing_file(path: &str) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    path.is_file().then_some(path)
}

#[cfg(target_os = "windows")]
fn require_file(path: PathBuf) -> io::Result<PathBuf> {
    if path.is_file() {
        Ok(path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("trusted executable not found: {}", path.display()),
        ))
    }
}
