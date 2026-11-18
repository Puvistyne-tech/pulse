mod common;

use common::{
    Observed, SCENARIO_DEADLINE, assert_chain, campus_with_owner, expect_frame, frames_until_pong,
    fresh_state, grant_role, joined_socket, member_session, message_count, observe, posted, send,
    send_message, spawn_server_with, stored_bodies, subscribed_socket, sync_rooms,
};
use pulse_protocol::fixtures::{NONCE_A, NONCE_B};
use pulse_protocol::{
    MESSAGE_BODY_MAX_CHARS, MessageAccepted, MessageCreated, MessageId, MessageRejected,
    MessageRejection, Role, RoomId, RoomUnavailable, SYNC_MAX_ROOMS, ServerFrame, close_code,
};
use pulse_server::message::next_message_id;
use rstest::rstest;
use tokio::time::timeout;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Clone, Copy)]
enum Target {
    Own,
    Foreign,
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Reply {
    Accepted,
    Rejected(MessageRejection),
}

#[rstest]
#[case::member_posts(Role::Member, Target::Own, "  hello  ".to_owned(), Reply::Accepted, vec!["hello".to_owned()])]
#[case::admin_posts(Role::Admin, Target::Own, "from admin".to_owned(), Reply::Accepted, vec!["from admin".to_owned()])]
#[case::multiline_body_kept(Role::Member, Target::Own, "a\nb\tc".to_owned(), Reply::Accepted, vec!["a\nb\tc".to_owned()])]
#[case::body_at_limit(
    Role::Member,
    Target::Own,
    "x".repeat(MESSAGE_BODY_MAX_CHARS),
    Reply::Accepted,
    vec!["x".repeat(MESSAGE_BODY_MAX_CHARS)],
)]
#[case::guest_forbidden(Role::Guest, Target::Own, "hi".to_owned(), Reply::Rejected(MessageRejection::Forbidden), vec![])]
#[case::blank_body(Role::Member, Target::Own, " \n ".to_owned(), Reply::Rejected(MessageRejection::InvalidBody), vec![])]
#[case::control_body(Role::Member, Target::Own, "a\u{7}b".to_owned(), Reply::Rejected(MessageRejection::InvalidBody), vec![])]
#[case::body_over_limit(
    Role::Member,
    Target::Own,
    "x".repeat(MESSAGE_BODY_MAX_CHARS + 1),
    Reply::Rejected(MessageRejection::InvalidBody),
    vec![],
)]
#[case::room_of_other_campus(Role::Member, Target::Foreign, "hi".to_owned(), Reply::Rejected(MessageRejection::UnknownRoom), vec![])]
#[case::room_does_not_exist(Role::Member, Target::Unknown, "hi".to_owned(), Reply::Rejected(MessageRejection::UnknownRoom), vec![])]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn posting_follows_role_body_and_room_rules(
    #[case] role: Role,
    #[case] target: Target,
    #[case] body: String,
    #[case] reply: Reply,
    #[case] stored: Vec<String>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let foreign = campus_with_owner(&state, "other@example.org", "south-quad").await;
        let sender = member_session(&state, "sender@example.org", campus.id, role).await;
        let addr = spawn_server_with(state.clone()).await;
        let mut watcher =
            subscribed_socket(addr, campus.id, &campus.owner.token, &campus.rooms[..1]).await;
        let (mut socket, _) = joined_socket(addr, campus.id, &sender.token).await;
        let room_id = match target {
            Target::Own => campus.rooms[0],
            Target::Foreign => foreign.rooms[0],
            Target::Unknown => RoomId::generate(),
        };
        send(&mut socket, &send_message(room_id, NONCE_A, &body)).await;
        let frame = expect_frame(&mut socket).await;
        let echoed = match reply {
            Reply::Accepted => vec![expect_frame(&mut watcher).await],
            Reply::Rejected(_) => frames_until_pong(&mut watcher, 7).await,
        };
        match (reply, frame) {
            (Reply::Accepted, ServerFrame::MessageAccepted(MessageAccepted { message })) => {
                assert_eq!(message.body, stored[0]);
                assert_eq!(message.room_id, room_id);
                assert_eq!(message.campus_id, campus.id);
                assert_eq!(message.author_id, sender.account.account_id);
                assert_eq!(message.author_display_name, "Fixture");
                assert_eq!(message.client_nonce, NONCE_A);
                assert_eq!(message.previous_message_id, None);
                assert_eq!(
                    echoed,
                    vec![ServerFrame::MessageCreated(MessageCreated { message })]
                );
            }
            (
                Reply::Rejected(reason),
                ServerFrame::MessageRejected(MessageRejected {
                    client_nonce,
                    reason: actual,
                }),
            ) => {
                assert_eq!(actual, reason);
                assert_eq!(client_nonce, NONCE_A);
                assert!(echoed.is_empty());
            }
            (_, other) => panic!("unexpected reply {other:?}"),
        }
        assert_eq!(stored_bodies(&state, campus.id).await, stored);
        assert_eq!(
            message_count(&state, campus.id).await,
            i64::try_from(stored.len()).expect("count")
        );
        assert_eq!(message_count(&state, foreign.id).await, 0);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn consecutive_posts_chain_within_each_room() {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let addr = spawn_server_with(state.clone()).await;
        let (mut socket, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let mut posts = Vec::new();
        for (index, room_index) in [0, 0, 1, 0, 1].into_iter().enumerate() {
            let room_id = campus.rooms[room_index];
            let nonce = pulse_protocol::ClientNonce::generate();
            posts.push(posted(&mut socket, room_id, nonce, &format!("post {index}")).await);
        }
        for room_id in &campus.rooms[..2] {
            let chain = posts
                .iter()
                .filter(|message| message.room_id == *room_id)
                .cloned()
                .collect::<Vec<_>>();
            assert_chain(&chain, None);
        }
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy)]
enum Change {
    Demoted(Role),
    Removed,
}

