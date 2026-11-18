mod common;

use std::collections::HashSet;
use std::time::Duration;

use axum::http::{Method, StatusCode};
use common::{
    Observed, api_raw, campus_for, campus_with_owner, campuses_of, created_messages,
    directory_counts, expect_frame, frames_until_pong, fresh_state, handshake_outcome,
    joined_socket, message_count, posted, rooms_of, send, send_message, sibling_state, signed_up,
    spawn_server_with, subscribed_socket, sync_rooms, tally_race,
};
use pulse_protocol::fixtures::{CAMPUS_A, CAMPUS_B, NONCE_A, NONCE_B};
use pulse_protocol::{
    AuthToken, CampusCreated, CampusId, ClientNonce, DEFAULT_ROOM_SLUGS, HandshakeRejected,
    HandshakeRejection, MessageRejected, MessageRejection, Role, RoomUnavailable, ServerFrame,
    api_path,
};
use pulse_server::fanout::campus_channel;
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
        let (created, conflicts) =
            tally_race(attempts, json!({"code": "conflict", "field": "slug"})).await;
        assert_eq!((created, conflicts), (1, racers - 1));
        assert_eq!(directory_counts(&state).await, (1, 1, 2));
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

#[rstest]
#[case::one_instance(false)]
#[case::two_instances(true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn subscriber_in_one_campus_never_observes_another_campus_messages(
    #[case] cross_instance: bool,
) {
    timeout(DEADLINE, async {
        let state = fresh_state().await;
        let north = campus_with_owner(&state, "north@example.org", "north-quad").await;
        let south = campus_with_owner(&state, "south@example.org", "south-quad").await;
        let north_addr = spawn_server_with(state.clone()).await;
        let south_addr = if cross_instance {
            spawn_server_with(sibling_state(&state)).await
        } else {
            north_addr
        };
        let mut north_watcher =
            subscribed_socket(north_addr, north.id, &north.owner.token, &north.rooms).await;
        let mut south_watcher =
            subscribed_socket(south_addr, south.id, &south.owner.token, &south.rooms).await;
        let (mut north_sender, _) = joined_socket(north_addr, north.id, &north.owner.token).await;
        let (mut south_sender, _) = joined_socket(south_addr, south.id, &south.owner.token).await;
        let mut north_posts = Vec::new();
        for index in 0..3 {
            let body = format!("north {index}");
            north_posts.push(
                posted(
                    &mut north_sender,
                    north.rooms[0],
                    ClientNonce::generate(),
                    &body,
                )
                .await,
            );
        }
        let south_post = posted(&mut south_sender, south.rooms[0], NONCE_A, "south").await;
        assert_eq!(created_messages(&mut north_watcher, 3).await, north_posts);
        assert_eq!(
            created_messages(&mut south_watcher, 1).await,
            vec![south_post]
        );
        assert!(frames_until_pong(&mut north_watcher, 1).await.is_empty());
        assert!(frames_until_pong(&mut south_watcher, 1).await.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy)]
enum Forged {
    OnlyForeignRoom,
    ForeignBesideOwnRoom,
}

#[rstest]
#[case::only_foreign_room(Forged::OnlyForeignRoom)]
#[case::foreign_beside_own_room(Forged::ForeignBesideOwnRoom)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn forged_foreign_room_in_sync_is_unavailable_and_never_subscribed(#[case] forged: Forged) {
    timeout(DEADLINE, async {
        let state = fresh_state().await;
        let north = campus_with_owner(&state, "north@example.org", "north-quad").await;
        let south = campus_with_owner(&state, "south@example.org", "south-quad").await;
        let addr = spawn_server_with(state.clone()).await;
        let (mut intruder, _) = joined_socket(addr, south.id, &south.owner.token).await;
        let foreign_room = north.rooms[0];
        let cursors = match forged {
            Forged::OnlyForeignRoom => vec![(foreign_room, None)],
            Forged::ForeignBesideOwnRoom => vec![(south.rooms[0], None), (foreign_room, None)],
        };
        send(&mut intruder, &sync_rooms(&cursors)).await;
        if matches!(forged, Forged::ForeignBesideOwnRoom) {
            assert!(matches!(
                expect_frame(&mut intruder).await,
                ServerFrame::RoomSynced(_)
            ));
        }
        assert_eq!(
            expect_frame(&mut intruder).await,
            ServerFrame::RoomUnavailable(RoomUnavailable {
                room_id: foreign_room
            })
        );
        let (mut north_sender, _) = joined_socket(addr, north.id, &north.owner.token).await;
        posted(&mut north_sender, foreign_room, NONCE_A, "private").await;
        send(
            &mut intruder,
            &send_message(foreign_room, NONCE_B, "intrusion"),
        )
        .await;
        assert_eq!(
            expect_frame(&mut intruder).await,
            ServerFrame::MessageRejected(MessageRejected {
                client_nonce: NONCE_B,
                reason: MessageRejection::UnknownRoom,
            })
        );
        assert!(frames_until_pong(&mut intruder, 2).await.is_empty());
        assert_eq!(message_count(&state, north.id).await, 1);
        assert_eq!(message_count(&state, south.id).await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::campus_a(CAMPUS_A, "pulse:campus:0193a5c0-0000-7000-8000-00000000000a:messages")]
#[case::campus_b(CAMPUS_B, "pulse:campus:0193a5c0-0000-7000-8000-00000000000b:messages")]
fn campus_channel_embeds_campus_id(#[case] campus_id: CampusId, #[case] expected: &str) {
    assert_eq!(campus_channel(campus_id), expected);
}

#[test]
fn distinct_campuses_never_share_a_channel() {
    assert_ne!(campus_channel(CAMPUS_A), campus_channel(CAMPUS_B));
}
