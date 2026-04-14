use std::{
    fs,
    io::{self, IsTerminal, Read},
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{pkcs8::DecodePrivateKey, Signer, SigningKey};
use reqwest::Method;
use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::{
    api::{
        ApiClient, ChallengeRequest, DissolveConfirmRequest, DissolveRequest, LoginRequest,
        ProvisionRequest, RefreshRequest, RegisterRequest, VerifyRequest,
    },
    cli::{
        AclsCommand, AggregateCommand, AggregateOptions, AppCommand, AuthCommand, BulkCommand,
        BulkOptions, Cli, Command, CronCommand, DataCommand, DescribeCommand, DocsCommand,
        FindCommand, FindOptions, FsCommand, FsOptions, KeysCommand, KeysCreateCommand,
        KeysRotateCommand, KeysSubcommand, PublicCommand, StatCommand, TrackedCommand,
        TrashedCommand, UserCommand, UserCreateCommand, UserListCommand, UserPasswordCommand,
        UserSubcommand,
    },
    config::AbbotikConfig,
    data,
};

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let mut config = AbbotikConfig::load_effective()?;
    let save_path = AbbotikConfig::config_path().ok();

    if let Some(base_url) = cli.globals.base_url {
        config.base_url = base_url;
    }
    if let Some(token) = cli.globals.token {
        config.token = Some(token);
    }
    if let Some(format) = cli.globals.format {
        config.output_format = format.parse().unwrap_or_default();
    }

    let client = ApiClient::new(config.clone())?;

    match cli.command {
        Command::Public(command) => public(command, &client).await?,
        Command::Auth(command) => {
            auth(command, &mut config, &client, save_path.as_deref()).await?;
        }
        Command::Health => print_json(&client.health().await?)?,
        Command::Docs(command) => docs(command, &client).await?,
        Command::Describe(command) => describe(command, &client).await?,
        Command::Data(command) => data(command, &client).await?,
        Command::Find(command) => find(command, &client).await?,
        Command::Aggregate(command) => aggregate(command, &client).await?,
        Command::Bulk(command) => bulk(command, &client).await?,
        Command::Acls(command) => acls(command, &client).await?,
        Command::Stat(command) => stat(command, &client).await?,
        Command::Tracked(command) => tracked(command, &client).await?,
        Command::Trashed(command) => trashed(command, &client).await?,
        Command::User(command) => user(command, &client).await?,
        Command::Keys(command) => keys(command, &client).await?,
        Command::Cron(command) => cron(command, &client).await?,
        Command::Fs(command) => fs(command, &client).await?,
        Command::App(command) => app(command, &client).await?,
    }

    Ok(())
}

async fn public(command: PublicCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::PublicSubcommand::Root => print_text(&client.get_text("/").await?)?,
        crate::cli::PublicSubcommand::Llms => print_text(&client.get_text("/llms.txt").await?)?,
    }
    Ok(())
}

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
    let normalized = fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path));
    Ok(normalized.to_string_lossy().to_string())
}

fn source_path_string(source: Option<&str>) -> anyhow::Result<Option<String>> {
    match source.and_then(|value| value.strip_prefix('@')) {
        Some(path) => Ok(Some(normalize_path_string(path)?)),
        None => Ok(None),
    }
}

