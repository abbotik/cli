use super::*;

pub(super) async fn run(command: CronCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::CronSubcommand::List => {
            print_json(&client.get_json::<Value>("/api/cron").await?)?
        }
        crate::cli::CronSubcommand::Create => print_json(
            &client
                .post_json::<_, Value>("/api/cron", &read_json_body_or_default(json!({}))?)
                .await?,
        )?,
        crate::cli::CronSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/api/cron/{}", arg.pid))
                .await?,
        )?,
        crate::cli::CronSubcommand::Update(arg) => print_json(
            &client
                .patch_json::<_, Value>(
                    &format!("/api/cron/{}", arg.pid),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::CronSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/api/cron/{}", arg.pid))
                .await?,
        )?,
        crate::cli::CronSubcommand::Enable(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/cron/{}/enable", arg.pid),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        crate::cli::CronSubcommand::Disable(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/api/cron/{}/disable", arg.pid),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
    }
    Ok(())
}
