use super::*;

pub(super) async fn run(command: DataCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::DataSubcommand::List(arg) => print_json(
            &client
                .get_json_with_query::<_, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data_helpers::query_pairs(&command.options),
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Create(arg) => print_json(
            &client
                .post_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data_helpers::query_pairs(&command.options),
                    &read_json_body_or_default(json!([]))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Update(arg) => print_json(
            &client
                .put_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data_helpers::query_pairs(&command.options),
                    &read_json_body_or_default(json!([]))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Patch(arg) => print_json(
            &client
                .patch_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data_helpers::query_pairs(&command.options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Delete(arg) => print_json(
            &client
                .delete_json_with_query::<_, Value>(
                    &format!("/api/data/{}", arg.model),
                    &data_helpers::query_pairs(&command.options),
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Get(arg) => print_json(
            &client
                .get_json_with_query::<_, Value>(
                    &format!("/api/data/{}/{}", arg.model, arg.id),
                    &data_helpers::query_pairs(&command.options),
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Put(arg) => print_json(
            &client
                .put_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}/{}", arg.model, arg.id),
                    &data_helpers::query_pairs(&command.options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::PatchRecord(arg) => print_json(
            &client
                .patch_json_with_query::<_, _, Value>(
                    &format!("/api/data/{}/{}", arg.model, arg.id),
                    &data_helpers::query_pairs(&command.options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::DeleteRecord(arg) => print_json(
            &client
                .delete_json_with_query::<_, Value>(
                    &format!("/api/data/{}/{}", arg.model, arg.id),
                    &data_helpers::query_pairs(&command.options),
                )
                .await?,
        )?,
        crate::cli::DataSubcommand::Relationship(arg) => {
            relationship(arg, client, &command.options).await?
        }
    }
    Ok(())
}

async fn relationship(
    command: crate::cli::RelationshipArg,
    client: &ApiClient,
    options: &crate::cli::DataOptions,
) -> anyhow::Result<()> {
    let base = format!(
        "/api/data/{}/{}/{}",
        command.model, command.id, command.relationship
    );
    match command.command {
        crate::cli::RelationshipSubcommand::Get => print_json::<Value>(
            &client
                .get_json_with_query::<_, Value>(&base, &data_helpers::query_pairs(options))
                .await?,
        )?,
        crate::cli::RelationshipSubcommand::Create => print_json(
            &client
                .post_json_with_query::<_, _, Value>(
                    &base,
                    &data_helpers::query_pairs(options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::RelationshipSubcommand::Update => print_json(
            &client
                .put_json_with_query::<_, _, Value>(
                    &base,
                    &data_helpers::query_pairs(options),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::RelationshipSubcommand::Delete => print_json::<Value>(
            &client
                .delete_json_with_query::<_, Value>(&base, &data_helpers::query_pairs(options))
                .await?,
        )?,
        crate::cli::RelationshipSubcommand::Child(child) => {
            let path = format!("{}/{}", base, child.child);
            match child.command {
                crate::cli::RelationshipChildSubcommand::Get => {
                    print_json::<Value>(
                        &client
                            .get_json_with_query::<_, Value>(
                                &path,
                                &data_helpers::query_pairs(options),
                            )
                            .await?,
                    )?;
                }
                crate::cli::RelationshipChildSubcommand::Put => {
                    print_json::<Value>(
                        &client
                            .put_json_with_query::<_, _, Value>(
                                &path,
                                &data_helpers::query_pairs(options),
                                &read_json_body_or_default(json!({}))?,
                            )
                            .await?,
                    )?;
                }
                crate::cli::RelationshipChildSubcommand::Patch => {
                    print_json::<Value>(
                        &client
                            .patch_json_with_query::<_, _, Value>(
                                &path,
                                &data_helpers::query_pairs(options),
                                &read_json_body_or_default(json!({}))?,
                            )
                            .await?,
                    )?;
                }
                crate::cli::RelationshipChildSubcommand::Delete => {
                    print_json::<Value>(
                        &client
                            .delete_json_with_query::<_, Value>(
                                &path,
                                &data_helpers::query_pairs(options),
                            )
                            .await?,
                    )?;
                }
            }
        }
    }
    Ok(())
}
