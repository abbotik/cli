use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::*;
use crate::config::OutputFormat;

const CONFIG_KEYS: &[&str] = &[
    "base_url",
    "token",
    "format",
    "machine_auth.tenant",
    "machine_auth.key_id",
    "machine_auth.key_fingerprint",
    "machine_auth.public_key_path",
    "machine_auth.private_key_path",
];

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
struct JwtSummaryClaims {
    #[serde(default)]
    auth_type: Option<String>,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    tenant: Option<String>,
    #[serde(default)]
    tenant_id: Option<String>,
    #[serde(default)]
    access: Option<String>,
    #[serde(default)]
    exp: Option<i64>,
    #[serde(default)]
    iat: Option<i64>,
    #[serde(default)]
    key_id: Option<String>,
    #[serde(default)]
    key_fingerprint: Option<String>,
}

pub(super) fn run(
    command: ConfigCommand,
    config: &AbbotikConfig,
    selected_profile: Option<&str>,
    save_path: Option<&Path>,
) -> anyhow::Result<()> {
    match command.command {
        None => {
            print_json(&config_summary(
                config,
                selected_profile.unwrap_or("default"),
                save_path,
            ))?;
            Ok(())
        }
        Some(ConfigSubcommand::Create(args)) => create_profile(args),
        Some(ConfigSubcommand::Use(args)) => use_profile(args),
        Some(ConfigSubcommand::List) => list_profiles(selected_profile),
        Some(ConfigSubcommand::Show(args)) => show_profile(args),
        Some(ConfigSubcommand::Set(args)) => set_profile_value(args),
        Some(ConfigSubcommand::Get(args)) => get_profile_value(args),
        Some(ConfigSubcommand::Delete(args)) => delete_profile(args, selected_profile),
        Some(ConfigSubcommand::Doctor) => doctor_config(config, selected_profile, save_path),
    }
}

pub(super) fn token_summary(token: Option<&str>) -> Value {
    let Some(token) = token else {
        return json!({
            "present": false
        });
    };

    let claims = decode_jwt_claims(token);
    let expires_at = claims
        .exp
        .and_then(|value| DateTime::<Utc>::from_timestamp(value, 0))
        .map(|value| value.to_rfc3339());
    let issued_at = claims
        .iat
        .and_then(|value| DateTime::<Utc>::from_timestamp(value, 0))
        .map(|value| value.to_rfc3339());
    let expires_in_seconds = claims.exp.map(|value| value - Utc::now().timestamp());

    json!({
        "present": true,
        "auth_type": claims.auth_type,
        "username": claims.username,
        "tenant": claims.tenant,
        "tenant_id": claims.tenant_id,
        "access": claims.access,
        "key_id": claims.key_id,
        "key_fingerprint": claims.key_fingerprint,
        "issued_at": issued_at,
        "expires_at": expires_at,
        "expires_in_seconds": expires_in_seconds,
    })
}

pub(super) fn config_summary(
    config: &AbbotikConfig,
    profile: &str,
    save_path: Option<&Path>,
) -> Value {
    json!({
        "profile": profile,
        "config_path": save_path.map(|path| path.display().to_string()),
        "base_url": config.base_url,
        "token": token_summary(config.token()),
        "machine_auth": {
            "present": config.machine_auth.is_some(),
            "tenant": config.machine_auth.as_ref().and_then(|value| value.tenant.clone()),
            "key_id": config.machine_auth.as_ref().and_then(|value| value.key_id.clone()),
            "key_fingerprint": config.machine_auth.as_ref().and_then(|value| value.key_fingerprint.clone()),
            "public_key_path": config.machine_auth.as_ref().and_then(|value| value.public_key_path.clone()),
            "private_key_path": config.machine_auth.as_ref().and_then(|value| value.private_key_path.clone()),
        }
    })
}

fn create_profile(args: ConfigCreateCommand) -> anyhow::Result<()> {
    let name = normalize_profile_name(&args.name)?;
    let path = AbbotikConfig::config_path(Some(&name))?;
    if path.exists() {
        anyhow::bail!("config profile `{name}` already exists");
    }

    let config = match args.url {
        Some(url) => AbbotikConfig::new(url),
        None => AbbotikConfig::default(),
    };
    config.save(Some(&name))?;

    print_json(&json!({
        "ok": true,
        "profile": name,
        "config_path": path.display().to_string(),
        "base_url": config.base_url,
    }))
}

fn use_profile(args: ConfigUseCommand) -> anyhow::Result<()> {
    let name = normalize_profile_name(&args.name)?;
    ensure_profile_exists(&name)?;
    AbbotikConfig::save_current_profile(&name)?;

    print_json(&json!({
        "ok": true,
        "profile": name,
        "current_profile_path": AbbotikConfig::current_profile_path()?.display().to_string(),
    }))
}

