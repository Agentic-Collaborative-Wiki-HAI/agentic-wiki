use crate::{
    domain::*,
    store::{Page, Store},
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqliteConnection};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct WikiRecord {
    pub id: String,
    pub title: String,
    pub parent_id: Option<String>,
    pub kind: String,
    pub revision: i32,
    pub metadata_version: i32,
    pub imported: bool,
}
pub(crate) fn title(value: &str) -> Result<&str, Error> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > 200 || value.chars().any(char::is_control) {
        return Err(Error::Invalid(
            "Enter a title between 1 and 200 characters.".into(),
        ));
    }
    Ok(value)
}
pub(crate) async fn parent_valid(
    conn: &mut SqliteConnection,
    id: &str,
    parent: Option<&str>,
) -> Result<(), Error> {
    let mut next = parent.map(str::to_owned);
    let mut seen = std::collections::HashSet::new();
    while let Some(p) = next {
        if p == id || !seen.insert(p.clone()) {
            return Err(Error::Invalid(
                "A page cannot be moved inside itself or its descendants.".into(),
            ));
        }
        let row = sqlx::query("SELECT parent_id FROM objects WHERE id=?")
            .bind(p)
            .fetch_optional(&mut *conn)
            .await?
            .ok_or(Error::NotFound)?;
        next = row.get("parent_id");
    }
    Ok(())
}
impl Store {
    pub async fn records(&self) -> Result<Vec<WikiRecord>, Error> {
        Ok(sqlx::query("SELECT o.*,EXISTS(SELECT 1 FROM imported_sources s WHERE s.object_id=o.id) AS imported FROM objects o ORDER BY lower(title),id").fetch_all(&self.pool).await?.into_iter().map(|r|WikiRecord{id:r.get("id"),title:r.get("title"),parent_id:r.get("parent_id"),kind:r.get("kind"),revision:r.get("current_revision"),metadata_version:r.get("metadata_version"),imported:r.get::<i32,_>("imported")==1}).collect())
    }
    pub async fn create_page(
        &self,
        id: &str,
        name: &str,
        parent: Option<&str>,
    ) -> Result<Page, Error> {
        uuid::Uuid::parse_str(id)
            .map_err(|_| Error::Invalid("Invalid creation request.".into()))?;
        let name = title(name)?;
        let mut tx = self.pool.begin().await?;
        if let Some(row) = sqlx::query("SELECT title,parent_id FROM objects WHERE id=?")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?
        {
            if row.get::<String, _>("title") != name
                || row.get::<Option<String>, _>("parent_id").as_deref() != parent
            {
                return Err(Error::Stale);
            }
            tx.commit().await?;
            return self.read(id).await;
        }
        parent_valid(&mut tx, id, parent).await?;
        let mut empty = paragraph(&uuid::Uuid::new_v4().to_string(), "");
        if let Block::Paragraph { content, .. } = &mut empty {
            content.clear();
        }
        let document = RichDocument {
            schema_version: 1,
            doc: DocumentNode {
                kind: DocumentKind::Doc,
                content: vec![empty],
            },
        };
        document.validate()?;
        sqlx::query(
            "INSERT INTO objects(id,title,parent_id,kind,current_revision) VALUES(?,?,?,'page',1)",
        )
        .bind(id)
        .bind(name)
        .bind(parent)
        .execute(&mut *tx)
        .await?;
        sqlx::query("INSERT INTO revisions(object_id,revision,document_json,actor_id) VALUES(?,1,?,'local-operator')").bind(id).bind(serde_json::to_string(&document)?).execute(&mut *tx).await?;
        tx.commit().await?;
        self.read(id).await
    }
    pub async fn organize_page(
        &self,
        id: &str,
        version: i32,
        name: &str,
        parent: Option<&str>,
    ) -> Result<Page, Error> {
        let name = title(name)?;
        let mut tx = self.pool.begin().await?;
        parent_valid(&mut tx, id, parent).await?;
        let row = sqlx::query("SELECT title,parent_id,metadata_version FROM objects WHERE id=?")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(Error::NotFound)?;
        if row.get::<i32, _>("metadata_version") != version {
            return Err(Error::Stale);
        }
        if row.get::<String, _>("title") != name
            || row.get::<Option<String>, _>("parent_id").as_deref() != parent
        {
            sqlx::query("UPDATE objects SET title=?,parent_id=?,metadata_version=metadata_version+1 WHERE id=?").bind(name).bind(parent).bind(id).execute(&mut *tx).await?;
            sqlx::query("INSERT INTO object_events(id,object_id,title,parent_id,metadata_version) VALUES(?,?,?,?,?)").bind(uuid::Uuid::new_v4().to_string()).bind(id).bind(name).bind(parent).bind(version+1).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        self.read(id).await
    }
}
