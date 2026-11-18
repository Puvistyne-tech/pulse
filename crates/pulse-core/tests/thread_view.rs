mod fixtures;

use fixtures::{
    NONCE_A, ROOM_A, SummarySpec, accepted_frame, chain_message, clock, close_thread,
    created_frame, inbox_synced_frame, mark_read_frame, message_id, open_thread, own_reply,
    peer_reply, received, rejected_frame, reply_frame, reply_intent, settled_machine, step_all,
    subscribe, subscription_frame, sync_frame_at, sync_inbox_frame, sync_shelf_frame,
    sync_thread_frame, thread_changed_frame, thread_rejected_frame, thread_synced_frame,
    timeline_connected, title_intent, topic_title_frame, with_replies,
};
use pulse_core::{THREAD_BURIED_DEPTH, THREAD_VIEW_FOLLOW_MS, TimelineEvent};
use pulse_protocol::{
    ApiError, ApiErrorCode, ClientFrame, DeliveryState, InputField, MessageRejection,
    SubscriptionOrigin, SubscriptionState, TOPIC_TITLE_MAX_CHARS, ThreadRejection,
};
use rstest::rstest;

fn following(root: u128) -> TimelineEvent {
    received(thread_changed_frame(&SummarySpec::new(root)))
}

fn baseline_with_replies() -> TimelineEvent {
    received(inbox_synced_frame(&[SummarySpec::replied(1, 5, 2)]))
}

fn follow(root: u128) -> ClientFrame {
    subscription_frame(
        root,
        SubscriptionState::Following,
        SubscriptionOrigin::Viewed,
    )
}

fn reconnect() -> TimelineEvent {
    timeline_connected(&[ROOM_A])
}

fn resync() -> ClientFrame {
    sync_frame_at(&[(ROOM_A, Some(message_id(1)), Some(0))])
}

fn run(events: Vec<TimelineEvent>) -> (pulse_core::TimelineMachine, Vec<ClientFrame>) {
    step_all(
        settled_machine(vec![received(created_frame(chain_message(ROOM_A, 1)))]),
        events,
    )
}

#[test]
fn follow_delay_is_three_seconds() {
    assert_eq!(THREAD_VIEW_FOLLOW_MS, 3000);
}

#[rstest]
#[case::one_millisecond_early_sends_no_follow(
    vec![open_thread(1, 1000), clock(1000 + THREAD_VIEW_FOLLOW_MS - 1)],
    vec![sync_thread_frame(1, None)],
    Some(1000 + THREAD_VIEW_FOLLOW_MS),
    None,
)]
#[case::exact_deadline_sends_one_follow(
    vec![open_thread(1, 1000), clock(1000 + THREAD_VIEW_FOLLOW_MS)],
    vec![sync_thread_frame(1, None), follow(1)],
    None,
    Some(SubscriptionState::Following),
)]
#[case::repeated_ticks_send_one_follow(
    vec![
        open_thread(1, 1000),
        clock(4000),
        clock(4001),
        clock(9000),
        clock(90_000),
    ],
    vec![sync_thread_frame(1, None), follow(1)],
    None,
    Some(SubscriptionState::Following),
)]
#[case::already_subscribed_sends_no_follow(
    vec![following(1), open_thread(1, 1000), clock(4000)],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 1)],
    None,
    Some(SubscriptionState::Following),
)]
#[case::close_before_deadline_sends_nothing(
    vec![open_thread(1, 1000), close_thread(), clock(9000)],
    vec![sync_thread_frame(1, None)],
    None,
    None,
)]
#[case::reopen_restarts_the_delay(
    vec![open_thread(1, 1000), close_thread(), open_thread(1, 2000), clock(4999)],
    vec![sync_thread_frame(1, None), sync_thread_frame(1, None)],
    Some(5000),
    None,
)]
#[case::reopen_follows_after_its_own_delay(
    vec![open_thread(1, 1000), close_thread(), open_thread(1, 2000), clock(5000)],
    vec![sync_thread_frame(1, None), sync_thread_frame(1, None), follow(1)],
    None,
    Some(SubscriptionState::Following),
)]
#[case::clock_going_backwards_is_ignored(
    vec![open_thread(1, 5000), clock(7000), clock(100), clock(7999)],
    vec![sync_thread_frame(1, None)],
    Some(8000),
    None,
)]
#[case::backwards_tick_does_not_undo_a_due_follow(
    vec![open_thread(1, 5000), clock(8000), clock(1), clock(8000)],
    vec![sync_thread_frame(1, None), follow(1)],
    None,
    Some(SubscriptionState::Following),
)]
#[case::switching_threads_restarts_the_delay(
    vec![open_thread(1, 1000), open_thread(2, 3000), clock(4000)],
    vec![sync_thread_frame(1, None), sync_thread_frame(2, None)],
    Some(6000),
    None,
)]
#[case::clock_without_open_thread_is_inert(vec![clock(50_000)], vec![], None, None)]
fn viewing_follows_after_the_injected_delay(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected_frames: Vec<ClientFrame>,
    #[case] deadline: Option<u64>,
    #[case] subscription: Option<SubscriptionState>,
) {
    let (machine, frames) = run(events);
    assert_eq!(frames, expected_frames);
    assert_eq!(machine.next_deadline_ms(), deadline);
    assert_eq!(machine.thread_snapshot().subscription, subscription);
}

