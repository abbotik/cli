use super::*;


#[derive(Debug, Clone, Default, Deserialize)]
struct TokenClaims {
    #[serde(default)]
    auth_type: Option<String>,
    #[serde(default)]
    tenant: Option<String>,
    #[serde(default)]
    key_id: Option<String>,
    #[serde(default)]
    key_fingerprint: Option<String>,
}

fn decode_token_claims(token: &str) -> anyhow::Result<TokenClaims> {
    let payload = token
        .split('.')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("token is not a valid JWT"))?;
    let bytes = URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|error| anyhow::anyhow!("failed to decode token payload: {error}"))?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn normalize_path_string(path: &str) -> anyhow::Result<String> {
    let normalized = stdfs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path));
    Ok(normalized.to_string_lossy().to_string())
}

fn source_path_string(source: Option<&str>) -> anyhow::Result<Option<String>> {
    match source.and_then(|value| value.strip_prefix('@')) {
        Some(path) => Ok(Some(normalize_path_string(path)?)),
        None => Ok(None),
    }
}

fn sign_machine_nonce(private_key_path: &str, nonce: &str) -> anyhow::Result<String> {
    let pem = stdfs::read_to_string(private_key_path)?;
    let signing_key = SigningKey::from_pkcs8_pem(&pem).map_err(|error| {
        anyhow::anyhow!("failed to decode Ed25519 private key at {private_key_path}: {error}")
    })?;
    let signature = signing_key.sign(nonce.as_bytes());
    Ok(URL_SAFE_NO_PAD.encode(signature.to_bytes()))
}

fn derive_public_key_pem(private_key_path: &str) -> anyhow::Result<String> {
    let pem = stdfs::read_to_string(private_key_path)?;
    let signing_key = SigningKey::from_pkcs8_pem(&pem).map_err(|error| {
        anyhow::anyhow!("failed to decode Ed25519 private key at {private_key_path}: {error}")
    })?;
    signing_key
        .verifying_key()
        .to_public_key_pem(Default::default())
        .map_err(|error| {
            anyhow::anyhow!("failed to encode Ed25519 public key from {private_key_path}: {error}")
        })
}

fn update_machine_auth_from_verify_response(
    config: &mut AbbotikConfig,
    verify_data: Option<&crate::api::VerifyData>,
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

async fn refresh_machine_auth(
    client: &ApiClient,
    config: &mut AbbotikConfig,
    claims: TokenClaims,
    save_path: Option<&Path>,
) -> anyhow::Result<Value> {
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

    let challenge = client
        .auth_challenge(&ChallengeRequest {
            tenant: Some(tenant.clone()),
            key_id,
            fingerprint,
        })
        .await?;
    let challenge_data = challenge
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("challenge response did not include challenge data"))?;
    let signature = sign_machine_nonce(&private_key_path, &challenge_data.nonce)?;
    let verify = client
        .auth_verify(&VerifyRequest {
            tenant: Some(tenant),
            challenge_id: Some(challenge_data.challenge_id.clone()),
            signature: Some(signature),
        })
        .await?;
    let next_token = verify
        .data
        .as_ref()
        .map(|data| data.token.clone())
        .ok_or_else(|| anyhow::anyhow!("verify response did not include a bearer token"))?;

    config.set_token(next_token.clone());
    update_machine_auth_from_verify_response(
        config,
        verify.data.as_ref(),
        &next_token,
        None,
        None,
    )?;
    save_config(config, save_path)?;

    Ok(json!({
        "success": verify.success,
        "data": {
            "token": next_token,
            "expires_in": verify.data.as_ref().map(|data| data.expires_in),
            "tenant": verify.data.as_ref().map(|data| data.tenant.clone()),
            "tenant_id": verify.data.as_ref().map(|data| data.tenant_id.clone()),
            "key_id": verify.data.as_ref().map(|data| data.key_id.clone()),
            "refresh_method": "challenge_verify"
        },
        "challenge": challenge,
        "verify": verify,
    }))
}

