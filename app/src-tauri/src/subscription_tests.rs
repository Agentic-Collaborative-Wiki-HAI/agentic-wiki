use super::*;
use crate::agent::SourceExcerpt;

fn request() -> ProposalRequest {
    ProposalRequest {
        page_id: "guide".into(),
        base_revision: 1,
        block_id: "booking".into(),
        selected_quote: "Use the old spreadsheet".into(),
        human_request: "Check the booking instructions".into(),
        sources: vec![SourceExcerpt {
            id: "recording-18".into(),
            revision: 1,
            passage: "Use the current booking calendar".into(),
        }],
    }
}
#[test]
fn outbound_request_contains_only_explicit_context_and_no_tools() {
    let r = request();
    let body = request_body("selected-model", &r).unwrap();
    assert_eq!(body["model"], "selected-model");
    assert_eq!(body["tools"], json!([]));
    assert_eq!(body["store"], false);
    assert_eq!(body["stream"], true);
    assert_eq!(body["input"].as_array().unwrap().len(), 1);
    let context: Value =
        serde_json::from_str(body["input"][0]["content"].as_str().unwrap()).unwrap();
    assert_eq!(context, serde_json::to_value(r).unwrap());
    assert_eq!(body.as_object().unwrap().len(), 6);
    assert!(request_body("", &request()).is_err());
}
#[test]
fn callback_rejects_wrong_state_client_and_declined_consent() {
    let parse = |q| Url::parse(&format!("http://127.0.0.1:1234/auth/callback?{q}")).unwrap();
    let valid = parse("state=expected&code=test-code&client_id=oaiapp_test");
    assert_eq!(
        validate_callback(&valid, "expected", None).unwrap(),
        ("test-code".into(), "oaiapp_test".into())
    );
    assert!(validate_callback(&valid, "wrong", None).is_err());
    assert!(validate_callback(&valid, "expected", Some("oaiapp_other")).is_err());
    assert!(
        validate_callback(
            &parse("state=expected&error=access_denied"),
            "expected",
            None
        )
        .is_err()
    );
    assert!(validate_callback(&parse("state=expected&code=c"), "expected", None).is_err());
    assert!(
        validate_callback(
            &parse("state=expected&code=c&client_id=dynamic_agent_client"),
            "expected",
            None
        )
        .is_err()
    );
    assert!(
        validate_callback(
            &parse("state=expected&code=c&code=another&client_id=oaiapp_test"),
            "expected",
            None
        )
        .is_err()
    );
    assert_eq!(
        validate_callback(
            &parse("state=expected&code=c"),
            "expected",
            Some("oaiapp_test")
        )
        .unwrap()
        .1,
        "oaiapp_test"
    );
}
#[test]
fn auth_endpoints_cannot_redirect_credentials() {
    assert!(auth_endpoint("https://auth.openai.com/api/accounts/oauth/token").is_ok());
    for value in [
        "http://auth.openai.com/token",
        "https://auth.openai.com.evil.example/token",
        "https://attacker@auth.openai.com/token",
        "https://auth.openai.com:444/token",
    ] {
        assert!(auth_endpoint(value).is_err(), "{value}");
    }
}
#[test]
fn only_complete_validated_model_output_becomes_a_candidate() {
    let r = request();
    let candidate = json!({"proposedWording":"Use the current calendar.", "explanation":"The selected excerpt names the calendar.", "citedSourceIds":["recording-18"]}).to_string();
    let delta = json!({"type":"response.output_text.delta","delta":candidate}).to_string();
    let mut partial = ResponseStream::default();
    partial.event(&delta).unwrap();
    assert!(partial.candidate(&r).is_err());
    let mut complete = ResponseStream::default();
    complete.event(&delta).unwrap();
    complete
        .event(r#"{"type":"response.completed","response":{"status":"completed"}}"#)
        .unwrap();
    assert_eq!(
        complete.candidate(&r).unwrap().cited_source_ids,
        vec!["recording-18"]
    );
    let mut invented = ResponseStream::default();
    invented
        .event(&delta.replace("recording-18", "invented-source"))
        .unwrap();
    invented
        .event(r#"{"type":"response.completed","response":{"status":"completed"}}"#)
        .unwrap();
    assert!(invented.candidate(&r).is_err());
    for event in ["response.failed", "response.incomplete", "error"] {
        assert!(
            ResponseStream::default()
                .event(&json!({"type":event}).to_string())
                .is_err()
        );
    }
}
#[test]
fn credentials_require_plan_scope_and_support_refresh_rotation() {
    let identity = || Identity {
        sub: "test-user".into(),
        email: None,
        nonce: None,
    };
    let mut response = json!({"access_token":"test-access", "id_token":"test-id", "refresh_token":"test-refresh", "scope":SCOPES, "token_type":"Bearer", "expires_in":3600});
    let first = parse_credentials(&response, "oaiapp_test", identity(), None).unwrap();
    response["refresh_token"] = json!("test-rotated");
    let rotated = parse_credentials(&response, "oaiapp_test", identity(), Some(&first)).unwrap();
    assert_eq!(rotated.refresh_token.as_deref(), Some("test-rotated"));
    response.as_object_mut().unwrap().remove("refresh_token");
    assert!(parse_credentials(&response, "oaiapp_test", identity(), None).is_err());
    assert!(parse_credentials(&response, "oaiapp_test", identity(), Some(&rotated)).is_ok());
    response["scope"] = json!("openid profile email");
    assert!(parse_credentials(&response, "oaiapp_test", identity(), Some(&rotated)).is_err());
}
#[tokio::test]
async fn private_connection_persists_host_without_opening_codex() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("chatgpt");
    let first = Subscription::open(path.clone()).unwrap();
    assert!(!first.status().await.connected);
    let host = first.state.lock().await.saved.host_id.clone();
    let next = Subscription::open(path.clone()).unwrap();
    assert_eq!(next.state.lock().await.saved.host_id, host);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            std::fs::metadata(path.join("connection.json"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
}
#[tokio::test]
async fn cancelling_sign_in_drops_pending_work_and_preserves_connection() {
    let dir = tempfile::tempdir().unwrap();
    let subscription = Subscription::open(dir.path().join("chatgpt")).unwrap();
    subscription.state.lock().await.waiting = true;
    *subscription.login_task.lock().await = Some(tokio::spawn(std::future::pending()));
    subscription.cancel_sign_in().await;
    assert!(!subscription.status().await.waiting);
    assert!(subscription.login_task.lock().await.is_none());
    assert!(subscription.path.exists());
}

#[test]
fn identity_requires_signature_issuer_audience_expiry_and_nonce() {
    let fixture: Value =
        serde_json::from_str(include_str!("../tests/fixtures/identity-fixture.json")).unwrap();
    let keys: JwkSet = serde_json::from_value(json!({"keys":fixture["keys"]})).unwrap();
    let token = |name: &str| fixture["tokens"][name].as_str().unwrap();
    assert_eq!(
        verify_identity(&keys, token("valid"), "oaiapp_test", Some("expected"))
            .unwrap()
            .sub,
        "test-only-subject"
    );
    for name in ["expired", "wrong_audience", "wrong_issuer", "wrong_nonce"] {
        assert!(
            verify_identity(&keys, token(name), "oaiapp_test", Some("expected")).is_err(),
            "{name}"
        );
    }
    let mut forged = token("valid").as_bytes().to_vec();
    let last = forged.len() - 10;
    forged[last] = if forged[last] == b'A' { b'B' } else { b'A' };
    assert!(
        verify_identity(
            &keys,
            std::str::from_utf8(&forged).unwrap(),
            "oaiapp_test",
            Some("expected")
        )
        .is_err()
    );
}
