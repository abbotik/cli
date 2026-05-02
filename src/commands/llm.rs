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
        LlmRoomSubcommand::Create => print_json(
            &client
                .post_json::<_, Value>("/llm/room", &read_json_body_or_default(json!({}))?)
                .await?,
        )?,
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
