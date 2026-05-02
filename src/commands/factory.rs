use super::*;

pub(super) async fn run(command: FactoryCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        FactorySubcommand::Submit(args) => submit_run(args, client).await?,
        FactorySubcommand::Start(arg) => print_json(
            &client
                .post_json::<_, Value>(&format!("/llm/factory/runs/{}/start", arg.id), &json!({}))
                .await?,
        )?,
        FactorySubcommand::Status(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/status", arg.id))
                .await?,
        )?,
        FactorySubcommand::Watch(arg) => watch_run(arg, client).await?,
        FactorySubcommand::Review(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/review", arg.id))
                .await?,
        )?,
    }
    Ok(())
}

async fn submit_run(args: FactorySubmitCommand, client: &ApiClient) -> anyhow::Result<()> {
    let create_response = client
        .post_json::<_, Value>("/llm/factory/runs", &create_body(args)?)
        .await?;
    let run_id = extract_run_id(&create_response)
        .ok_or_else(|| anyhow::anyhow!("factory submit response did not include a run id"))?;
    let start_response = client
        .post_json::<_, Value>(&format!("/llm/factory/runs/{run_id}/start"), &json!({}))
        .await?;
    print_json(&json!({
        "run_id": run_id,
        "create": create_response,
        "start": start_response,
    }))
}

async fn watch_run(args: FactoryWatchCommand, client: &ApiClient) -> anyhow::Result<()> {
    if args.interval == 0 {
        anyhow::bail!("factory watch --interval must be greater than zero");
    }

    print_text(&format!(
        "Watching {}. Press Ctrl-C to detach; the factory run will continue.",
        args.id
    ))?;

    let started = std::time::Instant::now();
    let timeout = args.timeout.map(std::time::Duration::from_secs);
    loop {
        if let Some(timeout) = timeout {
            if started.elapsed() >= timeout {
                anyhow::bail!(
                    "factory watch timed out after {} seconds",
                    timeout.as_secs()
                );
            }
        }

        let response = client
            .get_json::<Value>(&format!("/llm/factory/runs/{}/status", args.id))
            .await?;
        let status = response_data(&response);
        print_text(&watch_status_line(status))?;

        let stop = watch_stop(status, args.until);
        if stop.stop {
            if stop.failed {
                anyhow::bail!("factory run {} failed", args.id);
            }
            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs(args.interval)).await;
    }

    Ok(())
}

fn create_body(args: FactorySubmitCommand) -> anyhow::Result<Value> {
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
        _ => anyhow::bail!("factory submit requires exactly one of --prompt or --plan"),
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

fn extract_run_id(response: &Value) -> Option<String> {
    let data = response_data(response);
    data.get("run_id")
        .and_then(Value::as_str)
        .or_else(|| data.get("id").and_then(Value::as_str))
        .map(str::to_string)
}

fn response_data(response: &Value) -> &Value {
    response.get("data").unwrap_or(response)
}

fn watch_status_line(status: &Value) -> String {
    let status_text = string_field(status, "status").unwrap_or("unknown");
    let run_id = string_field(status, "run_id").unwrap_or("unknown");
    let stage = string_field(status, "current_stage").unwrap_or("n/a");
    let blockers = status
        .get("blockers")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let verification = match status
        .get("latest_verification_success")
        .and_then(Value::as_bool)
    {
        Some(true) => "passed",
        Some(false) => "failed",
        None => "n/a",
    };
    format!(
        "{} {run_id} {status_text} stage={stage} stages={} issues={} blockers={blockers} verification={verification}",
        chrono::Local::now().format("%H:%M:%S"),
        counts_summary(status.get("stage_counts")),
        counts_summary(status.get("issue_counts")),
    )
}

fn counts_summary(value: Option<&Value>) -> String {
    let Some(counts) = value.and_then(Value::as_object) else {
        return "none".to_string();
    };
    let order = ["pending", "ready", "running", "passed", "failed", "blocked"];
    let mut parts: Vec<String> = order
        .iter()
        .filter_map(|key| count_value(counts.get(*key)).map(|count| format!("{key}:{count}")))
        .collect();
    for (key, value) in counts {
        if !order.contains(&key.as_str()) {
            if let Some(count) = count_value(Some(value)) {
                parts.push(format!("{key}:{count}"));
            }
        }
    }
    if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(",")
    }
}

fn count_value(value: Option<&Value>) -> Option<u64> {
    match value {
        Some(Value::Number(number)) => number.as_u64(),
        _ => None,
    }
    .filter(|count| *count > 0)
}

struct WatchStop {
    stop: bool,
    failed: bool,
}

fn watch_stop(status: &Value, until: Option<FactoryWatchUntil>) -> WatchStop {
    let run_status = string_field(status, "status").unwrap_or("");
    let completed = run_status == "completed";
    let failed = run_status == "failed";
    let blocked = has_attention_blockers(status);
    let attention = blocked || run_status == "gated" || has_attention_gate(status);

    let requested = match until.unwrap_or(FactoryWatchUntil::Attention) {
        FactoryWatchUntil::Completed => completed,
        FactoryWatchUntil::Failed => failed,
        FactoryWatchUntil::Blocked => blocked,
        FactoryWatchUntil::Attention => completed || failed || attention,
    };

    WatchStop {
        stop: requested || failed,
        failed,
    }
}

fn has_attention_blockers(status: &Value) -> bool {
    status
        .get("blockers")
        .and_then(Value::as_array)
        .is_some_and(|blockers| !blockers.is_empty())
        || status_count(status, "stage_counts", "blocked") > 0
        || status_count(status, "issue_counts", "blocked") > 0
}

fn status_count(status: &Value, field: &str, key: &str) -> u64 {
    status
        .get(field)
        .and_then(Value::as_object)
        .and_then(|counts| counts.get(key))
        .and_then(Value::as_u64)
        .unwrap_or(0)
}

fn has_attention_gate(status: &Value) -> bool {
    status
        .get("latest_gate_verdicts")
        .and_then(Value::as_object)
        .is_some_and(|gates| {
            gates.values().any(|value| {
                matches!(
                    value.as_str(),
                    Some("fail" | "failed" | "needs_further_review" | "needs_review")
                )
            })
        })
}

fn string_field<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
    value.get(field).and_then(Value::as_str)
}
