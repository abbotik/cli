use super::*;

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
