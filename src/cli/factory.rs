use super::*;

#[derive(Args, Debug)]
pub struct FactoryCommand {
    #[command(subcommand)]
    pub command: FactorySubcommand,
}

#[derive(Subcommand, Debug)]
pub enum FactorySubcommand {
    /// Create a factory run from a prompt or plan file
    Create(FactoryCreateCommand),
    /// Start or wake a factory run
    Start(FactoryRunIdArg),
    /// Read aggregate run status
    Status(FactoryRunIdArg),
    /// Read one status snapshot for a run
    Watch(FactoryRunIdArg),
    /// List emitted artifacts for one run
    Artifacts(FactoryRunIdArg),
    /// Read the latest review bundle
    Review(FactoryRunIdArg),
}

#[derive(Args, Debug)]
pub struct FactoryCreateCommand {
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
