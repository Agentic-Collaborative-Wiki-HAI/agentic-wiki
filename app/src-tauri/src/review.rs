//! Persistent discussion and review. Only an explicit local decision can publish a candidate.
use crate::{
    agent::{PreviousProposalContext, ProposalCandidate, ProposalRequest, SourceExcerpt},
    domain::*,
    store::{Page, Store},
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqliteConnection};
use ts_rs::TS;
use uuid::Uuid;

fn id() -> String {
    Uuid::new_v4().to_string()
}
fn invalid(message: &str) -> Error {
    Error::Invalid(message.into())
}
fn bounded(value: &str) -> Result<(), Error> {
    if value.trim().is_empty() || value.len() > 16_000 {
        Err(invalid("Enter between 1 and 16,000 bytes of text."))
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct EvidenceRange {
    pub from_utf16: usize,
    pub to_utf16: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct EvidenceSelection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub range: Option<EvidenceRange>,
    pub object_id: String,
    pub revision: i32,
    pub block_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct Evidence {
    pub id: String,
    pub source: EvidenceSelection,
    pub title: String,
    pub quote: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct Message {
    pub id: String,
    pub actor_id: String,
    pub body: String,
    pub created_at: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct Proposal {
    pub id: String,
    pub thread_id: String,
    pub parent_id: Option<String>,
    pub base_revision: i32,
    pub anchor: PassageAnchor,
    pub candidate: ProposalCandidate,
    pub evidence: Vec<Evidence>,
    pub request_text: String,
    pub origin: String,
    pub status: String,
    pub stale: bool,
    pub published_revision: Option<i32>,
    pub decided_at: Option<String>,
    pub created_at: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct Discussion {
    pub id: String,
    pub page_id: String,
    pub base_revision: i32,
    pub anchor: PassageAnchor,
    pub resolved: bool,
    pub messages: Vec<Message>,
    pub proposals: Vec<Proposal>,
}

pub(crate) struct CandidateSubmission<'a> {
    pub thread_id: &'a str,
    pub request: &'a ProposalRequest,
    pub anchor: &'a PassageAnchor,
    pub evidence: &'a [Evidence],
    pub candidate: &'a ProposalCandidate,
    pub parent_id: Option<&'a str>,
    pub origin: &'a str,
}

pub(crate) async fn page_at(
    conn: &mut SqliteConnection,
    page_id: &str,
    revision: Option<i32>,
) -> Result<Page, Error> {
    let row = sqlx::query("SELECT o.id,o.title,o.kind,r.revision,r.document_json FROM objects o JOIN revisions r ON o.id=r.object_id WHERE o.id=? AND r.revision=COALESCE(?,o.current_revision)")
        .bind(page_id).bind(revision).fetch_optional(conn).await?.ok_or(Error::NotFound)?;
    let document: RichDocument = serde_json::from_str(row.get("document_json"))?;
    document.validate()?;
    Ok(Page {
        id: row.get("id"),
        title: row.get("title"),
        kind: row.get("kind"),
        revision: row.get("revision"),
        document,
    })
}
pub(crate) fn check_anchor(page: &Page, anchor: &PassageAnchor) -> Result<(), Error> {
    if anchor.state != AnchorState::Attached || anchor.from_utf16 >= anchor.to_utf16 {
        return Err(invalid("Select a passage in one paragraph."));
    }
    page.document.replace_passage(&PassagePatch {
        block_id: anchor.block_id.clone(),
        from_utf16: anchor.from_utf16,
        to_utf16: anchor.to_utf16,
        expected_quote: anchor.quote.clone(),
        replacement: vec![],
    })?;
    Ok(())
}
pub(crate) async fn evidence_on(
    conn: &mut SqliteConnection,
    selection: &EvidenceSelection,
) -> Result<Evidence, Error> {
    let page = page_at(conn, &selection.object_id, Some(selection.revision)).await?;
    let block = page
        .document
        .doc
        .content
        .iter()
        .find(|b| b.id() == selection.block_id)
        .ok_or(Error::NotFound)?;
    let quote = block
        .content()
        .iter()
        .map(Inline::plain)
        .collect::<String>();
    let (quote, suffix) = if let Some(range) = &selection.range {
        if range.from_utf16 >= range.to_utf16 {
            return Err(invalid("Choose a nonempty source passage."));
        }
        let start = utf16_boundary(&quote, range.from_utf16)?;
        let end = utf16_boundary(&quote, range.to_utf16)?;
        (
            quote[start..end].to_owned(),
            format!(":{}-{}", range.from_utf16, range.to_utf16),
        )
    } else {
        (quote, String::new())
    };
    bounded(&quote)?;
    Ok(Evidence {
        id: format!("{}:{}:{}{}", page.id, page.revision, block.id(), suffix),
        source: selection.clone(),
        title: page.title,
        quote,
    })
}
async fn discussion_on(conn: &mut SqliteConnection, thread_id: &str) -> Result<Discussion, Error> {
    let row = sqlx::query("SELECT * FROM threads WHERE id=?")
        .bind(thread_id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or(Error::NotFound)?;
    let page_id: String = row.get("page_id");
    let current = page_at(conn, &page_id, None).await?;
    let messages = sqlx::query("SELECT * FROM messages WHERE thread_id=? ORDER BY rowid")
        .bind(thread_id)
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|r| Message {
            id: r.get("id"),
            actor_id: r.get("actor_id"),
            body: r.get("body"),
            created_at: r.get("created_at"),
        })
        .collect();
    let mut proposals = vec![];
    for r in sqlx::query("SELECT p.*,d.published_revision,d.created_at AS decided_at FROM proposals p LEFT JOIN review_decisions d ON d.proposal_id=p.id WHERE thread_id=? ORDER BY p.rowid").bind(thread_id).fetch_all(&mut *conn).await? {
        let base:i32=r.get("base_revision");
        proposals.push(Proposal{id:r.get("id"),thread_id:thread_id.into(),parent_id:r.get("parent_id"),base_revision:base,anchor:serde_json::from_str(r.get("anchor_json"))?,candidate:serde_json::from_str(r.get("candidate_json"))?,evidence:serde_json::from_str(r.get("evidence_json"))?,request_text:r.get("request_text"),origin:r.get("origin"),status:r.get("status"),stale:base!=current.revision,published_revision:r.get("published_revision"),decided_at:r.get("decided_at"),created_at:r.get("created_at")});
    }
    let mut anchor: PassageAnchor = serde_json::from_str(row.get("anchor_json"))?;
    // Keep the original quote and revision inspectable when the current passage no longer matches.
    if check_anchor(&current, &anchor).is_err() {
        anchor.state = AnchorState::Detached;
    }
    Ok(Discussion {
        id: thread_id.into(),
        page_id,
        base_revision: row.get("base_revision"),
        anchor,
        resolved: row.get::<i32, _>("resolved") == 1,
        messages,
        proposals,
    })
}
pub(crate) async fn validate_citations(
    conn: &mut SqliteConnection,
    document: &RichDocument,
) -> Result<(), Error> {
    for block in &document.doc.content {
        for inline in block.content() {
            if let Inline::Citation { attrs } = inline {
                let exists: bool =
                    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM citations WHERE id=?)")
                        .bind(&attrs.citation_id)
                        .fetch_one(&mut *conn)
                        .await?;
                if !exists {
                    return Err(invalid("The document contains an unknown citation."));
                }
            }
        }
    }
    Ok(())
}
impl Store {
    pub async fn revision(&self, page_id: &str, revision: i32) -> Result<Page, Error> {
        page_at(&mut *self.pool.acquire().await?, page_id, Some(revision)).await
    }
    pub async fn evidence(&self, selection: &EvidenceSelection) -> Result<Evidence, Error> {
        evidence_on(&mut *self.pool.acquire().await?, selection).await
    }
    /// Bounded context from this page's citations and directly attached recording or imported text sources. No workspace-wide retrieval.
    pub async fn discussion_evidence(&self, page_id: &str) -> Result<Vec<Evidence>, Error> {
        Ok(self.default_evidence(page_id, "").await?.evidence)
    }

    pub async fn discussions(&self, page_id: &str) -> Result<Vec<Discussion>, Error> {
        let mut conn = self.pool.acquire().await?;
        let ids: Vec<String> =
            sqlx::query_scalar("SELECT id FROM threads WHERE page_id=? ORDER BY rowid")
                .bind(page_id)
                .fetch_all(&mut *conn)
                .await?;
        let mut out = vec![];
        for id in ids {
            out.push(discussion_on(&mut conn, &id).await?);
        }
        Ok(out)
    }
    pub async fn discussion(&self, thread_id: &str) -> Result<Discussion, Error> {
        discussion_on(&mut *self.pool.acquire().await?, thread_id).await
    }
    pub async fn create_discussion(
        &self,
        page_id: &str,
        base: i32,
        anchor: &PassageAnchor,
        body: &str,
    ) -> Result<Discussion, Error> {
        bounded(body)?;
        let mut tx = self.pool.begin().await?;
        let page = page_at(&mut tx, page_id, None).await?;
        if page.revision != base {
            return Err(Error::Stale);
        }
        check_anchor(&page, anchor)?;
        let thread_id = id();
        sqlx::query("INSERT INTO threads(id,page_id,base_revision,anchor_json) VALUES(?,?,?,?)")
            .bind(&thread_id)
            .bind(page_id)
            .bind(base)
            .bind(serde_json::to_string(anchor)?)
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "INSERT INTO messages(id,thread_id,actor_id,body) VALUES(?,?,'local-operator',?)",
        )
        .bind(id())
        .bind(&thread_id)
        .bind(body.trim())
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        self.discussion(&thread_id).await
    }
    pub async fn reply(&self, thread_id: &str, body: &str) -> Result<Discussion, Error> {
        bounded(body)?;
        sqlx::query(
            "INSERT INTO messages(id,thread_id,actor_id,body) VALUES(?,?,'local-operator',?)",
        )
        .bind(id())
        .bind(thread_id)
        .bind(body.trim())
        .execute(&self.pool)
        .await?;
        self.discussion(thread_id).await
    }
    pub async fn resolve_discussion(
        &self,
        thread_id: &str,
        resolved: bool,
    ) -> Result<Discussion, Error> {
        sqlx::query("UPDATE threads SET resolved=? WHERE id=?")
            .bind(resolved)
            .bind(thread_id)
            .execute(&self.pool)
            .await?;
        self.discussion(thread_id).await
    }
    pub async fn proposal_context(
        &self,
        thread_id: &str,
        request_text: &str,
        selections: &[EvidenceSelection],
        anchor: &PassageAnchor,
        base: i32,
        parent_id: Option<&str>,
    ) -> Result<(ProposalRequest, Vec<Evidence>), Error> {
        bounded(request_text)?;
        if selections.is_empty() || selections.len() > 8 {
            return Err(invalid("Choose between one and eight source passages."));
        }
        let mut conn = self.pool.acquire().await?;
        let discussion = discussion_on(&mut conn, thread_id).await?;
        let page = page_at(&mut conn, &discussion.page_id, None).await?;
        if page.revision != base {
            return Err(Error::Stale);
        }
        check_anchor(&page, anchor)?;
        let pending = discussion.proposals.iter().find(|p| p.status == "pending");
        let previous_proposal = match (pending, parent_id) {
            (Some(p), Some(parent)) if p.id == parent => Some(PreviousProposalContext {
                id: p.id.clone(),
                proposed_wording: p.candidate.proposed_wording.clone(),
                explanation: p.candidate.explanation.clone(),
            }),
            (None, None) => None,
            _ => {
                return Err(invalid(
                    "The pending suggestion changed. Reopen it before requesting another version.",
                ));
            }
        };

        let mut evidence = vec![];
        for s in selections {
            evidence.push(evidence_on(&mut conn, s).await?);
        }
        let request = ProposalRequest {
            page_id: page.id,
            base_revision: base,
            block_id: anchor.block_id.clone(),
            selected_quote: anchor.quote.clone(),
            human_request: request_text.into(),
            previous_proposal,
            sources: evidence
                .iter()
                .map(|e| SourceExcerpt {
                    id: e.id.clone(),
                    revision: e.source.revision,
                    passage: e.quote.clone(),
                })
                .collect(),
        };
        request.validate().map_err(invalid)?;
        Ok((request, evidence))
    }
    /// Called by the validated provider adapter, never exposed as an arbitrary frontend candidate command.
    pub(crate) async fn record_candidate(
        &self,
        submission: CandidateSubmission<'_>,
    ) -> Result<Discussion, Error> {
        let CandidateSubmission {
            thread_id,
            request,
            anchor,
            evidence,
            candidate,
            parent_id,
            origin,
        } = submission;
        candidate.validate_for(request).map_err(invalid)?;
        let mut tx = self.pool.begin().await?;
        let discussion = discussion_on(&mut tx, thread_id).await?;
        if discussion.page_id != request.page_id
            || request.block_id != anchor.block_id
            || request.selected_quote != anchor.quote
        {
            return Err(invalid(
                "The proposal context does not match its discussion.",
            ));
        }
        // Validate against the original requested revision even if the user edited the page while generation ran. Such a result is stored as stale, never applied.
        let page = page_at(&mut tx, &request.page_id, Some(request.base_revision)).await?;
        check_anchor(&page, anchor)?;
        let mut evidence_ids = std::collections::HashSet::new();
        for e in evidence {
            if !evidence_ids.insert(&e.id) {
                return Err(invalid("Duplicate evidence."));
            }
            let actual = evidence_on(&mut tx, &e.source).await?;
            if actual.id != e.id
                || actual.quote != e.quote
                || !request.sources.iter().any(|s| {
                    s.id == e.id && s.revision == e.source.revision && s.passage == e.quote
                })
            {
                return Err(invalid(
                    "Evidence differs from the supplied source revision.",
                ));
            }
        }
        if evidence.len() != request.sources.len() {
            return Err(invalid("Missing evidence."));
        }
        let pending = discussion.proposals.iter().find(|p| p.status == "pending");
        match (pending, parent_id) {
            (Some(p), Some(parent)) if p.id == parent => {
                let prior = request
                    .previous_proposal
                    .as_ref()
                    .ok_or_else(|| invalid("Missing previous proposal context."))?;
                if prior.id != p.id
                    || prior.proposed_wording != p.candidate.proposed_wording
                    || prior.explanation != p.candidate.explanation
                {
                    return Err(invalid("The previous proposal context changed."));
                }
                sqlx::query("UPDATE proposals SET status='superseded' WHERE id=?")
                    .bind(parent)
                    .execute(&mut *tx)
                    .await?;
            }
            (None, None) if request.previous_proposal.is_none() => {}
            _ => {
                return Err(invalid(
                    "The pending suggestion changed. Reopen it before requesting another version.",
                ));
            }
        }
        sqlx::query("INSERT INTO proposals(id,thread_id,parent_id,base_revision,anchor_json,candidate_json,evidence_json,request_text,origin) VALUES(?,?,?,?,?,?,?,?,?)")
            .bind(id()).bind(thread_id).bind(parent_id).bind(request.base_revision).bind(serde_json::to_string(anchor)?).bind(serde_json::to_string(candidate)?).bind(serde_json::to_string(evidence)?).bind(&request.human_request).bind(origin).execute(&mut *tx).await?;
        tx.commit().await?;
        self.discussion(thread_id).await
    }
    pub async fn revise_proposal(
        &self,
        proposal_id: &str,
        wording: &str,
        reason: &str,
    ) -> Result<Discussion, Error> {
        bounded(wording)?;
        bounded(reason)?;
        let thread_id: String = sqlx::query_scalar("SELECT thread_id FROM proposals WHERE id=?")
            .bind(proposal_id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(Error::NotFound)?;
        let d = self.discussion(&thread_id).await?;
        let p = d
            .proposals
            .iter()
            .find(|p| p.id == proposal_id && p.status == "pending")
            .ok_or_else(|| invalid("Only a pending proposal can be revised."))?;
        let selections = p
            .evidence
            .iter()
            .map(|e| e.source.clone())
            .collect::<Vec<_>>();
        let (request, evidence) = self
            .proposal_context(
                &thread_id,
                reason,
                &selections,
                &p.anchor,
                p.base_revision,
                Some(proposal_id),
            )
            .await?;
        let candidate = ProposalCandidate {
            proposed_wording: wording.into(),
            explanation: reason.into(),
            cited_source_ids: p.candidate.cited_source_ids.clone(),
        };
        self.record_candidate(CandidateSubmission {
            thread_id: &thread_id,
            request: &request,
            anchor: &p.anchor,
            evidence: &evidence,
            candidate: &candidate,
            parent_id: Some(proposal_id),
            origin: "human revision",
        })
        .await
    }
    pub async fn decide_proposal(
        &self,
        proposal_id: &str,
        accept: bool,
    ) -> Result<Discussion, Error> {
        let decision = if accept { "accepted" } else { "rejected" };
        let mut tx = self.pool.begin().await?;
        let thread_id: String = sqlx::query_scalar("SELECT thread_id FROM proposals WHERE id=?")
            .bind(proposal_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(Error::NotFound)?;
        let d = discussion_on(&mut tx, &thread_id).await?;
        let p = d
            .proposals
            .iter()
            .find(|p| p.id == proposal_id)
            .ok_or(Error::NotFound)?;
        if p.status == decision {
            tx.rollback().await?;
            return self.discussion(&thread_id).await;
        }
        if p.status != "pending" {
            return Err(invalid(
                "This suggestion already has a decision or a newer version.",
            ));
        }
        let mut published_revision = None;
        if accept {
            let page = page_at(&mut tx, &d.page_id, None).await?;
            if page.revision != p.base_revision {
                return Err(Error::Stale);
            }
            let mut replacement = vec![Inline::Text {
                text: p.candidate.proposed_wording.clone(),
                marks: vec![],
            }];
            for source_id in &p.candidate.cited_source_ids {
                let evidence = p
                    .evidence
                    .iter()
                    .find(|e| &e.id == source_id)
                    .ok_or_else(|| invalid("Unknown proposal evidence."))?;
                let actual = evidence_on(&mut tx, &evidence.source).await?;
                if actual.quote != evidence.quote {
                    return Err(invalid("The cited passage cannot be verified."));
                }
                let citation_id = id();
                sqlx::query("INSERT INTO citations(id,source_id,source_revision,block_id,quote,range_json) VALUES(?,?,?,?,?,?)").bind(&citation_id).bind(&evidence.source.object_id).bind(evidence.source.revision).bind(&evidence.source.block_id).bind(&evidence.quote).bind(evidence.source.range.as_ref().map(serde_json::to_string).transpose()?).execute(&mut *tx).await?;
                replacement.push(Inline::Citation {
                    attrs: CitationAttrs { citation_id },
                });
            }
            let next = page.document.replace_passage(&PassagePatch {
                block_id: p.anchor.block_id.clone(),
                from_utf16: p.anchor.from_utf16,
                to_utf16: p.anchor.to_utf16,
                expected_quote: p.anchor.quote.clone(),
                replacement,
            })?;
            validate_citations(&mut tx, &next).await?;
            sqlx::query("UPDATE objects SET current_revision=current_revision+1 WHERE id=? AND current_revision=?").bind(&d.page_id).bind(p.base_revision).execute(&mut *tx).await?;
            sqlx::query("INSERT INTO revisions(object_id,revision,document_json,actor_id) VALUES(?,?,?,'local-operator')").bind(&d.page_id).bind(p.base_revision+1).bind(serde_json::to_string(&next)?).execute(&mut *tx).await?;
            // Preserve any existing draft; its old base revision prevents silently overwriting this accepted change.
            published_revision = Some(p.base_revision + 1);
        }
        sqlx::query("UPDATE proposals SET status=? WHERE id=?")
            .bind(decision)
            .bind(proposal_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO review_decisions(proposal_id,decision,actor_id,published_revision) VALUES(?,?,'local-operator',?)").bind(proposal_id).bind(decision).bind(published_revision).execute(&mut *tx).await?;
        tx.commit().await?;
        self.discussion(&thread_id).await
    }
    pub async fn page_citations(
        &self,
        page_id: &str,
        revision: i32,
    ) -> Result<Vec<Evidence>, Error> {
        let mut conn = self.pool.acquire().await?;
        let page = page_at(&mut conn, page_id, Some(revision)).await?;
        let mut out = vec![];
        for block in page.document.doc.content {
            for node in block.content() {
                if let Inline::Citation { attrs } = node {
                    if out.iter().any(|e: &Evidence| e.id == attrs.citation_id) {
                        continue;
                    }
                    let r = sqlx::query("SELECT * FROM citations WHERE id=?")
                        .bind(&attrs.citation_id)
                        .fetch_optional(&mut *conn)
                        .await?
                        .ok_or(Error::NotFound)?;
                    let mut e = evidence_on(
                        &mut conn,
                        &EvidenceSelection {
                            object_id: r.get("source_id"),
                            revision: r.get("source_revision"),
                            block_id: r.get("block_id"),
                            range: r
                                .get::<Option<String>, _>("range_json")
                                .map(|v| serde_json::from_str(&v))
                                .transpose()?,
                        },
                    )
                    .await?;
                    if e.quote != r.get::<String, _>("quote") {
                        return Err(invalid("Stored citation differs from source."));
                    }
                    e.id = attrs.citation_id.clone();
                    out.push(e);
                }
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
#[path = "review_tests.rs"]
mod tests;
