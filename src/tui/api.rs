#![allow(dead_code)]

use anyhow::{anyhow, Context};
use serde::Deserialize;
use serde_json::Value;
use tokio::try_join;

use crate::api::{ApiClient, ApiEnvelope};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RoomActor {
    #[serde(default)]
    pub adapter: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoomRecord {
    pub id: String,
    pub purpose: String,
    pub status: String,
    pub rented_at: String,
    #[serde(default)]
    pub last_active_at: Option<String>,
    #[serde(default)]
    pub actors: Vec<RoomActor>,
    #[serde(default)]
    pub summary_text: Option<String>,
    #[serde(default)]
    pub last_error: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoomMessage {
    pub id: String,
    #[serde(default)]
    pub actor_key: Option<String>,
    pub author_kind: String,
    pub kind: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoomEvent {
    pub id: String,
    pub event_type: String,
    pub created_at: String,
    #[serde(default)]
    pub payload: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoomHistory {
    pub room: RoomRecord,
    pub messages: Vec<RoomMessage>,
    pub events: Vec<RoomEvent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactoryRun {
    pub id: String,
    pub status: String,
    pub updated_at: String,
    #[serde(default)]
    pub source_brief: Option<String>,
    #[serde(default)]
    pub current_stage: Option<String>,
    #[serde(default)]
    pub active_room_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactoryStatus {
    pub run_id: String,
    pub status: String,
    #[serde(default)]
    pub current_stage: Option<String>,
    #[serde(default)]
    pub stage_counts: std::collections::BTreeMap<String, u64>,
    #[serde(default)]
    pub issue_counts: std::collections::BTreeMap<String, u64>,
    #[serde(default)]
    pub latest_checkpoint_status: Option<String>,
    #[serde(default)]
    pub latest_checkpoint_id: Option<String>,
    #[serde(default)]
    pub latest_verification_success: Option<bool>,
    #[serde(default)]
    pub latest_verification_artifact_id: Option<String>,
    #[serde(default)]
    pub latest_gate_verdicts: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
    #[serde(default)]
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactoryStage {
    pub id: String,
    pub status: String,
    pub stage_name: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub assigned_room_id: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub verification_summary: Option<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactoryIssue {
    pub id: String,
    pub status: String,
    pub title: String,
    pub kind: String,
    #[serde(default)]
    pub stage_id: Option<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,
    #[serde(default)]
    pub verification_requirements: Vec<String>,
    #[serde(default)]
    pub assigned_room_id: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactoryCheckpoint {
    pub id: String,
    pub status: String,
    pub branch_name: String,
    pub worktree_path: String,
    pub base_ref: String,
    #[serde(default)]
    pub head_sha: Option<String>,
    #[serde(default)]
    pub included_issue_ids: Vec<String>,
    #[serde(default)]
    pub merged_issue_ids: Vec<String>,
    #[serde(default)]
    pub conflict_issue_ids: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactoryArtifact {
    pub id: String,
    pub artifact_type: String,
    pub version: u64,
    #[serde(default)]
    pub producer_stage_id: Option<String>,
    pub payload: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FactoryReview {
    pub id: String,
    pub plan_summary: String,
    #[serde(default)]
    pub artifact_summary: Vec<String>,
    #[serde(default)]
    pub checkpoint_summary: Vec<String>,
    #[serde(default)]
    pub issue_summary: Vec<String>,
    #[serde(default)]
    pub stage_summary: Vec<String>,
    #[serde(default)]
    pub gate_summary: Vec<String>,
    #[serde(default)]
    pub evidence_summary: Vec<String>,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
    #[serde(default)]
    pub next_actions: Vec<String>,
    #[serde(default)]
    pub open_risks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FactoryDetail {
    pub run: FactoryRun,
    pub status: FactoryStatus,
    pub stages: Vec<FactoryStage>,
    pub issues: Vec<FactoryIssue>,
    pub checkpoints: Vec<FactoryCheckpoint>,
    pub artifacts: Vec<FactoryArtifact>,
    pub review: FactoryReview,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseCreateResponse {
    pub room_id: String,
    #[serde(default)]
    pub room_created: bool,
    #[serde(default)]
    pub request_message_id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TuiApi {
    client: ApiClient,
}

#[derive(Debug, Clone)]
pub struct SendMessageRequest {
    pub room_id: Option<String>,
    pub input: String,
    pub purpose: Option<String>,
    pub provider: String,
    pub model: String,
    pub adapter: String,
}

impl TuiApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub fn token_present(&self) -> bool {
        self.client.token().is_some()
    }

    pub async fn list_rooms(&self) -> anyhow::Result<Vec<RoomRecord>> {
        self.unwrap_envelope(self.client.get_json("/llm/room").await)
            .await
            .context("failed to list rooms")
    }

    pub async fn load_room(&self, room_id: &str) -> anyhow::Result<RoomHistory> {
        self.unwrap_envelope(
            self.client
                .get_json(&format!("/llm/room/{room_id}/history"))
                .await,
        )
        .await
        .with_context(|| format!("failed to load room history for {room_id}"))
    }

    pub async fn list_factory_runs(&self) -> anyhow::Result<Vec<FactoryRun>> {
        self.unwrap_envelope(self.client.get_json("/llm/factory/runs").await)
            .await
            .context("failed to list factory runs")
    }

    pub async fn load_factory(&self, run_id: &str) -> anyhow::Result<FactoryDetail> {
        let run = self
            .unwrap_envelope(
                self.client
                    .get_json(&format!("/llm/factory/runs/{run_id}"))
                    .await,
            )
            .await
            .with_context(|| format!("failed to load factory run {run_id}"))?;

        let (status, stages, issues, checkpoints, artifacts, review) = try_join!(
            self.unwrap_envelope(
                self.client
                    .get_json(&format!("/llm/factory/runs/{run_id}/status"))
                    .await
            ),
            self.unwrap_envelope(
                self.client
                    .get_json(&format!("/llm/factory/runs/{run_id}/stages"))
                    .await
            ),
            self.unwrap_envelope(
                self.client
                    .get_json(&format!("/llm/factory/runs/{run_id}/issues"))
                    .await
            ),
            self.unwrap_envelope(
                self.client
                    .get_json(&format!("/llm/factory/runs/{run_id}/checkpoints"))
                    .await
            ),
            self.unwrap_envelope(
                self.client
                    .get_json(&format!("/llm/factory/runs/{run_id}/artifacts"))
                    .await
            ),
            self.unwrap_envelope(
                self.client
                    .get_json(&format!("/llm/factory/runs/{run_id}/review"))
                    .await
            ),
        )?;

        Ok(FactoryDetail {
            run,
            status,
            stages,
            issues,
            checkpoints,
            artifacts,
            review,
        })
    }

    pub async fn send_message(
        &self,
        request: SendMessageRequest,
    ) -> anyhow::Result<ResponseCreateResponse> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            input: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            room_id: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            purpose: Option<&'a str>,
            provider: &'a str,
            model: &'a str,
            adapter: &'a str,
            metadata: serde_json::Value,
        }

        self.client
            .post_json(
                "/v1/responses",
                &Body {
                    input: &request.input,
                    room_id: request.room_id.as_deref(),
                    purpose: request.purpose.as_deref(),
                    provider: &request.provider,
                    model: &request.model,
                    adapter: &request.adapter,
                    metadata: serde_json::json!({ "source": "abbot-tui" }),
                },
            )
            .await
            .context("failed to send room-backed response")
    }

    async fn unwrap_envelope<T>(
        &self,
        result: Result<ApiEnvelope<T>, crate::error::AbbotikError>,
    ) -> anyhow::Result<T> {
        let envelope = result.map_err(anyhow::Error::from)?;
        if !envelope.success {
            return Err(anyhow!(
                "{}",
                envelope
                    .message
                    .or(envelope.error)
                    .unwrap_or_else(|| "request failed".to_string())
            ));
        }
        envelope
            .data
            .ok_or_else(|| anyhow!("successful response was missing data"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_actor_defaults_are_optional() {
        let actor: RoomActor = serde_json::from_value(serde_json::json!({})).expect("actor parses");
        assert_eq!(actor.model, None);
        assert_eq!(actor.provider, None);
        assert_eq!(actor.adapter, None);
    }
}
