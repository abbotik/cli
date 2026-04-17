use super::*;
use crate::api::ProvisionData;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
pub(super) struct TokenClaims {
    #[serde(default)]
    pub auth_type: Option<String>,
    #[serde(default)]
    pub tenant: Option<String>,
    #[serde(default)]
    pub key_id: Option<String>,
    #[serde(default)]
    pub key_fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MachineRefreshContext {
    pub tenant: String,
    pub private_key_path: String,
    pub key_id: Option<String>,
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MachineConnectContext {
    pub tenant: String,
    pub key_id: Option<String>,
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct SavedMachinePaths {
    pub public_key_path: Option<String>,
    pub private_key_path: Option<String>,
}

pub(super) fn decode_token_claims(token: &str) -> anyhow::Result<TokenClaims> {
    let payload = token
        .split('.')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("token is not a valid JWT"))?;
    let bytes = URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|error| anyhow::anyhow!("failed to decode token payload: {error}"))?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub(super) fn normalize_path_string(path: &str) -> anyhow::Result<String> {
    let normalized = stdfs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path));
    Ok(normalized.to_string_lossy().to_string())
}

pub(super) fn source_path_string(source: Option<&str>) -> anyhow::Result<Option<String>> {
    match source.and_then(|value| value.strip_prefix('@')) {
        Some(path) => Ok(Some(normalize_path_string(path)?)),
        None => Ok(None),
    }
}

pub(super) fn machine_key_paths(
    key_source: Option<&str>,
    public_key_source: Option<&str>,
) -> anyhow::Result<(String, Option<String>)> {
    let private_key_path = key_source
        .map(|value| value.strip_prefix('@').unwrap_or(value))
        .ok_or_else(|| anyhow::anyhow!("machine connect requires --key <private-key-path>"))?;
    Ok((
        normalize_path_string(private_key_path)?,
        source_path_string(public_key_source)?,
    ))
}

pub(super) fn current_machine_token_claims(config: &AbbotikConfig) -> TokenClaims {
    config
        .token()
        .and_then(|token| decode_token_claims(token).ok())
        .filter(|claims| claims.auth_type.as_deref() == Some("public_key"))
        .unwrap_or_default()
}

pub(super) fn resolve_machine_refresh_context(
    config: &AbbotikConfig,
    claims: TokenClaims,
) -> anyhow::Result<MachineRefreshContext> {
    let machine_auth = config.machine_auth.clone().unwrap_or_default();
    let tenant = claims.tenant.or(machine_auth.tenant).ok_or_else(|| {
        anyhow::anyhow!("machine refresh requires a tenant in the saved token or config")
    })?;
    let private_key_path = machine_auth.private_key_path.ok_or_else(|| {
        anyhow::anyhow!("machine refresh requires a saved private key path in local config")
    })?;
    let key_id = claims.key_id.or(machine_auth.key_id);
    let fingerprint = claims.key_fingerprint.or(machine_auth.key_fingerprint);

    if key_id.is_none() && fingerprint.is_none() {
        return Err(anyhow::anyhow!(
            "machine refresh requires a saved key id or fingerprint in the token or config"
        ));
    }

    Ok(MachineRefreshContext {
        tenant,
        private_key_path,
        key_id,
        fingerprint,
    })
}

pub(super) fn resolve_machine_connect_context(
    requested_tenant: Option<String>,
    config: &AbbotikConfig,
    token_claims: &TokenClaims,
) -> anyhow::Result<MachineConnectContext> {
    let tenant = requested_tenant
        .or_else(|| {
            config
                .machine_auth
                .as_ref()
                .and_then(|machine| machine.tenant.clone())
        })
        .or_else(|| token_claims.tenant.clone())
        .ok_or_else(|| anyhow::anyhow!("machine connect requires --tenant or a saved machine tenant"))?;

    let machine_auth = config
        .machine_auth
        .clone()
        .filter(|machine| machine.tenant.as_deref() == Some(tenant.as_str()))
        .unwrap_or_default();

    Ok(MachineConnectContext {
        tenant,
        key_id: machine_auth.key_id.or_else(|| token_claims.key_id.clone()),
        fingerprint: machine_auth
            .key_fingerprint
            .or_else(|| token_claims.key_fingerprint.clone()),
    })
}

