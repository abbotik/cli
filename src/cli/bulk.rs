use super::*;

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
