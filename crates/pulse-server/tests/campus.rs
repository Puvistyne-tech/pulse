mod common;

use axum::http::{Method, StatusCode};
use common::{api, api_raw, campus_for, directory_counts, fresh_state, grant_role, signed_up};
use pulse_protocol::fixtures::{CAMPUS_B, auth_token};
use pulse_protocol::{
    CampusCreated, CampusList, CreateCampusRequest, DEFAULT_ROOM_SLUGS, Role, RoomList, api_path,
};
use rstest::rstest;
use serde_json::{Value, json};

#[derive(Clone, Copy)]
enum Actor {
    Owner,
    Other,
    Anonymous,
    Garbage,
}

#[derive(Clone, Copy)]
enum Target {
    Rooms,
    MalformedRooms,
    UnknownRooms,
    Campuses,
}

enum Outcome {
    Rooms(Vec<&'static str>),
    Created(&'static str),
    Failure(Value),
}

fn failure(code: &str, field: Option<&str>) -> Outcome {
    Outcome::Failure(json!({"code": code, "field": field}))
}

#[rstest]
#[case::owner_lists_rooms(
    Actor::Owner, None, Method::GET, Target::Rooms, None,
    StatusCode::OK, Outcome::Rooms(vec!["commons", "courtyard"]),
)]
#[case::member_lists_rooms(
    Actor::Other, Some(Role::Member), Method::GET, Target::Rooms, None,
    StatusCode::OK, Outcome::Rooms(vec!["commons", "courtyard"]),
)]
#[case::guest_lists_rooms(
    Actor::Other, Some(Role::Guest), Method::GET, Target::Rooms, None,
    StatusCode::OK, Outcome::Rooms(vec!["commons", "courtyard"]),
)]
#[case::non_member_lists_rooms(
    Actor::Other,
    None,
    Method::GET,
    Target::Rooms,
    None,
    StatusCode::NOT_FOUND,
    failure("not_found", None)
)]
#[case::malformed_campus_id(
    Actor::Owner,
    None,
    Method::GET,
    Target::MalformedRooms,
    None,
    StatusCode::NOT_FOUND,
    failure("not_found", None)
)]
#[case::unknown_campus(
    Actor::Owner,
    None,
    Method::GET,
    Target::UnknownRooms,
    None,
    StatusCode::NOT_FOUND,
    failure("not_found", None)
)]
#[case::anonymous_lists_rooms(
    Actor::Anonymous,
    None,
    Method::GET,
    Target::Rooms,
    None,
    StatusCode::UNAUTHORIZED,
    failure("unauthenticated", None)
)]
#[case::garbage_token_lists_rooms(
    Actor::Garbage,
    None,
    Method::GET,
    Target::Rooms,
    None,
    StatusCode::UNAUTHORIZED,
    failure("unauthenticated", None)
)]
#[case::anonymous_lists_campuses(
    Actor::Anonymous,
    None,
    Method::GET,
    Target::Campuses,
    None,
    StatusCode::UNAUTHORIZED,
    failure("unauthenticated", None)
)]
#[case::owner_creates_room(
    Actor::Owner,
    None,
    Method::POST,
    Target::Rooms,
    Some(r#"{"slug":"lab"}"#),
    StatusCode::CREATED,
    Outcome::Created("lab")
)]
#[case::admin_creates_room(
    Actor::Other,
    Some(Role::Admin),
    Method::POST,
    Target::Rooms,
    Some(r#"{"slug":"lab"}"#),
    StatusCode::CREATED,
    Outcome::Created("lab")
)]
#[case::member_creates_room(
    Actor::Other,
    Some(Role::Member),
    Method::POST,
    Target::Rooms,
    Some(r#"{"slug":"lab"}"#),
    StatusCode::CREATED,
    Outcome::Created("lab")
)]
#[case::guest_creates_room(
    Actor::Other,
    Some(Role::Guest),
    Method::POST,
    Target::Rooms,
    Some(r#"{"slug":"lab"}"#),
    StatusCode::FORBIDDEN,
    failure("forbidden", None)
)]
#[case::non_member_creates_room(
    Actor::Other,
    None,
    Method::POST,
    Target::Rooms,
    Some(r#"{"slug":"lab"}"#),
    StatusCode::NOT_FOUND,
    failure("not_found", None)
)]
#[case::duplicate_room_slug(
    Actor::Owner,
    None,
    Method::POST,
    Target::Rooms,
    Some(r#"{"slug":"commons"}"#),
    StatusCode::CONFLICT,
    failure("conflict", Some("slug"))
)]
#[case::invalid_room_slug(
    Actor::Owner,
    None,
    Method::POST,
    Target::Rooms,
    Some(r#"{"slug":"Bad Slug"}"#),
    StatusCode::BAD_REQUEST,
    failure("invalid_input", Some("slug"))
)]
#[case::malformed_room_json(
    Actor::Owner,
    None,
    Method::POST,
    Target::Rooms,
    Some("{"),
    StatusCode::BAD_REQUEST,
    failure("invalid_input", None)
)]
#[tokio::test]
async fn campus_access_matrix(
    #[case] actor: Actor,
    #[case] other_role: Option<Role>,
    #[case] method: Method,
    #[case] target: Target,
    #[case] body: Option<&'static str>,
    #[case] status: StatusCode,
    #[case] outcome: Outcome,
) {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let other = signed_up(&state, "other@example.org").await;
    let created = campus_for(&state, &owner.token, "north-quad").await;
    let campus_id = created.campus.campus_id;
    if let Some(role) = other_role {
        grant_role(&state, campus_id, other.account.account_id, role).await;
    }
    let path = match target {
        Target::Rooms => api_path::campus_rooms(campus_id),
        Target::MalformedRooms => "/v1/campuses/not-a-uuid/rooms".to_owned(),
        Target::UnknownRooms => api_path::campus_rooms(CAMPUS_B),
        Target::Campuses => api_path::CAMPUSES.to_owned(),
    };
    let token = match actor {
        Actor::Owner => Some(owner.token.clone()),
        Actor::Other => Some(other.token.clone()),
        Actor::Anonymous => None,
        Actor::Garbage => Some(auth_token("not-a-real-token")),
    };
    let (actual_status, actual_body) = api_raw(
        &state,
        method,
        &path,
        token.as_ref(),
        body.map(str::to_owned),
    )
    .await;
    assert_eq!(actual_status, status, "{actual_body}");
    match outcome {
        Outcome::Rooms(slugs) => {
            let list: RoomList = serde_json::from_value(actual_body).expect("room list");
            assert_eq!(list.campus_id, campus_id);
            assert_eq!(
                list.rooms
                    .iter()
                    .map(|room| room.slug.as_str())
                    .collect::<Vec<_>>(),
                slugs
            );
            assert!(list.rooms.iter().all(|room| room.campus_id == campus_id));
        }
        Outcome::Created(slug) => {
            assert_eq!(actual_body["slug"], json!(slug));
            assert_eq!(actual_body["campus_id"], json!(campus_id.to_string()));
        }
        Outcome::Failure(expected) => assert_eq!(actual_body, expected),
    }
}