fn sign_machine_nonce(private_key_path: &str, nonce: &str) -> anyhow::Result<String> {
    let pem = fs::read_to_string(private_key_path)?;
    let signing_key = SigningKey::from_pkcs8_pem(&pem)
        .map_err(|error| anyhow::anyhow!("failed to decode Ed25519 private key at {private_key_path}: {error}"))?;
    let signature = signing_key.sign(nonce.as_bytes());
    Ok(URL_SAFE_NO_PAD.encode(signature.to_bytes()))
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
    let tenant = claims
        .tenant
        .or(machine_auth.tenant)
        .ok_or_else(|| anyhow::anyhow!("machine refresh requires a tenant in the saved token or config"))?;
    let private_key_path = machine_auth
        .private_key_path
        .ok_or_else(|| anyhow::anyhow!("machine refresh requires a saved private key path in local config"))?;
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
    update_machine_auth_from_verify_response(config, verify.data.as_ref(), &next_token, None, None)?;
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

async fn auth(
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
                    email: args.email,
                    password: password.clone(),
                })
                .await?;
            let login_response = client
                .auth_login(&LoginRequest {
                    tenant: register_response.data.as_ref().map(|data| data.tenant.clone()),
                    tenant_id: None,
                    username: register_response.data.as_ref().map(|data| data.username.clone()),
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
                    if let Some(next_token) = response.data.as_ref().map(|data| data.token.clone()) {
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
                config.set_token(token.clone());
                update_machine_auth_from_verify_response(
                    config,
                    response.data.as_ref(),
                    &token,
                    args.save_public_key_path.as_deref(),
                    args.save_private_key_path.as_deref(),
                )?;
                save_config(config, save_path)?;
            }
            print_json(&response)?;
        }
        crate::cli::AuthSubcommand::Dissolve(command) => {
            auth_dissolve(command, client).await?;
        }
        crate::cli::AuthSubcommand::Token(command) => auth_token(command, config, save_path).await?,
        crate::cli::AuthSubcommand::Tenants => {
            print_json(&client.auth_tenants().await?)?;
        }
    }
    Ok(())
}

async fn docs(command: DocsCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::DocsSubcommand::Root => print_text(&client.get_text("/docs").await?)?,
        crate::cli::DocsSubcommand::Path { path } => {
            let path = path.unwrap_or_else(|| "/docs".to_string());
            print_text(&client.get_text(&path).await?)?;
        }
    }
    Ok(())
}

async fn describe(command: DescribeCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::DescribeSubcommand::List => {
            print_json(&client.get_json::<Value>("/api/describe").await?)?
        }
        crate::cli::DescribeSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/describe/{}", arg.model))
                .await?,
        )?,
        crate::cli::DescribeSubcommand::Create(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/describe/{}", arg.model),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DescribeSubcommand::Update(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/describe/{}", arg.model),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DescribeSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/describe/{}", arg.model))
                .await?,
        )?,
        crate::cli::DescribeSubcommand::Fields(fields) => describe_fields(fields, client).await?,
    }
    Ok(())
}

async fn describe_fields(
    command: crate::cli::DescribeFieldsCommand,
    client: &ApiClient,
) -> anyhow::Result<()> {
    match command.command {
        crate::cli::DescribeFieldsSubcommand::List(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/describe/{}/fields", arg.model))
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::BulkCreate(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/describe/{}/fields", arg.model),
                    &read_json_body_or_default(json!([]))?,
                )
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::BulkUpdate(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/describe/{}/fields", arg.model),
                    &read_json_body_or_default(json!([]))?,
                )
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/describe/{}/fields/{}", arg.model, arg.field))
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::Create(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/describe/{}/fields/{}", arg.model, arg.field),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::Update(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/describe/{}/fields/{}", arg.model, arg.field),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/describe/{}/fields/{}", arg.model, arg.field))
                .await?,
        )?,
    }
    Ok(())
}

