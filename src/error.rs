use std::{fmt, io};

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AbbotikError {
    #[error("invalid api base url: {0}")]
    InvalidBaseUrl(String),

    #[error("unsupported output format `{0}`; only `json` is supported")]
    UnsupportedOutputFormat(String),

    #[error("configuration path is unavailable on this platform")]
    ConfigPathUnavailable,

    #[error("failed to read config at {path}: {source}")]
    ConfigRead { path: String, source: io::Error },

    #[error("failed to write config at {path}: {source}")]
    ConfigWrite { path: String, source: io::Error },

    #[error("failed to serialize config: {0}")]
    ConfigSerialize(toml::ser::Error),

    #[error("failed to deserialize config: {0}")]
    ConfigDeserialize(toml::de::Error),

    #[error("request to {method} {url} failed: {source}")]
    Request {
        method: reqwest::Method,
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("server returned invalid JSON for {method} {url}: {source}")]
    InvalidResponse {
        method: reqwest::Method,
        url: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("server returned {status} for {method} {url}: {message}")]
    Http {
        status: StatusCode,
        method: reqwest::Method,
        url: String,
        message: String,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerErrorEnvelope {
    pub success: Option<bool>,
    pub error: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
}

impl ServerErrorEnvelope {
    pub fn message(&self) -> String {
        self.message
            .clone()
            .or_else(|| self.error.clone())
            .or_else(|| self.code.clone())
            .unwrap_or_else(|| "unknown server error".to_string())
    }
}

impl AbbotikError {
    pub fn http_message(&self) -> Option<&str> {
        match self {
            AbbotikError::Http { message, .. } => Some(message.as_str()),
            _ => None,
        }
    }
}

impl fmt::Display for ServerErrorEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message())
    }
}
