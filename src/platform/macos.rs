use super::Collection;
use crate::{
    command,
    model::{DeviceRecord, Section},
};
use serde_json::Value;
use std::{collections::BTreeMap, process::Command, time::Duration};

pub fn collect() -> Result<Collection, Box<dyn std::error::Error>> {
    let mut collection = Collection::default();
    collection
        .sections
        .push(collect_system(&mut collection.warnings));

    let data_types = [
        "SPHardwareDataType",
        "SPDisplaysDataType",
        "SPMemoryDataType",
        "SPStorageDataType",
        "SPNVMeDataType",
        "SPAudioDataType",
        "SPUSBDataType",
        "SPNetworkDataType",
        "SPBluetoothDataType",
        "SPPowerDataType",
        "SPPCIDataType",
        "SPThunderboltDataType",
        "SPExtensionsDataType",
    ];

    let profiler = command::macos_program("system_profiler");
    if let Some(profiler) = profiler {
        for data_type in data_types {
            match collect_profiler_type(&profiler, data_type) {
                Ok(section) => collection.sections.push(section),
                Err(error) => collection
                    .warnings
                    .push(format!("system_profiler {data_type} failed: {error}")),
            }
        }
    } else {
        collection
            .warnings
            .push("trusted /usr/sbin/system_profiler was not found".into());
    }

    if let Some(tool) = command::macos_program("systemextensionsctl") {
        let mut process = Command::new(tool);
        process.arg("list");
        match command::run_capture(&mut process, Duration::from_secs(20)) {
            Ok(output) if output.timed_out => collection
                .warnings
                .push("systemextensionsctl timed out after 20 seconds".into()),
            Ok(output) if output.status.success() => {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !text.is_empty() {
                    collection.sections.push(Section {
                        name: "System Extensions".into(),
                        records: vec![
                            DeviceRecord::new("systemextensionsctl").with_field("Raw", text)
                        ],
                    });
                }
            }
            Ok(output) => collection.warnings.push(format!(
                "systemextensionsctl failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )),
            Err(error) => collection
                .warnings
                .push(format!("systemextensionsctl could not run: {error}")),
        }
    }

    Ok(collection)
}

fn collect_system(warnings: &mut Vec<String>) -> Section {
    let mut section = Section::new("System");

    if let Some(sw_vers) = command::macos_program("sw_vers") {
        let mut process = Command::new(sw_vers);
        match command::run_capture(&mut process, Duration::from_secs(10)) {
            Ok(output) if output.success() => {
                let mut record = DeviceRecord::new("macOS");
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    if let Some((key, value)) = line.split_once(':') {
                        record.insert(key.trim(), value.trim());
                    }
                }
                section.push(record);
            }
            Ok(output) if output.timed_out => {
                warnings.push("sw_vers timed out after 10 seconds".into())
            }
            Ok(output) => warnings.push(format!(
                "sw_vers failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )),
            Err(error) => warnings.push(format!("sw_vers could not run: {error}")),
        }
    }

    if let Some(uname) = command::macos_program("uname") {
        let mut process = Command::new(uname);
        process.args(["-a"]);
        match command::run_capture(&mut process, Duration::from_secs(10)) {
            Ok(output) if output.success() => section.push(
                DeviceRecord::new("Kernel")
                    .with_field("uname", String::from_utf8_lossy(&output.stdout).trim()),
            ),
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

    section
}

fn collect_profiler_type(
    profiler: &std::path::Path,
    data_type: &str,
) -> Result<Section, Box<dyn std::error::Error>> {
    let mut process = Command::new(profiler);
    process.args([data_type, "-json", "-detailLevel", "full"]);
    let output = command::run_capture(&mut process, Duration::from_secs(45))?;

    if output.timed_out {
        return Err("timed out after 45 seconds".into());
    }
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr)
            .trim()
            .to_string()
            .into());
    }

    let value: Value = serde_json::from_slice(&output.stdout)?;
    let payload = value.get(data_type).unwrap_or(&value);
    let mut section = Section::new(friendly_section_name(data_type));
    append_top_level_records(payload, &mut section);
    Ok(section)
}

fn append_top_level_records(value: &Value, section: &mut Section) {
    match value {
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                section.push(value_to_record(item, index));
            }
        }
        _ => section.push(value_to_record(value, 0)),
    }
}

fn value_to_record(value: &Value, index: usize) -> DeviceRecord {
    let label = value
        .get("_name")
        .and_then(Value::as_str)
        .or_else(|| value.get("name").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("Item {}", index + 1));

    let mut fields = BTreeMap::new();
    flatten_json("", value, &mut fields);
    fields.remove("_name");

    DeviceRecord { label, fields }
}

fn flatten_json(prefix: &str, value: &Value, fields: &mut BTreeMap<String, String>) {
    match value {
        Value::Null => {}
        Value::Bool(v) => {
            if !prefix.is_empty() {
                fields.insert(prefix.to_string(), v.to_string());
            }
        }
        Value::Number(v) => {
            if !prefix.is_empty() {
                fields.insert(prefix.to_string(), v.to_string());
            }
        }
        Value::String(v) => {
            if !prefix.is_empty() && !v.trim().is_empty() {
                fields.insert(prefix.to_string(), v.clone());
            }
        }
        Value::Array(items) => {
            if items
                .iter()
                .all(|item| !item.is_object() && !item.is_array())
            {
                let joined = items
                    .iter()
                    .filter_map(|item| match item {
                        Value::String(v) => Some(v.clone()),
                        Value::Number(v) => Some(v.to_string()),
                        Value::Bool(v) => Some(v.to_string()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                if !prefix.is_empty() && !joined.is_empty() {
                    fields.insert(prefix.to_string(), joined);
                }
            } else {
                for (index, item) in items.iter().enumerate() {
                    let child = if prefix.is_empty() {
                        format!("[{index}]")
                    } else {
                        format!("{prefix}[{index}]")
                    };
                    flatten_json(&child, item, fields);
                }
            }
        }
        Value::Object(object) => {
            for (key, value) in object {
                let child = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_json(&child, value, fields);
            }
        }
    }
}

fn friendly_section_name(key: &str) -> &str {
    match key {
        "SPHardwareDataType" => "Hardware",
        "SPDisplaysDataType" => "Displays / GPU",
        "SPMemoryDataType" => "Memory",
        "SPStorageDataType" => "Storage",
        "SPNVMeDataType" => "NVMe",
        "SPAudioDataType" => "Audio Devices",
        "SPUSBDataType" => "USB Devices",
        "SPNetworkDataType" => "Network",
        "SPBluetoothDataType" => "Bluetooth",
        "SPPowerDataType" => "Power / Battery",
        "SPPCIDataType" => "PCI Devices",
        "SPThunderboltDataType" => "Thunderbolt / USB4",
        "SPExtensionsDataType" => "Kernel Extensions",
        _ => key,
    }
}
