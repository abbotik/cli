use super::*;

#[derive(Args, Debug)]
pub struct TrashedCommand {
    #[command(subcommand)]
    pub command: TrashedSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum TrashedSubcommand {
    List,
    Model(TrashedModelArg),
    Record(RecordArg),
}
