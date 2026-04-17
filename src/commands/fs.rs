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

#[cfg(test)]
mod tests {
    use super::fs_body_text;
    use crate::cli::FsOptions;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn fs_body_text_returns_inline_body() {
        let body = fs_body_text(&FsOptions {
            stat: false,
            body: Some("hello".to_string()),
        })
        .expect("body should build");

        assert_eq!(body, "hello");
    }

    #[test]
    fn fs_body_text_reads_at_prefixed_file() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("abbot-fs-body-{stamp}.txt"));
        fs::write(&path, "from-file").expect("temp file should write");

        let body = fs_body_text(&FsOptions {
            stat: false,
            body: Some(format!("@{}", path.display())),
        })
        .expect("body should read");

        assert_eq!(body, "from-file");
        let _ = fs::remove_file(path);
    }
}
