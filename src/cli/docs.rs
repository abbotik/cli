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
    Root(DocsRootCommand),
    /// Open a docs path directly
    Path(DocsPathCommand),
}

#[derive(Args, Debug, Default)]
#[command(after_long_help = DOCS_ROOT_AFTER_HELP)]
pub struct DocsRootCommand {}

#[derive(Args, Debug)]
#[command(after_long_help = DOCS_PATH_AFTER_HELP)]
pub struct DocsPathCommand {
    pub path: Option<String>,
}
