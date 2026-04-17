use super::*;

pub(super) async fn run(command: AppCommand, client: &ApiClient) -> anyhow::Result<()> {
    let path = command.path.unwrap_or_default();
    let full_path = if path.is_empty() {
        format!("/app/{}", command.app_name)
    } else {
        format!("/app/{}/{}", command.app_name, path.trim_start_matches('/'))
    };
    print_text(&client.get_text(&full_path).await?)?;
    Ok(())
}
