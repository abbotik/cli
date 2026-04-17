use super::*;

#[derive(Args, Debug)]
#[command(after_long_help = DESCRIBE_AFTER_HELP)]
pub struct DescribeCommand {
    #[command(subcommand)]
    pub command: DescribeSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DESCRIBE_AFTER_HELP)]
pub enum DescribeSubcommand {
    List,
    Get(ModelArg),
    Create(ModelArg),
    Update(ModelArg),
    Delete(ModelArg),
    Fields(DescribeFieldsCommand),
}

#[derive(Args, Debug)]
#[command(after_long_help = DESCRIBE_FIELDS_AFTER_HELP)]
pub struct DescribeFieldsCommand {
    #[command(subcommand)]
    pub command: DescribeFieldsSubcommand,
}

#[derive(Subcommand, Debug)]
#[command(after_long_help = DESCRIBE_FIELDS_AFTER_HELP)]
pub enum DescribeFieldsSubcommand {
    List(ModelArg),
    BulkCreate(ModelArg),
    BulkUpdate(ModelArg),
    Get(FieldArg),
    Create(FieldArg),
    Update(FieldArg),
    Delete(FieldArg),
}
