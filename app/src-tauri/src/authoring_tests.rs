use crate::{
    agent::ProposalCandidate,
    authoring::*,
    review::{CandidateSubmission, EvidenceSelection},
    store::Store,
};
async fn setup() -> (tempfile::TempDir, Store) {
    let d = tempfile::tempdir().unwrap();
    let s = Store::open(&d.path().join("wiki.sqlite")).await.unwrap();
    (d, s)
}
fn add_atom(doc: &mut RichDocument, atom: Inline) {
    if let Block::Paragraph { content, .. } = &mut doc.doc.content[1] {
        content.push(atom)
    } else {
        panic!("fixture paragraph")
    }
}
#[tokio::test]
async fn explicit_annotations_drafts_titles_and_instructions_survive_backup_and_restart() {
    let (dir, s) = setup().await;
    let mut p = s.read("getting-started").await.unwrap();
    let original = p.clone();
    let note = s
        .register_note(
            "Training",
            "Contact the equipment lead for hands-on training.",
        )
        .await
        .unwrap();
    let cite = s
        .register_citation(&EvidenceSelection {
            object_id: "orientation-recording".into(),
            revision: 1,
            block_id: "passage-18".into(),
            range: None,
        })
        .await
        .unwrap();
    assert!(
        s.references_for(&p.document).await.unwrap().is_empty(),
        "Registering alone never changes a page"
    );
    add_atom(
        &mut p.document,
        Inline::Note {
            attrs: NoteAttrs {
                note_id: note.id.clone(),
            },
        },
    );
    add_atom(
        &mut p.document,
        Inline::Citation {
            attrs: CitationAttrs {
                citation_id: cite.id.clone(),
            },
        },
    );
    s.put_edit_draft(&p.id, 1, &p.document, Some("Draft title"))
        .await
        .unwrap();
    assert_eq!(s.read(&p.id).await.unwrap().title, original.title);
    assert_eq!(
        s.draft(&p.id).await.unwrap().unwrap().title.as_deref(),
        Some("Draft title")
    );
    s.discard_draft(&p.id).await.unwrap();
    assert!(
        s.references_for(&s.read(&p.id).await.unwrap().document)
            .await
            .unwrap()
            .is_empty()
    );
    s.put_edit_draft(&p.id, 1, &p.document, Some("A useful guide"))
        .await
        .unwrap();
    let saved = s
        .save_edit(&p.id, 1, &p.document, Some("A useful guide"))
        .await
        .unwrap();
    assert_eq!(saved.title, "A useful guide");
    assert_eq!(s.revision(&p.id, 1).await.unwrap().title, original.title);
    assert_eq!(s.revision(&p.id, 2).await.unwrap().title, saved.title);
    assert!(
        s.save_edit(&p.id, 1, &p.document, Some("Stale title"))
            .await
            .is_err()
    );
    assert_eq!(s.read(&p.id).await.unwrap().title, saved.title);
    let refs = s.references_for(&saved.document).await.unwrap();
    assert_eq!(
        refs.iter().map(|r| r.kind.as_str()).collect::<Vec<_>>(),
        vec!["note", "citation"]
    );
    assert_eq!(refs[0].body, note.body);
    let instructions = AgentInstructions {
        proposal: "Keep the person's voice.".into(),
        chat: "Explain things simply.".into(),
    };
    s.save_instructions(&instructions).await.unwrap();
    let path = dir.path().join("annotations.wiki-backup");
    s.export_backup(path.to_str().unwrap()).await.unwrap();
    let runtime = crate::backup::WorkspaceRuntime {
        root: dir.path().into(),
        active: "original".into(),
    };
    let copy = runtime.restore(path.to_str().unwrap()).await.unwrap();
    let restored = Store::open(&crate::backup::database(dir.path(), &copy.id).unwrap())
        .await
        .unwrap();
    assert_eq!(
        restored.instructions().await.unwrap().proposal,
        instructions.proposal
    );
    assert_eq!(
        restored
            .references_for(&restored.read(&p.id).await.unwrap().document)
            .await
            .unwrap()[0]
            .body,
        note.body
    );
    s.pool.close().await;
    let reopened = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    assert_eq!(reopened.read(&p.id).await.unwrap().title, "A useful guide");
    assert_eq!(
        reopened.references_for(&saved.document).await.unwrap()[1]
            .evidence
            .as_ref()
            .unwrap()
            .source
            .revision,
        1
    );
}
#[tokio::test]
async fn source_free_rewrite_preserves_explicit_atoms_without_creating_references() {
    let (_dir, s) = setup().await;
    let mut p = s.read("getting-started").await.unwrap();
    let note = s.register_note("Context", "Keep this note.").await.unwrap();
    let cite = s
        .register_citation(&EvidenceSelection {
            object_id: "orientation-recording".into(),
            revision: 1,
            block_id: "passage-18".into(),
            range: None,
        })
        .await
        .unwrap();
    add_atom(
        &mut p.document,
        Inline::Note {
            attrs: NoteAttrs {
                note_id: note.id.clone(),
            },
        },
    );
    add_atom(
        &mut p.document,
        Inline::Citation {
            attrs: CitationAttrs {
                citation_id: cite.id.clone(),
            },
        },
    );
    s.save(&p.id, 1, &p.document).await.unwrap();
    let b = &p.document.doc.content[1];
    let quote = b.content().iter().map(Inline::plain).collect::<String>();
    let a = PassageAnchor {
        block_id: b.id().into(),
        from_utf16: 0,
        to_utf16: quote.encode_utf16().count(),
        quote,
        state: AnchorState::Attached,
    };
    let d = s
        .create_discussion(&p.id, 2, &a, "@agent Make this shorter.")
        .await
        .unwrap();
    let (request, evidence) = s
        .proposal_context(&d.id, "Make it shorter.", &[], &a, 2, None)
        .await
        .unwrap();
    assert!(request.sources.is_empty());
    let candidate = ProposalCandidate {
        proposed_wording: "Plan your first session with this guide.".into(),
        explanation: "Shortened the supplied wording.".into(),
        cited_source_ids: vec![],
    };
    let d = s
        .record_candidate(CandidateSubmission {
            thread_id: &d.id,
            request: &request,
            anchor: &a,
            evidence: &evidence,
            candidate: &candidate,
            parent_id: None,
            origin: "test fixture",
        })
        .await
        .unwrap();
    s.decide_proposal(&d.proposals[0].id, true).await.unwrap();
    let accepted = s.read(&p.id).await.unwrap();
    let entries = s.references_for(&accepted.document).await.unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].id, note.id);
    assert_eq!(entries[1].id, cite.id);
    assert_eq!(accepted.document.doc.content[1].content().len(), 3);
    assert!(
        s.discussion(&d.id).await.unwrap().proposals[0]
            .evidence
            .is_empty()
    );
}
#[tokio::test]
async fn invalid_notes_and_failed_title_save_leave_saved_content_unchanged() {
    let (_dir, s) = setup().await;
    let p = s.read("getting-started").await.unwrap();
    let mut invalid = p.document.clone();
    add_atom(
        &mut invalid,
        Inline::Note {
            attrs: NoteAttrs {
                note_id: "unknown".into(),
            },
        },
    );
    assert!(s.save(&p.id, 1, &invalid).await.is_err());
    assert!(s.put_edit_draft(&p.id, 1, &invalid, None).await.is_err());
    s.put_edit_draft(&p.id, 1, &p.document, Some("My draft"))
        .await
        .unwrap();
    sqlx::query("CREATE TRIGGER failed_title BEFORE UPDATE OF title ON objects BEGIN SELECT RAISE(ABORT,'injected'); END").execute(&s.pool).await.unwrap();
    assert!(
        s.save_edit(&p.id, 1, &p.document, Some("New title"))
            .await
            .is_err()
    );
    let unchanged = s.read(&p.id).await.unwrap();
    assert_eq!(unchanged.title, p.title);
    assert_eq!(unchanged.revision, 1);
    assert_eq!(
        s.draft(&p.id).await.unwrap().unwrap().title.as_deref(),
        Some("My draft")
    );
}
