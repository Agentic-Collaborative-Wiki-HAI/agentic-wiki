#[cfg(all(feature = "native-e2e", not(debug_assertions)))]
compile_error!("Native WebDriver instrumentation is only allowed in debug builds.");

pub mod agent;
pub mod authoring;
pub mod backup;
pub mod chat;
#[cfg(test)]
mod chat_tests;
pub mod codex;
#[cfg(feature = "desktop")]
mod desktop_chat;
#[cfg(feature = "desktop")]
mod desktop_workspace;
pub mod domain;
#[cfg(test)]
mod import_tests;
pub mod imports;
pub mod navigation;
pub mod review;
pub mod search;
pub mod sources;
pub mod store;
pub mod subscription;
#[cfg(test)]
mod workspace_tests;

#[cfg(feature = "desktop")]
mod desktop_imports;
#[cfg(feature = "desktop")]
mod desktop_review;

#[cfg(feature = "desktop")]
mod desktop {
    use super::{
        domain::RichDocument,
        store::{Draft, HistoryEntry, Page, Store},
    };
    use crate::desktop_chat::*;
    use crate::desktop_imports::*;
    use crate::desktop_review::*;
    use crate::desktop_workspace::*;
    use tauri::{Manager, State};
    #[tauri::command]
    async fn read_page(store: State<'_, Store>, id: String) -> Result<Page, String> {
        store.read(&id).await.map_err(|e| e.to_string())
    }
    #[tauri::command]
    async fn save_page(
        store: State<'_, Store>,
        id: String,
        base_revision: i32,
        document: RichDocument,
        title: Option<String>,
    ) -> Result<Page, String> {
        store
            .save_edit(&id, base_revision, &document, title.as_deref())
            .await
            .map_err(|e| e.to_string())
    }
    #[tauri::command]
    async fn save_draft(
        store: State<'_, Store>,
        id: String,
        base_revision: i32,
        document: RichDocument,
        title: Option<String>,
    ) -> Result<(), String> {
        store
            .put_edit_draft(&id, base_revision, &document, title.as_deref())
            .await
            .map_err(|e| e.to_string())
    }
    #[tauri::command]
    async fn read_draft(store: State<'_, Store>, id: String) -> Result<Option<Draft>, String> {
        store.draft(&id).await.map_err(|e| e.to_string())
    }
    #[tauri::command]
    async fn discard_draft(store: State<'_, Store>, id: String) -> Result<(), String> {
        store.discard_draft(&id).await.map_err(|e| e.to_string())
    }
    #[tauri::command]
    async fn page_history(
        store: State<'_, Store>,
        id: String,
    ) -> Result<Vec<HistoryEntry>, String> {
        store.history(&id).await.map_err(|e| e.to_string())
    }

    #[cfg(feature = "native-e2e")]
    #[tauri::command]
    fn native_test_focus(app: tauri::AppHandle) -> Result<(), String> {
        let window = app
            .get_webview_window("main")
            .ok_or("Test window missing")?;
        window.set_focus().map_err(|e| e.to_string())?;
        let webview: &tauri::Webview = window.as_ref();
        webview.set_focus().map_err(|e| e.to_string())
    }

    pub fn run() {
        let builder = tauri::Builder::default();
        #[cfg(feature = "native-e2e")]
        let builder = builder.plugin(tauri_plugin_wdio_webdriver::init());
        builder
            .setup(|app| {
                let app_dir = app.path().app_data_dir()?;
                let dir = app_dir.clone();
                #[cfg(feature = "native-e2e")]
                let dir = std::env::var_os("WIKI_E2E_DATA_DIR")
                    .map(std::path::PathBuf::from)
                    .unwrap_or(dir);
                let workspace_root = dir;
                #[cfg(feature = "native-e2e")]
                let isolated_test_root = workspace_root != app_dir && workspace_root.is_absolute();
                let workspace_id = crate::backup::selected(&workspace_root)?;
                let database = crate::backup::database(&workspace_root, &workspace_id)?;
                let dir = database.parent().unwrap().to_path_buf();
                app.manage(crate::backup::WorkspaceRuntime {
                    root: workspace_root,
                    active: workspace_id,
                });
                let connection_dir = dir.join("chatgpt");
                #[cfg(feature = "native-e2e")]
                let connection_dir = if std::env::var("WIKI_E2E_LIVE_SUBSCRIPTION").as_deref()
                    == Ok("1")
                {
                    if !isolated_test_root {
                        return Err(
                            "Live verification requires a separate absolute test workspace.".into(),
                        );
                    }
                    // An explicit live test uses this app's connection in place, never a copy of Codex credentials.
                    app_dir.join("chatgpt")
                } else {
                    connection_dir
                };
                std::fs::create_dir_all(&dir)?;
                let store = tauri::async_runtime::block_on(Store::open(&dir.join("wiki.sqlite")))?;
                app.manage(store);
                app.manage(ImportRuntime::default());
                app.manage(AgentRuntime {
                    connection: crate::subscription::Subscription::open(connection_dir)?,
                    requests: tokio::sync::Mutex::new(std::collections::HashMap::new()),
                });
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                chat_context,
                chat_turns,
                ask_chat,
                answer_drafts,
                create_answer_draft,
                save_answer_draft,
                publish_answer_draft,
                chat_page_origin,
                #[cfg(feature = "native-e2e")]
                native_test_chat,
                read_agent_instructions,
                default_agent_instructions,
                save_agent_instructions,
                register_citation,
                register_margin_note,
                document_references,
                wiki_records,
                create_wiki_page,
                organize_wiki_page,
                search_passages,
                select_evidence,
                default_evidence,
                add_manual_citation,
                export_workspace,
                restore_workspace,
                workspace_copies,
                select_workspace,
                open_import_original,
                source_settings,
                save_source_settings,
                installed_drive_tool,
                list_import_sources,
                preview_source_import,
                confirm_source_import,
                cancel_source_import,
                imported_sources,
                read_page,
                save_page,
                save_draft,
                read_draft,
                discard_draft,
                page_history,
                list_discussions,
                start_discussion,
                reply_discussion,
                resolve_discussion,
                decide_proposal,
                revise_proposal,
                discussion_evidence,
                page_citations,
                read_revision,
                subscription_status,
                subscription_sign_in,
                subscription_cancel_sign_in,
                subscription_models,
                ask_agent,
                cancel_agent,
                #[cfg(feature = "native-e2e")]
                native_test_candidate,
                #[cfg(feature = "native-e2e")]
                native_test_focus
            ])
            .run(tauri::generate_context!())
            .expect("unable to run Agentic Wiki");
    }
}
#[cfg(feature = "desktop")]
pub use desktop::run;