#[rstest]
#[case::follow_due_offline_is_sent_on_connect(
    vec![open_thread(1, 1000), TimelineEvent::Disconnected, clock(4000), reconnect()],
    vec![
        sync_thread_frame(1, None),
        resync(),
        sync_inbox_frame(), sync_shelf_frame(),
        follow(1),
        sync_thread_frame(1, None),
    ],
)]
#[case::unconfirmed_follow_is_resent_after_drop(
    vec![open_thread(1, 1000), clock(4000), TimelineEvent::Disconnected, reconnect()],
    vec![
        sync_thread_frame(1, None),
        follow(1),
        resync(),
        sync_inbox_frame(), sync_shelf_frame(),
        follow(1),
        sync_thread_frame(1, None),
    ],
)]
#[case::confirmed_follow_is_not_resent(
    vec![
        open_thread(1, 1000),
        clock(4000),
        following(1),
        TimelineEvent::Disconnected,
        reconnect(),
    ],
    vec![
        sync_thread_frame(1, None),
        follow(1),
        resync(),
        sync_inbox_frame(), sync_shelf_frame(),
        sync_thread_frame(1, None),
    ],
)]
#[case::pending_follow_is_sent_once(
    vec![
        open_thread(1, 1000),
        TimelineEvent::Disconnected,
        clock(4000),
        reconnect(),
        TimelineEvent::Disconnected,
        reconnect(),
    ],
    vec![
        sync_thread_frame(1, None),
        resync(),
        sync_inbox_frame(), sync_shelf_frame(),
        follow(1),
        sync_thread_frame(1, None),
        resync(),
        sync_inbox_frame(), sync_shelf_frame(),
        follow(1),
        sync_thread_frame(1, None),
    ],
)]
#[case::settled_thread_page_is_not_requested_again(
    vec![
        open_thread(1, 1000),
        received(thread_synced_frame(1, vec![], false)),
        TimelineEvent::Disconnected,
        reconnect(),
    ],
    vec![sync_thread_frame(1, None), resync(), sync_inbox_frame(), sync_shelf_frame()],
)]
#[case::closed_thread_is_not_requested_after_reconnect(
    vec![open_thread(1, 1000), close_thread(), TimelineEvent::Disconnected, reconnect()],
    vec![sync_thread_frame(1, None), resync(), sync_inbox_frame(), sync_shelf_frame()],
)]
#[case::open_while_offline_emits_nothing_until_connect(
    vec![TimelineEvent::Disconnected, open_thread(1, 1000), reconnect()],
    vec![resync(), sync_inbox_frame(), sync_shelf_frame(), sync_thread_frame(1, None)],
)]
fn reconnect_resumes_the_open_thread_and_pending_follow(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Vec<ClientFrame>,
) {
    let (_, frames) = run(events);
    assert_eq!(frames, expected);
}

