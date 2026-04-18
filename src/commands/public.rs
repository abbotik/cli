use super::*;

pub(super) async fn run(command: PublicCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::PublicSubcommand::Root(_) => print_text(&client.get_text("/").await?)?,
        crate::cli::PublicSubcommand::Llms(_) => print_text(&client.get_text("/llms.txt").await?)?,
    }
    Ok(())
}
