use super::*;

#[derive(Args, Debug)]
#[command(after_long_help = KEYS_AFTER_HELP)]
pub struct KeysCommand {
    #[command(subcommand)]
    pub command: KeysSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = KEYS_AFTER_HELP)]
pub enum KeysSubcommand {
    List,
    Create(KeysCreateCommand),
    Delete(KeysDeleteCommand),
    RevokeAll,
}

#[derive(Args, Debug)]
#[command(after_long_help = KEYS_CREATE_AFTER_HELP)]
pub struct KeysCreateCommand {
    /// JSON body from stdin or use --body to inline it
    #[arg(long)]
    pub body: Option<String>,

    /// Friendly name for the API key
    #[arg(long)]
    pub name: Option<String>,

    /// ISO 8601 expiration timestamp
    #[arg(long = "expires-at")]
    pub expires_at: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = KEYS_DELETE_AFTER_HELP)]
pub struct KeysDeleteCommand {
    pub key_id: String,
}
