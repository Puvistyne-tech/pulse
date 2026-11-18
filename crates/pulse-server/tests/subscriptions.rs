mod common;

use std::collections::BTreeSet;

use common::{
    Aim, Person, SCENARIO_DEADLINE, Stage, changed_summaries, inbox_of, mark_read, mention_rows,
    react, reaction_outcome, rejections_of, replied, send, set_subscription, settled_frames,
    summary_of,
};
use pulse_protocol::fixtures::EMOJI_A;
use pulse_protocol::{
    ClientNonce, MessageId, ReactionAction, ServerFrame, SubscriptionOrigin, SubscriptionState,
    ThreadRejection, mention_token,
};
use rstest::rstest;
use tokio::time::timeout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Read {
    Never,
    Root,
    Reply(usize),
}

#[derive(Clone, Copy)]
enum Step {
    Reply(Person, &'static [Person]),
    React(Person, Read),
}

fn resolve(read: Read, root: MessageId, replies: &[MessageId]) -> Option<MessageId> {
    match read {
        Read::Never => None,
        Read::Root => Some(root),
        Read::Reply(index) => Some(replies[index - 1]),
    }
}

fn token_list(stage: &Stage, people: &[Person]) -> String {
    people
        .iter()
        .map(|person| mention_token(stage.id_of(*person)))
        .collect::<Vec<_>>()
        .join(" ")
}

async fn play(
    stage: &Stage,
    root_mentions: &[Person],
    steps: &[Step],
) -> (MessageId, Vec<MessageId>) {
    let mut ada = stage.plain(Person::Ada).await;
    let body = format!("root {}", token_list(stage, root_mentions));
    let root = stage.root(&mut ada, &body).await.message_id;
    let mut replies = Vec::new();
    for step in steps {
        match step {
            Step::Reply(person, mentions) => {
                let mut socket = stage.plain(*person).await;
                let body = format!("reply {}", token_list(stage, mentions));
                let reply = replied(
                    &mut socket,
                    stage.room,
                    root,
                    ClientNonce::generate(),
                    &body,
                )
                .await;
                replies.push(reply.message_id);
            }
            Step::React(person, target) => {
                let mut socket = stage.plain(*person).await;
                let message = resolve(*target, root, &replies).expect("reaction target");
                send(
                    &mut socket,
                    &react(stage.room, message, EMOJI_A, ReactionAction::Add),
                )
                .await;
                reaction_outcome(&mut socket).await;
            }
        }
    }
    (root, replies)
}

#[rstest]
#[case::first_reply_follows_author_and_root_author(
    &[],
    &[Step::Reply(Person::Grace, &[])],
    vec![(Person::Ada, Read::Root, 1), (Person::Grace, Read::Reply(1), 1)],
)]
#[case::each_replier_follows_with_their_own_reply(
    &[],
    &[Step::Reply(Person::Grace, &[]), Step::Reply(Person::Linus, &[])],
    vec![
        (Person::Ada, Read::Root, 1),
        (Person::Grace, Read::Reply(1), 1),
        (Person::Linus, Read::Reply(2), 1),
    ],
)]
#[case::root_author_replying_follows_as_author(
    &[],
    &[Step::Reply(Person::Ada, &[])],
    vec![(Person::Ada, Read::Reply(1), 1)],
)]
#[case::own_reply_advances_read_through_and_revision(
    &[],
    &[Step::Reply(Person::Grace, &[]), Step::Reply(Person::Grace, &[])],
    vec![(Person::Ada, Read::Root, 1), (Person::Grace, Read::Reply(2), 2)],
)]
#[case::mention_in_root_follows_unread(&[Person::Grace], &[], vec![(Person::Grace, Read::Never, 1)])]
#[case::mention_in_reply_follows_unread(
    &[],
    &[Step::Reply(Person::Grace, &[Person::Linus])],
    vec![
        (Person::Ada, Read::Root, 1),
        (Person::Grace, Read::Reply(1), 1),
        (Person::Linus, Read::Never, 1),
    ],
)]
#[case::self_mention_is_skipped(
    &[],
    &[Step::Reply(Person::Grace, &[Person::Grace])],
    vec![(Person::Ada, Read::Root, 1), (Person::Grace, Read::Reply(1), 1)],
)]
#[case::mention_keeps_an_existing_follower_row(
    &[],
    &[Step::Reply(Person::Grace, &[]), Step::Reply(Person::Linus, &[Person::Grace])],
    vec![
        (Person::Ada, Read::Root, 1),
        (Person::Grace, Read::Reply(1), 1),
        (Person::Linus, Read::Reply(2), 1),
    ],
)]
#[case::reaction_on_root_follows_unread(
    &[],
    &[Step::React(Person::Grace, Read::Root)],
    vec![(Person::Grace, Read::Never, 1)],
)]
#[case::reaction_on_reply_follows_its_thread(
    &[],
    &[Step::Reply(Person::Linus, &[]), Step::React(Person::Grace, Read::Reply(1))],
    vec![
        (Person::Ada, Read::Root, 1),
        (Person::Linus, Read::Reply(1), 1),
        (Person::Grace, Read::Never, 1),
    ],
)]
#[case::reaction_keeps_an_existing_follower_row(
    &[],
    &[Step::Reply(Person::Grace, &[]), Step::React(Person::Grace, Read::Root)],
    vec![(Person::Ada, Read::Root, 1), (Person::Grace, Read::Reply(1), 1)],
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn actions_auto_follow_with_the_right_read_through(
    #[case] root_mentions: &'static [Person],
    #[case] steps: &'static [Step],
    #[case] expected: Vec<(Person, Read, i64)>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (root, replies) = play(&stage, root_mentions, steps).await;
        for (person, read, revision) in &expected {
            let row = stage.row_of(root, *person).await.expect("subscription row");
            assert_eq!(row.0, "following", "{person:?}");
            assert_eq!(row.1, resolve(*read, root, &replies), "{person:?}");
            assert_eq!(row.2, *revision, "{person:?}");
        }
        assert_eq!(
            common::count_rows(&stage.state, "thread_subscriptions", stage.campus.id).await,
            i64::try_from(expected.len()).expect("count")
        );
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Trigger {
    PeerReply,
    PeerReplyMentioningAda,
    AdaReacts,
    AdaReplies,
}

#[rstest]
#[case::muted_peer_reply(SubscriptionState::Muted, Trigger::PeerReply, "muted", Read::Root, 2)]
#[case::muted_mention(
    SubscriptionState::Muted,
    Trigger::PeerReplyMentioningAda,
    "muted",
    Read::Root,
    2
)]
#[case::muted_own_reaction(SubscriptionState::Muted, Trigger::AdaReacts, "muted", Read::Root, 2)]
#[case::muted_own_reply_only_moves_read(
    SubscriptionState::Muted,
    Trigger::AdaReplies,
    "muted",
    Read::Reply(2),
    3
)]
#[case::archived_peer_reply(
    SubscriptionState::Archived,
    Trigger::PeerReply,
    "archived",
    Read::Root,
    2
)]
#[case::archived_mention(
    SubscriptionState::Archived,
    Trigger::PeerReplyMentioningAda,
    "archived",
    Read::Root,
    2
)]
#[case::archived_own_reaction(
    SubscriptionState::Archived,
    Trigger::AdaReacts,
    "archived",
    Read::Root,
    2
)]
#[case::archived_own_reply_only_moves_read(
    SubscriptionState::Archived,
    Trigger::AdaReplies,
    "archived",
    Read::Reply(2),
    3
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn automatic_follows_never_override_an_explicit_state(
    #[case] state: SubscriptionState,
    #[case] trigger: Trigger,
    #[case] expected_state: &str,
    #[case] expected_read: Read,
    #[case] expected_revision: i64,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (root, mut replies) = play(&stage, &[], &[Step::Reply(Person::Grace, &[])]).await;
        let mut ada = stage.plain(Person::Ada).await;
        send(
            &mut ada,
            &set_subscription(stage.room, root, state, SubscriptionOrigin::Explicit),
        )
        .await;
        common::frames_until_pong(&mut ada, 3).await;
        let mut linus = stage.plain(Person::Linus).await;
        match trigger {
            Trigger::PeerReply => {
                replies.push(
                    replied(
                        &mut linus,
                        stage.room,
                        root,
                        ClientNonce::generate(),
                        "peer",
                    )
                    .await
                    .message_id,
                );
            }
            Trigger::PeerReplyMentioningAda => {
                let body = format!("peer {}", mention_token(stage.id_of(Person::Ada)));
                replies.push(
                    replied(&mut linus, stage.room, root, ClientNonce::generate(), &body)
                        .await
                        .message_id,
                );
            }
            Trigger::AdaReacts => {
                send(
                    &mut ada,
                    &react(stage.room, root, EMOJI_A, ReactionAction::Add),
                )
                .await;
                reaction_outcome(&mut ada).await;
            }
            Trigger::AdaReplies => {
                replies.push(
                    replied(&mut ada, stage.room, root, ClientNonce::generate(), "own")
                        .await
                        .message_id,
                );
            }
        }
        let row = stage.row_of(root, Person::Ada).await.expect("row");
        assert_eq!(row.0, expected_state);
        assert_eq!(row.1, resolve(expected_read, root, &replies));
        assert_eq!(row.2, expected_revision);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::first_follow(vec![SubscriptionState::Following], "following", 1)]
