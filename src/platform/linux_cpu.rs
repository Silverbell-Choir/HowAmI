use crate::model::{DeviceRecord, Section};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

pub fn collect() -> Section {
    let mut section = Section::new("CPU Topology / Frequency");
    let root = Path::new("/sys/devices/system/cpu");
    let Ok(entries) = fs::read_dir(root) else {
        return section;
    };

    let mut cpus = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(index) = name.strip_prefix("cpu") else {
            continue;
        };
        let Ok(index) = index.parse::<u32>() else {
            continue;
        };
        cpus.push((index, entry.path()));
    }
    cpus.sort_by_key(|(index, _)| *index);

    let mut packages = BTreeSet::new();
    let mut cores = BTreeSet::new();
    let mut online_count = 0usize;

    let mut records = Vec::new();
    for (index, path) in cpus {
        let package = read_i32(path.join("topology/physical_package_id"));
        let core = read_i32(path.join("topology/core_id"));
        if let Some(package_id) = package {
            packages.insert(package_id);
            if let Some(core_id) = core {
                cores.insert((package_id, core_id));
            }
        }

        let online = read_trimmed(path.join("online"))
            .map(|value| value != "0")
            .unwrap_or(true);
        if online {
            online_count += 1;
        }

        let mut record = DeviceRecord::new(format!("cpu{index}"));
        record.insert("Online", online.to_string());
        if let Some(value) = package {
            record.insert("PhysicalPackageId", value.to_string());
        }
        if let Some(value) = core {
            record.insert("CoreId", value.to_string());
        }
        for (field, relative) in [
            ("DieId", "topology/die_id"),
            ("ThreadSiblingsList", "topology/thread_siblings_list"),
            ("CoreSiblingsList", "topology/core_siblings_list"),
            ("ScalingDriver", "cpufreq/scaling_driver"),
            ("ScalingGovernor", "cpufreq/scaling_governor"),
            ("CpuInfoMinKHz", "cpufreq/cpuinfo_min_freq"),
            ("CpuInfoMaxKHz", "cpufreq/cpuinfo_max_freq"),
            ("ScalingCurrentKHz", "cpufreq/scaling_cur_freq"),
            ("ScalingMinKHz", "cpufreq/scaling_min_freq"),
            ("ScalingMaxKHz", "cpufreq/scaling_max_freq"),
        ] {
            if let Some(value) = read_trimmed(path.join(relative)) {
                record.insert(field, value);
            }
        }
        records.push(record);
    }

    if !records.is_empty() {
        let mut summary = DeviceRecord::new("Topology Summary");
        summary.insert("LogicalProcessors", records.len().to_string());
        summary.insert("OnlineLogicalProcessors", online_count.to_string());
        if !packages.is_empty() {
            summary.insert("PhysicalPackages", packages.len().to_string());
        }
        if !cores.is_empty() {
            summary.insert("PhysicalCores", cores.len().to_string());
        }
        section.push(summary);
        for record in records {
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

fn read_i32(path: PathBuf) -> Option<i32> {
    read_trimmed(path)?.parse().ok()
}
