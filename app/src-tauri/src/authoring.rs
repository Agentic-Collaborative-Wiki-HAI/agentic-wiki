use crate::{
    domain::*,
    review::{Evidence, EvidenceSelection, evidence_on},
    store::Store,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use ts_rs::TS;
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct ReferenceEntry {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub body: String,
    pub evidence: Option<Evidence>,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct AgentInstructions {
    pub proposal: String,
    pub chat: String,
}
pub const PROPOSAL_DEFAULT: &str = "Help with the selected wiki passage according to the person's message. Messages may ask for a rewrite, correction, shorter wording, a tone change or a source-based update. Preserve meaning for editorial requests; no external citation is required for rewriting the supplied text. Use supplied excerpts when making factual changes and explain any limits or conflicts. Explain the suggested edit briefly in plain language. Evidence stays with the discussion; it does not automatically become an article citation.";
pub const CHAT_DEFAULT: &str = "Help with the person's wiki message, including questions, editing instructions, planning and discussion. Be concise and use plain language. Cite supplied excerpts for factual claims drawn from them. A transformation of supplied text or an ordinary conversational response need not have citations. Explain missing evidence when the supplied context cannot support a factual answer. Do not invent facts or pretend to have changed the wiki.";
impl Default for AgentInstructions {
    fn default() -> Self {
        Self {
            proposal: PROPOSAL_DEFAULT.into(),
            chat: CHAT_DEFAULT.into(),
        }
    }
}
impl Store {
    pub async fn instructions(&self) -> Result<AgentInstructions, Error> {
        let r = sqlx::query("SELECT proposal,chat FROM agent_instructions WHERE id=1")
            .fetch_optional(&self.pool)
            .await?;
        Ok(r.map(|r| AgentInstructions {
            proposal: r.get("proposal"),
            chat: r.get("chat"),
        })
        .unwrap_or_default())
    }
    pub async fn save_instructions(&self, value: &AgentInstructions) -> Result<(), Error> {
        if [&value.proposal, &value.chat]
            .iter()
            .any(|s| s.trim().is_empty() || s.len() > 12000)
        {
            return Err(Error::Invalid(
                "Instructions must contain 1–12000 bytes each.".into(),
            ));
        }
        sqlx::query("INSERT INTO agent_instructions(id,proposal,chat) VALUES(1,?,?) ON CONFLICT(id) DO UPDATE SET proposal=excluded.proposal,chat=excluded.chat").bind(&value.proposal).bind(&value.chat).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn register_citation(
        &self,
        source: &EvidenceSelection,
    ) -> Result<ReferenceEntry, Error> {
        let mut tx = self.pool.begin().await?;
        let mut e = evidence_on(&mut tx, source).await?;
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO citations(id,source_id,source_revision,block_id,quote,range_json) VALUES(?,?,?,?,?,?)").bind(&id).bind(&source.object_id).bind(source.revision).bind(&source.block_id).bind(&e.quote).bind(source.range.as_ref().map(serde_json::to_string).transpose()?).execute(&mut *tx).await?;
        e.id = id.clone();
        tx.commit().await?;
        Ok(ReferenceEntry {
            id,
            kind: "citation".into(),
            title: e.title.clone(),
            body: e.quote.clone(),
            evidence: Some(e),
        })
    }
    pub async fn register_note(&self, title: &str, body: &str) -> Result<ReferenceEntry, Error> {
        let title = crate::navigation::title(title)?;
        if body.trim().is_empty() || body.len() > 16000 {
            return Err(Error::Invalid(
                "Write a margin note of at most 16000 bytes.".into(),
            ));
        }
        let id = uuid::Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO margin_notes(id,title,body) VALUES(?,?,?)")
            .bind(&id)
            .bind(title)
            .bind(body.trim())
            .execute(&self.pool)
            .await?;
        Ok(ReferenceEntry {
            id,
            kind: "note".into(),
            title: title.into(),
            body: body.trim().into(),
            evidence: None,
        })
    }
    pub async fn references_for(
        &self,
        document: &RichDocument,
    ) -> Result<Vec<ReferenceEntry>, Error> {
        document.validate()?;
        let mut tx = self.pool.begin().await?;
        crate::review::validate_citations(&mut tx, document).await?;
        let mut entries: Vec<ReferenceEntry> = vec![];
        for b in &document.doc.content {
            for node in b.content() {
                match node {
                    Inline::Note { attrs } => {
                        if entries.iter().any(|e| e.id == attrs.note_id) {
                            continue;
                        }
                        let r = sqlx::query("SELECT title,body FROM margin_notes WHERE id=?")
                            .bind(&attrs.note_id)
                            .fetch_one(&mut *tx)
                            .await?;
                        entries.push(ReferenceEntry {
                            id: attrs.note_id.clone(),
                            kind: "note".into(),
                            title: r.get("title"),
                            body: r.get("body"),
                            evidence: None,
                        });
                    }
                    Inline::Citation { attrs } => {
                        if entries.iter().any(|e| e.id == attrs.citation_id) {
                            continue;
                        }
                        let r = sqlx::query("SELECT * FROM citations WHERE id=?")
                            .bind(&attrs.citation_id)
                            .fetch_one(&mut *tx)
                            .await?;
                        let source = EvidenceSelection {
                            object_id: r.get("source_id"),
                            revision: r.get("source_revision"),
                            block_id: r.get("block_id"),
                            range: r
                                .get::<Option<String>, _>("range_json")
                                .map(|s| serde_json::from_str(&s))
                                .transpose()?,
                        };
                        let mut e = evidence_on(&mut tx, &source).await?;
                        e.id = attrs.citation_id.clone();
                        entries.push(ReferenceEntry {
                            id: e.id.clone(),
                            kind: "citation".into(),
                            title: e.title.clone(),
                            body: e.quote.clone(),
                            evidence: Some(e),
                        });
                    }
                    _ => {}
                }
            }
        }
        tx.commit().await?;
        Ok(entries)
    }
}

#[cfg(test)]
#[path = "authoring_tests.rs"]
mod tests;
