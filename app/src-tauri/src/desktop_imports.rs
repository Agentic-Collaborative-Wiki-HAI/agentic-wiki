use crate::{
    imports::*,
    sources,
    store::{Page, Store},
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tauri::State;
use tokio::sync::Mutex;
#[derive(Default)]
pub struct ImportRuntime {
    pending: Mutex<HashMap<String, (Instant, SourceSettings, PendingImport)>>,
}
#[tauri::command]
pub async fn source_settings(store: State<'_, Store>) -> Result<SourceSettings, String> {
    store.source_settings().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn save_source_settings(
    store: State<'_, Store>,
    settings: SourceSettings,
) -> Result<(), String> {
    store
        .save_source_settings(&settings)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub fn installed_drive_tool() -> Option<String> {
    sources::installed_drive_tool()
}
#[tauri::command]
pub async fn list_import_sources(
    store: State<'_, Store>,
    provider: SourceProvider,
    folder_id: Option<String>,
) -> Result<SourceListing, String> {
    let s = store.source_settings().await.map_err(|e| e.to_string())?;
    match provider {
        SourceProvider::Local => {
            tauri::async_runtime::spawn_blocking(move || sources::list_local(&s))
                .await
                .map_err(|_| "Could not read the local source folder.")?
                .map_err(|e| e.to_string())
        }
        SourceProvider::Drive => sources::list_drive(&s, folder_id.as_deref())
            .await
            .map_err(|e| e.to_string()),
    }
}
#[tauri::command]
pub async fn preview_source_import(
    store: State<'_, Store>,
    runtime: State<'_, ImportRuntime>,
    provider: SourceProvider,
    key: String,
) -> Result<ImportPreview, String> {
    let settings = store.source_settings().await.map_err(|e| e.to_string())?;
    let s = settings.clone();
    let source = match provider {
        SourceProvider::Local => {
            tauri::async_runtime::spawn_blocking(move || sources::read_local(&s, &key))
                .await
                .map_err(|_| "Could not read the local source file.")?
                .map_err(|e| e.to_string())?
        }
        SourceProvider::Drive => sources::read_drive(&s, &key)
            .await
            .map_err(|e| e.to_string())?,
    };
    let pending = store
        .preview_import(source)
        .await
        .map_err(|e| e.to_string())?;
    let result = pending.preview.clone();
    let mut map = runtime.pending.lock().await;
    map.retain(|_, (created, _, _)| created.elapsed() < Duration::from_secs(1800));
    if map.len() >= 8 {
        return Err("Close an existing import preview before opening another.".into());
    }
    map.insert(result.token.clone(), (Instant::now(), settings, pending));
    Ok(result)
}
#[tauri::command]
pub async fn confirm_source_import(
    store: State<'_, Store>,
    runtime: State<'_, ImportRuntime>,
    token: String,
    title: String,
    kind: ImportKind,
    parent_id: Option<String>,
) -> Result<Page, String> {
    let (created, settings, pending) = runtime
        .pending
        .lock()
        .await
        .get(&token)
        .cloned()
        .ok_or("This preview has expired. Select the source again.")?;
    if created.elapsed() >= Duration::from_secs(1800)
        || settings != store.source_settings().await.map_err(|e| e.to_string())?
    {
        return Err(
            "Source settings or the preview have changed. Preview the source again.".into(),
        );
    }
    store
        .commit_import(&pending, &title, kind, parent_id.as_deref())
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn cancel_source_import(
    runtime: State<'_, ImportRuntime>,
    token: String,
) -> Result<(), String> {
    runtime.pending.lock().await.remove(&token);
    Ok(())
}
#[tauri::command]
pub async fn imported_sources(store: State<'_, Store>) -> Result<Vec<ImportedSource>, String> {
    store.imported_sources().await.map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn open_import_original(store: State<'_, Store>, id: String) -> Result<(), String> {
    let source = store
        .imported_sources()
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|s| s.id == id)
        .ok_or("Source not found.")?;
    let url = source
        .source_url
        .ok_or("This source has no Google Doc link.")?;
    let document = sources::drive_id(&url).map_err(|e| e.to_string())?;
    let url = format!("https://docs.google.com/document/d/{document}/edit");
    #[cfg(target_os = "macos")]
    let status = tokio::process::Command::new("/usr/bin/open")
        .arg(&url)
        .status()
        .await;
    #[cfg(not(target_os = "macos"))]
    let status = tokio::process::Command::new("xdg-open")
        .arg(&url)
        .status()
        .await;
    if status
        .map_err(|_| "Could not open the original document in your browser.")?
        .success()
    {
        Ok(())
    } else {
        Err("Could not open the original document in your browser.".into())
    }
}