async fn data(command: DataCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::DataSubcommand::List(arg) => print_json(
            &client
                .get_json_with_query::<_, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data::query_pairs(&command.options),
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Create(arg) => print_json(
            &client
                .post_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data::query_pairs(&command.options),
                    &read_json_body_or_default(json!([]))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Update(arg) => print_json(
            &client
                .put_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data::query_pairs(&command.options),
                    &read_json_body_or_default(json!([]))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Patch(arg) => print_json(
            &client
                .patch_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data::query_pairs(&command.options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Delete(arg) => print_json(
            &client
                .delete_json_with_query::<_, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data::query_pairs(&command.options),
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Get(arg) => print_json(
            &client
                .get_json_with_query::<_, Value>(
                    &format!("/api/data/{}/{}", arg.model, arg.id),
                    &data::query_pairs(&command.options),
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Put(arg) => print_json(
            &client
                .put_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}/{}", arg.model, arg.id),
                    &data::query_pairs(&command.options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::PatchRecord(arg) => print_json(
            &client
                .patch_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}/{}", arg.model, arg.id),
                    &data::query_pairs(&command.options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::DeleteRecord(arg) => print_json(
            &client
                .delete_json_with_query::<_, Value>(
                    &format!("/api/data/{}/{}", arg.model, arg.id),
                    &data::query_pairs(&command.options),
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Relationship(arg) => {
            relationship(arg, client, &command.options).await?
        }
    }
    Ok(())
}

async fn relationship(
    command: crate::cli::RelationshipArg,
    client: &ApiClient,
    options: &crate::cli::DataOptions,
) -> anyhow::Result<()> {
    let base = format!(
        "/api/data/{}/{}/{}",
        command.model, command.id, command.relationship
    );
    match command.command {
        crate::cli::RelationshipSubcommand::Get => print_json::<Value>(
            &client
                .get_json_with_query::<_, Value>(&base, &data::query_pairs(options))
                .await?,
        )?,
        crate::cli::RelationshipSubcommand::Create => print_json(
            &client
                .post_json_with_query::<_, _, Value>(
                    &base,
                    &data::query_pairs(options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::RelationshipSubcommand::Update => print_json(
            &client
                .put_json_with_query::<_, _, Value>(
                    &base,
                    &data::query_pairs(options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::RelationshipSubcommand::Delete => print_json::<Value>(
            &client
                .delete_json_with_query::<_, Value>(&base, &data::query_pairs(options))
                .await?,
        )?,
        crate::cli::RelationshipSubcommand::Child(child) => {
            let path = format!("{}/{}", base, child.child);
            match child.command {
                crate::cli::RelationshipChildSubcommand::Get => {
                    print_json::<Value>(
                        &client
                            .get_json_with_query::<_, Value>(&path, &data::query_pairs(options))
                            .await?,
                    )?;
                }
                crate::cli::RelationshipChildSubcommand::Put => {
                    print_json::<Value>(
                        &client
                            .put_json_with_query::<_, _, Value>(
                                &path,
                                &data::query_pairs(options),
                                &read_json_body_or_default(json!({}))?,
                            )
                            .await?,
                    )?;
                }
                crate::cli::RelationshipChildSubcommand::Patch => {
                    print_json::<Value>(
                        &client
                            .patch_json_with_query::<_, _, Value>(
                                &path,
                                &data::query_pairs(options),
                                &read_json_body_or_default(json!({}))?,
                            )
                            .await?,
                    )?;
                }
                crate::cli::RelationshipChildSubcommand::Delete => {
                    print_json::<Value>(
                        &client
                            .delete_json_with_query::<_, Value>(&path, &data::query_pairs(options))
                            .await?,
                    )?;
                }
            }
        }
    }
    Ok(())
}

async fn find(command: FindCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::FindSubcommand::Query(arg) => {
            let body = find_query_body(&command.options)?;
            print_json(
                &client
                    .post_json::<_, Value>(&format!("/api/find/{}", arg.model), &body)
                    .await?,
            )?
        }
        crate::cli::FindSubcommand::Saved(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/find/{}/{}", arg.model, arg.target))
                .await?,
        )?,
    }
    Ok(())
}

async fn aggregate(command: AggregateCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::AggregateSubcommand::Get(arg) => {
            let query = aggregate_query(&command.options)?;
            print_json(
                &client
                    .get_json_with_query::<_, Value>(
                        &format!("/api/aggregate/{}", arg.model),
                        &query,
                    )
                    .await?,
            )?
        }
        crate::cli::AggregateSubcommand::Run(arg) => {
            let body = aggregate_body(&command.options)?;
            print_json(
                &client
                    .post_json::<_, Value>(&format!("/api/aggregate/{}", arg.model), &body)
                    .await?,
            )?
        }
    }
    Ok(())
}

async fn bulk(command: BulkCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::BulkSubcommand::Run => {
            let body = read_json_source_or_default(
                command.options.body.as_deref(),
                json!({"operations": []}),
            )?;
            let body = bulk_body(body)?;
            print_json(&client.post_json::<_, Value>("/api/bulk", &body).await?)?
        }
        crate::cli::BulkSubcommand::Create(arg) => {
            let body = bulk_model_body(&command.options, &arg.model, "create-all")?;
            print_json(&client.post_json::<_, Value>("/api/bulk", &body).await?)?
        }
        crate::cli::BulkSubcommand::Update(arg) => {
            let body = bulk_model_body(&command.options, &arg.model, "update-all")?;
            print_json(&client.post_json::<_, Value>("/api/bulk", &body).await?)?
        }
        crate::cli::BulkSubcommand::Delete(arg) => {
            let body = bulk_model_body(&command.options, &arg.model, "delete-all")?;
            print_json(&client.post_json::<_, Value>("/api/bulk", &body).await?)?
        }
        crate::cli::BulkSubcommand::Export => print_json(
            &client
                .post_json::<_, Value>("/api/bulk/export", &read_json_body_or_default(json!({}))?)
                .await?,
        )?,
        crate::cli::BulkSubcommand::Import => print_json(
            &client
                .post_json::<_, Value>("/api/bulk/import", &read_json_body_or_default(json!({}))?)
                .await?,
        )?,
    }
    Ok(())
}

async fn acls(command: AclsCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::AclsSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/acls/{}/{}", arg.model, arg.id))
                .await?,
        )?,
        crate::cli::AclsSubcommand::Create(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/acls/{}/{}", arg.model, arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::AclsSubcommand::Update(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/acls/{}/{}", arg.model, arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::AclsSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/acls/{}/{}", arg.model, arg.id))
                .await?,
        )?,
    }
    Ok(())
}

async fn stat(command: StatCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::StatSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/stat/{}/{}", arg.model, arg.id))
                .await?,
        )?,
    }
    Ok(())
}

async fn tracked(command: TrackedCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::TrackedSubcommand::List(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/tracked/{}/{}", arg.model, arg.id))
                .await?,
        )?,
        crate::cli::TrackedSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!(
                    "/api/tracked/{}/{}/{}",
                    arg.model, arg.id, arg.change
                ))
                .await?,
        )?,
    }
    Ok(())
}

async fn trashed(command: TrashedCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::TrashedSubcommand::List => {
            print_json(&client.get_json::<Value>("/api/trashed").await?)?
        }
        crate::cli::TrashedSubcommand::Model(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/trashed/{}", arg.model))
                .await?,
        )?,
        crate::cli::TrashedSubcommand::Record(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/trashed/{}/{}", arg.model, arg.id))
                .await?,
        )?,
    }
    Ok(())
}

async fn user(command: UserCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        UserSubcommand::Me => print_json(&client.get_json::<Value>("/api/user/me").await?)?,
        UserSubcommand::List(args) => {
            let query = user_list_query(&args);
            print_json(
                &client
                    .get_json_with_query::<_, Value>("/api/user", &query)
                    .await?,
            )?
        }
        UserSubcommand::Create(args) => {
            let body = user_create_body(args)?;
            print_json(&client.post_json::<_, Value>("/api/user", &body).await?)?
        }
        UserSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/user/{}", arg.id))
                .await?,
        )?,
        UserSubcommand::Update(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/user/{}", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        UserSubcommand::Delete(args) => {
            let body = json!({
                "confirm": args.confirm,
                "reason": args.reason,
            });
            print_json::<Value>(
                &client
                    .request_json(
                        Method::DELETE,
                        &format!("/api/user/{}", args.id),
                        Some(&body),
                    )
                    .await?,
            )?;
        }
        UserSubcommand::Password(args) => {
            let body = user_password_body(args)?;
            print_json::<Value>(
                &client
                    .post_json::<_, Value>(&format!("/api/user/{}/password", body.id), &body.body)
                    .await?,
            )?;
        }
        UserSubcommand::Sudo(args) => print_json(&client.auth_sudo(args.reason.as_deref()).await?)?,
        UserSubcommand::Fake(args) => {
            let body = json!({
                "user_id": args.user_id,
                "username": args.username,
            });
            print_json::<Value>(
                &client
                    .post_json::<_, Value>("/api/user/fake", &body)
                    .await?,
            )?
        }
    }
    Ok(())
}

async fn keys(command: KeysCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        KeysSubcommand::List => print_json(&client.get_json::<Value>("/api/keys").await?)?,
        KeysSubcommand::Create(args) => {
            let body = keys_create_body(args)?;
            print_json(&client.post_json::<_, Value>("/api/keys", &body).await?)?;
        }
        KeysSubcommand::Rotate(args) => {
            let body = keys_rotate_body(args)?;
            print_json(&client.post_json::<_, Value>("/api/keys/rotate", &body).await?)?;
        }
        KeysSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/keys/{}", arg.key_id))
                .await?,
        )?,
    }
    Ok(())
}

async fn cron(command: CronCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::CronSubcommand::List => {
            print_json(&client.get_json::<Value>("/api/cron").await?)?
        }
        crate::cli::CronSubcommand::Create => print_json(
            &client
                .post_json::<_, Value>("/api/cron", &read_json_body_or_default(json!({}))?)
                .await?,
        )?,
        crate::cli::CronSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/cron/{}", arg.pid))
                .await?,
        )?,
        crate::cli::CronSubcommand::Update(arg) => print_json(
            &client
                .patch_json::<_, Value>(
                    &format!("/api/cron/{}", arg.pid),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::CronSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/cron/{}", arg.pid))
                .await?,
        )?,
        crate::cli::CronSubcommand::Enable(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/cron/{}/enable", arg.pid),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::CronSubcommand::Disable(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/cron/{}/disable", arg.pid),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
    }
    Ok(())
}

async fn fs(command: FsCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::FsSubcommand::Get(arg) => {
            if command.options.stat {
                print_json(
                    &client
                        .get_json::<Value>(&format!("/fs/{}?stat=true", arg.path))
                        .await?,
                )?
            } else {
                print_text(&client.get_text(&format!("/fs/{}", arg.path)).await?)?
            }
        }
        crate::cli::FsSubcommand::Put(arg) => {
            let content = fs_body_text(&command.options)?;
            print_text(
                &client
                    .request_text(Method::PUT, &format!("/fs/{}", arg.path), Some(&content))
                    .await?,
            )?;
        }
        crate::cli::FsSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/fs/{}", arg.path))
                .await?,
        )?,
    }
    Ok(())
}