pub(super) fn resolve_saved_machine_paths(
    public_key_source: Option<&str>,
    save_public_key_path: Option<&str>,
    save_private_key_path: Option<&str>,
) -> anyhow::Result<SavedMachinePaths> {
    Ok(SavedMachinePaths {
        public_key_path: match save_public_key_path {
            Some(path) => Some(normalize_path_string(path)?),
            None => source_path_string(public_key_source)?,
        },
        private_key_path: save_private_key_path
            .map(normalize_path_string)
            .transpose()?,
    })
}

pub(super) fn update_machine_auth_from_verify_response(
    config: &mut AbbotikConfig,
    verify_data: Option<&VerifyData>,
    token: &str,
    public_key_path: Option<&str>,
    private_key_path: Option<&str>,
) -> anyhow::Result<()> {
    let claims = decode_token_claims(token).unwrap_or_default();
    let machine_auth = config.machine_auth_mut();

    if let Some(data) = verify_data {
        machine_auth.tenant = Some(data.tenant.clone());
        machine_auth.key_id = Some(data.key_id.clone());
    }
    if let Some(tenant) = claims.tenant {
        machine_auth.tenant = Some(tenant);
    }
    if let Some(key_id) = claims.key_id {
        machine_auth.key_id = Some(key_id);
    }
    if let Some(fingerprint) = claims.key_fingerprint {
        machine_auth.key_fingerprint = Some(fingerprint);
    }
    if let Some(path) = public_key_path {
        machine_auth.public_key_path = Some(normalize_path_string(path)?);
    }
    if let Some(path) = private_key_path {
        machine_auth.private_key_path = Some(normalize_path_string(path)?);
    }

    Ok(())
}

