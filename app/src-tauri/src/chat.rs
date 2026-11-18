//! Sourced answers and unpublished page drafts. Generation has no authority to publish a wiki page.
use crate::{
    domain::*,
    review::{Evidence, EvidenceSelection, evidence_on, validate_citations},
    search::EvidenceContext,
    store::{Page, Store},
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqliteConnection};
use std::collections::{HashMap, HashSet};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct AnswerParagraph {
    pub text: String,
    pub cited_source_ids: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct ChatAnswer {
    pub title: String,
    pub paragraphs: Vec<AnswerParagraph>,
    pub missing_evidence: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct ChatSource {
    pub id: String,
    pub title: String,
    pub revision: i32,
    pub passage: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct ChatExchange {
    pub question: String,
    pub answer: ChatAnswer,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct ChatRequest {
    pub page_id: String,
    pub page_revision: i32,
    pub page_title: String,
    pub question: String,
    pub sources: Vec<ChatSource>,
    pub history: Vec<ChatExchange>,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct ChatTurn {
    pub id: String,
    pub request: ChatRequest,
    pub evidence: Vec<Evidence>,
    pub answer: Option<ChatAnswer>,
    pub model: String,
    pub status: String,
    pub error: Option<String>,
    pub created_at: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct AnswerDraft {
    pub id: String,
    pub turn_id: String,
    pub title: String,
    pub parent_id: Option<String>,
    pub document: RichDocument,
    pub version: i32,
    pub evidence: Vec<Evidence>,
    pub model: String,
    pub published_page_id: Option<String>,
}
fn bad(s: &str) -> Error {
    Error::Invalid(s.into())
}
impl ChatRequest {
    pub fn validate(&self) -> Result<(), Error> {
        if self.page_id.is_empty()
            || self.page_revision < 1
            || self.question.trim().is_empty()
            || self.question.len() > 4000
            || self.sources.len() > 8
            || self.history.len() > 3
            || serde_json::to_vec(self)?.len() > 32000
        {
            return Err(bad("Narrow the question or select fewer source excerpts."));
        }
        let mut ids = HashSet::new();
        for s in &self.sources {
            if s.id.is_empty()
                || s.revision < 1
                || s.passage.trim().is_empty()
                || !ids.insert(&s.id)
            {
                return Err(bad(
                    "Each source must have a distinct identity and a saved passage.",
                ));
            }
        }
        Ok(())
    }
}
impl ChatAnswer {
    pub fn validate_for(&self, request: &ChatRequest) -> Result<(), Error> {
        request.validate()?;
        crate::navigation::title(&self.title)?;
        if serde_json::to_vec(self)?.len() > 16000
            || self.paragraphs.len() > 30
            || (self.paragraphs.is_empty() && self.missing_evidence.trim().is_empty())
        {
            return Err(bad(
                "The answer must contain supported text or explain the missing evidence.",
            ));
        }
        for p in &self.paragraphs {
            let mut seen = HashSet::new();
            if p.text.trim().is_empty() || p.cited_source_ids.is_empty() {
                return Err(bad(
                    "Every answer paragraph needs a supplied source citation.",
                ));
            }
            for id in &p.cited_source_ids {
                if !seen.insert(id) || !request.sources.iter().any(|s| &s.id == id) {
                    return Err(bad("The answer cites an unavailable or repeated source."));
                }
            }
        }
        // Citation membership is checked here; factual support remains something the reader must assess.
        Ok(())
    }
}
fn turn(row: sqlx::sqlite::SqliteRow) -> Result<ChatTurn, Error> {
    Ok(ChatTurn {
        id: row.get("id"),
        request: serde_json::from_str(row.get("request_json"))?,
        evidence: serde_json::from_str(row.get("evidence_json"))?,
        answer: row
            .get::<Option<String>, _>("answer_json")
            .map(|s| serde_json::from_str(&s))
            .transpose()?,
        model: row.get("model"),
        status: row.get("status"),
        error: row.get("error"),
        created_at: row.get("created_at"),
    })
}
async fn draft_on(conn: &mut SqliteConnection, id: &str) -> Result<AnswerDraft, Error> {
    let row=sqlx::query("SELECT d.*,t.model FROM chat_page_drafts d JOIN chat_turns t ON t.id=d.turn_id WHERE d.id=?").bind(id).fetch_optional(&mut *conn).await?.ok_or(Error::NotFound)?;
    let document: RichDocument = serde_json::from_str(row.get("document_json"))?;
    let mut evidence = vec![];
    let mut seen = HashSet::new();
    for b in &document.doc.content {
        for n in b.content() {
            if let Inline::Citation { attrs } = n {
                if !seen.insert(&attrs.citation_id) {
                    continue;
                }
                let r=sqlx::query("SELECT source_id,source_revision,block_id,quote,range_json FROM citations WHERE id=?").bind(&attrs.citation_id).fetch_one(&mut *conn).await?;
                let source = EvidenceSelection {
                    object_id: r.get("source_id"),
                    revision: r.get("source_revision"),
                    block_id: r.get("block_id"),
                    range: r
                        .get::<Option<String>, _>("range_json")
                        .map(|s| serde_json::from_str(&s))
                        .transpose()?,
                };
                let mut e = evidence_on(conn, &source).await?;
                e.id = attrs.citation_id.clone();
                evidence.push(e);
            }
        }
    }
    Ok(AnswerDraft {
        id: row.get("id"),
        turn_id: row.get("turn_id"),
        title: row.get("title"),
        parent_id: row.get("parent_id"),
        document,
        version: row.get("version"),
        evidence,
        model: row.get("model"),
        published_page_id: row.get("published_page_id"),
    })
}
impl Store {
    pub async fn chat_context(
        &self,
        page_id: &str,
        question: &str,
    ) -> Result<EvidenceContext, Error> {
        let linked = self.default_evidence(page_id, question).await?;
        let mut ids = linked.linked_ids;
        if !ids.iter().any(|id| id == page_id) {
            ids.push(page_id.into());
        }
        let page_ids = vec![page_id.to_owned()];
        let mut hits = self.search_passages(question, Some(&page_ids), 2).await?;
        if hits.is_empty() {
            hits = self.search_passages("", Some(&page_ids), 2).await?;
        }
        let mut evidence = linked.evidence;
        for h in hits {
            let e = self.evidence(&h.source).await?;
            if evidence.len() < 8
                && !evidence.iter().any(|old| old.id == e.id)
                && evidence.iter().map(|e| e.quote.len()).sum::<usize>() + e.quote.len() <= 16000
            {
                evidence.push(e);
            }
        }
        Ok(EvidenceContext{evidence,linked_ids:ids,notice:"Current page and linked sources only. Selected excerpts, not whole documents, are sent. Choose sources to inspect or change them.".into()})
    }
    pub async fn chat_turns(&self) -> Result<Vec<ChatTurn>, Error> {
        sqlx::query("SELECT * FROM (SELECT rowid AS position,* FROM chat_turns ORDER BY rowid DESC LIMIT 50) ORDER BY position").fetch_all(&self.pool).await?.into_iter().map(turn).collect()
    }
    pub async fn chat_turn(&self, id: &str) -> Result<ChatTurn, Error> {
        turn(
            sqlx::query("SELECT * FROM chat_turns WHERE id=?")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?
                .ok_or(Error::NotFound)?,
        )
    }
    pub async fn prepare_chat(
        &self,
        id: &str,
        page_id: &str,
        question: &str,
        sources: &[EvidenceSelection],
        model: &str,
    ) -> Result<ChatTurn, Error> {
        uuid::Uuid::parse_str(id).map_err(|_| bad("Invalid Chat request."))?;
        if sources.len() > 8 || model.trim().is_empty() || model.len() > 200 {
            return Err(bad(
                "Choose up to eight source excerpts and an available model.",
            ));
        }
        let mut tx = self.pool.begin().await?;
        let page = crate::review::page_at(&mut tx, page_id, None).await?;
        let mut evidence = vec![];
        for s in sources {
            evidence.push(evidence_on(&mut tx, s).await?);
        }
        let rows=sqlx::query("SELECT request_json,answer_json FROM chat_turns WHERE status='completed' ORDER BY rowid DESC LIMIT 3").fetch_all(&mut *tx).await?;
        let mut history = vec![];
        for r in rows {
            let old: ChatRequest = serde_json::from_str(r.get("request_json"))?;
            let exchange = ChatExchange {
                question: old.question,
                answer: serde_json::from_str(r.get("answer_json"))?,
            };
            if serde_json::to_vec(&history)?.len() + serde_json::to_vec(&exchange)?.len() <= 8000 {
                history.push(exchange);
            } else {
                break;
            }
        }
        history.reverse();
        let request = ChatRequest {
            page_id: page.id,
            page_revision: page.revision,
            page_title: page.title,
            question: question.trim().into(),
            sources: evidence
                .iter()
                .map(|e| ChatSource {
                    id: e.id.clone(),
                    title: e.title.clone(),
                    revision: e.source.revision,
                    passage: e.quote.clone(),
                })
                .collect(),
            history,
        };
        request.validate()?;
        sqlx::query("INSERT INTO chat_turns(id,page_id,page_revision,request_json,evidence_json,model,status) VALUES(?,?,?,?,?,?,'pending')").bind(id).bind(page_id).bind(page.revision).bind(serde_json::to_string(&request)?).bind(serde_json::to_string(&evidence)?).bind(model).execute(&mut *tx).await?;
        tx.commit().await?;
        self.chat_turn(id).await
    }
    pub async fn complete_chat(&self, id: &str, answer: &ChatAnswer) -> Result<ChatTurn, Error> {
        let t = self.chat_turn(id).await?;
        answer.validate_for(&t.request)?;
        let changed=sqlx::query("UPDATE chat_turns SET answer_json=?,status='completed',error=NULL WHERE id=? AND status='pending'").bind(serde_json::to_string(answer)?).bind(id).execute(&self.pool).await?;
        if changed.rows_affected() != 1 {
            return Err(bad("This Chat request is no longer pending."));
        }
        self.chat_turn(id).await
    }
    pub async fn fail_chat(&self, id: &str, cancelled: bool, error: &str) -> Result<(), Error> {
        sqlx::query("UPDATE chat_turns SET status=?,error=? WHERE id=? AND status='pending'")
            .bind(if cancelled { "cancelled" } else { "failed" })
            .bind(error)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn answer_draft(&self, id: &str) -> Result<AnswerDraft, Error> {
        draft_on(&mut *self.pool.acquire().await?, id).await
    }
    pub async fn answer_drafts(&self) -> Result<Vec<AnswerDraft>, Error> {
        let ids: Vec<String> = sqlx::query_scalar("SELECT id FROM chat_page_drafts ORDER BY rowid")
            .fetch_all(&self.pool)
            .await?;
        let mut drafts = vec![];
        for id in ids {
            drafts.push(self.answer_draft(&id).await?);
        }
        Ok(drafts)
    }
    pub async fn create_answer_draft(&self, turn_id: &str) -> Result<AnswerDraft, Error> {
        let t = self.chat_turn(turn_id).await?;
        let answer = t
            .answer
            .as_ref()
            .filter(|_| t.status == "completed")
            .ok_or_else(|| bad("Wait for a completed answer."))?;
        answer.validate_for(&t.request)?;
        if answer.paragraphs.is_empty() {
            return Err(bad("There is no supported answer to turn into a page yet."));
        }
        let mut tx = self.pool.begin().await?;
        if let Some(id) =
            sqlx::query_scalar::<_, String>("SELECT id FROM chat_page_drafts WHERE turn_id=?")
                .bind(turn_id)
                .fetch_optional(&mut *tx)
                .await?
        {
            let draft = draft_on(&mut tx, &id).await?;
            tx.commit().await?;
            return Ok(draft);
        }
        let mut citations: HashMap<String, String> = HashMap::new();
        let mut blocks = vec![];
        for p in &answer.paragraphs {
            let mut content = vec![text(&p.text)];
            for id in &p.cited_source_ids {
                let citation = if let Some(c) = citations.get(id) {
                    c.clone()
                } else {
                    let e = t
                        .evidence
                        .iter()
                        .find(|e| &e.id == id)
                        .ok_or_else(|| bad("Missing answer evidence."))?;
                    let resolved = evidence_on(&mut tx, &e.source).await?;
                    if resolved.quote != e.quote {
                        return Err(bad("Answer evidence does not match its saved source."));
                    }
                    let c = uuid::Uuid::new_v4().to_string();
                    sqlx::query("INSERT INTO citations(id,source_id,source_revision,block_id,quote,range_json) VALUES(?,?,?,?,?,?)").bind(&c).bind(&e.source.object_id).bind(e.source.revision).bind(&e.source.block_id).bind(&e.quote).bind(e.source.range.as_ref().map(serde_json::to_string).transpose()?).execute(&mut *tx).await?;
                    citations.insert(id.clone(), c.clone());
                    c
                };
                content.push(Inline::Citation {
                    attrs: CitationAttrs {
                        citation_id: citation,
                    },
                });
            }
            blocks.push(Block::Paragraph {
                attrs: BlockAttrs {
                    block_id: uuid::Uuid::new_v4().to_string(),
                },
                content,
            });
        }
        if !answer.missing_evidence.trim().is_empty() {
            blocks.push(heading(&uuid::Uuid::new_v4().to_string(), "Still to check"));
            blocks.push(paragraph(
                &uuid::Uuid::new_v4().to_string(),
                &answer.missing_evidence,
            ));
        }
        let document = RichDocument {
            schema_version: 1,
            doc: DocumentNode {
                kind: DocumentKind::Doc,
                content: blocks,
            },
        };
        document.validate()?;
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO chat_page_drafts(id,turn_id,title,parent_id,document_json) VALUES(?,?,?,?,?)").bind(&id).bind(turn_id).bind(&answer.title).bind(&t.request.page_id).bind(serde_json::to_string(&document)?).execute(&mut *tx).await?;
        let draft = draft_on(&mut tx, &id).await?;
        tx.commit().await?;
        Ok(draft)
    }
    pub async fn save_answer_draft(
        &self,
        id: &str,
        version: i32,
        title: &str,
        parent: Option<&str>,
        document: &RichDocument,
    ) -> Result<AnswerDraft, Error> {
        let title = crate::navigation::title(title)?;
        document.validate()?;
        let mut tx = self.pool.begin().await?;
        crate::navigation::parent_valid(&mut tx, id, parent).await?;
        validate_citations(&mut tx, document).await?;
        let n=sqlx::query("UPDATE chat_page_drafts SET title=?,parent_id=?,document_json=?,version=version+1 WHERE id=? AND version=? AND published_page_id IS NULL").bind(title).bind(parent).bind(serde_json::to_string(document)?).bind(id).bind(version).execute(&mut *tx).await?;
        if n.rows_affected() != 1 {
            return Err(Error::Stale);
        }
        let draft = draft_on(&mut tx, id).await?;
        tx.commit().await?;
        Ok(draft)
    }
    pub async fn publish_answer_draft(&self, id: &str, version: i32) -> Result<Page, Error> {
        let mut tx = self.pool.begin().await?;
        let draft = draft_on(&mut tx, id).await?;
        if let Some(published) = draft.published_page_id {
            tx.commit().await?;
            return self.read(&published).await;
        }
        if draft.version != version {
            return Err(Error::Stale);
        }
        crate::navigation::title(&draft.title)?;
        draft.document.validate()?;
        crate::navigation::parent_valid(&mut tx, id, draft.parent_id.as_deref()).await?;
        validate_citations(&mut tx, &draft.document).await?;
        sqlx::query(
            "INSERT INTO objects(id,title,parent_id,kind,current_revision) VALUES(?,?,?,'page',1)",
        )
        .bind(id)
        .bind(&draft.title)
        .bind(&draft.parent_id)
        .execute(&mut *tx)
        .await?;
        sqlx::query("INSERT INTO revisions(object_id,revision,document_json,actor_id) VALUES(?,1,?,'local-operator')").bind(id).bind(serde_json::to_string(&draft.document)?).execute(&mut *tx).await?;
        sqlx::query("UPDATE chat_page_drafts SET published_page_id=? WHERE id=?")
            .bind(id)
            .bind(id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        self.read(id).await
    }
    pub async fn chat_page_origin(&self, id: &str) -> Result<Option<ChatTurn>, Error> {
        let row=sqlx::query("SELECT t.* FROM chat_turns t JOIN chat_page_drafts d ON t.id=d.turn_id WHERE d.published_page_id=?").bind(id).fetch_optional(&self.pool).await?;
        row.map(turn).transpose()
    }
}

pub(crate) async fn validate_chat_records(conn: &mut SqliteConnection) -> Result<(), Error> {
    let rows = sqlx::query("SELECT * FROM chat_turns")
        .fetch_all(&mut *conn)
        .await?;
    for row in rows {
        let page_id: String = row.get("page_id");
        let page_revision: i32 = row.get("page_revision");
        let t = turn(row)?;
        t.request.validate()?;
        if t.request.page_id != page_id
            || t.request.page_revision != page_revision
            || t.evidence.len() != t.request.sources.len()
        {
            return Err(bad("Backup Chat context does not match its record."));
        }
        for (e, s) in t.evidence.iter().zip(&t.request.sources) {
            let resolved = evidence_on(conn, &e.source).await?;
            if resolved.id != e.id
                || resolved.quote != e.quote
                || s.id != e.id
                || s.passage != e.quote
                || s.title != e.title
                || s.revision != e.source.revision
            {
                return Err(bad("Backup contains inconsistent Chat evidence."));
            }
        }
        if let Some(a) = &t.answer {
            if t.status != "completed" {
                return Err(bad("Backup contains an unfinished Chat answer."));
            }
            a.validate_for(&t.request)?;
        } else if t.status == "completed" {
            return Err(bad("Backup is missing a completed Chat answer."));
        }
    }
    let rows = sqlx::query(
        "SELECT id,title,version,turn_id,published_page_id,document_json FROM chat_page_drafts",
    )
    .fetch_all(&mut *conn)
    .await?;
    for r in rows {
        crate::navigation::title(r.get("title"))?;
        if r.get::<i32, _>("version") < 1 {
            return Err(bad("Backup contains an invalid draft version."));
        }
        let origin: String = sqlx::query_scalar("SELECT status FROM chat_turns WHERE id=?")
            .bind(r.get::<String, _>("turn_id"))
            .fetch_one(&mut *conn)
            .await?;
        if origin != "completed" {
            return Err(bad("Backup draft has no completed answer."));
        }
        if let Some(published) = r.get::<Option<String>, _>("published_page_id") {
            if published != r.get::<String, _>("id") {
                return Err(bad("Backup draft has an inconsistent published identity."));
            }
            let original: String = sqlx::query_scalar(
                "SELECT document_json FROM revisions WHERE object_id=? AND revision=1",
            )
            .bind(published)
            .fetch_one(&mut *conn)
            .await?;
            if original != r.get::<String, _>("document_json") {
                return Err(bad("Backup draft does not match its original publication."));
            }
        }
    }
    Ok(())
}
