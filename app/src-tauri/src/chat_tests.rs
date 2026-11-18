use crate::{chat::*, domain::*, store::Store};
fn answer(t: &ChatTurn) -> ChatAnswer {
    ChatAnswer {
        title: "Booking guidance".into(),
        paragraphs: vec![AnswerParagraph {
            text: "Use the booking calendar linked from the equipment page.".into(),
            cited_source_ids: vec![
                t.evidence
                    .iter()
                    .find(|e| e.source.object_id == "orientation-recording")
                    .unwrap()
                    .id
                    .clone(),
            ],
        }],
        missing_evidence: "The excerpts do not identify tomorrow's available time slots.".into(),
    }
}
async fn pending(s: &Store) -> ChatTurn {
    let context = s
        .chat_context("getting-started", "booking calendar")
        .await
        .unwrap();
    s.prepare_chat(
        &uuid::Uuid::new_v4().to_string(),
        "getting-started",
        "Where do I book a session?",
        &context
            .evidence
            .into_iter()
            .map(|e| e.source)
            .collect::<Vec<_>>(),
        "fixture",
    )
    .await
    .unwrap()
}
#[tokio::test]
async fn context_is_bounded_and_does_not_search_unlinked_pages() {
    let dir = tempfile::tempdir().unwrap();
    let s = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    let other = s
        .create_page(
            &uuid::Uuid::new_v4().to_string(),
            "Private unrelated topic",
            None,
        )
        .await
        .unwrap();
    let ctx = s.chat_context("getting-started", "booking").await.unwrap();
    assert!(
        ctx.evidence
            .iter()
            .any(|e| e.source.object_id == "getting-started")
    );
    assert!(
        ctx.evidence
            .iter()
            .any(|e| e.source.object_id == "orientation-recording")
    );
    assert!(!ctx.linked_ids.contains(&other.id));
    assert!(!ctx.evidence.iter().any(|e| e.source.object_id == other.id));
    let t = pending(&s).await;
    assert!(t.request.validate().is_ok());
    assert!(s.chat_turns().await.unwrap().len() == 1);
    let body = crate::subscription::chat_request_body("fixture", &t.request).unwrap();
    assert_eq!(body["tools"], serde_json::json!([]));
    assert_eq!(body["store"], false);
    assert_eq!(body["input"].as_array().unwrap().len(), 1);
    assert!(!body.to_string().contains(&other.title));
}
#[tokio::test]
async fn rejects_unselected_citations_and_accepts_honest_missing_evidence() {
    let dir = tempfile::tempdir().unwrap();
    let s = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    let t = pending(&s).await;
    let mut a = answer(&t);
    a.paragraphs[0].cited_source_ids = vec!["invented".into()];
    assert!(s.complete_chat(&t.id, &a).await.is_err());
    a.paragraphs[0].cited_source_ids.clear();
    assert!(s.complete_chat(&t.id, &a).await.is_err());
    a.paragraphs.clear();
    a.missing_evidence = "The selected sources do not answer this question.".into();
    let done = s.complete_chat(&t.id, &a).await.unwrap();
    assert_eq!(done.status, "completed");
    assert!(s.create_answer_draft(&t.id).await.is_err());
    assert!(s.complete_chat(&t.id, &a).await.is_err());
    assert_eq!(s.read("getting-started").await.unwrap().revision, 1);
}
#[tokio::test]
async fn cancelled_and_interrupted_requests_never_publish_a_late_answer() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("wiki.sqlite");
    let s = Store::open(&path).await.unwrap();
    let t = pending(&s).await;
    s.fail_chat(&t.id, true, "Cancelled").await.unwrap();
    assert!(s.complete_chat(&t.id, &answer(&t)).await.is_err());
    let interrupted = pending(&s).await;
    s.pool.close().await;
    let s = Store::open(&path).await.unwrap();
    assert_eq!(s.chat_turn(&interrupted.id).await.unwrap().status, "failed");
    assert!(
        s.complete_chat(&interrupted.id, &answer(&interrupted))
            .await
            .is_err()
    );
}
#[tokio::test]
async fn draft_is_editable_durable_and_unpublished_until_explicit_save() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("wiki.sqlite");
    let s = Store::open(&path).await.unwrap();
    let t = pending(&s).await;
    s.complete_chat(&t.id, &answer(&t)).await.unwrap();
    let draft = s.create_answer_draft(&t.id).await.unwrap();
    assert_eq!(s.create_answer_draft(&t.id).await.unwrap().id, draft.id);
    assert!(s.read(&draft.id).await.is_err());
    assert!(!s.records().await.unwrap().iter().any(|r| r.id == draft.id));
    assert!(
        s.search_passages(
            "Booking guidance",
            Some(std::slice::from_ref(&draft.id)),
            10
        )
        .await
        .unwrap()
        .is_empty()
    );
    let mut doc = draft.document.clone();
    if let Block::Paragraph { content, .. } = &mut doc.doc.content[0] {
        content.insert(0, text("Before arranging training: "));
    }
    let edited = s
        .save_answer_draft(
            &draft.id,
            draft.version,
            "Prepare for a session",
            None,
            &doc,
        )
        .await
        .unwrap();
    assert!(
        s.save_answer_draft(&draft.id, draft.version, "Stale", None, &doc)
            .await
            .is_err()
    );
    assert!(
        s.publish_answer_draft(&draft.id, draft.version)
            .await
            .is_err()
    );
    s.pool.close().await;
    let s = Store::open(&path).await.unwrap();
    let restored = s.answer_draft(&draft.id).await.unwrap();
    assert_eq!(restored.document, doc);
    assert_eq!(restored.version, edited.version);
    assert_eq!(restored.evidence.len(), 1);
    assert_eq!(
        restored.evidence[0].source.object_id,
        "orientation-recording"
    );
    let p = s
        .publish_answer_draft(&draft.id, restored.version)
        .await
        .unwrap();
    assert_eq!(p.document, doc);
    assert_eq!(p.title, "Prepare for a session");
    assert_eq!(p.revision, 1);
    assert_eq!(
        s.publish_answer_draft(&draft.id, restored.version)
            .await
            .unwrap()
            .revision,
        1
    );
    assert!(
        s.save_answer_draft(&draft.id, restored.version, "Cannot overwrite", None, &doc)
            .await
            .is_err()
    );
    assert_eq!(
        s.page_citations(&p.id, 1).await.unwrap()[0].quote,
        restored.evidence[0].quote
    );
    assert_eq!(
        s.history(&p.id).await.unwrap()[0].actor_id,
        "local-operator"
    );
    assert_eq!(s.chat_page_origin(&p.id).await.unwrap().unwrap().id, t.id);
    let next = pending(&s).await;
    assert_eq!(next.request.history.len(), 1);
}
#[tokio::test]
async fn publication_rolls_back_as_a_unit_and_preserves_the_draft() {
    let dir = tempfile::tempdir().unwrap();
    let s = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    let t = pending(&s).await;
    s.complete_chat(&t.id, &answer(&t)).await.unwrap();
    let d = s.create_answer_draft(&t.id).await.unwrap();
    sqlx::query("CREATE TRIGGER fail_publication BEFORE INSERT ON revisions WHEN NEW.actor_id='local-operator' BEGIN SELECT RAISE(ABORT,'test failure'); END;").execute(&s.pool).await.unwrap();
    assert!(s.publish_answer_draft(&d.id, d.version).await.is_err());
    assert!(s.read(&d.id).await.is_err());
    assert!(
        s.answer_draft(&d.id)
            .await
            .unwrap()
            .published_page_id
            .is_none()
    );
    sqlx::query("DROP TRIGGER fail_publication")
        .execute(&s.pool)
        .await
        .unwrap();
    assert!(s.publish_answer_draft(&d.id, d.version).await.is_ok());
}

