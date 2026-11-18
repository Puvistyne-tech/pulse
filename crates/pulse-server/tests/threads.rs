mod common;

use common::{
    Aim, Person, SCENARIO_DEADLINE, Stage, assert_chain, mention_rows, message_count, next_of,
    plain_socket, react, replied, room_activity, seed_replies, send, send_reply, set_title,
    synced_page_at, thread_page, title_outcome,
};
use pulse_protocol::fixtures::{EMOJI_A, NONCE_A};
use pulse_protocol::{
    ClientNonce, MessageRejected, MessageRejection, MessageUpdated, ReactionAction,
    SYNC_PAGE_LIMIT, ServerFrame, TOPIC_TITLE_MAX_CHARS, ThreadRejected, ThreadRejection,
    mention_token,
};
use rstest::rstest;
use tokio::time::timeout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Accepted,
    Rejected(MessageRejection),
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn replies_join_the_room_chain_and_update_thread_stats() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = plain_socket(&stage, &stage.ada).await;
        let mut grace = plain_socket(&stage, &stage.grace).await;
        let root = stage.root(&mut ada, "root").await;
        let other = stage.root(&mut ada, "other root").await;
        let first = replied(
            &mut grace,
            stage.room,
            root.message_id,
            ClientNonce::generate(),
            "one",
        )
        .await;
        let second = replied(
            &mut ada,
            stage.room,
            root.message_id,
            ClientNonce::generate(),
            "two",
        )
        .await;
        let mut reader = stage.socket(&stage.linus).await;
        let page = synced_page_at(&mut reader, stage.room, None, None).await;
        assert_chain(&page.messages, None);
        assert_eq!(
            page.messages
                .iter()
                .map(|message| message.message_id)
                .collect::<Vec<_>>(),
            vec![
                root.message_id,
                other.message_id,
                first.message_id,
                second.message_id
            ]
        );
        let stored_root = &page.messages[0];
        assert_eq!(stored_root.thread_root_id, None);
        assert_eq!(stored_root.reply_count, 2);
        assert_eq!(stored_root.latest_reply_id, Some(second.message_id));
        assert_eq!(page.messages[1].reply_count, 0);
        assert_eq!(page.messages[1].latest_reply_id, None);
        for reply in &page.messages[2..] {
            assert_eq!(reply.thread_root_id, Some(root.message_id));
            assert_eq!(reply.reply_count, 0);
            assert_eq!(reply.latest_reply_id, None);
            assert_eq!(reply.topic_title, None);
        }
        assert_eq!(first.previous_message_id, Some(other.message_id));
        assert_eq!(second.previous_message_id, Some(first.message_id));
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::reply_to_top_level(Person::Grace, Aim::Top, "fine", Outcome::Accepted)]
#[case::reply_to_a_reply(
    Person::Grace,
    Aim::Reply,
    "nested",
    Outcome::Rejected(MessageRejection::UnknownThread)
)]
#[case::reply_to_unknown_message(
    Person::Grace,
    Aim::Unknown,
    "lost",
    Outcome::Rejected(MessageRejection::UnknownThread)
)]
#[case::reply_to_root_of_another_room(
    Person::Grace,
    Aim::Sibling,
    "wrong room",
    Outcome::Rejected(MessageRejection::UnknownThread)
)]
#[case::guest_cannot_reply(
    Person::Guest,
    Aim::Top,
    "hi",
    Outcome::Rejected(MessageRejection::Forbidden)
)]
#[case::guest_forbidden_before_thread_check(
    Person::Guest,
    Aim::Unknown,
    "hi",
    Outcome::Rejected(MessageRejection::Forbidden)
)]
#[case::blank_reply_body(
    Person::Grace,
    Aim::Top,
    "  ",
    Outcome::Rejected(MessageRejection::InvalidBody)
)]
#[case::body_invalid_before_thread_check(
    Person::Grace,
    Aim::Unknown,
    "a\u{7}",
    Outcome::Rejected(MessageRejection::InvalidBody)
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn replies_follow_thread_rules(
    #[case] who: Person,
    #[case] root: Aim,
    #[case] body: &str,
    #[case] expected: Outcome,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let target = stage.roots().await.pick(root);
        let session = stage.session(who);
        let mut sender = plain_socket(&stage, session).await;
        send(&mut sender, &send_reply(stage.room, target, NONCE_A, body)).await;
        let outcome = next_of(&mut sender, |frame| match frame {
            ServerFrame::MessageAccepted(_) => Some(Outcome::Accepted),
            ServerFrame::MessageRejected(MessageRejected {
                client_nonce,
                reason,
            }) => {
                assert_eq!(*client_nonce, NONCE_A);
                Some(Outcome::Rejected(*reason))
            }
            _ => None,
        })
        .await;
        assert_eq!(outcome, expected);
        let stored = 3 + i64::from(expected == Outcome::Accepted);
        assert_eq!(message_count(&stage.state, stage.campus.id).await, stored);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::set(vec!["Plans".to_owned()], Some("Plans"), 1)]
