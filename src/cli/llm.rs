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
    Providers,
    Models,
    Skills,
    Room(LlmRoomCommand),
    Factory(LlmFactoryCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = LLM_ROOM_AFTER_HELP)]
pub struct LlmRoomCommand {
    #[command(subcommand)]
    pub command: LlmRoomSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = LLM_ROOM_AFTER_HELP)]
pub enum LlmRoomSubcommand {
    List,
    Create,
    Get(RoomIdArg),
    Update(RoomIdArg),
    Message(RoomIdArg),
    Wake(RoomIdArg),
    Events(LlmRoomEventsCommand),
    History(RoomIdArg),
    Interrupt(RoomIdArg),
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
    List,
    Create,
    Get(FactoryRunIdArg),
    Status(FactoryRunIdArg),
    Checkpoints(FactoryRunIdArg),
    CreateCheckpoint(FactoryRunIdArg),
    Stages(FactoryRunIdArg),
    CreateStage(FactoryRunIdArg),
    UpdateStage(FactoryRunStageArg),
    Issues(FactoryRunIdArg),
    CreateIssue(FactoryRunIdArg),
    UpdateIssue(FactoryRunIssueArg),
    Artifacts(FactoryRunIdArg),
    Advance(FactoryRunIdArg),
    Verify(FactoryRunIdArg),
    GateCheck(FactoryRunIdArg),
    Review(FactoryRunIdArg),
}
