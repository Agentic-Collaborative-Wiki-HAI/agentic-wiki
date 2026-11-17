use crate::{
    domain::*,
    store::{Page, Store},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::Row;
use ts_rs::TS;

pub const MAX_TEXT_BYTES: usize = 1_000_000;
pub(crate) fn invalid(message: impl Into<String>) -> Error {
    Error::Import(message.into())
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct SourceSettings {
    pub local_folder: String,
    pub agent_hands_path: String,
    pub google_account: String,
    pub drive_folder: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "../../src/generated/")]
pub enum SourceProvider {
    Local,
    Drive,
}
impl SourceProvider {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Drive => "drive",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "../../src/generated/")]
pub enum ImportKind {
    Transcript,
    Notes,
}
impl ImportKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Transcript => "transcript",
            Self::Notes => "notes",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct SourceItem {
    pub folder: bool,
    pub key: String,
    pub title: String,
    pub modified: Option<String>,
    pub supported: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct SourceListing {
    pub items: Vec<SourceItem>,
    pub notice: String,
}

#[derive(Clone, Debug)]
pub struct SourceBytes {
    pub provider: SourceProvider,
    pub locator: String,
    pub name: String,
    pub url: Option<String>,
    pub modified: Option<String>,
    pub bytes: Vec<u8>,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct ImportPreview {
    pub token: String,
    pub title: String,
    pub text: String,
    pub provider: SourceProvider,
    pub source_name: String,
    pub source_url: Option<String>,
    pub source_modified: Option<String>,
    pub byte_count: usize,
    pub existing_id: Option<String>,
    pub existing_revision: Option<i32>,
    pub existing_kind: Option<ImportKind>,
    pub unchanged: bool,
}
#[derive(Clone, Debug)]
pub struct PendingImport {
    pub preview: ImportPreview,
    pub source: SourceBytes,
    pub hash: String,
    pub document: RichDocument,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct ImportedSource {
    pub id: String,
    pub title: String,
    pub kind: ImportKind,
    pub parent_id: Option<String>,
    pub provider: SourceProvider,
    pub source_name: String,
    pub source_url: Option<String>,
    pub source_modified: Option<String>,
    pub imported_revision: i32,
    pub imported_at: String,
}

pub fn parse_text(bytes: &[u8]) -> Result<(String, RichDocument), Error> {
    if bytes.len() > MAX_TEXT_BYTES {
        return Err(invalid(
            "Text imports are limited to 1 MB. Split this export into smaller documents.",
        ));
    }
    let raw = std::str::from_utf8(bytes).map_err(|_| {
        invalid("This file is not UTF-8 text. Export the transcript as UTF-8 TXT or Markdown.")
    })?;
    let text = raw
        .trim_start_matches('\u{feff}')
        .replace("\r\n", "\n")
        .replace('\r', "\n");
    if text.trim().is_empty() {
        return Err(invalid("The source is empty. Nothing was imported."));
    }
    if text
        .chars()
        .any(|c| c.is_control() && !matches!(c, '\n' | '\t'))
    {
        return Err(invalid(
            "This source contains binary or unsupported control characters.",
        ));
    }
    let mut blocks = vec![];
    for chunk in text.split("\n\n").filter(|s| !s.trim().is_empty()) {
        // Preserve transcript labels, timestamps and line breaks as source text; never infer who spoke or execute markup.
        let mut content = vec![];
        for (i, line) in chunk.lines().enumerate() {
            if i > 0 {
                content.push(Inline::HardBreak);
            }
            if !line.is_empty() {
                content.push(crate::domain::text(line));
            }
        }
        blocks.push(Block::Paragraph {
            attrs: BlockAttrs {
                block_id: format!("import-{}", blocks.len() + 1),
            },
            content,
        });
    }
    let document = RichDocument {
        schema_version: 1,
        doc: DocumentNode {
            kind: DocumentKind::Doc,
            content: blocks,
        },
    };
    document.validate()?;
    Ok((text, document))
}
impl Store {
    pub async fn source_settings(&self) -> Result<SourceSettings, Error> {
        let value: Option<String> =
            sqlx::query_scalar("SELECT value_json FROM app_settings WHERE key='sources'")
                .fetch_optional(&self.pool)
                .await?;
        value
            .map(|s| serde_json::from_str(&s).map_err(Error::from))
            .unwrap_or(Ok(SourceSettings::default()))
    }
    pub async fn save_source_settings(&self, settings: &SourceSettings) -> Result<(), Error> {
        let settings = crate::sources::validate_settings(settings)?;
        sqlx::query("INSERT INTO app_settings(key,value_json) VALUES('sources',?) ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json")
            .bind(serde_json::to_string(&settings)?).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn preview_import(&self, source: SourceBytes) -> Result<PendingImport, Error> {
        let (text, document) = parse_text(&source.bytes)?;
        let hash = format!("{:x}", Sha256::digest(&source.bytes));
        let existing = sqlx::query("SELECT o.id,o.title,o.current_revision,s.content_kind,i.sha256 FROM imported_sources s JOIN objects o ON o.id=s.object_id JOIN import_snapshots i ON i.object_id=s.object_id WHERE s.provider=? AND s.locator=? ORDER BY i.revision DESC LIMIT 1")
            .bind(source.provider.name()).bind(&source.locator).fetch_optional(&self.pool).await?;
        let preview = ImportPreview {
            token: uuid::Uuid::new_v4().to_string(),
            title: existing
                .as_ref()
                .map(|r| r.get("title"))
                .unwrap_or_else(|| source.name.clone()),
            text,
            provider: source.provider.clone(),
            source_name: source.name.clone(),
            source_url: source.url.clone(),
            source_modified: source.modified.clone(),
            byte_count: source.bytes.len(),
            existing_id: existing.as_ref().map(|r| r.get("id")),
            existing_revision: existing.as_ref().map(|r| r.get("current_revision")),
            existing_kind: existing.as_ref().map(|r| {
                if r.get::<String, _>("content_kind") == "notes" {
                    ImportKind::Notes
                } else {
                    ImportKind::Transcript
                }
            }),
            unchanged: existing
                .as_ref()
                .is_some_and(|r| r.get::<String, _>("sha256") == hash),
        };
        Ok(PendingImport {
            preview,
            source,
            hash,
            document,
        })
    }
    pub async fn commit_import(
        &self,
        pending: &PendingImport,
        title: &str,
        kind: ImportKind,
        parent: Option<&str>,
    ) -> Result<Page, Error> {
        let title = title.trim();
        if title.is_empty() || title.chars().count() > 200 || title.chars().any(char::is_control) {
            return Err(invalid("Enter a title between 1 and 200 characters."));
        }
        pending.document.validate()?;
        let mut tx = self.pool.begin().await?;
        let previous: Option<String> =
            sqlx::query_scalar("SELECT object_id FROM import_requests WHERE id=?")
                .bind(&pending.preview.token)
                .fetch_optional(&mut *tx)
                .await?;
        if let Some(id) = previous {
            tx.commit().await?;
            return self.read(&id).await;
        }
        let existing = sqlx::query("SELECT o.id,o.current_revision,s.content_kind,i.sha256 FROM imported_sources s JOIN objects o ON o.id=s.object_id JOIN import_snapshots i ON i.object_id=s.object_id WHERE s.provider=? AND s.locator=? ORDER BY i.revision DESC LIMIT 1")
            .bind(pending.source.provider.name()).bind(&pending.source.locator).fetch_optional(&mut *tx).await?;
        let (id, revision) = if let Some(row) = existing {
            let id: String = row.get("id");
            if row.get::<String, _>("content_kind") != kind.name() {
                return Err(invalid(
                    "This source already has a different content type. Open its existing copy.",
                ));
            }
            if row.get::<String, _>("sha256") == pending.hash {
                sqlx::query("INSERT INTO import_requests(id,object_id) VALUES(?,?)")
                    .bind(&pending.preview.token)
                    .bind(&id)
                    .execute(&mut *tx)
                    .await?;
                tx.commit().await?;
                return self.read(&id).await;
            }
            let current: i32 = row.get("current_revision");
            if pending.preview.existing_id.as_deref() != Some(&id)
                || pending.preview.existing_revision != Some(current)
            {
                return Err(Error::Stale);
            }
            sqlx::query(
                "UPDATE objects SET title=?,current_revision=current_revision+1,metadata_version=metadata_version+1 WHERE id=?",
            )
            .bind(title)
            .bind(&id)
            .execute(&mut *tx)
            .await?;
            (id, current + 1)
        } else {
            if pending.preview.existing_id.is_some() {
                return Err(Error::Stale);
            }
            if let Some(parent) = parent {
                let exists: bool =
                    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM objects WHERE id=?)")
                        .bind(parent)
                        .fetch_one(&mut *tx)
                        .await?;
                if !exists {
                    return Err(invalid("The selected parent page no longer exists."));
                }
            }
            let id = uuid::Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO objects(id,title,parent_id,kind,current_revision) VALUES(?,?,?,'page',1)").bind(&id).bind(title).bind(parent).execute(&mut *tx).await?;
            sqlx::query("INSERT INTO imported_sources(object_id,provider,locator,content_kind) VALUES(?,?,?,?)").bind(&id).bind(pending.source.provider.name()).bind(&pending.source.locator).bind(kind.name()).execute(&mut *tx).await?;
            (id, 1)
        };
        sqlx::query("INSERT INTO revisions(object_id,revision,document_json,actor_id) VALUES(?,?,?,'local-import')").bind(&id).bind(revision).bind(serde_json::to_string(&pending.document)?).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO import_snapshots(object_id,revision,sha256,original_bytes,source_name,source_url,source_modified) VALUES(?,?,?,?,?,?,?)")
            .bind(&id).bind(revision).bind(&pending.hash).bind(&pending.source.bytes).bind(&pending.source.name).bind(&pending.source.url).bind(&pending.source.modified).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO import_requests(id,object_id) VALUES(?,?)")
            .bind(&pending.preview.token)
            .bind(&id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        self.read(&id).await
    }
    pub async fn imported_sources(&self) -> Result<Vec<ImportedSource>, Error> {
        let rows = sqlx::query("SELECT o.id,o.title,o.parent_id,s.provider,s.content_kind,i.source_name,i.source_url,i.source_modified,i.revision,i.imported_at FROM imported_sources s JOIN objects o ON o.id=s.object_id JOIN import_snapshots i ON i.object_id=o.id WHERE i.revision=(SELECT MAX(j.revision) FROM import_snapshots j WHERE j.object_id=o.id) ORDER BY o.title,o.id").fetch_all(&self.pool).await?;
        Ok(rows
            .into_iter()
            .map(|r| ImportedSource {
                id: r.get("id"),
                title: r.get("title"),
                parent_id: r.get("parent_id"),
                provider: if r.get::<String, _>("provider") == "drive" {
                    SourceProvider::Drive
                } else {
                    SourceProvider::Local
                },
                kind: if r.get::<String, _>("content_kind") == "notes" {
                    ImportKind::Notes
                } else {
                    ImportKind::Transcript
                },
                source_name: r.get("source_name"),
                source_url: r.get("source_url"),
                source_modified: r.get("source_modified"),
                imported_revision: r.get("revision"),
                imported_at: r.get("imported_at"),
            })
            .collect())
    }
}
