use std::{
    env, fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::AbbotikError;

const CONFIG_DIR_NAME: &str = "abbot";
const CONFIG_DIR_CLI_NAME: &str = "cli";
const CONFIG_DIR_PROFILES_NAME: &str = "configs";
const CONFIG_DIR_HOSTS_NAME: &str = "hosts";
const CONFIG_FILE_NAME: &str = "config.toml";
const CURRENT_PROFILE_FILE_NAME: &str = "current-profile";
const CURRENT_HOST_FILE_NAME: &str = "current-host";

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostConfigEntry {
    pub host: String,
    pub path: PathBuf,
    pub config: AbbotikConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Json,
}

impl AbbotikConfig {
    pub fn config_root() -> Result<PathBuf, AbbotikError> {
        let home = dirs::home_dir().ok_or(AbbotikError::ConfigPathUnavailable)?;
        Ok(home
            .join(".config")
            .join(CONFIG_DIR_NAME)
            .join(CONFIG_DIR_CLI_NAME))
    }

    pub fn profiles_dir() -> Result<PathBuf, AbbotikError> {
        Ok(Self::config_root()?.join(CONFIG_DIR_PROFILES_NAME))
    }

    pub fn hosts_dir() -> Result<PathBuf, AbbotikError> {
        Ok(Self::config_root()?.join(CONFIG_DIR_HOSTS_NAME))
    }

    pub fn current_profile_path() -> Result<PathBuf, AbbotikError> {
        Ok(Self::config_root()?.join(CURRENT_PROFILE_FILE_NAME))
    }

    pub fn current_host_path() -> Result<PathBuf, AbbotikError> {
        Ok(Self::config_root()?.join(CURRENT_HOST_FILE_NAME))
    }

    pub fn load_current_profile() -> Result<Option<String>, AbbotikError> {
        let path = Self::current_profile_path()?;
        let raw = match fs::read_to_string(&path) {
            Ok(raw) => raw,
            Err(source) if source.kind() == ErrorKind::NotFound => return Ok(None),
            Err(source) => {
                return Err(AbbotikError::ConfigRead {
                    path: path.display().to_string(),
                    source,
                });
            }
        };

        let profile = raw.trim();
        if profile.is_empty() {
            Ok(None)
        } else {
            Ok(Some(profile.to_string()))
        }
    }

    pub fn save_current_profile(profile: &str) -> Result<(), AbbotikError> {
        let path = Self::current_profile_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| AbbotikError::ConfigWrite {
                path: parent.display().to_string(),
                source,
            })?;
        }

        fs::write(&path, format!("{profile}\n")).map_err(|source| AbbotikError::ConfigWrite {
            path: path.display().to_string(),
            source,
        })
    }

    pub fn clear_current_profile() -> Result<(), AbbotikError> {
        let path = Self::current_profile_path()?;
        match fs::remove_file(&path) {
            Ok(_) => Ok(()),
            Err(source) if source.kind() == ErrorKind::NotFound => Ok(()),
            Err(source) => Err(AbbotikError::ConfigWrite {
                path: path.display().to_string(),
                source,
            }),
        }
    }

    pub fn load_current_host() -> Result<Option<String>, AbbotikError> {
        let path = Self::current_host_path()?;
        let raw = match fs::read_to_string(&path) {
            Ok(raw) => raw,
            Err(source) if source.kind() == ErrorKind::NotFound => return Ok(None),
            Err(source) => {
                return Err(AbbotikError::ConfigRead {
                    path: path.display().to_string(),
                    source,
                });
            }
        };

        let host = raw.trim();
        if host.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self::normalize_host(host)?))
        }
    }

    pub fn save_current_host(host: &str) -> Result<(), AbbotikError> {
        let host = Self::normalize_host(host)?;
        let path = Self::current_host_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| AbbotikError::ConfigWrite {
                path: parent.display().to_string(),
                source,
            })?;
        }

        fs::write(&path, format!("{host}\n")).map_err(|source| AbbotikError::ConfigWrite {
            path: path.display().to_string(),
            source,
        })
    }

    pub fn clear_current_host() -> Result<(), AbbotikError> {
        let path = Self::current_host_path()?;
        match fs::remove_file(&path) {
            Ok(_) => Ok(()),
            Err(source) if source.kind() == ErrorKind::NotFound => Ok(()),
            Err(source) => Err(AbbotikError::ConfigWrite {
                path: path.display().to_string(),
                source,
            }),
        }
    }

    pub fn list_profiles() -> Result<Vec<String>, AbbotikError> {
        let dir = Self::profiles_dir()?;
        let entries = match fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(source) if source.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(source) => {
                return Err(AbbotikError::ConfigRead {
                    path: dir.display().to_string(),
                    source,
                });
            }
        };

        let mut profiles = entries
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();
                let extension = path.extension()?.to_str()?;
                if extension != "toml" {
                    return None;
                }
                path.file_stem()
                    .and_then(|value| value.to_str())
                    .map(ToOwned::to_owned)
            })
            .collect::<Vec<_>>();
        profiles.sort_unstable();
        Ok(profiles)
    }

    pub fn list_hosts() -> Result<Vec<HostConfigEntry>, AbbotikError> {
        let dir = Self::hosts_dir()?;
        let entries = match fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(source) if source.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(source) => {
                return Err(AbbotikError::ConfigRead {
                    path: dir.display().to_string(),
                    source,
                });
            }
        };

        let mut hosts = Vec::new();
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("toml") {
                continue;
            }
            let config = Self::load_from_path(&path)?;
            let host = Self::normalize_host(&config.base_url)?;
            hosts.push(HostConfigEntry { host, path, config });
        }
        hosts.sort_by(|left, right| left.host.cmp(&right.host));
        Ok(hosts)
    }

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

    pub fn load_host_effective(host: &str) -> Result<Self, AbbotikError> {
        let host = Self::normalize_host(host)?;
        let path = Self::host_config_path(&host)?;
        let mut config = match Self::load_from_path(&path) {
            Ok(config) => config,
            Err(AbbotikError::ConfigRead { source, .. })
                if source.kind() == ErrorKind::NotFound =>
            {
                Self::new(host)
            }
            Err(error) => return Err(error),
        };
        config.apply_token_format_env_overrides()?;
        Ok(config)
    }

    pub fn apply_env_overrides(&mut self) -> Result<(), AbbotikError> {
        if let Ok(base_url) = env::var("ABBOTIK_API_BASE_URL") {
            self.base_url = Self::normalize_host(&base_url)?;
        }

        self.apply_token_format_env_overrides()
    }

    pub fn apply_token_format_env_overrides(&mut self) -> Result<(), AbbotikError> {
        if let Ok(token) = env::var("ABBOTIK_API_TOKEN") {
            self.token = Some(token);
        }

        if let Ok(format) = env::var("ABBOTIK_API_FORMAT") {
            self.output_format = format.parse()?;
        }

        Ok(())
    }

    pub fn config_path(profile: Option<&str>) -> Result<PathBuf, AbbotikError> {
        let config_root = Self::config_root()?;

        match profile.filter(|value| !value.trim().is_empty()) {
            Some(profile) => Ok(config_root
                .join(CONFIG_DIR_PROFILES_NAME)
                .join(format!("{profile}.toml"))),
            None => Ok(config_root.join(CONFIG_FILE_NAME)),
        }
    }

    pub fn host_config_path(host: &str) -> Result<PathBuf, AbbotikError> {
        let host = Self::normalize_host(host)?;
        Ok(Self::hosts_dir()?.join(format!("{}.toml", Self::host_file_stem(&host)?)))
    }

    pub fn default_host() -> String {
        Self::default_base_url()
    }

    pub fn normalize_host(host: &str) -> Result<String, AbbotikError> {
        let host = host.trim();
        if host.is_empty() {
            return Err(AbbotikError::InvalidBaseUrl(host.to_string()));
        }
        let host = if host.contains("://") {
            host.to_string()
        } else {
            format!("{}://{host}", Self::default_scheme_for_bare_host(host))
        };
        let parsed =
            url::Url::parse(&host).map_err(|_| AbbotikError::InvalidBaseUrl(host.clone()))?;
        match parsed.scheme() {
            "http" | "https" => {}
            _ => return Err(AbbotikError::InvalidBaseUrl(host)),
        }

        let normalized = parsed.as_str().trim_end_matches('/').to_string();
        if normalized.is_empty() {
            Err(AbbotikError::InvalidBaseUrl(host))
        } else {
            Ok(normalized)
        }
    }

    fn default_scheme_for_bare_host(host: &str) -> &'static str {
        let lower = host.to_ascii_lowercase();
        if lower.starts_with("localhost")
            || lower.ends_with(".local")
            || lower.starts_with("127.")
            || lower.starts_with("10.")
            || lower.starts_with("192.168.")
            || lower.contains(':')
        {
            "http"
        } else {
            "https"
        }
    }

    fn host_file_stem(host: &str) -> Result<String, AbbotikError> {
        let parsed =
            url::Url::parse(host).map_err(|_| AbbotikError::InvalidBaseUrl(host.to_string()))?;
        let host_part = parsed.host_str().unwrap_or("host");
        let port_part = parsed
            .port()
            .map(|port| format!("-{port}"))
            .unwrap_or_default();
        let raw_slug = format!("{}-{}{}", parsed.scheme(), host_part, port_part);
        let slug = raw_slug
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() {
                    ch.to_ascii_lowercase()
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        let hash = Sha256::digest(host.as_bytes());
        let short_hash = hash
            .iter()
            .take(6)
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        Ok(format!("{slug}-{short_hash}"))
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
            .or_else(|| Self::load_current_profile().ok().flatten())
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

    pub fn save_host(&self) -> Result<(), AbbotikError> {
        let path = Self::host_config_path(&self.base_url)?;
        self.save_to_path(&path)
    }

    pub fn load_existing(profile: &str) -> Result<Self, AbbotikError> {
        let path = Self::config_path(Some(profile))?;
        Self::load_from_path(path)
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
    fn normalize_host_accepts_bare_local_and_public_hosts() {
        assert_eq!(
            AbbotikConfig::normalize_host("localhost:3000").expect("local host"),
            "http://localhost:3000"
        );
        assert_eq!(
            AbbotikConfig::normalize_host("api.abbotik.com").expect("public host"),
            "https://api.abbotik.com"
        );
        assert_eq!(
            AbbotikConfig::normalize_host("https://api.abbotik.com/").expect("trailing slash"),
            "https://api.abbotik.com"
        );
    }

    #[test]
    fn host_config_path_uses_host_directory() {
        let home = dirs::home_dir().expect("home directory should exist in tests");
        let path =
            AbbotikConfig::host_config_path("http://localhost:3000").expect("host config path");

        assert!(path.starts_with(home.join(".config").join("abbot").join("cli").join("hosts")));
        assert_eq!(
            path.extension().and_then(|value| value.to_str()),
            Some("toml")
        );
    }

    #[test]
    fn selected_profile_prefers_cli_then_env_then_none() {
        unsafe {
            std::env::remove_var("ABBOTIK_CONFIG");
        }
        let _ = AbbotikConfig::clear_current_profile();
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
        let _ = AbbotikConfig::clear_current_profile();
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
