use super::*;

#[derive(Args, Debug)]
pub struct StatCommand {
    #[command(subcommand)]
    pub command: StatSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum StatSubcommand {
    Get(RecordArg),
}
