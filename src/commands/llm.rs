use super::*;

pub(super) async fn run(command: LlmCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        LlmSubcommand::Providers(_) => {
            print_json(&client.get_json::<Value>("/llm/providers").await?)?
        }
        LlmSubcommand::Models(_) => {
            print_json(&client.get_json::<Value>("/llm/providers/models").await?)?
        }
        LlmSubcommand::Skills(_) => print_json(&client.get_json::<Value>("/llm/skills").await?)?,
        LlmSubcommand::Room(command) => llm_room(command, client).await?,
        LlmSubcommand::Factory(command) => llm_factory(command, client).await?,
    }
    Ok(())
}

async fn llm_room(command: LlmRoomCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        LlmRoomSubcommand::List => print_json(&client.get_json::<Value>("/llm/room").await?)?,
        LlmRoomSubcommand::Create(args) => create_room(args, client).await?,
        LlmRoomSubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/room/{}", arg.id))
                .await?,
        )?,
        LlmRoomSubcommand::Update(arg) => print_json(
            &client
                .patch_json::<_, Value>(
                    &format!("/llm/room/{}", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmRoomSubcommand::Message(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/room/{}/messages", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmRoomSubcommand::Wake(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/room/{}/wake", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmRoomSubcommand::Run(args) => run_room_prompt(args, client).await?,
        LlmRoomSubcommand::Events(args) => {
            let query = vec![("follow".to_string(), args.follow.to_string())];
            print_text(
                &client
                    .request_text_with_query::<(), _>(
                        Method::GET,
                        &format!("/llm/room/{}/events", args.id),
                        Some(&query),
                        None,
                    )
                    .await?,
            )?
        }
        LlmRoomSubcommand::History(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/room/{}/history", arg.id))
                .await?,
        )?,
        LlmRoomSubcommand::Interrupt(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/room/{}/interrupt", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmRoomSubcommand::Release(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/room/{}/release", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
    }
    Ok(())
}

async fn create_room(args: LlmRoomCreateCommand, client: &ApiClient) -> anyhow::Result<()> {
    if let Some(name) = args.name.as_deref() {
        ensure_room_name_available(client, name).await?;
    }

    let body = build_room_create_body(args)?;
    print_json(&client.post_json::<_, Value>("/llm/room", &body).await?)?;
    Ok(())
}

fn build_room_create_body(args: LlmRoomCreateCommand) -> anyhow::Result<Value> {
    let mut body = read_json_body_or_default(json!({}))?;
    let has_flags = args.name.is_some()
        || args.provider.is_some()
        || args.model.is_some()
        || args.purpose.is_some()
        || args.agent_id.is_some();

    if !has_flags {
        return Ok(body);
    }

    let object = body
        .as_object_mut()
        .ok_or_else(|| anyhow::anyhow!("room create body must be a JSON object"))?;

    if let Some(purpose) = args.purpose {
        object.insert("purpose".to_string(), Value::String(purpose));
    }

    if let Some(name) = args.name.clone() {
        let metadata = object
            .entry("metadata")
            .or_insert_with(|| json!({}))
            .as_object_mut()
            .ok_or_else(|| anyhow::anyhow!("room create metadata must be a JSON object"))?;
        metadata.insert("name".to_string(), Value::String(name));
    }

    if args.provider.is_some() || args.model.is_some() || args.agent_id.is_some() {
        let provider = args.provider.ok_or_else(|| {
            anyhow::anyhow!("room create requires --provider when setting agent flags")
        })?;
        let model = args.model.ok_or_else(|| {
            anyhow::anyhow!("room create requires --model when setting agent flags")
        })?;
        let agent_id = args
            .agent_id
            .or(args.name.map(|name| format!("{name}-agent")))
            .unwrap_or_else(|| "assistant-1".to_string());

        object.insert(
            "agents".to_string(),
            json!([
                {
                    "agent_id": agent_id,
                    "role": args.role,
                    "adapter": args.adapter,
                    "provider": provider,
                    "model": model,
                    "skills": []
                }
            ]),
        );
    }

    Ok(body)
}

async fn ensure_room_name_available(client: &ApiClient, name: &str) -> anyhow::Result<()> {
    if find_room_by_name(client, name).await?.is_some() {
        anyhow::bail!(
            "room named `{name}` already exists; use `abbot llm room run --name {name} ...`"
        );
    }
    Ok(())
}

async fn run_room_prompt(args: LlmRoomRunCommand, client: &ApiClient) -> anyhow::Result<()> {
    let room_id = resolve_room_id(client, args.name.as_deref(), args.id.as_deref()).await?;
    let response = client
        .post_json::<_, Value>(
            &format!("/llm/room/{room_id}/messages"),
            &json!({
                "kind": "task",
                "content": args.prompt,
                "metadata": { "source": "cli" }
            }),
        )
        .await?;
    let message_id = response
        .pointer("/data/id")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("room message response did not include data.id"))?
        .to_string();

    let answer = wait_for_room_output(
        client,
        &room_id,
        &message_id,
        args.timeout_seconds,
        args.poll_seconds,
        args.stream,
    )
    .await?;
    println!("{answer}");
    Ok(())
}

async fn resolve_room_id(
    client: &ApiClient,
    name: Option<&str>,
    id: Option<&str>,
) -> anyhow::Result<String> {
    match (name, id) {
        (Some(_), Some(_)) => anyhow::bail!("use either --name or --id, not both"),
        (Some(name), None) => find_room_by_name(client, name)
            .await?
            .ok_or_else(|| anyhow::anyhow!("no active room named `{name}`")),
        (None, Some(id)) => Ok(id.to_string()),
        (None, None) => anyhow::bail!("room run requires --name <name> or --id <room-id>"),
    }
}

async fn find_room_by_name(client: &ApiClient, name: &str) -> anyhow::Result<Option<String>> {
    let response = client.get_json::<Value>("/llm/room").await?;
    let rooms = response
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow::anyhow!("room list response did not include data array"))?;

    Ok(rooms
        .iter()
        .find(|room| {
            room.pointer("/metadata/name").and_then(Value::as_str) == Some(name)
                && !matches!(
                    room.get("status").and_then(Value::as_str),
                    Some("released" | "releasing" | "failed")
                )
        })
        .and_then(|room| room.get("id").and_then(Value::as_str))
        .map(ToOwned::to_owned))
}

async fn wait_for_room_output(
    client: &ApiClient,
    room_id: &str,
    trigger_message_id: &str,
    timeout_seconds: u64,
    poll_seconds: u64,
    stream: bool,
) -> anyhow::Result<String> {
    let timeout = std::time::Duration::from_secs(timeout_seconds);
    let poll = std::time::Duration::from_secs(poll_seconds.max(1));
    let started = std::time::Instant::now();
    let mut seen_event_ids: Vec<String> = Vec::new();

    loop {
        let history = client
            .get_json::<Value>(&format!("/llm/room/{room_id}/history"))
            .await?;

        if stream {
            print_new_room_events(&history, &mut seen_event_ids);
        }

        if room_failed(&history) {
            anyhow::bail!(
                "room failed: {}",
                history
                    .pointer("/data/room/last_error/message")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown error")
            );
        }

        if let Some(answer) = find_output_after_trigger(&history, trigger_message_id) {
            return Ok(answer);
        }

        if started.elapsed() >= timeout {
            anyhow::bail!("timed out waiting for room `{room_id}` after {timeout_seconds}s");
        }

        tokio::time::sleep(poll).await;
    }
}

fn print_new_room_events(history: &Value, seen_event_ids: &mut Vec<String>) {
    let Some(events) = history.pointer("/data/events").and_then(Value::as_array) else {
        return;
    };

    for event in events {
        let Some(id) = event.get("id").and_then(Value::as_str) else {
            continue;
        };
        if seen_event_ids.iter().any(|seen| seen == id) {
            continue;
        }
        seen_event_ids.push(id.to_string());

        if let Some(event_type) = event.get("event_type").and_then(Value::as_str) {
            eprintln!("{event_type}");
        }
    }
}

fn room_failed(history: &Value) -> bool {
    history.pointer("/data/room/status").and_then(Value::as_str) == Some("failed")
}

fn find_output_after_trigger(history: &Value, trigger_message_id: &str) -> Option<String> {
    let messages = history.pointer("/data/messages")?.as_array()?;
    let trigger_seq = messages
        .iter()
        .find(|message| message.get("id").and_then(Value::as_str) == Some(trigger_message_id))
        .and_then(|message| message.get("seq").and_then(Value::as_i64));

    messages
        .iter()
        .filter(|message| message.get("author_kind").and_then(Value::as_str) == Some("agent"))
        .filter(|message| message.get("kind").and_then(Value::as_str) == Some("output"))
        .filter(|message| match trigger_seq {
            Some(seq) => message
                .get("seq")
                .and_then(Value::as_i64)
                .is_some_and(|message_seq| message_seq > seq),
            None => true,
        })
        .filter_map(extract_message_text)
        .last()
}

fn extract_message_text(message: &Value) -> Option<String> {
    let content = message.get("content")?;
    if let Some(text) = content.as_str() {
        return Some(text.to_string());
    }

    let blocks = content.as_array()?;
    let text = blocks
        .iter()
        .filter(|block| block.get("type").and_then(Value::as_str) == Some("text"))
        .filter_map(|block| block.get("text").and_then(Value::as_str))
        .collect::<Vec<_>>()
        .join("\n");

    (!text.is_empty()).then_some(text)
}

async fn llm_factory(command: LlmFactoryCommand, client: &ApiClient) -> anyhow::Result<()> {
    match command.command {
        LlmFactorySubcommand::List => {
            print_json(&client.get_json::<Value>("/llm/factory/runs").await?)?
        }
        LlmFactorySubcommand::Create => print_json(
            &client
                .post_json::<_, Value>("/llm/factory/runs", &read_json_body_or_default(json!({}))?)
                .await?,
        )?,
        LlmFactorySubcommand::Get(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}", arg.id))
                .await?,
        )?,
        LlmFactorySubcommand::Start(arg) => print_json(
            &client
                .post_json::<_, Value>(&format!("/llm/factory/runs/{}/start", arg.id), &json!({}))
                .await?,
        )?,
        LlmFactorySubcommand::Status(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/status", arg.id))
                .await?,
        )?,
        LlmFactorySubcommand::Checkpoints(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/checkpoints", arg.id))
                .await?,
        )?,
        LlmFactorySubcommand::CreateCheckpoint(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/checkpoints", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::Stages(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/stages", arg.id))
                .await?,
        )?,
        LlmFactorySubcommand::CreateStage(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/stages", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::UpdateStage(arg) => print_json(
            &client
                .patch_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/stages/{}", arg.id, arg.stage_id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::Issues(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/issues", arg.id))
                .await?,
        )?,
        LlmFactorySubcommand::CreateIssue(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/issues", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::UpdateIssue(arg) => print_json(
            &client
                .patch_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/issues/{}", arg.id, arg.issue_id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::DispatchIssue(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!(
                        "/llm/factory/runs/{}/issues/{}/dispatch",
                        arg.id, arg.issue_id
                    ),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::Artifacts(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/artifacts", arg.id))
                .await?,
        )?,
        LlmFactorySubcommand::CreateArtifact(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/artifacts", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::Advance(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/advance", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::Verify(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/verify", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::GateCheck(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/gate-check", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::CreateGate(arg) => print_json(
            &client
                .post_json::<_, Value>(
                    &format!("/llm/factory/runs/{}/gates", arg.id),
                    &read_json_body_or_default(json!({}))?,
                )
                .await?,
        )?,
        LlmFactorySubcommand::Review(arg) => print_json(
            &client
                .get_json::<Value>(&format!("/llm/factory/runs/{}/review", arg.id))
                .await?,
        )?,
    }
    Ok(())
}
