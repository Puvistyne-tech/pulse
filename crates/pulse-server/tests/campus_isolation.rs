mod common;

use std::collections::HashSet;

use axum::http::{Method, StatusCode};
use common::{
    Observed, SCENARIO_DEADLINE, api_raw, campus_for, campus_with_owner, campuses_of, count_rows,
    created_messages, directory_counts, download_tickets_of, expect_frame, frames_until_pong,
    fresh_state, grant_role, handshake_outcome, joined_socket, mark_read, member_session,
    message_count, next_of, parsed_tickets, pin_count, pin_frame, posted, react, rooms_of,
    seed_attachments, seed_links, seed_messages, seed_pins, seed_shelf, send, send_message,
    send_reply, set_subscription, set_title, shelf_frame, shelf_state_of, sibling_state, signed_up,
    spawn_server_with, subscribed_socket, sync_rooms, sync_shelf_frame, sync_thread, tally_race,
    vault_frame,
};
use pulse_protocol::fixtures::{CAMPUS_A, CAMPUS_B, EMOJI_A, NONCE_A, NONCE_B};
use pulse_protocol::{
    AuthToken, CampusCreated, CampusId, ClientNonce, DEFAULT_ROOM_SLUGS, HandshakeRejected,
    HandshakeRejection, MessageRejected, MessageRejection, PinRejection, ReactionAction,
    ReactionRejection, Role, RoomUnavailable, ServerFrame, ShelfAction, ShelfRejection,
    SubscriptionOrigin, SubscriptionState, ThreadRejection, VaultRejection, VaultTab, api_path,
    mention_token,
};
use pulse_server::fanout::campus_channel;
use rstest::rstest;
use serde_json::json;
use tokio::task::JoinSet;
use tokio::time::timeout;

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
    timeout(SCENARIO_DEADLINE, async {
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
    timeout(SCENARIO_DEADLINE, async {
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
    timeout(SCENARIO_DEADLINE, async {
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
    timeout(SCENARIO_DEADLINE, async {
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
    timeout(SCENARIO_DEADLINE, async {
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

#[derive(Clone, Copy, Debug)]
enum Probe {
    Reply,
    React,
    Subscribe,
    ViewThread,
    MarkRead,
    Title,
    SyncThread,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refusal {
    Message(MessageRejection),
    Reaction(ReactionRejection),
    Thread(ThreadRejection),
}

#[rstest]
#[case::reply_to_foreign_root(
    Probe::Reply,
    false,
    Refusal::Message(MessageRejection::UnknownThread)
)]
#[case::reply_in_foreign_room(Probe::Reply, true, Refusal::Message(MessageRejection::UnknownRoom))]
#[case::react_to_foreign_message(
    Probe::React,
    false,
    Refusal::Reaction(ReactionRejection::UnknownMessage)
)]
#[case::react_in_foreign_room(
    Probe::React,
    true,
    Refusal::Reaction(ReactionRejection::UnknownMessage)
)]
#[case::subscribe_to_foreign_root(
    Probe::Subscribe,
    false,
    Refusal::Thread(ThreadRejection::UnknownThread)
)]
#[case::subscribe_in_foreign_room(
    Probe::Subscribe,
    true,
    Refusal::Thread(ThreadRejection::UnknownThread)
)]
#[case::view_foreign_root(
    Probe::ViewThread,
    false,
    Refusal::Thread(ThreadRejection::UnknownThread)
)]
#[case::view_in_foreign_room(
    Probe::ViewThread,
    true,
    Refusal::Thread(ThreadRejection::UnknownThread)
)]
#[case::mark_foreign_root_read(
    Probe::MarkRead,
    false,
    Refusal::Thread(ThreadRejection::UnknownThread)
)]
#[case::mark_read_in_foreign_room(
    Probe::MarkRead,
    true,
    Refusal::Thread(ThreadRejection::UnknownThread)
)]
#[case::title_foreign_root(Probe::Title, false, Refusal::Thread(ThreadRejection::UnknownThread))]
#[case::title_in_foreign_room(Probe::Title, true, Refusal::Thread(ThreadRejection::UnknownThread))]
#[case::sync_foreign_thread(
    Probe::SyncThread,
    false,
    Refusal::Thread(ThreadRejection::UnknownThread)
)]
#[case::sync_thread_in_foreign_room(
    Probe::SyncThread,
    true,
    Refusal::Thread(ThreadRejection::UnknownThread)
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn thread_requests_naming_foreign_campus_data_are_refused_without_effect(
    #[case] probe: Probe,
    #[case] foreign_room: bool,
    #[case] expected: Refusal,
) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let north = campus_with_owner(&state, "north@example.org", "north-quad").await;
        let south = campus_with_owner(&state, "south@example.org", "south-quad").await;
        let addr = spawn_server_with(state.clone()).await;
        let (mut north_owner, _) = joined_socket(addr, north.id, &north.owner.token).await;
        let root = posted(
            &mut north_owner,
            north.rooms[0],
            ClientNonce::generate(),
            "private root",
        )
        .await;
        let (mut intruder, _) = joined_socket(addr, south.id, &south.owner.token).await;
        let (own_room, foreign_room_id, target) = (south.rooms[0], north.rooms[0], root.message_id);
        let room = if foreign_room {
            foreign_room_id
        } else {
            own_room
        };
        let outgoing = match probe {
            Probe::Reply => send_reply(room, target, NONCE_A, "intrusion"),
            Probe::React => react(room, target, EMOJI_A, ReactionAction::Add),
            Probe::Subscribe => set_subscription(
                room,
                target,
                SubscriptionState::Muted,
                SubscriptionOrigin::Explicit,
            ),
            Probe::ViewThread => set_subscription(
                room,
                target,
                SubscriptionState::Following,
                SubscriptionOrigin::Viewed,
            ),
            Probe::MarkRead => mark_read(room, target, target),
            Probe::Title => set_title(room, target, "stolen"),
            Probe::SyncThread => sync_thread(room, target, None),
        };
        send(&mut intruder, &outgoing).await;
        let refusal = next_of(&mut intruder, |frame| match frame {
            ServerFrame::MessageRejected(rejected) => Some(Refusal::Message(rejected.reason)),
            ServerFrame::ReactionRejected(rejected) => Some(Refusal::Reaction(rejected.reason)),
            ServerFrame::ThreadRejected(rejected) => Some(Refusal::Thread(rejected.reason)),
            ServerFrame::MessageAccepted(_)
            | ServerFrame::MessageUpdated(_)
            | ServerFrame::ThreadChanged(_)
            | ServerFrame::ThreadSynced(_) => panic!("foreign data was served: {frame:?}"),
            _ => None,
        })
        .await;
        assert_eq!(refusal, expected);
        assert!(frames_until_pong(&mut intruder, 5).await.is_empty());
        for table in ["message_reactions", "thread_subscriptions", "threads"] {
            for campus in [north.id, south.id] {
                assert_eq!(count_rows(&state, table, campus).await, 0, "{table}");
            }
        }
        assert_eq!(message_count(&state, north.id).await, 1);
        assert_eq!(message_count(&state, south.id).await, 0);
        let mut reader =
            subscribed_socket(addr, north.id, &north.owner.token, &north.rooms[..1]).await;
        let page = common::synced_page_at(&mut reader, north.rooms[0], None, None).await;
        assert_eq!(page.messages, vec![root]);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::foreign_member(true, 0)]
