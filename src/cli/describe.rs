use super::*;

#[derive(Args, Debug)]
pub struct DescribeCommand {
    #[command(subcommand)]
    pub command: DescribeSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum DescribeSubcommand {
    List,
    Get(ModelArg),
    Create(ModelArg),
    Update(ModelArg),
    Delete(ModelArg),
    Fields(DescribeFieldsCommand),
}

#[derive(Args, Debug)]
pub struct DescribeFieldsCommand {
    #[command(subcommand)]
    pub command: DescribeFieldsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum DescribeFieldsSubcommand {
    List(ModelArg),
    BulkCreate(ModelArg),
    BulkUpdate(ModelArg),
    Get(FieldArg),
    Create(FieldArg),
    Update(FieldArg),
    Delete(FieldArg),
}
