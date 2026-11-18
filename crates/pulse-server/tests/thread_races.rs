mod common;

use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use common::{
    ClientSocket, Person, SCENARIO_DEADLINE, Stage, assert_chain, changed_summaries, extra_members,
    inbox_of, react, reaction_rows, replied, room_activity, send, set_subscription, settle_all,
    synced_page_at,
};
use pulse_protocol::fixtures::{EMOJI_A, EMOJI_B};
use pulse_protocol::{
    ClientNonce, ReactionAction, RoomMessage, ServerFrame, SubscriptionOrigin, SubscriptionState,
};
use rstest::rstest;
use tokio::task::JoinSet;
use tokio::time::timeout;

async fn sample_inbox_until(
    mut socket: ClientSocket,
    stop: Arc<AtomicBool>,
) -> (ClientSocket, usize) {
    let mut samples = 0;
    while !stop.load(Ordering::SeqCst) {
        for summary in inbox_of(&mut socket).await {
            assert_eq!(summary.unread_replies, summary.root.reply_count);
            assert_eq!(
                summary.latest_reply.as_ref().map(|reply| reply.message_id),
                summary.root.latest_reply_id
            );
            samples += 1;
        }
    }
    (socket, samples)
}

#[rstest]
#[case::four_senders(4)]
#[case::sixteen_senders(16)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_replies_leave_every_follower_socket_with_exact_unread(#[case] senders: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let mut poster = stage.poster().await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let mut linus_a = stage.watching(Person::Linus).await;
        let linus_b = stage.watching(Person::Linus).await;
        let sampling = stage.watching(Person::Linus).await;
        send(
            &mut linus_a,
            &set_subscription(
                stage.room,
                root,
                SubscriptionState::Following,
                SubscriptionOrigin::Explicit,
            ),
        )
        .await;
        common::frames_until_pong(&mut linus_a, 4).await;
        let stop = Arc::new(AtomicBool::new(false));
        let sampler = tokio::spawn(sample_inbox_until(sampling, Arc::clone(&stop)));
        let members = extra_members(&stage.state, stage.campus.id, senders).await;
        let mut replying = JoinSet::new();
        for member in members {
            let (addr, campus_id, room) = (stage.addr, stage.campus.id, stage.room);
            replying.spawn(async move {
                let (mut socket, _) = common::joined_socket(addr, campus_id, &member.token).await;
                let reply = replied(&mut socket, room, root, ClientNonce::generate(), "race").await;
                common::frames_until_pong(&mut socket, 8).await;
                reply
            });
        }
        let mut replies = Vec::new();
        while let Some(done) = replying.join_next().await {
            replies.push(done.expect("sender task"));
        }
        stop.store(true, Ordering::SeqCst);
        let (_sampling, samples) = sampler.await.expect("sampler task");
        assert!(samples > 0);
        let mut sockets = [linus_a, linus_b];
        let frames = settle_all(&mut sockets, &mut poster, stage.room).await;
        let expected = replies
            .iter()
            .map(|reply| reply.message_id)
            .collect::<BTreeSet<_>>();
        assert_eq!(expected.len(), senders);
        for seen in &frames {
            let created = seen
                .iter()
                .filter_map(|frame| match frame {
                    ServerFrame::MessageCreated(created)
                        if created.message.thread_root_id == Some(root) =>
                    {
                        Some(created.message.message_id)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(created.len(), senders);
            assert_eq!(created.iter().copied().collect::<BTreeSet<_>>(), expected);
        }
        let page = synced_page_at(&mut sockets[0], stage.room, None, None).await;
        assert_chain(&page.messages, None);
        assert_eq!(page.messages.len(), senders + 2);
        let reply_count = u32::try_from(senders).expect("count");
        for socket in &mut sockets {
            let threads = inbox_of(socket).await;
            assert_eq!(threads.len(), 1);
            assert_eq!(threads[0].unread_replies, reply_count);
            assert_eq!(threads[0].unread_mentions, 0);
            assert_eq!(
                threads
                    .iter()
                    .filter(|thread| thread.unread_replies > 0)
                    .count(),
                1
            );
        }
        let ada_threads = inbox_of(&mut ada).await;
        assert_eq!(ada_threads[0].unread_replies, reply_count);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::eight_rounds(8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn automatic_follow_racing_an_explicit_mute_converges_on_the_highest_revision(
    #[case] rounds: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut poster = stage.poster().await;
        let mut author = stage.plain(Person::Ada).await;
        let mut phone = stage.watching(Person::Ada).await;
        let mut laptop = stage.watching(Person::Ada).await;
        let mut interleavings = BTreeSet::new();
        for round in 0..rounds {
            let root = stage
                .root(&mut author, &format!("root {round}"))
                .await
                .message_id;
            let (addr, campus_id, room) = (stage.addr, stage.campus.id, stage.room);
            let grace_token = stage.grace.token.clone();
            let replier = tokio::spawn(async move {
                let (mut socket, _) = common::joined_socket(addr, campus_id, &grace_token).await;
                replied(&mut socket, room, root, ClientNonce::generate(), "racing").await;
                common::frames_until_pong(&mut socket, 6).await;
            });
            send(
                &mut phone,
                &set_subscription(
                    room,
                    root,
                    SubscriptionState::Muted,
                    SubscriptionOrigin::Explicit,
                ),
            )
            .await;
            let direct = common::frames_until_pong(&mut phone, 6).await;
            replier.await.expect("replier task");
            let mut sockets = [phone, laptop];
            let mut frames = settle_all(&mut sockets, &mut poster, room).await;
            frames[0].extend(direct);
            let [rejoined_phone, rejoined_laptop] = sockets;
            phone = rejoined_phone;
            laptop = rejoined_laptop;
            let row = stage.row_of(root, Person::Ada).await.expect("one row");
            assert_eq!(row.0, "muted");
            interleavings.insert(row.2);
            let stored: i64 =
                common::count_rows(&stage.state, "thread_subscriptions", stage.campus.id).await;
            assert_eq!(stored, 2 * i64::try_from(round + 1).expect("round"));
            for seen in &frames {
                let latest = changed_summaries(seen)
                    .into_iter()
                    .filter(|summary| summary.root.message_id == root)
                    .max_by_key(|summary| summary.revision)
                    .expect("summary for the root");
                assert_eq!(latest.subscription, SubscriptionState::Muted);
                assert_eq!(i64::try_from(latest.revision).expect("revision"), row.2);
            }
        }
        assert!(
            interleavings
                .iter()
                .all(|revision| (1..=2).contains(revision))
        );
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::two_sockets(2)]
#[case::eight_sockets(8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn identical_concurrent_reactions_create_one_row_and_one_broadcast(#[case] sockets: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let mut poster = stage.poster().await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let mut watchers = [
            stage.watching(Person::Linus).await,
            stage.watching(Person::Ada).await,
        ];
        let before = room_activity(&stage.state, stage.campus.id, stage.room).await;
        let mut reacting = JoinSet::new();
        for _ in 0..sockets {
            let (addr, campus_id, room) = (stage.addr, stage.campus.id, stage.room);
            let token = stage.grace.token.clone();
            reacting.spawn(async move {
                let (mut socket, _) = common::joined_socket(addr, campus_id, &token).await;
                send(
                    &mut socket,
                    &react(room, root, EMOJI_A, ReactionAction::Add),
                )
                .await;
                let frame = common::reaction_outcome(&mut socket).await;
                common::frames_until_pong(&mut socket, 8).await;
                frame
            });
        }
        let mut replies = Vec::new();
        while let Some(done) = reacting.join_next().await {
            replies.push(done.expect("reaction task"));
        }
        let messages = replies
            .into_iter()
            .map(|frame| match frame {
                ServerFrame::MessageUpdated(updated) => updated.message,
                other => panic!("expected update, got {other:?}"),
            })
            .collect::<Vec<RoomMessage>>();
        assert!(messages.iter().all(|message| message == &messages[0]));
        assert_eq!(messages[0].reactions.len(), 1);
        assert_eq!(
            messages[0].reactions[0].account_ids,
            vec![stage.id_of(Person::Grace)]
        );
        assert_eq!(
            reaction_rows(&stage.state, stage.campus.id, root).await,
            vec![(EMOJI_A.to_owned(), stage.id_of(Person::Grace))]
        );
        assert_eq!(
            room_activity(&stage.state, stage.campus.id, stage.room).await,
            before + 1
        );
        let frames = settle_all(&mut watchers, &mut poster, stage.room).await;
        for seen in &frames {
            let broadcasts = seen
                .iter()
                .filter(|frame| matches!(frame, ServerFrame::MessageUpdated(_)))
                .count();
            assert_eq!(broadcasts, 1);
        }
    })
    .await
    .expect("scenario deadline");
}

