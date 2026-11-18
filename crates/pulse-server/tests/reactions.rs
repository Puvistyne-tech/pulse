mod common;

use std::collections::BTreeSet;

use common::{
    Person, SCENARIO_DEADLINE, Stage, react, reaction_outcome, reaction_rows, room_activity,
    seed_messages, send, settled_frames, synced_page_at,
};
use pulse_protocol::fixtures::{EMOJI_A, EMOJI_B};
use pulse_protocol::{
    ClientNonce, MessageId, MessageUpdated, REACTION_KINDS_MAX, ReactionAction, ReactionRejected,
    ReactionRejection, RoomId, SYNC_PAGE_LIMIT, ServerFrame,
};
use rstest::rstest;
use tokio::time::timeout;

use ReactionAction::{Add, Remove};

fn kind(index: usize) -> String {
    char::from_u32(0x1F600 + u32::try_from(index).expect("index"))
        .expect("emoji")
        .to_string()
}

async fn updated(socket: &mut common::ClientSocket) -> pulse_protocol::RoomMessage {
    match reaction_outcome(socket).await {
        ServerFrame::MessageUpdated(MessageUpdated { message }) => message,
        other => panic!("expected update, got {other:?}"),
    }
}

#[rstest]
#[case::add(vec![Add], vec![1], 1)]
#[case::add_twice_changes_once(vec![Add, Add], vec![1, 0], 1)]
#[case::remove_without_add_changes_nothing(vec![Remove], vec![0], 0)]
#[case::add_then_remove(vec![Add, Remove], vec![1, 1], 0)]
#[case::remove_twice_changes_once(vec![Add, Remove, Remove], vec![1, 1, 0], 0)]
#[case::add_remove_add(vec![Add, Remove, Add], vec![1, 1, 1], 1)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reactions_are_idempotent_and_bump_activity_only_when_changed(
    #[case] actions: Vec<ReactionAction>,
    #[case] bumps: Vec<i64>,
    #[case] final_count: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let mut watcher = stage.watching(Person::Linus).await;
        let mut poster = stage.poster().await;
        let mut grace = stage.plain(Person::Grace).await;
        let mut observed = Vec::new();
        let mut returned = Vec::new();
        for action in &actions {
            let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
            send(&mut grace, &react(stage.room, root, EMOJI_A, *action)).await;
            returned.push(updated(&mut grace).await);
            let after = room_activity(&stage.state, stage.campus.id, stage.room).await;
            observed.push(after - before);
        }
        assert_eq!(observed, bumps);
        let last = returned.last().expect("reply");
        assert_eq!(last.reactions.len(), final_count);
        assert_eq!(
            reaction_rows(&stage.state, stage.campus.id, root)
                .await
                .len(),
            final_count
        );
        let total_bumps = bumps.iter().sum::<i64>();
        assert_eq!(
            last.activity_seq,
            u64::try_from(room_activity(&stage.state, stage.campus.id, stage.room).await)
                .expect("sequence")
        );
        let frames = settled_frames(&mut watcher, &mut poster, stage.room).await;
        let broadcast = frames
            .iter()
            .filter(|frame| matches!(frame, ServerFrame::MessageUpdated(_)))
            .count();
        assert_eq!(i64::try_from(broadcast).expect("count"), total_bumps);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn tallies_are_sorted_by_emoji_and_account() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let plan = [
            (Person::Grace, "✅"),
            (Person::Grace, EMOJI_A),
            (Person::Grace, EMOJI_B),
            (Person::Linus, EMOJI_B),
            (Person::Linus, EMOJI_A),
            (Person::Ada, EMOJI_A),
            (Person::Guest, EMOJI_A),
        ];
        let mut last = None;
        for (person, emoji) in plan {
            let mut socket = stage.plain(person).await;
            send(&mut socket, &react(stage.room, root, emoji, Add)).await;
            last = Some(updated(&mut socket).await);
        }
        let message = last.expect("reaction");
        let emoji = message
            .reactions
            .iter()
            .map(|tally| tally.emoji.clone())
            .collect::<Vec<_>>();
        let mut sorted = emoji.clone();
        sorted.sort();
        assert_eq!(emoji, sorted);
        assert_eq!(emoji.len(), 3);
        for tally in &message.reactions {
            let mut accounts = tally.account_ids.clone();
            accounts.sort();
            assert_eq!(tally.account_ids, accounts);
        }
        let counts = message
            .reactions
            .iter()
            .map(|tally| (tally.emoji.as_str(), tally.account_ids.len()))
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_eq!(counts[EMOJI_A], 4);
        assert_eq!(counts[EMOJI_B], 2);
        assert_eq!(counts["✅"], 1);
        let mut reader = stage.watching(Person::Linus).await;
        let page = synced_page_at(&mut reader, stage.room, None, None).await;
        assert_eq!(page.messages[0].reactions, message.reactions);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::below_the_cap_new_kind(REACTION_KINDS_MAX - 1, REACTION_KINDS_MAX - 1, None)]