fn list_profiles(selected_profile: Option<&str>) -> anyhow::Result<()> {
    let profiles = AbbotikConfig::list_profiles()?;
    let current = AbbotikConfig::load_current_profile()?;
    let default_path = AbbotikConfig::config_path(None)?;
    let mut listed_profiles = vec![json!({
        "name": "default",
        "kind": "default",
        "active": selected_profile.is_none(),
        "selected_by_current_profile": current.is_none(),
        "config_path": default_path.display().to_string(),
        "exists": default_path.exists(),
    })];

    listed_profiles.extend(profiles.iter().map(|profile| json!({
        "name": profile,
        "kind": "named",
        "active": selected_profile == Some(profile.as_str()),
        "selected_by_current_profile": current.as_deref() == Some(profile.as_str()),
        "config_path": AbbotikConfig::config_path(Some(profile)).ok().map(|path| path.display().to_string()),
        "exists": true,
    })));

    print_json(&json!({
        "profiles": listed_profiles,
        "active_profile": selected_profile.unwrap_or("default"),
        "current_profile": current,
    }))
}

fn show_profile(args: ConfigShowCommand) -> anyhow::Result<()> {
    let name = normalize_profile_name(&args.name)?;
    let config = AbbotikConfig::load_existing(&name)?;
    let path = AbbotikConfig::config_path(Some(&name))?;
    print_json(&config_summary(&config, &name, Some(&path)))
}

fn set_profile_value(args: ConfigSetCommand) -> anyhow::Result<()> {
    let name = normalize_profile_name(&args.name)?;
    let mut config = AbbotikConfig::load_existing(&name)?;
    let key = normalize_key(&args.key)?;
    match (args.unset, args.value.as_deref()) {
        (true, _) => unset_config_key(&mut config, &key)?,
        (false, Some(value)) => set_config_key(&mut config, &key, value)?,
        (false, None) => anyhow::bail!("missing value for `abbot config set {name} {key}`"),
    }
    config.save(Some(&name))?;

    let path = AbbotikConfig::config_path(Some(&name))?;
    print_json(&config_summary(&config, &name, Some(&path)))
}

fn get_profile_value(args: ConfigGetCommand) -> anyhow::Result<()> {
    let name = normalize_profile_name(&args.name)?;
    let config = AbbotikConfig::load_existing(&name)?;
    let key = normalize_key(&args.key)?;
    print_json(&json!({
        "profile": name,
        "key": key,
        "value": get_config_key(&config, &key),
    }))
}

fn delete_profile(args: ConfigDeleteCommand, selected_profile: Option<&str>) -> anyhow::Result<()> {
    let name = normalize_profile_name(&args.name)?;
    let path = ensure_profile_exists(&name)?;
    stdfs::remove_file(&path)
        .map_err(|error| anyhow::anyhow!("failed to delete {}: {error}", path.display()))?;

    if selected_profile == Some(name.as_str())
        || AbbotikConfig::load_current_profile()?.as_deref() == Some(name.as_str())
    {
        AbbotikConfig::clear_current_profile()?;
    }

    print_json(&json!({
        "ok": true,
        "profile": name,
        "deleted_path": path.display().to_string(),
    }))
}

