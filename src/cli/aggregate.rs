use super::*;

#[derive(Args, Debug, Default, Clone)]
pub struct AggregateOptions {
    /// Count all records
    #[arg(long)]
    pub count: bool,

    /// Sum of field values
    #[arg(long)]
    pub sum: Option<String>,

    /// Average of field values
    #[arg(long)]
    pub avg: Option<String>,

    /// Minimum field value
    #[arg(long)]
    pub min: Option<String>,

    /// Maximum field value
    #[arg(long)]
    pub max: Option<String>,

    /// Apply a JSON where filter from stdin, a file (@path), or inline JSON
    #[arg(long = "where")]
    pub r#where: Option<String>,

    /// Full POST body from stdin, a file (@path), or inline JSON
    #[arg(long)]
    pub body: Option<String>,
}

#[derive(Args, Debug)]
#[command(after_long_help = AGGREGATE_AFTER_HELP)]
pub struct AggregateCommand {
    #[command(flatten)]
    pub options: AggregateOptions,

    #[command(subcommand)]
    pub command: AggregateSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = AGGREGATE_AFTER_HELP)]
pub enum AggregateSubcommand {
    Get(ModelArg),
    Run(ModelArg),
}
