use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid document: {0}")]
    Invalid(String),
    #[error("This document has changed. Reload before saving or reviewing this change.")]
    Stale,
    #[error("The requested page does not exist.")]
    NotFound,
    #[error("Storage error: {0}")]
    Storage(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct RichDocument {
    pub schema_version: u32,
    pub doc: DocumentNode,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct DocumentNode {
    #[serde(rename = "type")]
    pub kind: DocumentKind,
    pub content: Vec<Block>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[ts(export_to = "../../src/generated/")]
pub enum DocumentKind {
    #[serde(rename = "doc")]
    Doc,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "type", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub enum Block {
    #[serde(rename = "paragraph")]
    Paragraph {
        attrs: BlockAttrs,
        #[serde(default)]
        content: Vec<Inline>,
    },
    #[serde(rename = "heading")]
    Heading {
        attrs: HeadingAttrs,
        #[serde(default)]
        content: Vec<Inline>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export_to = "../../src/generated/")]
pub struct BlockAttrs {
    pub block_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export_to = "../../src/generated/")]
pub struct HeadingAttrs {
    pub block_id: String,
    pub level: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "type", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub enum Inline {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        marks: Vec<Mark>,
    },
    #[serde(rename = "hardBreak")]
    HardBreak,
    #[serde(rename = "citation")]
    Citation { attrs: CitationAttrs },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export_to = "../../src/generated/")]
pub struct CitationAttrs {
    pub citation_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "type", deny_unknown_fields)]
#[ts(export_to = "../../src/generated/")]
pub enum Mark {
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "italic")]
    Italic,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "link")]
    Link { attrs: LinkAttrs },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(deny_unknown_fields)]
#[ts(export_to = "../../src/generated/")]
pub struct LinkAttrs {
    pub href: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct PassagePatch {
    pub block_id: String,
    pub from_utf16: usize,
    pub to_utf16: usize,
    pub expected_quote: String,
    pub replacement: Vec<Inline>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, export_to = "../../src/generated/")]
pub struct PassageAnchor {
    pub block_id: String,
    pub from_utf16: usize,
    pub to_utf16: usize,
    pub quote: String,
    pub state: AnchorState,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "../../src/generated/")]
pub enum AnchorState {
    Attached,
    Detached,
}

impl Block {
    pub fn id(&self) -> &str {
        match self {
            Self::Paragraph { attrs, .. } => &attrs.block_id,
            Self::Heading { attrs, .. } => &attrs.block_id,
        }
    }
    pub fn content(&self) -> &Vec<Inline> {
        match self {
            Self::Paragraph { content, .. } | Self::Heading { content, .. } => content,
        }
    }
}

impl Inline {
    pub fn plain(&self) -> String {
        match self {
            Self::Text { text, .. } => text.clone(),
            Self::HardBreak => "\n".into(),
            Self::Citation { .. } => "\u{fffc}".into(),
        }
    }
    pub fn len_utf16(&self) -> usize {
        self.plain().encode_utf16().count()
    }
}

fn invalid(s: &str) -> Error {
    Error::Invalid(s.into())
}

fn validate_inline(content: &[Inline]) -> Result<(), Error> {
    if content.len() > 10_000 {
        return Err(invalid("too many inline nodes"));
    }
    for n in content {
        match n {
            Inline::Text { text, marks } => {
                if text.is_empty() {
                    return Err(invalid("empty text node"));
                }
                let mut seen = HashSet::new();
                for mark in marks {
                    let name = match mark {
                        Mark::Bold => "bold",
                        Mark::Italic => "italic",
                        Mark::Code => "code",
                        Mark::Link { attrs } => {
                            if !(attrs.href.starts_with("https://")
                                || attrs.href.starts_with("http://")
                                || attrs.href.starts_with('#'))
                            {
                                return Err(invalid("unsupported link protocol"));
                            }
                            "link"
                        }
                    };
                    if !seen.insert(name) {
                        return Err(invalid("duplicate mark"));
                    }
                }
            }
            Inline::Citation { attrs } if attrs.citation_id.trim().is_empty() => {
                return Err(invalid("missing citation identity"));
            }
            _ => {}
        }
    }
    Ok(())
}

impl RichDocument {
    pub fn validate(&self) -> Result<(), Error> {
        if self.schema_version != 1 {
            return Err(invalid("unsupported schema version"));
        }
        if self.doc.content.is_empty() || self.doc.content.len() > 2000 {
            return Err(invalid("invalid block count"));
        }
        if serde_json::to_vec(self)?.len() > 2_000_000 {
            return Err(invalid("document is too large"));
        }
        let mut ids = HashSet::new();
        for block in &self.doc.content {
            if block.id().is_empty() || !ids.insert(block.id()) {
                return Err(invalid("missing or duplicate block ID"));
            }
            if let Block::Heading { attrs, .. } = block
                && !(1..=6).contains(&attrs.level)
            {
                return Err(invalid("invalid heading level"));
            }
            validate_inline(block.content())?;
        }
        Ok(())
    }

    pub fn replace_passage(&self, patch: &PassagePatch) -> Result<Self, Error> {
        self.validate()?;
        validate_inline(&patch.replacement)?;
        if patch.from_utf16 > patch.to_utf16 {
            return Err(invalid("reversed range"));
        }
        let mut next = self.clone();
        let block = next
            .doc
            .content
            .iter_mut()
            .find(|b| b.id() == patch.block_id)
            .ok_or_else(|| invalid("missing block"))?;
        let Block::Paragraph { content, .. } = block else {
            return Err(invalid("the first patch contract supports paragraphs only"));
        };
        let text = content.iter().map(Inline::plain).collect::<String>();
        let from = utf16_boundary(&text, patch.from_utf16)?;
        let to = utf16_boundary(&text, patch.to_utf16)?;
        if text[from..to] != patch.expected_quote {
            return Err(invalid("selected text does not match"));
        }
        let left = inline_slice(content, 0, patch.from_utf16)?;
        let right = inline_slice(content, patch.to_utf16, text.encode_utf16().count())?;
        let combined = left
            .into_iter()
            .chain(patch.replacement.clone())
            .chain(right);
        *content = normalize(combined.collect());
        next.validate()?;
        Ok(next)
    }
}

