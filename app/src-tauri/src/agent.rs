//! Provider-independent proposal data. This module has no storage or canonical-write capability.
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/generated/")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceExcerpt {
    pub id: String,
    pub revision: i32,
    pub passage: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/generated/")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreviousProposalContext {
    pub id: String,
    pub proposed_wording: String,
    pub explanation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/generated/")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProposalRequest {
    pub page_id: String,
    pub base_revision: i32,
    pub block_id: String,
    pub selected_quote: String,
    pub human_request: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_proposal: Option<PreviousProposalContext>,
    pub sources: Vec<SourceExcerpt>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/generated/")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProposalCandidate {
    pub proposed_wording: String,
    pub explanation: String,
    pub cited_source_ids: Vec<String>,
}

impl ProposalRequest {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.base_revision < 1
            || self.page_id.trim().is_empty()
            || self.block_id.trim().is_empty()
        {
            return Err("A proposal requires an identified page, revision and passage.");
        }
        if self.selected_quote.trim().is_empty() || self.human_request.trim().is_empty() {
            return Err("A proposal requires selected text and a human request.");
        }
        if self.sources.len() > 8
            || serde_json::to_vec(self)
                .map_err(|_| "Invalid context")?
                .len()
                > 32_000
        {
            return Err("The selected context exceeds the request limit.");
        }
        if let Some(previous) = &self.previous_proposal
            && (previous.id.trim().is_empty()
                || previous.proposed_wording.trim().is_empty()
                || previous.explanation.trim().is_empty())
        {
            return Err("A prior proposal requires its identity, wording and explanation.");
        }
        let mut ids = HashSet::new();
        for source in &self.sources {
            if source.id.trim().is_empty()
                || source.revision < 1
                || source.passage.trim().is_empty()
                || !ids.insert(&source.id)
            {
                return Err("Source excerpts require unique identities, revisions and text.");
            }
        }
        Ok(())
    }
}

impl ProposalCandidate {
    pub fn validate_for(&self, request: &ProposalRequest) -> Result<(), &'static str> {
        request.validate()?;
        if self.proposed_wording.trim().is_empty()
            || self.explanation.trim().is_empty()
            || self.cited_source_ids.is_empty()
        {
            return Err(
                "A candidate requires proposed wording, an explanation and a selected source citation.",
            );
        }
        if serde_json::to_vec(self)
            .map_err(|_| "Invalid candidate")?
            .len()
            > 16_000
        {
            return Err("The candidate exceeds the response limit.");
        }
        let mut ids = HashSet::new();
        for id in &self.cited_source_ids {
            if !request.sources.iter().any(|source| &source.id == id) || !ids.insert(id) {
                return Err("A candidate may cite only distinct supplied source identities.");
            }
        }
        // Structural validation does not establish that a source supports the wording. Human review remains required.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> ProposalRequest {
        ProposalRequest {
            page_id: "guide".into(),
            base_revision: 1,
            block_id: "booking".into(),
            selected_quote: "Use the spreadsheet".into(),
            human_request: "Check against this source".into(),
            previous_proposal: None,
            sources: vec![SourceExcerpt {
                id: "recording-18".into(),
                revision: 1,
                passage: "Use the current booking calendar".into(),
            }],
        }
    }
    #[test]
    fn rejects_context_and_citation_scope_expansion() {
        let r = request();
        let mut candidate = ProposalCandidate {
            proposed_wording: "Use the calendar".into(),
            explanation: "The supplied excerpt names the calendar".into(),
            cited_source_ids: vec!["recording-18".into()],
        };
        assert!(candidate.validate_for(&r).is_ok());
        candidate.cited_source_ids.push("outside-source".into());
        assert!(candidate.validate_for(&r).is_err());
        let mut oversized = r.clone();
        oversized.human_request = "x".repeat(32_000);
        assert!(oversized.validate().is_err());
        let mut unknown = serde_json::to_value(r).unwrap();
        unknown["filesystemRoot"] = "/private".into();
        assert!(serde_json::from_value::<ProposalRequest>(unknown).is_err());
    }
}
