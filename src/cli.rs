use clap::{Args, Parser, Subcommand};

const CLI_LONG_ABOUT: &str = include_str!("../docs/help/cli-long-about.md");
const CLI_AFTER_HELP: &str = include_str!("../docs/help/cli-after-help.md");
const COMMAND_AFTER_HELP: &str = include_str!("../docs/help/command-after-help.md");
const PUBLIC_AFTER_HELP: &str = include_str!("../docs/help/public-after-help.md");
const AUTH_AFTER_HELP: &str = include_str!("../docs/help/auth-after-help.md");
const AUTH_LOGIN_AFTER_HELP: &str = include_str!("../docs/help/auth-login-after-help.md");
const AUTH_REGISTER_AFTER_HELP: &str = include_str!("../docs/help/auth-register-after-help.md");
const AUTH_REFRESH_AFTER_HELP: &str = include_str!("../docs/help/auth-refresh-after-help.md");
const AUTH_PROVISION_AFTER_HELP: &str = include_str!("../docs/help/auth-provision-after-help.md");
const AUTH_CHALLENGE_AFTER_HELP: &str = include_str!("../docs/help/auth-challenge-after-help.md");
const AUTH_VERIFY_AFTER_HELP: &str = include_str!("../docs/help/auth-verify-after-help.md");
const AUTH_DISSOLVE_AFTER_HELP: &str = include_str!("../docs/help/auth-dissolve-after-help.md");
const AUTH_DISSOLVE_CONFIRM_AFTER_HELP: &str = include_str!("../docs/help/auth-dissolve-confirm-after-help.md");
const AUTH_TOKEN_AFTER_HELP: &str = include_str!("../docs/help/auth-token-after-help.md");
const DOCS_AFTER_HELP: &str = include_str!("../docs/help/docs-after-help.md");
const DESCRIBE_AFTER_HELP: &str = include_str!("../docs/help/describe-after-help.md");
const DESCRIBE_FIELDS_AFTER_HELP: &str = include_str!("../docs/help/describe-fields-after-help.md");
const DATA_AFTER_HELP: &str = include_str!("../docs/help/data-after-help.md");
const DATA_RELATIONSHIP_AFTER_HELP: &str = include_str!("../docs/help/data-relationship-after-help.md");
const DATA_RELATIONSHIP_CHILD_AFTER_HELP: &str = include_str!("../docs/help/data-relationship-child-after-help.md");
const FIND_AFTER_HELP: &str = include_str!("../docs/help/find-after-help.md");
const AGGREGATE_AFTER_HELP: &str = include_str!("../docs/help/aggregate-after-help.md");
const BULK_AFTER_HELP: &str = include_str!("../docs/help/bulk-after-help.md");
const ACLS_AFTER_HELP: &str = include_str!("../docs/help/acls-after-help.md");
const STAT_AFTER_HELP: &str = include_str!("../docs/help/stat-after-help.md");
const TRACKED_AFTER_HELP: &str = include_str!("../docs/help/tracked-after-help.md");
const TRASHED_AFTER_HELP: &str = include_str!("../docs/help/trashed-after-help.md");
const USER_AFTER_HELP: &str = include_str!("../docs/help/user-after-help.md");
const USER_LIST_AFTER_HELP: &str = include_str!("../docs/help/user-list-after-help.md");
const USER_CREATE_AFTER_HELP: &str = include_str!("../docs/help/user-create-after-help.md");
const USER_DELETE_AFTER_HELP: &str = include_str!("../docs/help/user-delete-after-help.md");
const USER_PASSWORD_AFTER_HELP: &str = include_str!("../docs/help/user-password-after-help.md");
const USER_SUDO_AFTER_HELP: &str = include_str!("../docs/help/user-sudo-after-help.md");
const USER_FAKE_AFTER_HELP: &str = include_str!("../docs/help/user-fake-after-help.md");
const KEYS_AFTER_HELP: &str = include_str!("../docs/help/keys-after-help.md");
const KEYS_CREATE_AFTER_HELP: &str = include_str!("../docs/help/keys-create-after-help.md");
const KEYS_ROTATE_AFTER_HELP: &str = include_str!("../docs/help/keys-rotate-after-help.md");
const KEYS_DELETE_AFTER_HELP: &str = include_str!("../docs/help/keys-delete-after-help.md");
const CRON_AFTER_HELP: &str = include_str!("../docs/help/cron-after-help.md");
const FS_AFTER_HELP: &str = include_str!("../docs/help/fs-after-help.md");
const APP_AFTER_HELP: &str = include_str!("../docs/help/app-after-help.md");

