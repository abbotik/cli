use super::*;

#[derive(Args, Debug)]
pub struct KeysCommand {
    #[command(subcommand)]
    pub command: KeysSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum KeysSubcommand {
    /// List the current user's global bearer API keys
    List(KeysListCommand),
    /// Mint a new global bearer API key for the current user
    Create(KeysCreateCommand),
    /// Revoke one global bearer API key by key ID
    Delete(KeysDeleteCommand),
    /// Revoke every global bearer API key owned by the current user
    RevokeAll(KeysRevokeAllCommand),
}

#[derive(Args, Debug, Default)]
pub struct KeysListCommand {}

#[derive(Args, Debug)]
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
pub struct KeysDeleteCommand {
    pub key_id: String,
}

#[derive(Args, Debug, Default)]
pub struct KeysRevokeAllCommand {}
