use super::*;

pub(super) async fn run(command: StatCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::StatSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/stat/{}/{}", arg.model, arg.id))
                .await?,
        )?,
    }
    Ok(())
}
