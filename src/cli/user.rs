use super::*;

#[derive(Args, Debug)]
#[command(after_long_help = USER_AFTER_HELP)]
pub struct UserCommand {
    #[command(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = USER_AFTER_HELP)]
pub enum UserSubcommand {
    Me,
    List(UserListCommand),
    Create(UserCreateCommand),
    Invite(UserInviteCommand),
    ApiKeys(UserApiKeysCommand),
    Get(UserIdArg),
    Update(UserIdArg),
    Delete(UserDeleteCommand),
    Password(UserPasswordCommand),
    Sudo(UserSudoCommand),
    Fake(UserFakeCommand),
}

#[derive(Args, Debug, Default)]
#[command(after_long_help = USER_LIST_AFTER_HELP)]
pub struct UserListCommand {
    /// Maximum number of records to return
    #[arg(long)]
    pub limit: Option<u32>,

    /// Number of records to skip
    #[arg(long)]
    pub offset: Option<u32>,
}

#[derive(Args, Debug)]
#[command(after_long_help = USER_CREATE_AFTER_HELP)]
pub struct UserCreateCommand {
    /// JSON body from stdin or use --body to inline it
    #[arg(long)]
    pub body: Option<String>,

    /// Optional name
    #[arg(long)]
    pub name: Option<String>,

    /// Optional auth identifier
    #[arg(long)]
    pub auth: Option<String>,

    /// Optional access level
    #[arg(long)]
    pub access: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = USER_INVITE_AFTER_HELP)]
pub struct UserInviteCommand {
    /// Canonical username to reserve for the invited user
    #[arg(long)]
    pub username: Option<String>,

    /// Invite type: human, machine, or either
    #[arg(long = "invite-type")]
    pub invite_type: Option<String>,

    /// Tenant-local access level for the invited user
    #[arg(long)]
    pub access: Option<String>,

    /// Record-level read grants to attach to the invite
    #[arg(long = "access-read")]
    pub access_read: Vec<String>,

    /// Record-level edit grants to attach to the invite
    #[arg(long = "access-edit")]
    pub access_edit: Vec<String>,

    /// Record-level full grants to attach to the invite
    #[arg(long = "access-full")]
    pub access_full: Vec<String>,

    /// Invite lifetime in seconds
    #[arg(long = "expires-in")]
    pub expires_in: Option<u64>,
}

#[derive(Args, Debug)]
#[command(after_long_help = USER_KEYS_AFTER_HELP)]
pub struct UserApiKeysCommand {
    #[command(subcommand)]
    pub command: UserApiKeysSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = USER_KEYS_AFTER_HELP)]
pub enum UserApiKeysSubcommand {
    List,
    Create(UserApiKeysCreateCommand),
    Delete(UserApiKeyIdArg),
    RevokeAll,
}

#[derive(Args, Debug)]
#[command(after_long_help = USER_KEYS_CREATE_AFTER_HELP)]
pub struct UserApiKeysCreateCommand {
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
#[command(after_long_help = USER_DELETE_AFTER_HELP)]
pub struct UserDeleteCommand {
    pub id: String,

    /// Explicitly confirm self-deactivation
    #[arg(long)]
    pub confirm: bool,

    /// Optional audit-trail reason
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = USER_PASSWORD_AFTER_HELP)]
pub struct UserPasswordCommand {
    pub id: String,

    /// Current password when changing your own password
    #[arg(long = "current-password")]
    pub current_password: Option<String>,

    /// New password to set
    #[arg(long = "new-password")]
    pub new_password: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = USER_SUDO_AFTER_HELP)]
pub struct UserSudoCommand {
    /// Audit-trail reason for the elevation
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = USER_FAKE_AFTER_HELP)]
pub struct UserFakeCommand {
    /// Target user ID to impersonate
    #[arg(long = "user-id")]
    pub user_id: Option<String>,

    /// Target username to impersonate
    #[arg(long)]
    pub username: Option<String>,
}
