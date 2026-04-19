use serde_json::Value;

use super::*;

pub(super) async fn run(
    _command: DoctorCommand,
    client: &ApiClient,
    config: &AbbotikConfig,
    selected_profile: Option<&str>,
    save_path: Option<&Path>,
) -> anyhow::Result<()> {
    let token = config.token();
    let token_summary = super::config_cmd::token_summary(token);
    let auth_type = token_summary
        .get("auth_type")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned);

    let health = probe_health(client).await;
    let introspect = probe_introspect(client, token.is_some()).await;
    let refresh_probe = match auth_type.as_deref() {
        Some("username") if token.is_some() => probe_refresh(client, token.unwrap()).await,
        Some("public_key") if token.is_some() => probe_machine_refresh(config),
        _ => RefreshProbe::Skipped {
            reason: "not applicable for the current saved token".to_string(),
        },
    };

    let diagnosis = diagnosis_for(config, &health, &introspect, &refresh_probe);
    let next_steps = next_steps_for(config, &health, &introspect, &refresh_probe);
    let ok = matches!(health, HealthProbe::Ok(_)) && matches!(introspect, IntrospectProbe::Ok(_));

    let report = json!({
        "ok": ok,
        "profile": selected_profile.unwrap_or("default"),
        "config_path": save_path.map(|path| path.display().to_string()),
        "base_url": client.base_url().to_string(),
        "token": token_summary,
        "health": health.to_json(),
        "introspect": introspect.to_json(),
        "refresh_probe": refresh_probe.to_json(),
        "diagnosis": diagnosis,
        "next_steps": next_steps,
    });

    if stdio::stdout().is_terminal() {
        print_text(&render_human_report(&report))?;
    } else {
        print_json(&report)?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
enum HealthProbe {
    Ok(Value),
    Err { error: String },
}

impl HealthProbe {
    fn to_json(&self) -> Value {
        match self {
            HealthProbe::Ok(response) => json!({
                "attempted": true,
                "ok": true,
                "response": response,
            }),
            HealthProbe::Err { error } => json!({
                "attempted": true,
                "ok": false,
                "error": error,
            }),
        }
    }
}

#[derive(Debug, Clone)]
enum IntrospectProbe {
    Skipped { reason: String },
    Ok(Value),
    Err { error: String },
}

impl IntrospectProbe {
    fn to_json(&self) -> Value {
        match self {
            IntrospectProbe::Skipped { reason } => json!({
                "attempted": false,
                "reason": reason,
            }),
            IntrospectProbe::Ok(response) => json!({
                "attempted": true,
                "ok": true,
                "response": response,
            }),
            IntrospectProbe::Err { error } => json!({
                "attempted": true,
                "ok": false,
                "error": error,
            }),
        }
    }
}

#[derive(Debug, Clone)]
enum RefreshProbe {
    Skipped { reason: String },
    Available,
    Blocked { error: String },
    Impossible { reason: String },
}

impl RefreshProbe {
    fn to_json(&self) -> Value {
        match self {
            RefreshProbe::Skipped { reason } => json!({
                "attempted": false,
                "reason": reason,
            }),
            RefreshProbe::Available => json!({
                "attempted": true,
                "available": true,
            }),
            RefreshProbe::Blocked { error } => json!({
                "attempted": true,
                "available": false,
                "error": error,
            }),
            RefreshProbe::Impossible { reason } => json!({
                "attempted": true,
                "available": false,
                "reason": reason,
            }),
        }
    }
}

async fn probe_introspect(client: &ApiClient, has_token: bool) -> IntrospectProbe {
    if !has_token {
        return IntrospectProbe::Skipped {
            reason: "no saved bearer token".to_string(),
        };
    }

    match client.get_json::<Value>("/api/user/introspect").await {
        Ok(value) => IntrospectProbe::Ok(value),
        Err(error) => IntrospectProbe::Err {
            error: error.to_string(),
        },
    }
}

async fn probe_health(client: &ApiClient) -> HealthProbe {
    match client.health().await {
        Ok(value) => HealthProbe::Ok(serde_json::to_value(value).unwrap_or(Value::Null)),
        Err(error) => HealthProbe::Err {
            error: error.to_string(),
        },
    }
}

async fn probe_refresh(client: &ApiClient, token: &str) -> RefreshProbe {
    match client
        .auth_refresh(&RefreshRequest {
            token: token.to_string(),
        })
        .await
    {
        Ok(_) => RefreshProbe::Available,
        Err(error) => RefreshProbe::Blocked {
            error: error.to_string(),
        },
    }
}

fn probe_machine_refresh(config: &AbbotikConfig) -> RefreshProbe {
    let Some(machine_auth) = config.machine_auth.as_ref() else {
        return RefreshProbe::Impossible {
            reason: "saved token is machine-auth but local machine-auth config is missing"
                .to_string(),
        };
    };

    if machine_auth.private_key_path.is_none() {
        return RefreshProbe::Impossible {
            reason: "saved machine token exists but no private key path is saved in config"
                .to_string(),
        };
    }

    if machine_auth.key_id.is_none() && machine_auth.key_fingerprint.is_none() {
        return RefreshProbe::Impossible {
            reason: "saved machine token exists but config is missing key id and fingerprint"
                .to_string(),
        };
    }

    RefreshProbe::Available
}

fn diagnosis_for(
    config: &AbbotikConfig,
    health: &HealthProbe,
    introspect: &IntrospectProbe,
    refresh_probe: &RefreshProbe,
) -> String {
    if let HealthProbe::Err { error } = health {
        return format!("The configured server could not be reached cleanly: {error}");
    }

    if config.token().is_none() {
        return "No saved bearer token exists for the active profile.".to_string();
    }

    match introspect {
        IntrospectProbe::Ok(_) => match refresh_probe {
            RefreshProbe::Blocked { error } if error.contains("Local auth is disabled") => {
                "The saved token currently works, but this host disables local token refresh and expects Auth0/OIDC for fresh username sessions.".to_string()
            }
            _ => "The saved token is currently accepted by the server.".to_string(),
        },
        IntrospectProbe::Err { error } => match refresh_probe {
            RefreshProbe::Blocked { error: refresh_error }
                if refresh_error.contains("Local auth is disabled") =>
            {
                "The saved username token is not usable for re-auth here because this host disables local auth refresh and expects Auth0/OIDC.".to_string()
            }
            RefreshProbe::Available => {
                "The saved token failed the live identity probe, but refresh is available for this auth mode.".to_string()
            }
            RefreshProbe::Impossible { reason } => format!(
                "The saved token failed the live identity probe, and machine refresh is not possible: {reason}"
            ),
            RefreshProbe::Blocked { .. } | RefreshProbe::Skipped { .. } => format!(
                "The saved token failed the live identity probe: {error}"
            ),
        },
        IntrospectProbe::Skipped { .. } => "No saved token is available to inspect.".to_string(),
    }
}

fn next_steps_for(
    config: &AbbotikConfig,
    health: &HealthProbe,
    introspect: &IntrospectProbe,
    refresh_probe: &RefreshProbe,
) -> Vec<String> {
    if let HealthProbe::Err { .. } = health {
        return vec![
            "Run `abbot config` to confirm the active base URL and selected profile.".to_string(),
            "Run `abbot health` to retry the live server check directly.".to_string(),
            "Fix the server URL or network path before debugging auth state further.".to_string(),
        ];
    }

    if config.token().is_none() {
        return vec![
            "Run `abbot config` to confirm the active base URL and config file.".to_string(),
            "Set or obtain a bearer token for this host, then save it with `abbot auth token set <jwt>`.".to_string(),
            "If this host still supports local username auth, use `abbot auth login --tenant <tenant> --username <user> --password <password>`.".to_string(),
        ];
    }

    match (introspect, refresh_probe) {
        (IntrospectProbe::Ok(_), RefreshProbe::Blocked { error })
            if error.contains("Local auth is disabled") =>
        {
            vec![
                "Your current token still works, so `abbot tui` and authenticated API calls should work until it expires.".to_string(),
                "Do not rely on `abbot auth refresh` for this host; it is blocked by server auth policy.".to_string(),
                "For a fresh human token here, use the host's Auth0/OIDC path or set a new bearer token with `abbot auth token set <jwt>`.".to_string(),
            ]
        }
        (IntrospectProbe::Ok(_), _) => vec![
            "You are authenticated right now. Try `abbot tui` or `abbot user introspect`.".to_string(),
            "Run `abbot config` whenever you need the active config path and base URL.".to_string(),
        ],
        (_, RefreshProbe::Available) => vec![
            "Run `abbot auth refresh` to renew the saved token and update local config.".to_string(),
            "Then rerun `abbot doctor` to confirm the live identity probe passes.".to_string(),
        ],
        (_, RefreshProbe::Blocked { error }) if error.contains("Local auth is disabled") => vec![
            "This host disables local username refresh, so `abbot auth refresh` will not fix the session.".to_string(),
            "Use the host's Auth0/OIDC path to obtain a fresh human token, then save it with `abbot auth token set <jwt>`.".to_string(),
            "Run `abbot config` first if you need to confirm which host and profile you are targeting.".to_string(),
        ],
        (_, RefreshProbe::Impossible { reason }) => vec![
            format!("Machine-style refresh is blocked locally: {reason}"),
            "Repair the saved machine-auth config or reconnect with `abbot auth machine connect ...`.".to_string(),
        ],
        _ => vec![
            "Run `abbot config` to inspect the active profile and host.".to_string(),
            "If you have a valid bearer token already, save it with `abbot auth token set <jwt>`.".to_string(),
        ],
    }
}

fn render_human_report(report: &Value) -> String {
    let ok = report.get("ok").and_then(Value::as_bool).unwrap_or(false);
    let status = if ok { "OK" } else { "ATTENTION" };
    let config_path = string_or(report.get("config_path"), "unknown");
    let base_url = string_or(report.get("base_url"), "unknown");
    let profile = string_or(report.get("profile"), "default");
    let token = report.get("token").unwrap_or(&Value::Null);
    let health = report.get("health").unwrap_or(&Value::Null);
    let introspect = report.get("introspect").unwrap_or(&Value::Null);
    let refresh = report.get("refresh_probe").unwrap_or(&Value::Null);
    let diagnosis = string_or(report.get("diagnosis"), "No diagnosis available.");
    let next_steps = string_list(report.get("next_steps"));

    let token_present = token
        .get("present")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let token_status = if token_present { "saved" } else { "missing" };
    let auth_type = string_or(token.get("auth_type"), "unknown");
    let username = string_or(token.get("username"), "-");
    let tenant = string_or(token.get("tenant"), "-");
    let access = string_or(token.get("access"), "-");
    let expires_at = string_or(token.get("expires_at"), "-");

    let introspect_line = match (
        introspect
            .get("attempted")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        introspect.get("ok").and_then(Value::as_bool),
    ) {
        (false, _) => format!(
            "not attempted ({})",
            string_or(introspect.get("reason"), "no reason")
        ),
        (true, Some(true)) => {
            let user = introspect
                .get("response")
                .and_then(|value| value.get("data"))
                .and_then(|value| value.get("user"))
                .and_then(|value| value.get("username"))
                .and_then(Value::as_str)
                .unwrap_or(username.as_str());
            let tenant_name = introspect
                .get("response")
                .and_then(|value| value.get("data"))
                .and_then(|value| value.get("tenant"))
                .and_then(|value| value.get("name"))
                .and_then(Value::as_str)
                .unwrap_or(tenant.as_str());
            format!("accepted by server as {user}@{tenant_name}")
        }
        _ => format!(
            "failed ({})",
            string_or(introspect.get("error"), "unknown error")
        ),
    };

    let health_line = match health.get("ok").and_then(Value::as_bool) {
        Some(true) => {
            let status = health
                .get("response")
                .and_then(|value| value.get("data"))
                .and_then(|value| value.get("status"))
                .and_then(Value::as_str)
                .unwrap_or("ok");
            format!("reachable ({status})")
        }
        _ => format!(
            "failed ({})",
            string_or(health.get("error"), "unknown error")
        ),
    };

    let refresh_line = match (
        refresh
            .get("attempted")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        refresh.get("available").and_then(Value::as_bool),
    ) {
        (false, _) => format!(
            "not attempted ({})",
            string_or(refresh.get("reason"), "no reason")
        ),
        (true, Some(true)) => "available".to_string(),
        _ => {
            let detail = refresh
                .get("error")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
                .or_else(|| {
                    refresh
                        .get("reason")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned)
                })
                .unwrap_or_else(|| "blocked".to_string());
            format!("blocked ({detail})")
        }
    };

    let mut lines = vec![
        format!("Doctor: {status}"),
        String::new(),
        "Config".to_string(),
        format!("  profile: {profile}"),
        format!("  path: {config_path}"),
        format!("  base URL: {base_url}"),
        String::new(),
        "Token".to_string(),
        format!("  state: {token_status}"),
        format!("  auth type: {auth_type}"),
        format!("  user: {username}"),
        format!("  tenant: {tenant}"),
        format!("  access: {access}"),
        format!("  expires: {expires_at}"),
        String::new(),
        "Checks".to_string(),
        format!("  health: {health_line}"),
        format!("  introspect: {introspect_line}"),
        format!("  refresh: {refresh_line}"),
        String::new(),
        "Diagnosis".to_string(),
        format!("  {diagnosis}"),
    ];

    if !next_steps.is_empty() {
        lines.push(String::new());
        lines.push("Next Steps".to_string());
        lines.extend(
            next_steps
                .into_iter()
                .enumerate()
                .map(|(index, step)| format!("  {}. {}", index + 1, step)),
        );
    }

    lines.join("\n")
}