#[case::first_mute(vec![SubscriptionState::Muted], "muted", 1)]
#[case::repeated_state_is_a_no_op(vec![SubscriptionState::Muted, SubscriptionState::Muted], "muted", 1)]
#[case::change_bumps_revision(vec![SubscriptionState::Muted, SubscriptionState::Archived], "archived", 2)]
#[case::every_change_bumps(
    vec![
        SubscriptionState::Following,
        SubscriptionState::Muted,
        SubscriptionState::Following,
        SubscriptionState::Muted,
    ],
    "muted",
    4,
)]
#[case::no_ops_between_changes(
    vec![
        SubscriptionState::Muted,
        SubscriptionState::Muted,
        SubscriptionState::Archived,
        SubscriptionState::Archived,
        SubscriptionState::Following,
    ],
    "following",
    3,
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn explicit_changes_bump_the_revision_once_per_change(
    #[case] states: Vec<SubscriptionState>,
    #[case] expected_state: &str,
    #[case] expected_revision: i64,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.watching(Person::Ada).await;
        let mut poster = stage.poster().await;
        let root = stage.root(&mut ada, "root").await.message_id;
        for state in &states {
            send(
                &mut ada,
                &set_subscription(stage.room, root, *state, SubscriptionOrigin::Explicit),
            )
            .await;
        }
        let frames = settled_frames(&mut ada, &mut poster, stage.room).await;
        let row = stage.row_of(root, Person::Ada).await.expect("row");
        assert_eq!(row.0, expected_state);
        assert_eq!(row.1, Some(root));
        assert_eq!(row.2, expected_revision);
        let revisions = changed_summaries(&frames)
            .iter()
            .map(|summary| summary.revision)
            .collect::<BTreeSet<_>>();
        let expected =
            (1..=u64::try_from(expected_revision).expect("revision")).collect::<BTreeSet<_>>();
        assert_eq!(revisions, expected);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::viewed_inserts_following_at_the_latest_message(
    Person::Linus,
    None,
    SubscriptionOrigin::Viewed,
    SubscriptionState::Following,
    Aim::Top,
    None,
    Some(("following", Read::Reply(1), 1)),
)]
#[case::viewed_keeps_a_muted_row(
    Person::Linus,
    Some(SubscriptionState::Muted),
    SubscriptionOrigin::Viewed,
    SubscriptionState::Following,
    Aim::Top,
    None,
    Some(("muted", Read::Reply(1), 1)),
)]
#[case::viewed_keeps_an_archived_row(
    Person::Linus,
    Some(SubscriptionState::Archived),
    SubscriptionOrigin::Viewed,
    SubscriptionState::Following,
    Aim::Top,
    None,
    Some(("archived", Read::Reply(1), 1)),
)]
#[case::viewed_muted_is_forbidden(
    Person::Linus,
    None,
    SubscriptionOrigin::Viewed,
    SubscriptionState::Muted,
    Aim::Top,
    Some(ThreadRejection::Forbidden),
    None
)]
#[case::viewed_archived_is_forbidden(
    Person::Linus,
    None,
    SubscriptionOrigin::Viewed,
    SubscriptionState::Archived,
    Aim::Top,
    Some(ThreadRejection::Forbidden),
    None
)]
#[case::viewed_on_a_reply_is_unknown(
    Person::Linus,
    None,
    SubscriptionOrigin::Viewed,
    SubscriptionState::Following,
    Aim::Reply,
    Some(ThreadRejection::UnknownThread),
    None
)]
#[case::explicit_on_unknown_message(
    Person::Linus,
    None,
    SubscriptionOrigin::Explicit,
    SubscriptionState::Muted,
    Aim::Unknown,
    Some(ThreadRejection::UnknownThread),
    None
)]
#[case::explicit_on_root_of_another_room(
    Person::Linus,
    None,
    SubscriptionOrigin::Explicit,
    SubscriptionState::Muted,
    Aim::Sibling,
    Some(ThreadRejection::UnknownThread),
    None
)]
#[case::guest_may_subscribe(
    Person::Guest,
    None,
    SubscriptionOrigin::Explicit,
    SubscriptionState::Muted,
    Aim::Top,
    None,
    Some(("muted", Read::Reply(1), 1)),
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn subscription_requests_follow_origin_and_target_rules(
    #[case] person: Person,
    #[case] setup: Option<SubscriptionState>,
    #[case] origin: SubscriptionOrigin,
    #[case] state: SubscriptionState,
    #[case] target: Aim,
    #[case] rejection: Option<ThreadRejection>,
    #[case] row: Option<(&str, Read, i64)>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (root, replies) = play(&stage, &[], &[Step::Reply(Person::Grace, &[])]).await;
        let mut ada = stage.plain(Person::Ada).await;
        let sibling = common::posted(
            &mut ada,
            stage.campus.rooms[1],
            ClientNonce::generate(),
            "elsewhere",
        )
        .await;
        let mut socket = stage.watching(person).await;
        let mut poster = stage.poster().await;
        if let Some(initial) = setup {
            send(
                &mut socket,
                &set_subscription(stage.room, root, initial, SubscriptionOrigin::Explicit),
            )
            .await;
        }
        let message = common::aimed(target, root, replies[0], sibling.message_id);
        send(
            &mut socket,
            &set_subscription(stage.room, message, state, origin),
        )
        .await;
        let frames = settled_frames(&mut socket, &mut poster, stage.room).await;
        assert_eq!(
            rejections_of(&frames),
            rejection.into_iter().collect::<Vec<_>>()
        );
        let stored = stage.row_of(message, person).await;
        let expected = row.map(|(state, read, revision)| {
            (state.to_owned(), resolve(read, root, &replies), revision)
        });
        assert_eq!(stored, expected);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::forward(vec![Read::Reply(2)], Read::Reply(2), 2, vec![])]
