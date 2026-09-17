use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemReport {
    pub meta: ReportMeta,
    pub sections: Vec<Section>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMeta {
    pub app_name: String,
    pub app_version: String,
    pub generated_at: String,
    pub os: String,
    pub architecture: String,
    pub elevated: bool,
    pub privacy_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub records: Vec<DeviceRecord>,
}

impl Section {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            records: Vec::new(),
        }
    }

    pub fn push(&mut self, record: DeviceRecord) {
        self.records.push(record);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRecord {
    pub label: String,
    pub fields: BTreeMap<String, String>,
}

impl DeviceRecord {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            fields: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let value = value.into();
        if !value.trim().is_empty() {
            self.fields.insert(key.into(), value);
        }
    }

    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.insert(key, value);
        self
    }
}
