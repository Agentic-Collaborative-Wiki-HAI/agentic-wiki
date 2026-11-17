use crate::{domain::*, imports::*, review::EvidenceSelection, sources, store::Store};
use std::fs;
fn source(text: &str) -> SourceBytes {
    SourceBytes {
        provider: SourceProvider::Local,
        locator: "/synthetic/meeting.txt".into(),
        name: "Meeting.txt".into(),
        url: None,
        modified: Some("fixture".into()),
        bytes: text.as_bytes().to_vec(),
    }
}
async fn store() -> (tempfile::TempDir, Store) {
    let dir = tempfile::tempdir().unwrap();
    let s = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    (dir, s)
}
#[test]
fn imports_text_without_inventing_speakers_or_executing_markup() {
    let (_,d)=parse_text("\u{feff}00:12 Dana: 🔬 Check the calendar.\r\nMei: Thanks.\r\n\r\n<script>not executable</script>".as_bytes()).unwrap();
    assert_eq!(d.doc.content.len(), 2);
    assert!(
        d.doc.content[0]
            .content()
            .iter()
            .any(|n| matches!(n, Inline::HardBreak))
    );
    assert!(
        d.doc.content[1]
            .content()
            .iter()
            .map(Inline::plain)
            .collect::<String>()
            .contains("<script>")
    );
    assert!(parse_text(&[0xff]).is_err());
    assert!(parse_text(b"\0bad").is_err());
    assert!(parse_text(b"  \n").is_err());
    assert!(parse_text(&vec![b'a'; MAX_TEXT_BYTES + 1]).is_err());
}
#[tokio::test]
async fn settings_and_folder_boundaries_survive_restart() {
    let (dir, s) = store().await;
    let folder = dir.path().join("source");
    fs::create_dir(&folder).unwrap();
    fs::write(folder.join("notes.txt"), b"A real local note.").unwrap();
    fs::write(folder.join("cloud.gdoc"), b"pointer").unwrap();
    fs::write(folder.join("audio.mp3"), [0, 1, 2]).unwrap();
    let settings = SourceSettings {
        local_folder: folder.to_str().unwrap().into(),
        ..Default::default()
    };
    s.save_source_settings(&settings).await.unwrap();
    let listed = sources::list_local(&settings).unwrap();
    assert_eq!(listed.items.len(), 3);
    assert_eq!(listed.items.iter().filter(|i| i.supported).count(), 1);
    assert_eq!(
        sources::read_local(&settings, "notes.txt").unwrap().bytes,
        b"A real local note."
    );
    for name in [
        "../notes.txt",
        "/tmp/notes.txt",
        "cloud.gdoc",
        "audio.mp3",
        "missing.txt",
        "sub/notes.txt",
    ] {
        assert!(sources::read_local(&settings, name).is_err(), "{name}");
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(folder.join("notes.txt"), folder.join("alias.txt")).unwrap();
        assert!(sources::read_local(&settings, "alias.txt").is_err());
    }
    let mut bad = settings.clone();
    bad.local_folder = folder.join("missing").to_string_lossy().into();
    assert!(s.save_source_settings(&bad).await.is_err());
    s.pool.close().await;
    let reopened = Store::open(&dir.path().join("wiki.sqlite")).await.unwrap();
    assert_eq!(
        reopened.source_settings().await.unwrap(),
        sources::validate_settings(&settings).unwrap()
    );
}
#[tokio::test]
async fn import_retry_refresh_history_and_original_bytes_are_atomic() {
    let (_dir, s) = store().await;
    let pending = s
        .preview_import(source(
            "00:18 Dana: Use the calendar.\n\n00:22 Mei: Understood.",
        ))
        .await
        .unwrap();
    assert!(s.imported_sources().await.unwrap().is_empty());
    let first = s
        .commit_import(
            &pending,
            "Meeting",
            ImportKind::Transcript,
            Some("getting-started"),
        )
        .await
        .unwrap();
    assert_eq!(first.revision, 1);
    assert_eq!(
        s.commit_import(&pending, "Meeting", ImportKind::Transcript, None)
            .await
            .unwrap()
            .id,
        first.id
    );
    let fresh = s
        .preview_import(source(
            "00:18 Dana: Use the calendar.\n\n00:22 Mei: Understood.",
        ))
        .await
        .unwrap();
    assert!(fresh.preview.unchanged);
    assert_eq!(
        s.commit_import(&fresh, "Meeting", ImportKind::Transcript, None)
            .await
            .unwrap()
            .revision,
        1
    );
    let bytes: Vec<u8> =
        sqlx::query_scalar("SELECT original_bytes FROM import_snapshots WHERE object_id=?")
            .bind(&first.id)
            .fetch_one(&s.pool)
            .await
            .unwrap();
    assert_eq!(bytes, pending.source.bytes);
    assert!(
        s.discussion_evidence("getting-started")
            .await
            .unwrap()
            .iter()
            .any(|e| e.source.object_id == first.id)
    );
    let old = EvidenceSelection {
        range: None,
        object_id: first.id.clone(),
        revision: 1,
        block_id: "import-1".into(),
    };
    let changed = s
        .preview_import(source("00:18 Dana: Use the new calendar."))
        .await
        .unwrap();
    assert!(!changed.preview.unchanged);
    let updated = s
        .commit_import(&changed, "Meeting", ImportKind::Transcript, None)
        .await
        .unwrap();
    assert_eq!(updated.id, first.id);
    assert_eq!(updated.revision, 2);
    assert_eq!(
        s.evidence(&old).await.unwrap().quote,
        "00:18 Dana: Use the calendar."
    );
    let stale = s
        .preview_import(source("Another source update."))
        .await
        .unwrap();
    let mut edited = updated.document.clone();
    edited
        .doc
        .content
        .push(paragraph("human", "A human correction."));
    s.save(&updated.id, 2, &edited).await.unwrap();
    assert!(matches!(
        s.commit_import(&stale, "Meeting", ImportKind::Transcript, None)
            .await,
        Err(Error::Stale)
    ));
    assert_eq!(s.read(&updated.id).await.unwrap().document, edited);
    let duplicate = s
        .preview_import(source("00:18 Dana: Use the new calendar."))
        .await
        .unwrap();
    assert!(duplicate.preview.unchanged);
    assert_eq!(
        s.commit_import(&duplicate, "Meeting", ImportKind::Transcript, None)
            .await
            .unwrap()
            .document,
        edited
    );
    assert_eq!(s.imported_sources().await.unwrap().len(), 1);
    assert_eq!(s.history(&first.id).await.unwrap().len(), 3);
}
#[tokio::test]
async fn failed_commit_has_no_orphan_and_duplicate_titles_remain_distinct() {
    let (_dir, s) = store().await;
    let first = s.preview_import(source("Meeting one")).await.unwrap();
    sqlx::query("CREATE TRIGGER fail_import BEFORE INSERT ON import_snapshots BEGIN SELECT RAISE(ABORT,'test failure'); END").execute(&s.pool).await.unwrap();
    assert!(
        s.commit_import(&first, "Meeting", ImportKind::Notes, None)
            .await
            .is_err()
    );
    assert!(s.imported_sources().await.unwrap().is_empty());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM objects")
            .fetch_one(&s.pool)
            .await
            .unwrap(),
        2
    );
    sqlx::query("DROP TRIGGER fail_import")
        .execute(&s.pool)
        .await
        .unwrap();
    let a = s
        .commit_import(&first, "Meeting", ImportKind::Notes, None)
        .await
        .unwrap();
    let mut other = source("Meeting two");
    other.locator = "/other/meeting.txt".into();
    let second = s.preview_import(other).await.unwrap();
    let b = s
        .commit_import(&second, "Meeting", ImportKind::Notes, None)
        .await
        .unwrap();
    assert_ne!(a.id, b.id);
    let reclassified = s.preview_import(source("Meeting one")).await.unwrap();
    assert!(
        s.commit_import(&reclassified, "Meeting", ImportKind::Transcript, None)
            .await
            .is_err()
    );
}

