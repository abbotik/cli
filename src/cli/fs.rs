use super::*;

#[derive(Args, Debug, Default, Clone)]
pub struct FsOptions {
    /// Return filesystem metadata as JSON instead of file content
    #[arg(long)]
    pub stat: bool,

    /// File content from stdin, a file (@path), or inline text
    #[arg(long)]
    pub body: Option<String>,
}

#[derive(Args, Debug)]
pub struct FsCommand {
    #[command(flatten)]
    pub options: FsOptions,

    #[command(subcommand)]
    pub command: FsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum FsSubcommand {
    Get(PathArg),
    Put(PathArg),
    Delete(PathArg),
}
