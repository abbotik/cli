use super::*;

pub(super) async fn run(command: FsCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        crate::cli::FsSubcommand::Get(arg) => {
            if command.options.stat {
                print_json(
                    &client
                        .get_json::<Value>(&format!("/fs/{}?stat=true", arg.path))
                        .await?,
                )?
            } else {
                print_text(&client.get_text(&format!("/fs/{}", arg.path)).await?)?
            }
        }
        crate::cli::FsSubcommand::Put(arg) => {
            let content = fs_body_text(&command.options)?;
            print_text(
                &client
                    .request_text(Method::PUT, &format!("/fs/{}", arg.path), Some(&content))
                    .await?,
            )?;
        }
        crate::cli::FsSubcommand::Delete(arg) => print_json(
            &client
                .delete_json::<Value>(&format!("/fs/{}", arg.path))
                .await?,
        )?,
    }
    Ok(())
}

fn fs_body_text(options: &FsOptions) -> anyhow::Result<String> {
    match options.body.as_deref() {
        Some("-") => read_stdin_or_empty(),
        Some(source) if source.starts_with('@') => Ok(stdfs::read_to_string(&source[1..])?),
        Some(source) => Ok(source.to_string()),
        None => read_stdin_or_empty(),
    }
}
