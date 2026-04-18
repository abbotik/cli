use super::*;

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_AFTER_HELP)]
pub struct AuthCommand {
    #[command(subcommand)]
    pub command: AuthSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = AUTH_AFTER_HELP)]
pub enum AuthSubcommand {
    /// Log in to an existing tenant
    Login(AuthLoginCommand),
    /// Register a new tenant
    Register(AuthRegisterCommand),
    /// Refresh a token
    Refresh(AuthRefreshCommand),
    /// Bootstrap machine auth with the first tenant-bound public key
    Provision(AuthProvisionCommand),
    /// Ask for a machine-auth signing challenge
    Challenge(AuthChallengeCommand),
    /// Verify a signed challenge and mint an Abbotik bearer token
    Verify(AuthVerifyCommand),
    /// Machine auth happy-path commands
    Machine(AuthMachineCommand),
    /// Dissolve a tenant via the two-step confirmation flow
    Dissolve(AuthDissolveCommand),
    /// Show, set, or clear the saved JWT
    Token(AuthTokenCommand),
    /// List tenants available for login
    Tenants(AuthTenantsCommand),
}

#[derive(Args, Debug, Default)]
#[command(after_long_help = AUTH_TENANTS_AFTER_HELP)]
pub struct AuthTenantsCommand {}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_LOGIN_AFTER_HELP)]
pub struct AuthLoginCommand {
    /// Tenant name to authenticate against
    #[arg(long)]
    pub tenant: Option<String>,

    /// Tenant ID to authenticate against
    #[arg(long = "tenant-id")]
    pub tenant_id: Option<String>,

    /// Canonical username for the tenant user
    #[arg(long)]
    pub username: Option<String>,

    /// Password for the tenant user
    #[arg(long)]
    pub password: Option<String>,

    /// Override the requested response format
    #[arg(long)]
    pub format: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_REGISTER_AFTER_HELP)]
pub struct AuthRegisterCommand {
    /// Tenant name to register
    #[arg(long)]
    pub tenant: Option<String>,

    /// Canonical username for the tenant owner
    #[arg(long)]
    pub username: Option<String>,

    /// One-time invite code for joining an existing tenant user
    #[arg(long = "invite-code")]
    pub invite_code: Option<String>,

    /// Email address for Auth0 user provisioning
    #[arg(long)]
    pub email: Option<String>,

    /// Password for the tenant owner
    #[arg(long)]
    pub password: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_REFRESH_AFTER_HELP)]
pub struct AuthRefreshCommand {
    /// Refresh token to exchange; defaults to the saved token
    #[arg(long)]
    pub token: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_PROVISION_AFTER_HELP)]
pub struct AuthProvisionCommand {
    /// Tenant name to provision
    #[arg(long)]
    pub tenant: Option<String>,

    /// Canonical username for the bootstrap root user
    #[arg(long)]
    pub username: Option<String>,

    /// One-time invite code for joining an existing tenant machine user
    #[arg(long = "invite-code")]
    pub invite_code: Option<String>,

    /// Public key PEM, use - for stdin or @<path> for a file
    #[arg(long = "public-key")]
    pub public_key: Option<String>,

    /// Save the source public key path into local config for future machine refresh
    #[arg(long = "save-public-key-path")]
    pub save_public_key_path: Option<String>,

    /// Save the matching private key path into local config for future machine refresh
    #[arg(long = "save-private-key-path")]
    pub save_private_key_path: Option<String>,

    /// Public-key algorithm
    #[arg(long)]
    pub algorithm: Option<String>,

    /// Friendly name for the provisioned key
    #[arg(long = "key-name")]
    pub key_name: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_CHALLENGE_AFTER_HELP)]
pub struct AuthChallengeCommand {
    /// Tenant name to authenticate against
    #[arg(long)]
    pub tenant: Option<String>,

    /// Key ID to challenge
    #[arg(long = "key-id")]
    pub key_id: Option<String>,

