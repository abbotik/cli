use super::*;

#[derive(Args, Debug)]
pub struct FactoryCommand {
    #[command(subcommand)]
    pub command: FactorySubcommand,
}

#[derive(Subcommand, Debug)]
pub enum FactorySubcommand {
    /// List visible factory runs
    List(FactoryJsonOutput),
    /// Submit prompt text to Factory and wake the run
    #[command(visible_alias = "create")]
    Submit(FactorySubmitCommand),
    /// Submit prompt text, wake the run, and wait
    Run(FactoryRunCommand),
    /// Start or wake a factory run
    Start(FactoryRunOutputCommand),
    /// Read aggregate run status
    Status(FactoryRunOutputCommand),
    /// Cancel a factory run
    #[command(visible_alias = "stop")]
    Cancel(FactoryCancelCommand),
    /// Attach to a run until completion, failure, timeout, or attention
    Watch(FactoryWatchCommand),
    /// Read the latest review bundle
    Review(FactoryRunOutputCommand),
}

#[derive(Args, Debug)]
pub struct FactoryJsonOutput {
    /// Print raw JSON response
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct FactoryRunOutputCommand {
    /// Factory run id
    #[arg(value_name = "RUN")]
    pub id: String,

    /// Print raw JSON response
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct FactoryCancelCommand {
    /// Factory run id
    #[arg(value_name = "RUN")]
    pub id: String,

    /// Cancellation reason
    #[arg(long)]
    pub reason: Option<String>,

    /// Print raw JSON response
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct FactorySubmitCommand {
    /// Prompt text to compile into a factory run
    #[arg(value_name = "PROMPT", conflicts_with_all = ["prompt", "prompt_file"])]
    pub prompt_text: Option<String>,

    /// Prompt text to compile into a factory run
    #[arg(long = "prompt", value_name = "PROMPT", conflicts_with_all = ["prompt_text", "prompt_file"])]
    pub prompt: Option<String>,

    /// Read prompt text from PATH, or stdin with -
    #[arg(long = "prompt-file", value_name = "PATH", conflicts_with_all = ["prompt_text", "prompt"])]
    pub prompt_file: Option<std::path::PathBuf>,

    /// Workflow kind, for example software.delivery
    #[arg(long)]
    pub workflow: Option<String>,

    /// Subject in type:id form, for example repo:abbotik/api
    #[arg(long)]
    pub subject: Option<String>,

    /// Optional run title
    #[arg(long)]
    pub title: Option<String>,

    /// Print raw JSON response
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct FactoryRunCommand {
    #[command(flatten)]
    pub submit: FactorySubmitCommand,

    #[command(flatten)]
    pub wait: FactoryWaitOptions,
}

#[derive(Args, Debug)]
pub struct FactoryWaitOptions {
    /// Poll interval in seconds
    #[arg(long, default_value_t = 10)]
    pub interval: u64,

    /// Stop waiting after this many seconds
    #[arg(
        long = "wait-timeout",
        visible_alias = "timeout",
        default_value_t = 600
    )]
    pub timeout: u64,

    /// Stop condition to wait for
    #[arg(long, value_enum)]
    pub until: Option<FactoryWatchUntil>,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactoryWatchUntil {
    Completed,
    Failed,
    Cancelled,
    Blocked,
    Attention,
}

#[derive(Args, Debug)]
pub struct FactoryWatchCommand {
    /// Factory run id
    #[arg(value_name = "RUN")]
    pub id: String,

    #[command(flatten)]
    pub wait: FactoryWaitOptions,

    /// Print raw JSON status responses while watching
    #[arg(long)]
    pub json: bool,
}