#[case::own_member(false, 1)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn mentions_are_stored_only_for_members_of_the_same_campus(
    #[case] foreign: bool,
    #[case] stored: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let north = campus_with_owner(&state, "north@example.org", "north-quad").await;
        let south = campus_with_owner(&state, "south@example.org", "south-quad").await;
        let south_member =
            member_session(&state, "member@example.org", south.id, Role::Member).await;
        let addr = spawn_server_with(state.clone()).await;
        let (mut poster, _) = joined_socket(addr, south.id, &south.owner.token).await;
        let target = if foreign {
            north.owner.account.account_id
        } else {
            south_member.account.account_id
        };
        let body = format!("hello {}", mention_token(target));
        let message = posted(&mut poster, south.rooms[0], ClientNonce::generate(), &body).await;
        assert_eq!(message.mentions.len(), stored);
        assert_eq!(message.body, body);
        assert_eq!(
            common::mention_rows(&state, south.id, message.message_id)
                .await
                .len(),
            stored
        );
        assert_eq!(count_rows(&state, "message_mentions", north.id).await, 0);
        assert_eq!(
            count_rows(&state, "thread_subscriptions", north.id).await,
            0
        );
        assert_eq!(
            common::subscription_row(&state, south.id, message.message_id, target)
                .await
                .is_some(),
            stored == 1
        );
    })
    .await
    .expect("scenario deadline");
}

