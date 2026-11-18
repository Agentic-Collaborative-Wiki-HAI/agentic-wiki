use crate::{
    backup::{WorkspaceCopy, WorkspaceRuntime},
    domain::PassageAnchor,
    navigation::WikiRecord,
    review::{Evidence, EvidenceSelection},
    search::{EvidenceContext, PassageResult},
    store::{Page, Store},
};
use tauri::State;
#[tauri::command]
pub async fn wiki_records(store: State<'_, Store>) -> Result<Vec<WikiRecord>, String> {
    store.records().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_wiki_page(
    store: State<'_, Store>,
    id: String,
    title: String,
    parent_id: Option<String>,
) -> Result<Page, String> {
    store
        .create_page(&id, &title, parent_id.as_deref())
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn organize_wiki_page(
    store: State<'_, Store>,
    id: String,
    metadata_version: i32,
    title: String,
    parent_id: Option<String>,
) -> Result<Page, String> {
    store
        .organize_page(&id, metadata_version, &title, parent_id.as_deref())
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn search_passages(
    store: State<'_, Store>,
    query: String,
    object_ids: Option<Vec<String>>,
) -> Result<Vec<PassageResult>, String> {
    store
        .search_passages(&query, object_ids.as_deref(), 60)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn select_evidence(
    store: State<'_, Store>,
    source: EvidenceSelection,
) -> Result<Evidence, String> {
    store.evidence(&source).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn default_evidence(
    store: State<'_, Store>,
    id: String,
    query: String,
) -> Result<EvidenceContext, String> {
    store
        .default_evidence(&id, &query)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn add_manual_citation(
    store: State<'_, Store>,
    id: String,
    page_id: String,
    base_revision: i32,
    anchor: PassageAnchor,
    source: EvidenceSelection,
) -> Result<Page, String> {
    store
        .manual_citation(&id, &page_id, base_revision, &anchor, &source)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn export_workspace(store: State<'_, Store>, path: String) -> Result<String, String> {
    store.export_backup(&path).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn restore_workspace(
    runtime: State<'_, WorkspaceRuntime>,
    path: String,
) -> Result<WorkspaceCopy, String> {
    runtime.restore(&path).await.map_err(|e| e.to_string())
}
#[tauri::command]
pub fn workspace_copies(
    runtime: State<'_, WorkspaceRuntime>,
) -> Result<Vec<WorkspaceCopy>, String> {
    runtime.copies().map_err(|e| e.to_string())
}
#[tauri::command]
pub fn select_workspace(runtime: State<'_, WorkspaceRuntime>, id: String) -> Result<(), String> {
    runtime.select(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn read_agent_instructions(
    store: State<'_, Store>,
) -> Result<crate::authoring::AgentInstructions, String> {
    store.instructions().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub fn default_agent_instructions() -> crate::authoring::AgentInstructions {
    Default::default()
}
#[tauri::command]
pub async fn save_agent_instructions(
    store: State<'_, Store>,
    instructions: crate::authoring::AgentInstructions,
) -> Result<(), String> {
    store
        .save_instructions(&instructions)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn register_citation(
    store: State<'_, Store>,
    source: EvidenceSelection,
) -> Result<crate::authoring::ReferenceEntry, String> {
    store
        .register_citation(&source)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn register_margin_note(
    store: State<'_, Store>,
    title: String,
    body: String,
) -> Result<crate::authoring::ReferenceEntry, String> {
    store
        .register_note(&title, &body)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn document_references(
    store: State<'_, Store>,
    document: crate::domain::RichDocument,
) -> Result<Vec<crate::authoring::ReferenceEntry>, String> {
    store
        .references_for(&document)
        .await
        .map_err(|e| e.to_string())
}
