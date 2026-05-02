use super::*;

#[derive(Args, Debug)]
pub struct ApiCommand {
    #[command(subcommand)]
    pub command: ApiSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ApiSubcommand {
    /// Record ACL management
    Acls(AclsCommand),
    /// Aggregate operations
    Aggregate(AggregateCommand),
    /// Multi-operation transactions
    Bulk(BulkCommand),
    /// Scheduled process workflows
    Cron(CronCommand),
    /// Model data operations
    Data(DataCommand),
    /// Model metadata and schema management
    Describe(DescribeCommand),
    /// Advanced query operations
    Find(FindCommand),
    /// Self-service bearer API key management
    Keys(KeysCommand),
    /// Record metadata
    Stat(StatCommand),
    /// Change tracking
    Tracked(TrackedCommand),
    /// Soft-delete and restore workflows
    Trashed(TrashedCommand),
    /// User, machine-key, secret, and sudo routes
    User(UserCommand),
}
