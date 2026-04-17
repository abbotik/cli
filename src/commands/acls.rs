use super::*;

pub(super) async fn run(command: AclsCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::AclsSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/acls/{}/{}", arg.model, arg.id))
                .await?,
        )?,
        crate::cli::AclsSubcommand::Create(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/acls/{}/{}", arg.model, arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::AclsSubcommand::Update(arg) => print_json(
            &client
                .put_json::<_, Value>(
                    &format!("/api/acls/{}/{}", arg.model, arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::AclsSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/acls/{}/{}", arg.model, arg.id))
                .await?,
        )?,
    }
    Ok(())
}
