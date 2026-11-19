mod common;

use std::collections::BTreeSet;
use std::time::Duration;

use axum::http::{Method, StatusCode};
use common::steward::StewardClient;
use common::{
    ClientSocket, Person, SCENARIO_DEADLINE, Stage, api, campus_lounge, count_rows,
    frames_until_pong, issue_steward, join_lounge_frame, leave_lounge_frame,
    lounge_heartbeat_frame, lounge_rows, next_of, presence_size, send, sibling_state,
    spawn_server_with, sync_lounges_frame,
};
use pulse_protocol::{
    AccountId, CampusId, CreateRoomRequest, Lounge, LoungeId, LoungeJoined, LoungeRejection,
    LoungeScope, ServerFrame, api_path,
};
use pulse_server::lounge::{LoungeConfig, presence_key};
use rstest::rstest;
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};

async fn lounges_of(socket: &mut ClientSocket) -> Vec<Lounge> {
    send(socket, &sync_lounges_frame()).await;
    next_of(socket, |frame| match frame {
        ServerFrame::LoungesSynced(synced) => Some(synced.lounges.clone()),
        _ => None,
    })
    .await
}

async fn lounge_by_id(socket: &mut ClientSocket, lounge_id: LoungeId) -> Lounge {
    lounges_of(socket)
        .await
        .into_iter()
        .find(|lounge| lounge.lounge_id == lounge_id)
        .expect("lounge listed")
}

async fn join(socket: &mut ClientSocket, lounge_id: LoungeId) -> LoungeJoined {
    send(socket, &join_lounge_frame(lounge_id)).await;
    next_of(socket, |frame| match frame {
        ServerFrame::LoungeJoined(joined) if joined.lounge.lounge_id == lounge_id => {
            Some(joined.clone())
        }
        _ => None,
    })
    .await
}

async fn rejection(socket: &mut ClientSocket, lounge_id: LoungeId) -> LoungeRejection {
    next_of(socket, |frame| match frame {
        ServerFrame::LoungeRejected(rejected) if rejected.lounge_id == lounge_id => {
            Some(rejected.reason)
        }
        _ => None,
    })
    .await
}

async fn changed_until(
    socket: &mut ClientSocket,
    lounge_id: LoungeId,
    mut accept: impl FnMut(&Lounge) -> bool,
) -> Lounge {
    next_of(socket, |frame| match frame {
        ServerFrame::LoungeChanged(changed)
            if changed.lounge.lounge_id == lounge_id && accept(&changed.lounge) =>
        {
            Some(changed.lounge.clone())
        }
        _ => None,
    })
    .await
}

fn members(lounge: &Lounge) -> BTreeSet<AccountId> {
    lounge
        .participants
        .iter()
        .map(|participant| participant.account_id)
        .collect()
}

