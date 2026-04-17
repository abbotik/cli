use super::*;

#[derive(Args, Debug)]
#[command(after_long_help = DOCS_AFTER_HELP)]
pub struct DocsCommand {
    #[command(subcommand)]
    pub command: DocsSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DOCS_AFTER_HELP)]
pub enum DocsSubcommand {
    /// Open the API overview
    Root,
    /// Open a docs path directly
    Path { path: Option<String> },
}