#[derive(Parser, Debug)]
#[command(
    name = "monk",
    about = "CLI for the Monk API at https://monk-api.com",
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
    /// Override the Monk API base URL
    #[arg(long = "base-url")]
    pub base_url: Option<String>,

    /// Override the stored bearer token
    #[arg(long)]
    pub token: Option<String>,

    /// Override the preferred response format
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
    /// User and sudo workflows
    User(UserCommand),
    /// Tenant machine key management
    Keys(KeysCommand),
    /// Scheduled process workflows
    Cron(CronCommand),
    /// Tenant filesystem workflows
    Fs(FsCommand),
    /// Dynamic app packages
    App(AppCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = PUBLIC_AFTER_HELP)]
pub struct PublicCommand {
    #[command(subcommand)]
    pub command: PublicSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = PUBLIC_AFTER_HELP)]
pub enum PublicSubcommand {
    /// Open the human-facing root document
    Root,
    /// Open the agent-facing root document
    Llms,
}

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
    /// Verify a signed challenge and mint a Monk bearer token
    Verify(AuthVerifyCommand),
    /// Dissolve a tenant via the two-step confirmation flow
    Dissolve(AuthDissolveCommand),
    /// Show, set, or clear the saved JWT
    Token(AuthTokenCommand),
    /// List tenants available for login
    Tenants,
}

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

    /// Public key PEM, use - for stdin or @<path> for a file
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

    /// Challenge ID returned by monk auth provision/challenge
    #[arg(long = "challenge-id")]
    pub challenge_id: Option<String>,

    /// Base64url signature, use - for stdin or @<path> for a file
    #[arg(long)]
    pub signature: Option<String>,
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

#[derive(Args, Debug)]
#[command(after_long_help = DOCS_AFTER_HELP)]
pub struct DocsCommand {
    #[command(subcommand)]
    pub command: DocsSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DOCS_AFTER_HELP)]
pub enum DocsSubcommand {
    /// Open the API overview
    Root,
    /// Open a docs path directly
    Path { path: Option<String> },
}

#[derive(Args, Debug)]
#[command(after_long_help = DESCRIBE_AFTER_HELP)]
pub struct DescribeCommand {
    #[command(subcommand)]
    pub command: DescribeSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DESCRIBE_AFTER_HELP)]
pub enum DescribeSubcommand {
    List,
    Get(ModelArg),
    Create(ModelArg),
    Update(ModelArg),
    Delete(ModelArg),
    Fields(DescribeFieldsCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = DESCRIBE_FIELDS_AFTER_HELP)]
pub struct DescribeFieldsCommand {
    #[command(subcommand)]
    pub command: DescribeFieldsSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DESCRIBE_FIELDS_AFTER_HELP)]
pub enum DescribeFieldsSubcommand {
    List(ModelArg),
    BulkCreate(ModelArg),
    BulkUpdate(ModelArg),
    Get(FieldArg),
    Create(FieldArg),
    Update(FieldArg),
    Delete(FieldArg),
}

#[derive(Args, Debug, Default, Clone)]
pub struct DataOptions {
    /// Include soft-deleted records
    #[arg(long)]
    pub include_trashed: bool,

    /// Include permanently deleted records
    #[arg(long)]
    pub include_deleted: bool,

    /// Remove the success envelope from responses
    #[arg(long)]
    pub unwrap: bool,

    /// Select a comma-separated field list
    #[arg(long)]
    pub select: Option<String>,

    /// Apply a JSON where filter
    #[arg(long = "where")]
    pub r#where: Option<String>,

    /// Exclude timestamp fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub stat: Option<bool>,

    /// Exclude ACL fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub access: Option<bool>,

    /// Perform permanent delete
    #[arg(long)]
    pub permanent: bool,

    /// Enable upsert mode for creates
    #[arg(long)]
    pub upsert: bool,
}

#[derive(Args, Debug)]
#[command(after_long_help = DATA_AFTER_HELP)]
pub struct DataCommand {
    #[command(flatten)]
    pub options: DataOptions,

    #[command(subcommand)]
    pub command: DataSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DATA_AFTER_HELP)]