    /// Key fingerprint to challenge
    #[arg(long)]
    pub fingerprint: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_VERIFY_AFTER_HELP)]
pub struct AuthVerifyCommand {
    /// Tenant name to authenticate against
    #[arg(long)]
    pub tenant: Option<String>,

    /// Challenge ID returned by abbot auth provision/challenge
    #[arg(long = "challenge-id")]
    pub challenge_id: Option<String>,

    /// Base64url signature, use - for stdin or @<path> for a file
    #[arg(long)]
    pub signature: Option<String>,

    /// Save the source public key path into local config for future machine refresh
    #[arg(long = "save-public-key-path")]
    pub save_public_key_path: Option<String>,

    /// Save the matching private key path into local config for future machine refresh
    #[arg(long = "save-private-key-path")]
    pub save_private_key_path: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_MACHINE_AFTER_HELP)]
pub struct AuthMachineCommand {
    #[command(subcommand)]
    pub command: AuthMachineSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = AUTH_MACHINE_AFTER_HELP)]
pub enum AuthMachineSubcommand {
    /// Connect a machine key by provisioning or re-verifying automatically
    Connect(AuthMachineConnectCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_MACHINE_CONNECT_AFTER_HELP)]
pub struct AuthMachineConnectCommand {
    /// Tenant name to authenticate against
    #[arg(long)]
    pub tenant: Option<String>,

    /// Canonical username for first-time machine bootstrap
    #[arg(long)]
    pub username: Option<String>,

    /// One-time invite code for joining an existing tenant machine user
    #[arg(long = "invite-code")]
    pub invite_code: Option<String>,

    /// Path to an Ed25519 private key PEM; plain paths and @<path> both work
    #[arg(long = "key")]
    pub key: Option<String>,

    /// Optional public key PEM override, use - for stdin or @<path> for a file
    #[arg(long = "public-key")]
    pub public_key: Option<String>,

    /// Public-key algorithm
    #[arg(long)]
    pub algorithm: Option<String>,

    /// Friendly name for the provisioned key
    #[arg(long = "key-name")]
    pub key_name: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_DISSOLVE_AFTER_HELP)]
pub struct AuthDissolveCommand {
    #[command(subcommand)]
    pub command: AuthDissolveSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = AUTH_DISSOLVE_AFTER_HELP)]
pub enum AuthDissolveSubcommand {
    /// Request a short-lived dissolution confirmation token
    Request(AuthDissolveRequestCommand),
    /// Consume a confirmation token and dissolve the tenant
    Confirm(AuthDissolveConfirmCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_DISSOLVE_AFTER_HELP)]
pub struct AuthDissolveRequestCommand {
    /// Tenant name to dissolve
    #[arg(long)]
    pub tenant: Option<String>,

    /// Canonical username for the tenant owner
    #[arg(long)]
    pub username: Option<String>,

    /// Password for the tenant owner
    #[arg(long)]
    pub password: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_DISSOLVE_CONFIRM_AFTER_HELP)]
pub struct AuthDissolveConfirmCommand {
    /// Confirmation token from auth dissolve request
    #[arg(long = "confirmation-token")]
    pub confirmation_token: String,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_TOKEN_AFTER_HELP)]
pub struct AuthTokenCommand {
    #[command(subcommand)]
    pub command: AuthTokenSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = AUTH_TOKEN_AFTER_HELP)]
pub enum AuthTokenSubcommand {
    /// Print the saved JWT
    Get,
    /// Overwrite the saved JWT
    Set(AuthTokenSetCommand),
    /// Remove the saved JWT
    Clear,
}

#[derive(Args, Debug)]
#[command(after_long_help = AUTH_TOKEN_AFTER_HELP)]
pub struct AuthTokenSetCommand {
    /// JWT value to save; use - for stdin or @<path> for a file
    pub token: String,
}