#[case::trimmed(vec!["  Plans  ".to_owned()], Some("Plans"), 1)]
#[case::replaced(vec!["Plans".to_owned(), "Agenda".to_owned()], Some("Agenda"), 2)]
#[case::cleared(vec!["Plans".to_owned(), String::new()], None, 2)]
#[case::blank_clears(vec!["Plans".to_owned(), "   ".to_owned()], None, 2)]
#[case::clearing_nothing_is_accepted(vec![String::new()], None, 1)]
#[case::at_limit(vec!["t".repeat(TOPIC_TITLE_MAX_CHARS)], Some("t"), 1)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn topic_titles_update_the_root_and_bump_activity(
    #[case] titles: Vec<String>,
    #[case] final_title: Option<&str>,
    #[case] bumps: i64,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = plain_socket(&stage, &stage.ada).await;
        let root = stage.root(&mut ada, "root").await;
        let mut watcher = stage.socket(&stage.linus).await;
        let mut grace = plain_socket(&stage, &stage.grace).await;
        let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
        let mut updated = Vec::new();
        for title in &titles {
            send(&mut grace, &set_title(stage.room, root.message_id, title)).await;
            match title_outcome(&mut grace).await {
                ServerFrame::MessageUpdated(MessageUpdated { message }) => updated.push(message),
                other => panic!("expected update, got {other:?}"),
            }
        }
        let expected_title = final_title.map(|text| {
            if text == "t" {
                "t".repeat(TOPIC_TITLE_MAX_CHARS)
            } else {
                text.to_owned()
            }
        });
        let last = updated.last().expect("update");
        assert_eq!(last.message_id, root.message_id);
        assert_eq!(last.topic_title, expected_title);
        assert_eq!(
            room_activity(&stage.state, stage.campus.id, stage.room).await,
            before + bumps
        );
        assert_eq!(
            last.activity_seq,
            u64::try_from(before + bumps).expect("sequence")
        );
        let broadcast = next_of(&mut watcher, |frame| match frame {
            ServerFrame::MessageUpdated(MessageUpdated { message })
                if message.activity_seq == last.activity_seq =>
            {
                Some(message.clone())
            }
            _ => None,
        })
        .await;
        assert_eq!(&broadcast, last);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::guest_cannot_title(Person::Guest, Aim::Top, ThreadRejection::Forbidden)]