async fn app(command: AppCommand, client: &ApiClient) -> anyhow::Result<()> {
    let path = command.path.unwrap_or_default();
    let full_path = if path.is_empty() {
        format!("/app/{}", command.app_name)
    } else {
        format!("/app/{}/{}", command.app_name, path.trim_start_matches('/'))
    };
    print_text(&client.get_text(&full_path).await?)?;
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> anyhow::Result<()> {
    let text = serde_json::to_string_pretty(value)?;
    println!("{text}");
    Ok(())
}

fn print_text(value: &str) -> anyhow::Result<()> {
    println!("{value}");
    Ok(())
}

async fn auth_dissolve(command: crate::cli::AuthDissolveCommand, client: &ApiClient) -> anyhow::Result<()> {
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

fn save_config(config: &AbbotikConfig, save_path: Option<&Path>) -> anyhow::Result<()> {
    if let Some(path) = save_path {
        config.save_to_path(path)?;
    }
    Ok(())
}

fn read_stdin_or_empty() -> anyhow::Result<String> {
    if io::stdin().is_terminal() {
        return Ok(String::new());
    }

    let mut buffer = String::new();
    let mut stdin = io::stdin();
    if stdin.read_to_string(&mut buffer).is_ok() && !buffer.trim().is_empty() {
        return Ok(buffer);
    }
    Ok(String::new())
}

fn read_json_body_or_default(default: Value) -> anyhow::Result<Value> {
    let raw = read_stdin_or_empty()?;
    if raw.trim().is_empty() {
        return Ok(default);
    }

    Ok(serde_json::from_str(&raw)?)
}

fn read_json_source_or_default(source: Option<&str>, default: Value) -> anyhow::Result<Value> {
    match source {
        Some(source) if !source.is_empty() => read_json_source(source),
        _ => Ok(default),
    }
}

fn read_json_source(source: &str) -> anyhow::Result<Value> {
    let raw = if source == "-" {
        read_stdin_or_empty()?
    } else if let Some(path) = source.strip_prefix('@') {
        fs::read_to_string(path)?
    } else {
        source.to_string()
    };

    if raw.trim().is_empty() {
        return Ok(Value::Null);
    }

    Ok(serde_json::from_str(&raw)?)
}

fn fs_body_text(options: &FsOptions) -> anyhow::Result<String> {
    match options.body.as_deref() {
        Some("-") => read_stdin_or_empty(),
        Some(source) if source.starts_with('@') => Ok(fs::read_to_string(&source[1..])?),
        Some(source) => Ok(source.to_string()),
        None => read_stdin_or_empty(),
    }
}

fn read_secret_source_option(source: Option<&str>) -> anyhow::Result<Option<String>> {
    source.map(read_secret_source).transpose()
}

fn read_secret_source(source: &str) -> anyhow::Result<String> {
    let raw = if source == "-" {
        read_stdin_or_empty()?
    } else if let Some(path) = source.strip_prefix('@') {
        fs::read_to_string(path)?
    } else {
        source.to_string()
    };

    Ok(trim_one_trailing_newline(raw))
}

fn trim_one_trailing_newline(mut value: String) -> String {
    if value.ends_with('\n') {
        value.pop();
        if value.ends_with('\r') {
            value.pop();
        }
    }
    value
}

fn bulk_body(body: Value) -> anyhow::Result<Value> {
    match body {
        Value::Object(map) => {
            if map.contains_key("operations") {
                Ok(Value::Object(map))
            } else {
                Err(anyhow::anyhow!(
                    "bulk body must contain an operations array"
                ))
            }
        }
        other => Err(anyhow::anyhow!(
            "bulk body must be a JSON object, got {other}"
        )),
    }
}

fn bulk_model_body(options: &BulkOptions, model: &str, operation: &str) -> anyhow::Result<Value> {
    let body = match options.body.as_deref() {
        Some(source) => read_json_source(source)?,
        None => read_json_body_or_default(json!([]))?,
    };

    if !body.is_array() {
        return Err(anyhow::anyhow!("bulk model body must be a JSON array"));
    }

    Ok(json!({
        "operations": [
            {
                "operation": operation,
                "model": model,
                "data": body,
            }
        ]
    }))
}

fn aggregate_query(options: &AggregateOptions) -> anyhow::Result<Vec<(String, String)>> {
    let mut query = Vec::new();
    if options.count {
        query.push(("count".to_string(), String::new()));
    }
    if let Some(sum) = &options.sum {
        query.push(("sum".to_string(), sum.clone()));
    }
    if let Some(avg) = &options.avg {
        query.push(("avg".to_string(), avg.clone()));
    }
    if let Some(min) = &options.min {
        query.push(("min".to_string(), min.clone()));
    }
    if let Some(max) = &options.max {
        query.push(("max".to_string(), max.clone()));
    }
    if let Some(where_source) = &options.r#where {
        let value = read_json_source(where_source)?;
        query.push(("where".to_string(), serde_json::to_string(&value)?));
    }
    Ok(query)
}

fn aggregate_body(options: &AggregateOptions) -> anyhow::Result<Value> {
    if let Some(body_source) = &options.body {
        let body = read_json_source(body_source)?;
        if !body.is_object() {
            return Err(anyhow::anyhow!("aggregate body must be a JSON object"));
        }
        return Ok(body);
    }

    let mut object = serde_json::Map::new();
    if let Some(where_source) = &options.r#where {
        object.insert("where".to_string(), read_json_source(where_source)?);
    }

    let mut aggregate = serde_json::Map::new();
    if options.count {
        aggregate.insert("count".to_string(), json!({"$count": "*"}));
    }
    if let Some(sum) = &options.sum {
        aggregate.insert("sum".to_string(), json!({"$sum": sum}));
    }
    if let Some(avg) = &options.avg {
        aggregate.insert("avg".to_string(), json!({"$avg": avg}));
    }
    if let Some(min) = &options.min {
        aggregate.insert("min".to_string(), json!({"$min": min}));
    }
    if let Some(max) = &options.max {
        aggregate.insert("max".to_string(), json!({"$max": max}));
    }
    if !aggregate.is_empty() {
        object.insert("aggregate".to_string(), Value::Object(aggregate));
    }

    if object.is_empty() {
        return Err(anyhow::anyhow!(
            "aggregate requires at least one flag or --body source"
        ));
    }

    Ok(Value::Object(object))
}

