use crate::{
    agent::ProposalCandidate,
    backup::{self, WorkspaceRuntime},
    domain::*,
    imports::*,
    review::*,
    store::Store,
};
use sqlx::Row;
use std::fs;

async fn setup() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    (dir, store)
}
async fn page(s: &Store, title: &str, text: &str, parent: Option<&str>) -> crate::store::Page {
    let id = uuid::Uuid::new_v4().to_string();
    let mut page = s.create_page(&id, title, parent).await.unwrap();
    page.document.doc.content = vec![paragraph("body", text)];
    s.save(&id, 1, &page.document).await.unwrap()
}
fn anchor(text: &str) -> PassageAnchor {
    PassageAnchor {
        block_id: "body".into(),
        from_utf16: 0,
        to_utf16: text.encode_utf16().count(),
        quote: text.into(),
        state: AnchorState::Attached,
    }
}
async fn imported(s: &Store, text: &str, parent: Option<&str>) -> crate::store::Page {
    let preview = s
        .preview_import(SourceBytes {
            provider: SourceProvider::Local,
            locator: "/fictional/meeting.txt".into(),
            name: "Meeting.txt".into(),
            url: None,
            modified: None,
            bytes: text.as_bytes().to_vec(),
        })
        .await
        .unwrap();
    s.commit_import(&preview, "Meeting", ImportKind::Transcript, parent)
        .await
        .unwrap()
}
#[tokio::test]
async fn navigation_is_durable_rejects_cycles_and_does_not_revise_content() {
    let (dir, s) = setup().await;
    let parent = page(&s, "Project", "Overview", None).await;
    let child = page(&s, "Notes", "Keep this passage.", Some(&parent.id)).await;
    let same = s
        .create_page(&child.id, "Notes", Some(&parent.id))
        .await
        .unwrap();
    assert_eq!(same.revision, child.revision);
    assert!(
        s.organize_page(&parent.id, 1, "Project", Some(&child.id))
            .await
            .is_err()
    );
    assert!(
        s.organize_page(&child.id, 1, "Notes", Some(&child.id))
            .await
            .is_err()
    );
    assert!(s.organize_page(&child.id, 1, " ", None).await.is_err());
    assert!(
        s.organize_page(&child.id, 1, "Notes", Some("missing"))
            .await
            .is_err()
    );
    let moved = s
        .organize_page(&child.id, 1, "Renamed notes", None)
        .await
        .unwrap();
    assert_eq!(moved.document, child.document);
    assert_eq!(moved.revision, child.revision);
    assert!(
        s.organize_page(&child.id, 1, "Stale update", None)
            .await
            .is_err()
    );
    s.pool.close().await;
    let s = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    let records = s.records().await.unwrap();
    let r = records.iter().find(|r| r.id == child.id).unwrap();
    assert_eq!(r.title, "Renamed notes");
    assert_eq!(r.parent_id, None);
    assert_eq!(r.metadata_version, 2);
    assert_eq!(s.history(&child.id).await.unwrap().len(), 2);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM object_events WHERE object_id=?")
        .bind(child.id)
        .fetch_one(&s.pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}
#[tokio::test]
async fn search_uses_saved_content_and_current_breadcrumbs_and_exact_unicode_excerpts() {
    let (_dir, s) = setup().await;
    let root = page(&s, "Research", "Root", None).await;
    let text = format!(
        "{} 🔬 quokka-needle is here. {}",
        "intro ".repeat(700),
        "end ".repeat(700)
    );
    let p = page(&s, "Session notes", &text, Some(&root.id)).await;
    let hits = s.search_passages("quokka-needle", None, 20).await.unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].breadcrumb, vec!["Research", "Session notes"]);
    let evidence = s.evidence(&hits[0].source).await.unwrap();
    assert!(evidence.quote.contains("quokka-needle"));
    assert!(evidence.quote.len() < 8000);
    let range = hits[0].source.range.as_ref().unwrap();
    assert_eq!(
        evidence.quote,
        text[utf16_boundary(&text, range.from_utf16).unwrap()
            ..utf16_boundary(&text, range.to_utf16).unwrap()]
    );
    let mut draft = p.document.clone();
    draft.doc.content = vec![paragraph("body", "private-draft-only")];
    s.put_draft(&p.id, p.revision, &draft).await.unwrap();
    assert!(
        s.search_passages("private-draft-only", None, 20)
            .await
            .unwrap()
            .is_empty()
    );
    s.organize_page(&p.id, 1, "Renamed session", None)
        .await
        .unwrap();
    assert_eq!(
        s.search_passages("quokka", None, 20).await.unwrap()[0].breadcrumb,
        vec!["Renamed session"]
    );
    s.save(&p.id, p.revision, &draft).await.unwrap();
    assert!(
        s.search_passages("quokka", None, 20)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        s.search_passages("private-draft-only", None, 20)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        s.search_passages("\" OR * NOT: ; DROP TABLE objects;", None, 20)
            .await
            .is_ok()
    );
    assert_eq!(s.read(&p.id).await.unwrap().revision, 3);
}
#[tokio::test]
async fn long_linked_transcripts_are_bounded_and_other_sources_require_selection() {
    let (_dir, s) = setup().await;
    let p = page(&s, "Project", "Check booking instructions.", None).await;
    let text = (0..30)
        .map(|n| {
            format!(
                "Speaker {n}: booking calendar note {n}. {}",
                "more detail ".repeat(800)
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let source = imported(&s, &text, Some(&p.id)).await;
    let unlinked = page(&s, "Elsewhere", "booking private other project", None).await;
    let context = s.default_evidence(&p.id, "booking").await.unwrap();
    assert_eq!(context.evidence.len(), 6);
    assert_eq!(context.linked_ids, vec![source.id.clone()]);
    assert!(
        context
            .evidence
            .iter()
            .all(|e| e.source.object_id == source.id && e.quote.len() < 8000)
    );
    assert!(
        !context
            .evidence
            .iter()
            .any(|e| e.source.object_id == unlinked.id)
    );
    let other = s
        .search_passages("private", Some(&[unlinked.id]), 10)
        .await
        .unwrap();
    assert_eq!(other.len(), 1);
    assert!(s.evidence(&other[0].source).await.is_ok());
}
#[tokio::test]
async fn manual_citation_is_atomic_pinned_idempotent_and_survives_moves() {
    let (_dir, s) = setup().await;
    let p = page(&s, "Draft", "Check the calendar.", None).await;
    let source = page(
        &s,
        "Evidence",
        "🔬 Use the calendar, never the spreadsheet.",
        None,
    )
    .await;
    let selection = EvidenceSelection {
        object_id: source.id.clone(),
        revision: 2,
        block_id: "body".into(),
        range: Some(EvidenceRange {
            from_utf16: 3,
            to_utf16: 20,
        }),
    };
    let id = uuid::Uuid::new_v4().to_string();
    let cited = s
        .manual_citation(&id, &p.id, 2, &anchor("Check the calendar."), &selection)
        .await
        .unwrap();
    assert_eq!(cited.revision, 3);
    assert_eq!(
        s.manual_citation(&id, &p.id, 2, &anchor("Check the calendar."), &selection)
            .await
            .unwrap()
            .revision,
        3
    );
    assert!(
        s.manual_citation(
            &uuid::Uuid::new_v4().to_string(),
            &p.id,
            2,
            &anchor("Check the calendar."),
            &selection
        )
        .await
        .is_err()
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM citations")
        .fetch_one(&s.pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    let evidence = s.page_citations(&p.id, 3).await.unwrap();
    assert_eq!(evidence[0].quote, "Use the calendar,");
    s.organize_page(&source.id, 1, "Moved source", Some(&p.id))
        .await
        .unwrap();
    s.save(&source.id, 2, &source.document).await.unwrap();
    let pinned = s.page_citations(&p.id, 3).await.unwrap();
    assert_eq!(pinned[0].source.revision, 2);
    assert_eq!(pinned[0].source.range, selection.range);
    let bad = EvidenceSelection {
        range: Some(EvidenceRange {
            from_utf16: 1,
            to_utf16: 2,
        }),
        ..selection
    };
    assert!(s.evidence(&bad).await.is_err());
}
async fn proposal(s: &Store, p: &crate::store::Page, source: EvidenceSelection) -> Discussion {
    let a = anchor("Check the calendar.");
    let d = s
        .create_discussion(&p.id, p.revision, &a, "@agent use this source")
        .await
        .unwrap();
    let (request, evidence) = s
        .proposal_context(&d.id, "Use this source", &[source], &a, p.revision, None)
        .await
        .unwrap();
    let candidate = ProposalCandidate {
        proposed_wording: "Use the booking calendar.".into(),
        explanation: "The source gives the current booking instructions.".into(),
        cited_source_ids: vec![evidence[0].id.clone()],
    };
    s.record_candidate(CandidateSubmission {
        thread_id: &d.id,
        request: &request,
        anchor: &a,
        evidence: &evidence,
        candidate: &candidate,
        parent_id: None,
        origin: "Test fixture",
    })
    .await
    .unwrap()
}
#[tokio::test]
async fn backup_restores_complete_workspace_without_connections_or_overwrite() {
    let (dir, s) = setup().await;
    let p = page(&s, "Project", "Check the calendar.", None).await;
    let source = imported(&s, "00:18 Dana: Use the booking calendar.", Some(&p.id)).await;
    let selection = s
        .search_passages("booking", Some(std::slice::from_ref(&source.id)), 20)
        .await
        .unwrap()
        .remove(0)
        .source;
    let rejected = proposal(&s, &p, selection.clone()).await;
    s.decide_proposal(&rejected.proposals[0].id, false)
        .await
        .unwrap();
    assert_eq!(s.read(&p.id).await.unwrap().revision, 2);
    let accepted = proposal(&s, &p, selection).await;
    s.decide_proposal(&accepted.proposals[0].id, true)
        .await
        .unwrap();
    let current = s.read(&p.id).await.unwrap();
    s.put_draft(&p.id, 3, &current.document).await.unwrap();
    s.organize_page(&p.id, 1, "Renamed project", Some("getting-started"))
        .await
        .unwrap();
    let settings = SourceSettings {
        local_folder: dir.path().to_str().unwrap().into(),
        ..Default::default()
    };
    s.save_source_settings(&settings).await.unwrap();
    fs::create_dir(dir.path().join("chatgpt")).unwrap();
    fs::write(
        dir.path().join("chatgpt/fake-credentials"),
        "secret-test-marker",
    )
    .unwrap();
    let backup = dir.path().join("copy.wiki-backup");
    s.export_backup(backup.to_str().unwrap()).await.unwrap();
    assert!(s.export_backup(backup.to_str().unwrap()).await.is_err());
    let runtime = WorkspaceRuntime {
        root: dir.path().into(),
        active: "original".into(),
    };
    let copy = runtime.restore(backup.to_str().unwrap()).await.unwrap();
    assert_eq!(backup::selected(dir.path()).unwrap(), "original");
    let restored_path = backup::database(dir.path(), &copy.id).unwrap();
    let restored = Store::open(&restored_path).await.unwrap();
    assert_eq!(
        restored.read(&p.id).await.unwrap().document,
        current.document
    );
    assert!(restored.draft(&p.id).await.unwrap().is_some());
    assert_eq!(restored.imported_sources().await.unwrap().len(), 1);
    assert_eq!(
        restored.page_citations(&p.id, 3).await.unwrap()[0].quote,
        "00:18 Dana: Use the booking calendar."
    );
    let threads = restored.discussions(&p.id).await.unwrap();
    assert_eq!(threads.len(), 2);
    assert!(threads.iter().any(|d| d.proposals[0].status == "accepted"));
    assert!(threads.iter().any(|d| d.proposals[0].status == "rejected"));
    assert_eq!(restored.history(&p.id).await.unwrap().len(), 3);
    assert_eq!(
        restored.source_settings().await.unwrap(),
        SourceSettings::default()
    );
    assert!(!restored_path.parent().unwrap().join("chatgpt").exists());
    let original: Vec<u8> = sqlx::query("SELECT original_bytes FROM import_snapshots")
        .fetch_one(&restored.pool)
        .await
        .unwrap()
        .get("original_bytes");
    assert_eq!(original, b"00:18 Dana: Use the booking calendar.");
    runtime.select(&copy.id).unwrap();
    assert_eq!(backup::selected(dir.path()).unwrap(), copy.id);
    assert!(runtime.select("../escape").is_err());
    restored
        .organize_page(&p.id, 2, "Only the restored copy", None)
        .await
        .unwrap();
    assert_eq!(s.read(&p.id).await.unwrap().title, "Renamed project");
    runtime.select("original").unwrap();
}
#[tokio::test]
async fn damaged_or_foreign_backup_is_rejected_without_creating_a_copy() {
    use base64::{Engine, engine::general_purpose::STANDARD};
    use sha2::{Digest, Sha256};
    let (dir, s) = setup().await;
    let path = dir.path().join("good.wiki-backup");
    s.export_backup(path.to_str().unwrap()).await.unwrap();
    let runtime = WorkspaceRuntime {
        root: dir.path().into(),
        active: "original".into(),
    };
    let mut archive: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    archive["sha256"] = "wrong".into();
    let bad = dir.path().join("bad.wiki-backup");
    fs::write(&bad, serde_json::to_vec(&archive).unwrap()).unwrap();
    assert!(runtime.restore(bad.to_str().unwrap()).await.is_err());
    let db = dir.path().join("foreign.sqlite");
    fs::write(
        &db,
        STANDARD
            .decode(archive["database"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    let foreign = Store::open(&db).await.unwrap();
    sqlx::query(
        "CREATE TRIGGER foreign_trigger AFTER INSERT ON revisions BEGIN DELETE FROM drafts; END",
    )
    .execute(&foreign.pool)
    .await
    .unwrap();
    foreign.pool.close().await;
    let bytes = fs::read(&db).unwrap();
    archive["sha256"] = format!("{:x}", Sha256::digest(&bytes)).into();
    archive["database"] = STANDARD.encode(bytes).into();
    fs::write(&bad, serde_json::to_vec(&archive).unwrap()).unwrap();
    assert!(runtime.restore(bad.to_str().unwrap()).await.is_err());
    assert_eq!(runtime.copies().unwrap().len(), 1);
    assert_eq!(s.records().await.unwrap().len(), 2);
}

#[tokio::test]
async fn default_context_deduplicates_citations_and_bounds_multibyte_sources() {
    let (_dir, s) = setup().await;
    let p = page(&s, "Project", "Check the calendar.", None).await;
    let source = imported(
        &s,
        &format!("calendar {}", "記録🧪 ".repeat(7000)),
        Some(&p.id),
    )
    .await;
    let hit = s
        .search_passages("calendar", Some(std::slice::from_ref(&source.id)), 10)
        .await
        .unwrap()
        .remove(0);
    for base in [2, 3] {
        s.manual_citation(
            &uuid::Uuid::new_v4().to_string(),
            &p.id,
            base,
            &anchor("Check the calendar."),
            &hit.source,
        )
        .await
        .unwrap();
    }
    let context = s.default_evidence(&p.id, "").await.unwrap();
    let ids: std::collections::HashSet<_> = context.evidence.iter().map(|e| &e.id).collect();
    assert_eq!(ids.len(), context.evidence.len());
    assert!(
        context
            .evidence
            .iter()
            .map(|e| e.quote.len())
            .sum::<usize>()
            <= 12000
    );
    let d = s
        .create_discussion(
            &p.id,
            4,
            &anchor("Check the calendar."),
            "@agent Check the calendar.",
        )
        .await
        .unwrap();
    let selections: Vec<_> = context.evidence.iter().map(|e| e.source.clone()).collect();
    assert!(
        s.proposal_context(
            &d.id,
            "Check the calendar.",
            &selections,
            &anchor("Check the calendar."),
            4,
            None
        )
        .await
        .is_ok()
    );
}
