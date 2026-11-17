#[cfg(all(feature = "native-e2e", not(debug_assertions)))]
compile_error!("Native WebDriver instrumentation is only allowed in debug builds.");

pub mod agent;
pub mod codex;
pub mod domain;
pub mod store;

#[cfg(feature = "desktop")]
mod desktop {
    use super::{
        domain::RichDocument,
        store::{Draft, HistoryEntry, Page, Store},
    };
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
                let dir = app.path().app_data_dir()?;
                #[cfg(feature = "native-e2e")]
                let dir = std::env::var_os("WIKI_E2E_DATA_DIR")
                    .map(std::path::PathBuf::from)
                    .unwrap_or(dir);
                std::fs::create_dir_all(&dir)?;
                let store = tauri::async_runtime::block_on(Store::open(&dir.join("wiki.sqlite")))?;
                app.manage(store);
                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                read_page,
                save_page,
                save_draft,
                read_draft,
                discard_draft,
                page_history
            ])
            .run(tauri::generate_context!())
            .expect("unable to run Agentic Wiki");
    }
}
#[cfg(feature = "desktop")]
pub use desktop::run;
