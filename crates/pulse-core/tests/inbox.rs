mod fixtures;

use fixtures::{
    ACCOUNT_A, CAMPUS_B, SummarySpec, archive_selection, cancel_inbox_reply, created_frame,
    inbox_synced_frame, mark_read_frame, mentioning, message_id, move_selection, own_reply,
    peer_reply, permutations, received, settled_machine, start_inbox_reply, step_all, subscribe,
    subscription_frame, summary, thread_changed_frame, thread_rejected_frame, thread_roots,
    toggle_inbox,
};
use pulse_core::TimelineEvent;
use pulse_protocol::{
    ClientFrame, InboxStep, ServerFrame, SubscriptionOrigin, SubscriptionState, ThreadChanged,
    ThreadRejection,
};
use rstest::rstest;

const NEXT: InboxStep = InboxStep::Next;
const PREVIOUS: InboxStep = InboxStep::Previous;

fn baseline() -> TimelineEvent {
    received(inbox_synced_frame(&[SummarySpec::new(1)]))
}

fn reply_to_one(index: u128) -> TimelineEvent {
    received(created_frame(peer_reply(index, 1)))
}

fn mention_reply(index: u128) -> TimelineEvent {
    received(created_frame(mentioning(
        peer_reply(index, 1),
        &[ACCOUNT_A],
    )))
}

fn read_summary() -> TimelineEvent {
    received(thread_changed_frame(&SummarySpec {
        latest: Some(3),
        replies: 2,
        read_through: Some(3),
        revision: 2,
        ..SummarySpec::new(1)
    }))
}

fn changed(spec: SummarySpec) -> TimelineEvent {
    received(thread_changed_frame(&spec))
}

fn specs_frame(specs: &[SummarySpec]) -> TimelineEvent {
    received(inbox_synced_frame(specs))
}

fn three_threads() -> TimelineEvent {
    specs_frame(&[
        SummarySpec::new(1),
        SummarySpec::new(2),
        SummarySpec::new(3),
    ])
}

fn open_with_three(extra: Vec<TimelineEvent>) -> pulse_core::TimelineMachine {
    let mut events = vec![three_threads(), toggle_inbox()];
    events.extend(extra);
    settled_machine(events)
}

#[rstest]
#[case::replies_are_unread_once_each(
    vec![baseline(), reply_to_one(2), reply_to_one(3), reply_to_one(4), reply_to_one(3)],
    3,
    0,
    1,
)]
#[case::own_reply_marks_everything_before_it_read(
    vec![baseline(), reply_to_one(2), reply_to_one(3), received(created_frame(own_reply(4, 1))), reply_to_one(5)],
    1,
    0,
    1,
)]
#[case::server_read_through_supersedes_live_replies(
    vec![baseline(), reply_to_one(2), reply_to_one(3), read_summary()],
    0,
    0,
    0,
)]
#[case::mentions_count_once_each(
    vec![baseline(), mention_reply(2), reply_to_one(3), mention_reply(2)],
    2,
    1,
    1,
)]
fn unread_state_is_independent_of_delivery_order(
    #[case] events: Vec<TimelineEvent>,
    #[case] unread_replies: u32,
    #[case] unread_mentions: u32,
    #[case] unread_threads: u32,
) {
    let reference = settled_machine(events.clone()).inbox_snapshot();
    assert_eq!(reference.items.len(), 1);
    assert_eq!(reference.items[0].unread_replies, unread_replies);
    assert_eq!(reference.items[0].unread_mentions, unread_mentions);
    assert_eq!(reference.unread_threads, unread_threads);
    permutations(&events).into_iter().for_each(|order| {
        assert_eq!(settled_machine(order.clone()).inbox_snapshot(), reference);
    });
}

