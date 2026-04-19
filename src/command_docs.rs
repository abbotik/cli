use clap::CommandFactory;
use include_dir::{include_dir, Dir};

use crate::cli::Cli;

static HELP_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/docs/help");

pub(crate) fn render_command_doc(path: &[String]) -> anyhow::Result<String> {
    validate_command_path(path)?;

    let resolved_path = nearest_doc_path(path).ok_or_else(|| {
        anyhow::anyhow!(
            "no embedded markdown doc found for `{}` or any parent command",
            display_command_path(path)
        )
    })?;
    let body = read_doc_body(&resolved_path)?;

    if resolved_path == path {
        Ok(body.to_string())
    } else {
        Ok(format!(
            "> No dedicated markdown doc for `{}`. Showing `{}`.\n\n{}",
            display_command_path(path),
            display_command_path(&resolved_path),
            body
        ))
    }
}

fn validate_command_path(path: &[String]) -> anyhow::Result<()> {
    let mut command = Cli::command();

    for (index, segment) in path.iter().enumerate() {
        let next = command
            .get_subcommands()
            .find(|subcommand| subcommand.get_name() == segment)
            .cloned()
            .ok_or_else(|| unknown_command_error(&command, path, index))?;
        command = next;
    }

    Ok(())
}

fn unknown_command_error(command: &clap::Command, path: &[String], index: usize) -> anyhow::Error {
    let attempted = display_command_path(&path[..=index]);
    let mut choices = command
        .get_subcommands()
        .map(|subcommand| subcommand.get_name())
        .filter(|name| *name != "help")
        .collect::<Vec<_>>();
    choices.sort_unstable();

    if choices.is_empty() {
        anyhow::anyhow!("unknown command path `{attempted}`")
    } else {
        anyhow::anyhow!(
            "unknown command path `{attempted}`; expected one of: {}",
            choices.join(", ")
        )
    }
}

fn nearest_doc_path(path: &[String]) -> Option<Vec<String>> {
    for len in (0..=path.len()).rev() {
        let candidate = path[..len].to_vec();
        if HELP_DIR.get_file(doc_filename(&candidate)).is_some() {
            return Some(candidate);
        }
    }

    None
}

fn read_doc_body(path: &[String]) -> anyhow::Result<&'static str> {
    HELP_DIR
        .get_file(doc_filename(path))
        .and_then(|file| file.contents_utf8())
        .ok_or_else(|| anyhow::anyhow!("embedded doc file `{}` is missing", doc_filename(path)))
}

fn doc_filename(path: &[String]) -> String {
    if path.is_empty() {
        "abbot.md".to_string()
    } else {
        format!("abbot-{}.md", path.join("-"))
    }
}

fn display_command_path(path: &[String]) -> String {
    if path.is_empty() {
        "abbot".to_string()
    } else {
        format!("abbot {}", path.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_root_doc() {
        let text = render_command_doc(&[]).expect("root doc should render");
        assert!(text.contains("# abbot"));
    }

    #[test]
    fn falls_back_to_parent_doc_when_leaf_markdown_is_missing() {
        let path = vec!["llm".to_string(), "room".to_string(), "list".to_string()];
        let text = render_command_doc(&path).expect("parent doc should render");

        assert!(text.contains("No dedicated markdown doc"));
        assert!(text.contains("`abbot llm room list`"));
        assert!(text.contains("`abbot llm room`"));
    }

    #[test]
    fn rejects_unknown_command_paths() {
        let path = vec!["init".to_string()];
        let error = render_command_doc(&path).expect_err("unknown command should fail");
        assert!(error
            .to_string()
            .contains("unknown command path `abbot init`"));
    }
}