fn save_machine_verify_result(
    config: &mut AbbotikConfig,
    verify_data: Option<&VerifyData>,
    token: &str,
    public_key_path: Option<&str>,
    private_key_path: Option<&str>,
    save_path: Option<&Path>,
) -> anyhow::Result<()> {
    config.set_token(token.to_string());
    update_machine_auth_from_verify_response(
        config,
        verify_data,
        token,
        public_key_path,
        private_key_path,
    )?;
    save_config(config, save_path)?;
    Ok(())
}

fn machine_key_paths(
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

fn machine_public_key_pem(
    public_key_source: Option<&str>,
    private_key_path: &str,
) -> anyhow::Result<String> {
    match read_secret_source_option(public_key_source)? {
        Some(public_key) => Ok(public_key),
        None => derive_public_key_pem(private_key_path),
    }
}

pub(super) fn vec_or_none(values: Vec<String>) -> Option<Vec<String>> {
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

async fn machine_connect(
    args: crate::cli::AuthMachineConnectCommand,
    config: &mut AbbotikConfig,
    client: &ApiClient,
    save_path: Option<&Path>,
) -> anyhow::Result<()> {
    let (private_key_path, public_key_path) =
        machine_key_paths(args.key.as_deref(), args.public_key.as_deref())?;
    let token_claims = config
        .token()
        .and_then(|token| decode_token_claims(token).ok())
        .filter(|claims| claims.auth_type.as_deref() == Some("public_key"))
        .unwrap_or_default();
    let tenant = args
        .tenant
        .clone()
        .or_else(|| {
            config
                .machine_auth
                .as_ref()
                .and_then(|machine| machine.tenant.clone())
        })
        .or(token_claims.tenant.clone())
        .ok_or_else(|| {
            anyhow::anyhow!("machine connect requires --tenant or a saved machine tenant")
        })?;

    let machine_auth = config
        .machine_auth
        .clone()
        .filter(|machine| machine.tenant.as_deref() == Some(tenant.as_str()))
        .unwrap_or_default();
    let key_id = machine_auth.key_id.clone().or(token_claims.key_id.clone());
    let fingerprint = machine_auth
        .key_fingerprint
        .clone()
        .or(token_claims.key_fingerprint.clone());

    if args.invite_code.is_none() && (key_id.is_some() || fingerprint.is_some()) {
        let challenge = client
            .auth_challenge(&ChallengeRequest {
                tenant: Some(tenant.clone()),
                key_id,
                fingerprint,
            })
            .await?;
        let challenge_data = challenge
            .data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("challenge response did not include challenge data"))?;
        let signature = sign_machine_nonce(&private_key_path, &challenge_data.nonce)?;
        let verify = client
            .auth_verify(&VerifyRequest {
                tenant: Some(tenant),
                challenge_id: Some(challenge_data.challenge_id.clone()),
                signature: Some(signature),
            })
            .await?;
        let token = verify
            .data
            .as_ref()
            .map(|data| data.token.clone())
            .ok_or_else(|| anyhow::anyhow!("verify response did not include a bearer token"))?;
        save_machine_verify_result(
            config,
            verify.data.as_ref(),
            &token,
            public_key_path.as_deref(),
            Some(&private_key_path),
            save_path,
        )?;
        print_json(&json!({
            "success": verify.success,
            "mode": "reconnect",
            "challenge": challenge,
            "verify": verify,
        }))?;
        return Ok(());
    }

    let username = args.username.clone().ok_or_else(|| {
        anyhow::anyhow!(
            "machine connect requires --username when no saved machine key metadata exists"
        )
    })?;
    let public_key = machine_public_key_pem(args.public_key.as_deref(), &private_key_path)?;
    let provision = client
        .auth_provision(&ProvisionRequest {
            tenant: Some(tenant.clone()),
            username: Some(username),
            invite_code: args.invite_code.clone(),
            public_key: Some(public_key),
            algorithm: args.algorithm,
            key_name: args.key_name,
        })
        .await?;
    let provision_data = provision
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("provision response did not include challenge data"))?;
    let signature = sign_machine_nonce(&private_key_path, &provision_data.challenge.nonce)?;
    let verify = client
        .auth_verify(&VerifyRequest {
            tenant: Some(tenant),
            challenge_id: Some(provision_data.challenge.challenge_id.clone()),
            signature: Some(signature),
        })
        .await?;
    let token = verify
        .data
        .as_ref()
        .map(|data| data.token.clone())
        .ok_or_else(|| anyhow::anyhow!("verify response did not include a bearer token"))?;
    save_machine_verify_result(
        config,
        verify.data.as_ref(),
        &token,
        public_key_path.as_deref(),
        Some(&private_key_path),
        save_path,
    )?;
    print_json(&json!({
        "success": verify.success,
        "mode": "provision",
        "provision": provision,
        "verify": verify,
    }))?;
    Ok(())
}

