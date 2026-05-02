use clap::{Args, Parser, Subcommand};

use crate::config::OutputFormat;

mod acls;
mod aggregate;
mod api_cmd;
mod args;
mod auth;
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
mod keys;
mod llm;
mod mcp;
mod stat;
mod tracked;
mod trashed;
mod tui;
mod update;
mod user;

pub use acls::*;
pub use aggregate::*;
pub use api_cmd::*;
pub use args::*;
pub use auth::*;
pub use bulk::*;
pub use command::*;
pub use config_cmd::*;
pub use cron::*;
pub use data::*;
pub use describe::*;
pub use docs::*;
pub use doctor::*;
pub use factory::*;
pub use find::*;
pub use keys::*;
pub use llm::*;
pub use mcp::*;
pub use stat::*;
pub use tracked::*;
pub use trashed::*;
pub use tui::*;
pub use update::*;
pub use user::*;

#[derive(Parser, Debug)]
#[command(name = "abbot", version, about = "Operator CLI for Abbotik")]
pub struct Cli {
    #[command(flatten)]
    pub globals: GlobalOptions,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug, Default)]
pub struct GlobalOptions {
    /// Use a named config profile stored under ~/.config/abbot/cli/configs/<name>.toml
    #[arg(long)]
    pub config: Option<String>,

    /// Override the Abbotik API base URL
    #[arg(long = "base-url")]
    pub base_url: Option<String>,

    /// Override the stored bearer token
    #[arg(long)]
    pub token: Option<String>,

    /// Override the preferred response format (json only)
    #[arg(long, value_enum)]
    pub format: Option<OutputFormat>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Route-shaped access to /api/<name> families
    Api(ApiCommand),
    /// Authentication and tenant bootstrap
    Auth(AuthCommand),
    /// Show the active CLI config summary
    Config(ConfigCommand),
    /// API documentation helpers
    Docs(DocsCommand),
    /// Explain auth and config state for the active profile
    Doctor(DoctorCommand),
    /// High-level durable factory workflow operations
    Factory(FactoryCommand),
    /// Read embedded command guide markdown
    Guide(CommandDocsCommand),
    /// LLM rooms, factory runs, and provider discovery
    Llm(LlmCommand),
    /// MCP tool listing and tool calls
    Mcp(McpCommand),
    /// Terminal operator console for rooms and factory runs
    Tui(TuiCommand),
    /// Update this CLI to the latest release
    Update(UpdateCommand),
}