fn doctor_config(
    config: &AbbotikConfig,
    selected_profile: Option<&str>,
    save_path: Option<&Path>,
) -> anyhow::Result<()> {
    let profile = selected_profile.unwrap_or("default");
    let mut issues = Vec::new();
    let mut checks = Vec::new();

    let base_url_valid = config.base_url().is_ok();
    checks.push(json!({
        "name": "base_url",
        "ok": base_url_valid,
        "detail": if base_url_valid { "valid URL" } else { "invalid URL" },
    }));
    if !base_url_valid {
        issues.push("The configured base URL is not a valid absolute URL.".to_string());
    }

    let token = config.token();
    let token_decodable =
        token.map(|value| decode_jwt_claims(value) != JwtSummaryClaims::default());
    checks.push(json!({
        "name": "token_shape",
        "ok": token.is_none() || token_decodable == Some(true),
        "detail": match token {
            None => "no saved token",
            Some(_) if token_decodable == Some(true) => "JWT claims decoded",
            Some(_) => "token is present but not decodable as JWT claims",
        }
    }));
    if token.is_some() && token_decodable != Some(true) {
        issues.push(
            "The saved bearer token is present but its JWT claims could not be decoded locally."
                .to_string(),
        );
    }

    let machine_auth = config.machine_auth.as_ref();
    let private_key_ok = machine_auth
        .and_then(|value| value.private_key_path.as_deref())
        .map(path_exists)
        .unwrap_or(true);
    checks.push(json!({
        "name": "machine_auth.private_key_path",
        "ok": private_key_ok,
        "detail": machine_auth
            .and_then(|value| value.private_key_path.as_deref())
            .map(|value| format!("exists={}", path_exists(value)))
            .unwrap_or_else(|| "not set".to_string()),
    }));
    if !private_key_ok {
        issues.push("The saved machine private key path does not exist on disk.".to_string());
    }

    let public_key_ok = machine_auth
        .and_then(|value| value.public_key_path.as_deref())
        .map(path_exists)
        .unwrap_or(true);
    checks.push(json!({
        "name": "machine_auth.public_key_path",
        "ok": public_key_ok,
        "detail": machine_auth
            .and_then(|value| value.public_key_path.as_deref())
            .map(|value| format!("exists={}", path_exists(value)))
            .unwrap_or_else(|| "not set".to_string()),
    }));
    if !public_key_ok {
        issues.push("The saved machine public key path does not exist on disk.".to_string());
    }

    let report = json!({
        "ok": issues.is_empty(),
        "profile": profile,
        "config_path": save_path.map(|path| path.display().to_string()),
        "summary": config_summary(config, profile, save_path),
        "checks": checks,
        "issues": issues,
        "supported_keys": CONFIG_KEYS,
    });

    if stdio::stdout().is_terminal() {
        print_text(&render_config_doctor_human_report(&report))?;
    } else {
        print_json(&report)?;
    }

    Ok(())
}

fn decode_jwt_claims(token: &str) -> JwtSummaryClaims {
    let payload = token.split('.').nth(1);
    let Some(payload) = payload else {
        return JwtSummaryClaims::default();
    };

    let Ok(bytes) = URL_SAFE_NO_PAD.decode(payload) else {
        return JwtSummaryClaims::default();
    };

    serde_json::from_slice(&bytes).unwrap_or_default()
}

fn normalize_profile_name(name: &str) -> anyhow::Result<String> {
    let name = name.trim();
    if name.is_empty() {
        anyhow::bail!("profile name must not be empty");
    }
    if name == "default" {
        anyhow::bail!("`default` is reserved for ~/.config/abbot/cli/config.toml");
    }
    if name.contains('/') || name.contains('\\') {
        anyhow::bail!("profile name must not contain path separators");
    }
    Ok(name.to_string())
}

fn ensure_profile_exists(name: &str) -> anyhow::Result<PathBuf> {
    let path = AbbotikConfig::config_path(Some(name))?;
    if !path.exists() {
        anyhow::bail!("config profile `{name}` does not exist");
    }
    Ok(path)
}

fn normalize_key(key: &str) -> anyhow::Result<String> {
    let key = key.trim().replace('-', "_");
    if CONFIG_KEYS.contains(&key.as_str()) {
        Ok(key)
    } else {
        anyhow::bail!(
            "unsupported config key `{key}`; expected one of: {}",
            CONFIG_KEYS.join(", ")
        )
    }
}

fn set_config_key(config: &mut AbbotikConfig, key: &str, value: &str) -> anyhow::Result<()> {
    match key {
        "base_url" => config.base_url = value.to_string(),
        "token" => config.token = Some(value.to_string()),
        "format" => config.output_format = value.parse()?,
        "machine_auth.tenant" => config.machine_auth_mut().tenant = Some(value.to_string()),
        "machine_auth.key_id" => config.machine_auth_mut().key_id = Some(value.to_string()),
        "machine_auth.key_fingerprint" => {
            config.machine_auth_mut().key_fingerprint = Some(value.to_string())
        }
        "machine_auth.public_key_path" => {
            config.machine_auth_mut().public_key_path = Some(value.to_string())
        }
        "machine_auth.private_key_path" => {
            config.machine_auth_mut().private_key_path = Some(value.to_string())
        }
        _ => anyhow::bail!("unsupported config key `{key}`"),
    }
    Ok(())
}

fn unset_config_key(config: &mut AbbotikConfig, key: &str) -> anyhow::Result<()> {
    match key {
        "base_url" => config.base_url = AbbotikConfig::default().base_url,
        "token" => config.token = None,
        "format" => config.output_format = OutputFormat::default(),
        "machine_auth.tenant" => config.machine_auth_mut().tenant = None,
        "machine_auth.key_id" => config.machine_auth_mut().key_id = None,
        "machine_auth.key_fingerprint" => config.machine_auth_mut().key_fingerprint = None,
        "machine_auth.public_key_path" => config.machine_auth_mut().public_key_path = None,
        "machine_auth.private_key_path" => config.machine_auth_mut().private_key_path = None,
        _ => anyhow::bail!("unsupported config key `{key}`"),
    }

    if config.machine_auth.as_ref().is_some_and(|value| {
        value.tenant.is_none()
            && value.key_id.is_none()
            && value.key_fingerprint.is_none()
            && value.public_key_path.is_none()
            && value.private_key_path.is_none()
    }) {
        config.machine_auth = None;
    }

    Ok(())
}

