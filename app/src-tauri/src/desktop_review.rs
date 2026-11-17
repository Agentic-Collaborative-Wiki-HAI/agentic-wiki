use crate::{
    agent::ProposalCandidate,
    domain::PassageAnchor,
    review::{CandidateSubmission, Discussion, Evidence, EvidenceSelection},
    store::{Page, Store},
    subscription::{AvailableModel, ConnectionStatus, Subscription},
};
use std::{collections::HashMap, sync::Arc};
use tauri::State;
use tokio::sync::{Mutex, oneshot};

pub struct AgentRuntime {
    pub connection: Arc<Subscription>,
    pub requests: Mutex<HashMap<String, oneshot::Sender<()>>>,
}
#[tauri::command]
pub async fn list_discussions(
    store: State<'_, Store>,
    id: String,
) -> Result<Vec<Discussion>, String> {
    store.discussions(&id).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn start_discussion(
    store: State<'_, Store>,
    id: String,
    base_revision: i32,
    anchor: PassageAnchor,
    body: String,
) -> Result<Discussion, String> {
    store
        .create_discussion(&id, base_revision, &anchor, &body)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn reply_discussion(
    store: State<'_, Store>,
    id: String,
    body: String,
) -> Result<Discussion, String> {
    store.reply(&id, &body).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn resolve_discussion(
    store: State<'_, Store>,
    id: String,
    resolved: bool,
) -> Result<Discussion, String> {
    store
        .resolve_discussion(&id, resolved)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn decide_proposal(
    store: State<'_, Store>,
    id: String,
    accept: bool,
) -> Result<Discussion, String> {
    store
        .decide_proposal(&id, accept)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn revise_proposal(
    store: State<'_, Store>,
    id: String,
    wording: String,
    reason: String,
) -> Result<Discussion, String> {
    store
        .revise_proposal(&id, &wording, &reason)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn discussion_evidence(
    store: State<'_, Store>,
    id: String,
) -> Result<Vec<Evidence>, String> {
    store
        .discussion_evidence(&id)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn page_citations(
    store: State<'_, Store>,
    id: String,
    revision: i32,
) -> Result<Vec<Evidence>, String> {
    store
        .page_citations(&id, revision)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn read_revision(
    store: State<'_, Store>,
    id: String,
    revision: i32,
) -> Result<Page, String> {
    store
        .revision(&id, revision)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn subscription_status(
    runtime: State<'_, AgentRuntime>,
) -> Result<ConnectionStatus, String> {
    Ok(runtime.connection.status().await)
}
#[tauri::command]
pub async fn subscription_sign_in(runtime: State<'_, AgentRuntime>) -> Result<(), String> {
    runtime.connection.start_sign_in().await
}
#[tauri::command]
pub async fn subscription_cancel_sign_in(runtime: State<'_, AgentRuntime>) -> Result<(), String> {
    runtime.connection.cancel_sign_in().await;
    Ok(())
}
#[tauri::command]
pub async fn subscription_models(
    runtime: State<'_, AgentRuntime>,
) -> Result<Vec<AvailableModel>, String> {
    runtime.connection.models().await
}

#[derive(serde::Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct AskRequest {
    pub run_id: String,
    pub thread_id: String,
    pub request_text: String,
    pub sources: Vec<EvidenceSelection>,
    pub anchor: PassageAnchor,
    pub base_revision: i32,
    pub parent_id: Option<String>,
    pub model: String,
    #[serde(default)]
    pub message_id: Option<String>,
}

#[tauri::command]
pub async fn ask_agent(
    store: State<'_, Store>,
    runtime: State<'_, AgentRuntime>,
    request: AskRequest,
) -> Result<Discussion, String> {
    if request.run_id.len() > 100 || request.run_id.is_empty() {
        return Err("Invalid request ID.".into());
    }
    let (context, evidence) = store
        .proposal_context(
            &request.thread_id,
            &request.request_text,
            &request.sources,
            &request.anchor,
            request.base_revision,
            request.parent_id.as_deref(),
        )
        .await
        .map_err(|e| e.to_string())?;
    if let Some(message_id) = &request.message_id {
        let discussion = store
            .discussion(&request.thread_id)
            .await
            .map_err(|e| e.to_string())?;
        if !discussion.messages.iter().any(|m| {
            &m.id == message_id
                && m.actor_id == "local-operator"
                && m.body == request.request_text.trim()
        }) {
            return Err("The request does not match the posted comment.".into());
        }
    }
    let (tx, mut cancel) = oneshot::channel();
    {
        let mut pending = runtime.requests.lock().await;
        if !pending.is_empty() {
            return Err("Wait for the current agent request or cancel it first.".into());
        }
        pending.insert(request.run_id.clone(), tx);
    }
    let result = async {
        if request.message_id.is_none() {
            store
                .reply(&request.thread_id, &request.request_text)
                .await
                .map_err(|e| e.to_string())?;
        }
        let generation = runtime.connection.generate(&request.model, &context);
        let candidate: ProposalCandidate = tokio::select! {
            biased;
            _ = &mut cancel => return Err("Request cancelled. No suggestion was saved.".into()),
            result = generation => result?,
        };
        // The cancellation acknowledgement and final save share this gate. A late cancel cannot promise that a saved result was discarded.
        let mut pending = runtime.requests.lock().await;
        if pending.remove(&request.run_id).is_none() {
            return Err("Request cancelled. No suggestion was saved.".into());
        }
        store
            .record_candidate(CandidateSubmission {
                thread_id: &request.thread_id,
                request: &context,
                anchor: &request.anchor,
                evidence: &evidence,
                candidate: &candidate,
                parent_id: request.parent_id.as_deref(),
                origin: &format!("ChatGPT · {}", request.model),
            })
            .await
            .map_err(|e| e.to_string())
    }
    .await;
    runtime.requests.lock().await.remove(&request.run_id);
    result
}
#[tauri::command]
pub async fn cancel_agent(
    runtime: State<'_, AgentRuntime>,
    run_id: String,
) -> Result<bool, String> {
    Ok(runtime
        .requests
        .lock()
        .await
        .remove(&run_id)
        .is_some_and(|sender| sender.send(()).is_ok()))
}

// Test injection is compiled out of ordinary builds and cannot be selected through the product UI.
#[cfg(feature = "native-e2e")]
#[tauri::command]
pub async fn native_test_candidate(
    store: State<'_, Store>,
    thread_id: String,
    sources: Option<Vec<EvidenceSelection>>,
) -> Result<Discussion, String> {
    let d = store
        .discussion(&thread_id)
        .await
        .map_err(|e| e.to_string())?;
    let current = store.read(&d.page_id).await.map_err(|e| e.to_string())?;
    let parent = d
        .proposals
        .iter()
        .find(|p| p.status == "pending")
        .map(|p| p.id.as_str());
    let sources = sources.unwrap_or_else(|| {
        vec![EvidenceSelection {
            range: None,
            object_id: "orientation-recording".into(),
            revision: 1,
            block_id: "passage-18".into(),
        }]
    });
    let (context, evidence) = store
        .proposal_context(
            &thread_id,
            "Check the recording's booking-calendar guidance.",
            &sources,
            &d.anchor,
            current.revision,
            parent,
        )
        .await
        .map_err(|e| e.to_string())?;
    let candidate = ProposalCandidate {
        proposed_wording:
            "Check the equipment page for the current booking calendar before your first session."
                .into(),
        explanation: "The orientation transcript names the booking calendar on the equipment page."
            .into(),
        cited_source_ids: vec![evidence[0].id.clone()],
    };
    store
        .record_candidate(CandidateSubmission {
            thread_id: &thread_id,
            request: &context,
            anchor: &d.anchor,
            evidence: &evidence,
            candidate: &candidate,
            parent_id: parent,
            origin: "Test fixture · no model call",
        })
        .await
        .map_err(|e| e.to_string())
}
