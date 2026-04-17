use super::*;

pub(super) async fn run(command: PublicCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::PublicSubcommand::Root => print_text(&client.get_text("/").await?)?,
        crate::cli::PublicSubcommand::Llms => print_text(&client.get_text("/llms.txt").await?)?,
    }
    Ok(())
}
