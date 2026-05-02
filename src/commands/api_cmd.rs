use super::*;

pub(super) async fn run(command: ApiCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        ApiSubcommand::Acls(command) => acls::run(command, client).await?,
        ApiSubcommand::Aggregate(command) => aggregate::run(command, client).await?,
        ApiSubcommand::Bulk(command) => bulk::run(command, client).await?,
        ApiSubcommand::Cron(command) => cron::run(command, client).await?,
        ApiSubcommand::Data(command) => data::run(command, client).await?,
        ApiSubcommand::Describe(command) => describe::run(command, client).await?,
        ApiSubcommand::Find(command) => find::run(command, client).await?,
        ApiSubcommand::Keys(command) => keys::run(command, client).await?,
        ApiSubcommand::Stat(command) => stat::run(command, client).await?,
        ApiSubcommand::Tracked(command) => tracked::run(command, client).await?,
        ApiSubcommand::Trashed(command) => trashed::run(command, client).await?,
        ApiSubcommand::User(command) => user::run(command, client).await?,
    }

    Ok(())
}