fn string_or(value: Option<&Value>, fallback: &str) -> String {
    value
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| fallback.to_string())
}

fn string_list(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnosis_marks_missing_token() {
        let config = AbbotikConfig::default();
        let diagnosis = diagnosis_for(
            &config,
            &HealthProbe::Ok(json!({"data": {"status": "ok"}})),
            &IntrospectProbe::Skipped {
                reason: "no saved bearer token".to_string(),
            },
            &RefreshProbe::Skipped {
                reason: "not applicable".to_string(),
            },
        );
        assert_eq!(
            diagnosis,
            "No saved bearer token exists for the active profile."
        );
    }

    #[test]
    fn next_steps_call_out_local_auth_disabled() {
        let config = AbbotikConfig::default().with_token("header.payload.sig");
        let steps = next_steps_for(
            &config,
            &HealthProbe::Ok(json!({"data": {"status": "ok"}})),
            &IntrospectProbe::Err {
                error: "token rejected".to_string(),
            },
            &RefreshProbe::Blocked {
                error: "server returned 403 Forbidden for POST https://integration.abbotik.com/auth/refresh: Local auth is disabled. Auth0/OIDC is the production authentication authority.".to_string(),
            },
        );

        assert!(steps
            .iter()
            .any(|step| step.contains("disables local username refresh")));
    }

    #[test]
    fn render_human_report_includes_key_sections() {
        let output = render_human_report(&json!({
            "ok": true,
            "profile": "default",
            "config_path": "/tmp/config.toml",
            "base_url": "https://integration.abbotik.com/",
            "token": {
                "present": true,
                "auth_type": "username",
                "username": "ianzepp",
                "tenant": "ianzepp",
                "access": "root",
                "expires_at": "2026-04-25T03:51:51+00:00"
            },
            "health": {
                "attempted": true,
                "ok": true,
                "response": {
                    "data": {
                        "status": "ok"
                    }
                }
            },
            "introspect": {
                "attempted": true,
                "ok": true,
                "response": {
                    "data": {
                        "user": { "username": "ianzepp" },
                        "tenant": { "name": "ianzepp" }
                    }
                }
            },
            "refresh_probe": {
                "attempted": true,
                "available": false,
                "error": "Local auth is disabled."
            },
            "diagnosis": "The saved token currently works.",
            "next_steps": [
                "Run `abbot tui`."
            ]
        }));

        assert!(output.contains("Doctor: OK"));
        assert!(output.contains("Config"));
        assert!(output.contains("Token"));
        assert!(output.contains("Checks"));
        assert!(output.contains("Diagnosis"));
        assert!(output.contains("Next Steps"));
        assert!(output.contains("health: reachable (ok)"));
        assert!(output.contains("accepted by server as ianzepp@ianzepp"));
    }
}
