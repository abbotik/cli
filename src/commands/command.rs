use super::*;

pub(super) async fn run(command: CommandDocsCommand) -> anyhow::Result<()> {
    print_text(&crate::command_docs::render_command_doc(&command.path)?)?;
    Ok(())
}
