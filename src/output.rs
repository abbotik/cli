use serde::Serialize;

use crate::config::OutputFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Human,
    Json,
}

impl OutputMode {
    pub fn from_format(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => OutputMode::Json,
        }
    }
}

pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}
