use clap::{Args, Parser, Subcommand};

mod acls;
mod aggregate;
mod app;
mod args;
mod auth;
mod bulk;
mod cron;
mod data;
mod describe;
mod docs;
mod find;
mod fs;
mod keys;
mod llm;
mod public;
mod stat;
mod tracked;
mod trashed;
mod user;

pub use acls::*;
pub use aggregate::*;
pub use app::*;
pub use args::*;
pub use auth::*;
pub use bulk::*;
pub use cron::*;
pub use data::*;
pub use describe::*;
pub use docs::*;
pub use find::*;
pub use fs::*;
pub use keys::*;
pub use llm::*;
pub use public::*;
pub use stat::*;
pub use tracked::*;
pub use trashed::*;
pub use user::*;

const CLI_LONG_ABOUT: &str = include_str!("../../docs/help/cli-long-about.md");
const CLI_AFTER_HELP: &str = include_str!("../../docs/help/cli-after-help.md");
const COMMAND_AFTER_HELP: &str = include_str!("../../docs/help/command-after-help.md");
const PUBLIC_AFTER_HELP: &str = include_str!("../../docs/help/public-after-help.md");
const PUBLIC_ROOT_AFTER_HELP: &str = include_str!("../../docs/help/public-root-after-help.md");
const PUBLIC_LLMS_AFTER_HELP: &str = include_str!("../../docs/help/public-llms-after-help.md");
const AUTH_AFTER_HELP: &str = include_str!("../../docs/help/auth-after-help.md");
const AUTH_LOGIN_AFTER_HELP: &str = include_str!("../../docs/help/auth-login-after-help.md");
const AUTH_REGISTER_AFTER_HELP: &str = include_str!("../../docs/help/auth-register-after-help.md");
const AUTH_REFRESH_AFTER_HELP: &str = include_str!("../../docs/help/auth-refresh-after-help.md");
const AUTH_PROVISION_AFTER_HELP: &str =
    include_str!("../../docs/help/auth-provision-after-help.md");
const AUTH_CHALLENGE_AFTER_HELP: &str =
    include_str!("../../docs/help/auth-challenge-after-help.md");
const AUTH_VERIFY_AFTER_HELP: &str = include_str!("../../docs/help/auth-verify-after-help.md");
const AUTH_MACHINE_AFTER_HELP: &str = include_str!("../../docs/help/auth-machine-after-help.md");
const AUTH_MACHINE_CONNECT_AFTER_HELP: &str =
    include_str!("../../docs/help/auth-machine-connect-after-help.md");
const AUTH_DISSOLVE_AFTER_HELP: &str = include_str!("../../docs/help/auth-dissolve-after-help.md");
const AUTH_DISSOLVE_CONFIRM_AFTER_HELP: &str =
    include_str!("../../docs/help/auth-dissolve-confirm-after-help.md");
const AUTH_TOKEN_AFTER_HELP: &str = include_str!("../../docs/help/auth-token-after-help.md");
const AUTH_TENANTS_AFTER_HELP: &str = include_str!("../../docs/help/auth-tenants-after-help.md");
const DOCS_AFTER_HELP: &str = include_str!("../../docs/help/docs-after-help.md");
const DOCS_ROOT_AFTER_HELP: &str = include_str!("../../docs/help/docs-root-after-help.md");
const DOCS_PATH_AFTER_HELP: &str = include_str!("../../docs/help/docs-path-after-help.md");
const DESCRIBE_AFTER_HELP: &str = include_str!("../../docs/help/describe-after-help.md");
const DESCRIBE_FIELDS_AFTER_HELP: &str =
    include_str!("../../docs/help/describe-fields-after-help.md");
const DATA_AFTER_HELP: &str = include_str!("../../docs/help/data-after-help.md");
const DATA_RELATIONSHIP_AFTER_HELP: &str =
    include_str!("../../docs/help/data-relationship-after-help.md");
const DATA_RELATIONSHIP_CHILD_AFTER_HELP: &str =
    include_str!("../../docs/help/data-relationship-child-after-help.md");
const FIND_AFTER_HELP: &str = include_str!("../../docs/help/find-after-help.md");
const AGGREGATE_AFTER_HELP: &str = include_str!("../../docs/help/aggregate-after-help.md");
const BULK_AFTER_HELP: &str = include_str!("../../docs/help/bulk-after-help.md");
const ACLS_AFTER_HELP: &str = include_str!("../../docs/help/acls-after-help.md");
const STAT_AFTER_HELP: &str = include_str!("../../docs/help/stat-after-help.md");
const TRACKED_AFTER_HELP: &str = include_str!("../../docs/help/tracked-after-help.md");
const TRASHED_AFTER_HELP: &str = include_str!("../../docs/help/trashed-after-help.md");
const USER_AFTER_HELP: &str = include_str!("../../docs/help/user-after-help.md");
const USER_ME_AFTER_HELP: &str = include_str!("../../docs/help/user-me-after-help.md");
const USER_INTROSPECT_AFTER_HELP: &str =
    include_str!("../../docs/help/user-introspect-after-help.md");
