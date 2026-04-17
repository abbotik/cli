use super::*;

pub(super) async fn run(command: DescribeCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::DescribeSubcommand::List => {
            print_json(&client.get_json::<Value>("/api/describe").await?)?
        }
        crate::cli::DescribeSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/describe/{}", arg.model))
                .await?,
        )?,
        crate::cli::DescribeSubcommand::Create(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/describe/{}", arg.model),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DescribeSubcommand::Update(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/describe/{}", arg.model),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DescribeSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/describe/{}", arg.model))
                .await?,
        )?,
        crate::cli::DescribeSubcommand::Fields(fields) => describe_fields(fields, client).await?,
    }
    Ok(())
}

async fn describe_fields(
    command: crate::cli::DescribeFieldsCommand,
    client: &ApiClient,
) -> anyhow::Result<()> {
    match command.command {
        crate::cli::DescribeFieldsSubcommand::List(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/describe/{}/fields", arg.model))
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::BulkCreate(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/describe/{}/fields", arg.model),
                    &read_json_body_or_default(json!([]))?,
                )
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::BulkUpdate(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/describe/{}/fields", arg.model),
                    &read_json_body_or_default(json!([]))?,
                )
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/describe/{}/fields/{}", arg.model, arg.field))
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::Create(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/describe/{}/fields/{}", arg.model, arg.field),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::Update(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/describe/{}/fields/{}", arg.model, arg.field),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DescribeFieldsSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/describe/{}/fields/{}", arg.model, arg.field))
                .await?,
        )?,
    }
    Ok(())
}
