mod common;

use std::collections::HashSet;
use std::time::Duration;

use axum::http::{Method, StatusCode};
use common::{
    Observed, api_raw, campus_for, campuses_of, fresh_state, handshake_outcome, rooms_of,
    signed_up, spawn_server_with,
};
use pulse_protocol::{
    AuthToken, CampusCreated, CampusId, DEFAULT_ROOM_SLUGS, HandshakeRejected, HandshakeRejection,
    Role, ServerFrame, api_path,
};
use rstest::rstest;
use serde_json::json;
use tokio::task::JoinSet;
use tokio::time::timeout;

const DEADLINE: Duration = Duration::from_secs(60);

struct Tenant {
    token: AuthToken,
    campus_id: CampusId,
    slug: String,
}

#[rstest]
#[case::two_accounts(2)]
#[case::eight_accounts(8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_accounts_see_only_their_own_campuses_and_rooms(#[case] accounts: usize) {
    timeout(DEADLINE, async {
        let state = fresh_state().await;
        let mut sessions = Vec::new();
        for index in 0..accounts {
            sessions.push(signed_up(&state, &format!("tenant-{index}@example.org")).await);
        }
        let mut creations = JoinSet::new();
        for (index, session) in sessions.iter().enumerate() {
            let state = state.clone();
            let token = session.token.clone();
            creations.spawn(async move {
                let slug = format!("campus-{index}");
                let created = campus_for(&state, &token, &slug).await;
                (index, token, slug, created)
            });
        }
        let mut tenants: Vec<Option<Tenant>> = (0..accounts).map(|_| None).collect();
        while let Some(done) = creations.join_next().await {
            let (index, token, slug, created): (usize, AuthToken, String, CampusCreated) =
                done.expect("creation task");
            tenants[index] = Some(Tenant {
                token,
                campus_id: created.campus.campus_id,
                slug,
            });
        }
        let tenants: Vec<Tenant> = tenants
            .into_iter()
            .map(|tenant| tenant.expect("tenant"))
            .collect();
        let mut readers = JoinSet::new();
        for (index, tenant) in tenants.iter().enumerate() {
            let state = state.clone();
            let token = tenant.token.clone();
            let foreign: Vec<CampusId> = tenants
                .iter()
                .filter(|other| other.campus_id != tenant.campus_id)
                .map(|other| other.campus_id)
                .collect();
            let own = tenant.campus_id;
            readers.spawn(async move {
                let list = campuses_of(&state, &token).await;
                let rooms = rooms_of(&state, &token, own).await;
                let mut foreign_statuses = Vec::new();
                for campus_id in foreign {
                    let (status, _) =
                        common::api_get(&state, &api_path::campus_rooms(campus_id), Some(&token))
                            .await;
                    foreign_statuses.push(status);
                }
                (index, list, rooms, foreign_statuses)
            });
        }
        let mut seen = HashSet::new();
        while let Some(done) = readers.join_next().await {
            let (index, list, rooms, foreign_statuses) = done.expect("reader task");
            let tenant = &tenants[index];
            assert_eq!(
                list.campuses
                    .iter()
                    .map(|campus| (campus.campus_id, campus.slug.as_str(), campus.role))
                    .collect::<Vec<_>>(),
                vec![(tenant.campus_id, tenant.slug.as_str(), Role::Owner)]
            );
            assert_eq!(rooms.campus_id, tenant.campus_id);
            assert!(
                rooms
                    .rooms
                    .iter()
                    .all(|room| room.campus_id == tenant.campus_id)
            );
            assert_eq!(
                rooms
                    .rooms
                    .iter()
                    .map(|room| room.slug.as_str())
                    .collect::<Vec<_>>(),
                DEFAULT_ROOM_SLUGS
            );
            assert!(
                foreign_statuses
                    .iter()
                    .all(|status| *status == StatusCode::NOT_FOUND)
            );
            assert_eq!(foreign_statuses.len(), accounts - 1);
            assert!(seen.insert(index));
        }
        assert_eq!(seen.len(), accounts);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::three_racers(3)]
#[case::eight_racers(8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_same_slug_creates_yield_exactly_one_campus(#[case] racers: usize) {
    timeout(DEADLINE, async {
        let state = fresh_state().await;
        let mut sessions = Vec::new();
        for index in 0..racers {
            sessions.push(signed_up(&state, &format!("racer-{index}@example.org")).await);
        }
        let mut attempts = JoinSet::new();
        for session in &sessions {
            let state = state.clone();
            let token = session.token.clone();
            attempts.spawn(async move {
                let body = json!({"slug": "contested", "display_name": "Contested"}).to_string();
                api_raw(
                    &state,
                    Method::POST,
                    api_path::CAMPUSES,
                    Some(&token),
                    Some(body),
                )
                .await
            });
        }
        let mut created = 0;
        let mut conflicts = 0;
        while let Some(done) = attempts.join_next().await {
            let (status, body) = done.expect("attempt task");
            match status {
                StatusCode::CREATED => created += 1,
                StatusCode::CONFLICT => {
                    conflicts += 1;
                    assert_eq!(body, json!({"code": "conflict", "field": "slug"}));
                }
                other => panic!("unexpected status {other}: {body}"),
            }
        }
        assert_eq!((created, conflicts), (1, racers - 1));
        let counts = sqlx::query_as::<_, (i64, i64, i64)>(
            "SELECT (SELECT count(*) FROM campuses), \
                    (SELECT count(*) FROM campus_members), \
                    (SELECT count(*) FROM rooms)",
        )
        .fetch_one(&state.db)
        .await
        .expect("counts");
        assert_eq!(counts, (1, 1, 2));
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::four_tenants(4)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_handshakes_admit_only_the_owning_campus(#[case] tenants: usize) {
    timeout(DEADLINE, async {
        let state = fresh_state().await;
        let mut roster = Vec::new();
        for index in 0..tenants {
            let session = signed_up(&state, &format!("socket-{index}@example.org")).await;
            let created =
                campus_for(&state, &session.token, &format!("socket-campus-{index}")).await;
            roster.push((session.token, created.campus.campus_id));
        }
        let addr = spawn_server_with(state).await;
        let mut attempts = JoinSet::new();
        for (owner_index, (token, _)) in roster.iter().enumerate() {
            for (campus_index, (_, campus_id)) in roster.iter().enumerate() {
                let token = token.clone();
                let campus_id = *campus_id;
                attempts.spawn(async move {
                    (
                        owner_index,
                        campus_index,
                        campus_id,
                        handshake_outcome(addr, campus_id, &token).await,
                    )
                });
            }
        }
        let mut observed = 0;
        while let Some(done) = attempts.join_next().await {
            let (owner_index, campus_index, campus_id, outcome) = done.expect("handshake task");
            observed += 1;
            if owner_index == campus_index {
                let Observed::Frame(ServerFrame::Welcome(welcome)) = outcome else {
                    panic!("owner {owner_index} expected welcome, got {outcome:?}");
                };
                assert_eq!(welcome.campus_id, campus_id);
                assert_eq!(welcome.role, Role::Owner);
            } else {
                assert_eq!(
                    outcome,
                    Observed::Frame(ServerFrame::HandshakeRejected(HandshakeRejected {
                        reason: HandshakeRejection::CampusUnavailable,
                    })),
                    "account {owner_index} reached campus {campus_index}"
                );
            }
        }
        assert_eq!(observed, tenants * tenants);
    })
    .await
    .expect("scenario deadline");
}
