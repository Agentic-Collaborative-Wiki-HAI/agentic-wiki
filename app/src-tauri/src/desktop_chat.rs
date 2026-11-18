use crate::{
    chat::{AnswerDraft, ChatTurn},
    desktop_review::AgentRuntime,
    domain::RichDocument,
    review::EvidenceSelection,
    search::EvidenceContext,
    store::{Page, Store},
};
use tauri::State;
use tokio::sync::oneshot;
#[tauri::command]
pub async fn chat_context(
    store: State<'_, Store>,
    id: String,
    query: String,
) -> Result<EvidenceContext, String> {
    store
        .chat_context(&id, &query)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn chat_turns(store: State<'_, Store>) -> Result<Vec<ChatTurn>, String> {
    store.chat_turns().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn answer_drafts(store: State<'_, Store>) -> Result<Vec<AnswerDraft>, String> {
    store.answer_drafts().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_answer_draft(
    store: State<'_, Store>,
    turn_id: String,
) -> Result<AnswerDraft, String> {
    store
        .create_answer_draft(&turn_id)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn save_answer_draft(
    store: State<'_, Store>,
    id: String,
    version: i32,
    title: String,
    parent_id: Option<String>,
    document: RichDocument,
) -> Result<AnswerDraft, String> {
    store
        .save_answer_draft(&id, version, &title, parent_id.as_deref(), &document)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn publish_answer_draft(
    store: State<'_, Store>,
    id: String,
    version: i32,
) -> Result<Page, String> {
    store
        .publish_answer_draft(&id, version)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn chat_page_origin(
    store: State<'_, Store>,
    id: String,
) -> Result<Option<ChatTurn>, String> {
    store.chat_page_origin(&id).await.map_err(|e| e.to_string())
}
#[derive(serde::Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct ChatSubmission {
    pub id: String,
    pub page_id: String,
    pub question: String,
    pub sources: Vec<EvidenceSelection>,
    pub model: String,
}
#[tauri::command]
pub async fn ask_chat(
    store: State<'_, Store>,
    runtime: State<'_, AgentRuntime>,
    request: ChatSubmission,
) -> Result<ChatTurn, String> {
    uuid::Uuid::parse_str(&request.id).map_err(|_| "Invalid Chat request.")?;
    let (sender, mut cancel) = oneshot::channel();
    {
        let mut pending = runtime.requests.lock().await;
        if !pending.is_empty() {
            return Err("Wait for the current agent request or cancel it first.".into());
        }
        pending.insert(request.id.clone(), sender);
    }
    let mut was_cancelled = false;
    let result = async {
        let t = store.prepare_chat(&request.id, &request.page_id, &request.question, &request.sources, &request.model).await.map_err(|e| e.to_string())?;
        let generation = runtime.connection.answer(&request.model, &t.request);
        let answer = tokio::select! {
            biased;
            _ = &mut cancel => { was_cancelled = true; return Err("Request cancelled. No answer was saved.".into()); },
            result = generation => result?,
        };
        let mut pending = runtime.requests.lock().await;
        if pending.remove(&request.id).is_none() {
            was_cancelled = true;
            return Err("Request cancelled. No answer was saved.".into());
        }
        store.complete_chat(&request.id, &answer).await.map_err(|e| e.to_string())
    }.await;
    let mut pending = runtime.requests.lock().await;
    pending.remove(&request.id);
    if let Err(error) = &result {
        store
            .fail_chat(&request.id, was_cancelled, error)
            .await
            .map_err(|e| e.to_string())?;
    }
    result
}
// Fictional output only in the opt-in debug build; the production generation path never falls back to it.
#[cfg(feature = "native-e2e")]
#[tauri::command]
pub async fn native_test_chat(
    store: State<'_, Store>,
    request: ChatSubmission,
    missing: bool,
) -> Result<ChatTurn, String> {
    let t = store
        .prepare_chat(
            &request.id,
            &request.page_id,
            &request.question,
            &request.sources,
            "Test fixture · no model call",
        )
        .await
        .map_err(|e| e.to_string())?;
    let paragraphs = if missing {
        vec![]
    } else {
        vec![crate::chat::AnswerParagraph{text:"Check the equipment page for the current booking calendar before your first session.".into(),cited_source_ids:vec![t.evidence.first().ok_or("Select fixture evidence")?.id.clone()]}]
    };
    let answer = crate::chat::ChatAnswer {
        title: "Planning a microscope session".into(),
        paragraphs,
        missing_evidence: if missing {
            "The selected excerpts do not establish the answer. Choose another source.".into()
        } else {
            String::new()
        },
    };
    store
        .complete_chat(&request.id, &answer)
        .await
        .map_err(|e| e.to_string())
}
