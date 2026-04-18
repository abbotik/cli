use std::{
    env, fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::error::AbbotikError;

const CONFIG_DIR_NAME: &str = "abbot";
const CONFIG_DIR_CLI_NAME: &str = "cli";
const CONFIG_DIR_PROFILES_NAME: &str = "configs";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MachineAuthConfig {
    #[serde(default)]
    pub tenant: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub key_fingerprint: Option<String>,
    #[serde(default)]
    pub public_key_path: Option<String>,
    #[serde(default)]
    pub private_key_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbbotikConfig {
    #[serde(default = "AbbotikConfig::default_base_url")]
    pub base_url: String,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default = "AbbotikConfig::default_output_format")]
    pub output_format: OutputFormat,
    #[serde(default)]
    pub machine_auth: Option<MachineAuthConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Json,
}

impl AbbotikConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            token: None,
            output_format: OutputFormat::Json,
            machine_auth: None,
        }
    }

    pub fn from_env() -> Result<Self, AbbotikError> {
        let mut config = Self::default();
        config.apply_env_overrides()?;
        Ok(config)
    }

    pub fn load_effective(profile: Option<&str>) -> Result<Self, AbbotikError> {
        let mut config = Self::load(profile)?;
        config.apply_env_overrides()?;
        Ok(config)
    }

    pub fn apply_env_overrides(&mut self) -> Result<(), AbbotikError> {
        if let Ok(base_url) = env::var("ABBOTIK_API_BASE_URL") {
            self.base_url = base_url;
        }

        if let Ok(token) = env::var("ABBOTIK_API_TOKEN") {
            self.token = Some(token);
        }

        if let Ok(format) = env::var("ABBOTIK_API_FORMAT") {
            self.output_format = format.parse()?;
        }

        Ok(())
    }

    pub fn config_path(profile: Option<&str>) -> Result<PathBuf, AbbotikError> {
        let home = dirs::home_dir().ok_or(AbbotikError::ConfigPathUnavailable)?;
        let config_root = home
            .join(".config")
            .join(CONFIG_DIR_NAME)
            .join(CONFIG_DIR_CLI_NAME);

        match profile.filter(|value| !value.trim().is_empty()) {
            Some(profile) => Ok(config_root
                .join(CONFIG_DIR_PROFILES_NAME)
                .join(format!("{profile}.toml"))),
            None => Ok(config_root.join(CONFIG_FILE_NAME)),
        }
    }

    pub fn selected_profile(cli_profile: Option<&str>) -> Option<String> {
        cli_profile
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .or_else(|| {
                env::var("ABBOTIK_CONFIG")
                    .ok()
                    .map(|value| value.trim().to_string())
                    .filter(|value| !value.is_empty())
            })
    }

    pub fn load(profile: Option<&str>) -> Result<Self, AbbotikError> {
        let path = Self::config_path(profile)?;
        match Self::load_from_path(&path) {
            Ok(config) => Ok(config),
            Err(AbbotikError::ConfigRead { source, .. })
                if source.kind() == ErrorKind::NotFound =>
            {
                Ok(Self::default())
            }
            Err(error) => Err(error),
        }
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, AbbotikError> {
        let path = path.as_ref();
        let raw = fs::read_to_string(path).map_err(|source| AbbotikError::ConfigRead {
            path: path.display().to_string(),
            source,
        })?;
        toml::from_str(&raw).map_err(AbbotikError::ConfigDeserialize)
    }

    pub fn save(&self, profile: Option<&str>) -> Result<(), AbbotikError> {
        let path = Self::config_path(profile)?;
        self.save_to_path(&path)
    }

    pub fn save_to_path(&self, path: impl AsRef<Path>) -> Result<(), AbbotikError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| AbbotikError::ConfigWrite {
                path: parent.display().to_string(),
                source,
            })?;
        }

        let toml = toml::to_string_pretty(self).map_err(AbbotikError::ConfigSerialize)?;
        fs::write(path, toml).map_err(|source| AbbotikError::ConfigWrite {
            path: path.display().to_string(),
            source,
        })
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn clear_token(&mut self) {
        self.token = None;
    }

    pub fn machine_auth_mut(&mut self) -> &mut MachineAuthConfig {
        self.machine_auth
            .get_or_insert_with(MachineAuthConfig::default)
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    pub fn base_url(&self) -> Result<url::Url, AbbotikError> {
        url::Url::parse(&self.base_url)
            .map_err(|_| AbbotikError::InvalidBaseUrl(self.base_url.clone()))
    }
}

impl Default for AbbotikConfig {
    fn default() -> Self {
        Self {
            base_url: Self::default_base_url(),
            token: None,
            output_format: Self::default_output_format(),
            machine_auth: None,
        }
    }
}