#[case::never_moves_backwards(vec![Read::Reply(2), Read::Reply(1)], Read::Reply(2), 2, vec![])]
#[case::same_point_twice_is_a_no_op(vec![Read::Reply(2), Read::Reply(2)], Read::Reply(2), 2, vec![])]
#[case::advances_twice(vec![Read::Reply(1), Read::Reply(3)], Read::Reply(3), 3, vec![])]
#[case::older_than_the_baseline_is_ignored(vec![Read::Root], Read::Root, 1, vec![])]
#[case::message_of_another_thread_is_rejected(
    vec![Read::Never],
    Read::Root,
    1,
    vec![ThreadRejection::UnknownThread],
)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn read_cursor_is_monotonic_and_scoped_to_the_thread(
    #[case] reads: Vec<Read>,
    #[case] expected_read: Read,
    #[case] expected_revision: i64,
    #[case] rejections: Vec<ThreadRejection>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let steps = [
            Step::Reply(Person::Grace, &[]),
            Step::Reply(Person::Grace, &[]),
            Step::Reply(Person::Grace, &[]),
        ];
        let (root, replies) = play(&stage, &[], &steps).await;
        let mut ada = stage.watching(Person::Ada).await;
        let mut poster = stage.poster().await;
        let other_root = stage.root(&mut poster, "other root").await.message_id;
        let foreign_reply = replied(
            &mut poster,
            stage.room,
            other_root,
            ClientNonce::generate(),
            "elsewhere",
        )
        .await;
        for read in &reads {
            let through = resolve(*read, root, &replies).unwrap_or(foreign_reply.message_id);
            send(&mut ada, &mark_read(stage.room, root, through)).await;
        }
        let frames = settled_frames(&mut ada, &mut poster, stage.room).await;
        assert_eq!(rejections_of(&frames), rejections);
        let row = stage.row_of(root, Person::Ada).await.expect("row");
        assert_eq!(row.1, resolve(expected_read, root, &replies));
        assert_eq!(row.2, expected_revision);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::unknown_message(Aim::Unknown, vec![ThreadRejection::UnknownThread])]
