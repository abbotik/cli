use super::*;

#[derive(Args, Debug)]
pub struct TrackedCommand {
    #[command(subcommand)]
    pub command: TrackedSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum TrackedSubcommand {
    List(RecordArg),
    Get(TrackedRecordArg),
}