pub(super) async fn run(
    command: AuthCommand,
    config: &mut AbbotikConfig,
    client: &ApiClient,
    save_path: Option<&Path>,
) -> anyhow::Result<()> {
    match command.command {
        crate::cli::AuthSubcommand::Login(args) => {
            let password = read_secret_source_option(args.password.as_deref())?;
            let response = client
                .auth_login(&LoginRequest {
                    tenant: args.tenant,
                    tenant_id: args.tenant_id,
                    username: args.username,
                    password,
                    format: args.format,
                })
                .await?;
            let token = response
                .data
                .as_ref()
                .map(|data| data.token.clone())
                .unwrap_or_default();
            if !token.is_empty() {
                config.set_token(token.clone());
                save_config(config, save_path)?;
            }
            print_json(&response)?;
        }
        crate::cli::AuthSubcommand::Register(args) => {
            let password = read_secret_source_option(args.password.as_deref())?;
            let register_response = client
                .auth_register(&RegisterRequest {
                    tenant: args.tenant,
                    username: args.username,
                    invite_code: args.invite_code,
                    email: args.email,
                    password: password.clone(),
                })
                .await?;
            let register_username = register_response
                .data
                .as_ref()
                .and_then(|data| data.user.as_ref().map(|user| user.username.clone()))
                .or_else(|| {
                    register_response
                        .data
                        .as_ref()
                        .and_then(|data| data.username.clone())
                });
            let login_response = client
                .auth_login(&LoginRequest {
                    tenant: register_response
                        .data
                        .as_ref()
                        .map(|data| data.tenant.clone()),
                    tenant_id: None,
                    username: register_username,
                    password,
                    format: None,
                })
                .await?;
            if let Some(token) = login_response.data.as_ref().map(|data| data.token.clone()) {
                config.set_token(token);
                save_config(config, save_path)?;
            }
            print_json(&json!({
                "success": register_response.success && login_response.success,
                "register": register_response,
                "login": login_response,
            }))?;
        }
        crate::cli::AuthSubcommand::Refresh(args) => {
            let token = args
                .token
                .or_else(|| config.token.clone())
                .ok_or_else(|| anyhow::anyhow!("refresh requires a token or saved config token"))?;

            let response = match decode_token_claims(&token) {
                Ok(claims) if claims.auth_type.as_deref() == Some("public_key") => {
                    refresh_machine_auth(client, config, claims, save_path).await?
                }
                _ => {
                    let response = client.auth_refresh(&RefreshRequest { token }).await?;
                    if let Some(next_token) = response.data.as_ref().map(|data| data.token.clone())
                    {
                        config.set_token(next_token);
                        save_config(config, save_path)?;
                    }
                    json!(response)
                }
            };

            print_json(&response)?;
        }
        crate::cli::AuthSubcommand::Provision(args) => {
            let public_key = read_secret_source_option(args.public_key.as_deref())?;
            let response = client
                .auth_provision(&ProvisionRequest {
                    tenant: args.tenant,
                    username: args.username,
                    invite_code: args.invite_code,
                    public_key,
                    algorithm: args.algorithm,
                    key_name: args.key_name,
                })
                .await?;

            let saved_public_key_path = match args.save_public_key_path.as_deref() {
                Some(path) => Some(normalize_path_string(path)?),
                None => source_path_string(args.public_key.as_deref())?,
            };
            let saved_private_key_path = args
                .save_private_key_path
                .as_deref()
                .map(normalize_path_string)
                .transpose()?;

            if saved_public_key_path.is_some() || saved_private_key_path.is_some() {
                if let Some(data) = response.data.as_ref() {
                    let machine_auth = config.machine_auth_mut();
                    machine_auth.tenant = Some(data.tenant.clone());
                    machine_auth.key_id = Some(data.key.id.clone());
                    machine_auth.key_fingerprint = Some(data.key.fingerprint.clone());
                    if let Some(path) = saved_public_key_path {
                        machine_auth.public_key_path = Some(path);
                    }
                    if let Some(path) = saved_private_key_path {
                        machine_auth.private_key_path = Some(path);
                    }
                    save_config(config, save_path)?;
                }
            }

            print_json(&response)?;
        }
        crate::cli::AuthSubcommand::Challenge(args) => {
            let response = client
                .auth_challenge(&ChallengeRequest {
                    tenant: args.tenant,
                    key_id: args.key_id,
                    fingerprint: args.fingerprint,
                })
                .await?;
            print_json(&response)?;
        }
        crate::cli::AuthSubcommand::Verify(args) => {
            let signature = read_secret_source_option(args.signature.as_deref())?;
            let response = client
                .auth_verify(&VerifyRequest {
                    tenant: args.tenant,
                    challenge_id: args.challenge_id,
                    signature,
                })
                .await?;
            if let Some(token) = response.data.as_ref().map(|data| data.token.clone()) {
                save_machine_verify_result(
                    config,
                    response.data.as_ref(),
                    &token,
                    args.save_public_key_path.as_deref(),
                    args.save_private_key_path.as_deref(),
                    save_path,
                )?;
            }
            print_json(&response)?;
        }
        crate::cli::AuthSubcommand::Machine(command) => match command.command {
            crate::cli::AuthMachineSubcommand::Connect(args) => {
                machine_connect(args, config, client, save_path).await?;
            }
        },
        crate::cli::AuthSubcommand::Dissolve(command) => {
            auth_dissolve(command, client).await?;
        }
        crate::cli::AuthSubcommand::Token(command) => {
            auth_token(command, config, save_path).await?
        }
        crate::cli::AuthSubcommand::Tenants => {
            print_json(&client.auth_tenants().await?)?;
        }
    }
    Ok(())
}

