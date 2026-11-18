mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;

use common::{
    SCENARIO_DEADLINE, assert_chain, campus_with_owner, created_messages, frames_until_pong,
    fresh_state, joined_socket, member_session, posted, sibling_state, spawn_server_with,
    subscribed_socket,
};
use pulse_protocol::{ClientNonce, Role, RoomId, RoomMessage};
use rstest::rstest;
use tokio::task::JoinSet;
use tokio::time::timeout;

fn per_room(messages: Vec<RoomMessage>) -> BTreeMap<RoomId, Vec<RoomMessage>> {
    let mut rooms: BTreeMap<RoomId, Vec<RoomMessage>> = BTreeMap::new();
    for message in messages {
        rooms.entry(message.room_id).or_default().push(message);
    }
    rooms
        .values_mut()
        .for_each(|room| room.sort_by_key(|message| message.message_id));
    rooms
}

#[rstest]
#[case::one_instance(1, 3, 3, 4)]
#[case::two_instances(2, 6, 8, 5)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn every_subscriber_receives_every_message_exactly_once(
    #[case] instances: usize,
    #[case] subscribers: usize,
    #[case] senders: usize,
    #[case] posts_each: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let mut addresses: Vec<SocketAddr> = vec![spawn_server_with(state.clone()).await];
        for _ in 1..instances {
            addresses.push(spawn_server_with(sibling_state(&state)).await);
        }
        let rooms = campus.rooms[..2].to_vec();
        let mut watchers = Vec::new();
        for index in 0..subscribers {
            let addr = addresses[index % addresses.len()];
            watchers.push(subscribed_socket(addr, campus.id, &campus.owner.token, &rooms).await);
        }
        let mut posting = JoinSet::new();
        for index in 0..senders {
            let session = member_session(
                &state,
                &format!("sender-{index}@example.org"),
                campus.id,
                Role::Member,
            )
            .await;
            let addr = addresses[index % addresses.len()];
            let campus_id = campus.id;
            let rooms = rooms.clone();
            posting.spawn(async move {
                let (mut socket, _) = joined_socket(addr, campus_id, &session.token).await;
                let mut accepted = Vec::new();
                for post in 0..posts_each {
                    let room_id = rooms[(index + post) % rooms.len()];
                    let nonce = ClientNonce::generate();
                    accepted.push(posted(&mut socket, room_id, nonce, "race").await);
                }
                accepted
            });
        }
        let mut expected = Vec::new();
        while let Some(done) = posting.join_next().await {
            expected.extend(done.expect("sender task"));
        }
        let expected_ids = expected
            .iter()
            .map(|message| message.message_id)
            .collect::<BTreeSet<_>>();
        assert_eq!(expected_ids.len(), senders * posts_each);
        for watcher in &mut watchers {
            let received = created_messages(watcher, expected.len()).await;
            let ids = received
                .iter()
                .map(|message| message.message_id)
                .collect::<Vec<_>>();
            assert_eq!(ids.len(), ids.iter().collect::<BTreeSet<_>>().len());
            assert_eq!(ids.into_iter().collect::<BTreeSet<_>>(), expected_ids);
            let rooms_seen = per_room(received);
            assert_eq!(rooms_seen.len(), rooms.len());
            rooms_seen
                .values()
                .for_each(|room| assert_chain(room, None));
            assert!(frames_until_pong(watcher, 3).await.is_empty());
        }
        let stored = per_room(expected);
        stored.values().for_each(|room| assert_chain(room, None));
    })
    .await
    .expect("scenario deadline");
}
