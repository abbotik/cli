use super::*;

pub(super) async fn run(command: FactoryCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        FactorySubcommand::List(args) => {
            let response = client.get_json::<Value>("/llm/factory/runs").await?;
            if args.json {
                print_json(&response)?;
            } else {
                print_text(&render_runs_table(response_data(&response)))?;
            }
        }
        FactorySubcommand::Submit(args) => submit_run(args, client).await?,
        FactorySubcommand::Run(args) => run_factory(args, client).await?,
        FactorySubcommand::Start(arg) => {
            let response = client
                .post_json::<_, Value>(&format!("/llm/factory/runs/{}/start", arg.id), &json!({}))
                .await?;
            if arg.json {
                print_json(&response)?;
            } else {
                print_text(&render_start_response(response_data(&response)))?;
            }
        }
        FactorySubcommand::Status(arg) => {
            let response = client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/status", arg.id))
                .await?;
            if arg.json {
                print_json(&response)?;
            } else {
                print_text(&render_status(response_data(&response)))?;
            }
        }
        FactorySubcommand::Cancel(arg) => {
            let mut body = json!({});
            if let Some(reason) = arg.reason.as_deref() {
                body["reason"] = Value::String(reason.to_string());
            }
            let response = client
                .post_json::<_, Value>(&format!("/llm/factory/runs/{}/cancel", arg.id), &body)
                .await?;
            if arg.json {
                print_json(&response)?;
            } else {
                print_text(&render_cancel_response(response_data(&response)))?;
            }
        }
        FactorySubcommand::Watch(arg) => watch_run(arg, client).await?,
        FactorySubcommand::Review(arg) => {
            let response = client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/review", arg.id))
                .await?;
            if arg.json {
                print_json(&response)?;
            } else {
                print_text(&render_review(response_data(&response)))?;
            }
        }
    }
    Ok(())
}

fn render_runs_table(value: &Value) -> String {
    let rows: Vec<Vec<String>> = value
        .as_array()
        .into_iter()
        .flatten()
        .map(|run| {
            vec![
                short_id(
                    string_field(run, "id")
                        .unwrap_or_else(|| string_field(run, "run_id").unwrap_or("unknown")),
                ),
                string_field(run, "status").unwrap_or("unknown").to_string(),
                string_field(run, "current_stage")
                    .unwrap_or("n/a")
                    .to_string(),
                string_field(run, "workflow_kind")
                    .unwrap_or("n/a")
                    .to_string(),
                string_field(run, "source_brief")
                    .or_else(|| string_field(run, "title"))
                    .map(|value| truncate(value, 48))
                    .unwrap_or_else(|| "n/a".to_string()),
            ]
        })
        .collect();
    if rows.is_empty() {
        "No factory runs.".to_string()
    } else {
        render_table(&["ID", "STATUS", "STAGE", "WORKFLOW", "BRIEF"], rows)
    }
}

fn render_submit_response(value: &Value) -> String {
    let run_id = string_field(value, "run_id").unwrap_or("unknown");
    let create = response_data(value.get("create").unwrap_or(&Value::Null));
    let start = response_data(value.get("start").unwrap_or(&Value::Null));
    let rows = vec![
        vec![
            short_id(run_id),
            string_field(create, "status")
                .unwrap_or("unknown")
                .to_string(),
            string_field(create, "current_stage")
                .unwrap_or("n/a")
                .to_string(),
            "create".to_string(),
        ],
        vec![
            short_id(run_id),
            string_field(start, "status")
                .unwrap_or("unknown")
                .to_string(),
            string_field(start, "current_stage")
                .unwrap_or("n/a")
                .to_string(),
            "start".to_string(),
        ],
    ];
    render_table(&["ID", "STATUS", "STAGE", "STEP"], rows)
}

fn render_start_response(value: &Value) -> String {
    let rows = vec![vec![
        short_id(string_field(value, "run_id").unwrap_or("unknown")),
        string_field(value, "status")
            .unwrap_or("unknown")
            .to_string(),
        bool_field(value, "completed").to_string(),
        bool_field(value, "failed").to_string(),
        value
            .get("actions")
            .and_then(Value::as_array)
            .map_or(0, Vec::len)
            .to_string(),
    ]];
    render_table(&["ID", "STATUS", "DONE", "FAILED", "ACTIONS"], rows)
}