#[case::title_on_a_reply(Person::Grace, Aim::Reply, ThreadRejection::UnknownThread)]
#[case::title_on_unknown_message(Person::Grace, Aim::Unknown, ThreadRejection::UnknownThread)]
#[case::title_on_root_of_another_room(Person::Grace, Aim::Sibling, ThreadRejection::UnknownThread)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn topic_title_rejections_leave_activity_untouched(
    #[case] who: Person,
    #[case] root: Aim,
    #[case] reason: ThreadRejection,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let target = stage.roots().await.pick(root);
        let session = stage.session(who);
        let mut sender = plain_socket(&stage, session).await;
        let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
        send(&mut sender, &set_title(stage.room, target, "Plans")).await;
        assert_eq!(
            title_outcome(&mut sender).await,
            ServerFrame::ThreadRejected(ThreadRejected {
                room_id: stage.room,
                root_message_id: target,
                reason,
            })
        );
        assert_eq!(
            room_activity(&stage.state, stage.campus.id, stage.room).await,
            before
        );
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::control_character("bad\u{7}title")]
#[case::over_limit(&"t".repeat(TOPIC_TITLE_MAX_CHARS + 1))]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn invalid_topic_titles_are_rejected(#[case] title: &str) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = plain_socket(&stage, &stage.ada).await;
        let root = stage.root(&mut ada, "root").await;
        let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
        send(&mut ada, &set_title(stage.room, root.message_id, title)).await;
        assert_eq!(
            title_outcome(&mut ada).await,
            ServerFrame::ThreadRejected(ThreadRejected {
                room_id: stage.room,
                root_message_id: root.message_id,
                reason: ThreadRejection::InvalidTopicTitle,
            })
        );
        assert_eq!(
            room_activity(&stage.state, stage.campus.id, stage.room).await,
            before
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reconnect_delta_sync_delivers_replies_and_updates_without_gaps_or_duplicates() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.socket(&stage.ada).await;
        let root = stage.root(&mut ada, "root").await;
        let quiet = stage.root(&mut ada, "quiet").await;
        let initial = synced_page_at(&mut ada, stage.room, None, None).await;
        let cursor = initial.messages.last().map(|message| message.message_id);
        let seen_activity = initial.activity_seq;
        drop(ada);
        let mut grace = plain_socket(&stage, &stage.grace).await;
        let reply = replied(
            &mut grace,
            stage.room,
            root.message_id,
            ClientNonce::generate(),
            "while away",
        )
        .await;
        send(&mut grace, &set_title(stage.room, root.message_id, "Plans")).await;
        title_outcome(&mut grace).await;
        send(
            &mut grace,
            &react(stage.room, root.message_id, EMOJI_A, ReactionAction::Add),
        )
        .await;
        common::reaction_outcome(&mut grace).await;
        let mut returning = plain_socket(&stage, &stage.ada).await;
        let catch_up =
            synced_page_at(&mut returning, stage.room, cursor, Some(seen_activity)).await;
        assert_eq!(
            catch_up
                .messages
                .iter()
                .map(|message| message.message_id)
                .collect::<Vec<_>>(),
            vec![reply.message_id]
        );
        assert_chain(&catch_up.messages, cursor);
        assert_eq!(catch_up.updates.len(), 1);
        assert_eq!(catch_up.updates[0].message_id, root.message_id);
        assert_eq!(catch_up.updates[0].topic_title.as_deref(), Some("Plans"));
        assert_eq!(catch_up.updates[0].reactions[0].emoji, EMOJI_A);
        assert!(
            catch_up
                .updates
                .iter()
                .all(|update| update.message_id != quiet.message_id)
        );
        assert_eq!(
            catch_up.activity_seq,
            u64::try_from(room_activity(&stage.state, stage.campus.id, stage.room).await)
                .expect("sequence")
        );
        assert!(!catch_up.has_more);
        let again = synced_page_at(
            &mut returning,
            stage.room,
            Some(reply.message_id),
            Some(catch_up.activity_seq),
        )
        .await;
        assert!(again.messages.is_empty());
        assert!(again.updates.is_empty());
        assert_eq!(again.activity_seq, catch_up.activity_seq);
        let full = synced_page_at(&mut returning, stage.room, None, None).await;
        assert_eq!(full.messages.len(), 3);
        assert_eq!(full.messages[0], catch_up.updates[0]);
        assert_eq!(full.messages[2], catch_up.messages[0]);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn thread_sync_pages_the_root_and_every_reply_once() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = plain_socket(&stage, &stage.ada).await;
        let root = stage.root(&mut ada, "root").await;
        let seeded = seed_replies(
            &stage.state,
            stage.campus.id,
            stage.room,
            root.message_id,
            common::Stage::account(&stage.grace),
            root.message_id,
            SYNC_PAGE_LIMIT + 6,
        )
        .await;
        let mut guest = plain_socket(&stage, &stage.guest).await;
        let first = thread_page(&mut guest, stage.room, root.message_id, None).await;
        assert_eq!(first.messages.len(), SYNC_PAGE_LIMIT);
        assert!(first.has_more);
        assert_eq!(first.messages[0].message_id, root.message_id);
        let second = thread_page(
            &mut guest,
            stage.room,
            root.message_id,
            first.messages.last().map(|message| message.message_id),
        )
        .await;
        assert!(!second.has_more);
        let walked = first
            .messages
            .iter()
            .chain(second.messages.iter())
            .map(|message| message.message_id)
            .collect::<Vec<_>>();
        let mut expected = vec![root.message_id];
        expected.extend(seeded);
        assert_eq!(walked, expected);
        assert_eq!(
            first.messages[0].reply_count,
            u32::try_from(SYNC_PAGE_LIMIT + 6).expect("count")
        );
        let tail = thread_page(
            &mut guest,
            stage.room,
            root.message_id,
            expected.last().copied(),
        )
        .await;
        assert!(tail.messages.is_empty());
        assert!(!tail.has_more);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::reply_is_not_a_root(Aim::Reply)]
