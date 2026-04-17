use super::*;

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
