use super::*;

fn anchor() -> PassageAnchor {
    let quote = "Reserve the microscope using the shared spreadsheet before your first session.";
    PassageAnchor {
        block_id: "booking".into(),
        from_utf16: 0,
        to_utf16: quote.encode_utf16().count(),
        quote: quote.into(),
        state: AnchorState::Attached,
    }
}
async fn pending(store: &Store) -> Discussion {
    let a = anchor();
    let d = store
        .create_discussion(
            "getting-started",
            1,
            &a,
            "Please check this against the orientation recording.",
        )
        .await
        .unwrap();
    candidate(store, &d, None, &a, 1).await
}
async fn candidate(
    store: &Store,
    d: &Discussion,
    parent: Option<&str>,
    a: &PassageAnchor,
    base: i32,
) -> Discussion {
    let source = EvidenceSelection {
        object_id: "orientation-recording".into(),
        revision: 1,
        block_id: "passage-18".into(),
    };
    let (request, evidence) = store
        .proposal_context(
            &d.id,
            "Use the supplied recording to check the booking instructions.",
            &[source],
            a,
            base,
        )
        .await
        .unwrap();
    let candidate = ProposalCandidate {
        proposed_wording:
            "Check the equipment page for the current booking calendar before your first session."
                .into(),
        explanation:
            "The orientation transcript points to the booking calendar on the equipment page."
                .into(),
        cited_source_ids: vec![evidence[0].id.clone()],
    };
    store
        .record_candidate(CandidateSubmission {
            thread_id: &d.id,
            request: &request,
            anchor: a,
            evidence: &evidence,
            candidate: &candidate,
            parent_id: parent,
            origin: "test fixture; no model call",
        })
        .await
        .unwrap()
}
async fn setup() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    (dir, store)
}
#[tokio::test]
async fn accept_commits_wording_citation_decision_and_survives_restart() {
    let (dir, s) = setup().await;
    let d = pending(&s).await;
    let pid = &d.proposals[0].id;
    let accepted = s.decide_proposal(pid, true).await.unwrap();
    assert_eq!(accepted.proposals[0].status, "accepted");
    assert_eq!(accepted.proposals[0].published_revision, Some(2));
    assert!(!accepted.resolved);
    let page = s.read("getting-started").await.unwrap();
    assert_eq!(page.revision, 2);
    let cites = s.page_citations(&page.id, 2).await.unwrap();
    assert_eq!(cites.len(), 1);
    assert_eq!(cites[0].source.revision, 1);
    let block = page
        .document
        .doc
        .content
        .iter()
        .find(|b| b.id() == "booking")
        .unwrap();
    assert_eq!(
        block.content()[0].plain(),
        accepted.proposals[0].candidate.proposed_wording
    );
    assert!(matches!(block.content()[1], Inline::Citation { .. }));
    s.decide_proposal(pid, true).await.unwrap();
    assert_eq!(s.history(&page.id).await.unwrap().len(), 2);
    drop(s);
    let reopened = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    assert_eq!(
        reopened.read(&page.id).await.unwrap().document,
        page.document
    );
    let again = reopened.discussion(&d.id).await.unwrap();
    assert_eq!(again.messages.len(), 1);
    assert_eq!(again.proposals[0].status, "accepted");
    assert_eq!(
        reopened.page_citations(&page.id, 2).await.unwrap()[0].quote,
        cites[0].quote
    );
}
#[tokio::test]
async fn rejection_and_resolution_do_not_publish_or_accept() {
    let (_dir, s) = setup().await;
    let before = s.read("getting-started").await.unwrap();
    let d = pending(&s).await;
    let resolved = s.resolve_discussion(&d.id, true).await.unwrap();
    assert!(resolved.resolved);
    assert_eq!(resolved.proposals[0].status, "pending");
    s.reply(&d.id, "I do not think the source supports this change.")
        .await
        .unwrap();
    let result = s.decide_proposal(&d.proposals[0].id, false).await.unwrap();
    assert_eq!(result.messages.len(), 2);
    assert_eq!(result.proposals[0].published_revision, None);
    assert_eq!(s.read(&before.id).await.unwrap().document, before.document);
    assert_eq!(s.history(&before.id).await.unwrap().len(), 1);
    assert!(s.page_citations(&before.id, 1).await.unwrap().is_empty());
    s.decide_proposal(&d.proposals[0].id, false).await.unwrap();
    assert!(s.decide_proposal(&d.proposals[0].id, true).await.is_err());
}
#[tokio::test]
async fn stale_requires_explicit_refresh_and_rereview() {
    let (_dir, s) = setup().await;
    let d = pending(&s).await;
    let mut page = s.read("getting-started").await.unwrap();
    page.document
        .doc
        .content
        .push(paragraph("extra", "An unrelated edit."));
    s.save(&page.id, 1, &page.document).await.unwrap();
    assert!(matches!(
        s.decide_proposal(&d.proposals[0].id, true).await,
        Err(Error::Stale)
    ));
    let stale = s.discussion(&d.id).await.unwrap();
    assert!(stale.proposals[0].stale);
    let refreshed = candidate(&s, &stale, Some(&d.proposals[0].id), &anchor(), 2).await;
    assert_eq!(refreshed.proposals[0].status, "superseded");
    assert_eq!(refreshed.proposals[1].base_revision, 2);
    assert!(!refreshed.proposals[1].stale);
    s.decide_proposal(&refreshed.proposals[1].id, true)
        .await
        .unwrap();
    assert_eq!(s.read(&page.id).await.unwrap().revision, 3);
}
#[tokio::test]
async fn human_revision_preserves_original_candidate_and_provenance() {
    let (_dir, s) = setup().await;
    let d = pending(&s).await;
    let original = d.proposals[0].candidate.proposed_wording.clone();
    let revised = s
        .revise_proposal(
            &d.proposals[0].id,
            "Check the equipment page for booking instructions.",
            "Keep the wording concise.",
        )
        .await
        .unwrap();
    assert_eq!(revised.proposals[0].candidate.proposed_wording, original);
    assert_eq!(
        revised.proposals[1].parent_id,
        Some(d.proposals[0].id.clone())
    );
    assert_eq!(revised.proposals[1].origin, "human revision");
    assert_eq!(
        revised.proposals[1].evidence[0].id,
        d.proposals[0].evidence[0].id
    );
    assert!(s.decide_proposal(&d.proposals[0].id, true).await.is_err());
    s.decide_proposal(&revised.proposals[1].id, true)
        .await
        .unwrap();
}
#[tokio::test]
async fn failed_decision_rolls_back_content_citation_and_keeps_draft() {
    let (_dir, s) = setup().await;
    let page = s.read("getting-started").await.unwrap();
    s.put_draft(&page.id, 1, &page.document).await.unwrap();
    let d = pending(&s).await;
    sqlx::query("CREATE TRIGGER fail_review BEFORE INSERT ON review_decisions BEGIN SELECT RAISE(ABORT,'deliberate test failure'); END;").execute(&s.pool).await.unwrap();
    assert!(s.decide_proposal(&d.proposals[0].id, true).await.is_err());
    assert_eq!(s.read(&page.id).await.unwrap().revision, 1);
    assert_eq!(
        s.discussion(&d.id).await.unwrap().proposals[0].status,
        "pending"
    );
    assert!(s.draft(&page.id).await.unwrap().is_some());
    let n: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM citations")
        .fetch_one(&s.pool)
        .await
        .unwrap();
    assert_eq!(n, 0);
    sqlx::query("DROP TRIGGER fail_review")
        .execute(&s.pool)
        .await
        .unwrap();
    s.decide_proposal(&d.proposals[0].id, true).await.unwrap();
    let draft = s.draft(&page.id).await.unwrap().unwrap();
    assert_eq!(draft.base_revision, 1);
    assert!(matches!(
        s.save(&page.id, 1, &draft.document).await,
        Err(Error::Stale)
    ));
}
#[tokio::test]
async fn source_edits_do_not_change_pinned_evidence() {
    let (_dir, s) = setup().await;
    let d = pending(&s).await;
    let mut source = s.read("orientation-recording").await.unwrap();
    source.document.doc.content[1] = paragraph("passage-18", "Changed source text.");
    s.save(&source.id, 1, &source.document).await.unwrap();
    s.decide_proposal(&d.proposals[0].id, true).await.unwrap();
    let e = s.page_citations("getting-started", 2).await.unwrap();
    assert_eq!(e[0].source.revision, 1);
    assert!(e[0].quote.contains("booking calendar"));
    assert_eq!(s.revision(&source.id, 1).await.unwrap().revision, 1);
}
#[tokio::test]
async fn rejects_invalid_selection_unknown_evidence_and_dangling_citations() {
    let (_dir, s) = setup().await;
    let mut a = anchor();
    a.quote = "Wrong quote".into();
    assert!(
        s.create_discussion("getting-started", 1, &a, "Comment")
            .await
            .is_err()
    );
    let mut page = s.read("getting-started").await.unwrap();
    if let Block::Paragraph { content, .. } = &mut page.document.doc.content[1] {
        content.push(Inline::Citation {
            attrs: CitationAttrs {
                citation_id: "invented".into(),
            },
        })
    }
    assert!(s.save(&page.id, 1, &page.document).await.is_err());
    assert!(s.put_draft(&page.id, 1, &page.document).await.is_err());
    assert!(
        s.evidence(&EvidenceSelection {
            object_id: "orientation-recording".into(),
            revision: 99,
            block_id: "passage-18".into()
        })
        .await
        .is_err()
    );
}
#[tokio::test]
async fn concurrent_repeated_acceptance_publishes_once() {
    let (_dir, s) = setup().await;
    let d = pending(&s).await;
    let id = &d.proposals[0].id;
    let (a, b) = tokio::join!(s.decide_proposal(id, true), s.decide_proposal(id, true));
    assert!(a.is_ok() && b.is_ok());
    assert_eq!(s.history("getting-started").await.unwrap().len(), 2);
}

#[tokio::test]
async fn blank_editor_paragraphs_do_not_break_source_selection() {
    let (_dir, s) = setup().await;
    let mut page = s.read("getting-started").await.unwrap();
    page.document.doc.content.push(Block::Paragraph {
        attrs: BlockAttrs {
            block_id: "blank".into(),
        },
        content: vec![],
    });
    s.save(&page.id, 1, &page.document).await.unwrap();
    let sources = s.available_evidence().await.unwrap();
    assert!(
        sources
            .iter()
            .any(|e| e.source.object_id == "orientation-recording")
    );
    assert!(!sources.iter().any(|e| e.source.block_id == "blank"));
}
