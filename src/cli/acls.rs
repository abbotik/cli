use super::*;

#[derive(Args, Debug)]
pub struct AclsCommand {
    #[command(subcommand)]
    pub command: AclsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum AclsSubcommand {
    Get(RecordArg),
    Create(RecordArg),
    Update(RecordArg),
    Delete(RecordArg),
}