pub enum DataSubcommand {
    /// List records for a model via GET /api/data/:model
    List(ModelArg),
    /// Create one or more records via POST /api/data/:model
    Create(ModelArg),
    /// Bulk update records by id via PUT /api/data/:model
    Update(ModelArg),
    /// Bulk update records by filter via PATCH /api/data/:model
    Patch(ModelArg),
    /// Soft delete records via DELETE /api/data/:model
    Delete(ModelArg),
    /// Fetch a single record via GET /api/data/:model/:id
    Get(RecordArg),
    /// Update a single record via PUT /api/data/:model/:id
    Put(RecordArg),
    /// Patch a single record via PATCH /api/data/:model/:id
    #[command(name = "patch-record")]
    PatchRecord(RecordArg),
    /// Soft delete a single record via DELETE /api/data/:model/:id
    DeleteRecord(RecordArg),
    /// Work with owned relationship routes under /api/data/:model/:id/:relationship
    Relationship(RelationshipArg),
}

#[derive(Args, Debug)]
#[command(after_long_help = DATA_RELATIONSHIP_AFTER_HELP)]
pub struct RelationshipArg {
    pub model: String,
    pub id: String,
    pub relationship: String,
    #[command(subcommand)]
    pub command: RelationshipSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DATA_RELATIONSHIP_AFTER_HELP)]
pub enum RelationshipSubcommand {
    /// List child records via GET /api/data/:model/:id/:relationship
    Get,
    /// Create a child record via POST /api/data/:model/:id/:relationship
    Create,
    /// Bulk update child records via PUT /api/data/:model/:id/:relationship
    Update,
    /// Soft delete child records via DELETE /api/data/:model/:id/:relationship
    Delete,
    /// Address a specific nested child record
    Child(RelationshipChildCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = DATA_RELATIONSHIP_CHILD_AFTER_HELP)]
pub struct RelationshipChildCommand {
    pub child: String,
    #[command(subcommand)]
    pub command: RelationshipChildSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DATA_RELATIONSHIP_CHILD_AFTER_HELP)]
pub enum RelationshipChildSubcommand {
    /// Fetch a nested child record via GET /api/data/:model/:id/:relationship/:child
    Get,
    /// Update a nested child record via PUT /api/data/:model/:id/:relationship/:child
    Put,
    /// Patch a nested child record via PATCH /api/data/:model/:id/:relationship/:child
    Patch,
    /// Soft delete a nested child record via DELETE /api/data/:model/:id/:relationship/:child
    Delete,
}

#[derive(Args, Debug, Default, Clone)]
pub struct FindOptions {
    /// Project a comma-separated field list
    #[arg(long)]
    pub select: Option<String>,

    /// Apply a JSON where filter from stdin, a file (@path), or inline JSON
    #[arg(long = "where")]
    pub r#where: Option<String>,

    /// Apply a comma-separated order list
    #[arg(long)]
    pub order: Option<String>,

    /// Limit the number of returned records
    #[arg(long)]
    pub limit: Option<u32>,

    /// Skip the first N matching records
    #[arg(long)]
    pub offset: Option<u32>,
}

#[derive(Args, Debug)]
#[command(after_long_help = FIND_AFTER_HELP)]
pub struct FindCommand {
    #[command(flatten)]
    pub options: FindOptions,

    #[command(subcommand)]
    pub command: FindSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = FIND_AFTER_HELP)]
pub enum FindSubcommand {
    Query(ModelArg),
    Saved(FindSavedArg),
}

#[derive(Args, Debug)]
pub struct FindSavedArg {
    pub model: String,
    pub target: String,
}

#[derive(Args, Debug, Default, Clone)]
pub struct AggregateOptions {
    /// Count all records
    #[arg(long)]
    pub count: bool,

    /// Sum of field values
    #[arg(long)]
    pub sum: Option<String>,

    /// Average of field values
    #[arg(long)]
    pub avg: Option<String>,

    /// Minimum field value
    #[arg(long)]
    pub min: Option<String>,

    /// Maximum field value
    #[arg(long)]
    pub max: Option<String>,

    /// Apply a JSON where filter from stdin, a file (@path), or inline JSON
    #[arg(long = "where")]
    pub r#where: Option<String>,

    /// Full POST body from stdin, a file (@path), or inline JSON
    #[arg(long)]
    pub body: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AGGREGATE_AFTER_HELP)]
pub struct AggregateCommand {
    #[command(flatten)]
    pub options: AggregateOptions,

