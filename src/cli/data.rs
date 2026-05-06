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

#[derive(Args, Debug, Default, Clone)]
pub struct DataReadOptions {
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
}

#[derive(Args, Debug, Default, Clone)]
pub struct DataCreateOptions {
    /// Remove the success envelope from responses
    #[arg(long)]
    pub unwrap: bool,

    /// Exclude timestamp fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub stat: Option<bool>,

    /// Exclude ACL fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub access: Option<bool>,

    /// Enable upsert mode for creates
    #[arg(long)]
    pub upsert: bool,
}

#[derive(Args, Debug, Default, Clone)]
pub struct DataMutationOptions {
    /// Remove the success envelope from responses
    #[arg(long)]
    pub unwrap: bool,

    /// Exclude timestamp fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub stat: Option<bool>,

    /// Exclude ACL fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub access: Option<bool>,
}

#[derive(Args, Debug, Default, Clone)]
pub struct DataPatchOptions {
    /// Remove the success envelope from responses
    #[arg(long)]
    pub unwrap: bool,

    /// Apply a JSON where filter
    #[arg(long = "where")]
    pub r#where: Option<String>,

    /// Exclude timestamp fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub stat: Option<bool>,

    /// Exclude ACL fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub access: Option<bool>,
}

#[derive(Args, Debug, Default, Clone)]
pub struct DataDeleteOptions {
    /// Remove the success envelope from responses
    #[arg(long)]
    pub unwrap: bool,

    /// Apply a JSON where filter
    #[arg(long = "where")]
    pub r#where: Option<String>,

    /// Perform permanent delete
    #[arg(long)]
    pub permanent: bool,
}

#[derive(Args, Debug, Default, Clone)]
pub struct DataRecordReadOptions {
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

    /// Exclude timestamp fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub stat: Option<bool>,

    /// Exclude ACL fields
    #[arg(long, value_parser = clap::builder::BoolishValueParser::new())]
    pub access: Option<bool>,
}

#[derive(Args, Debug, Default, Clone)]
pub struct DataRecordDeleteOptions {
    /// Remove the success envelope from responses
    #[arg(long)]
    pub unwrap: bool,

    /// Perform permanent delete
    #[arg(long)]
    pub permanent: bool,
}

#[derive(Args, Debug)]
pub struct DataCommand {
    #[command(subcommand)]
    pub command: DataSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum DataSubcommand {
    /// List records for a model via GET /api/data/:model
    List(DataListArg),
    /// Create one or more records via POST /api/data/:model
    Create(DataCreateArg),
    /// Bulk update records by id via PUT /api/data/:model
    Update(DataUpdateArg),
    /// Bulk update records by filter via PATCH /api/data/:model
    Patch(DataPatchArg),
    /// Soft delete records via DELETE /api/data/:model
    Delete(DataDeleteArg),
    /// Fetch a single record via GET /api/data/:model/:id
    Get(DataGetArg),
    /// Update a single record via PUT /api/data/:model/:id
    Put(DataPutArg),
    /// Soft delete a single record via DELETE /api/data/:model/:id
    DeleteRecord(DataDeleteRecordArg),
    /// Work with owned relationship routes under /api/data/:model/:id/:relationship
    Relationship(RelationshipArg),
}

#[derive(Args, Debug)]
pub struct DataListArg {
    #[command(flatten)]
    pub options: DataReadOptions,
    pub model: String,
}

#[derive(Args, Debug)]
pub struct DataCreateArg {
    #[command(flatten)]
    pub options: DataCreateOptions,
    pub model: String,
}

#[derive(Args, Debug)]
pub struct DataUpdateArg {
    #[command(flatten)]
    pub options: DataMutationOptions,
    pub model: String,
}

#[derive(Args, Debug)]
pub struct DataPatchArg {
    #[command(flatten)]
    pub options: DataPatchOptions,
    pub model: String,
}

#[derive(Args, Debug)]
pub struct DataDeleteArg {
    #[command(flatten)]
    pub options: DataDeleteOptions,
    pub model: String,
}

#[derive(Args, Debug)]
pub struct DataGetArg {
    #[command(flatten)]
    pub options: DataRecordReadOptions,
    pub model: String,
    pub id: String,
}

#[derive(Args, Debug)]
pub struct DataPutArg {
    #[command(flatten)]
    pub options: DataMutationOptions,
    pub model: String,
    pub id: String,
}

#[derive(Args, Debug)]
pub struct DataDeleteRecordArg {
    #[command(flatten)]
    pub options: DataRecordDeleteOptions,
    pub model: String,
    pub id: String,
}

#[derive(Args, Debug)]
pub struct RelationshipArg {
    #[command(flatten)]
    pub options: DataReadOptions,
    pub model: String,
    pub id: String,
    pub relationship: String,
    #[command(subcommand)]
    pub command: RelationshipSubcommand,
}

#[derive(Subcommand, Debug)]
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
pub struct RelationshipChildCommand {
    pub child: String,
    #[command(subcommand)]
    pub command: RelationshipChildSubcommand,
}

#[derive(Subcommand, Debug)]
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

impl From<&DataReadOptions> for DataOptions {
    fn from(options: &DataReadOptions) -> Self {
        Self {
            include_trashed: options.include_trashed,
            include_deleted: options.include_deleted,
            unwrap: options.unwrap,
            select: options.select.clone(),
            r#where: options.r#where.clone(),
            limit: options.limit,
            stat: options.stat,
            access: options.access,
            permanent: false,
            upsert: false,
        }
    }
}

impl From<&DataCreateOptions> for DataOptions {
    fn from(options: &DataCreateOptions) -> Self {
        Self {
            unwrap: options.unwrap,
            stat: options.stat,
            access: options.access,
            upsert: options.upsert,
            ..Self::default()
        }
    }
}

impl From<&DataMutationOptions> for DataOptions {
    fn from(options: &DataMutationOptions) -> Self {
        Self {
            unwrap: options.unwrap,
            stat: options.stat,
            access: options.access,
            ..Self::default()
        }
    }
}

impl From<&DataPatchOptions> for DataOptions {
    fn from(options: &DataPatchOptions) -> Self {
        Self {
            unwrap: options.unwrap,
            r#where: options.r#where.clone(),
            stat: options.stat,
            access: options.access,
            ..Self::default()
        }
    }
}

impl From<&DataDeleteOptions> for DataOptions {
    fn from(options: &DataDeleteOptions) -> Self {
        Self {
            unwrap: options.unwrap,
            r#where: options.r#where.clone(),
            permanent: options.permanent,
            ..Self::default()
        }
    }
}

impl From<&DataRecordReadOptions> for DataOptions {
    fn from(options: &DataRecordReadOptions) -> Self {
        Self {
            include_trashed: options.include_trashed,
            include_deleted: options.include_deleted,
            unwrap: options.unwrap,
            select: options.select.clone(),
            stat: options.stat,
            access: options.access,
            ..Self::default()
        }
    }
}

impl From<&DataRecordDeleteOptions> for DataOptions {
    fn from(options: &DataRecordDeleteOptions) -> Self {
        Self {
            unwrap: options.unwrap,
            permanent: options.permanent,
            ..Self::default()
        }
    }
}
