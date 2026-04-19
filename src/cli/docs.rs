use super::*;

#[derive(Args, Debug)]
pub struct DocsCommand {
    #[command(subcommand)]
    pub command: DocsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum DocsSubcommand {
    /// Open the API overview
    Root(DocsRootCommand),
    /// Open a docs path directly
    Path(DocsPathCommand),
}

#[derive(Args, Debug, Default)]
pub struct DocsRootCommand {}

#[derive(Args, Debug)]
pub struct DocsPathCommand {
    pub path: Option<String>,
}
