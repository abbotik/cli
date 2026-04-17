use super::*;

pub(super) async fn run(command: TrashedCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::TrashedSubcommand::List => {
            print_json(&client.get_json::<Value>("/api/trashed").await?)?
        }
        crate::cli::TrashedSubcommand::Model(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/trashed/{}", arg.model))
                .await?,
        )?,
        crate::cli::TrashedSubcommand::Record(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/trashed/{}/{}", arg.model, arg.id))
                .await?,
        )?,
    }
    Ok(())
}
