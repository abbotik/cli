use super::*;

#[derive(Args, Debug, Default, Clone)]
pub struct FindOptions {
    /// Project a comma-separated field list
    #[arg(long)]
    pub select: Option<String>,

    /// Apply a JSON where filter from stdin, a file (@path), or inline JSON
    #[arg(long = "where")]
    pub r#where: Option<String>,

    /// Apply a comma-separated order list
    #[arg(long)]
    pub order: Option<String>,

    /// Limit the number of returned records
    #[arg(long)]
    pub limit: Option<u32>,

    /// Skip the first N matching records
    #[arg(long)]
    pub offset: Option<u32>,
}

#[derive(Args, Debug)]
#[command(after_long_help = FIND_AFTER_HELP)]
pub struct FindCommand {
    #[command(flatten)]
    pub options: FindOptions,

    #[command(subcommand)]
    pub command: FindSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = FIND_AFTER_HELP)]
pub enum FindSubcommand {
    Query(ModelArg),
    Saved(FindSavedArg),
}

#[derive(Args, Debug)]
pub struct FindSavedArg {
    pub model: String,
    pub target: String,
}
