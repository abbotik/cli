use super::*;

pub(super) async fn run(command: DocsCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::DocsSubcommand::Root => print_text(&client.get_text("/docs").await?)?,
        crate::cli::DocsSubcommand::Path { path } => {
            let path = path.unwrap_or_else(|| "/docs".to_string());
            print_text(&client.get_text(&path).await?)?;
        }
    }
    Ok(())
}
