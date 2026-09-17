use crate::{model::SystemReport, report};
use chrono::Local;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub fn resolve_output_dir(explicit: Option<PathBuf>) -> PathBuf {
    if let Some(path) = explicit {
        return path;
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(profile) = env::var_os("USERPROFILE") {
            let desktop = PathBuf::from(profile).join("Desktop");
            if desktop.is_dir() {
                return desktop;
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = env::var_os("HOME") {
            let home = PathBuf::from(home);
            let desktop = home.join("Desktop");
            if desktop.is_dir() {
                return desktop;
            }
            return home;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(home) = env::var_os("HOME") {
            let home = PathBuf::from(home);
            if let Some(desktop) = linux_xdg_desktop(&home) {
                if desktop.is_dir() {
                    return desktop;
                }
            }
            let desktop = home.join("Desktop");
            if desktop.is_dir() {
                return desktop;
            }
            return home;
        }
    }

    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(target_os = "linux")]
fn linux_xdg_desktop(home: &Path) -> Option<PathBuf> {
    let config = fs::read_to_string(home.join(".config/user-dirs.dirs")).ok()?;
    for line in config.lines() {
        let line = line.trim();
        let Some(value) = line.strip_prefix("XDG_DESKTOP_DIR=") else {
            continue;
        };
        let value = value.trim().trim_matches('"');
        let home_text = home.to_string_lossy();
        return Some(PathBuf::from(value.replace("$HOME", &home_text)));
    }
    None
}

pub fn write_reports(
    report_data: &SystemReport,
    output_dir: &Path,
) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;

    let stamp = Local::now().format("%Y%m%d_%H%M%S");
    let txt_path = output_dir.join(format!("HowAmI_Report_{stamp}.txt"));
    let json_path = output_dir.join(format!("HowAmI_Report_{stamp}.json"));

    fs::write(&txt_path, report::render_text(report_data))?;
    fs::write(&json_path, serde_json::to_vec_pretty(report_data)?)?;

    Ok((txt_path, json_path))
}