fn find_query_body(options: &FindOptions) -> anyhow::Result<Value> {
    let mut body = match options.r#where.as_deref() {
        Some(source) => read_json_source(source)?,
        None => Value::Object(serde_json::Map::new()),
    };

    let object = match &mut body {
        Value::Object(map) => map,
        Value::Null => {
            body = Value::Object(serde_json::Map::new());
            match &mut body {
                Value::Object(map) => map,
                _ => unreachable!(),
            }
        }
        other => {
            return Err(anyhow::anyhow!(
                "find where JSON must be an object, got {other}"
            ));
        }
    };

    if let Some(select) = &options.select {
        object.insert(
            "select".to_string(),
            Value::Array(
                select
                    .split(',')
                    .map(|part| part.trim())
                    .filter(|part| !part.is_empty())
                    .map(|part| Value::String(part.to_string()))
                    .collect(),
            ),
        );
    }
    if let Some(order) = &options.order {
        object.insert(
            "order".to_string(),
            Value::Array(
                order
                    .split(',')
                    .map(|part| part.trim())
                    .filter(|part| !part.is_empty())
                    .map(|part| Value::String(part.to_string()))
                    .collect(),
            ),
        );
    }
    if let Some(limit) = options.limit {
        object.insert("limit".to_string(), Value::Number(limit.into()));
    }
    if let Some(offset) = options.offset {
        object.insert("offset".to_string(), Value::Number(offset.into()));
    }

    Ok(body)
}

