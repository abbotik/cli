use anyhow::Context;

use super::*;

pub(super) async fn run(_command: TuiCommand, client: &ApiClient) -> anyhow::Result<()> {
    crate::tui::run(client.clone())
        .await
        .context("abbot tui failed")
}
