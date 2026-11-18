mod common;

use std::collections::BTreeMap;

use common::{
    SCENARIO_DEADLINE, assert_chain, campus_with_owner, created_messages, expect_frame,
    frames_until_pong, fresh_state, joined_socket, member_session, posted, seed_messages, send,
    spawn_server_with, subscribed_socket, sync_rooms, synced_page,
};
use pulse_protocol::{ClientNonce, MessageId, Role, RoomMessage, SYNC_PAGE_LIMIT, ServerFrame};
use rstest::rstest;
use tokio::time::timeout;
use uuid::Uuid;

#[rstest]
#[case::empty_room(0, None, vec![], false)]
#[case::below_limit(30, None, (1..=30).collect(), false)]
#[case::exactly_limit(SYNC_PAGE_LIMIT, None, (1..=SYNC_PAGE_LIMIT).collect(), false)]
#[case::latest_window_when_over_limit(130, None, (31..=130).collect(), false)]
#[case::cursor_with_more_pages(SYNC_PAGE_LIMIT + 2, Some(1), (2..=SYNC_PAGE_LIMIT + 1).collect(), true)]
#[case::cursor_leaving_exactly_one_page(SYNC_PAGE_LIMIT + 1, Some(1), (2..=SYNC_PAGE_LIMIT + 1).collect(), false)]
#[case::cursor_at_tip(5, Some(5), vec![], false)]
#[case::cursor_before_everything(3, Some(0), vec![1, 2, 3], false)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn first_page_follows_cursor_rules(
    #[case] seeded: usize,
    #[case] after_index: Option<usize>,
    #[case] expected: Vec<usize>,
    #[case] has_more: bool,
) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let room_id = campus.rooms[0];
        let ids = seed_messages(
            &state,
            campus.id,
            room_id,
            campus.owner.account.account_id,
            seeded,
        )
        .await;
        let after = after_index.map(|index| {
            index
                .checked_sub(1)
                .map_or(MessageId(Uuid::nil()), |position| ids[position])
        });
        let addr = spawn_server_with(state).await;
        let (mut socket, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let page = synced_page(&mut socket, room_id, after).await;
        assert_eq!(page.room_id, room_id);
        assert_eq!(
            page.messages
                .iter()
                .map(|message| message.message_id)
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|index| ids[index - 1])
                .collect::<Vec<_>>()
        );
        assert_eq!(page.has_more, has_more);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn following_has_more_walks_every_message_once() {
    timeout(SCENARIO_DEADLINE, async {
        let total = SYNC_PAGE_LIMIT * 2 + 37;
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let room_id = campus.rooms[0];
        let ids = seed_messages(
            &state,
            campus.id,
            room_id,
            campus.owner.account.account_id,
            total,
        )
        .await;
        let addr = spawn_server_with(state).await;
        let (mut socket, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let mut cursor = Some(MessageId(Uuid::nil()));
        let mut walked = Vec::new();
        let mut page_sizes = Vec::new();
        loop {
            let page = synced_page(&mut socket, room_id, cursor).await;
            page_sizes.push(page.messages.len());
            cursor = page.messages.last().map(|message| message.message_id);
            walked.extend(page.messages);
            if !page.has_more {
                break;
            }
        }
        assert_eq!(page_sizes, vec![SYNC_PAGE_LIMIT, SYNC_PAGE_LIMIT, 37]);
        assert_eq!(
            walked
                .iter()
                .map(|message| message.message_id)
                .collect::<Vec<_>>(),
            ids
        );
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::peer_posted_nothing(0)]
#[case::peer_posted_one(1)]
#[case::peer_posted_five(5)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reconnect_with_cursor_returns_exactly_the_missed_messages(#[case] missed: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let peer = member_session(&state, "peer@example.org", campus.id, Role::Member).await;
        let addr = spawn_server_with(state).await;
        let room_id = campus.rooms[0];
        let (mut peer_socket, _) = joined_socket(addr, campus.id, &peer.token).await;
        let mut watcher = subscribed_socket(addr, campus.id, &campus.owner.token, &[room_id]).await;
        let seen = posted(&mut peer_socket, room_id, ClientNonce::generate(), "before").await;
        assert_eq!(created_messages(&mut watcher, 1).await, vec![seen.clone()]);
        drop(watcher);
        let mut missed_messages: Vec<RoomMessage> = Vec::new();
        for index in 0..missed {
            let body = format!("missed {index}");
            missed_messages
                .push(posted(&mut peer_socket, room_id, ClientNonce::generate(), &body).await);
        }
        let (mut reconnected, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let page = synced_page(&mut reconnected, room_id, Some(seen.message_id)).await;
        assert_eq!(page.messages, missed_messages);
        assert!(!page.has_more);
        let live = posted(&mut peer_socket, room_id, ClientNonce::generate(), "live").await;
        assert_eq!(created_messages(&mut reconnected, 1).await, vec![live]);
        assert!(frames_until_pong(&mut reconnected, 9).await.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::small_burst(10, 20)]
#[case::burst_past_one_page(40, 130)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn posting_during_sync_leaves_sync_and_live_complete(
    #[case] live_posts: usize,
    #[case] backlog: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let peer = member_session(&state, "peer@example.org", campus.id, Role::Member).await;
        let room_id = campus.rooms[0];
        let seeded = seed_messages(
            &state,
            campus.id,
            room_id,
            campus.owner.account.account_id,
            backlog,
        )
        .await;
        let addr = spawn_server_with(state).await;
        let (mut peer_socket, _) = joined_socket(addr, campus.id, &peer.token).await;
        let (mut socket, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let cursor = seeded[4];
        let expected_total = backlog - 5 + live_posts;
        let poster = tokio::spawn(async move {
            for index in 0..live_posts {
                let body = format!("live {index}");
                posted(&mut peer_socket, room_id, ClientNonce::generate(), &body).await;
            }
        });
        send(&mut socket, &sync_rooms(&[(room_id, Some(cursor))])).await;
        let mut union: BTreeMap<MessageId, RoomMessage> = BTreeMap::new();
        while union.len() < expected_total {
            match expect_frame(&mut socket).await {
                ServerFrame::RoomSynced(page) => {
                    let tip = page.messages.last().map(|message| message.message_id);
                    page.messages.into_iter().for_each(|message| {
                        union.insert(message.message_id, message);
                    });
                    if let (true, Some(tip)) = (page.has_more, tip) {
                        send(&mut socket, &sync_rooms(&[(room_id, Some(tip))])).await;
                    }
                }
                ServerFrame::MessageCreated(created) => {
                    union.insert(created.message.message_id, created.message);
                }
                other => panic!("unexpected frame {other:?}"),
            }
        }
        poster.await.expect("poster task");
        assert_eq!(union.len(), expected_total);
        let ordered = union.into_values().collect::<Vec<_>>();
        assert_chain(&ordered, Some(cursor));
    })
    .await
    .expect("scenario deadline");
}
