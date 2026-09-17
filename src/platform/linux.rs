use super::Collection;
use crate::{
    command, edid,
    model::{DeviceRecord, Section},
};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

pub fn collect() -> Result<Collection, Box<dyn std::error::Error>> {
    let mut collection = Collection::default();

    collection.sections.push(collect_system(&mut collection.warnings));
    collection.sections.push(collect_cpu());
    collection.sections.push(collect_dmi());
    collection.sections.push(collect_memory());

    match collect_lsblk() {
        Ok(section) => collection.sections.push(section),
        Err(error) => collection
            .warnings
            .push(format!("lsblk storage collection failed: {error}")),
    }

    collection.sections.push(collect_pci_sysfs());
    collection.sections.push(collect_usb_sysfs());
    collection.sections.push(collect_network());
    collection.sections.push(collect_displays());

    match collect_optional_command("PCI Devices (lspci)", "lspci", &["-nnk"], 30) {
        Ok(Some(section)) => collection.sections.push(section),
        Ok(None) => {}
        Err(error) => collection.warnings.push(error),
    }
    match collect_optional_command("USB Devices (lsusb)", "lsusb", &["-v"], 30) {
        Ok(Some(section)) => collection.sections.push(section),
        Ok(None) => {}
        Err(error) => collection.warnings.push(error),
    }
    match collect_dmidecode_memory() {
        Ok(Some(section)) => collection.sections.push(section),
        Ok(None) => {}
        Err(error) => collection.warnings.push(error),
    }

    Ok(collection)
}

fn collect_system(warnings: &mut Vec<String>) -> Section {
    let mut section = Section::new("System");
    let mut os = DeviceRecord::new("Linux");

    if let Ok(text) = fs::read_to_string("/etc/os-release") {
        for line in text.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                os.insert(key, value.trim_matches('"'));
            }
        }
    }

    if let Some(uname) = command::linux_program("uname") {
        let mut process = Command::new(uname);
        process.args(["-srmo"]);
        match command::run_capture(&mut process, Duration::from_secs(10)) {
            Ok(output) if output.success() => {
                os.insert("Kernel", String::from_utf8_lossy(&output.stdout).trim())
            }
            Ok(output) if output.timed_out => {
                warnings.push("uname timed out after 10 seconds".into())
            }
            Ok(output) => warnings.push(format!(
                "uname failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )),
            Err(error) => warnings.push(format!("uname could not run: {error}")),
        }
    }

    if let Ok(hostname) = fs::read_to_string("/etc/hostname") {
        os.insert("Hostname", hostname.trim());
    }
    if let Ok(version) = fs::read_to_string("/proc/version") {
        os.insert("KernelBuild", version.trim());
    }

    section.push(os);
    section
}

fn collect_cpu() -> Section {
    let mut section = Section::new("CPU");
    let text = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let blocks: Vec<&str> = text.split("\n\n").filter(|b| !b.trim().is_empty()).collect();

    let mut summary = DeviceRecord::new("CPU Summary");
    let logical = std::thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(blocks.len());
    summary.insert("LogicalProcessors", logical.to_string());

    if let Some(first) = blocks.first() {
        for line in first.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "model name" | "vendor_id" | "cpu family" | "model" | "stepping"
                    | "microcode" | "cpu MHz" | "cache size" | "flags" | "Features"
                    | "Hardware" | "Revision" | "Serial" => summary.insert(key, value),
                    _ => {}
                }
            }
        }
    }

    section.push(summary);
    section
}

fn collect_dmi() -> Section {
    let mut section = Section::new("Mainboard / BIOS / DMI");
    let root = Path::new("/sys/class/dmi/id");
    let mut record = DeviceRecord::new("DMI");

    let fields = [
        ("SystemVendor", "sys_vendor"),
        ("ProductName", "product_name"),
        ("ProductVersion", "product_version"),
        ("ProductSerial", "product_serial"),
        ("ProductUUID", "product_uuid"),
        ("BoardVendor", "board_vendor"),
        ("BoardName", "board_name"),
        ("BoardVersion", "board_version"),
        ("BoardSerial", "board_serial"),
        ("ChassisVendor", "chassis_vendor"),
        ("ChassisType", "chassis_type"),
        ("ChassisSerial", "chassis_serial"),
        ("BIOSVendor", "bios_vendor"),
        ("BIOSVersion", "bios_version"),
        ("BIOSDate", "bios_date"),
    ];

    for (label, file) in fields {
        if let Some(value) = read_trimmed(root.join(file)) {
            record.insert(label, value);
        }
    }

    section.push(record);
    section
}

