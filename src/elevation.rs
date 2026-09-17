use std::{env, path::Path, process::Command};

#[derive(Debug)]
pub struct ElevationState {
    pub elevated: bool,
    pub warning: Option<String>,
}

#[derive(Debug)]
pub enum ElevationOutcome {
    Continue(ElevationState),
    Relaunched,
}

pub fn ensure_elevated(
    no_elevate: bool,
    relaunch_marker: bool,
) -> Result<ElevationOutcome, Box<dyn std::error::Error>> {
    let elevated = is_elevated();
    if elevated || no_elevate {
        return Ok(ElevationOutcome::Continue(ElevationState {
            elevated,
            warning: if no_elevate && !elevated {
                Some("Elevation was disabled; some hardware data may be unavailable. / 관리자 권한 상승이 비활성화되어 일부 정보가 누락될 수 있습니다.".into())
            } else {
                None
            },
        }));
    }

    if relaunch_marker {
        return Ok(ElevationOutcome::Continue(ElevationState {
            elevated: false,
            warning: Some("Elevation did not succeed; continuing with standard-user access. / 관리자 권한 상승에 실패하여 일반 권한으로 계속합니다.".into()),
        }));
    }

    let exe = env::current_exe()?;
    match relaunch_as_admin(&exe) {
        Ok(true) => Ok(ElevationOutcome::Relaunched),
        Ok(false) => Ok(ElevationOutcome::Continue(ElevationState {
            elevated: false,
            warning: Some("Administrator/root permission was not granted; some hardware data may be unavailable. / 관리자/root 권한이 허용되지 않아 일부 정보가 누락될 수 있습니다.".into()),
        })),
        Err(error) => Ok(ElevationOutcome::Continue(ElevationState {
            elevated: false,
            warning: Some(format!(
                "Failed to request elevation: {error}. Continuing with reduced access. / 권한 상승 요청 실패: {error}. 제한된 권한으로 계속합니다."
            )),
        })),
    }
}

#[cfg(target_os = "windows")]
fn is_elevated() -> bool {
    let script = "([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)";
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output();

    output
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|text| text.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn is_elevated() -> bool {
    Command::new("id")
        .arg("-u")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|text| text.trim() == "0")
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
fn relaunch_as_admin(exe: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let path = powershell_single_quote(&exe.to_string_lossy());
    let script = format!(
        "Start-Process -FilePath '{path}' -Verb RunAs -ArgumentList '--elevated'"
    );
    let status = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &script,
        ])
        .status()?;
    Ok(status.success())
}

#[cfg(target_os = "macos")]
fn relaunch_as_admin(exe: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let command = format!("{} --elevated", shell_single_quote(&exe.to_string_lossy()));
    let script = format!(
        "do shell script \"{}\" with administrator privileges",
        applescript_double_quote(&command)
    );
    let status = Command::new("osascript").args(["-e", &script]).status()?;
    Ok(status.success())
}

#[cfg(target_os = "linux")]
fn relaunch_as_admin(exe: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    if command_exists("pkexec") {
        let status = Command::new("pkexec").arg(exe).arg("--elevated").status()?;
        return Ok(status.success());
    }

    if command_exists("sudo") {
        let status = Command::new("sudo").arg(exe).arg("--elevated").status()?;
        return Ok(status.success());
    }

    Ok(false)
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

#[cfg(target_os = "linux")]
fn command_exists(name: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {name} >/dev/null 2>&1")])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
