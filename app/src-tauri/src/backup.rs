use crate::{
    domain::{Error, RichDocument},
    store::Store,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{
    Connection, Row,
    sqlite::{SqliteConnectOptions, SqliteConnection},
};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use ts_rs::TS;
const MAX_BACKUP: u64 = 100_000_000;
fn bad(s: &str) -> Error {
    Error::Invalid(s.into())
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Archive {
    format_version: u32,
    sha256: String,
    database: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct WorkspaceCopy {
    pub id: String,
    pub label: String,
    pub active: bool,
    pub selected: bool,
}
pub struct WorkspaceRuntime {
    pub root: PathBuf,
    pub active: String,
}
fn copy_id(id: &str) -> Result<(), Error> {
    if id != "original" {
        uuid::Uuid::parse_str(id).map_err(|_| bad("Invalid workspace copy."))?;
    }
    Ok(())
}
pub fn database(root: &Path, id: &str) -> Result<PathBuf, Error> {
    copy_id(id)?;
    Ok(if id == "original" {
        root.join("wiki.sqlite")
    } else {
        root.join("restored").join(id).join("wiki.sqlite")
    })
}
pub fn selected(root: &Path) -> Result<String, Error> {
    let path = root.join("active-workspace.json");
    if !path.exists() {
        return Ok("original".into());
    }
    let bytes = fs::read(path).map_err(|_| bad("Cannot read workspace selection."))?;
    if bytes.len() > 1000 {
        return Err(bad("Invalid workspace selection."));
    }
    let id: String = serde_json::from_slice(&bytes)?;
    copy_id(&id)?;
    if !database(root, &id)?.is_file() {
        return Err(bad(
            "The selected workspace is missing. Your original database is unchanged.",
        ));
    }
    Ok(id)
}
fn user_path(value: &str) -> Result<PathBuf, Error> {
    let p = if let Some(rest) = value.strip_prefix("~/") {
        PathBuf::from(std::env::var_os("HOME").ok_or_else(|| bad("Home folder unavailable."))?)
            .join(rest)
    } else {
        PathBuf::from(value)
    };
    if !p.is_absolute() || p.extension().and_then(|s| s.to_str()) != Some("wiki-backup") {
        return Err(bad("Use an absolute .wiki-backup file path."));
    }
    Ok(p)
}
async fn schema(conn: &mut SqliteConnection) -> Result<Vec<(String, String, String)>, Error> {
    Ok(sqlx::query("SELECT type,name,sql FROM sqlite_schema WHERE sql IS NOT NULL AND name NOT LIKE 'sqlite_%' ORDER BY type,name").fetch_all(conn).await?.into_iter().map(|r|(r.get("type"),r.get("name"),r.get("sql"))).collect())
}
async fn validate_copy(path: &Path) -> Result<i64, Error> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .read_only(true)
        .foreign_keys(true);
    let mut conn = SqliteConnection::connect_with(&options).await?;
    let check: String = sqlx::query_scalar("PRAGMA integrity_check")
        .fetch_one(&mut conn)
        .await?;
    if check != "ok" {
        return Err(bad("Backup database integrity check failed."));
    }
    let expected_dir =
        tempfile::tempdir().map_err(|_| bad("Cannot create validation workspace."))?;
    let expected = Store::open(&expected_dir.path().join("wiki.sqlite")).await?;
    let expected_schema = schema(&mut *expected.pool.acquire().await?).await?;
    if schema(&mut conn).await? != expected_schema {
        return Err(bad(
            "This backup has an unsupported database schema. Use a matching app version.",
        ));
    }
    let expected_migrations: Vec<(i64, Vec<u8>)> = sqlx::query_as(
        "SELECT version,checksum FROM _sqlx_migrations WHERE success=1 ORDER BY version",
    )
    .fetch_all(&expected.pool)
    .await?;
    let migrations: Vec<(i64, Vec<u8>)> = sqlx::query_as(
        "SELECT version,checksum FROM _sqlx_migrations WHERE success=1 ORDER BY version",
    )
    .fetch_all(&mut conn)
    .await?;
    if migrations != expected_migrations {
        return Err(bad("Backup migrations do not match this app version."));
    }
    expected.pool.close().await;
    if !sqlx::query("PRAGMA foreign_key_check")
        .fetch_all(&mut conn)
        .await?
        .is_empty()
    {
        return Err(bad("Backup contains broken record references."));
    }
    crate::chat::validate_chat_records(&mut conn).await?;
    let heads:i64=sqlx::query_scalar("SELECT count(*) FROM objects o LEFT JOIN revisions r ON r.object_id=o.id AND r.revision=o.current_revision WHERE r.object_id IS NULL").fetch_one(&mut conn).await?;
    if heads != 0 {
        return Err(bad("Backup is missing current page revisions."));
    }
    for row in sqlx::query(
        "SELECT document_json FROM revisions UNION ALL SELECT document_json FROM drafts UNION ALL SELECT document_json FROM chat_page_drafts",
    )
    .fetch_all(&mut conn)
    .await?
    {
        let doc: RichDocument = serde_json::from_str(row.get("document_json"))?;
        doc.validate()?;
        crate::review::validate_citations(&mut conn, &doc).await?;
    }
    for row in
        sqlx::query("SELECT source_id,source_revision,block_id,quote,range_json FROM citations")
            .fetch_all(&mut conn)
            .await?
    {
        let selection = crate::review::EvidenceSelection {
            object_id: row.get("source_id"),
            revision: row.get("source_revision"),
            block_id: row.get("block_id"),
            range: row
                .get::<Option<String>, _>("range_json")
                .map(|v| serde_json::from_str(&v))
                .transpose()?,
        };
        let evidence = crate::review::evidence_on(&mut conn, &selection).await?;
        if evidence.quote != row.get::<String, _>("quote") {
            return Err(bad("Backup citation does not match its pinned source."));
        }
    }
    for row in sqlx::query("SELECT original_bytes,sha256 FROM import_snapshots")
        .fetch_all(&mut conn)
        .await?
    {
        let bytes: Vec<u8> = row.get("original_bytes");
        if format!("{:x}", Sha256::digest(bytes)) != row.get::<String, _>("sha256") {
            return Err(bad("An imported original failed its checksum check."));
        }
    }
    // Detect parent loops before the recursive tree UI opens this copy.
    let parents: Vec<(String, Option<String>)> = sqlx::query_as("SELECT id,parent_id FROM objects")
        .fetch_all(&mut conn)
        .await?;
    for (id, _) in &parents {
        let mut seen = std::collections::HashSet::new();
        let mut next = Some(id);
        while let Some(p) = next {
            if !seen.insert(p) {
                return Err(bad("Backup has a cycle in its wiki tree."));
            }
            next = parents
                .iter()
                .find(|(i, _)| i == p)
                .and_then(|(_, p)| p.as_ref());
        }
    }
    let count = parents.len() as i64;
    conn.close().await?;
    Ok(count)
}
impl Store {
    pub async fn export_backup(&self, destination: &str) -> Result<String, Error> {
        let destination = user_path(destination)?;
        if destination.exists() {
            return Err(bad(
                "That backup file already exists. Choose a new filename.",
            ));
        }
        let dir =
            tempfile::tempdir().map_err(|_| bad("Cannot create a private backup snapshot."))?;
        let copy = dir.path().join("wiki.sqlite");
        sqlx::query("VACUUM INTO ?")
            .bind(copy.to_string_lossy().as_ref())
            .execute(&self.pool)
            .await?;
        let mut conn =
            SqliteConnection::connect_with(&SqliteConnectOptions::new().filename(&copy)).await?;
        // Connections, machine-specific settings and rebuildable search caches do not travel with the content.
        sqlx::query("DELETE FROM app_settings")
            .execute(&mut conn)
            .await?;
        sqlx::query("DELETE FROM passage_search")
            .execute(&mut conn)
            .await?;
        sqlx::query("DELETE FROM search_versions")
            .execute(&mut conn)
            .await?;
        sqlx::query("VACUUM").execute(&mut conn).await?;
        conn.close().await?;
        validate_copy(&copy).await?;
        let bytes = fs::read(copy).map_err(|_| bad("Could not read the backup snapshot."))?;
        if bytes.len() as u64 > MAX_BACKUP * 3 / 4 {
            return Err(bad("This build supports backup files up to 100 MB."));
        }
        let archive = Archive {
            format_version: 1,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            database: STANDARD.encode(bytes),
        };
        let parent = destination
            .parent()
            .ok_or_else(|| bad("Choose a backup folder."))?;
        let mut pending = tempfile::NamedTempFile::new_in(parent)
            .map_err(|_| bad("Cannot write in that backup folder."))?;
        serde_json::to_writer(&mut pending, &archive)?;
        pending.flush().map_err(|_| bad("Backup write failed."))?;
        if pending
            .as_file()
            .metadata()
            .map_err(|_| bad("Cannot inspect backup size."))?
            .len()
            > MAX_BACKUP
        {
            return Err(bad("This build supports backup files up to 100 MB."));
        }
        pending
            .as_file()
            .sync_all()
            .map_err(|_| bad("Backup sync failed."))?;
        pending
            .persist_noclobber(&destination)
            .map_err(|_| bad("Backup could not be saved without overwriting another file."))?;
        Ok(destination.to_string_lossy().into())
    }
}
impl WorkspaceRuntime {
    pub fn copies(&self) -> Result<Vec<WorkspaceCopy>, Error> {
        let selection = selected(&self.root)?;
        let mut out = vec![WorkspaceCopy {
            id: "original".into(),
            label: "Original workspace".into(),
            active: self.active == "original",
            selected: selection == "original",
        }];
        let folder = self.root.join("restored");
        if folder.is_dir() {
            for entry in fs::read_dir(folder).map_err(|_| bad("Cannot list restored copies."))? {
                let entry = entry.map_err(|_| bad("Cannot read restored copy."))?;
                let id = entry.file_name().to_string_lossy().to_string();
                if uuid::Uuid::parse_str(&id).is_ok()
                    && !entry
                        .file_type()
                        .map_err(|_| bad("Cannot inspect workspace copy."))?
                        .is_symlink()
                    && entry.path().join("wiki.sqlite").is_file()
                {
                    let label = fs::read_to_string(entry.path().join("label.txt"))
                        .unwrap_or_else(|_| format!("Restored {}", &id[..8]));
                    out.push(WorkspaceCopy {
                        active: self.active == id,
                        selected: selection == id,
                        id,
                        label,
                    });
                }
            }
        }
        out[1..].sort_by(|a, b| a.label.cmp(&b.label));
        Ok(out)
    }
    pub async fn restore(&self, source: &str) -> Result<WorkspaceCopy, Error> {
        let source = user_path(source)?;
        let mut options = fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let file = options
            .open(source)
            .map_err(|_| bad("The backup file is unavailable."))?;
        let meta = file.metadata().map_err(|_| bad("Cannot inspect backup."))?;
        if !meta.is_file() || meta.len() > MAX_BACKUP {
            return Err(bad("Choose a regular backup file no larger than 100 MB."));
        }
        let mut bytes = vec![];
        file.take(MAX_BACKUP + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| bad("Cannot read backup."))?;
        if bytes.len() as u64 > MAX_BACKUP {
            return Err(bad("Backup exceeds 100 MB."));
        }
        let archive: Archive = serde_json::from_slice(&bytes)?;
        if archive.format_version != 1 {
            return Err(bad("This backup format is not supported."));
        }
        let db = STANDARD
            .decode(archive.database)
            .map_err(|_| bad("Backup data is malformed."))?;
        if !db.starts_with(b"SQLite format 3\0")
            || format!("{:x}", Sha256::digest(&db)) != archive.sha256
        {
            return Err(bad("Backup checksum failed. Nothing was restored."));
        }
        let restored = self.root.join("restored");
        fs::create_dir_all(&restored).map_err(|_| bad("Cannot create restore directory."))?;
        let temp = tempfile::tempdir_in(&restored).map_err(|_| bad("Cannot stage restore."))?;
        let path = temp.path().join("wiki.sqlite");
        fs::write(&path, db).map_err(|_| bad("Could not stage the backup database."))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
                .map_err(|_| bad("Cannot protect restored database."))?;
        }
        let count = validate_copy(&path).await?;
        let mut conn =
            SqliteConnection::connect_with(&SqliteConnectOptions::new().filename(&path)).await?;
        sqlx::query("DELETE FROM app_settings")
            .execute(&mut conn)
            .await?;
        let now: String = sqlx::query_scalar("SELECT strftime('%Y-%m-%d %H:%M UTC','now')")
            .fetch_one(&mut conn)
            .await?;
        conn.close().await?;
        let id = uuid::Uuid::new_v4().to_string();
        let label = format!("Restored {now} · {count} pages");
        fs::write(temp.path().join("label.txt"), &label)
            .map_err(|_| bad("Cannot label restored copy."))?;
        fs::rename(temp.path(), restored.join(&id))
            .map_err(|_| bad("Cannot finalize restored copy."))?;
        Ok(WorkspaceCopy {
            id,
            label,
            active: false,
            selected: false,
        })
    }
    pub fn select(&self, id: &str) -> Result<(), Error> {
        copy_id(id)?;
        if !self.copies()?.iter().any(|c| c.id == id) {
            return Err(bad("Workspace copy is unavailable."));
        }
        let mut file = tempfile::NamedTempFile::new_in(&self.root)
            .map_err(|_| bad("Cannot save workspace selection."))?;
        serde_json::to_writer(&mut file, id)?;
        file.flush()
            .map_err(|_| bad("Cannot save workspace selection."))?;
        file.as_file()
            .sync_all()
            .map_err(|_| bad("Cannot save workspace selection."))?;
        file.persist(self.root.join("active-workspace.json"))
            .map_err(|_| bad("Cannot save workspace selection."))?;
        Ok(())
    }
}