fn collect_memory() -> Section {
    let mut section = Section::new("Memory");
    let mut record = DeviceRecord::new("Memory Summary");
    if let Ok(text) = fs::read_to_string("/proc/meminfo") {
        for line in text.lines() {
            if let Some((key, value)) = line.split_once(':') {
                if matches!(
                    key,
                    "MemTotal" | "MemFree" | "MemAvailable" | "SwapTotal" | "SwapFree"
                ) {
                    record.insert(key, value.trim());
                }
            }
        }
    }
    section.push(record);
    section
}

fn collect_lsblk() -> Result<Section, Box<dyn std::error::Error>> {
    let lsblk = command::linux_program("lsblk").ok_or("lsblk was not found in trusted paths")?;
    let columns = "NAME,KNAME,TYPE,SIZE,MODEL,VENDOR,SERIAL,REV,TRAN,ROTA,FSTYPE,FSVER,LABEL,UUID,MOUNTPOINT";
    let mut process = Command::new(lsblk);
    process.args(["-J", "-b", "-o", columns]);
    let output = command::run_capture(&mut process, Duration::from_secs(20))?;

    if output.timed_out {
        return Err("lsblk timed out after 20 seconds".into());
    }
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr)
            .trim()
            .to_string()
            .into());
    }

    let json: Value = serde_json::from_slice(&output.stdout)?;
    let mut section = Section::new("Storage");
    if let Some(devices) = json.get("blockdevices").and_then(Value::as_array) {
        for device in devices {
            append_block_device(device, "", &mut section);
        }
    }
    Ok(section)
}

fn append_block_device(value: &Value, parent: &str, section: &mut Section) {
    let Some(object) = value.as_object() else {
        return;
    };
    let name = object
        .get("name")
        .and_then(value_to_string)
        .unwrap_or_else(|| "block-device".into());
    let label = if parent.is_empty() {
        name.clone()
    } else {
        format!("{parent}/{name}")
    };

    let mut record = DeviceRecord::new(label.clone());
    for (key, value) in object {
        if key == "children" {
            continue;
        }
        if let Some(text) = value_to_string(value) {
            record.insert(key, text);
        }
    }
    section.push(record);

    if let Some(children) = object.get("children").and_then(Value::as_array) {
        for child in children {
            append_block_device(child, &label, section);
        }
    }
}

fn collect_pci_sysfs() -> Section {
    let mut section = Section::new("PCI Devices");
    let root = Path::new("/sys/bus/pci/devices");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let label = entry.file_name().to_string_lossy().to_string();
        let mut record = DeviceRecord::new(label);
        for field in [
            "vendor",
            "device",
            "subsystem_vendor",
            "subsystem_device",
            "class",
            "revision",
            "numa_node",
            "modalias",
        ] {
            if let Some(value) = read_trimmed(path.join(field)) {
                record.insert(field, value);
            }
        }
        if let Some(driver) = symlink_file_name(path.join("driver")) {
            record.insert("driver", driver.clone());
            if let Some(version) =
                read_trimmed(Path::new("/sys/module").join(&driver).join("version"))
            {
                record.insert("driver_version", version);
            }
        }
        section.push(record);
    }
    section
}