#[rstest]
#[case::has_more_pages_from_the_last_reply(
    vec![
        open_thread(1, 0),
        received(thread_synced_frame(1, vec![peer_reply(2, 1), peer_reply(3, 1)], true)),
    ],
    vec![sync_thread_frame(1, None), sync_thread_frame(1, Some(3))],
    true,
)]
#[case::final_page_stops_paging(
    vec![
        open_thread(1, 0),
        received(thread_synced_frame(1, vec![peer_reply(2, 1), peer_reply(3, 1)], true)),
        received(thread_synced_frame(1, vec![peer_reply(4, 1)], false)),
    ],
    vec![sync_thread_frame(1, None), sync_thread_frame(1, Some(3))],
    false,
)]
#[case::empty_page_with_has_more_does_not_loop(
    vec![open_thread(1, 0), received(thread_synced_frame(1, vec![], true))],
    vec![sync_thread_frame(1, None)],
    false,
)]
#[case::page_for_another_root_is_not_followed(
    vec![
        received(created_frame(chain_message(ROOM_A, 2))),
        open_thread(1, 0),
        received(thread_synced_frame(2, vec![peer_reply(3, 2)], true)),
    ],
    vec![sync_thread_frame(1, None)],
    true,
)]
#[case::page_after_close_is_not_followed(
    vec![
        open_thread(1, 0),
        close_thread(),
        received(thread_synced_frame(1, vec![peer_reply(2, 1)], true)),
    ],
    vec![sync_thread_frame(1, None)],
    false,
)]
fn thread_sync_pages_until_the_final_page(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected_frames: Vec<ClientFrame>,
    #[case] syncing: bool,
) {
    let (machine, frames) = run(events);
    assert_eq!(frames, expected_frames);
    assert_eq!(machine.thread_snapshot().syncing, syncing);
}

#[rstest]
#[case::open_marks_the_root_when_subscribed(
    vec![following(1), open_thread(1, 0)],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 1)],
)]
#[case::open_marks_through_the_known_latest_reply(
    vec![baseline_with_replies(), open_thread(1, 0)],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 5)],
)]
#[case::open_without_subscription_marks_nothing(
    vec![open_thread(1, 0)],
    vec![sync_thread_frame(1, None)],
)]
#[case::live_peer_reply_is_marked_read(
    vec![
        following(1),
        open_thread(1, 0),
        received(created_frame(peer_reply(2, 1))),
    ],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 1), mark_read_frame(1, 2)],
)]
#[case::each_new_reply_is_marked(
    vec![
        following(1),
        open_thread(1, 0),
        received(created_frame(peer_reply(2, 1))),
        received(created_frame(peer_reply(3, 1))),
    ],
    vec![
        sync_thread_frame(1, None),
        mark_read_frame(1, 1),
        mark_read_frame(1, 2),
        mark_read_frame(1, 3),
    ],
)]
#[case::duplicate_reply_is_marked_once(
    vec![
        following(1),
        open_thread(1, 0),
        received(created_frame(peer_reply(2, 1))),
        received(created_frame(peer_reply(2, 1))),
    ],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 1), mark_read_frame(1, 2)],
)]
#[case::own_reply_is_not_marked(
    vec![
        following(1),
        open_thread(1, 0),
        received(created_frame(own_reply(2, 1))),
    ],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 1)],
)]
#[case::reply_to_another_thread_is_not_marked(
    vec![
        following(1),
        following(2),
        received(created_frame(chain_message(ROOM_A, 2))),
        open_thread(1, 0),
        received(created_frame(peer_reply(3, 2))),
    ],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 1)],
)]
#[case::reply_after_close_is_not_marked(
    vec![
        following(1),
        open_thread(1, 0),
        close_thread(),
        received(created_frame(peer_reply(2, 1))),
    ],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 1)],
)]
#[case::reply_without_subscription_is_not_marked(
    vec![open_thread(1, 0), received(created_frame(peer_reply(2, 1)))],
    vec![sync_thread_frame(1, None)],
)]
#[case::final_page_marks_when_subscribed_later(
    vec![
        open_thread(1, 0),
        following(1),
        received(thread_synced_frame(1, vec![peer_reply(2, 1)], false)),
    ],
    vec![sync_thread_frame(1, None), mark_read_frame(1, 2)],
)]
#[case::intermediate_page_does_not_mark(
    vec![
        open_thread(1, 0),
        following(1),
        received(thread_synced_frame(1, vec![peer_reply(2, 1)], true)),
    ],
    vec![sync_thread_frame(1, None), sync_thread_frame(1, Some(2))],
)]
fn viewed_threads_are_marked_read(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Vec<ClientFrame>,
) {
    let (_, frames) = run(events);
    assert_eq!(frames, expected);
}

