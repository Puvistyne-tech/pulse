mod common;

use std::time::Duration;

use axum::http::{Method, StatusCode};
use common::{api, campus_for, campuses_of, count_rows, fresh_state, grant_role, signed_up};
use futures_util::future::join_all;
use pulse_protocol::AuthToken;
use pulse_protocol::fixtures::CAMPUS_B;
use pulse_protocol::{
    CampusCreated, CampusId, CampusInvite, DEFAULT_ROOM_SLUGS, INVITE_SECRET_CHARS, InviteCode,
    JoinCampusRequest, Role, api_path,
};
use pulse_server::AppState;
use rstest::rstest;
use serde_json::{Value, json};
use tokio::time::timeout;

const CONCURRENT_JOINS: usize = 8;

async fn invite_request(
    state: &AppState,
    token: &AuthToken,
    campus_id: CampusId,
) -> (StatusCode, Value) {
    let nothing: Option<&()> = None;
    api(
        state,
        Method::POST,
        &api_path::campus_invites(campus_id),
        Some(token),
        nothing,
    )
    .await
}

async fn issued_invite(state: &AppState, token: &AuthToken, campus_id: CampusId) -> CampusInvite {
    let (status, body) = invite_request(state, token, campus_id).await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    serde_json::from_value(body).expect("campus invite")
}

async fn join_request(state: &AppState, token: &AuthToken, code: String) -> (StatusCode, Value) {
    api(
        state,
        Method::POST,
        api_path::CAMPUS_MEMBERSHIPS,
        Some(token),
        Some(&JoinCampusRequest { code }),
    )
    .await
}

#[rstest]
#[case::owner(None, StatusCode::CREATED)]
#[case::admin(Some(Role::Admin), StatusCode::CREATED)]
#[case::member(Some(Role::Member), StatusCode::FORBIDDEN)]
#[case::guest(Some(Role::Guest), StatusCode::FORBIDDEN)]
#[case::steward(Some(Role::Steward), StatusCode::FORBIDDEN)]
#[tokio::test]
async fn invite_creation_requires_member_management(
    #[case] role: Option<Role>,
    #[case] status: StatusCode,
) {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let other = signed_up(&state, "other@example.org").await;
    let campus_id = campus_for(&state, &owner.token, "north-quad")
        .await
        .campus
        .campus_id;
    let actor = match role {
        None => &owner,
        Some(role) => {
            grant_role(&state, campus_id, other.account.account_id, role).await;
            &other
        }
    };
    let (actual, body) = invite_request(&state, &actor.token, campus_id).await;
    assert_eq!(actual, status, "{body}");
    let expected_rows = i64::from(status == StatusCode::CREATED);
    assert_eq!(
        count_rows(&state, "campus_invites", campus_id).await,
        expected_rows
    );
    if status == StatusCode::CREATED {
        let invite: CampusInvite = serde_json::from_value(body).expect("campus invite");
        assert_eq!(invite.campus_id, campus_id);
        assert!(invite.code.starts_with(&format!("{campus_id}.")));
    }
}

#[tokio::test]
async fn invite_creation_hides_unknown_campus() {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    campus_for(&state, &owner.token, "north-quad").await;
    let (status, body) = invite_request(&state, &owner.token, CAMPUS_B).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({"code": "not_found", "field": null}));
}

#[derive(Clone, Copy)]
enum Code {
    Issued,
    Expired,
    WrongSecret,
    OtherCampus,
    Malformed,
}

#[rstest]
#[case::issued(Code::Issued, StatusCode::OK)]
#[case::expired(Code::Expired, StatusCode::BAD_REQUEST)]
#[case::wrong_secret(Code::WrongSecret, StatusCode::BAD_REQUEST)]
#[case::other_campus(Code::OtherCampus, StatusCode::BAD_REQUEST)]
#[case::malformed(Code::Malformed, StatusCode::BAD_REQUEST)]
#[tokio::test]
async fn joining_with_invite_code(#[case] code: Code, #[case] status: StatusCode) {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let joiner = signed_up(&state, "joiner@example.org").await;
    let campus_id = campus_for(&state, &owner.token, "north-quad")
        .await
        .campus
        .campus_id;
    let foreign = campus_for(&state, &owner.token, "south-quad")
        .await
        .campus
        .campus_id;
    let invite = issued_invite(&state, &owner.token, campus_id).await;
    if matches!(code, Code::Expired) {
        sqlx::query(
            "UPDATE campus_invites SET expires_at = now() - interval '1 second' \
             WHERE campus_id = $1",
        )
        .bind(campus_id.as_uuid())
        .execute(&state.db)
        .await
        .expect("expire invite");
    }
    let secret = invite.code.split_once('.').expect("separator").1.to_owned();
    let raw = match code {
        Code::Issued | Code::Expired => invite.code.clone(),
        Code::WrongSecret => InviteCode {
            campus_id,
            secret: "z".repeat(INVITE_SECRET_CHARS),
        }
        .render(),
        Code::OtherCampus => InviteCode {
            campus_id: foreign,
            secret,
        }
        .render(),
        Code::Malformed => "not-an-invite".to_owned(),
    };
    let (actual, body) = join_request(&state, &joiner.token, raw).await;
    assert_eq!(actual, status, "{body}");
    let joined: Vec<CampusId> = campuses_of(&state, &joiner.token)
        .await
        .campuses
        .iter()
        .map(|campus| campus.campus_id)
        .collect();
    if status == StatusCode::OK {
        let created: CampusCreated = serde_json::from_value(body).expect("joined campus");
        assert_eq!(created.campus.campus_id, campus_id);
        assert_eq!(created.campus.role, Role::Member);
        assert_eq!(
            created
                .rooms
                .iter()
                .map(|room| room.slug.as_str())
                .collect::<Vec<_>>(),
            DEFAULT_ROOM_SLUGS
        );
        assert_eq!(joined, vec![campus_id]);
    } else {
        assert_eq!(
            body,
            json!({"code": "invalid_input", "field": "invite_code"})
        );
        assert!(joined.is_empty());
    }
}

#[tokio::test]
async fn joining_keeps_existing_role() {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let campus_id = campus_for(&state, &owner.token, "north-quad")
        .await
        .campus
        .campus_id;
    let invite = issued_invite(&state, &owner.token, campus_id).await;
    let (status, body) = join_request(&state, &owner.token, invite.code).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["campus"]["role"], json!("owner"));
    assert_eq!(count_rows(&state, "campus_members", campus_id).await, 1);
}

#[tokio::test]
async fn concurrent_joins_create_one_membership() {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let joiner = signed_up(&state, "joiner@example.org").await;
    let campus_id = campus_for(&state, &owner.token, "north-quad")
        .await
        .campus
        .campus_id;
    let invite = issued_invite(&state, &owner.token, campus_id).await;
    let attempts =
        (0..CONCURRENT_JOINS).map(|_| join_request(&state, &joiner.token, invite.code.clone()));
    let outcomes = timeout(Duration::from_secs(30), join_all(attempts))
        .await
        .expect("joins settle");
    assert!(outcomes.iter().all(|(status, _)| *status == StatusCode::OK));
    assert_eq!(count_rows(&state, "campus_members", campus_id).await, 2);
}
