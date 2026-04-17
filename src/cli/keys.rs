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
    Rotate(KeysRotateCommand),
    Delete(KeysDeleteCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = KEYS_CREATE_AFTER_HELP)]
pub struct KeysCreateCommand {
    /// Tenant-local user ID to bind the key to
    #[arg(long = "user-id")]
    pub user_id: Option<String>,

    /// Public key PEM, use - for stdin or @<path> for a file
    #[arg(long = "public-key")]
    pub public_key: Option<String>,

    /// Friendly name for the API key
    #[arg(long)]
    pub name: Option<String>,

    /// Public-key algorithm
    #[arg(long)]
    pub algorithm: Option<String>,

    /// ISO 8601 expiration timestamp
    #[arg(long = "expires-at")]
    pub expires_at: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = KEYS_ROTATE_AFTER_HELP)]
pub struct KeysRotateCommand {
    /// Existing key ID to rotate
    #[arg(long = "key-id")]
    pub key_id: Option<String>,

    /// Replacement public key PEM, use - for stdin or @<path> for a file
    #[arg(long = "new-public-key")]
    pub new_public_key: Option<String>,

    /// Public-key algorithm
    #[arg(long)]
    pub algorithm: Option<String>,

    /// Friendly name for the replacement key
    #[arg(long = "new-name")]
    pub new_name: Option<String>,

    /// Delay before the old key is revoked
    #[arg(long = "revoke-old-after-seconds")]
    pub revoke_old_after_seconds: Option<u32>,
}

#[derive(Args, Debug)]
#[command(after_long_help = KEYS_DELETE_AFTER_HELP)]
pub struct KeysDeleteCommand {
    pub key_id: String,
}