fn storm_step(account: usize, step: usize) -> (ReactionAction, &'static str) {
    let mixed = account
        .wrapping_mul(2_654_435_761)
        .wrapping_add(step.wrapping_mul(40_503))
        .wrapping_add(step / 3);
    let action = if mixed.is_multiple_of(3) || step % 4 == 3 {
        ReactionAction::Remove
    } else {
        ReactionAction::Add
    };
    let emoji = if (mixed >> 3).is_multiple_of(2) {
        EMOJI_A
    } else {
        EMOJI_B
    };
    (action, emoji)
}

#[rstest]
#[case::five_accounts_thirty_steps(5, 30)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn add_remove_storm_converges_every_socket_on_the_database(
    #[case] accounts: usize,
    #[case] steps: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let mut poster = stage.poster().await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let mut watchers = [
            stage.watching(Person::Linus).await,
            stage.watching(Person::Guest).await,
            stage.watching(Person::Ada).await,
        ];
        let sessions = extra_members(&stage.state, stage.campus.id, accounts).await;
        let mut storm = JoinSet::new();
        for (account, session) in sessions.into_iter().enumerate() {
            let (addr, campus_id, room) = (stage.addr, stage.campus.id, stage.room);
            storm.spawn(async move {
                let (mut socket, _) = common::joined_socket(addr, campus_id, &session.token).await;
                for step in 0..steps {
                    let (action, emoji) = storm_step(account, step);
                    send(&mut socket, &react(room, root, emoji, action)).await;
                    common::reaction_outcome(&mut socket).await;
                }
                common::frames_until_pong(&mut socket, 8).await;
            });
        }
        while let Some(done) = storm.join_next().await {
            done.expect("storm task");
        }
        let mut reader = stage.plain(Person::Ada).await;
        let page = synced_page_at(&mut reader, stage.room, None, None).await;
        let stored = page
            .messages
            .iter()
            .find(|message| message.message_id == root)
            .expect("root")
            .clone();
        let rows = reaction_rows(&stage.state, stage.campus.id, root).await;
        let tallied = stored
            .reactions
            .iter()
            .flat_map(|tally| {
                tally
                    .account_ids
                    .iter()
                    .map(|account| (tally.emoji.clone(), *account))
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(rows.iter().cloned().collect::<BTreeSet<_>>(), tallied);
        assert_eq!(rows.len(), tallied.len());
        let activity = room_activity(&stage.state, stage.campus.id, stage.room).await;
        assert_eq!(
            stored.activity_seq,
            u64::try_from(activity).expect("sequence")
        );
        let frames = settle_all(&mut watchers, &mut poster, stage.room).await;
        for seen in &frames {
            let updates = seen
                .iter()
                .filter_map(|frame| match frame {
                    ServerFrame::MessageUpdated(updated) => Some(updated.message.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(i64::try_from(updates.len()).expect("count"), activity);
            let sequences = updates
                .iter()
                .map(|message| message.activity_seq)
                .collect::<BTreeSet<_>>();
            assert_eq!(sequences.len(), updates.len());
            let newest = updates
                .iter()
                .max_by_key(|message| message.activity_seq)
                .expect("update");
            assert_eq!(newest, &stored);
        }
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::viewed_follow(
    SubscriptionOrigin::Viewed,
    SubscriptionState::Following,
    "following",
    2
)]
#[case::viewed_follow_wide(
    SubscriptionOrigin::Viewed,
    SubscriptionState::Following,
    "following",
    8
)]
#[case::explicit_mute(SubscriptionOrigin::Explicit, SubscriptionState::Muted, "muted", 2)]
#[case::explicit_mute_wide(SubscriptionOrigin::Explicit, SubscriptionState::Muted, "muted", 8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_identical_subscriptions_apply_exactly_once(
    #[case] origin: SubscriptionOrigin,
    #[case] state: SubscriptionState,
    #[case] expected_state: &str,
    #[case] sockets: usize,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let mut poster = stage.poster().await;
        let root = stage.root(&mut ada, "root").await.message_id;
        let mut linus = Vec::new();
        for _ in 0..sockets {
            linus.push(stage.watching(Person::Linus).await);
        }
        let mut requests = JoinSet::new();
        for mut socket in linus {
            let room = stage.room;
            requests.spawn(async move {
                send(&mut socket, &set_subscription(room, root, state, origin)).await;
                socket
            });
        }
        let mut linus = Vec::new();
        while let Some(done) = requests.join_next().await {
            linus.push(done.expect("request task"));
        }
        let frames = settle_all(&mut linus, &mut poster, stage.room).await;
        let row = stage.row_of(root, Person::Linus).await.expect("row");
        assert_eq!((row.0.as_str(), row.2), (expected_state, 1));
        assert_eq!(
            common::count_rows(&stage.state, "thread_subscriptions", stage.campus.id).await,
            1
        );
        let deliveries = frames
            .iter()
            .flat_map(|seen| changed_summaries(seen))
            .collect::<Vec<_>>();
        assert_eq!(deliveries.len(), sockets + 1);
        assert!(deliveries.iter().all(|summary| summary.revision == 1));
        assert!(
            deliveries
                .iter()
                .all(|summary| summary.root.message_id == root)
        );
    })
    .await
    .expect("scenario deadline");
}