    #[command(subcommand)]
    pub command: AggregateSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = AGGREGATE_AFTER_HELP)]
pub enum AggregateSubcommand {
    Get(ModelArg),
    Run(ModelArg),
}

#[derive(Args, Debug, Default, Clone)]
pub struct BulkOptions {
    /// JSON body from stdin, a file (@path), or inline JSON
    #[arg(long)]
    pub body: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = BULK_AFTER_HELP)]
pub struct BulkCommand {
    #[command(flatten)]
    pub options: BulkOptions,

    #[command(subcommand)]
    pub command: BulkSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = BULK_AFTER_HELP)]
pub enum BulkSubcommand {
    /// Execute an arbitrary bulk payload
    Run,
    /// Create many records in one model
    Create(ModelArg),
    /// Update many records in one model
    Update(ModelArg),
    /// Delete many records in one model
    Delete(ModelArg),
    Export,
    Import,
}

#[derive(Args, Debug)]
#[command(after_long_help = ACLS_AFTER_HELP)]
pub struct AclsCommand {
    #[command(subcommand)]
    pub command: AclsSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = ACLS_AFTER_HELP)]
pub enum AclsSubcommand {
    Get(RecordArg),
    Create(RecordArg),
    Update(RecordArg),
    Delete(RecordArg),
}

#[derive(Args, Debug)]
#[command(after_long_help = STAT_AFTER_HELP)]
pub struct StatCommand {
    #[command(subcommand)]
    pub command: StatSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = STAT_AFTER_HELP)]
pub enum StatSubcommand {
    Get(RecordArg),
}

#[derive(Args, Debug)]
#[command(after_long_help = TRACKED_AFTER_HELP)]
pub struct TrackedCommand {
    #[command(subcommand)]
    pub command: TrackedSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = TRACKED_AFTER_HELP)]
pub enum TrackedSubcommand {
    List(RecordArg),
    Get(TrackedRecordArg),
}

#[derive(Args, Debug)]
#[command(after_long_help = TRASHED_AFTER_HELP)]
pub struct TrashedCommand {
    #[command(subcommand)]
    pub command: TrashedSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = TRASHED_AFTER_HELP)]
pub enum TrashedSubcommand {
    List,
    Model(TrashedModelArg),
    Record(RecordArg),
}

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

#[derive(Args, Debug)]
#[command(after_long_help = CRON_AFTER_HELP)]
pub struct CronCommand {
    #[command(subcommand)]
    pub command: CronSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = CRON_AFTER_HELP)]
pub enum CronSubcommand {
    List,
    Create,
    Get(CronIdArg),
    Update(CronIdArg),
    Delete(CronIdArg),
    Enable(CronIdArg),
    Disable(CronIdArg),
}

#[derive(Args, Debug, Default, Clone)]
#[command(after_long_help = FS_AFTER_HELP)]
pub struct FsOptions {
    /// Return filesystem metadata as JSON instead of file content
    #[arg(long)]
    pub stat: bool,

    /// File content from stdin, a file (@path), or inline text
    #[arg(long)]
    pub body: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = FS_AFTER_HELP)]
pub struct FsCommand {
    #[command(flatten)]
    pub options: FsOptions,

    #[command(subcommand)]
    pub command: FsSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = FS_AFTER_HELP)]
pub enum FsSubcommand {
    Get(PathArg),
    Put(PathArg),
    Delete(PathArg),
}

#[derive(Args, Debug)]
#[command(after_long_help = APP_AFTER_HELP)]
pub struct AppCommand {
    pub app_name: String,
    pub path: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct ModelArg {
    pub model: String,
}

#[derive(Args, Debug, Clone)]
pub struct FieldArg {
    pub model: String,
    pub field: String,
}

#[derive(Args, Debug, Clone)]
pub struct RecordArg {
    pub model: String,
    pub id: String,
}

#[derive(Args, Debug, Clone)]
pub struct TrackedRecordArg {
    pub model: String,
    pub id: String,
    pub change: String,
}

#[derive(Args, Debug, Clone)]
pub struct TrashedModelArg {
    pub model: String,
}

#[derive(Args, Debug, Clone)]
pub struct UserIdArg {
    pub id: String,
}

#[derive(Args, Debug, Clone)]
pub struct CronIdArg {
    pub pid: String,
}

#[derive(Args, Debug, Clone)]
pub struct PathArg {
    pub path: String,
}
