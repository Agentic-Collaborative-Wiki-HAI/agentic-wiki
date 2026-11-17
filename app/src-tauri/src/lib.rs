#[cfg(all(feature = "native-e2e", not(debug_assertions)))]
compile_error!("Native WebDriver instrumentation is only allowed in debug builds.");

pub mod agent;
pub mod codex;
pub mod domain;
pub mod review;
pub mod store;
pub mod subscription;

#[cfg(feature = "desktop")]
mod desktop_review;

#[cfg(feature = "desktop")]
mod desktop {
    use super::{
        domain::RichDocument,
        store::{Draft, HistoryEntry, Page, Store},
    };
    use crate::desktop_review::*;
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
    ) -> Result<Page, String> {
        store
            .save(&id, base_revision, &document)
            .await
            .map_err(|e| e.to_string())
    }
    #[tauri::command]
    async fn save_draft(
        store: State<'_, Store>,
        id: String,
        base_revision: i32,
        document: RichDocument,
    ) -> Result<(), String> {
        store
            .put_draft(&id, base_revision, &document)
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
                let connection_dir = dir.join("chatgpt");
                #[cfg(feature = "native-e2e")]
                let connection_dir = if std::env::var("WIKI_E2E_LIVE_SUBSCRIPTION").as_deref()
                    == Ok("1")
                {
                    if dir == app_dir || !dir.is_absolute() {
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
                app.manage(AgentRuntime {
                    connection: crate::subscription::Subscription::open(connection_dir)?,
                    requests: tokio::sync::Mutex::new(std::collections::HashMap::new()),
                });
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
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
                available_evidence,
                page_citations,
                read_revision,
                subscription_status,
                subscription_sign_in,
                subscription_cancel_sign_in,
                subscription_models,
                ask_agent,
                cancel_agent,
                #[cfg(feature = "native-e2e")]
                native_test_candidate
            ])
            .run(tauri::generate_context!())
            .expect("unable to run Agentic Wiki");
    }
}
#[cfg(feature = "desktop")]
pub use desktop::run;
