use super::*;

#[derive(Args, Debug)]
#[command(after_long_help = LLM_AFTER_HELP)]
pub struct LlmCommand {
    #[command(subcommand)]
    pub command: LlmSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = LLM_AFTER_HELP)]
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
#[command(after_long_help = LLM_PROVIDERS_AFTER_HELP)]
pub struct LlmProvidersCommand {}

#[derive(Args, Debug, Default)]
#[command(after_long_help = LLM_MODELS_AFTER_HELP)]
pub struct LlmModelsCommand {}

#[derive(Args, Debug, Default)]
#[command(after_long_help = LLM_SKILLS_AFTER_HELP)]
pub struct LlmSkillsCommand {}

#[derive(Args, Debug)]
#[command(after_long_help = LLM_ROOM_AFTER_HELP)]
pub struct LlmRoomCommand {
    #[command(subcommand)]
    pub command: LlmRoomSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = LLM_ROOM_AFTER_HELP)]
pub enum LlmRoomSubcommand {
    /// List visible rooms
    List,
    /// Rent a new room
    Create,
    /// Fetch one room by ID
    Get(RoomIdArg),
    /// Update mutable room configuration
    Update(RoomIdArg),
    /// Inject one semantic room message
    Message(RoomIdArg),
    /// Wake a room explicitly
    Wake(RoomIdArg),
    /// Replay durable room events and optionally follow live SSE
    Events(LlmRoomEventsCommand),
    /// Read durable room history
    History(RoomIdArg),
    /// Interrupt an in-flight turn
    Interrupt(RoomIdArg),
    /// Release a room explicitly
    Release(RoomIdArg),
}

#[derive(Args, Debug)]
#[command(after_long_help = LLM_ROOM_AFTER_HELP)]
pub struct LlmRoomEventsCommand {
    pub id: String,

    /// Keep the SSE stream attached after replaying durable history
    #[arg(long)]
    pub follow: bool,
}

#[derive(Args, Debug)]
#[command(after_long_help = LLM_FACTORY_AFTER_HELP)]
pub struct LlmFactoryCommand {
    #[command(subcommand)]
    pub command: LlmFactorySubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = LLM_FACTORY_AFTER_HELP)]
pub enum LlmFactorySubcommand {
    /// List visible factory runs
    List,
    /// Start a new factory run
    Create,
    /// Fetch one factory run by ID
    Get(FactoryRunIdArg),
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
    /// List emitted artifacts for one run
    Artifacts(FactoryRunIdArg),
    /// Advance the run state machine
    Advance(FactoryRunIdArg),
    /// Execute verification and persist the report
    Verify(FactoryRunIdArg),
    /// Evaluate and persist a gate verdict
    GateCheck(FactoryRunIdArg),
    /// Read the latest review bundle
    Review(FactoryRunIdArg),
}
