use super::*;

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