async fn auth_dissolve(
    command: crate::cli::AuthDissolveCommand,
    client: &ApiClient,
) -> anyhow::Result<()> {
    match command.command {
        crate::cli::AuthDissolveSubcommand::Request(args) => {
            let password = read_secret_source_option(args.password.as_deref())?;
            let response = client
                .auth_dissolve(&DissolveRequest {
                    tenant: args.tenant,
                    username: args.username,
                    password,
                })
                .await?;
            print_json(&response)?;
        }
        crate::cli::AuthDissolveSubcommand::Confirm(args) => {
            let response = client
                .auth_dissolve_confirm(&DissolveConfirmRequest {
                    confirmation_token: args.confirmation_token,
                })
                .await?;
            print_json(&response)?;
        }
    }
    Ok(())
}

async fn auth_token(
    command: crate::cli::AuthTokenCommand,
    config: &mut AbbotikConfig,
    save_path: Option<&Path>,
) -> anyhow::Result<()> {
    match command.command {
        crate::cli::AuthTokenSubcommand::Get => {
            let token = config
                .token()
                .ok_or_else(|| anyhow::anyhow!("no saved token available"))?;
            println!("{token}");
        }
        crate::cli::AuthTokenSubcommand::Set(args) => {
            let token = read_secret_source(&args.token)?;
            config.set_token(token);
            save_config(config, save_path)?;
        }
        crate::cli::AuthTokenSubcommand::Clear => {
            config.clear_token();
            save_config(config, save_path)?;
        }
    }
    Ok(())
}