#[rstest]
#[case::viewing_a_reply_clears_its_unread(
    vec![baseline_with_replies(), open_thread(1, 0)],
    0,
)]
#[case::unviewed_thread_keeps_unread(vec![baseline_with_replies()], 2)]
#[case::reply_while_viewing_stays_read(
    vec![baseline_with_replies(), open_thread(1, 0), received(created_frame(peer_reply(6, 1)))],
    0,
)]
#[case::reply_after_close_is_unread(
    vec![
        baseline_with_replies(),
        open_thread(1, 0),
        close_thread(),
        received(created_frame(peer_reply(6, 1))),
    ],
    1,
)]
fn viewing_updates_unread_counts(#[case] events: Vec<TimelineEvent>, #[case] unread: u32) {
    let (machine, _) = run(events);
    assert_eq!(machine.inbox_snapshot().items[0].unread_replies, unread);
}

fn buried_events(top_level_between: u128, other_thread_replies: u128) -> Vec<TimelineEvent> {
    let first_top = 2;
    let last_top = first_top + top_level_between;
    let reply = last_top + other_thread_replies;
    let mut events = vec![received(created_frame(with_replies(
        chain_message(ROOM_A, 1),
        1,
        Some(reply),
    )))];
    events.extend(
        (first_top..last_top).map(|index| received(created_frame(chain_message(ROOM_A, index)))),
    );
    events.extend(
        (last_top..reply).map(|index| received(created_frame(peer_reply(index, first_top)))),
    );
    events.push(received(created_frame(peer_reply(reply, 1))));
    events
}

#[rstest]
#[case::one_below_the_depth_stays_hidden(THREAD_BURIED_DEPTH as u128 - 1, 0, false)]
#[case::at_the_depth_is_shown(THREAD_BURIED_DEPTH as u128, 0, true)]
#[case::beyond_the_depth_is_shown(THREAD_BURIED_DEPTH as u128 + 3, 0, true)]
#[case::no_messages_between_stays_hidden(0, 0, false)]
#[case::replies_to_other_threads_do_not_bury(THREAD_BURIED_DEPTH as u128 - 1, 4, false)]
fn latest_reply_shows_only_when_buried(
    #[case] top_level_between: u128,
    #[case] other_thread_replies: u128,
    #[case] shown: bool,
) {
    let (machine, _) = run(buried_events(top_level_between, other_thread_replies));
    let snapshot = machine.snapshot(Some(ROOM_A));
    let preview = snapshot.entries[0].thread.as_ref().expect("preview");
    let latest_id = 2 + top_level_between + other_thread_replies;
    assert_eq!(preview.show_latest_reply, shown);
    assert_eq!(
        preview.latest_reply.as_ref().map(|reply| reply.message_id),
        Some(message_id(latest_id))
    );
    assert_eq!(preview.reply_count, 1);
}

fn confirmed_reply_with_nonce() -> pulse_protocol::RoomMessage {
    pulse_protocol::RoomMessage {
        client_nonce: NONCE_A,
        ..own_reply(2, 1)
    }
}

fn thread_state(
    machine: &pulse_core::TimelineMachine,
) -> (Vec<(Option<u128>, DeliveryState)>, bool) {
    let snapshot = machine.thread_snapshot();
    let replies = snapshot
        .replies
        .iter()
        .map(|entry| {
            (
                entry.message_id.map(|id| {
                    (1..=9)
                        .find(|index| message_id(*index) == id)
                        .expect("known id")
                }),
                entry.delivery,
            )
        })
        .collect();
    (replies, snapshot.syncing)
}

