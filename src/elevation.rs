use crate::command;
use std::{path::Path, process::Command, time::Duration};

pub fn is_elevated() -> bool {
    #[cfg(target_os = "windows")]
    {
        let Ok(powershell) = command::windows_powershell() else {
            return false;
        };
        let script = "([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)";
        let mut process = Command::new(powershell);
        process.args(["-NoProfile", "-NonInteractive", "-Command", script]);
        return command::run_capture(&mut process, Duration::from_secs(5))
            .ok()
            .filter(|output| output.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|text| text.trim().eq_ignore_ascii_case("true"))
            .unwrap_or(false);
    }

    #[cfg(target_os = "macos")]
    {
        let Some(id) = command::macos_program("id") else {
            return false;
        };
        let mut process = Command::new(id);
        process.arg("-u");
        return command::run_capture(&mut process, Duration::from_secs(5))
            .ok()
            .filter(|output| output.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|text| text.trim() == "0")
            .unwrap_or(false);
    }

    #[cfg(target_os = "linux")]
    {
        let Some(id) = command::linux_program("id") else {
            return false;
        };
        let mut process = Command::new(id);
        process.arg("-u");
        return command::run_capture(&mut process, Duration::from_secs(5))
            .ok()
            .filter(|output| output.success())
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|text| text.trim() == "0")
            .unwrap_or(false);
    }

    #[allow(unreachable_code)]
    false
}

pub fn run_elevated_child(
    exe: &Path,
    handoff: &Path,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        return run_windows(exe, handoff, token);
    }

    #[cfg(target_os = "macos")]
    {
        return run_macos(exe, handoff, token);
    }

    #[cfg(target_os = "linux")]
    {
        return run_linux(exe, handoff, token);
    }

    #[allow(unreachable_code)]
    Err("privilege elevation is not supported on this operating system".into())
}

#[cfg(target_os = "windows")]
fn run_windows(
    exe: &Path,
    handoff: &Path,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let powershell = command::windows_powershell()?;
    let exe = powershell_single_quote(&exe.to_string_lossy());
    let handoff = powershell_single_quote(&handoff.to_string_lossy());
    let token = powershell_single_quote(token);
    let script = format!(
        "$ErrorActionPreference='Stop'; try {{ $a = '--elevated-child \"' + '{handoff}' + '\" \"' + '{token}' + '\"'; $p = Start-Process -FilePath '{exe}' -Verb RunAs -ArgumentList $a -Wait -PassThru; exit [int]$p.ExitCode }} catch {{ exit 1 }}"
    );

    let status = Command::new(powershell)
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("UAC elevation was cancelled or the elevated collector failed ({status})").into())
    }
}

#[cfg(target_os = "macos")]
fn run_macos(
    exe: &Path,
    handoff: &Path,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let osascript = command::macos_program("osascript")
        .ok_or("trusted /usr/bin/osascript was not found")?;
    let shell_command = format!(
        "{} --elevated-child {} {}",
        shell_single_quote(&exe.to_string_lossy()),
        shell_single_quote(&handoff.to_string_lossy()),
        shell_single_quote(token)
    );
    let script = format!(
        "do shell script \"{}\" with administrator privileges",
        applescript_double_quote(&shell_command)
    );

    let status = Command::new(osascript).args(["-e", &script]).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("administrator authorization was cancelled or the elevated collector failed ({status})").into())
    }
}

#[cfg(target_os = "linux")]
fn run_linux(
    exe: &Path,
    handoff: &Path,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(pkexec) = command::linux_program("pkexec") {
        let status = Command::new(pkexec)
            .arg(exe)
            .arg("--elevated-child")
            .arg(handoff)
            .arg(token)
            .status()?;
        if status.success() {
            return Ok(());
        }
        return Err(format!("pkexec authorization was cancelled or failed ({status})").into());
    }

    if let Some(sudo) = command::linux_program("sudo") {
        let status = Command::new(sudo)
            .arg(exe)
            .arg("--elevated-child")
            .arg(handoff)
            .arg(token)
            .status()?;
        if status.success() {
            return Ok(());
        }
        return Err(format!("sudo authorization was cancelled or failed ({status})").into());
    }

    Err("neither pkexec nor sudo is available in trusted system paths".into())
}

#[cfg(target_os = "windows")]
fn powershell_single_quote(value: &str) -> String {
    value.replace('\'', "''")
}

#[cfg(target_os = "macos")]
fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[cfg(target_os = "macos")]
fn applescript_double_quote(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
