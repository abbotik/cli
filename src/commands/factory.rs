use super::*;

pub(super) async fn run(command: FactoryCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        FactorySubcommand::Create(args) => print_json(
            &client
                .post_json::<_, Value>("/llm/factory/runs", &create_body(args)?)
                .await?,
        )?,
        FactorySubcommand::Start(arg) => print_json(
            &client
                .post_json::<_, Value>(&format!("/llm/factory/runs/{}/start", arg.id), &json!({}))
                .await?,
        )?,
        FactorySubcommand::Status(arg) | FactorySubcommand::Watch(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/status", arg.id))
                .await?,
        )?,
        FactorySubcommand::Artifacts(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/artifacts", arg.id))
                .await?,
        )?,
        FactorySubcommand::Review(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/review", arg.id))
                .await?,
        )?,
    }
    Ok(())
}

fn create_body(args: FactoryCreateCommand) -> anyhow::Result<Value> {
    let (kind, format, content) = match (args.prompt, args.plan) {
        (Some(prompt), None) => ("prompt", "text", prompt),
        (None, Some(path)) => {
            let content = stdfs::read_to_string(&path).map_err(|error| {
                anyhow::anyhow!("failed to read plan {}: {error}", path.display())
            })?;
            let format = if path.extension().and_then(|value| value.to_str()) == Some("md") {
                "markdown"
            } else {
                "text"
            };
            ("plan", format, content)
        }
        _ => anyhow::bail!("factory create requires exactly one of --prompt or --plan"),
    };

    let mut body = json!({
        "source": {
            "kind": kind,
            "format": format,
            "content": content,
        },
    });
    if let Some(workflow) = args.workflow {
        body["workflow_kind"] = Value::String(workflow);
    }
    if let Some(title) = args.title {
        body["title"] = Value::String(title);
    }
    if let Some(subject) = args.subject {
        let Some((subject_type, subject_id)) = subject.split_once(':') else {
            anyhow::bail!("--subject must use type:id form, for example repo:abbotik/api");
        };
        if subject_type.is_empty() || subject_id.is_empty() {
            anyhow::bail!("--subject must use type:id form, for example repo:abbotik/api");
        }
        body["subject_type"] = Value::String(subject_type.to_string());
        body["subject_id"] = Value::String(subject_id.to_string());
    }

    Ok(body)
}