#[cfg(unix)]
fn fake_drive(dir: &std::path::Path, mode: &str) -> SourceSettings {
    let scripts = dir.join("use-google-drive/scripts");
    fs::create_dir_all(&scripts).unwrap();
    fs::write(scripts.join("mode"), mode).unwrap();
    fs::write(
        scripts.join("export.txt"),
        format!(
            "00:01 Dana: {}\n\nEND OF FULL EXPORT",
            "Full transcript. ".repeat(2000)
        ),
    )
    .unwrap();
    fs::write(scripts.join("drive"),r##"#!/bin/bash
set -eu
cd "$(dirname "$0")"
printf '%s\n' "$1" >> calls
mode=$(cat mode)
[ "$mode" != "fail" ] || exit 7
case "$1" in
list) printf '%s' '{"files":[{"id":"doc1","name":"Meeting","mimeType":"application/vnd.google-apps.document","parents":["folder1"]}],"nextPageToken":"more"}' ;;
meta)
  if [ "$3" = folder1 ]; then printf '%s' '{"id":"folder1","name":"Meet","mimeType":"application/vnd.google-apps.folder","parents":[]}';exit 0;fi
  if [ "$3" = meeting1 ]; then printf '%s' '{"id":"meeting1","name":"Meeting folder","mimeType":"application/vnd.google-apps.folder","parents":["folder1"]}';exit 0;fi
  if [ "$mode" = nested ]; then printf '%s' '{"id":"doc1","name":"Meeting","mimeType":"application/vnd.google-apps.document","modifiedTime":"1","parents":["meeting1"]}';exit 0;fi
  [ "$mode" != "bad-json" ] || { echo 'not-json';exit 0; }
  version=1
  if [ "$mode" = "changed" ]; then if [ -f seen ]; then version=2; else touch seen; fi; fi
  printf '{"id":"doc1","name":"Meeting","mimeType":"application/vnd.google-apps.document","modifiedTime":"%s","parents":["folder1"]}' "$version"
  ;;
