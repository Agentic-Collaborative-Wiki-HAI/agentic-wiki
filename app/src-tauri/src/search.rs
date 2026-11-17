use crate::{
    domain::*,
    navigation::WikiRecord,
    review::{Evidence, EvidenceRange, EvidenceSelection, check_anchor, evidence_on, page_at},
    store::{Page, Store},
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct PassageResult {
    pub source: EvidenceSelection,
    pub title: String,
    pub breadcrumb: Vec<String>,
    pub snippet: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../src/generated/")]
pub struct EvidenceContext {
    pub evidence: Vec<Evidence>,
    pub linked_ids: Vec<String>,
    pub notice: String,
}
fn bad(s: &str) -> Error {
    Error::Invalid(s.into())
}
fn chunks(text: &str) -> Vec<(usize, usize, String)> {
    let chars: Vec<char> = text.chars().collect();
    let mut result = vec![];
    let mut start = 0;
    let mut offset = 0;
    while start < chars.len() {
        let mut end = (start + 1800).min(chars.len());
        if end < chars.len()
            && let Some(i) = (start + 900..end).rev().find(|i| chars[*i].is_whitespace())
        {
            end = i + 1
        }
        let part: String = chars[start..end].iter().collect();
        let len = part.encode_utf16().count();
        if !part.trim().is_empty() {
            result.push((offset, offset + len, part))
        }
        offset += len;
        start = end;
    }
    result
}
fn query(value: &str) -> String {
    value
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| s.chars().count() > 1)
        .take(32)
        .filter(|s| {
            !matches!(
                s.to_lowercase().as_str(),
                "the"
                    | "and"
                    | "please"
                    | "agent"
                    | "this"
                    | "that"
                    | "with"
                    | "from"
                    | "for"
                    | "are"
                    | "was"
                    | "to"
                    | "of"
                    | "in"
                    | "is"
                    | "it"
                    | "an"
                    | "as"
                    | "on"
                    | "be"
                    | "by"
                    | "or"
            )
        })
        .map(|s| format!("\"{}\"*", s.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" OR ")
}
fn breadcrumb(records: &[WikiRecord], id: &str) -> Vec<String> {
    let mut out = vec![];
    let mut next = Some(id);
    let mut seen = HashSet::new();
    while let Some(id) = next {
        if !seen.insert(id) {
            break;
        }
        if let Some(r) = records.iter().find(|r| r.id == id) {
            out.push(r.title.clone());
            next = r.parent_id.as_deref()
        } else {
            break;
        }
    }
    out.reverse();
    out
}
impl Store {
    pub async fn search_passages(
        &self,
        text: &str,
        objects: Option<&[String]>,
        limit: i64,
    ) -> Result<Vec<PassageResult>, Error> {
        if text.len() > 4000 || objects.is_some_and(|v| v.len() > 500) {
            return Err(bad("Narrow this search."));
        }
        let records = self.records().await?;
        let mut tx = self.pool.begin().await?;
        // Rebuildable index: only current saved revisions are indexed, never drafts or private external files.
        let rows=sqlx::query("SELECT o.id,o.title,o.current_revision,r.document_json FROM objects o JOIN revisions r ON r.object_id=o.id AND r.revision=o.current_revision LEFT JOIN search_versions v ON v.object_id=o.id WHERE v.object_id IS NULL OR v.revision!=o.current_revision OR v.title!=o.title").fetch_all(&mut *tx).await?;
        for r in rows {
            let id: String = r.get("id");
            let title: String = r.get("title");
            let revision: i32 = r.get("current_revision");
            let doc: RichDocument = serde_json::from_str(r.get("document_json"))?;
            doc.validate()?;
            sqlx::query("DELETE FROM passage_search WHERE object_id=?")
                .bind(&id)
                .execute(&mut *tx)
                .await?;
            for block in &doc.doc.content {
                if !matches!(block, Block::Paragraph { .. }) {
                    continue;
                }
                let plain = block
                    .content()
                    .iter()
                    .map(Inline::plain)
                    .collect::<String>();
                for (from, to, body) in chunks(&plain) {
                    sqlx::query("INSERT INTO passage_search(title,body,object_id,revision,block_id,start_utf16,end_utf16) VALUES(?,?,?,?,?,?,?)").bind(&title).bind(body).bind(&id).bind(revision).bind(block.id()).bind(from as i64).bind(to as i64).execute(&mut *tx).await?;
                }
            }
            sqlx::query("INSERT INTO search_versions(object_id,revision,title) VALUES(?,?,?) ON CONFLICT(object_id) DO UPDATE SET revision=excluded.revision,title=excluded.title").bind(id).bind(revision).bind(title).execute(&mut *tx).await?;
        }
        sqlx::query("DELETE FROM passage_search WHERE object_id NOT IN (SELECT id FROM objects)")
            .execute(&mut *tx)
            .await?;
        let filter = objects.map(serde_json::to_string).transpose()?;
        let q = query(text);
        let limit = limit.clamp(1, 100);
        let rows = if q.is_empty() {
            if !text.trim().is_empty() {
                tx.commit().await?;
                return Ok(vec![]);
            }
            sqlx::query("SELECT *,substr(body,1,240) AS excerpt FROM passage_search WHERE (? IS NULL OR object_id IN (SELECT value FROM json_each(?))) ORDER BY lower(title),rowid LIMIT ?").bind(&filter).bind(&filter).bind(limit).fetch_all(&mut *tx).await?
        } else {
            sqlx::query("SELECT *,snippet(passage_search,1,'','',' … ',28) AS excerpt FROM passage_search WHERE passage_search MATCH ? AND (? IS NULL OR object_id IN (SELECT value FROM json_each(?))) ORDER BY bm25(passage_search,3.0,1.0),object_id,rowid LIMIT ?").bind(q).bind(&filter).bind(&filter).bind(limit).fetch_all(&mut *tx).await?
        };
        let out = rows
            .into_iter()
            .map(|r| {
                let object_id: String = r.get("object_id");
                PassageResult {
                    breadcrumb: breadcrumb(&records, &object_id),
                    title: r.get("title"),
                    snippet: r.get("excerpt"),
                    source: EvidenceSelection {
                        object_id,
                        revision: r.get("revision"),
                        block_id: r.get("block_id"),
                        range: Some(EvidenceRange {
                            from_utf16: r.get::<i64, _>("start_utf16") as usize,
                            to_utf16: r.get::<i64, _>("end_utf16") as usize,
                        }),
                    },
                }
            })
            .collect();
        tx.commit().await?;
        Ok(out)
    }
    pub async fn default_evidence(
        &self,
        page_id: &str,
        text: &str,
    ) -> Result<EvidenceContext, Error> {
        let page = self.read(page_id).await?;
        let cited = self.page_citations(page_id, page.revision).await?;
        let mut linked: HashSet<String> =
            cited.iter().map(|e| e.source.object_id.clone()).collect();
        for r in self.records().await? {
            if (r.kind == "recording" || r.imported)
                && (r.parent_id.as_deref() == Some(page_id) || r.id == page_id)
            {
                linked.insert(r.id);
            }
        }
        let mut linked_ids: Vec<_> = linked.into_iter().collect();
        linked_ids.sort();
        let mut evidence = vec![];
        for e in cited {
            let mut e = self.evidence(&e.source).await?;
            if e.quote.chars().count() > 1800 {
                let base = e.source.range.as_ref().map_or(0, |r| r.from_utf16);
                let excerpt: String = e.quote.chars().take(1800).collect();
                e.source.range = Some(EvidenceRange {
                    from_utf16: base,
                    to_utf16: base + excerpt.encode_utf16().count(),
                });
                e = self.evidence(&e.source).await?;
            }
            if !evidence.iter().any(|old: &Evidence| old.id == e.id)
                && evidence.iter().map(|e| e.quote.len()).sum::<usize>() + e.quote.len() <= 12000
            {
                evidence.push(e);
            }
            if evidence.len() == 4 {
                break;
            }
        }
        let found = self.search_passages(text, Some(&linked_ids), 32).await?;
        for hit in found {
            if evidence.len() >= 6 {
                break;
            }
            let e = self.evidence(&hit.source).await?;
            if evidence.iter().map(|e| e.quote.len()).sum::<usize>() + e.quote.len() <= 12000
                && !evidence.iter().any(|old| {
                    old.source.object_id == e.source.object_id
                        && old.source.revision == e.source.revision
                        && old.source.block_id == e.source.block_id
                        && (old.source.range.is_none() || old.source.range == e.source.range)
                })
            {
                evidence.push(e)
            }
        }
        Ok(EvidenceContext{evidence,linked_ids,notice:"Up to six excerpts from linked sources, selected using the passage and your request. Inspect or change them before sending; these are excerpts, not the whole transcript.".into()})
    }
    pub async fn manual_citation(
        &self,
        id: &str,
        page_id: &str,
        base: i32,
        anchor: &PassageAnchor,
        source: &EvidenceSelection,
    ) -> Result<Page, Error> {
        uuid::Uuid::parse_str(id).map_err(|_| bad("Invalid citation request."))?;
        let mut tx = self.pool.begin().await?;
        let page = page_at(&mut tx, page_id, None).await?;
        if page.document.doc.content.iter().any(|b| {
            b.content()
                .iter()
                .any(|n| matches!(n,Inline::Citation{attrs} if attrs.citation_id==id))
        }) {
            tx.commit().await?;
            return Ok(page);
        }
        if page.revision != base {
            return Err(Error::Stale);
        }
        check_anchor(&page, anchor)?;
        let e = evidence_on(&mut tx, source).await?;
        sqlx::query("INSERT INTO citations(id,source_id,source_revision,block_id,quote,range_json) VALUES(?,?,?,?,?,?)").bind(id).bind(&source.object_id).bind(source.revision).bind(&source.block_id).bind(&e.quote).bind(source.range.as_ref().map(serde_json::to_string).transpose()?).execute(&mut *tx).await?;
        let document = page.document.replace_passage(&PassagePatch {
            block_id: anchor.block_id.clone(),
            from_utf16: anchor.to_utf16,
            to_utf16: anchor.to_utf16,
            expected_quote: String::new(),
            replacement: vec![Inline::Citation {
                attrs: CitationAttrs {
                    citation_id: id.into(),
                },
            }],
        })?;
        sqlx::query("UPDATE objects SET current_revision=current_revision+1 WHERE id=?")
            .bind(page_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO revisions(object_id,revision,document_json,actor_id) VALUES(?,?,?,'local-operator')").bind(page_id).bind(base+1).bind(serde_json::to_string(&document)?).execute(&mut *tx).await?;
        tx.commit().await?;
        self.read(page_id).await
    }
}
