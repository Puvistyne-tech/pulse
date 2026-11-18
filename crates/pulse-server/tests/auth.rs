mod common;

use axum::http::{Method, StatusCode};
use common::{api, api_raw, fresh_state, signed_up};
use pulse_protocol::api_path;
use pulse_protocol::fixtures::VALID_PASSWORD;
use rstest::rstest;
use serde_json::{Value, json};

const SEEDED_EMAIL: &str = "ada@example.org";

fn credentials(email: &str, password: &str) -> String {
    json!({"email": email, "password": password}).to_string()
}

fn registration(email: &str, display_name: &str, password: &str) -> String {
    json!({"email": email, "display_name": display_name, "password": password}).to_string()
}

fn error(code: &str, field: Option<&str>) -> Value {
    json!({"code": code, "field": field})
}

enum Outcome {
    Session { email: &'static str },
    Failure(Value),
}

#[rstest]
#[case::sign_up_valid(
    false,
    api_path::ACCOUNTS,
    registration("ada@example.org", "Ada", VALID_PASSWORD),
    StatusCode::CREATED,
    Outcome::Session { email: "ada@example.org" },
)]
#[case::sign_up_normalizes_mixed_case_email(
    false,
    api_path::ACCOUNTS,
    registration("  Ada@Example.ORG ", "Ada", VALID_PASSWORD),
    StatusCode::CREATED,
    Outcome::Session { email: "ada@example.org" },
)]
#[case::sign_up_invalid_email(
    false,
    api_path::ACCOUNTS,
    registration("not-an-email", "Ada", VALID_PASSWORD),
    StatusCode::BAD_REQUEST,
    Outcome::Failure(error("invalid_input", Some("email")))
)]
#[case::sign_up_invalid_display_name(
    false,
    api_path::ACCOUNTS,
    registration("ada@example.org", "   ", VALID_PASSWORD),
    StatusCode::BAD_REQUEST,
    Outcome::Failure(error("invalid_input", Some("display_name")))
)]
#[case::sign_up_short_password(
    false,
    api_path::ACCOUNTS,
    registration("ada@example.org", "Ada", "short"),
    StatusCode::BAD_REQUEST,
    Outcome::Failure(error("invalid_input", Some("password")))
)]
#[case::sign_up_duplicate_email(
    true,
    api_path::ACCOUNTS,
    registration("ada@example.org", "Other", VALID_PASSWORD),
    StatusCode::CONFLICT,
    Outcome::Failure(error("conflict", Some("email")))
)]
#[case::sign_up_duplicate_email_other_case(
    true,
    api_path::ACCOUNTS,
    registration("ADA@example.org", "Other", VALID_PASSWORD),
    StatusCode::CONFLICT,
    Outcome::Failure(error("conflict", Some("email")))
)]
#[case::sign_up_malformed_json(
    false,
    api_path::ACCOUNTS,
    "{".to_owned(),
    StatusCode::BAD_REQUEST,
    Outcome::Failure(error("invalid_input", None)),
)]
#[case::sign_in_valid(
    true,
    api_path::AUTH_TOKENS,
    credentials("ada@example.org", VALID_PASSWORD),
    StatusCode::OK,
    Outcome::Session { email: "ada@example.org" },
)]
#[case::sign_in_mixed_case_email(
    true,
    api_path::AUTH_TOKENS,
    credentials("Ada@EXAMPLE.org", VALID_PASSWORD),
    StatusCode::OK,
    Outcome::Session { email: "ada@example.org" },
)]
#[case::sign_in_wrong_password(
    true,
    api_path::AUTH_TOKENS,
    credentials("ada@example.org", "wrong-password-entirely"),
    StatusCode::UNAUTHORIZED,
    Outcome::Failure(error("unauthenticated", None))
)]
#[case::sign_in_unknown_email(
    false,
    api_path::AUTH_TOKENS,
    credentials("nobody@example.org", VALID_PASSWORD),
    StatusCode::UNAUTHORIZED,
    Outcome::Failure(error("unauthenticated", None))
)]
#[case::sign_in_invalid_email(
    false,
    api_path::AUTH_TOKENS,
    credentials("nope", VALID_PASSWORD),
    StatusCode::BAD_REQUEST,
    Outcome::Failure(error("invalid_input", Some("email")))
)]
#[case::sign_in_empty_password(
    false,
    api_path::AUTH_TOKENS,
    credentials("ada@example.org", ""),
    StatusCode::BAD_REQUEST,
    Outcome::Failure(error("invalid_input", Some("password")))
)]
#[case::sign_in_malformed_json(
    false,
    api_path::AUTH_TOKENS,
    "[]".to_owned(),
    StatusCode::BAD_REQUEST,
    Outcome::Failure(error("invalid_input", None)),
)]
#[tokio::test]
async fn credential_requests(
    #[case] seeded: bool,
    #[case] path: &str,
    #[case] body: String,
    #[case] status: StatusCode,
    #[case] outcome: Outcome,
) {
    let state = fresh_state().await;
    if seeded {
        signed_up(&state, SEEDED_EMAIL).await;
    }
    let (actual_status, actual_body) = api_raw(&state, Method::POST, path, None, Some(body)).await;
    assert_eq!(actual_status, status, "{actual_body}");
    match outcome {
        Outcome::Session { email } => {
            assert_eq!(actual_body["account"]["email"], json!(email));
            assert!(
                actual_body["token"]
                    .as_str()
                    .is_some_and(|token| !token.is_empty())
            );
            assert!(actual_body.get("password_hash").is_none());
        }
        Outcome::Failure(expected) => assert_eq!(actual_body, expected),
    }
}

#[tokio::test]
async fn sign_out_revokes_only_the_presented_token() {
    let state = fresh_state().await;
    let first = signed_up(&state, SEEDED_EMAIL).await;
    let (_, second_body) = api_raw(
        &state,
        Method::POST,
        api_path::AUTH_TOKENS,
        None,
        Some(credentials(SEEDED_EMAIL, VALID_PASSWORD)),
    )
    .await;
    let second: pulse_protocol::AuthSession =
        serde_json::from_value(second_body).expect("auth session");
    assert_ne!(first.token, second.token);
    let nothing: Option<&()> = None;
    let (status, body) = api(
        &state,
        Method::DELETE,
        api_path::CURRENT_AUTH_TOKEN,
        Some(&first.token),
        nothing,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(body, Value::Null);
    let (revoked, _) = api(
        &state,
        Method::GET,
        api_path::CAMPUSES,
        Some(&first.token),
        nothing,
    )
    .await;
    let (still_valid, _) = api(
        &state,
        Method::GET,
        api_path::CAMPUSES,
        Some(&second.token),
        nothing,
    )
    .await;
    assert_eq!(revoked, StatusCode::UNAUTHORIZED);
    assert_eq!(still_valid, StatusCode::OK);
}

#[tokio::test]
async fn sign_out_without_bearer_is_unauthenticated() {
    let state = fresh_state().await;
    let nothing: Option<&()> = None;
    let (status, body) = api(
        &state,
        Method::DELETE,
        api_path::CURRENT_AUTH_TOKEN,
        None,
        nothing,
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body, error("unauthenticated", None));
}
