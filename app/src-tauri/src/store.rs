use crate::domain::*;
use serde::{Deserialize, Serialize};
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{path::Path, time::Duration};
use ts_rs::TS;

#[derive(Clone)]
pub struct Store {
    pub(crate) pool: SqlitePool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct Page {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub revision: i32,
    pub document: RichDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct Draft {
    pub base_revision: i32,
    pub document: RichDocument,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct HistoryEntry {
    pub revision: i32,
    pub actor_id: String,
    pub created_at: String,
}

impl Store {
    pub async fn open(path: &Path) -> Result<Self, Error> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .foreign_keys(true)
            .busy_timeout(Duration::from_secs(5));
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| Error::Invalid(format!("Migration failed: {e}")))?;
        let store = Self { pool };
        store.seed().await?;
        Ok(store)
    }
    async fn seed(&self) -> Result<(), Error> {
        let guide = RichDocument {
            schema_version: 1,
            doc: DocumentNode {
                kind: DocumentKind::Doc,
                content: vec![
                    heading("overview-heading", "Overview"),
                    paragraph(
                        "overview",
                        "Use this page to find the microscope instructions, arrange training and prepare for your first session.",
                    ),
                    heading("before-heading", "Before your first session"),
                    paragraph(
                        "booking",
                        "Reserve the microscope using the shared spreadsheet before your first session.",
                    ),
                    heading("help-heading", "Getting help"),
                    paragraph(
                        "help",
                        "For hands-on training, contact the equipment lead listed on the equipment page.",
                    ),
                ],
            },
        };
        let source = RichDocument {
            schema_version: 1,
            doc: DocumentNode {
                kind: DocumentKind::Doc,
                content: vec![
                    heading("transcript", "Transcript"),
                    paragraph(
                        "passage-18",
                        "The equipment page links to the current guide and the booking calendar. Check those before planning a session.",
                    ),
                ],
            },
        };
        let mut tx = self.pool.begin().await?;
        for (id, title, kind, document) in [
            (
                "getting-started",
                "Getting started with the microscope",
                "page",
                guide,
            ),
            (
                "orientation-recording",
                "Microscope orientation recording",
                "recording",
                source,
            ),
        ] {
            let result = sqlx::query(
                "INSERT OR IGNORE INTO objects(id,title,kind,parent_id,current_revision) VALUES(?,?,?,?,1)",
            )
            .bind(id)
            .bind(title)
            .bind(kind)
            .bind(if kind == "recording" { Some("getting-started") } else { None })
            .execute(&mut *tx)
            .await?;
            if result.rows_affected() == 1 {
                sqlx::query("INSERT INTO revisions(object_id,revision,document_json,actor_id) VALUES(?,1,?,'example-seed')").bind(id).bind(serde_json::to_string(&document)?).execute(&mut *tx).await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }
    pub async fn read(&self, id: &str) -> Result<Page, Error> {
        let row=sqlx::query("SELECT o.id,o.title,o.kind,o.current_revision,r.document_json FROM objects o JOIN revisions r ON r.object_id=o.id AND r.revision=o.current_revision WHERE o.id=?").bind(id).fetch_optional(&self.pool).await?.ok_or(Error::NotFound)?;
        let document: RichDocument = serde_json::from_str(row.get("document_json"))?;
        document.validate()?;
        Ok(Page {
            id: row.get("id"),
            title: row.get("title"),
            kind: row.get("kind"),
            revision: row.get("current_revision"),
            document,
        })
    }
    pub async fn save(&self, id: &str, base: i32, document: &RichDocument) -> Result<Page, Error> {
        document.validate()?;
        let encoded = serde_json::to_string(document)?;
        let mut tx = self.pool.begin().await?;
        crate::review::validate_citations(&mut tx, document).await?;
        let changed=sqlx::query("UPDATE objects SET current_revision=current_revision+1 WHERE id=? AND current_revision=?").bind(id).bind(base).execute(&mut *tx).await?;
        if changed.rows_affected() != 1 {
            return Err(Error::Stale);
        }
        sqlx::query("INSERT INTO revisions(object_id,revision,document_json,actor_id) VALUES(?,?,?,'local-operator')").bind(id).bind(base+1).bind(encoded).execute(&mut *tx).await?;
        sqlx::query("DELETE FROM drafts WHERE object_id=?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        self.read(id).await
    }
    pub async fn put_draft(
        &self,
        id: &str,
        base: i32,
        document: &RichDocument,
    ) -> Result<(), Error> {
        document.validate()?;
        let mut tx = self.pool.begin().await?;
        crate::review::validate_citations(&mut tx, document).await?;
        let current: Option<i32> =
            sqlx::query_scalar("SELECT current_revision FROM objects WHERE id=?")
                .bind(id)
                .fetch_optional(&mut *tx)
                .await?;
        if current != Some(base) {
            return Err(Error::Stale);
        }
        sqlx::query("INSERT INTO drafts(object_id,base_revision,document_json) VALUES(?,?,?) ON CONFLICT(object_id) DO UPDATE SET base_revision=excluded.base_revision,document_json=excluded.document_json").bind(id).bind(base).bind(serde_json::to_string(document)?).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
    pub async fn draft(&self, id: &str) -> Result<Option<Draft>, Error> {
        let row = sqlx::query("SELECT base_revision,document_json FROM drafts WHERE object_id=?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        row.map(|r| {
            let document: RichDocument = serde_json::from_str(r.get("document_json"))?;
            document.validate()?;
            Ok(Draft {
                base_revision: r.get("base_revision"),
                document,
            })
        })
        .transpose()
    }
    pub async fn discard_draft(&self, id: &str) -> Result<(), Error> {
        sqlx::query("DELETE FROM drafts WHERE object_id=?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn history(&self, id: &str) -> Result<Vec<HistoryEntry>, Error> {
        Ok(sqlx::query("SELECT revision,actor_id,created_at FROM revisions WHERE object_id=? ORDER BY revision DESC").bind(id).fetch_all(&self.pool).await?.into_iter().map(|r|HistoryEntry{revision:r.get("revision"),actor_id:r.get("actor_id"),created_at:r.get("created_at")}).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn saves_reopens_and_rejects_stale_or_invalid_changes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wiki.sqlite");
        let s = Store::open(&path).await.unwrap();
        let first = s.read("getting-started").await.unwrap();
        let mut changed = first.document.clone();
        changed
            .doc
            .content
            .push(paragraph("new", "A persisted paragraph."));
        s.put_draft(&first.id, first.revision, &changed)
            .await
            .unwrap();
        assert!(s.draft(&first.id).await.unwrap().is_some());
        let saved = s.save(&first.id, first.revision, &changed).await.unwrap();
        assert_eq!(saved.revision, 2);
        assert!(s.draft(&first.id).await.unwrap().is_none());
        assert!(matches!(
            s.save(&first.id, 1, &first.document).await,
            Err(Error::Stale)
        ));
        let mut invalid = changed.clone();
        invalid.schema_version = 44;
        assert!(s.save(&first.id, 2, &invalid).await.is_err());
        assert_eq!(s.history(&first.id).await.unwrap().len(), 2);
        s.pool.close().await;
        let reopened = Store::open(&path).await.unwrap();
        assert_eq!(reopened.read(&first.id).await.unwrap().document, changed);
    }
    #[tokio::test]
    async fn failed_revision_insert_rolls_back_head_and_keeps_draft() {
        let dir = tempfile::tempdir().unwrap();
        let s = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
        let page = s.read("getting-started").await.unwrap();
        s.put_draft(&page.id, 1, &page.document).await.unwrap();
        sqlx::query("CREATE TRIGGER test_fail BEFORE INSERT ON revisions WHEN NEW.revision=2 BEGIN SELECT RAISE(ABORT,'injected write failure'); END;").execute(&s.pool).await.unwrap();
        assert!(s.save(&page.id, 1, &page.document).await.is_err());
        assert_eq!(s.read(&page.id).await.unwrap().revision, 1);
        assert!(s.draft(&page.id).await.unwrap().is_some());
    }
}