#[case::reply_as_root(Aim::Reply, vec![ThreadRejection::UnknownThread])]
#[case::root_of_another_room(Aim::Sibling, vec![ThreadRejection::UnknownThread])]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn marking_read_on_a_non_thread_is_rejected(
    #[case] target: Aim,
    #[case] rejections: Vec<ThreadRejection>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (root, replies) = play(&stage, &[], &[Step::Reply(Person::Grace, &[])]).await;
        let mut ada = stage.watching(Person::Ada).await;
        let mut poster = stage.poster().await;
        let sibling = stage
            .root_in(&mut poster, stage.campus.rooms[1], "elsewhere")
            .await;
        let message = common::aimed(target, root, replies[0], sibling.message_id);
        send(&mut ada, &mark_read(stage.room, message, root)).await;
        let frames = settled_frames(&mut ada, &mut poster, stage.room).await;
        assert_eq!(rejections_of(&frames), rejections);
        assert_eq!(stage.row_of(root, Person::Ada).await.expect("row").2, 1);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn marking_read_without_a_subscription_creates_nothing() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (root, replies) = play(&stage, &[], &[Step::Reply(Person::Grace, &[])]).await;
        let mut linus = stage.watching(Person::Linus).await;
        let mut poster = stage.poster().await;
        send(&mut linus, &mark_read(stage.room, root, replies[0])).await;
        let frames = settled_frames(&mut linus, &mut poster, stage.room).await;
        assert!(changed_summaries(&frames).is_empty());
        assert!(rejections_of(&frames).is_empty());
        assert_eq!(stage.row_of(root, Person::Linus).await, None);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn inbox_sync_reports_exact_unread_counts_and_order() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let ada_token = mention_token(stage.id_of(Person::Ada));
        let mut ada = stage.watching(Person::Ada).await;
        let mut grace = stage.plain(Person::Grace).await;
        let mut linus = stage.plain(Person::Linus).await;
        let first = stage.root(&mut ada, "first").await.message_id;
        let second = stage.root(&mut ada, "second").await.message_id;
        replied(&mut grace, stage.room, first, ClientNonce::generate(), "a1").await;
        let a2 = replied(&mut grace, stage.room, first, ClientNonce::generate(), "a2")
            .await
            .message_id;
        let b1 = replied(
            &mut linus,
            stage.room,
            first,
            ClientNonce::generate(),
            &format!("b1 {ada_token}"),
        )
        .await
        .message_id;
        let c1 = replied(
            &mut grace,
            stage.room,
            second,
            ClientNonce::generate(),
            &format!("c1 {ada_token}"),
        )
        .await
        .message_id;
        let threads = inbox_of(&mut ada).await;
        assert_eq!(
            threads
                .iter()
                .map(|summary| summary.root.message_id)
                .collect::<Vec<_>>(),
            vec![second, first]
        );
        let (one, two) = (summary_of(&threads, first), summary_of(&threads, second));
        assert_eq!(
            (
                one.unread_replies,
                one.unread_mentions,
                one.read_through,
                one.subscription,
                one.revision
            ),
            (3, 1, Some(first), SubscriptionState::Following, 1)
        );
        assert_eq!(
            one.latest_reply.as_ref().map(|reply| reply.message_id),
            Some(b1)
        );
        assert_eq!(one.root.reply_count, 3);
        assert_eq!(
            (
                two.unread_replies,
                two.unread_mentions,
                two.read_through,
                two.revision
            ),
            (1, 1, Some(second), 1)
        );
        assert_eq!(
            two.latest_reply.as_ref().map(|reply| reply.message_id),
            Some(c1)
        );
        send(&mut ada, &mark_read(stage.room, first, a2)).await;
        let after_partial = inbox_of(&mut ada).await;
        let partial = summary_of(&after_partial, first);
        assert_eq!(
            (
                partial.unread_replies,
                partial.unread_mentions,
                partial.revision
            ),
            (1, 1, 2)
        );
        send(&mut ada, &mark_read(stage.room, first, b1)).await;
        let after_full = inbox_of(&mut ada).await;
        let full = summary_of(&after_full, first);
        assert_eq!(
            (full.unread_replies, full.unread_mentions, full.revision),
            (0, 0, 3)
        );
        let own = replied(
            &mut ada,
            stage.room,
            second,
            ClientNonce::generate(),
            "mine",
        )
        .await
        .message_id;
        let after_own = inbox_of(&mut ada).await;
        let mine = summary_of(&after_own, second);
        assert_eq!(
            (mine.unread_replies, mine.unread_mentions, mine.read_through),
            (0, 0, Some(own))
        );
        assert_eq!(
            after_own
                .iter()
                .map(|summary| summary.root.message_id)
                .collect::<Vec<_>>(),
            vec![second, first]
        );
        let graces = inbox_of(&mut grace).await;
        let own_view = summary_of(&graces, first);
        assert_eq!(
            (
                own_view.unread_replies,
                own_view.unread_mentions,
                own_view.read_through
            ),
            (1, 0, Some(a2))
        );
        assert_eq!(
            mention_rows(&stage.state, stage.campus.id, b1).await,
            vec![stage.id_of(Person::Ada)]
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn inbox_counts_root_mentions_for_followers_without_replies() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (root, _) = play(&stage, &[Person::Grace], &[]).await;
        let mut grace = stage.plain(Person::Grace).await;
        let threads = inbox_of(&mut grace).await;
        assert_eq!(threads.len(), 1);
        let summary = summary_of(&threads, root);
        assert_eq!(
            (
                summary.unread_replies,
                summary.unread_mentions,
                summary.read_through,
                summary.latest_reply.is_none()
            ),
            (0, 1, None, true)
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn guest_inbox_is_readable_and_empty_without_subscriptions() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        play(&stage, &[], &[Step::Reply(Person::Grace, &[])]).await;
        let mut guest = stage.plain(Person::Guest).await;
        assert!(inbox_of(&mut guest).await.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn thread_changes_reach_only_the_sockets_of_the_affected_account() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada_phone = stage.watching(Person::Ada).await;
        let mut ada_laptop = stage.watching(Person::Ada).await;
        let mut grace = stage.watching(Person::Grace).await;
        let mut linus = stage.watching(Person::Linus).await;
        let mut poster = stage.poster().await;
        let root = stage.root(&mut ada_phone, "root").await.message_id;
        let reply = replied(
            &mut grace,
            stage.room,
            root,
            ClientNonce::generate(),
            "hello",
        )
        .await
        .message_id;
        let mut grace_frames = common::frames_until_pong(&mut grace, 5).await;
        let nonce = ClientNonce::generate();
        common::posted(&mut poster, stage.room, nonce, "sentinel").await;
        let mut seen = Vec::new();
        for socket in [&mut ada_phone, &mut ada_laptop, &mut grace, &mut linus] {
            let frames = common::frames_through_created(socket, nonce).await;
            seen.push(frames);
        }
        grace_frames.extend(seen[2].clone());
        seen[2] = grace_frames;
        let seen = seen
            .iter()
            .map(|frames| changed_summaries(frames))
            .collect::<Vec<_>>();
        let [phone, laptop, graces, linuses] = <[Vec<_>; 4]>::try_from(seen).expect("four sockets");
        for summaries in [&phone, &laptop] {
            assert!(!summaries.is_empty());
            assert!(summaries.iter().all(|summary| {
                summary.read_through == Some(root)
                    && summary.unread_replies == 1
                    && summary.root.message_id == root
            }));
        }
        assert!(!graces.is_empty());
        assert!(
            graces
                .iter()
                .all(|summary| summary.read_through == Some(reply) && summary.unread_replies == 0)
        );
        assert!(linuses.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn explicit_subscription_replies_with_the_new_summary() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let mut ada = stage.plain(Person::Ada).await;
        let root = stage.root(&mut ada, "root").await.message_id;
        send(
            &mut ada,
            &set_subscription(
                stage.room,
                root,
                SubscriptionState::Muted,
                SubscriptionOrigin::Explicit,
            ),
        )
        .await;
        let changed = common::next_of(&mut ada, |frame| match frame {
            ServerFrame::ThreadChanged(changed) => Some(changed.summary.clone()),
            _ => None,
        })
        .await;
        assert_eq!(changed.subscription, SubscriptionState::Muted);
        assert_eq!(changed.revision, 1);
        assert_eq!(changed.root.message_id, root);
        assert_eq!(changed.read_through, Some(root));
    })
    .await
    .expect("scenario deadline");
}
