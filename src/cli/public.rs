use super::*;

#[derive(Args, Debug)]
pub struct PublicCommand {
    #[command(subcommand)]
    pub command: PublicSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum PublicSubcommand {
    /// Open the human-facing root document
    Root(PublicRootCommand),
    /// Open the agent-facing root document
    Llms(PublicLlmsCommand),
}

#[derive(Args, Debug, Default)]
pub struct PublicRootCommand {}

#[derive(Args, Debug, Default)]
pub struct PublicLlmsCommand {}