#[tokio::test]
async fn backup_recovers_answers_and_unpublished_drafts_with_provenance() {
    use crate::backup::{self, WorkspaceRuntime};
    let dir = tempfile::tempdir().unwrap();
    let s = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    let t = pending(&s).await;
    s.complete_chat(&t.id, &answer(&t)).await.unwrap();
    let d = s.create_answer_draft(&t.id).await.unwrap();
    let path = dir.path().join("chat.wiki-backup");
    s.export_backup(path.to_str().unwrap()).await.unwrap();
    let runtime = WorkspaceRuntime {
        root: dir.path().into(),
        active: "original".into(),
    };
    let copy = runtime.restore(path.to_str().unwrap()).await.unwrap();
    let restored = Store::open(&backup::database(dir.path(), &copy.id).unwrap())
        .await
        .unwrap();
    assert_eq!(
        restored.answer_draft(&d.id).await.unwrap().document,
        d.document
    );
    assert_eq!(restored.chat_turn(&t.id).await.unwrap().status, "completed");
    assert!(restored.read(&d.id).await.is_err());
    let page = restored
        .publish_answer_draft(&d.id, d.version)
        .await
        .unwrap();
    assert_eq!(
        restored
            .chat_page_origin(&page.id)
            .await
            .unwrap()
            .unwrap()
            .id,
        t.id
    );
    assert!(s.read(&d.id).await.is_err());
}