#[case::at_the_cap_existing_kind(REACTION_KINDS_MAX, 3, None)]
#[case::at_the_cap_new_kind(
    REACTION_KINDS_MAX,
    REACTION_KINDS_MAX,
    Some(ReactionRejection::TooManyKinds)
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn distinct_emoji_per_message_are_capped(
    #[case] existing: usize,
    #[case] requested: usize,
    #[case] rejection: Option<ReactionRejection>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let root = stage.root(&mut ada, "root").await.message_id;
        for index in 0..existing {
            sqlx::query(
                "INSERT INTO message_reactions (campus_id, room_id, message_id, account_id, emoji) \
                 VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(stage.campus.id.as_uuid())
            .bind(stage.room.as_uuid())
            .bind(root.as_uuid())
            .bind(stage.id_of(Person::Linus).as_uuid())
            .bind(kind(index))
            .execute(&stage.state.db)
            .await
            .expect("seed reaction");
        }
        let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
        let rows_before = reaction_rows(&stage.state, stage.campus.id, root)
            .await
            .len();
        let mut grace = stage.plain(Person::Grace).await;
        send(&mut grace, &react(stage.room, root, &kind(requested), Add)).await;
        let frame = reaction_outcome(&mut grace).await;
        match rejection {
            None => {
                assert!(matches!(frame, ServerFrame::MessageUpdated(_)));
                assert_eq!(
                    reaction_rows(&stage.state, stage.campus.id, root)
                        .await
                        .len(),
                    rows_before + 1
                );
                assert_eq!(
                    room_activity(&stage.state, stage.campus.id, stage.room).await,
                    before + 1
                );
            }
            Some(reason) => {
                assert_eq!(
                    frame,
                    ServerFrame::ReactionRejected(ReactionRejected {
                        room_id: stage.room,
                        message_id: root,
                        emoji: kind(requested),
                        reason,
                    })
                );
                assert_eq!(
                    reaction_rows(&stage.state, stage.campus.id, root)
                        .await
                        .len(),
                    rows_before
                );
                assert_eq!(
                    room_activity(&stage.state, stage.campus.id, stage.room).await,
                    before
                );
            }
        }
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy)]
enum Where {
    Message,
    UnknownMessage,
    SiblingRoomMessage,
    UnknownRoom,
}

#[rstest]
#[case::ascii_is_invalid("a", Where::Message, ReactionRejection::InvalidEmoji)]
#[case::empty_is_invalid("", Where::Message, ReactionRejection::InvalidEmoji)]
#[case::blank_is_invalid("   ", Where::Message, ReactionRejection::InvalidEmoji)]
#[case::two_emoji_with_space_are_invalid("👍 🎉", Where::Message, ReactionRejection::InvalidEmoji)]
#[case::emoji_checked_before_message("a", Where::UnknownMessage, ReactionRejection::InvalidEmoji)]
#[case::unknown_message(EMOJI_A, Where::UnknownMessage, ReactionRejection::UnknownMessage)]
#[case::message_of_another_room(
    EMOJI_A,
    Where::SiblingRoomMessage,
    ReactionRejection::UnknownMessage
)]
#[case::unknown_room(EMOJI_A, Where::UnknownRoom, ReactionRejection::UnknownMessage)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn invalid_reactions_are_rejected_without_side_effects(
    #[case] emoji: &str,
    #[case] place: Where,
    #[case] reason: ReactionRejection,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let sibling = stage
            .root_in(&mut ada, stage.campus.rooms[1], "elsewhere")
            .await
            .message_id;
        let (room_id, message_id) = match place {
            Where::Message => (stage.room, root),
            Where::UnknownMessage => (stage.room, MessageId::generate()),
            Where::SiblingRoomMessage => (stage.room, sibling),
            Where::UnknownRoom => (RoomId::generate(), root),
        };
        let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
        let mut grace = stage.plain(Person::Grace).await;
        send(&mut grace, &react(room_id, message_id, emoji, Add)).await;
        assert_eq!(
            reaction_outcome(&mut grace).await,
            ServerFrame::ReactionRejected(ReactionRejected {
                room_id,
                message_id,
                emoji: emoji.to_owned(),
                reason,
            })
        );
        assert_eq!(
            common::count_rows(&stage.state, "message_reactions", stage.campus.id).await,
            0
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
async fn guests_may_react() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let mut guest = stage.plain(Person::Guest).await;
        send(&mut guest, &react(stage.room, root, EMOJI_A, Add)).await;
        let message = updated(&mut guest).await;
        assert_eq!(message.reactions.len(), 1);
        assert_eq!(
            message.reactions[0].account_ids,
            vec![stage.id_of(Person::Guest)]
        );
        assert_eq!(
            reaction_rows(&stage.state, stage.campus.id, root).await,
            vec![(EMOJI_A.to_owned(), stage.id_of(Person::Guest))]
        );
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy)]
struct Touch(usize, ReactionAction);