#[case::unknown_message(Aim::Unknown)]
#[case::root_of_another_room(Aim::Sibling)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn thread_sync_of_a_non_root_is_unknown_thread(#[case] root: Aim) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let target = stage.roots().await.pick(root);
        let mut ada = plain_socket(&stage, &stage.ada).await;
        send(&mut ada, &common::sync_thread(stage.room, target, None)).await;
        let frame = next_of(&mut ada, |frame| match frame {
            ServerFrame::ThreadRejected(_) | ServerFrame::ThreadSynced(_) => Some(frame.clone()),
            _ => None,
        })
        .await;
        assert_eq!(
            frame,
            ServerFrame::ThreadRejected(ThreadRejected {
                room_id: stage.room,
                root_message_id: target,
                reason: ThreadRejection::UnknownThread,
            })
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn mentions_resolve_members_and_ignore_strangers() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = plain_socket(&stage, &stage.ada).await;
        let root = stage.root(&mut ada, "root").await;
        let stranger = pulse_protocol::AccountId::generate();
        let body = format!(
            "hi {} and {} and {} and {}",
            mention_token(Stage::account(&stage.grace)),
            mention_token(stranger),
            mention_token(Stage::account(&stage.grace)),
            mention_token(Stage::account(&stage.linus)),
        );
        let reply = replied(
            &mut ada,
            stage.room,
            root.message_id,
            ClientNonce::generate(),
            &body,
        )
        .await;
        let mut expected = vec![Stage::account(&stage.grace), Stage::account(&stage.linus)];
        expected.sort();
        assert_eq!(
            mention_rows(&stage.state, stage.campus.id, reply.message_id).await,
            expected
        );
        let mut mentioned = reply
            .mentions
            .iter()
            .map(|mention| mention.account_id)
            .collect::<Vec<_>>();
        mentioned.sort();
        assert_eq!(mentioned, expected);
        assert!(
            reply
                .mentions
                .iter()
                .all(|mention| mention.display_name == "Fixture")
        );
        assert_eq!(reply.body, body);
    })
    .await
    .expect("scenario deadline");
}
