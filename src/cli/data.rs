use super::*;

#[derive(Args, Debug, Default, Clone)]
pub struct DataOptions {
    /// Include soft-deleted records
    #[arg(long)]
    pub include_trashed: bool,

    /// Include permanently deleted records
    #[arg(long)]
    pub include_deleted: bool,

    /// Remove the success envelope from responses
    #[arg(long)]
    pub unwrap: bool,

    /// Select a comma-separated field list
    #[arg(long)]
    pub select: Option<String>,

    /// Apply a JSON where filter
    #[arg(long = "where")]
    pub r#where: Option<String>,

    /// Limit the number of returned records
    #[arg(long)]
    pub limit: Option<u32>,

    /// Exclude timestamp fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub stat: Option<bool>,

    /// Exclude ACL fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub access: Option<bool>,

    /// Perform permanent delete
    #[arg(long)]
    pub permanent: bool,

    /// Enable upsert mode for creates
    #[arg(long)]
    pub upsert: bool,
}

#[derive(Args, Debug)]
#[command(after_long_help = DATA_AFTER_HELP)]
pub struct DataCommand {
    #[command(flatten)]
    pub options: DataOptions,

    #[command(subcommand)]
    pub command: DataSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DATA_AFTER_HELP)]
pub enum DataSubcommand {
    /// List records for a model via GET /api/data/:model
    List(ModelArg),
    /// Create one or more records via POST /api/data/:model
    Create(ModelArg),
    /// Bulk update records by id via PUT /api/data/:model
    Update(ModelArg),
    /// Bulk update records by filter via PATCH /api/data/:model
    Patch(ModelArg),
    /// Soft delete records via DELETE /api/data/:model
    Delete(ModelArg),
    /// Fetch a single record via GET /api/data/:model/:id
    Get(RecordArg),
    /// Update a single record via PUT /api/data/:model/:id
    Put(RecordArg),
    /// Patch a single record via PATCH /api/data/:model/:id
    #[command(name = "patch-record")]
    PatchRecord(RecordArg),
    /// Soft delete a single record via DELETE /api/data/:model/:id
    DeleteRecord(RecordArg),
    /// Work with owned relationship routes under /api/data/:model/:id/:relationship
    Relationship(RelationshipArg),
}

#[derive(Args, Debug)]
#[command(after_long_help = DATA_RELATIONSHIP_AFTER_HELP)]
pub struct RelationshipArg {
    pub model: String,
    pub id: String,
    pub relationship: String,
    #[command(subcommand)]
    pub command: RelationshipSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DATA_RELATIONSHIP_AFTER_HELP)]
pub enum RelationshipSubcommand {
    /// List child records via GET /api/data/:model/:id/:relationship
    Get,
    /// Create a child record via POST /api/data/:model/:id/:relationship
    Create,
    /// Bulk update child records via PUT /api/data/:model/:id/:relationship
    Update,
    /// Soft delete child records via DELETE /api/data/:model/:id/:relationship
    Delete,
    /// Address a specific nested child record
    Child(RelationshipChildCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = DATA_RELATIONSHIP_CHILD_AFTER_HELP)]
pub struct RelationshipChildCommand {
    pub child: String,
    #[command(subcommand)]
    pub command: RelationshipChildSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DATA_RELATIONSHIP_CHILD_AFTER_HELP)]
pub enum RelationshipChildSubcommand {
    /// Fetch a nested child record via GET /api/data/:model/:id/:relationship/:child
    Get,
    /// Update a nested child record via PUT /api/data/:model/:id/:relationship/:child
    Put,
    /// Patch a nested child record via PATCH /api/data/:model/:id/:relationship/:child
    Patch,
    /// Soft delete a nested child record via DELETE /api/data/:model/:id/:relationship/:child
    Delete,
}