fn render_cancel_response(value: &Value) -> String {
    let rows = vec![vec![
        short_id(
            string_field(value, "run_id")
                .unwrap_or_else(|| string_field(value, "id").unwrap_or("unknown")),
        ),
        string_field(value, "status")
            .unwrap_or("cancelled")
            .to_string(),
        string_field(value, "reason")
            .map(|value| truncate(value, 48))
            .unwrap_or_else(|| "n/a".to_string()),
    ]];
    render_table(&["ID", "STATUS", "REASON"], rows)
}

fn render_status(status: &Value) -> String {
    let rows = vec![vec![
        short_id(string_field(status, "run_id").unwrap_or("unknown")),
        string_field(status, "status")
            .unwrap_or("unknown")
            .to_string(),
        string_field(status, "current_stage")
            .unwrap_or("n/a")
            .to_string(),
        counts_summary(status.get("stage_counts")),
        counts_summary(status.get("issue_counts")),
        status
            .get("blockers")
            .and_then(Value::as_array)
            .map_or(0, Vec::len)
            .to_string(),
        verification_label(status).to_string(),
    ]];
    let mut output = render_table(
        &[
            "ID",
            "STATUS",
            "STAGE",
            "STAGES",
            "ISSUES",
            "BLOCKERS",
            "VERIFICATION",
        ],
        rows,
    );
    append_list_section(&mut output, "Blockers", status.get("blockers"));
    append_list_section(&mut output, "Next actions", status.get("next_actions"));
    output
}

fn render_review(review: &Value) -> String {
    let rows = vec![vec![
        short_id(string_field(review, "run_id").unwrap_or("unknown")),
        truncate(string_field(review, "plan_summary").unwrap_or("n/a"), 72),
        review
            .get("blockers")
            .and_then(Value::as_array)
            .map_or(0, Vec::len)
            .to_string(),
        review
            .get("next_actions")
            .and_then(Value::as_array)
            .map_or(0, Vec::len)
            .to_string(),
    ]];
    let mut output = render_table(&["ID", "SUMMARY", "BLOCKERS", "NEXT"], rows);
    append_list_section(&mut output, "Stages", review.get("stage_summary"));
    append_list_section(&mut output, "Issues", review.get("issue_summary"));
    append_list_section(&mut output, "Gates", review.get("gate_summary"));
    append_list_section(&mut output, "Evidence", review.get("evidence_summary"));
    append_list_section(&mut output, "Blockers", review.get("blockers"));
    append_list_section(&mut output, "Next actions", review.get("next_actions"));
    append_list_section(&mut output, "Open risks", review.get("open_risks"));
    output
}

fn render_table(headers: &[&str], rows: Vec<Vec<String>>) -> String {
    let mut widths: Vec<usize> = headers.iter().map(|header| header.len()).collect();
    for row in &rows {
        for (index, cell) in row.iter().enumerate() {
            if let Some(width) = widths.get_mut(index) {
                *width = (*width).max(cell.len());
            }
        }
    }

    let mut output = String::new();
    output.push_str(&render_table_row(
        &headers
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>(),
        &widths,
    ));
    output.push('\n');
    output.push_str(
        &widths
            .iter()
            .map(|width| "-".repeat(*width))
            .collect::<Vec<_>>()
            .join("  "),
    );
    for row in rows {
        output.push('\n');
        output.push_str(&render_table_row(&row, &widths));
    }
    output
}

fn render_table_row(row: &[String], widths: &[usize]) -> String {
    row.iter()
        .enumerate()
        .map(|(index, cell)| format!("{cell:<width$}", width = widths[index]))
        .collect::<Vec<_>>()
        .join("  ")
}