fn user_list_query(args: &UserListCommand) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(limit) = args.limit {
        query.push(("limit".to_string(), limit.to_string()));
    }
    if let Some(offset) = args.offset {
        query.push(("offset".to_string(), offset.to_string()));
    }
    query
}

fn user_create_body(args: UserCreateCommand) -> anyhow::Result<Value> {
    if let Some(body) = args.body {
        return Ok(serde_json::from_str(&body)?);
    }

    let mut object = Map::new();
    if let Some(name) = args.name {
        object.insert("name".to_string(), Value::String(name));
    }
    if let Some(auth) = args.auth {
        object.insert("auth".to_string(), Value::String(auth));
    }
    if let Some(access) = args.access {
        object.insert("access".to_string(), Value::String(access));
    }

    if object.is_empty() {
        return read_json_body_or_default(json!({}));
    }

    Ok(Value::Object(object))
}

struct PasswordBody {
    id: String,
    body: Value,
}

fn user_password_body(args: UserPasswordCommand) -> anyhow::Result<PasswordBody> {
    let mut object = Map::new();
    if let Some(current_password) = args.current_password {
        object.insert(
            "current_password".to_string(),
            Value::String(current_password),
        );
    }
    if let Some(new_password) = args.new_password {
        object.insert("new_password".to_string(), Value::String(new_password));
    }

    if object.is_empty() {
        return Ok(PasswordBody {
            id: args.id,
            body: read_json_body_or_default(json!({}))?,
        });
    }

    Ok(PasswordBody {
        id: args.id,
        body: Value::Object(object),
    })
}

