use super::*;

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