fn collect_usb_sysfs() -> Section {
    let mut section = Section::new("USB Devices");
    let root = Path::new("/sys/bus/usb/devices");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.join("idVendor").exists() || !path.join("idProduct").exists() {
            continue;
        }
        let product = read_trimmed(path.join("product"));
        let fallback = entry.file_name().to_string_lossy().to_string();
        let mut record = DeviceRecord::new(product.unwrap_or(fallback));
        let fields = [
            "manufacturer",
            "product",
            "serial",
            "idVendor",
            "idProduct",
            "bcdDevice",
            "speed",
            "busnum",
            "devnum",
            "version",
            "bDeviceClass",
        ];
        for field in fields {
            if let Some(value) = read_trimmed(path.join(field)) {
                record.insert(field, value);
            }
        }
        if let Some(driver) = symlink_file_name(path.join("driver")) {
            record.insert("driver", driver);
        }
        section.push(record);
    }
    section
}

fn collect_network() -> Section {
    let mut section = Section::new("Network Adapters");
    let root = Path::new("/sys/class/net");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let mut record = DeviceRecord::new(name);
        for field in ["address", "operstate", "mtu", "speed", "duplex", "type"] {
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
        section.push(record);
    }
    section
}

fn collect_displays() -> Section {
    let mut section = Section::new("Displays / DRM");
    let root = Path::new("/sys/class/drm");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let connector_name = entry.file_name().to_string_lossy().to_string();
        let Some(status) = read_trimmed(path.join("status")) else {
            continue;
        };
        if status != "connected" {
            continue;
        }

        let mut record = DeviceRecord::new(connector_name);
        record.insert("status", status);
        if let Some(modes) = read_trimmed(path.join("modes")) {
            record.insert("modes", modes.replace('\n', ", "));
        }
        if let Ok(raw_edid) = fs::read(path.join("edid")) {
            if !raw_edid.is_empty() {
                record.insert("edid_bytes", raw_edid.len().to_string());
                if let Some(fields) = edid::parse_edid(&raw_edid) {
                    for (key, value) in fields {
                        record.insert(key, value);
                    }
                }
            }
        }
        section.push(record);
    }
    section
}

fn collect_optional_command(
    section_name: &str,
    command_name: &str,
    args: &[&str],
    timeout_seconds: u64,
) -> Result<Option<Section>, String> {
    let Some(program) = command::linux_program(command_name) else {
        return Ok(None);
    };

    let mut process = Command::new(program);
    process.args(args);
    let output = command::run_capture(&mut process, Duration::from_secs(timeout_seconds))
        .map_err(|error| format!("{command_name} could not run: {error}"))?;

    if output.timed_out {
        return Err(format!(
            "{command_name} timed out after {timeout_seconds} seconds"
        ));
    }
    if !output.status.success() {
        return Err(format!(
            "{command_name} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        return Ok(None);
    }

    Ok(Some(Section {
        name: section_name.into(),
        records: vec![DeviceRecord::new(command_name).with_field("Raw", text)],
    }))
}

fn collect_dmidecode_memory() -> Result<Option<Section>, String> {
    let Some(dmidecode) = command::linux_program("dmidecode") else {
        return Ok(None);
    };

    let mut process = Command::new(dmidecode);
    process.args(["--type", "memory"]);
    let output = command::run_capture(&mut process, Duration::from_secs(20))
        .map_err(|error| format!("dmidecode could not run: {error}"))?;

    if output.timed_out {
        return Err("dmidecode timed out after 20 seconds".into());
    }
    if !output.status.success() {
        return Err(format!(
            "dmidecode failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut section = Section::new("Memory Modules (DMI)");
    let mut index = 0usize;

    for block in text.split("\n\n") {
        if !block.contains("Memory Device") {
            continue;
        }
        index += 1;
        let mut record = DeviceRecord::new(format!("Memory Device {index}"));
        for line in block.lines().map(str::trim) {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                if !value.is_empty() && value != "Not Specified" && value != "Unknown" {
                    record.insert(key, value);
                }
            }
        }
        if !record.fields.is_empty() {
            section.push(record);
        }
    }

    Ok((!section.records.is_empty()).then_some(section))
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
            .map(|name| name.to_string_lossy().to_string())
    })
}

fn value_to_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(value) => Some(value.clone()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Number(value) => Some(value.to_string()),
        other => Some(other.to_string()),
    }
}
