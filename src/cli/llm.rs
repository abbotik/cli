use super::*;

#[derive(Args, Debug)]
pub struct LlmCommand {
    #[command(subcommand)]
    pub command: LlmSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum LlmSubcommand {
    /// List enabled rentable models grouped by provider
    Providers(LlmProvidersCommand),
    /// List enabled rentable model descriptors
    Models(LlmModelsCommand),
    /// List known room skills
    Skills(LlmSkillsCommand),
    /// Manage bounded live LLM rooms
    Room(LlmRoomCommand),
    /// Manage durable factory runs, stages, issues, and review state
    Factory(LlmFactoryCommand),
}

#[derive(Args, Debug, Default)]
pub struct LlmProvidersCommand {}

#[derive(Args, Debug, Default)]
pub struct LlmModelsCommand {}

#[derive(Args, Debug, Default)]
pub struct LlmSkillsCommand {}

#[derive(Args, Debug)]
pub struct LlmRoomCommand {
    #[command(subcommand)]
    pub command: LlmRoomSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum LlmRoomSubcommand {
    /// List visible rooms
    List,
    /// Rent a new room
    Create(LlmRoomCreateCommand),
    /// Fetch one room by ID
    Get(RoomIdArg),
    /// Update mutable room configuration
    Update(RoomIdArg),
    /// Inject one semantic room message
    Message(RoomIdArg),
    /// Wake a room explicitly
    Wake(RoomIdArg),
    /// Send one prompt to a room and wait for the result
    Run(LlmRoomRunCommand),
    /// Replay durable room events and optionally follow live SSE
    Events(LlmRoomEventsCommand),
    /// Read durable room history
    History(RoomIdArg),
    /// Interrupt an in-flight turn
    Interrupt(RoomIdArg),
    /// Release a room explicitly
    Release(LlmRoomReleaseCommand),
}

#[derive(Args, Debug, Default)]
pub struct LlmRoomCreateCommand {
    /// Stable CLI name for the room
    #[arg(long)]
    pub name: Option<String>,

    /// Provider to rent, such as openrouter
    #[arg(long)]
    pub provider: Option<String>,

    /// Model to rent, such as openai/gpt-5.4
    #[arg(long)]
    pub model: Option<String>,

    /// Human-readable room purpose
    #[arg(long)]
    pub purpose: Option<String>,

    /// Agent id stored in the room roster
    #[arg(long = "agent-id")]
    pub agent_id: Option<String>,

    /// Agent role stored in the room roster
    #[arg(long, default_value = "assistant")]
    pub role: String,

    /// Agent adapter
    #[arg(long, default_value = "pi")]
    pub adapter: String,
}

#[derive(Args, Debug)]
pub struct LlmRoomRunCommand {
    /// Prompt to send to the room
    pub prompt: String,

    /// Stable CLI room name
    #[arg(long)]
    pub name: Option<String>,

    /// Room id, when not using --name
    #[arg(long)]
    pub id: Option<String>,

    /// Stream assistant text to stdout as it arrives
    #[arg(long)]
    pub stream: bool,

    /// Print room event diagnostics to stderr while waiting
    #[arg(long)]
    pub debug: bool,

    /// Maximum time to wait for an agent output
    #[arg(long = "timeout-seconds", default_value_t = 120)]
    pub timeout_seconds: u64,

    /// Poll interval while waiting
    #[arg(long = "poll-seconds", default_value_t = 1)]
    pub poll_seconds: u64,
}

#[derive(Args, Debug)]
pub struct LlmRoomReleaseCommand {
    /// Room id, when not using --name or --id
    #[arg(value_name = "ID")]
    pub positional_id: Option<String>,

    /// Stable CLI room name
    #[arg(long)]
    pub name: Option<String>,

    /// Room id, when not using positional ID
    #[arg(long)]
    pub id: Option<String>,
}

#[derive(Args, Debug)]
pub struct LlmRoomEventsCommand {
    pub id: String,

    /// Keep the SSE stream attached after replaying durable history
    #[arg(long)]
    pub follow: bool,
}

#[derive(Args, Debug)]
pub struct LlmFactoryCommand {
    #[command(subcommand)]
    pub command: LlmFactorySubcommand,
}

#[derive(Subcommand, Debug)]
pub enum LlmFactorySubcommand {
    /// List visible factory runs
    List,
    /// Start a new factory run
    Create,
    /// Fetch one factory run by ID
    Get(FactoryRunIdArg),
    /// Start or wake the factory supervisor for one run
    Start(FactoryRunIdArg),
    /// Read aggregate run status
    Status(FactoryRunIdArg),
    /// List checkpoint records for one run
    Checkpoints(FactoryRunIdArg),
    /// Create a checkpoint record for one run
    CreateCheckpoint(FactoryRunIdArg),
    /// List stage records for one run
    Stages(FactoryRunIdArg),
    /// Create a stage record for one run
    CreateStage(FactoryRunIdArg),
    /// Update one stage record
    UpdateStage(FactoryRunStageArg),
    /// List issue records for one run
    Issues(FactoryRunIdArg),
    /// Create an issue record for one run
    CreateIssue(FactoryRunIdArg),
    /// Update one issue record
    UpdateIssue(FactoryRunIssueArg),
    /// Dispatch one issue with caller-supplied prompt content
    DispatchIssue(FactoryRunIssueArg),
    /// List emitted artifacts for one run
    Artifacts(FactoryRunIdArg),
    /// Create an artifact record for one run
    CreateArtifact(FactoryRunIdArg),
    /// Advance the run state machine
    Advance(FactoryRunIdArg),
    /// Execute verification and persist the report
    Verify(FactoryRunIdArg),
    /// Evaluate and persist a gate verdict
    GateCheck(FactoryRunIdArg),
    /// Create an externally supplied gate verdict
    CreateGate(FactoryRunIdArg),
    /// Read the latest review bundle
    Review(FactoryRunIdArg),
}