pub(super) fn update_machine_auth_from_provision(
    config: &mut AbbotikConfig,
    provision_data: &ProvisionData,
    saved_paths: SavedMachinePaths,
) {
    let machine_auth = config.machine_auth_mut();
    machine_auth.tenant = Some(provision_data.tenant.clone());
    machine_auth.key_id = Some(provision_data.key.id.clone());
    machine_auth.key_fingerprint = Some(provision_data.key.fingerprint.clone());
    if let Some(path) = saved_paths.public_key_path {
        machine_auth.public_key_path = Some(path);
    }
    if let Some(path) = saved_paths.private_key_path {
        machine_auth.private_key_path = Some(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api::{ProvisionChallengeData, ProvisionData, ProvisionKeyData, ProvisionUserData},
        config::MachineAuthConfig,
    };
    use serde_json::json;

    fn machine_token(claims: serde_json::Value) -> String {
        let payload = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&claims).expect("claims serialize"));
        format!("header.{payload}.sig")
    }

    #[test]
    fn resolve_machine_refresh_context_prefers_claims_and_requires_identity() {
        let mut config = AbbotikConfig::default();
        config.machine_auth = Some(MachineAuthConfig {
            tenant: Some("from-config".to_string()),
            key_id: Some("key-config".to_string()),
            key_fingerprint: Some("fp-config".to_string()),
            public_key_path: None,
            private_key_path: Some("/tmp/key.pem".to_string()),
        });

        let ctx = resolve_machine_refresh_context(
            &config,
            TokenClaims {
                tenant: Some("from-token".to_string()),
                key_id: Some("key-token".to_string()),
                key_fingerprint: None,
                auth_type: Some("public_key".to_string()),
            },
        )
        .expect("context should resolve");

        assert_eq!(ctx.tenant, "from-token");
        assert_eq!(ctx.key_id.as_deref(), Some("key-token"));
        assert_eq!(ctx.fingerprint.as_deref(), Some("fp-config"));
        assert_eq!(ctx.private_key_path, "/tmp/key.pem");
    }

    #[test]
    fn resolve_machine_connect_context_uses_matching_tenant_machine_auth() {
        let mut config = AbbotikConfig::default();
        config.machine_auth = Some(MachineAuthConfig {
            tenant: Some("acme".to_string()),
            key_id: Some("key-1".to_string()),
            key_fingerprint: Some("fp-1".to_string()),
            public_key_path: None,
            private_key_path: None,
        });

        let ctx = resolve_machine_connect_context(
            None,
            &config,
            &TokenClaims {
                tenant: Some("wrong".to_string()),
                key_id: Some("token-key".to_string()),
                key_fingerprint: Some("token-fp".to_string()),
                auth_type: Some("public_key".to_string()),
            },
        )
        .expect("context should resolve");

        assert_eq!(ctx.tenant, "acme");
        assert_eq!(ctx.key_id.as_deref(), Some("key-1"));
        assert_eq!(ctx.fingerprint.as_deref(), Some("fp-1"));
    }

    #[test]
    fn update_machine_auth_from_verify_response_updates_paths_and_claims() {
        let mut config = AbbotikConfig::default();
        let token = machine_token(json!({
            "auth_type": "public_key",
            "tenant": "acme",
            "key_id": "key-claim",
            "key_fingerprint": "fp-claim"
        }));

        update_machine_auth_from_verify_response(
            &mut config,
            Some(&VerifyData {
                token: token.clone(),
                expires_in: 3600,
                tenant: "tenant-from-verify".to_string(),
                tenant_id: "tenant_123".to_string(),
                key_id: "key-from-verify".to_string(),
            }),
            &token,
            Some("/tmp/public.pem"),
            Some("/tmp/private.pem"),
        )
        .expect("update should succeed");

        let machine_auth = config.machine_auth.expect("machine auth should exist");
        assert_eq!(machine_auth.tenant.as_deref(), Some("acme"));
        assert_eq!(machine_auth.key_id.as_deref(), Some("key-claim"));
        assert_eq!(machine_auth.key_fingerprint.as_deref(), Some("fp-claim"));
        assert!(machine_auth.public_key_path.as_deref().unwrap().ends_with("/tmp/public.pem"));
        assert!(machine_auth.private_key_path.as_deref().unwrap().ends_with("/tmp/private.pem"));
    }

    #[test]
    fn resolve_saved_machine_paths_uses_explicit_overrides_first() {
        let paths = resolve_saved_machine_paths(
            Some("@/tmp/source-public.pem"),
            Some("/tmp/override-public.pem"),
            Some("/tmp/private.pem"),
        )
        .expect("paths should resolve");

        assert!(paths.public_key_path.as_deref().unwrap().ends_with("/tmp/override-public.pem"));
        assert!(paths.private_key_path.as_deref().unwrap().ends_with("/tmp/private.pem"));
    }

    #[test]
    fn update_machine_auth_from_provision_copies_provision_metadata() {
        let mut config = AbbotikConfig::default();
        update_machine_auth_from_provision(
            &mut config,
            &ProvisionData {
                tenant: "acme".to_string(),
                tenant_id: "tenant_123".to_string(),
                user: ProvisionUserData {
                    id: "user_123".to_string(),
                    username: "bot".to_string(),
                    access: "full".to_string(),
                },
                key: ProvisionKeyData {
                    id: "key_123".to_string(),
                    name: Some("bot-key".to_string()),
                    algorithm: "ed25519".to_string(),
                    fingerprint: "fp_123".to_string(),
                },
                challenge: ProvisionChallengeData {
                    challenge_id: "challenge_123".to_string(),
                    nonce: "nonce".to_string(),
                    expires_in: 60,
                },
            },
            SavedMachinePaths {
                public_key_path: Some("/tmp/public.pem".to_string()),
                private_key_path: Some("/tmp/private.pem".to_string()),
            },
        );

        let machine_auth = config.machine_auth.expect("machine auth should exist");
        assert_eq!(machine_auth.tenant.as_deref(), Some("acme"));
        assert_eq!(machine_auth.key_id.as_deref(), Some("key_123"));
        assert_eq!(machine_auth.key_fingerprint.as_deref(), Some("fp_123"));
        assert_eq!(machine_auth.public_key_path.as_deref(), Some("/tmp/public.pem"));
        assert_eq!(machine_auth.private_key_path.as_deref(), Some("/tmp/private.pem"));
    }

    #[test]
    fn current_machine_token_claims_ignores_non_machine_tokens() {
        let mut config = AbbotikConfig::default();
        config.set_token(machine_token(json!({
            "auth_type": "password",
            "tenant": "acme"
        })));

        assert_eq!(current_machine_token_claims(&config), TokenClaims::default());
    }
}