#[rstest]
#[case::following_read_stays_visible(SubscriptionState::Following, 0, 0, true, 0)]
#[case::following_unread_counts(SubscriptionState::Following, 2, 0, true, 1)]
#[case::following_mention_counts(SubscriptionState::Following, 0, 2, true, 1)]
#[case::muted_replies_hidden(SubscriptionState::Muted, 3, 0, false, 0)]
#[case::muted_mention_resurfaces(SubscriptionState::Muted, 3, 1, true, 1)]
#[case::archived_read_hidden(SubscriptionState::Archived, 0, 0, false, 0)]
#[case::archived_unread_resurfaces(SubscriptionState::Archived, 1, 0, true, 1)]
#[case::archived_mention_resurfaces(SubscriptionState::Archived, 0, 1, true, 1)]
fn visibility_and_unread_follow_subscription_state(
    #[case] state: SubscriptionState,
    #[case] unread_replies: u32,
    #[case] unread_mentions: u32,
    #[case] visible: bool,
    #[case] unread_threads: u32,
) {
    let snapshot = settled_machine(vec![specs_frame(&[SummarySpec {
        state,
        unread_replies,
        unread_mentions,
        ..SummarySpec::new(1)
    }])])
    .inbox_snapshot();
    assert_eq!(snapshot.items.len(), usize::from(visible));
    assert_eq!(snapshot.unread_threads, unread_threads);
    if visible {
        assert_eq!(snapshot.items[0].subscription, state);
        assert_eq!(snapshot.items[0].unread_replies, unread_replies);
        assert_eq!(snapshot.items[0].unread_mentions, unread_mentions);
    }
}

#[rstest]
#[case::latest_activity_first(
    vec![
        SummarySpec::replied(1, 9, 1),
        SummarySpec::new(2),
        SummarySpec::replied(3, 5, 1),
    ],
    vec![1, 3, 2],
)]
#[case::roots_without_replies_order_by_root(
    vec![SummarySpec::new(1), SummarySpec::new(3), SummarySpec::new(2)],
    vec![3, 2, 1],
)]
#[case::equal_activity_breaks_ties_by_root_descending(
    vec![SummarySpec::replied(1, 9, 1), SummarySpec::replied(2, 9, 1)],
    vec![2, 1],
)]
fn items_order_by_activity_then_root_descending(
    #[case] specs: Vec<SummarySpec>,
    #[case] expected: Vec<u128>,
) {
    let snapshot = settled_machine(vec![specs_frame(&specs)]).inbox_snapshot();
    assert_eq!(
        thread_roots(&snapshot),
        expected.into_iter().map(message_id).collect::<Vec<_>>()
    );
}

#[rstest]
#[case::unread_threads_counts_threads_not_replies(
    vec![
        SummarySpec::replied(1, 9, 5),
        SummarySpec::replied(2, 8, 1),
        SummarySpec::new(3),
    ],
    2,
)]
#[case::muted_mention_counts_but_muted_replies_do_not(
    vec![
        SummarySpec { state: SubscriptionState::Muted, unread_mentions: 1, unread_replies: 4, ..SummarySpec::new(1) },
        SummarySpec { state: SubscriptionState::Muted, unread_replies: 4, ..SummarySpec::new(2) },
    ],
    1,
)]
fn unread_threads_counts_visible_items_with_unread(
    #[case] specs: Vec<SummarySpec>,
    #[case] expected: u32,
) {
    let snapshot = settled_machine(vec![specs_frame(&specs)]).inbox_snapshot();
    assert_eq!(snapshot.unread_threads, expected);
}

