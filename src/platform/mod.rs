use crate::model::Section;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
mod linux_cpu;
#[cfg(target_os = "linux")]
mod linux_storage;
#[cfg(target_os = "linux")]
mod linux_supplement;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
mod windows_extra;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Collection {
    pub sections: Vec<Section>,
    pub warnings: Vec<String>,
}

pub fn collect() -> Result<Collection, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let mut collection = windows::collect()?;
        match windows_extra::collect() {
            Ok(extra) => {
                collection.sections.extend(extra.sections);
                collection.warnings.extend(extra.warnings);
            }
            Err(error) => collection
                .warnings
                .push(format!("Windows supplemental collector failed: {error}")),
        }
        return Ok(collection);
    }

    #[cfg(target_os = "macos")]
    {
        return macos::collect();
    }

    #[cfg(target_os = "linux")]
    {
        let mut collection = linux::collect()?;
        collection.sections.push(linux_cpu::collect());
        collection.sections.extend(linux_storage::collect());
        collection.sections.extend(linux_supplement::collect());
        return Ok(collection);
    }

    #[allow(unreachable_code)]
    Err("HowAmI currently supports Windows, macOS, and Linux only".into())
}
