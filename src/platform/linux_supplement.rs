use crate::model::{DeviceRecord, Section};
use std::{fs, path::{Path, PathBuf}};

pub fn collect() -> Vec<Section> {
    vec![
        collect_gpus(),
        collect_power_supplies(),
        collect_hwmon(),
        collect_audio(),
        collect_input_devices(),
        collect_bluetooth(),
    ]
}

fn collect_gpus() -> Section {
    let mut section = Section::new("GPU / DRM Cards");
    let root = Path::new("/sys/class/drm");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(index) = name.strip_prefix("card") else {
            continue;
        };
        if index.is_empty() || !index.chars().all(|ch| ch.is_ascii_digit()) {
            continue;
        }

        let device = entry.path().join("device");
        if !device.exists() {
            continue;
        }

        let mut record = DeviceRecord::new(name);
        for field in [
            "vendor",
            "device",
            "subsystem_vendor",
            "subsystem_device",
            "revision",
            "boot_vga",
            "modalias",
        ] {
            if let Some(value) = read_trimmed(device.join(field)) {
                record.insert(field, value);
            }
        }

        if let Some(driver) = symlink_file_name(device.join("driver")) {
            record.insert("driver", driver.clone());
            if let Some(version) = read_trimmed(Path::new("/sys/module").join(&driver).join("version")) {
                record.insert("driver_version", version);
            }
        }

        if let Some(uevent) = read_trimmed(device.join("uevent")) {
            for line in uevent.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    if matches!(key, "DRIVER" | "PCI_ID" | "PCI_SUBSYS_ID" | "PCI_SLOT_NAME") {
                        record.insert(key, value);
                    }
                }
            }
        }

        section.push(record);
    }

    section
}

fn collect_power_supplies() -> Section {
    let mut section = Section::new("Power / Battery");
    let root = Path::new("/sys/class/power_supply");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    let fields = [
        "type",
        "manufacturer",
        "model_name",
        "serial_number",
        "technology",
        "status",
        "capacity",
        "capacity_level",
        "cycle_count",
        "energy_full_design",
        "energy_full",
        "energy_now",
        "charge_full_design",
        "charge_full",
        "charge_now",
        "voltage_min_design",
        "voltage_now",
        "current_now",
        "power_now",
        "online",
    ];

    for entry in entries.flatten() {
        let path = entry.path();
        let mut record = DeviceRecord::new(entry.file_name().to_string_lossy());
        for field in fields {
            if let Some(value) = read_trimmed(path.join(field)) {
                record.insert(field, value);
            }
        }
        if !record.fields.is_empty() {
            section.push(record);
        }
    }

    section
}

fn collect_hwmon() -> Section {
    let mut section = Section::new("Hardware Sensors / hwmon");
    let root = Path::new("/sys/class/hwmon");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let label = read_trimmed(path.join("name"))
            .unwrap_or_else(|| entry.file_name().to_string_lossy().to_string());
        let mut record = DeviceRecord::new(label);

        let Ok(files) = fs::read_dir(&path) else {
            continue;
        };
        for file in files.flatten() {
            let field = file.file_name().to_string_lossy().to_string();
            if !is_sensor_field(&field) {
                continue;
            }
            if let Some(value) = read_trimmed(file.path()) {
                record.insert(field, value);
            }
        }

        if !record.fields.is_empty() {
            section.push(record);
        }
    }

    section
}

fn is_sensor_field(name: &str) -> bool {
    let prefixes = ["temp", "fan", "in", "curr", "power", "energy", "humidity", "pwm"];
    let suffixes = [
        "_input",
        "_label",
        "_min",
        "_max",
        "_crit",
        "_alarm",
        "_average",
        "_highest",
    ];
    prefixes.iter().any(|prefix| name.starts_with(prefix))
        && suffixes.iter().any(|suffix| name.ends_with(suffix))
}

fn collect_audio() -> Section {
    let mut section = Section::new("Audio / ALSA");

    if let Ok(text) = fs::read_to_string("/proc/asound/cards") {
        for block in split_alsa_cards(&text) {
            section.push(block);
        }
    }

    if let Ok(modules) = fs::read_to_string("/proc/asound/modules") {
        let text = modules.trim();
        if !text.is_empty() {
            section.push(DeviceRecord::new("ALSA Modules").with_field("Raw", text));
        }
    }

    section
}

fn split_alsa_cards(text: &str) -> Vec<DeviceRecord> {
    let mut records = Vec::new();
    let mut current: Option<DeviceRecord> = None;

    for line in text.lines() {
        let trimmed = line.trim_end();
        let leading_trimmed = trimmed.trim_start();
        let starts_card = leading_trimmed
            .split_once(' ')
            .map(|(first, _)| first.chars().all(|ch| ch.is_ascii_digit()))
            .unwrap_or(false)
            && leading_trimmed.contains('[');

        if starts_card {
            if let Some(record) = current.take() {
                records.push(record);
            }
            current = Some(
                DeviceRecord::new(leading_trimmed.to_string())
                    .with_field("Description", leading_trimmed.to_string()),
            );
        } else if let Some(record) = current.as_mut() {
            if !leading_trimmed.is_empty() {
                let prior = record.fields.get("Details").cloned().unwrap_or_default();
                let details = if prior.is_empty() {
                    leading_trimmed.to_string()
                } else {
                    format!("{prior} | {leading_trimmed}")
                };
                record.insert("Details", details);
            }
        }
    }

    if let Some(record) = current {
        records.push(record);
    }
    records
}

fn collect_input_devices() -> Section {
    let mut section = Section::new("Input Devices");
    let Ok(text) = fs::read_to_string("/proc/bus/input/devices") else {
        return section;
    };

    for (index, block) in text.split("\n\n").filter(|block| !block.trim().is_empty()).enumerate() {
        let mut record = DeviceRecord::new(format!("Input Device {}", index + 1));
        for line in block.lines() {
            let line = line.trim();
            if let Some(value) = line.strip_prefix("N: Name=") {
                let value = value.trim_matches('"');
                record.label = value.to_string();
                record.insert("Name", value);
            } else if let Some((prefix, value)) = line.split_once(':') {
                let key = match prefix {
                    "I" => "ID",
                    "P" => "PhysicalPath",
                    "S" => "SysfsPath",
                    "U" => "UniqueId",
                    "H" => "Handlers",
                    "B" => "Bitmap",
                    _ => continue,
                };
                record.insert(key, value.trim());
            }
        }
        if !record.fields.is_empty() {
            section.push(record);
        }
    }

    section
}

fn collect_bluetooth() -> Section {
    let mut section = Section::new("Bluetooth Controllers");
    let root = Path::new("/sys/class/bluetooth");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let mut record = DeviceRecord::new(entry.file_name().to_string_lossy());
        for field in ["address", "name", "manufacturer", "product", "serial"] {
            if let Some(value) = read_trimmed(path.join(field)) {
                record.insert(field, value);
            }
        }
        if let Some(driver) = symlink_file_name(path.join("device/driver")) {
            record.insert("driver", driver.clone());
            if let Some(version) = read_trimmed(Path::new("/sys/module").join(&driver).join("version")) {
                record.insert("driver_version", version);
            }
        }
        if !record.fields.is_empty() {
            section.push(record);
        }
    }

    section
}

fn read_trimmed(path: PathBuf) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

fn symlink_file_name(path: PathBuf) -> Option<String> {
    fs::read_link(path).ok().and_then(|target| {
        target.file_name().map(|name| name.to_string_lossy().to_string())
    })
}