fn short_ttl(ttl_ms: u64) -> LoungeConfig {
    LoungeConfig {
        presence_ttl_ms: ttl_ms,
        ..LoungeConfig::default()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sync_lists_the_campus_lounge_then_one_per_room() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let listed = lounges_of(&mut ada).await;
        assert_eq!(listed.len(), 1 + stage.campus.rooms.len());
        assert_eq!(listed[0].scope, LoungeScope::Campus);
        assert_eq!(listed[0].room_id, None);
        let room_scoped = listed[1..]
            .iter()
            .map(|lounge| lounge.room_id)
            .collect::<BTreeSet<_>>();
        assert_eq!(
            room_scoped,
            stage
                .campus
                .rooms
                .iter()
                .copied()
                .map(Some)
                .collect::<BTreeSet<_>>()
        );
        assert!(
            listed[1..]
                .iter()
                .all(|lounge| lounge.scope == LoungeScope::Room)
        );
        assert!(
            listed
                .iter()
                .all(|lounge| lounge.campus_id == stage.campus.id
                    && lounge.participants.is_empty()
                    && lounge.revision == 0)
        );
        let stored = lounge_rows(&stage.state, stage.campus.id).await;
        assert_eq!(
            listed
                .iter()
                .map(|lounge| lounge.lounge_id)
                .collect::<Vec<_>>(),
            stored
                .iter()
                .map(|(lounge_id, _)| *lounge_id)
                .collect::<Vec<_>>()
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn joins_leaves_and_disconnects_fan_out_to_other_members() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage.state, stage.campus.id).await;
        let ada_id = stage.id_of(Person::Ada);
        let mut grace = stage.plain(Person::Grace).await;
        let mut ada = stage.plain(Person::Ada).await;
        let joined = join(&mut ada, lounge_id).await;
        assert_eq!(members(&joined.lounge), BTreeSet::from([ada_id]));
        assert_eq!(joined.ticket.lounge_id, lounge_id);
        let seen = changed_until(&mut grace, lounge_id, |lounge| {
            members(lounge).contains(&ada_id)
        })
        .await;
        assert!(seen.revision >= joined.lounge.revision);
        send(&mut ada, &leave_lounge_frame(lounge_id)).await;
        let left = changed_until(&mut grace, lounge_id, |lounge| {
            lounge.participants.is_empty()
        })
        .await;
        assert!(left.revision > seen.revision);
        join(&mut ada, lounge_id).await;
        changed_until(&mut grace, lounge_id, |lounge| {
            members(lounge).contains(&ada_id)
        })
        .await;
        drop(ada);
        let gone = changed_until(&mut grace, lounge_id, |lounge| {
            lounge.participants.is_empty()
        })
        .await;
        assert!(gone.revision > left.revision);
        assert_eq!(
            presence_size(&stage.state, stage.campus.id, lounge_id).await,
            0
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn stewards_never_see_lounge_traffic() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage.state, stage.campus.id).await;
        let (_, token) = issue_steward(
            &stage.state,
            stage.campus.id,
            stage.campus.owner.account.account_id,
            "Scribe",
            &stage.campus.rooms,
        )
        .await;
        let mut steward = StewardClient::connect(stage.addr, stage.campus.id, &token).await;
        let mut grace = stage.plain(Person::Grace).await;
        let mut ada = stage.plain(Person::Ada).await;
        join(&mut ada, lounge_id).await;
        send(&mut ada, &leave_lounge_frame(lounge_id)).await;
        changed_until(&mut grace, lounge_id, |lounge| lounge.revision >= 2).await;
        let seen = frames_until_pong(&mut steward.socket, 8).await;
        assert!(
            seen.iter()
                .all(|frame| !matches!(frame, ServerFrame::LoungeChanged(_)))
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn presence_expires_without_heartbeats_and_heartbeats_extend_it() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let addr = spawn_server_with(stage.state.clone().with_lounge(short_ttl(1_000))).await;
        let lounge_id = campus_lounge(&stage.state, stage.campus.id).await;
        let ada_id = stage.id_of(Person::Ada);
        let (mut ada, _) = common::joined_socket(addr, stage.campus.id, &stage.ada.token).await;
        let (mut observer, _) =
            common::joined_socket(addr, stage.campus.id, &stage.grace.token).await;
        let joined = join(&mut ada, lounge_id).await;
        let first_expiry = joined.lounge.participants[0].present_until_ms;
        sleep(Duration::from_millis(600)).await;
        send(&mut ada, &lounge_heartbeat_frame(lounge_id)).await;
        let beat = changed_until(&mut observer, lounge_id, |lounge| {
            lounge
                .participants
                .first()
                .is_some_and(|member| member.present_until_ms > first_expiry)
        })
        .await;
        assert_eq!(members(&beat), BTreeSet::from([ada_id]));
        sleep(Duration::from_millis(600)).await;
        let still = lounge_by_id(&mut observer, lounge_id).await;
        assert_eq!(members(&still), BTreeSet::from([ada_id]));
        sleep(Duration::from_millis(1_300)).await;
        let expired = lounge_by_id(&mut observer, lounge_id).await;
        assert!(expired.participants.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::two_leavers_two_stayers(2, 2)]
#[case::all_leave(4, 0)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_joins_and_leaves_converge_on_exact_members(
    #[case] leavers: usize,
    #[case] stayers: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage.state, stage.campus.id).await;
        let sessions =
            common::extra_members(&stage.state, stage.campus.id, leavers + stayers).await;
        let mut watcher = stage.plain(Person::Linus).await;
        let mut churn = JoinSet::new();
        for (index, session) in sessions.iter().enumerate() {
            let addr = stage.addr;
            let campus_id: CampusId = stage.campus.id;
            let token = session.token.clone();
            let leaves = index < leavers;
            churn.spawn(async move {
                let (mut socket, welcome) = common::joined_socket(addr, campus_id, &token).await;
                join(&mut socket, lounge_id).await;
                if leaves {
                    send(&mut socket, &leave_lounge_frame(lounge_id)).await;
                    let _ = frames_until_pong(&mut socket, 1).await;
                }
                (socket, welcome.account_id, leaves)
            });
        }
        let mut sockets = Vec::new();
        let mut expected = BTreeSet::new();
        while let Some(done) = churn.join_next().await {
            let (socket, account_id, leaves) = done.expect("churn task");
            if !leaves {
                expected.insert(account_id);
            }
            sockets.push(socket);
        }
        let mutations = u64::try_from(leavers * 2 + stayers).expect("mutations");
        let settled = changed_until(&mut watcher, lounge_id, |lounge| {
            lounge.revision == mutations
        })
        .await;
        assert_eq!(members(&settled), expected);
        let current = lounge_by_id(&mut watcher, lounge_id).await;
        assert_eq!(current.revision, mutations);
        assert_eq!(members(&current), expected);
        assert_eq!(
            presence_size(&stage.state, stage.campus.id, lounge_id).await,
            expected.len()
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_sessions_of_one_account_share_a_single_participant() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage.state, stage.campus.id).await;
        let ada_id = stage.id_of(Person::Ada);
        let mut watcher = stage.plain(Person::Grace).await;
        let mut first = stage.plain(Person::Ada).await;
        let mut second = stage.plain(Person::Ada).await;
        join(&mut first, lounge_id).await;
        let both = join(&mut second, lounge_id).await;
        assert_eq!(both.lounge.participants.len(), 1);
        assert_eq!(
            presence_size(&stage.state, stage.campus.id, lounge_id).await,
            2
        );
        send(&mut first, &leave_lounge_frame(lounge_id)).await;
        changed_until(&mut watcher, lounge_id, |lounge| {
            lounge.revision > both.lounge.revision
        })
        .await;
        let remaining = lounge_by_id(&mut watcher, lounge_id).await;
        assert_eq!(members(&remaining), BTreeSet::from([ada_id]));
        assert_eq!(
            presence_size(&stage.state, stage.campus.id, lounge_id).await,
            1
        );
        drop(second);
        changed_until(&mut watcher, lounge_id, |lounge| {
            lounge.participants.is_empty()
        })
        .await;
        assert_eq!(
            presence_size(&stage.state, stage.campus.id, lounge_id).await,
            0
        );
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::member(Person::Ada, true)]
#[case::guest(Person::Guest, false)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn only_speaking_roles_get_a_speaking_slot(#[case] person: Person, #[case] can_speak: bool) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage.state, stage.campus.id).await;
        let mut socket = stage.plain(person).await;
        let joined = join(&mut socket, lounge_id).await;
        assert_eq!(joined.lounge.participants.len(), 1);
        assert_eq!(joined.lounge.participants[0].can_speak, can_speak);
        assert_eq!(
            joined.lounge.participants[0].account_id,
            stage.id_of(person)
        );
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::unknown_lounge(false)]
#[case::lounge_of_another_campus(true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn joining_a_lounge_outside_the_campus_is_unknown(#[case] foreign: bool) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let target = if foreign {
            let other =
                common::campus_with_owner(&stage.state, "other@example.org", "south-quad").await;
            campus_lounge(&stage.state, other.id).await
        } else {
            LoungeId::generate()
        };
        let mut ada = stage.plain(Person::Ada).await;
        send(&mut ada, &join_lounge_frame(target)).await;
        assert_eq!(
            rejection(&mut ada, target).await,
            LoungeRejection::UnknownLounge
        );
        send(&mut ada, &lounge_heartbeat_frame(target)).await;
        assert_eq!(
            rejection(&mut ada, target).await,
            LoungeRejection::UnknownLounge
        );
        if foreign {
            let other_campus = lounge_rows(&stage.state, stage.campus.id).await;
            assert!(
                other_campus
                    .iter()
                    .all(|(lounge_id, _)| *lounge_id != target)
            );
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_full_lounge_turns_newcomers_away() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage.state, stage.campus.id).await;
        let mut connection = stage
            .state
            .valkey
            .get_multiplexed_async_connection()
            .await
            .expect("valkey");
        let expires = common::ms_from_now(60_000);
        for _ in 0..pulse_protocol::LOUNGE_PARTICIPANTS_MAX {
            let member = format!(
                "{}:{}:1",
                AccountId::generate(),
                pulse_protocol::SessionId::generate()
            );
            redis::cmd("ZADD")
                .arg(presence_key(stage.campus.id, lounge_id))
                .arg(expires)
                .arg(member)
                .query_async::<()>(&mut connection)
                .await
                .expect("seed presence");
        }
        let mut ada = stage.plain(Person::Ada).await;
        send(&mut ada, &join_lounge_frame(lounge_id)).await;
        assert_eq!(rejection(&mut ada, lounge_id).await, LoungeRejection::Full);
        assert_eq!(
            presence_size(&stage.state, stage.campus.id, lounge_id).await,
            pulse_protocol::LOUNGE_PARTICIPANTS_MAX
        );
        let _ = frames_until_pong(&mut ada, 3).await;
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_valkey_outage_degrades_lounges_without_closing_sockets() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let lounge_id = campus_lounge(&stage.state, stage.campus.id).await;
        let mut broken = sibling_state(&stage.state);
        broken.valkey = redis::Client::open("redis://127.0.0.1:1").expect("client");
        let addr = spawn_server_with(broken).await;
        let (mut ada, _) = common::joined_socket(addr, stage.campus.id, &stage.ada.token).await;
        assert!(lounges_of(&mut ada).await.is_empty());
        send(&mut ada, &join_lounge_frame(lounge_id)).await;
        assert_eq!(
            rejection(&mut ada, lounge_id).await,
            LoungeRejection::Unavailable
        );
        send(&mut ada, &lounge_heartbeat_frame(lounge_id)).await;
        assert_eq!(
            rejection(&mut ada, lounge_id).await,
            LoungeRejection::UnknownLounge
        );
        send(&mut ada, &leave_lounge_frame(lounge_id)).await;
        assert!(frames_until_pong(&mut ada, 5).await.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::four(4)]
