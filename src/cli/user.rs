use super::*;

#[derive(Args, Debug)]
pub struct UserCommand {
    #[command(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum UserSubcommand {
    /// Show the current authenticated user profile
    Me(UserMeCommand),
    /// Return the trusted execution context for the current bearer token
    Introspect(UserIntrospectCommand),
    /// List users in the current tenant
    List(UserListCommand),
    /// Create a user in the current tenant
    Create(UserCreateCommand),
    /// Create a one-time invite for a future tenant user
    Invite(UserInviteCommand),
    /// Manage tenant-bound machine public keys
    MachineKeys(UserMachineKeysCommand),
    /// Fetch one user by ID or `me`
    Get(UserIdArg),
    /// Update one user by ID or `me`
    Update(UserIdArg),
    /// Delete one user by ID or `me`
    Delete(UserDeleteCommand),
    /// Change a user's password
    Password(UserPasswordCommand),
    /// Mint a short-lived sudo token
    Sudo(UserSudoCommand),
    /// Mint a short-lived impersonation token
    Fake(UserFakeCommand),
}

#[derive(Args, Debug, Default)]
pub struct UserMeCommand {}

#[derive(Args, Debug, Default)]
pub struct UserIntrospectCommand {}

#[derive(Args, Debug, Default)]
pub struct UserListCommand {
    /// Maximum number of records to return
    #[arg(long)]
    pub limit: Option<u32>,

    /// Number of records to skip
    #[arg(long)]
    pub offset: Option<u32>,
}

#[derive(Args, Debug)]
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

    /// Optional access level: deny, read, edit, full, or root
    #[arg(long, value_parser = ["deny", "read", "edit", "full", "root"])]
    pub access: Option<String>,
}

#[derive(Args, Debug)]
pub struct UserInviteCommand {
    /// Canonical username to reserve for the invited user
    #[arg(long)]
    pub username: Option<String>,

    /// Invite type: human, machine, or either
    #[arg(long = "invite-type")]
    pub invite_type: Option<String>,

    /// Tenant-local access level for the invited user: deny, read, edit, full, or root
    #[arg(long, value_parser = ["deny", "read", "edit", "full", "root"])]
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
pub struct UserMachineKeysCommand {
    #[command(subcommand)]
    pub command: UserMachineKeysSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum UserMachineKeysSubcommand {
    /// List tenant machine keys with fingerprint-first metadata
    List,
    /// Add a public key bound to a tenant-local user
    Create(UserMachineKeysCreateCommand),
    /// Rotate a machine key with an overlap window
    Rotate(UserMachineKeysRotateCommand),
    /// Revoke one tenant machine key by key ID
    Delete(UserMachineKeyIdArg),
}

#[derive(Args, Debug)]
pub struct UserMachineKeysCreateCommand {
    /// Tenant-local user ID to bind the key to
    #[arg(long = "user-id")]
    pub user_id: Option<String>,

    /// Public key PEM, use - for stdin or @<path> for a file
    #[arg(long = "public-key")]
    pub public_key: Option<String>,

    /// Friendly name for the machine key
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
pub struct UserMachineKeysRotateCommand {
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
pub struct UserMachineKeyIdArg {
    pub key_id: String,
}

#[derive(Args, Debug)]
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
pub struct UserSudoCommand {
    /// Audit-trail reason for the elevation
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Args, Debug)]
pub struct UserFakeCommand {
    /// Target user ID to impersonate
    #[arg(long = "user-id")]
    pub user_id: Option<String>,

    /// Target username to impersonate
    #[arg(long)]
    pub username: Option<String>,
}