fn keys_create_body(args: KeysCreateCommand) -> anyhow::Result<Value> {
    let mut object = Map::new();
    if let Some(user_id) = args.user_id {
        object.insert("user_id".to_string(), Value::String(user_id));
    }
    if let Some(public_key) = read_secret_source_option(args.public_key.as_deref())? {
        object.insert("public_key".to_string(), Value::String(public_key));
    }
    if let Some(name) = args.name {
        object.insert("name".to_string(), Value::String(name));
    }
    if let Some(algorithm) = args.algorithm {
        object.insert("algorithm".to_string(), Value::String(algorithm));
    }
    if let Some(expires_at) = args.expires_at {
        object.insert("expires_at".to_string(), Value::String(expires_at));
    }
    Ok(Value::Object(object))
}

fn keys_rotate_body(args: KeysRotateCommand) -> anyhow::Result<Value> {
    let mut object = Map::new();
    if let Some(key_id) = args.key_id {
        object.insert("key_id".to_string(), Value::String(key_id));
    }
    if let Some(new_public_key) = read_secret_source_option(args.new_public_key.as_deref())? {
        object.insert("new_public_key".to_string(), Value::String(new_public_key));
    }
    if let Some(algorithm) = args.algorithm {
        object.insert("algorithm".to_string(), Value::String(algorithm));
    }
    if let Some(new_name) = args.new_name {
        object.insert("new_name".to_string(), Value::String(new_name));
    }
    if let Some(revoke_old_after_seconds) = args.revoke_old_after_seconds {
        object.insert(
            "revoke_old_after_seconds".to_string(),
            Value::Number(revoke_old_after_seconds.into()),
        );
    }
    Ok(Value::Object(object))
}
