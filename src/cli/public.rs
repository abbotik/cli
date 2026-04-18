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
    Root(PublicRootCommand),
    /// Open the agent-facing root document
    Llms(PublicLlmsCommand),
}

#[derive(Args, Debug, Default)]
#[command(after_long_help = PUBLIC_ROOT_AFTER_HELP)]
pub struct PublicRootCommand {}

#[derive(Args, Debug, Default)]
#[command(after_long_help = PUBLIC_LLMS_AFTER_HELP)]
pub struct PublicLlmsCommand {}
