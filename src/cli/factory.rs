use super::*;

#[derive(Args, Debug)]
pub struct FactoryCommand {
    #[command(subcommand)]
    pub command: FactorySubcommand,
}

#[derive(Subcommand, Debug)]
pub enum FactorySubcommand {
    /// Submit a prompt or plan to Factory and wake the run
    #[command(visible_alias = "create")]
    Submit(FactorySubmitCommand),
    /// Start or wake a factory run
    Start(FactoryRunIdArg),
    /// Read aggregate run status
    Status(FactoryRunIdArg),
    /// Attach to a run until completion, failure, timeout, or attention
    Watch(FactoryWatchCommand),
    /// Read the latest review bundle
    Review(FactoryRunIdArg),
}

#[derive(Args, Debug)]
pub struct FactorySubmitCommand {
    /// Prompt text to compile into a factory run
    #[arg(long, conflicts_with = "plan")]
    pub prompt: Option<String>,

    /// Markdown or text plan file to compile into a factory run
    #[arg(long, value_name = "PATH", conflicts_with = "prompt")]
    pub plan: Option<std::path::PathBuf>,

    /// Workflow kind, for example software.delivery
    #[arg(long)]
    pub workflow: Option<String>,

    /// Subject in type:id form, for example repo:abbotik/api
    #[arg(long)]
    pub subject: Option<String>,

    /// Optional run title
    #[arg(long)]
    pub title: Option<String>,
}

#[derive(Args, Debug)]
pub struct FactoryWatchCommand {
    pub id: String,

    /// Poll interval in seconds
    #[arg(long, default_value_t = 10)]
    pub interval: u64,

    /// Stop watching after this many seconds
    #[arg(long)]
    pub timeout: Option<u64>,

    /// Stop condition to wait for
    #[arg(long, value_enum)]
    pub until: Option<FactoryWatchUntil>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactoryWatchUntil {
    Completed,
    Failed,
    Blocked,
    Attention,
}
