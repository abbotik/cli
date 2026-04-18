use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::*;

#[derive(Debug, Clone, Deserialize, Default)]
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
    _command: ConfigCommand,
    config: &AbbotikConfig,
    selected_profile: Option<&str>,
    save_path: Option<&Path>,
) -> anyhow::Result<()> {
    print_json(&json!({
        "profile": selected_profile.unwrap_or("default"),
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
    }))?;
    Ok(())
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
}