download)
  [ "$4" = "--out" ] && [ "$6" = "--mime" ] && [ "$7" = "text/plain" ]
  cp export.txt "$5"
  ;;
*) exit 9 ;;
esac
"##).unwrap();
    SourceSettings {
        agent_hands_path: scripts.join("drive").to_string_lossy().into(),
        google_account: "example".into(),
        drive_folder: "folder1".into(),
        local_folder: String::new(),
    }
}
#[cfg(unix)]
#[tokio::test]
async fn drive_adapter_exports_full_text_and_reports_partial_listing() {
    let dir = tempfile::tempdir().unwrap();
    let mut settings = fake_drive(dir.path(), "ok");
    settings.local_folder = "/missing-local-folder".into(); // The Drive route must work independently of local sync.
    let list = sources::list_drive(&settings, None).await.unwrap();
    assert_eq!(list.items.len(), 1);
    assert!(list.notice.contains("not the full folder"));
    let source = sources::read_drive(&settings, "https://docs.google.com/document/d/doc1/edit")
        .await
        .unwrap();
    assert!(source.bytes.len() > 20_000);
    assert!(source.bytes.ends_with(b"END OF FULL EXPORT"));
    assert_eq!(
        source.url.as_deref(),
        Some("https://docs.google.com/document/d/doc1/edit")
    );
    let calls = fs::read_to_string(dir.path().join("use-google-drive/scripts/calls")).unwrap();
    assert_eq!(calls, "meta\nlist\nmeta\ndownload\nmeta\n");
    settings.drive_folder = "wrong-folder".into();
    assert!(sources::read_drive(&settings, "doc1").await.is_err());
    assert!(sources::drive_id("https://evil.example/doc1").is_err());
}
#[cfg(unix)]
#[tokio::test]
async fn drive_adapter_rejects_export_changes_failures_and_invalid_metadata() {
    for mode in ["changed", "fail", "bad-json"] {
        let dir = tempfile::tempdir().unwrap();
        let settings = fake_drive(dir.path(), mode);
        assert!(
            sources::read_drive(&settings, "doc1").await.is_err(),
            "{mode}"
        );
    }
}

#[cfg(unix)]
#[tokio::test]
async fn drive_adapter_verifies_nested_folder_ancestry_before_listing_and_export() {
    let dir = tempfile::tempdir().unwrap();
    let mut settings = fake_drive(dir.path(), "nested");
    assert!(
        sources::list_drive(&settings, Some("meeting1"))
            .await
            .is_ok()
    );
    assert!(sources::list_drive(&settings, Some("doc1")).await.is_err());
    let source = sources::read_drive(&settings, "doc1").await.unwrap();
    assert!(source.bytes.ends_with(b"END OF FULL EXPORT"));
    fs::write(dir.path().join("use-google-drive/scripts/calls"), "").unwrap();
    settings.drive_folder = "different-root".into();
    assert!(
        sources::list_drive(&settings, Some("meeting1"))
            .await
            .is_err()
    );
    assert!(sources::read_drive(&settings, "doc1").await.is_err());
    let calls = fs::read_to_string(dir.path().join("use-google-drive/scripts/calls")).unwrap();
    assert!(!calls.contains("list"));
    assert!(!calls.contains("download"));
}