#[rstest]
#[case::reply_is_pending_until_accepted(
    vec![open_thread(1, 0), reply_intent(1, NONCE_A, "  hello  ")],
    vec![(None, DeliveryState::Pending)],
)]
#[case::accepted_echo_confirms_the_pending_reply(
    vec![
        open_thread(1, 0),
        reply_intent(1, NONCE_A, "hello"),
        received(accepted_frame(confirmed_reply_with_nonce())),
    ],
    vec![(Some(2), DeliveryState::Confirmed)],
)]
#[case::created_echo_after_accepted_is_one_reply(
    vec![
        open_thread(1, 0),
        reply_intent(1, NONCE_A, "hello"),
        received(accepted_frame(confirmed_reply_with_nonce())),
        received(created_frame(confirmed_reply_with_nonce())),
    ],
    vec![(Some(2), DeliveryState::Confirmed)],
)]
#[case::rejected_reply_fails_in_place(
    vec![
        open_thread(1, 0),
        reply_intent(1, NONCE_A, "hello"),
        received(rejected_frame(NONCE_A, MessageRejection::UnknownThread)),
    ],
    vec![(None, DeliveryState::Failed)],
)]
#[case::blank_reply_is_not_queued(
    vec![open_thread(1, 0), reply_intent(1, NONCE_A, "   ")],
    vec![],
)]
fn replies_reconcile_by_client_nonce(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Vec<(Option<u128>, DeliveryState)>,
) {
    let (machine, _) = run(events);
    assert_eq!(thread_state(&machine).0, expected);
}

#[rstest]
#[case::reply_frame_carries_the_root(
    vec![reply_intent(1, NONCE_A, "  hello  ")],
    vec![reply_frame(1, NONCE_A, "hello")],
    None,
)]
#[case::blank_reply_emits_nothing(
    vec![reply_intent(1, NONCE_A, "  ")],
    vec![],
    Some(ApiError::invalid(InputField::MessageBody)),
)]
#[case::title_is_trimmed(
    vec![title_intent(1, "  Plans  ")],
    vec![topic_title_frame(1, "Plans")],
    None,
)]
#[case::blank_title_clears(vec![title_intent(1, "   ")], vec![topic_title_frame(1, "")], None)]
#[case::control_title_is_invalid(
    vec![title_intent(1, "bad\u{7}")],
    vec![],
    Some(ApiError::invalid(InputField::TopicTitle)),
)]
#[case::overlong_title_is_invalid(
    vec![title_intent(1, &"t".repeat(TOPIC_TITLE_MAX_CHARS + 1))],
    vec![],
    Some(ApiError::invalid(InputField::TopicTitle)),
)]
#[case::title_at_limit_is_sent(
    vec![title_intent(1, &"t".repeat(TOPIC_TITLE_MAX_CHARS))],
    vec![topic_title_frame(1, &"t".repeat(TOPIC_TITLE_MAX_CHARS))],
    None,
)]
#[case::explicit_subscription_is_sent_explicit(
    vec![subscribe(1, SubscriptionState::Muted)],
    vec![subscription_frame(1, SubscriptionState::Muted, SubscriptionOrigin::Explicit)],
    None,
)]
fn thread_intents_validate_then_emit_frames(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected_frames: Vec<ClientFrame>,
    #[case] expected_error: Option<ApiError>,
) {
    let (machine, frames) = run(events);
    assert_eq!(frames, expected_frames);
    assert_eq!(machine.snapshot(Some(ROOM_A)).error, expected_error);
}

#[test]
fn offline_reply_is_queued_and_sent_on_connect() {
    let (machine, frames) = run(vec![
        TimelineEvent::Disconnected,
        open_thread(1, 0),
        reply_intent(1, NONCE_A, "later"),
        reconnect(),
    ]);
    assert_eq!(
        frames,
        vec![
            resync(),
            sync_inbox_frame(),
            sync_shelf_frame(),
            reply_frame(1, NONCE_A, "later"),
            sync_thread_frame(1, None),
        ]
    );
    assert_eq!(
        thread_state(&machine).0,
        vec![(None, DeliveryState::Pending)]
    );
}

#[rstest]
#[case::forbidden(ThreadRejection::Forbidden, ApiError::new(ApiErrorCode::Forbidden))]
#[case::unknown_thread(ThreadRejection::UnknownThread, ApiError::new(ApiErrorCode::NotFound))]
#[case::invalid_title(
    ThreadRejection::InvalidTopicTitle,
    ApiError::invalid(InputField::TopicTitle)
)]
fn thread_rejections_surface_as_timeline_errors(
    #[case] reason: ThreadRejection,
    #[case] expected: ApiError,
) {
    let (machine, _) = run(vec![
        open_thread(1, 0),
        received(thread_rejected_frame(1, reason)),
    ]);
    assert_eq!(machine.snapshot(Some(ROOM_A)).error, Some(expected));
    assert!(!machine.thread_snapshot().syncing);
}