#[tokio::test]
async fn campus_creation_makes_owner_and_default_rooms() {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let request = CreateCampusRequest {
        slug: "north-quad".to_owned(),
        display_name: "North Quad".to_owned(),
    };
    let (status, body) = api(
        &state,
        Method::POST,
        api_path::CAMPUSES,
        Some(&owner.token),
        Some(&request),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    let created: CampusCreated = serde_json::from_value(body).expect("campus created");
    assert_eq!(created.campus.slug, "north-quad");
    assert_eq!(created.campus.display_name, "North Quad");
    assert_eq!(created.campus.role, Role::Owner);
    assert_eq!(
        created
            .rooms
            .iter()
            .map(|room| room.slug.as_str())
            .collect::<Vec<_>>(),
        DEFAULT_ROOM_SLUGS
    );
    let campus_id = created.campus.campus_id;
    assert!(created.rooms.iter().all(|room| room.campus_id == campus_id));
    let stored_role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM campus_members WHERE campus_id = $1 AND account_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(owner.account.account_id.as_uuid())
    .fetch_one(&state.db)
    .await
    .expect("membership row");
    assert_eq!(stored_role, "owner");
}

#[rstest]
#[case::invalid_slug(r#"{"slug":"A","display_name":"North"}"#, "slug")]
#[case::invalid_display_name(r#"{"slug":"north-quad","display_name":""}"#, "display_name")]
#[tokio::test]
async fn campus_creation_rejects_invalid_input(#[case] body: &str, #[case] field: &str) {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let (status, actual) = api_raw(
        &state,
        Method::POST,
        api_path::CAMPUSES,
        Some(&owner.token),
        Some(body.to_owned()),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(actual, json!({"code": "invalid_input", "field": field}));
}

#[tokio::test]
async fn listing_shows_only_member_campuses_in_creation_order() {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let other = signed_up(&state, "other@example.org").await;
    let first = campus_for(&state, &owner.token, "zulu-campus").await;
    let second = campus_for(&state, &owner.token, "alpha-campus").await;
    campus_for(&state, &other.token, "foreign-campus").await;
    let nothing: Option<&()> = None;
    let (status, body) = api(
        &state,
        Method::GET,
        api_path::CAMPUSES,
        Some(&owner.token),
        nothing,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let list: CampusList = serde_json::from_value(body).expect("campus list");
    assert_eq!(
        list.campuses
            .iter()
            .map(|campus| campus.campus_id)
            .collect::<Vec<_>>(),
        vec![first.campus.campus_id, second.campus.campus_id]
    );
    assert!(
        list.campuses
            .iter()
            .all(|campus| campus.role == Role::Owner)
    );
}

#[tokio::test]
async fn duplicate_slug_conflicts_and_leaves_no_partial_rows() {
    let state = fresh_state().await;
    let first = signed_up(&state, "first@example.org").await;
    let second = signed_up(&state, "second@example.org").await;
    campus_for(&state, &first.token, "north-quad").await;
    let request = CreateCampusRequest {
        slug: "north-quad".to_owned(),
        display_name: "Duplicate".to_owned(),
    };
    let (status, body) = api(
        &state,
        Method::POST,
        api_path::CAMPUSES,
        Some(&second.token),
        Some(&request),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body, json!({"code": "conflict", "field": "slug"}));
    assert_eq!(directory_counts(&state).await, (1, 1, 2));
}

#[rstest]
#[case::padded("  North  ", "North")]
#[tokio::test]
async fn campus_creation_stores_trimmed_display_name(#[case] raw: &str, #[case] stored: &str) {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let (status, body) = api_raw(
        &state,
        Method::POST,
        api_path::CAMPUSES,
        Some(&owner.token),
        Some(json!({"slug": "north-quad", "display_name": raw}).to_string()),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    assert_eq!(body["campus"]["display_name"], json!(stored));
    let persisted = sqlx::query_scalar::<_, String>("SELECT display_name FROM campuses")
        .fetch_one(&state.db)
        .await
        .expect("campus row");
    assert_eq!(persisted, stored);
}
