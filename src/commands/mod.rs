pub(super) use std::{
    fs as stdfs,
    io::{self as stdio, IsTerminal, Read},
    path::{Path, PathBuf},
};

pub(super) use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
pub(super) use ed25519_dalek::{
    pkcs8::{DecodePrivateKey, EncodePublicKey},
    Signer, SigningKey,
};
pub(super) use reqwest::Method;
pub(super) use serde::Deserialize;
pub(super) use serde_json::{json, Map, Value};

pub(super) use crate::{
    api::{
        ApiClient, ChallengeRequest, DissolveConfirmRequest, DissolveRequest, InviteRequest,
        LoginRequest, ProvisionRequest, RefreshRequest, RegisterRequest, VerifyData, VerifyRequest,
    },
    cli::*,
    config::AbbotikConfig,
    data as data_helpers,
};

mod acls;
mod aggregate;
mod api_cmd;
mod auth;
mod auth_support;
mod bulk;
mod command;
mod config_cmd;
mod cron;
mod data;
mod describe;
mod docs;
mod doctor;
mod factory;
mod find;
mod io;
mod keys;
mod llm;
mod mcp;
mod shared;
mod stat;
mod tracked;
mod trashed;
mod tui;
mod update;
mod user;

use self::io::{
    read_json_body_or_default, read_json_source, read_json_source_or_default, read_secret_source,
    read_secret_source_option, read_stdin_or_empty,
};
use self::shared::vec_or_none;

pub async fn run(cli: Cli) -> anyhow::Result<()> {
    let explicit_profile = explicit_profile(cli.globals.config.as_deref());
    let global_host = global_host(&cli.globals)?;
    let command_host = auth_command_host(&cli.command);
    if explicit_profile.is_some() && command_host.is_some() {
        anyhow::bail!("`--config` cannot be combined with an auth host argument");
    }

    let (mut config, save_path, selected_profile, selected_host) =
        if let Some(profile) = explicit_profile {
            let mut config = AbbotikConfig::load_effective(Some(&profile))?;
            if let Some(host) = global_host.as_ref() {
                config.base_url = host.clone();
            }
            let save_path = AbbotikConfig::config_path(Some(&profile)).ok();
            (config, save_path, Some(profile), None)
        } else {
            let host = selected_host_for_command(&cli.command, global_host.as_deref())?;
            let config = AbbotikConfig::load_host_effective(&host)?;
            let save_path = AbbotikConfig::host_config_path(&host).ok();
            (config, save_path, None, Some(host))
        };

    if let Some(token) = cli.globals.token.as_ref() {
        config.token = Some(token.clone());
    }
    if let Some(format) = cli.globals.format.as_ref() {
        config.output_format = format.clone();
    }

    let token_overridden =
        cli.globals.token.is_some() || std::env::var_os("ABBOTIK_API_TOKEN").is_some();
    let mut client = ApiClient::new(config.clone())?;
    if !token_overridden
        && should_auto_refresh_token(&cli.command)
        && auth::refresh_saved_token_if_needed(&client, &mut config, save_path.as_deref()).await?
    {
        client = ApiClient::new(config.clone())?;
    }

    match cli.command {
        Command::Api(command) => api_cmd::run(command, &client).await?,
        Command::Auth(command) => {
            auth::run(
                command,
                &mut config,
                &client,
                save_path.as_deref(),
                selected_host.as_deref(),
            )
            .await?
        }
        Command::Config(command) => config_cmd::run(
            command,
            &config,
            selected_profile.as_deref(),
            save_path.as_deref(),
        )?,
        Command::Doctor(command) => {
            doctor::run(
                command,
                &client,
                &config,
                selected_profile.as_deref(),
                selected_host.as_deref(),
                save_path.as_deref(),
            )
            .await?
        }
        Command::Docs(command) => docs::run(command, &client).await?,
        Command::Guide(command) => command::run(command).await?,
        Command::Llm(command) => llm::run(command, &client).await?,
        Command::Mcp(command) => mcp::run(command, &client).await?,
        Command::Factory(command) => factory::run(command, &client).await?,
        Command::Tui(command) => tui::run(command, &client).await?,
        Command::Update(command) => update::run(command).await?,
    }

    Ok(())
}

fn should_auto_refresh_token(command: &Command) -> bool {
    !matches!(
        command,
        Command::Auth(_)
            | Command::Config(_)
            | Command::Doctor(_)
            | Command::Guide(_)
            | Command::Update(_)
    )
}

fn explicit_profile(cli_profile: Option<&str>) -> Option<String> {
    cli_profile
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .or_else(|| {
            std::env::var("ABBOTIK_CONFIG")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        })
}

fn global_host(globals: &GlobalOptions) -> anyhow::Result<Option<String>> {
    match (globals.host.as_deref(), globals.base_url.as_deref()) {
        (Some(host), Some(base_url)) => {
            let host = AbbotikConfig::normalize_host(host)?;
            let base_url = AbbotikConfig::normalize_host(base_url)?;
            if host != base_url {
                anyhow::bail!("`--host` and `--base-url` target different hosts");
            }
            Ok(Some(host))
        }
        (Some(host), None) | (None, Some(host)) => Ok(Some(AbbotikConfig::normalize_host(host)?)),
        (None, None) => Ok(std::env::var("ABBOTIK_API_BASE_URL")
            .ok()
            .map(|value| AbbotikConfig::normalize_host(&value))
            .transpose()?),
    }
}

fn auth_command_host(command: &Command) -> Option<&str> {
    match command {
        Command::Auth(command) => match &command.command {
            AuthSubcommand::Login(args) => args.host.as_deref(),
            AuthSubcommand::Use(args) => Some(args.host.as_str()),
            AuthSubcommand::Logout(args) => args.host.as_deref(),
            _ => None,
        },
        _ => None,
    }
}

fn selected_host_for_command(
    command: &Command,
    global_host: Option<&str>,
) -> anyhow::Result<String> {
    let command_host = auth_command_host(command)
        .map(AbbotikConfig::normalize_host)
        .transpose()?;
    let global_host = global_host.map(AbbotikConfig::normalize_host).transpose()?;
    if let (Some(command_host), Some(global_host)) = (&command_host, &global_host) {
        if command_host != global_host {
            anyhow::bail!("auth host argument and `--host` target different hosts");
        }
    }

    if let Some(host) = command_host.or(global_host) {
        return Ok(host);
    }

    if matches!(
        command,
        Command::Auth(AuthCommand {
            command: AuthSubcommand::Login(_)
        })
    ) {
        return Ok(AbbotikConfig::default_host());
    }

    Ok(AbbotikConfig::load_current_host()?.unwrap_or_else(AbbotikConfig::default_host))
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

fn save_config(config: &AbbotikConfig, save_path: Option<&Path>) -> anyhow::Result<()> {
    if let Some(path) = save_path {
        config.save_to_path(path)?;
    }
    Ok(())
}