pub fn utf16_boundary(text: &str, offset: usize) -> Result<usize, Error> {
    let mut units = 0;
    for (bytes, ch) in text.char_indices() {
        if units == offset {
            return Ok(bytes);
        }
        units += ch.len_utf16();
        if units > offset {
            return Err(invalid("range splits a Unicode surrogate pair"));
        }
    }
    if units == offset {
        Ok(text.len())
    } else {
        Err(invalid("range exceeds the passage"))
    }
}

fn inline_slice(content: &[Inline], from: usize, to: usize) -> Result<Vec<Inline>, Error> {
    let mut result = Vec::new();
    let mut start = 0;
    for node in content {
        let end = start + node.len_utf16();
        let a = from.max(start);
        let b = to.min(end);
        if a < b {
            match node {
                Inline::Text { text, marks } => {
                    let left = utf16_boundary(text, a - start)?;
                    let right = utf16_boundary(text, b - start)?;
                    result.push(Inline::Text {
                        text: text[left..right].into(),
                        marks: marks.clone(),
                    });
                }
                _ => result.push(node.clone()),
            }
        }
        start = end;
    }
    Ok(result)
}

fn normalize(content: Vec<Inline>) -> Vec<Inline> {
    let mut result: Vec<Inline> = Vec::new();
    for node in content {
        if let Inline::Text { text, marks } = &node
            && let Some(Inline::Text {
                text: previous,
                marks: previous_marks,
            }) = result.last_mut()
            && marks == previous_marks
        {
            previous.push_str(text);
            continue;
        }
        result.push(node);
    }
    result
}

pub fn map_anchor(anchor: &PassageAnchor, patch: &PassagePatch) -> PassageAnchor {
    let mut next = anchor.clone();
    if anchor.block_id != patch.block_id || anchor.state == AnchorState::Detached {
        return next;
    }
    if patch.to_utf16 <= anchor.from_utf16 {
        let added: usize = patch.replacement.iter().map(Inline::len_utf16).sum();
        let removed = patch.to_utf16 - patch.from_utf16;
        next.from_utf16 = anchor.from_utf16 - removed + added;
        next.to_utf16 = anchor.to_utf16 - removed + added;
    } else if patch.from_utf16 < anchor.to_utf16 {
        next.state = AnchorState::Detached;
    }
    next
}

pub fn text(value: &str) -> Inline {
    Inline::Text {
        text: value.into(),
        marks: vec![],
    }
}
pub fn paragraph(id: &str, value: &str) -> Block {
    Block::Paragraph {
        attrs: BlockAttrs {
            block_id: id.into(),
        },
        content: vec![text(value)],
    }
}
pub fn heading(id: &str, value: &str) -> Block {
    Block::Heading {
        attrs: HeadingAttrs {
            block_id: id.into(),
            level: 2,
        },
        content: vec![text(value)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn doc() -> RichDocument {
        RichDocument {
            schema_version: 1,
            doc: DocumentNode {
                kind: DocumentKind::Doc,
                content: vec![paragraph("p1", "A 🔬 e\u{301} example")],
            },
        }
    }
    #[test]
    fn rejects_surrogate_split_and_wrong_quote() {
        let patch = PassagePatch {
            block_id: "p1".into(),
            from_utf16: 3,
            to_utf16: 4,
            expected_quote: "".into(),
            replacement: vec![text("x")],
        };
        assert!(doc().replace_passage(&patch).is_err());
        let patch = PassagePatch {
            from_utf16: 2,
            expected_quote: "wrong".into(),
            ..patch
        };
        assert!(doc().replace_passage(&patch).is_err());
    }
    #[test]
    fn maps_before_and_detaches_overlap() {
        let a = PassageAnchor {
            block_id: "p1".into(),
            from_utf16: 5,
            to_utf16: 8,
            quote: "e\u{301} ".into(),
            state: AnchorState::Attached,
        };
        let p = PassagePatch {
            block_id: "p1".into(),
            from_utf16: 2,
            to_utf16: 4,
            expected_quote: "🔬".into(),
            replacement: vec![text("scope")],
        };
        assert_eq!(map_anchor(&a, &p).from_utf16, 8);
        assert_eq!(
            map_anchor(
                &a,
                &PassagePatch {
                    from_utf16: 5,
                    to_utf16: 7,
                    ..p
                }
            )
            .state,
            AnchorState::Detached
        );
    }
    #[test]
    fn rejects_unknown_schema_duplicate_ids_and_unsafe_link() {
        let mut d = doc();
        d.schema_version = 2;
        assert!(d.validate().is_err());
        d.schema_version = 1;
        d.doc.content.push(d.doc.content[0].clone());
        assert!(d.validate().is_err());
        d.doc.content.pop();
        let Block::Paragraph { content, .. } = &mut d.doc.content[0] else {
            unreachable!()
        };
        *content = vec![Inline::Text {
            text: "bad".into(),
            marks: vec![Mark::Link {
                attrs: LinkAttrs {
                    href: "javascript:alert(1)".into(),
                },
            }],
        }];
        assert!(d.validate().is_err());
    }
}