#[rstest]
#[case::closed_inbox_has_no_selection(vec![], None, false, false)]
#[case::open_selects_first(vec![toggle_inbox()], Some(3), false, true)]
#[case::next_moves_down(vec![toggle_inbox(), move_selection(NEXT)], Some(2), false, true)]
#[case::next_clamps_at_last(
    vec![toggle_inbox(), move_selection(NEXT), move_selection(NEXT), move_selection(NEXT), move_selection(NEXT)],
    Some(1),
    false,
    true,
)]
#[case::previous_clamps_at_first(vec![toggle_inbox(), move_selection(PREVIOUS)], Some(3), false, true)]
#[case::previous_after_next(
    vec![toggle_inbox(), move_selection(NEXT), move_selection(NEXT), move_selection(PREVIOUS)],
    Some(2),
    false,
    true,
)]
#[case::move_while_closed_is_ignored(vec![move_selection(NEXT)], None, false, false)]
#[case::toggle_twice_closes_and_clears(vec![toggle_inbox(), move_selection(NEXT), toggle_inbox()], None, false, false)]
#[case::reply_starts_on_selection(vec![toggle_inbox(), start_inbox_reply()], Some(3), true, true)]
#[case::reply_needs_open_inbox(vec![start_inbox_reply()], None, false, false)]
#[case::moving_cancels_reply(
    vec![toggle_inbox(), start_inbox_reply(), move_selection(NEXT)],
    Some(2),
    false,
    true,
)]
#[case::cancel_ends_reply(
    vec![toggle_inbox(), start_inbox_reply(), cancel_inbox_reply()],
    Some(3),
    false,
    true,
)]
fn selection_and_reply_follow_inbox_intents(
    #[case] events: Vec<TimelineEvent>,
    #[case] selected: Option<u128>,
    #[case] replying: bool,
    #[case] open: bool,
) {
    let snapshot = {
        let mut all = vec![three_threads()];
        all.extend(events);
        settled_machine(all).inbox_snapshot()
    };
    assert_eq!(snapshot.selected_root_message_id, selected.map(message_id));
    assert_eq!(snapshot.replying, replying);
    assert_eq!(snapshot.open, open);
}

#[rstest]
#[case::middle_vanishes_selects_same_index(
    vec![NEXT],
    SummarySpec { state: SubscriptionState::Archived, revision: 2, ..SummarySpec::new(2) },
    Some(1),
)]
#[case::last_vanishes_clamps_to_new_last(
    vec![NEXT, NEXT],
    SummarySpec { state: SubscriptionState::Archived, revision: 2, ..SummarySpec::new(1) },
    Some(2),
)]
#[case::first_vanishes_selects_next_in_line(
    vec![],
    SummarySpec { state: SubscriptionState::Archived, revision: 2, ..SummarySpec::new(3) },
    Some(2),
)]
#[case::selection_follows_root_when_it_moves(
    vec![NEXT, NEXT],
    SummarySpec { latest: Some(9), replies: 1, unread_replies: 1, revision: 2, ..SummarySpec::new(1) },
    Some(1),
)]
fn selection_follows_the_root_and_survives_vanishing(
    #[case] steps: Vec<InboxStep>,
    #[case] change: SummarySpec,
    #[case] expected: Option<u128>,
) {
    let mut events = steps.into_iter().map(move_selection).collect::<Vec<_>>();
    events.push(changed(change));
    let snapshot = open_with_three(events).inbox_snapshot();
    assert_eq!(snapshot.selected_root_message_id, expected.map(message_id));
}

#[test]
fn selection_clears_when_the_last_item_vanishes() {
    let snapshot = settled_machine(vec![
        specs_frame(&[SummarySpec::new(1)]),
        toggle_inbox(),
        changed(SummarySpec {
            state: SubscriptionState::Archived,
            revision: 2,
            ..SummarySpec::new(1)
        }),
    ])
    .inbox_snapshot();
    assert_eq!(snapshot.items, vec![]);
    assert_eq!(snapshot.selected_root_message_id, None);
}

