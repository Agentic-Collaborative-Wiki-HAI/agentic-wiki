use crate::{domain::Error, imports::*};
use serde::Deserialize;
use std::{
    fs,
    io::Read,
    path::{Component, Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{io::AsyncReadExt, process::Command};

fn path(value: &str) -> Result<PathBuf, Error> {
    let expanded = if let Some(rest) = value.strip_prefix("~/") {
        PathBuf::from(
            std::env::var_os("HOME").ok_or_else(|| invalid("Home directory is unavailable."))?,
        )
        .join(rest)
    } else {
        PathBuf::from(value)
    };
    if !expanded.is_absolute() {
        return Err(invalid("Use an absolute folder or tool path."));
    }
    Ok(expanded)
}
pub fn drive_id(value: &str) -> Result<String, Error> {
    let value = value.trim();
    let id = [
        "https://docs.google.com/document/d/",
        "https://drive.google.com/drive/folders/",
        "https://drive.google.com/drive/u/0/folders/",
    ]
    .into_iter()
    .find_map(|prefix| value.strip_prefix(prefix))
    .unwrap_or(value)
    .split(['/', '?', '#'])
    .next()
    .unwrap_or("");
    if id.is_empty()
        || id.len() > 200
        || !id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(invalid(
            "Enter a Google document/folder ID or its standard sharing link.",
        ));
    }
    Ok(id.into())
}
pub fn validate_settings(s: &SourceSettings) -> Result<SourceSettings, Error> {
    let mut s = s.clone();
    for value in [
        &mut s.local_folder,
        &mut s.agent_hands_path,
        &mut s.google_account,
        &mut s.drive_folder,
    ] {
        *value = value.trim().to_owned();
        if value.len() > 4096 || value.chars().any(char::is_control) {
            return Err(invalid("Source settings contain an invalid value."));
        }
    }
    if !s.local_folder.is_empty() {
        let root = fs::canonicalize(path(&s.local_folder)?).map_err(|_| {
            invalid("The source folder is unavailable. Check its path and sync status.")
        })?;
        if !root.is_dir() {
            return Err(invalid("Choose a source folder, not a file."));
        }
        s.local_folder = root.to_string_lossy().into();
    }
    if !s.agent_hands_path.is_empty() {
        let script = path(&s.agent_hands_path)?;
        if !script.ends_with("use-google-drive/scripts/drive") || !script.is_file() {
            return Err(invalid(
                "Choose the Agent Hands use-google-drive/scripts/drive entrypoint.",
            ));
        }
        // Keep the installed path: shell-relative sibling lookup must resolve within the chosen Agent Hands checkout.
        s.agent_hands_path = fs::canonicalize(script)
            .map_err(|_| invalid("The Agent Hands entrypoint is unavailable."))?
            .to_string_lossy()
            .into();
    }
    if !s.google_account.is_empty()
        && !s
            .google_account
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(invalid("Use the account alias configured in Agent Hands."));
    }
    if !s.drive_folder.is_empty() {
        s.drive_folder = drive_id(&s.drive_folder)?;
    }
    Ok(s)
}
pub fn installed_drive_tool() -> Option<String> {
    let home = std::env::var_os("HOME").map(PathBuf::from)?;
    [
        home.join(".codex/skills/use-google-drive/scripts/drive"),
        home.join(".claude/skills/use-google-drive/scripts/drive"),
    ]
    .into_iter()
    .find_map(|p| fs::canonicalize(p).ok())
    .filter(|p| p.is_file())
    .map(|p| p.to_string_lossy().into())
}
fn text_extension(p: &Path) -> bool {
    p.extension().and_then(|s| s.to_str()).is_some_and(|s| {
        matches!(
            s.to_ascii_lowercase().as_str(),
            "txt" | "md" | "srt" | "vtt"
        )
    })
}
fn local_root(s: &SourceSettings) -> Result<PathBuf, Error> {
    if s.local_folder.is_empty() {
        return Err(invalid("Set a local source folder in Settings first."));
    }
    fs::canonicalize(path(&s.local_folder)?).map_err(|_| {
        invalid("The source folder is unavailable. Check that it is synced and readable.")
    })
}
pub fn list_local(s: &SourceSettings) -> Result<SourceListing, Error> {
    let root = local_root(s)?;
    let mut entries =
        fs::read_dir(root).map_err(|_| invalid("The source folder cannot be read."))?;
    let mut items = vec![];
    let mut scanned = 0;
    let mut limited = false;
    for entry in entries.by_ref() {
        scanned += 1;
        if scanned > 500 {
            limited = true;
            break;
        }
        let entry = entry.map_err(|_| invalid("A source folder entry could not be read."))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        let kind = entry
            .file_type()
            .map_err(|_| invalid("A source file could not be inspected."))?;
        if !kind.is_file() {
            continue;
        }
        items.push(SourceItem {
            folder: false,
            key: name.clone(),
            title: name,
            modified: None,
            supported: text_extension(&entry.path()),
        });
    }
    items.sort_by_key(|a| a.title.to_lowercase());
    Ok(SourceListing {items, notice:if limited {"Showing a bounded sample from the first 500 entries. Choose a smaller source folder to see everything. "} else {""}.to_owned()+"TXT, Markdown, SRT and VTT text files in this folder only. Google Doc shortcuts need the Meet / Drive source; audio, DOCX and PDF import are not available yet."})
}
fn read_bounded(path: &Path) -> Result<Vec<u8>, Error> {
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(path).map_err(|_| {
        invalid("The file could not be opened. Check access and whether sync has downloaded it.")
    })?;
    let metadata = file
        .metadata()
        .map_err(|_| invalid("The file could not be inspected."))?;
    if !metadata.is_file() {
        return Err(invalid("Choose a regular text file."));
    }
    if metadata.len() > MAX_TEXT_BYTES as u64 {
        return Err(invalid("Text imports are limited to 1 MB."));
    }
    let mut bytes = vec![];
    file.take(MAX_TEXT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| invalid("The source could not be read completely."))?;
    if bytes.len() > MAX_TEXT_BYTES {
        return Err(invalid("Text imports are limited to 1 MB."));
    }
    Ok(bytes)
}
pub fn read_local(s: &SourceSettings, key: &str) -> Result<SourceBytes, Error> {
    let key_path = Path::new(key);
    if key_path.components().count() != 1
        || !matches!(key_path.components().next(), Some(Component::Normal(_)))
        || key.contains('\\')
    {
        return Err(invalid(
            "Select a file directly inside the configured source folder.",
        ));
    }
    if !text_extension(key_path) {
        return Err(invalid(
            "This format is not a text import. Use TXT, Markdown, SRT or VTT; use Meet / Drive for Google Docs.",
        ));
    }
    let root = local_root(s)?;
    let candidate = root.join(key_path);
    if fs::symlink_metadata(&candidate)
        .map_err(|_| invalid("The source file is missing."))?
        .file_type()
        .is_symlink()
    {
        return Err(invalid(
            "File symlinks are not imported. Put the actual text file in the source folder.",
        ));
    }
    let canonical =
        fs::canonicalize(&candidate).map_err(|_| invalid("The source file is unavailable."))?;
    if canonical.parent() != Some(root.as_path()) {
        return Err(invalid("The source is outside the configured folder."));
    }
    let bytes = read_bounded(&canonical)?;
    let modified = fs::metadata(&canonical)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| format!("Unix time {}", d.as_secs()));
    Ok(SourceBytes {
        provider: SourceProvider::Local,
        locator: canonical.to_string_lossy().into(),
        name: key.into(),
        url: None,
        modified,
        bytes,
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DriveFile {
    id: String,
    name: String,
    mime_type: String,
    modified_time: Option<String>,
    #[serde(default)]
    parents: Vec<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DriveList {
    files: Vec<DriveFile>,
    next_page_token: Option<String>,
}

// Kill a timed-out shell and its descendants, not just the wrapper while curl keeps running.
struct ProcessGroup(u32);
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        #[cfg(unix)]
        if self.0 != 0 {
            unsafe {
                libc::kill(-(self.0 as i32), libc::SIGKILL);
            }
        }
    }
}
async fn bounded_output(
    reader: impl tokio::io::AsyncRead + Unpin,
    limit: u64,
) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];
    reader
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .await
        .map_err(|_| invalid("Agent Hands output could not be read."))?;
    if bytes.len() as u64 > limit {
        return Err(invalid(
            "Agent Hands returned more output than this import can safely process.",
        ));
    }
    Ok(bytes)
}
async fn run_drive(
    s: &SourceSettings,
    operation: &str,
    arguments: &[&str],
) -> Result<Vec<u8>, Error> {
    if s.agent_hands_path.is_empty() || s.google_account.is_empty() {
        return Err(invalid(
            "Set the Agent Hands Drive tool and account alias in Settings first.",
        ));
    }
    let mut bridge_settings = s.clone();
    bridge_settings.local_folder.clear();
    let s = validate_settings(&bridge_settings)?;
    let mut cmd = Command::new("/bin/bash");
    cmd.arg("--noprofile")
        .arg("--norc")
        .arg(&s.agent_hands_path)
        .arg(operation)
        .arg(&s.google_account)
        .args(arguments)
        .env_remove("BASH_ENV")
        .env_remove("ENV")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let mut search_paths: Vec<PathBuf> =
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()).collect();
    search_paths.extend([
        PathBuf::from("/opt/homebrew/bin"),
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("/usr/bin"),
        PathBuf::from("/bin"),
    ]);
    if let Some(home) = std::env::var_os("HOME") {
        search_paths.push(PathBuf::from(home).join(".local/bin"));
    }
    if let Ok(paths) = std::env::join_paths(search_paths) {
        cmd.env("PATH", paths);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.as_std_mut().process_group(0);
    }
    let mut child=cmd.spawn().map_err(|_|invalid("Agent Hands could not start. Check the tool path and its Bash, curl, jq and Python/uv installation."))?;
    let mut group = ProcessGroup(child.id().unwrap_or(0));
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let operation_future = async {
        let (out, _) = tokio::try_join!(
            bounded_output(stdout, 512_000),
            bounded_output(stderr, 64_000)
        )?;
        let status = child
            .wait()
            .await
            .map_err(|_| invalid("Agent Hands did not finish cleanly."))?;
        if !status.success() {
            return Err(invalid(format!(
                "Agent Hands {operation} failed. Check the selected account’s Drive access and the document or folder link. No source was imported."
            )));
        }
        Ok(out)
    };
    let result = tokio::time::timeout(Duration::from_secs(60), operation_future)
        .await
        .map_err(|_| {
            invalid("Agent Hands timed out after 60 seconds. Check the connection and retry.")
        })?;
    if result.is_ok() {
        group.0 = 0;
    }
    result
}
async fn meta(s: &SourceSettings, id: &str) -> Result<DriveFile, Error> {
    let bytes = run_drive(s, "meta", &[id, "--json"]).await?;
    let meta: DriveFile = serde_json::from_slice(&bytes)
        .map_err(|_| invalid("Agent Hands returned invalid document metadata."))?;
    if meta.id != id {
        return Err(invalid(
            "The returned Google file does not match the requested ID.",
        ));
    }
    Ok(meta)
}
async fn within_drive_folder(s: &SourceSettings, file: &DriveFile) -> Result<(), Error> {
    if s.drive_folder.is_empty() {
        return Ok(());
    }
    let root = drive_id(&s.drive_folder)?;
    if file.id == root {
        return Ok(());
    }
    let mut parents = file.parents.clone();
    let mut visited = std::collections::HashSet::new();
    for _ in 0..16 {
        if parents.contains(&root) {
            return Ok(());
        }
        if parents.len() != 1 || !visited.insert(parents[0].clone()) {
            break;
        }
        parents = meta(s, &parents[0]).await?.parents;
    }
    Err(invalid(
        "This source is outside the configured Meet folder, or its folder ancestry could not be verified. Choose a source inside that folder.",
    ))
}
pub async fn list_drive(
    s: &SourceSettings,
    selected_folder: Option<&str>,
) -> Result<SourceListing, Error> {
    if s.drive_folder.is_empty() {
        return Err(invalid(
            "Set a Meet folder in Settings, or paste a Google Doc link to preview one document.",
        ));
    }
    let folder = drive_id(selected_folder.unwrap_or(&s.drive_folder))?;
    let info = meta(s, &folder).await?;
    if info.mime_type != "application/vnd.google-apps.folder" {
        return Err(invalid("Choose a Google Drive folder."));
    }
    within_drive_folder(s, &info).await?;
    let bytes = run_drive(s, "list", &[&folder, "--json"]).await?;
    let list: DriveList = serde_json::from_slice(&bytes)
        .map_err(|_| invalid("Agent Hands returned an invalid folder listing."))?;
    let items = list
        .files
        .into_iter()
        .filter(|f| {
            matches!(
                f.mime_type.as_str(),
                "application/vnd.google-apps.document" | "application/vnd.google-apps.folder"
            )
        })
        .map(|f| SourceItem {
            folder: f.mime_type == "application/vnd.google-apps.folder",
            key: f.id,
            title: f.name,
            modified: f.modified_time,
            supported: true,
        })
        .collect();
    Ok(SourceListing {items,notice:if list.next_page_token.is_some(){"This folder has more results than Agent Hands returned. This is one page, not the full folder. Paste a specific document link or choose a smaller folder."}else{"Open a meeting folder or select a Google Doc to preview. Confirm whether its content is a transcript or meeting notes before importing."}.into()})
}
pub async fn read_drive(s: &SourceSettings, key: &str) -> Result<SourceBytes, Error> {
    let id = drive_id(key)?;
    let before = meta(s, &id).await?;
    if before.mime_type != "application/vnd.google-apps.document" {
        return Err(invalid(
            "Choose a native Google Doc containing a transcript or notes.",
        ));
    }
    within_drive_folder(s, &before).await?;
    let dir = tempfile::tempdir()
        .map_err(|_| invalid("A private export folder could not be created."))?;
    let out = dir.path().join("transcript.txt");
    run_drive(
        s,
        "download",
        &[
            &id,
            "--out",
            out.to_str()
                .ok_or_else(|| invalid("Invalid temporary export path."))?,
            "--mime",
            "text/plain",
        ],
    )
    .await?;
    let bytes = read_bounded(&out)?;
    let after = meta(s, &id).await?;
    if before.modified_time != after.modified_time
        || before.name != after.name
        || before.parents != after.parents
        || before.mime_type != after.mime_type
    {
        return Err(invalid(
            "The Google Doc changed during export. Preview it again before importing.",
        ));
    }
    within_drive_folder(s, &after).await?;
    Ok(SourceBytes {
        provider: SourceProvider::Drive,
        locator: id.clone(),
        name: before.name,
        url: Some(format!("https://docs.google.com/document/d/{id}/edit")),
        modified: before.modified_time,
        bytes,
    })
}
