use crate::{model::SystemReport, report};
use chrono::Local;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[cfg(target_os = "windows")]
#[repr(C)]
struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

#[cfg(target_os = "windows")]
#[link(name = "shell32")]
extern "system" {
    fn SHGetKnownFolderPath(
        rfid: *const Guid,
        dw_flags: u32,
        token: *mut std::ffi::c_void,
        path: *mut *mut u16,
    ) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "ole32")]
extern "system" {
    fn CoTaskMemFree(memory: *mut std::ffi::c_void);
}

#[cfg(target_os = "windows")]
const FOLDERID_DESKTOP: Guid = Guid {
    data1: 0xB4BFCC3A,
    data2: 0xDB2C,
    data3: 0x424C,
    data4: [0xB0, 0x29, 0x7F, 0xE9, 0x9A, 0x87, 0xC6, 0x41],
};

pub fn resolve_output_dir(explicit: Option<PathBuf>) -> PathBuf {
    if let Some(path) = explicit {
        return path;
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(desktop) = windows_desktop() {
            if desktop.is_dir() {
                return desktop;
            }
        }
        if let Some(profile) = env::var_os("USERPROFILE") {
            let profile = PathBuf::from(profile);
            let desktop = profile.join("Desktop");
            if desktop.is_dir() {
                return desktop;
            }
            return profile;
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

#[cfg(target_os = "windows")]
fn windows_desktop() -> Option<PathBuf> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt, ptr, slice};

    let mut raw: *mut u16 = ptr::null_mut();
    let result = unsafe {
        SHGetKnownFolderPath(&FOLDERID_DESKTOP, 0, ptr::null_mut(), &mut raw)
    };
    if result < 0 || raw.is_null() {
        return None;
    }

    let mut len = 0usize;
    unsafe {
        while *raw.add(len) != 0 {
            len += 1;
        }
        let path = PathBuf::from(OsString::from_wide(slice::from_raw_parts(raw, len)));
        CoTaskMemFree(raw.cast());
        Some(path)
    }
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

    let stamp = Local::now().format("%Y%m%d_%H%M%S_%3f");
    let txt_path = output_dir.join(format!("HowAmI_Report_{stamp}.txt"));
    let json_path = output_dir.join(format!("HowAmI_Report_{stamp}.json"));

    fs::write(&txt_path, report::render_text(report_data))?;
    fs::write(&json_path, serde_json::to_vec_pretty(report_data)?)?;

    Ok((txt_path, json_path))
}