fn append_list_section(output: &mut String, title: &str, value: Option<&Value>) {
    let items: Vec<String> = value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.as_str().map(str::to_string))
        .collect();
    if items.is_empty() {
        return;
    }
    output.push_str("\n\n");
    output.push_str(title);
    output.push_str(":\n");
    for item in items {
        output.push_str("  - ");
        output.push_str(&item);
        output.push('\n');
    }
    if output.ends_with('\n') {
        output.pop();
    }
}

async fn submit_run(args: FactorySubmitCommand, client: &ApiClient) -> anyhow::Result<()> {
    let json_output = args.json;
    let SubmittedFactoryRun {
        run_id,
        create_response,
        start_response,
    } = submit_factory_run(args, client).await?;
    let response = json!({
        "run_id": run_id,
        "create": create_response,
        "start": start_response,
    });
    if json_output {
        print_json(&response)
    } else {
        print_text(&render_submit_response(&response))
    }
}

async fn run_factory(args: FactoryRunCommand, client: &ApiClient) -> anyhow::Result<()> {
    let json_output = args.submit.json;
    let SubmittedFactoryRun {
        run_id,
        create_response,
        start_response,
    } = submit_factory_run(args.submit, client).await?;
    if json_output {
        print_json(&json!({
            "run_id": run_id,
            "create": create_response,
            "start": start_response,
        }))?;
    } else {
        print_text(&format!("Submitted factory run {run_id}."))?;
    }
    watch_factory_run(&run_id, args.wait, json_output, client).await
}

struct SubmittedFactoryRun {
    run_id: String,
    create_response: Value,
    start_response: Value,
}

async fn submit_factory_run(
    args: FactorySubmitCommand,
    client: &ApiClient,
) -> anyhow::Result<SubmittedFactoryRun> {
    let create_response = client
        .post_json::<_, Value>("/llm/factory/runs", &create_body(args)?)
        .await?;
    let run_id = extract_run_id(&create_response)
        .ok_or_else(|| anyhow::anyhow!("factory submit response did not include a run id"))?;
    let start_response = client
        .post_json::<_, Value>(&format!("/llm/factory/runs/{run_id}/start"), &json!({}))
        .await?;

    Ok(SubmittedFactoryRun {
        run_id,
        create_response,
        start_response,
    })
}

async fn watch_run(args: FactoryWatchCommand, client: &ApiClient) -> anyhow::Result<()> {
    watch_factory_run(&args.id, args.wait, args.json, client).await
}

async fn watch_factory_run(
    run_id: &str,
    wait: FactoryWaitOptions,
    json_output: bool,
    client: &ApiClient,
) -> anyhow::Result<()> {
    if wait.interval == 0 {
        anyhow::bail!("factory watch --interval must be greater than zero");
    }

    if !json_output {
        print_text(&format!(
            "Watching {}. Press Ctrl-C to detach; the factory run will continue.",
            run_id
        ))?;
    }

    let started = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(wait.timeout);
    loop {
        if started.elapsed() >= timeout {
            anyhow::bail!(
                "factory watch timed out after {} seconds",
                timeout.as_secs()
            );
        }

        let response = client
            .get_json::<Value>(&format!("/llm/factory/runs/{run_id}/status"))
            .await?;
        let status = response_data(&response);
        if json_output {
            print_json(&response)?;
        } else {
            print_text(&watch_status_line(status))?;
        }

        let stop = watch_stop(status, wait.until);
        if stop.stop {
            if stop.failed {
                anyhow::bail!("factory run {run_id} failed");
            }
            break;
        }

        tokio::time::sleep(std::time::Duration::from_secs(wait.interval)).await;
    }

    Ok(())
}

