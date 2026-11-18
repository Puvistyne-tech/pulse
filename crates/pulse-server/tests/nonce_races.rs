mod common;

use std::collections::BTreeSet;
use std::sync::Arc;

use common::{
    SCENARIO_DEADLINE, campus_with_owner, created_messages, frames_until_pong, fresh_state,
    joined_socket, member_session, message_count, posted, sibling_state, spawn_server_with,
    subscribed_socket,
};
use pulse_protocol::fixtures::NONCE_A;
use pulse_protocol::{Role, RoomMessage};
use rstest::rstest;
use tokio::sync::Barrier;
use tokio::task::JoinSet;
use tokio::time::timeout;

#[derive(Clone, Copy)]
enum Spread {
    SameRoom,
    TwoRooms,
}

#[rstest]
#[case::two_sockets_same_room(2, Spread::SameRoom)]
#[case::eight_sockets_same_room(8, Spread::SameRoom)]
#[case::eight_sockets_two_rooms(8, Spread::TwoRooms)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn racing_sockets_of_one_author_store_the_nonce_once(
    #[case] sockets: usize,
    #[case] spread: Spread,
) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let author = member_session(&state, "author@example.org", campus.id, Role::Member).await;
        let addresses = [
            spawn_server_with(state.clone()).await,
            spawn_server_with(sibling_state(&state)).await,
        ];
        let mut watcher = subscribed_socket(
            addresses[1],
            campus.id,
            &campus.owner.token,
            &campus.rooms[..2],
        )
        .await;
        let barrier = Arc::new(Barrier::new(sockets));
        let mut racers = JoinSet::new();
        for index in 0..sockets {
            let addr = addresses[index % addresses.len()];
            let (mut socket, _) = joined_socket(addr, campus.id, &author.token).await;
            let barrier = Arc::clone(&barrier);
            let room_id = match spread {
                Spread::SameRoom => campus.rooms[0],
                Spread::TwoRooms => campus.rooms[index % 2],
            };
            racers.spawn(async move {
                barrier.wait().await;
                posted(&mut socket, room_id, NONCE_A, "once").await
            });
        }
        let mut accepted: Vec<RoomMessage> = Vec::new();
        while let Some(done) = racers.join_next().await {
            accepted.push(done.expect("racer task"));
        }
        assert_eq!(accepted.len(), sockets);
        assert_eq!(
            accepted
                .iter()
                .map(|message| message.message_id)
                .collect::<BTreeSet<_>>()
                .len(),
            1
        );
        assert!(accepted.iter().all(|message| message == &accepted[0]));
        assert_eq!(message_count(&state, campus.id).await, 1);
        assert_eq!(
            created_messages(&mut watcher, 1).await,
            vec![accepted[0].clone()]
        );
        assert!(frames_until_pong(&mut watcher, 5).await.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::sequential(false)]
#[case::concurrent(true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn nonce_reused_in_another_room_returns_the_original(#[case] concurrent: bool) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let addr = spawn_server_with(state.clone()).await;
        let (mut first, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let (mut second, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let (left, right) = (campus.rooms[0], campus.rooms[1]);
        let (original, replay) = if concurrent {
            tokio::join!(
                posted(&mut first, left, NONCE_A, "same"),
                posted(&mut second, right, NONCE_A, "same")
            )
        } else {
            let original = posted(&mut first, left, NONCE_A, "same").await;
            (original, posted(&mut second, right, NONCE_A, "same").await)
        };
        assert_eq!(original, replay);
        assert_eq!(message_count(&state, campus.id).await, 1);
        assert!(campus.rooms[..2].contains(&original.room_id));
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn same_nonce_from_two_authors_creates_two_messages() {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let member = member_session(&state, "member@example.org", campus.id, Role::Member).await;
        let addr = spawn_server_with(state.clone()).await;
        let (mut owner_socket, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let (mut member_socket, _) = joined_socket(addr, campus.id, &member.token).await;
        let room_id = campus.rooms[0];
        let (by_owner, by_member) = tokio::join!(
            posted(&mut owner_socket, room_id, NONCE_A, "same"),
            posted(&mut member_socket, room_id, NONCE_A, "same")
        );
        assert_ne!(by_owner.message_id, by_member.message_id);
        assert_eq!(by_owner.author_id, campus.owner.account.account_id);
        assert_eq!(by_member.author_id, member.account.account_id);
        assert_eq!(message_count(&state, campus.id).await, 2);
        let replay = posted(&mut owner_socket, room_id, NONCE_A, "same").await;
        assert_eq!(replay, by_owner);
        assert_eq!(message_count(&state, campus.id).await, 2);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn same_nonce_in_another_campus_is_independent() {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let north = campus_with_owner(&state, "north@example.org", "north-quad").await;
        let south = campus_with_owner(&state, "south@example.org", "south-quad").await;
        assert_ne!(north.id, south.id);
        let addr = spawn_server_with(state.clone()).await;
        let (mut north_socket, _) = joined_socket(addr, north.id, &north.owner.token).await;
        let (mut south_socket, _) = joined_socket(addr, south.id, &south.owner.token).await;
        let (north_post, south_post) = tokio::join!(
            posted(&mut north_socket, north.rooms[0], NONCE_A, "same"),
            posted(&mut south_socket, south.rooms[0], NONCE_A, "same")
        );
        assert_ne!(north_post.message_id, south_post.message_id);
        assert_eq!(north_post.campus_id, north.id);
        assert_eq!(south_post.campus_id, south.id);
        assert_eq!(message_count(&state, north.id).await, 1);
        assert_eq!(message_count(&state, south.id).await, 1);
    })
    .await
    .expect("scenario deadline");
}