const USER_LIST_AFTER_HELP: &str = include_str!("../../docs/help/user-list-after-help.md");
const USER_CREATE_AFTER_HELP: &str = include_str!("../../docs/help/user-create-after-help.md");
const USER_DELETE_AFTER_HELP: &str = include_str!("../../docs/help/user-delete-after-help.md");
const USER_PASSWORD_AFTER_HELP: &str = include_str!("../../docs/help/user-password-after-help.md");
const USER_SUDO_AFTER_HELP: &str = include_str!("../../docs/help/user-sudo-after-help.md");
const USER_FAKE_AFTER_HELP: &str = include_str!("../../docs/help/user-fake-after-help.md");
const USER_INVITE_AFTER_HELP: &str = include_str!("../../docs/help/user-invite-after-help.md");
const USER_KEYS_AFTER_HELP: &str = include_str!("../../docs/help/user-keys-after-help.md");
const USER_KEYS_CREATE_AFTER_HELP: &str =
    include_str!("../../docs/help/user-keys-create-after-help.md");
const USER_KEYS_ROTATE_AFTER_HELP: &str =
    include_str!("../../docs/help/user-keys-rotate-after-help.md");
const USER_KEYS_DELETE_AFTER_HELP: &str =
    include_str!("../../docs/help/user-keys-delete-after-help.md");
const KEYS_AFTER_HELP: &str = include_str!("../../docs/help/keys-after-help.md");
const KEYS_LIST_AFTER_HELP: &str = include_str!("../../docs/help/keys-list-after-help.md");
const KEYS_CREATE_AFTER_HELP: &str = include_str!("../../docs/help/keys-create-after-help.md");
const KEYS_DELETE_AFTER_HELP: &str = include_str!("../../docs/help/keys-delete-after-help.md");
const KEYS_REVOKE_ALL_AFTER_HELP: &str =
    include_str!("../../docs/help/keys-revoke-all-after-help.md");
const LLM_AFTER_HELP: &str = include_str!("../../docs/help/llm-after-help.md");
const LLM_PROVIDERS_AFTER_HELP: &str = include_str!("../../docs/help/llm-providers-after-help.md");
const LLM_MODELS_AFTER_HELP: &str = include_str!("../../docs/help/llm-models-after-help.md");
const LLM_SKILLS_AFTER_HELP: &str = include_str!("../../docs/help/llm-skills-after-help.md");
const LLM_ROOM_AFTER_HELP: &str = include_str!("../../docs/help/llm-room-after-help.md");
const LLM_FACTORY_AFTER_HELP: &str = include_str!("../../docs/help/llm-factory-after-help.md");
const CRON_AFTER_HELP: &str = include_str!("../../docs/help/cron-after-help.md");
const FS_AFTER_HELP: &str = include_str!("../../docs/help/fs-after-help.md");
const APP_AFTER_HELP: &str = include_str!("../../docs/help/app-after-help.md");

#[derive(Parser, Debug)]
#[command(
    name = "abbot",
    about = "CLI for the Abbotik API at https://api.abbotik.com",
    long_about = CLI_LONG_ABOUT,
    after_help = CLI_AFTER_HELP,
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

    /// Override the preferred response format (json, toon, yaml)
    #[arg(long)]
    pub format: Option<String>,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = COMMAND_AFTER_HELP)]
pub enum Command {
    /// Public surfaces and discovery
    Public(PublicCommand),
    /// Authentication and tenant bootstrap
    Auth(AuthCommand),
    /// Health checks
    Health,
    /// API documentation helpers
    Docs(DocsCommand),
    /// Model metadata and schema management
    Describe(DescribeCommand),
    /// Model data operations
    Data(DataCommand),
    /// Advanced query operations
    Find(FindCommand),
    /// Aggregate operations
    Aggregate(AggregateCommand),
    /// Multi-operation transactions
    Bulk(BulkCommand),
    /// Record ACL management
    Acls(AclsCommand),
    /// Record metadata
    Stat(StatCommand),
    /// Change tracking
    Tracked(TrackedCommand),
    /// Soft-delete and restore workflows
    Trashed(TrashedCommand),
    /// User, machine-key, and sudo workflows
    User(UserCommand),
    /// Self-service bearer API key management
    Keys(KeysCommand),
    /// LLM rooms, factory runs, and provider discovery
    Llm(LlmCommand),
    /// Scheduled process workflows
    Cron(CronCommand),
    /// Tenant filesystem workflows
    Fs(FsCommand),
    /// Dynamic app packages
    App(AppCommand),
}