#[case::eight(8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_room_creation_with_one_slug_yields_one_room_and_one_lounge(
    #[case] attempts: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let rooms_before = count_rows(&stage.state, "rooms", stage.campus.id).await;
        let lounges_before = count_rows(&stage.state, "lounges", stage.campus.id).await;
        let mut creating = JoinSet::new();
        for _ in 0..attempts {
            let state = sibling_state(&stage.state);
            let token = stage.campus.owner.token.clone();
            let campus_id = stage.campus.id;
            creating.spawn(async move {
                let request = CreateRoomRequest {
                    slug: "workshop".to_owned(),
                };
                api(
                    &state,
                    Method::POST,
                    &api_path::campus_rooms(campus_id),
                    Some(&token),
                    Some(&request),
                )
                .await
                .0
            });
        }
        let mut statuses = Vec::new();
        while let Some(done) = creating.join_next().await {
            statuses.push(done.expect("creation task"));
        }
        assert_eq!(
            statuses
                .iter()
                .filter(|status| **status == StatusCode::CREATED)
                .count(),
            1
        );
        assert_eq!(
            statuses
                .iter()
                .filter(|status| **status == StatusCode::CONFLICT)
                .count(),
            attempts - 1
        );
        assert_eq!(
            count_rows(&stage.state, "rooms", stage.campus.id).await,
            rooms_before + 1
        );
        assert_eq!(
            count_rows(&stage.state, "lounges", stage.campus.id).await,
            lounges_before + 1
        );
    })
    .await
    .expect("scenario deadline");
}
