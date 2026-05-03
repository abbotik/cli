use super::*;

#[derive(Args, Debug, Clone)]
pub struct ModelArg {
    pub model: String,
}

#[derive(Args, Debug, Clone)]
pub struct FieldArg {
    pub model: String,
    pub field: String,
}

#[derive(Args, Debug, Clone)]
pub struct RecordArg {
    pub model: String,
    pub id: String,
}

#[derive(Args, Debug, Clone)]
pub struct TrackedRecordArg {
    pub model: String,
    pub id: String,
    pub change: String,
}

#[derive(Args, Debug, Clone)]
pub struct TrashedModelArg {
    pub model: String,
}

#[derive(Args, Debug, Clone)]
pub struct UserIdArg {
    pub id: String,
}

#[derive(Args, Debug, Clone)]
pub struct UserApiKeyIdArg {
    pub key_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct CronIdArg {
    pub pid: String,
}

#[derive(Args, Debug, Clone)]
pub struct PathArg {
    pub path: String,
}

#[derive(Args, Debug, Clone)]
pub struct RoomIdArg {
    pub id: String,
}

#[derive(Args, Debug, Clone)]
pub struct FactoryRunIdArg {
    /// Factory run id
    #[arg(value_name = "RUN")]
    pub id: String,
}

#[derive(Args, Debug, Clone)]
pub struct FactoryRunStageArg {
    /// Factory run id
    #[arg(value_name = "RUN")]
    pub id: String,

    /// Factory stage id
    #[arg(value_name = "STAGE")]
    pub stage_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct FactoryRunIssueArg {
    /// Factory run id
    #[arg(value_name = "RUN")]
    pub id: String,

    /// Factory issue id
    #[arg(value_name = "ISSUE")]
    pub issue_id: String,
}
