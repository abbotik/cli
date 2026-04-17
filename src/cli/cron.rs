use super::*;

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
