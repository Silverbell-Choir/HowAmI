use super::Collection;
use crate::model::{DeviceRecord, Section};
use serde_json::Value;
use std::{collections::BTreeMap, process::Command};

pub fn collect() -> Result<Collection, Box<dyn std::error::Error>> {
    let mut collection = Collection::default();

    let mut os_section = Section::new("System");
    if let Ok(output) = Command::new("sw_vers").output() {
        if output.status.success() {
            let mut record = DeviceRecord::new("macOS");
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if let Some((key, value)) = line.split_once(':') {
                    record.insert(key.trim(), value.trim());
                }
            }
            os_section.push(record);
        }
    }
    if let Ok(output) = Command::new("uname").args(["-a"]).output() {
        if output.status.success() {
            os_section.push(
                DeviceRecord::new("Kernel")
                    .with_field("uname", String::from_utf8_lossy(&output.stdout).trim()),
            );
        }
    }
    collection.sections.push(os_section);

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

    let output = Command::new("system_profiler")
        .args(data_types)
        .args(["-json", "-detailLevel", "full"])
        .output()?;

    if output.status.success() {
        let value: Value = serde_json::from_slice(&output.stdout)?;
        if let Some(object) = value.as_object() {
            for (key, value) in object {
                let mut section = Section::new(friendly_section_name(key));
                append_top_level_records(value, &mut section);
                collection.sections.push(section);
            }
        }
    } else {
        collection.warnings.push(format!(
            "system_profiler failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    if let Ok(output) = Command::new("systemextensionsctl").arg("list").output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !text.is_empty() {
                collection.sections.push(Section {
                    name: "System Extensions".into(),
                    records: vec![DeviceRecord::new("systemextensionsctl").with_field("Raw", text)],
                });
            }
        }
    }

    Ok(collection)
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
            fields.insert(prefix.to_string(), v.to_string());
        }
        Value::Number(v) => {
            fields.insert(prefix.to_string(), v.to_string());
        }
        Value::String(v) => {
            if !v.trim().is_empty() {
                fields.insert(prefix.to_string(), v.clone());
            }
        }
        Value::Array(items) => {
            if items.iter().all(|item| !item.is_object() && !item.is_array()) {
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
                if !joined.is_empty() {
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