fn get_config_key(config: &AbbotikConfig, key: &str) -> Value {
    match key {
        "base_url" => Value::String(config.base_url.clone()),
        "token" => config
            .token
            .as_ref()
            .map(|value| Value::String(value.clone()))
            .unwrap_or(Value::Null),
        "format" => Value::String(config.output_format.as_str().to_string()),
        "machine_auth.tenant" => optional_string(
            config
                .machine_auth
                .as_ref()
                .and_then(|value| value.tenant.as_ref()),
        ),
        "machine_auth.key_id" => optional_string(
            config
                .machine_auth
                .as_ref()
                .and_then(|value| value.key_id.as_ref()),
        ),
        "machine_auth.key_fingerprint" => optional_string(
            config
                .machine_auth
                .as_ref()
                .and_then(|value| value.key_fingerprint.as_ref()),
        ),
        "machine_auth.public_key_path" => optional_string(
            config
                .machine_auth
                .as_ref()
                .and_then(|value| value.public_key_path.as_ref()),
        ),
        "machine_auth.private_key_path" => optional_string(
            config
                .machine_auth
                .as_ref()
                .and_then(|value| value.private_key_path.as_ref()),
        ),
        _ => Value::Null,
    }
}

fn optional_string(value: Option<&String>) -> Value {
    value.cloned().map(Value::String).unwrap_or(Value::Null)
}

fn path_exists(value: &str) -> bool {
    Path::new(value).exists()
}

fn render_config_doctor_human_report(report: &Value) -> String {
    let ok = report.get("ok").and_then(Value::as_bool).unwrap_or(false);
    let status = if ok { "OK" } else { "ATTENTION" };
    let profile = report
        .get("profile")
        .and_then(Value::as_str)
        .unwrap_or("default");
    let config_path = report
        .get("config_path")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let issues = report
        .get("issues")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let checks = report
        .get("checks")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut lines = vec![
        format!("Config Doctor: {status}"),
        String::new(),
        format!("Profile: {profile}"),
        format!("Path: {config_path}"),
        String::new(),
        "Checks".to_string(),
    ];

    for check in checks {
        let name = check
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let detail = check.get("detail").and_then(Value::as_str).unwrap_or("-");
        let state = if check.get("ok").and_then(Value::as_bool).unwrap_or(false) {
            "ok"
        } else {
            "bad"
        };
        lines.push(format!("  {name}: {state} ({detail})"));
    }

    if !issues.is_empty() {
        lines.push(String::new());
        lines.push("Issues".to_string());
        for issue in issues {
            if let Some(issue) = issue.as_str() {
                lines.push(format!("  - {issue}"));
            }
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jwt(claims: Value) -> String {
        let payload = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&claims).expect("claims encode"));
        format!("header.{payload}.sig")
    }

    #[test]
    fn token_summary_marks_missing_token() {
        let summary = token_summary(None);
        assert_eq!(summary["present"], Value::Bool(false));
    }

    #[test]
    fn token_summary_extracts_human_claims() {
        let token = jwt(json!({
            "auth_type": "username",
            "username": "alice",
            "tenant": "acme",
            "access": "root",
            "exp": Utc::now().timestamp() + 3600,
        }));

        let summary = token_summary(Some(&token));
        assert_eq!(summary["present"], Value::Bool(true));
        assert_eq!(summary["username"], Value::String("alice".to_string()));
        assert_eq!(summary["tenant"], Value::String("acme".to_string()));
        assert_eq!(summary["access"], Value::String("root".to_string()));
    }

    #[test]
    fn normalize_key_accepts_known_keys() {
        assert_eq!(normalize_key("base-url").expect("key"), "base_url");
        assert_eq!(
            normalize_key("machine_auth.public_key_path").expect("key"),
            "machine_auth.public_key_path"
        );
    }

    #[test]
    fn unset_clears_machine_auth_section_when_empty() {
        let mut config = AbbotikConfig::default();
        config.machine_auth_mut().tenant = Some("acme".to_string());
        unset_config_key(&mut config, "machine_auth.tenant").expect("unset");
        assert!(config.machine_auth.is_none());
    }
}