#[rstest]
#[case::demoted_to_guest(Change::Demoted(Role::Guest), false)]
#[case::removed_from_campus(Change::Removed, true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn open_socket_rereads_role_for_every_frame(
    #[case] change: Change,
    #[case] room_hidden: bool,
) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let member =
            member_session(&state, "member@example.org", campus.id, Role::Member).await;
        let addr = spawn_server_with(state.clone()).await;
        let (mut socket, _) = joined_socket(addr, campus.id, &member.token).await;
        let room_id = campus.rooms[0];
        posted(&mut socket, room_id, NONCE_A, "before").await;
        match change {
            Change::Demoted(role) => {
                grant_role(&state, campus.id, member.account.account_id, role).await;
            }
            Change::Removed => {
                sqlx::query("DELETE FROM campus_members WHERE campus_id = $1 AND account_id = $2")
                    .bind(campus.id.as_uuid())
                    .bind(member.account.account_id.as_uuid())
                    .execute(&state.db)
                    .await
                    .expect("remove member");
            }
        }
        send(&mut socket, &send_message(room_id, NONCE_B, "after")).await;
        assert_eq!(
            expect_frame(&mut socket).await,
            ServerFrame::MessageRejected(MessageRejected {
                client_nonce: NONCE_B,
                reason: MessageRejection::Forbidden,
            })
        );
        send(&mut socket, &sync_rooms(&[(room_id, None)])).await;
        let synced = expect_frame(&mut socket).await;
        assert_eq!(
            matches!(synced, ServerFrame::RoomUnavailable(RoomUnavailable { room_id: id }) if id == room_id),
            room_hidden,
            "{synced:?}"
        );
        assert_eq!(stored_bodies(&state, campus.id).await, vec!["before"]);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::at_limit(SYNC_MAX_ROOMS, None)]
#[case::over_limit(SYNC_MAX_ROOMS + 1, Some(close_code::POLICY_VIOLATION))]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn sync_rooms_enforces_cursor_limit(#[case] count: usize, #[case] close: Option<u16>) {
    timeout(SCENARIO_DEADLINE, async {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let addr = spawn_server_with(state).await;
        let (mut socket, _) = joined_socket(addr, campus.id, &campus.owner.token).await;
        let cursors = (0..count)
            .map(|_| (RoomId::generate(), None))
            .collect::<Vec<_>>();
        send(&mut socket, &sync_rooms(&cursors)).await;
        match close {
            Some(code) => assert_eq!(observe(&mut socket).await, Observed::Close(code)),
            None => {
                for (room_id, _) in &cursors {
                    assert_eq!(
                        expect_frame(&mut socket).await,
                        ServerFrame::RoomUnavailable(RoomUnavailable { room_id: *room_id })
                    );
                }
            }
        }
    })
    .await
    .expect("scenario deadline");
}

fn timestamped(offset_secs: i64) -> MessageId {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_secs();
    let seconds = now.checked_add_signed(offset_secs).expect("offset");
    MessageId(Uuid::new_v7(Timestamp::from_unix(
        ContextV7::new(),
        seconds,
        0,
    )))
}

#[rstest]
#[case::no_previous(None, 1, false)]
#[case::past_previous(Some(-3_600), 1, false)]
#[case::future_previous(Some(3_600), 1, true)]
#[case::future_chain(Some(7_200), 50, true)]
fn next_message_id_is_strictly_monotonic(
    #[case] offset_secs: Option<i64>,
    #[case] steps: usize,
    #[case] consecutive: bool,
) {
    let mut previous = offset_secs.map(timestamped);
    for _ in 0..steps {
        let next = next_message_id(previous);
        assert_eq!(next.as_uuid().get_version_num(), 7);
        if let Some(previous) = previous {
            assert!(next > previous);
            if consecutive {
                assert_eq!(next.as_uuid().as_u128(), previous.as_uuid().as_u128() + 1);
            }
        }
        previous = Some(next);
    }
}