struct Shared {
    addr: std::net::SocketAddr,
    north: common::Campus,
    south: common::Campus,
    dual: pulse_protocol::AuthSession,
    state: pulse_server::AppState,
}

async fn shared_account() -> Shared {
    let state = fresh_state().await;
    let north = campus_with_owner(&state, "north@example.org", "north-quad").await;
    let south = campus_with_owner(&state, "south@example.org", "south-quad").await;
    let dual = signed_up(&state, "dual@example.org").await;
    grant_role(&state, north.id, dual.account.account_id, Role::Member).await;
    grant_role(&state, south.id, dual.account.account_id, Role::Member).await;
    let addr = spawn_server_with(state.clone()).await;
    Shared {
        addr,
        north,
        south,
        dual,
        state,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn thread_changes_never_reach_the_same_account_in_another_campus() {
    timeout(SCENARIO_DEADLINE, async {
        let shared = shared_account().await;
        let (addr, north, south) = (shared.addr, &shared.north, &shared.south);
        let mut dual_north =
            subscribed_socket(addr, north.id, &shared.dual.token, &north.rooms[..1]).await;
        let mut dual_south =
            subscribed_socket(addr, south.id, &shared.dual.token, &south.rooms[..1]).await;
        let (mut north_owner, _) = joined_socket(addr, north.id, &north.owner.token).await;
        let (mut south_owner, _) = joined_socket(addr, south.id, &south.owner.token).await;
        let root = posted(
            &mut dual_north,
            north.rooms[0],
            ClientNonce::generate(),
            "dual root",
        )
        .await;
        common::replied(
            &mut north_owner,
            north.rooms[0],
            root.message_id,
            ClientNonce::generate(),
            "reply",
        )
        .await;
        common::frames_until_pong(&mut north_owner, 3).await;
        let nonce = ClientNonce::generate();
        posted(&mut south_owner, south.rooms[0], nonce, "south sentinel").await;
        let southern = common::frames_through_created(&mut dual_south, nonce).await;
        assert_eq!(southern.len(), 1);
        assert!(common::changed_summaries(&southern).is_empty());
        let nonce = ClientNonce::generate();
        posted(&mut north_owner, north.rooms[0], nonce, "north sentinel").await;
        let northern = common::frames_through_created(&mut dual_north, nonce).await;
        let changes = common::changed_summaries(&northern);
        assert!(!changes.is_empty());
        assert!(
            changes
                .iter()
                .all(|summary| summary.root.campus_id == north.id)
        );
        assert_eq!(
            count_rows(&shared.state, "thread_subscriptions", south.id).await,
            0
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn inbox_sync_lists_only_threads_of_the_connected_campus() {
    timeout(SCENARIO_DEADLINE, async {
        let shared = shared_account().await;
        let (addr, north, south) = (shared.addr, &shared.north, &shared.south);
        let mut dual_north =
            subscribed_socket(addr, north.id, &shared.dual.token, &north.rooms[..1]).await;
        let mut dual_south =
            subscribed_socket(addr, south.id, &shared.dual.token, &south.rooms[..1]).await;
        let (mut north_owner, _) = joined_socket(addr, north.id, &north.owner.token).await;
        let (mut south_owner, _) = joined_socket(addr, south.id, &south.owner.token).await;
        let northern_root = posted(
            &mut dual_north,
            north.rooms[0],
            ClientNonce::generate(),
            "north root",
        )
        .await;
        let southern_root = posted(
            &mut dual_south,
            south.rooms[0],
            ClientNonce::generate(),
            "south root",
        )
        .await;
        common::replied(
            &mut north_owner,
            north.rooms[0],
            northern_root.message_id,
            ClientNonce::generate(),
            "north reply",
        )
        .await;
        common::replied(
            &mut south_owner,
            south.rooms[0],
            southern_root.message_id,
            ClientNonce::generate(),
            "south reply",
        )
        .await;
        let northern = common::inbox_of(&mut dual_north).await;
        let southern = common::inbox_of(&mut dual_south).await;
        assert_eq!(
            northern
                .iter()
                .map(|summary| summary.root.message_id)
                .collect::<Vec<_>>(),
            vec![northern_root.message_id]
        );
        assert_eq!(
            southern
                .iter()
                .map(|summary| summary.root.message_id)
                .collect::<Vec<_>>(),
            vec![southern_root.message_id]
        );
        assert!(
            northern
                .iter()
                .all(|summary| summary.root.campus_id == north.id)
        );
        assert!(
            southern
                .iter()
                .all(|summary| summary.root.campus_id == south.id)
        );
        let bystander = member_session(
            &shared.state,
            "bystander@example.org",
            south.id,
            Role::Member,
        )
        .await;
        let (mut outsider, _) = joined_socket(addr, south.id, &bystander.token).await;
        assert!(common::inbox_of(&mut outsider).await.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum MediaProbe {
    PinForeignMessage,
    UnpinForeignMessage,
    Vault(VaultTab),
    SaveForeignMessage,
    DownloadForeignAttachment,
    SyncShelfOfSharedAccount,
}

#[rstest]
#[case::pin(MediaProbe::PinForeignMessage)]
#[case::unpin(MediaProbe::UnpinForeignMessage)]
#[case::vault_media(MediaProbe::Vault(VaultTab::Media))]
#[case::vault_files(MediaProbe::Vault(VaultTab::Files))]
#[case::vault_links(MediaProbe::Vault(VaultTab::Links))]
#[case::vault_pinned(MediaProbe::Vault(VaultTab::Pinned))]
#[case::shelf_save(MediaProbe::SaveForeignMessage)]
#[case::download(MediaProbe::DownloadForeignAttachment)]
#[case::shelf_sync(MediaProbe::SyncShelfOfSharedAccount)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn media_vault_and_shelf_requests_never_cross_a_campus(#[case] probe: MediaProbe) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let near = campus_with_owner(&state, "near@example.org", "north-quad").await;
        let far = campus_with_owner(&state, "far@example.org", "south-quad").await;
        let (near_account, far_account) =
            (near.owner.account.account_id, far.owner.account.account_id);
        grant_role(&state, near.id, far_account, Role::Member).await;
        let (room, far_room) = (near.rooms[0], far.rooms[0]);
        let seeded = seed_messages(&state, near.id, room, near_account, 2).await;
        seed_attachments(
            &state,
            near.id,
            room,
            near_account,
            &[(seeded[0], "image/png", "image")],
        )
        .await;
        seed_links(&state, near.id, room, &seeded[1..]).await;
        seed_pins(&state, near.id, room, near_account, &seeded[..1]).await;
        seed_shelf(
            &state,
            near.id,
            room,
            far_account,
            &seeded[..1],
            Some(3_600_000),
        )
        .await;
        let attachment_id = sqlx::query_scalar::<_, uuid::Uuid>(
            "SELECT attachment_id FROM attachments WHERE campus_id = $1",
        )
        .bind(near.id.as_uuid())
        .fetch_one(&state.db)
        .await
        .expect("attachment");
        let addr = spawn_server_with(state.clone()).await;
        let (mut socket, _) = joined_socket(addr, far.id, &far.owner.token).await;
        let near_shape = |state: &pulse_server::AppState| {
            let state = state.clone();
            let (campus_id, message) = (near.id, seeded[0]);
            async move {
                (
                    pin_count(&state, campus_id, room).await,
                    shelf_state_of(&state, campus_id, far_account, message).await,
                )
            }
        };
        let before = near_shape(&state).await;
        match probe {
            MediaProbe::PinForeignMessage | MediaProbe::UnpinForeignMessage => {
                let pinned = matches!(probe, MediaProbe::PinForeignMessage);
                let aimed = if pinned { seeded[1] } else { seeded[0] };
                send(&mut socket, &pin_frame(room, aimed, pinned)).await;
                match expect_frame(&mut socket).await {
                    ServerFrame::PinRejected(rejected) => {
                        assert_eq!(rejected.reason, PinRejection::UnknownMessage)
                    }
                    other => panic!("expected pin rejection, got {other:?}"),
                }
            }
            MediaProbe::Vault(tab) => {
                send(&mut socket, &vault_frame(room, tab, None)).await;
                match expect_frame(&mut socket).await {
                    ServerFrame::VaultRejected(rejected) => {
                        assert_eq!(rejected.reason, VaultRejection::UnknownRoom)
                    }
                    other => panic!("expected vault rejection, got {other:?}"),
                }
            }
            MediaProbe::SaveForeignMessage => {
                send(
                    &mut socket,
                    &shelf_frame(room, seeded[1], ShelfAction::Save, None),
                )
                .await;
                match expect_frame(&mut socket).await {
                    ServerFrame::ShelfRejected(rejected) => {
                        assert_eq!(rejected.reason, ShelfRejection::UnknownMessage)
                    }
                    other => panic!("expected shelf rejection, got {other:?}"),
                }
                send(
                    &mut socket,
                    &shelf_frame(far_room, seeded[1], ShelfAction::Save, None),
                )
                .await;
                match expect_frame(&mut socket).await {
                    ServerFrame::ShelfRejected(rejected) => {
                        assert_eq!(rejected.reason, ShelfRejection::UnknownMessage)
                    }
                    other => panic!("expected shelf rejection, got {other:?}"),
                }
            }
            MediaProbe::DownloadForeignAttachment => {
                let (status, body) = download_tickets_of(
                    &state,
                    &far.owner.token,
                    far.id,
                    vec![pulse_protocol::AttachmentId(attachment_id)],
                )
                .await;
                assert_eq!(status, StatusCode::OK, "{body}");
                assert!(parsed_tickets(body).tickets.is_empty());
            }
            MediaProbe::SyncShelfOfSharedAccount => {
                send(&mut socket, &sync_shelf_frame()).await;
                match expect_frame(&mut socket).await {
                    ServerFrame::ShelfSynced(synced) => assert!(synced.items.is_empty()),
                    other => panic!("expected shelf sync, got {other:?}"),
                }
            }
        }
        assert_eq!(near_shape(&state).await, before);
        assert_eq!(count_rows(&state, "shelf_items", far.id).await, 0);
        assert_eq!(count_rows(&state, "message_pins", far.id).await, 0);
    })
    .await
    .expect("scenario deadline");
}

struct Twin {
    state: pulse_server::AppState,
    addr: std::net::SocketAddr,
    ours: common::Campus,
    theirs: common::Campus,
}

async fn twin_campuses() -> Twin {
    let state = fresh_state().await;
    let ours = campus_with_owner(&state, "ours@example.org", "north-quad").await;
    let theirs = campus_with_owner(&state, "theirs@example.org", "south-quad").await;
    let addr = spawn_server_with(state.clone()).await;
    Twin {
        state,
        addr,
        ours,
        theirs,
    }
}

#[derive(Clone, Copy, Debug)]
enum ForeignCall {
    History,
    Thread,
    Post,
    Subscribe,
}

#[rstest]
#[case::history(ForeignCall::History)]
#[case::thread(ForeignCall::Thread)]
#[case::post(ForeignCall::Post)]
#[case::subscribe(ForeignCall::Subscribe)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_steward_cannot_reach_rooms_of_another_campus(#[case] call: ForeignCall) {
    timeout(SCENARIO_DEADLINE, async {
        let twin = twin_campuses().await;
        let foreign_room = twin.theirs.rooms[0];
        let mut theirs =
            subscribed_socket(twin.addr, twin.theirs.id, &twin.theirs.owner.token, &[foreign_room]).await;
        let root = posted(&mut theirs, foreign_room, ClientNonce::generate(), "private").await;
        let (_, token) = common::issue_steward(
            &twin.state,
            twin.ours.id,
            twin.ours.owner.account.account_id,
            "Scribe",
            &twin.ours.rooms,
        )
        .await;
        let mut steward =
            common::steward::StewardClient::connect(twin.addr, twin.ours.id, &token).await;
        let before = count_rows(&twin.state, "room_messages", twin.theirs.id).await;
        let call_id = pulse_protocol::ToolCallId::generate();
        match call {
            ForeignCall::History => {
                let result = steward
                    .call_tool_raw(
                        call_id,
                        "read_room_history",
                        &format!("{{\"room_id\":\"{foreign_room}\"}}"),
                    )
                    .await;
                assert!(result.is_error);
                assert_eq!(result.content[0].text, "unknown_room");
            }
            ForeignCall::Thread => {
                let result = steward
                    .call_tool_raw(
                        call_id,
                        "read_thread",
                        &format!(
                            "{{\"room_id\":\"{foreign_room}\",\"root_message_id\":\"{}\"}}",
                            root.message_id
                        ),
                    )
                    .await;
                assert!(result.is_error);
                assert_eq!(result.content[0].text, "unknown_room");
            }
            ForeignCall::Post => {
                let result = steward
                    .call_tool_raw(
                        call_id,
                        "post_message",
                        &format!(
                            "{{\"room_id\":\"{foreign_room}\",\"client_nonce\":\"{}\",\"body\":\"intrude\"}}",
                            ClientNonce::generate()
                        ),
                    )
                    .await;
                assert!(result.is_error);
                assert_eq!(result.content[0].text, "unknown_room");
            }
            ForeignCall::Subscribe => {
                send(&mut steward.socket, &sync_rooms(&[(foreign_room, None)])).await;
                let frame = expect_frame(&mut steward.socket).await;
                assert_eq!(
                    frame,
                    ServerFrame::RoomUnavailable(RoomUnavailable { room_id: foreign_room })
                );
            }
        }
        assert_eq!(count_rows(&twin.state, "room_messages", twin.theirs.id).await, before);
        let leaked = frames_until_pong(&mut theirs, 4).await;
        assert!(leaked.iter().all(|frame| !matches!(
            frame,
            ServerFrame::MessageCreated(created) if created.message.message_id != root.message_id
        )));
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn stream_lounge_steward_and_work_frames_stay_inside_their_campus() {
    timeout(SCENARIO_DEADLINE, async {
        let twin = twin_campuses().await;
        let our_room = twin.ours.rooms[0];
        let mut outsider = common::joined_socket(twin.addr, twin.theirs.id, &twin.theirs.owner.token)
            .await
            .0;
        send(&mut outsider, &sync_rooms(&[(twin.theirs.rooms[0], None)])).await;
        let mut ours =
            subscribed_socket(twin.addr, twin.ours.id, &twin.ours.owner.token, &[our_room]).await;
        let (steward, token) = common::issue_steward(
            &twin.state,
            twin.ours.id,
            twin.ours.owner.account.account_id,
            "Scribe",
            &[our_room],
        )
        .await;
        let mut client =
            common::steward::StewardClient::connect(twin.addr, twin.ours.id, &token).await;
        let our_lounge = common::lounge_rows(&twin.state, twin.ours.id).await[0].0;
        let their_lounge = common::lounge_rows(&twin.state, twin.theirs.id).await[0].0;
        let stream_id = pulse_protocol::StreamId::generate();
        client.stream_reply(stream_id, our_room, None, &["hello"]).await;
        send(&mut ours, &common::join_lounge_frame(our_lounge)).await;
        send(
            &mut ours,
            &common::sync_stewards_frame(),
        )
        .await;
        let mut mention = subscribed_socket(twin.addr, twin.ours.id, &twin.ours.owner.token, &[our_room]).await;
        posted(
            &mut mention,
            our_room,
            ClientNonce::generate(),
            &common::mentioning(steward.steward_id, "task"),
        )
        .await;
        client.next_work_item().await;
        next_of(&mut ours, |frame| match frame {
            ServerFrame::AgentStreamChunk(chunk) if chunk.phase == pulse_protocol::StreamPhase::End => Some(()),
            _ => None,
        })
        .await;
        send(&mut outsider, &common::join_lounge_frame(their_lounge)).await;
        let mut seen = Vec::new();
        loop {
            let frame = expect_frame(&mut outsider).await;
            let own_join = matches!(&frame, ServerFrame::LoungeChanged(changed) if changed.lounge.lounge_id == their_lounge);
            seen.push(frame);
            if own_join {
                break;
            }
        }
        for frame in &seen {
            match frame {
                ServerFrame::AgentStreamChunk(_)
                | ServerFrame::StewardChanged(_)
                | ServerFrame::StewardWorkItem(_)
                | ServerFrame::MessageCreated(_) => panic!("leaked across campuses: {frame:?}"),
                ServerFrame::LoungeChanged(changed) => {
                    assert_eq!(changed.lounge.campus_id, twin.theirs.id);
                }
                _ => {}
            }
        }
        send(&mut outsider, &common::sync_stewards_frame()).await;
        let listed = next_of(&mut outsider, |frame| match frame {
            ServerFrame::StewardsSynced(synced) => Some(synced.stewards.clone()),
            _ => None,
        })
        .await;
        assert!(listed.is_empty());
    })
    .await
    .expect("scenario deadline");
}