fn create_body(args: FactorySubmitCommand) -> anyhow::Result<Value> {
    let (format, content) = match (args.prompt_text, args.prompt, args.prompt_file) {
        (Some(prompt), None, None) | (None, Some(prompt), None) => ("text", prompt),
        (None, None, Some(path)) => {
            let content = read_prompt_file(&path)?;
            let format = if path != Path::new("-")
                && path.extension().and_then(|value| value.to_str()) == Some("md")
            {
                "markdown"
            } else {
                "text"
            };
            (format, content)
        }
        _ => anyhow::bail!(
            "factory submit requires exactly one of PROMPT, --prompt, or --prompt-file"
        ),
    };
    if content.trim().is_empty() {
        anyhow::bail!("factory submit prompt must not be empty");
    }

    let mut body = json!({
        "source": {
            "kind": "prompt",
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

fn read_prompt_file(path: &Path) -> anyhow::Result<String> {
    if path == Path::new("-") {
        let content = read_stdin_or_empty()?;
        if content.trim().is_empty() {
            anyhow::bail!("--prompt-file - did not receive prompt text on stdin");
        }
        return Ok(content);
    }

    stdfs::read_to_string(path)
        .map_err(|error| anyhow::anyhow!("failed to read prompt file {}: {error}", path.display()))
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
    let verification = verification_label(status);
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
    let cancelled = run_status == "cancelled";
    let blocked = has_attention_blockers(status);
    let attention = blocked || run_status == "gated" || has_attention_gate(status);

    let requested = match until.unwrap_or(FactoryWatchUntil::Attention) {
        FactoryWatchUntil::Completed => completed,
        FactoryWatchUntil::Failed => failed,
        FactoryWatchUntil::Cancelled => cancelled,
        FactoryWatchUntil::Blocked => blocked,
        FactoryWatchUntil::Attention => completed || failed || cancelled || attention,
    };

    WatchStop {
        stop: requested || failed || cancelled,
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

fn bool_field(value: &Value, field: &str) -> bool {
    value.get(field).and_then(Value::as_bool).unwrap_or(false)
}

fn verification_label(status: &Value) -> &str {
    string_field(status, "latest_verification").unwrap_or_else(|| {
        match status
            .get("latest_verification_success")
            .and_then(Value::as_bool)
        {
            Some(true) => "passed",
            Some(false) => "failed",
            None => "n/a",
        }
    })
}

fn short_id(value: &str) -> String {
    if value.len() > 8 {
        value.chars().take(8).collect()
    } else {
        value.to_string()
    }
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let keep = max_chars.saturating_sub(3);
    format!("{}...", value.chars().take(keep).collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn submit_args() -> FactorySubmitCommand {
        FactorySubmitCommand {
            prompt_text: None,
            prompt: None,
            prompt_file: None,
            workflow: None,
            subject: None,
            title: None,
            json: false,
        }
    }

    #[test]
    fn create_body_accepts_positional_prompt() {
        let mut args = submit_args();
        args.prompt_text = Some("ship it".to_string());

        let body = create_body(args).expect("body");

        assert_eq!(body.pointer("/source/kind"), Some(&json!("prompt")));
        assert_eq!(body.pointer("/source/format"), Some(&json!("text")));
        assert_eq!(body.pointer("/source/content"), Some(&json!("ship it")));
    }

    #[test]
    fn create_body_reads_markdown_prompt_file_as_prompt() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "abbot-factory-prompt-{}-{unique}.md",
            std::process::id()
        ));
        stdfs::write(&path, "# Plan\n\nShip it.\n").expect("write prompt file");

        let mut args = submit_args();
        args.prompt_file = Some(path.clone());
        let body = create_body(args).expect("body");
        let _ = stdfs::remove_file(&path);

        assert_eq!(body.pointer("/source/kind"), Some(&json!("prompt")));
        assert_eq!(body.pointer("/source/format"), Some(&json!("markdown")));
        assert_eq!(
            body.pointer("/source/content"),
            Some(&json!("# Plan\n\nShip it.\n"))
        );
    }

    #[test]
    fn create_body_rejects_missing_prompt_input() {
        let error = create_body(submit_args()).expect_err("missing prompt should fail");

        assert!(error
            .to_string()
            .contains("exactly one of PROMPT, --prompt, or --prompt-file"));
    }

    #[test]
    fn watch_status_line_prefers_latest_verification_status() {
        let line = watch_status_line(&json!({
            "run_id": "run-1",
            "status": "gated",
            "current_stage": "gated",
            "stage_counts": { "passed": 1 },
            "issue_counts": { "passed": 1 },
            "latest_verification": "skipped",
            "latest_verification_success": true,
            "blockers": [],
        }));

        assert!(line.contains("verification=skipped"));
    }
}