#[rstest]
#[case::everything_from_the_start(Some(0), vec![Touch(0, Add), Touch(1, Add), Touch(2, Add)], vec![0, 1, 2])]
#[case::missing_cursor_counts_as_zero(None, vec![Touch(0, Add), Touch(1, Add), Touch(2, Add)], vec![0, 1, 2])]
#[case::only_newer_activity(Some(2), vec![Touch(0, Add), Touch(1, Add), Touch(2, Add)], vec![2])]
#[case::fully_caught_up(Some(3), vec![Touch(0, Add), Touch(1, Add), Touch(2, Add)], vec![])]
#[case::removal_is_an_update_too(Some(0), vec![Touch(0, Add), Touch(1, Add), Touch(0, Remove)], vec![1, 0])]
#[case::repeated_changes_deliver_the_message_once(Some(0), vec![Touch(0, Add), Touch(0, Remove), Touch(0, Add)], vec![0])]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reconnect_receives_reaction_updates_newer_than_the_activity_cursor(
    #[case] activity_after: Option<u64>,
    #[case] touches: Vec<Touch>,
    #[case] delivered: Vec<usize>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ids = seed_messages(
            &stage.state,
            stage.campus.id,
            stage.room,
            stage.id_of(Person::Ada),
            3,
        )
        .await;
        let tip = ids[2];
        let mut grace = stage.plain(Person::Grace).await;
        for Touch(index, action) in &touches {
            send(
                &mut grace,
                &react(stage.room, ids[*index], EMOJI_A, *action),
            )
            .await;
            updated(&mut grace).await;
        }
        let mut returning = stage.plain(Person::Linus).await;
        let page = synced_page_at(&mut returning, stage.room, Some(tip), activity_after).await;
        assert!(page.messages.is_empty());
        assert!(!page.has_more);
        assert_eq!(
            page.updates
                .iter()
                .map(|update| update.message_id)
                .collect::<Vec<_>>(),
            delivered
                .iter()
                .map(|index| ids[*index])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            page.activity_seq,
            u64::try_from(room_activity(&stage.state, stage.campus.id, stage.room).await)
                .expect("sequence")
        );
        let tallies = page
            .updates
            .iter()
            .map(|update| update.reactions.len())
            .collect::<Vec<_>>();
        let expected = delivered
            .iter()
            .map(|index| {
                let last = touches
                    .iter()
                    .rfind(|Touch(touched, _)| touched == index)
                    .expect("touch");
                usize::from(last.1 == Add)
            })
            .collect::<Vec<_>>();
        assert_eq!(tallies, expected);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reaction_updates_page_by_activity_without_gaps_or_duplicates() {
    timeout(SCENARIO_DEADLINE, async {
        let total = SYNC_PAGE_LIMIT + 5;
        let stage = Stage::open().await;
        let ids = seed_messages(
            &stage.state,
            stage.campus.id,
            stage.room,
            stage.id_of(Person::Ada),
            total,
        )
        .await;
        let tip = *ids.last().expect("tip");
        let mut grace = stage.plain(Person::Grace).await;
        for id in &ids {
            send(&mut grace, &react(stage.room, *id, EMOJI_A, Add)).await;
            updated(&mut grace).await;
        }
        let mut returning = stage.plain(Person::Linus).await;
        let mut cursor = Some(0);
        let mut delivered = Vec::new();
        let mut page_sizes = Vec::new();
        let final_sequence = loop {
            let page = synced_page_at(&mut returning, stage.room, Some(tip), cursor).await;
            assert!(page.messages.is_empty());
            page_sizes.push(page.updates.len());
            delivered.extend(page.updates.iter().map(|update| update.message_id));
            cursor = Some(page.activity_seq);
            if !page.has_more {
                break page.activity_seq;
            }
        };
        assert_eq!(page_sizes, vec![SYNC_PAGE_LIMIT, 5]);
        assert_eq!(delivered, ids);
        assert_eq!(delivered.iter().collect::<BTreeSet<_>>().len(), total);
        assert_eq!(
            final_sequence,
            u64::try_from(room_activity(&stage.state, stage.campus.id, stage.room).await)
                .expect("sequence")
        );
        let caught_up =
            synced_page_at(&mut returning, stage.room, Some(tip), Some(final_sequence)).await;
        assert!(caught_up.updates.is_empty());
        assert!(!caught_up.has_more);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn reaction_on_a_reply_updates_the_reply_and_follows_the_thread() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let reply =
            common::replied(&mut ada, stage.room, root, ClientNonce::generate(), "reply").await;
        let mut grace = stage.plain(Person::Grace).await;
        send(
            &mut grace,
            &react(stage.room, reply.message_id, EMOJI_A, Add),
        )
        .await;
        let message = updated(&mut grace).await;
        assert_eq!(message.message_id, reply.message_id);
        assert_eq!(message.thread_root_id, Some(root));
        assert_eq!(message.reactions.len(), 1);
        let follow = stage.row_of(root, Person::Grace).await.expect("follow row");
        assert_eq!((follow.0.as_str(), follow.1), ("following", None));
    })
    .await
    .expect("scenario deadline");
}
