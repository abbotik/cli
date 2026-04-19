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
mod app;
mod auth;
mod auth_support;
mod bulk;
mod config_cmd;
mod cron;
mod data;
mod describe;
mod docs;
mod doctor;
mod find;
mod fs;
mod io;
mod keys;
mod llm;
mod public;
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
    let selected_profile = AbbotikConfig::selected_profile(cli.globals.config.as_deref());
    let mut config = AbbotikConfig::load_effective(selected_profile.as_deref())?;
    let save_path = AbbotikConfig::config_path(selected_profile.as_deref()).ok();

    if let Some(base_url) = cli.globals.base_url.as_ref() {
        config.base_url = base_url.clone();
    }
    if let Some(token) = cli.globals.token.as_ref() {
        config.token = Some(token.clone());
    }
    if let Some(format) = cli.globals.format.as_ref() {
        config.output_format = format.clone();
    }

    let client = ApiClient::new(config.clone())?;

    match cli.command {
        Command::Public(command) => public::run(command, &client).await?,
        Command::Auth(command) => {
            auth::run(command, &mut config, &client, save_path.as_deref()).await?
        }
        Command::Health => print_json(&client.health().await?)?,
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
                save_path.as_deref(),
            )
            .await?
        }
        Command::Docs(command) => docs::run(command, &client).await?,
        Command::Describe(command) => describe::run(command, &client).await?,
        Command::Data(command) => data::run(command, &client).await?,
        Command::Find(command) => find::run(command, &client).await?,
        Command::Aggregate(command) => aggregate::run(command, &client).await?,
        Command::Bulk(command) => bulk::run(command, &client).await?,
        Command::Acls(command) => acls::run(command, &client).await?,
        Command::Stat(command) => stat::run(command, &client).await?,
        Command::Tracked(command) => tracked::run(command, &client).await?,
        Command::Trashed(command) => trashed::run(command, &client).await?,
        Command::User(command) => user::run(command, &client).await?,
        Command::Keys(command) => keys::run(command, &client).await?,
        Command::Llm(command) => llm::run(command, &client).await?,
        Command::Cron(command) => cron::run(command, &client).await?,
        Command::Fs(command) => fs::run(command, &client).await?,
        Command::App(command) => app::run(command, &client).await?,
        Command::Tui(command) => tui::run(command, &client).await?,
        Command::Update(command) => update::run(command).await?,
    }

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

fn save_config(config: &AbbotikConfig, save_path: Option<&Path>) -> anyhow::Result<()> {
    if let Some(path) = save_path {
        config.save_to_path(path)?;
    }
    Ok(())
}
