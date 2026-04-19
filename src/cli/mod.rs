use clap::{Args, Parser, Subcommand};

use crate::config::OutputFormat;

mod acls;
mod aggregate;
mod app;
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
mod find;
mod fs;
mod keys;
mod llm;
mod public;
mod stat;
mod tracked;
mod trashed;
mod tui;
mod update;
mod user;

pub use acls::*;
pub use aggregate::*;
pub use app::*;
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
pub use find::*;
pub use fs::*;
pub use keys::*;
pub use llm::*;
pub use public::*;
pub use stat::*;
pub use tracked::*;
pub use trashed::*;
pub use tui::*;
pub use update::*;
pub use user::*;

#[derive(Parser, Debug)]
#[command(
    name = "abbot",
    about = "CLI for the Abbotik API at https://api.abbotik.com"
)]
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
    /// Authentication and tenant bootstrap
    Auth(AuthCommand),
    /// Show the active CLI config summary
    Config(ConfigCommand),
    /// Model data operations
    Data(DataCommand),
    /// Model metadata and schema management
    Describe(DescribeCommand),
    /// Record ACL management
    Acls(AclsCommand),
    /// Aggregate operations
    Aggregate(AggregateCommand),
    /// Dynamic app packages
    App(AppCommand),
    /// Multi-operation transactions
    Bulk(BulkCommand),
    /// Read embedded markdown docs for a command path
    Command(CommandDocsCommand),
    /// Scheduled process workflows
    Cron(CronCommand),
    /// API documentation helpers
    Docs(DocsCommand),
    /// Explain auth and config state for the active profile
    Doctor(DoctorCommand),
    /// Advanced query operations
    Find(FindCommand),
    /// Tenant filesystem workflows
    Fs(FsCommand),
    /// Health checks
    Health,
    /// Self-service bearer API key management
    Keys(KeysCommand),
    /// LLM rooms, factory runs, and provider discovery
    Llm(LlmCommand),
    /// Public surfaces and discovery
    Public(PublicCommand),
    /// Record metadata
    Stat(StatCommand),
    /// Change tracking
    Tracked(TrackedCommand),
    /// Soft-delete and restore workflows
    Trashed(TrashedCommand),
    /// Terminal operator console for rooms and factory runs
    Tui(TuiCommand),
    /// Update this CLI to the latest release
    Update(UpdateCommand),
    /// User, machine-key, and sudo workflows
    User(UserCommand),
}