impl AbbotikConfig {
    fn default_base_url() -> String {
        "https://api.abbotik.com".to_string()
    }

    fn default_output_format() -> OutputFormat {
        OutputFormat::Json
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{AbbotikConfig, MachineAuthConfig, OutputFormat};

    #[test]
    fn default_base_url_points_to_public_api() {
        assert_eq!(AbbotikConfig::default().base_url, "https://api.abbotik.com");
    }

    #[test]
    fn default_config_path_uses_home_dot_config_abbot_cli() {
        let home = dirs::home_dir().expect("home directory should exist in tests");
        let expected = home
            .join(".config")
            .join("abbot")
            .join("cli")
            .join("config.toml");

        assert_eq!(
            AbbotikConfig::config_path(None).expect("config path should resolve"),
            expected
        );
    }

    #[test]
    fn named_config_path_uses_profile_directory() {
        let home = dirs::home_dir().expect("home directory should exist in tests");
        let expected = home
            .join(".config")
            .join("abbot")
            .join("cli")
            .join("configs")
            .join("staging.toml");

        assert_eq!(
            AbbotikConfig::config_path(Some("staging")).expect("config path should resolve"),
            expected
        );
    }

    #[test]
    fn selected_profile_prefers_cli_then_env_then_none() {
        unsafe {
            std::env::remove_var("ABBOTIK_CONFIG");
        }
        assert_eq!(AbbotikConfig::selected_profile(None), None);

        unsafe {
            std::env::set_var("ABBOTIK_CONFIG", "staging");
        }
        assert_eq!(
            AbbotikConfig::selected_profile(None),
            Some("staging".to_string())
        );
        assert_eq!(
            AbbotikConfig::selected_profile(Some("prod")),
            Some("prod".to_string())
        );

        unsafe {
            std::env::remove_var("ABBOTIK_CONFIG");
        }
    }

    #[test]
    fn save_and_load_round_trips_token_state() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("abbot-config-{stamp}.toml"));

        let config = AbbotikConfig {
            base_url: "https://example.com".to_string(),
            token: Some("jwt-one".to_string()),
            output_format: OutputFormat::Json,
            machine_auth: Some(MachineAuthConfig {
                tenant: Some("acme".to_string()),
                key_id: Some("key-1".to_string()),
                key_fingerprint: Some("fp_1234".to_string()),
                public_key_path: Some("/tmp/machine.pub".to_string()),
                private_key_path: Some("/tmp/machine.key".to_string()),
            }),
        };

        config.save_to_path(&path).expect("config should save");
        let loaded = AbbotikConfig::load_from_path(&path).expect("config should load");

        assert_eq!(loaded.base_url, "https://example.com");
        assert_eq!(loaded.token.as_deref(), Some("jwt-one"));
        assert_eq!(loaded.output_format, OutputFormat::Json);
        assert_eq!(
            loaded
                .machine_auth
                .as_ref()
                .and_then(|m| m.tenant.as_deref()),
            Some("acme")
        );
        assert_eq!(
            loaded
                .machine_auth
                .as_ref()
                .and_then(|m| m.public_key_path.as_deref()),
            Some("/tmp/machine.pub")
        );
        assert_eq!(
            loaded
                .machine_auth
                .as_ref()
                .and_then(|m| m.private_key_path.as_deref()),
            Some("/tmp/machine.key")
        );

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn token_set_and_clear_behaves_like_logout() {
        let mut config = AbbotikConfig::default();

        config.set_token("jwt-two");
        assert_eq!(config.token.as_deref(), Some("jwt-two"));

        config.clear_token();
        assert_eq!(config.token, None);
    }

    #[test]
    fn machine_auth_mut_creates_default_section() {
        let mut config = AbbotikConfig::default();
        config.machine_auth_mut().private_key_path = Some("/tmp/machine.key".to_string());

        assert_eq!(
            config
                .machine_auth
                .as_ref()
                .and_then(|m| m.private_key_path.as_deref()),
            Some("/tmp/machine.key")
        );
    }

    #[test]
    fn load_from_missing_path_reports_config_read() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("abbot-missing-{stamp}.toml"));

        let loaded = AbbotikConfig::load_from_path(&path);
        assert!(matches!(
            loaded,
            Err(super::AbbotikError::ConfigRead { .. })
        ));
    }

    #[test]
    fn unsupported_output_format_is_rejected() {
        let parsed = "yaml".parse::<OutputFormat>();
        assert!(matches!(
            parsed,
            Err(super::AbbotikError::UnsupportedOutputFormat(value)) if value == "yaml"
        ));
    }
}

impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Json => "json",
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = AbbotikError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            _ => Err(AbbotikError::UnsupportedOutputFormat(value.to_string())),
        }
    }
}