#[rstest]
#[case::archive_selected_without_replies(
    vec![],
    vec![
        subscription_frame(3, SubscriptionState::Archived, SubscriptionOrigin::Explicit),
        mark_read_frame(3, 3),
    ],
    vec![2, 1],
)]
#[case::archive_marks_read_through_latest_reply(
    vec![NEXT],
    vec![
        subscription_frame(2, SubscriptionState::Archived, SubscriptionOrigin::Explicit),
        mark_read_frame(2, 2),
    ],
    vec![3, 1],
)]
fn archive_emits_state_and_read_frames_and_removes_the_item(
    #[case] steps: Vec<InboxStep>,
    #[case] expected_frames: Vec<ClientFrame>,
    #[case] remaining: Vec<u128>,
) {
    let mut events = vec![toggle_inbox()];
    events.extend(steps.into_iter().map(move_selection));
    let machine = settled_machine(vec![three_threads()]);
    let (machine, _) = step_all(machine, events);
    let (machine, frames) = step_all(machine, vec![archive_selection()]);
    assert_eq!(frames, expected_frames);
    assert_eq!(
        thread_roots(&machine.inbox_snapshot()),
        remaining.into_iter().map(message_id).collect::<Vec<_>>()
    );
}

#[test]
fn archive_reads_through_the_latest_reply() {
    let machine = settled_machine(vec![
        specs_frame(&[SummarySpec::replied(1, 6, 2)]),
        toggle_inbox(),
    ]);
    let (_, frames) = step_all(machine, vec![archive_selection()]);
    assert_eq!(
        frames,
        vec![
            subscription_frame(1, SubscriptionState::Archived, SubscriptionOrigin::Explicit),
            mark_read_frame(1, 6),
        ]
    );
}

#[rstest]
#[case::inbox_closed(vec![three_threads()])]
#[case::nothing_selected(vec![toggle_inbox()])]
fn archive_without_selection_emits_nothing(#[case] events: Vec<TimelineEvent>) {
    let machine = settled_machine(events);
    let (_, frames) = step_all(machine, vec![archive_selection()]);
    assert_eq!(frames, vec![]);
}

#[test]
fn archive_while_disconnected_emits_nothing() {
    let machine = settled_machine(vec![three_threads(), toggle_inbox()]);
    let (machine, _) = step_all(machine, vec![TimelineEvent::Disconnected]);
    let (machine, frames) = step_all(machine, vec![archive_selection()]);
    assert_eq!(frames, vec![]);
    assert_eq!(machine.inbox_snapshot().items.len(), 3);
}

struct Standing {
    subscription: SubscriptionState,
    unread_replies: u32,
    unread_mentions: u32,
    reply_count: u32,
}

fn standing(
    subscription: SubscriptionState,
    unread_replies: u32,
    unread_mentions: u32,
    reply_count: u32,
) -> Standing {
    Standing {
        subscription,
        unread_replies,
        unread_mentions,
        reply_count,
    }
}

fn muted_with_mention(revision: u64) -> SummarySpec {
    SummarySpec {
        state: SubscriptionState::Muted,
        unread_mentions: 1,
        revision,
        ..SummarySpec::new(1)
    }
}

fn replied(revision: u64, state: SubscriptionState) -> SummarySpec {
    SummarySpec {
        state,
        revision,
        ..SummarySpec::replied(1, 4, 2)
    }
}

