use super::*;

pub(super) async fn run(command: TrackedCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::TrackedSubcommand::List(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/tracked/{}/{}", arg.model, arg.id))
                .await?,
        )?,
        crate::cli::TrackedSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!(
                    "/api/tracked/{}/{}/{}",
                    arg.model, arg.id, arg.change
                ))
                .await?,
        )?,
    }
    Ok(())
}
