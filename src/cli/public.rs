use super::*;

#[derive(Args, Debug)]
#[command(after_long_help = PUBLIC_AFTER_HELP)]
pub struct PublicCommand {
    #[command(subcommand)]
    pub command: PublicSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = PUBLIC_AFTER_HELP)]
pub enum PublicSubcommand {
    /// Open the human-facing root document
    Root,
    /// Open the agent-facing root document
    Llms,
}
