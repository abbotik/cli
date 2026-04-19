use super::*;

#[derive(Args, Debug)]
pub struct CronCommand {
    #[command(subcommand)]
    pub command: CronSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum CronSubcommand {
    List,
    Create,
    Get(CronIdArg),
    Update(CronIdArg),
    Delete(CronIdArg),
    Enable(CronIdArg),
    Disable(CronIdArg),
}
