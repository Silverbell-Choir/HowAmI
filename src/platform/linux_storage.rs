use crate::model::{DeviceRecord, Section};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn collect() -> Vec<Section> {
    vec![collect_nvme(), collect_block_device_state()]
}

fn collect_nvme() -> Section {
    let mut section = Section::new("NVMe Controllers");
    let root = Path::new("/sys/class/nvme");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !name.starts_with("nvme") || name.contains('n') {
            continue;
        }
        let path = entry.path();
        let mut record = DeviceRecord::new(name);
        for field in [
            "model",
            "serial",
            "firmware_rev",
            "state",
            "transport",
            "address",
            "cntlid",
            "numa_node",
            "queue_count",
            "sqsize",
            "kato",
        ] {
            if let Some(value) = read_trimmed(path.join(field)) {
                record.insert(field, value);
            }
        }
        if let Some(driver) = symlink_file_name(path.join("device/driver")) {
            record.insert("driver", driver.clone());
            if let Some(version) =
                read_trimmed(Path::new("/sys/module").join(&driver).join("version"))
            {
                record.insert("driver_version", version);
            }
        }
        if !record.fields.is_empty() {
            section.push(record);
        }
    }

    section
}

fn collect_block_device_state() -> Section {
    let mut section = Section::new("Block Device State");
    let root = Path::new("/sys/block");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("loop") || name.starts_with("ram") || name.starts_with("zram") {
            continue;
        }
        let path = entry.path();
        let mut record = DeviceRecord::new(name);

        if let Some(value) = read_trimmed(path.join("size")) {
            record.insert("SizeSectors512", value.clone());
            if let Ok(sectors) = value.parse::<u64>() {
                if let Some(bytes) = sectors.checked_mul(512) {
                    record.insert("SizeBytes", bytes.to_string());
                }
            }
        }

        for (field, relative) in [
            ("ReadOnly", "ro"),
            ("Removable", "removable"),
            ("LogicalBlockSizeBytes", "queue/logical_block_size"),
            ("PhysicalBlockSizeBytes", "queue/physical_block_size"),
            ("Rotational", "queue/rotational"),
            ("Scheduler", "queue/scheduler"),
            ("Vendor", "device/vendor"),
            ("Model", "device/model"),
            ("Revision", "device/rev"),
            ("DeviceState", "device/state"),
            ("Serial", "device/serial"),
            ("WWID", "device/wwid"),
        ] {
            if let Some(value) = read_trimmed(path.join(relative)) {
                record.insert(field, value);
            }
        }

        if let Some(driver) = symlink_file_name(path.join("device/driver")) {
            record.insert("driver", driver.clone());
            if let Some(version) =
                read_trimmed(Path::new("/sys/module").join(&driver).join("version"))
            {
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
        target
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
    })
}