#[rstest]
#[case::higher_revision_replaces(
    vec![changed(SummarySpec::new(1)), changed(muted_with_mention(2))],
    standing(SubscriptionState::Muted, 0, 1, 0),
)]
#[case::lower_revision_with_newer_watermark_is_ignored(
    vec![changed(SummarySpec { revision: 3, ..SummarySpec::new(1) }), changed(replied(2, SubscriptionState::Following))],
    standing(SubscriptionState::Following, 0, 0, 0),
)]
#[case::lower_revision_with_other_state_is_ignored(
    vec![changed(muted_with_mention(3)), changed(SummarySpec { revision: 2, unread_mentions: 1, ..SummarySpec::new(1) })],
    standing(SubscriptionState::Muted, 0, 1, 0),
)]
#[case::equal_revision_with_newer_watermark_is_accepted(
    vec![changed(SummarySpec { revision: 2, ..SummarySpec::new(1) }), changed(replied(2, SubscriptionState::Following))],
    standing(SubscriptionState::Following, 2, 0, 2),
)]
#[case::equal_revision_with_older_watermark_is_ignored(
    vec![changed(replied(2, SubscriptionState::Following)), changed(SummarySpec { revision: 2, ..SummarySpec::new(1) })],
    standing(SubscriptionState::Following, 2, 0, 2),
)]
fn summaries_apply_by_revision_then_watermark(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Standing,
) {
    let snapshot = settled_machine(events).inbox_snapshot();
    assert_eq!(snapshot.items.len(), 1);
    let item = &snapshot.items[0];
    assert_eq!(item.subscription, expected.subscription);
    assert_eq!(item.unread_replies, expected.unread_replies);
    assert_eq!(item.unread_mentions, expected.unread_mentions);
    assert_eq!(item.reply_count, expected.reply_count);
}

#[rstest]
#[case::explicit_request_is_optimistic(vec![subscribe(1, SubscriptionState::Muted)], SubscriptionState::Muted)]
#[case::confirmation_clears_request(
    vec![
        subscribe(1, SubscriptionState::Muted),
        changed(SummarySpec { state: SubscriptionState::Muted, unread_mentions: 1, revision: 2, ..SummarySpec::new(1) }),
    ],
    SubscriptionState::Muted,
)]
#[case::rejection_clears_request(
    vec![
        subscribe(1, SubscriptionState::Muted),
        received(thread_rejected_frame(1, ThreadRejection::Forbidden)),
    ],
    SubscriptionState::Following,
)]
#[case::unknown_thread_rejection_clears_request(
    vec![
        subscribe(1, SubscriptionState::Muted),
        received(thread_rejected_frame(1, ThreadRejection::UnknownThread)),
    ],
    SubscriptionState::Following,
)]
#[case::disconnect_clears_request(
    vec![subscribe(1, SubscriptionState::Muted), TimelineEvent::Disconnected],
    SubscriptionState::Following,
)]
fn requested_subscription_is_optimistic_until_settled(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: SubscriptionState,
) {
    let mut all = vec![received(inbox_synced_frame(&[SummarySpec {
        unread_mentions: 1,
        ..SummarySpec::new(1)
    }]))];
    all.extend(events);
    let snapshot = settled_machine(all).inbox_snapshot();
    assert_eq!(snapshot.items.len(), 1);
    assert_eq!(snapshot.items[0].subscription, expected);
}

fn foreign(mut summary_value: pulse_protocol::ThreadSummary) -> pulse_protocol::ThreadSummary {
    summary_value.root.campus_id = CAMPUS_B;
    summary_value
}

#[rstest]
#[case::foreign_inbox_sync(received(ServerFrame::InboxSynced(pulse_protocol::InboxSynced {
    threads: vec![foreign(summary(&SummarySpec::new(1)))],
})))]
#[case::foreign_thread_changed(received(ServerFrame::ThreadChanged(ThreadChanged {
    summary: foreign(summary(&SummarySpec::new(1))),
})))]
fn foreign_campus_summaries_are_ignored(#[case] frame: TimelineEvent) {
    let snapshot = settled_machine(vec![frame]).inbox_snapshot();
    assert_eq!(snapshot.items, vec![]);
    assert_eq!(snapshot.unread_threads, 0);
}

#[test]
fn foreign_campus_reply_does_not_change_unread() {
    let foreign_reply = pulse_protocol::RoomMessage {
        campus_id: CAMPUS_B,
        ..peer_reply(2, 1)
    };
    let snapshot =
        settled_machine(vec![baseline(), received(created_frame(foreign_reply))]).inbox_snapshot();
    assert_eq!(snapshot.items[0].unread_replies, 0);
}
