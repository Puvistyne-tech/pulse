mod fixtures;

use fixtures::{
    CAMPUS_B, NONCE_A, ROOM_A, ROOM_B, STEWARD_A, STEWARD_NAME, STREAM_A, STREAM_B, abort_event,
    chain_message, chunk_event, clock, created_frame, message_id, nonce, open_thread, received,
    send_requested, settled_machine, steward_message, stream_chunk, stream_event,
    timeline_connected,
};
use pulse_core::{TimelineEvent, TimelineMachine};
use pulse_protocol::{
    ClientNonce, DeliveryState, STREAM_STALE_AFTER_MS, StreamAbortReason, StreamId, StreamPhase,
    StreamState, TimelineEntry, mention_token,
};
use rstest::rstest;

type Trace = (ClientNonce, String, Option<StreamState>);

fn trace(entries: &[TimelineEntry]) -> Vec<Trace> {
    entries
        .iter()
        .map(|entry| (entry.client_nonce, entry.body.clone(), entry.stream))
        .collect()
}

fn expected(rows: &[(ClientNonce, &str, Option<StreamState>)]) -> Vec<Trace> {
    rows.iter()
        .map(|(nonce, body, stream)| (*nonce, (*body).to_owned(), *stream))
        .collect()
}

fn timeline_of(machine: &TimelineMachine) -> Vec<Trace> {
    trace(&machine.snapshot(Some(ROOM_A)).entries)
}

fn begin(stream_id: StreamId, text: &str) -> TimelineEvent {
    stream_event(stream_id, StreamPhase::Begin, 0, text)
}

fn delta(stream_id: StreamId, sequence: u32, text: &str) -> TimelineEvent {
    stream_event(stream_id, StreamPhase::Delta, sequence, text)
}

fn end(stream_id: StreamId, sequence: u32) -> TimelineEvent {
    stream_event(stream_id, StreamPhase::End, sequence, "")
}

fn a() -> ClientNonce {
    STREAM_A.client_nonce()
}

fn b() -> ClientNonce {
    STREAM_B.client_nonce()
}

const STREAMING: Option<StreamState> = Some(StreamState::Streaming);
const FINALIZING: Option<StreamState> = Some(StreamState::Finalizing);

#[rstest]
#[case::begin_opens_an_empty_entry(vec![begin(STREAM_A, "")], &[(a(), "", STREAMING)])]
#[case::begin_text_is_kept(vec![begin(STREAM_A, "he")], &[(a(), "he", STREAMING)])]
#[case::deltas_concatenate(
    vec![begin(STREAM_A, ""), delta(STREAM_A, 1, "he"), delta(STREAM_A, 2, "llo")],
    &[(a(), "hello", STREAMING)]
)]
#[case::end_finalizes(
    vec![begin(STREAM_A, ""), delta(STREAM_A, 1, "hi"), end(STREAM_A, 2)],
    &[(a(), "hi", FINALIZING)]
)]
#[case::delta_without_begin_is_ignored(vec![delta(STREAM_A, 1, "x")], &[])]
#[case::begin_off_zero_is_ignored(
    vec![stream_event(STREAM_A, StreamPhase::Begin, 1, "x")],
    &[]
)]
#[case::repeated_begin_keeps_the_first(
    vec![begin(STREAM_A, "a"), delta(STREAM_A, 1, "b"), begin(STREAM_A, "zzz")],
    &[(a(), "ab", STREAMING)]
)]
#[case::gap_drops_the_stream(vec![begin(STREAM_A, ""), delta(STREAM_A, 2, "x")], &[])]
#[case::replayed_delta_drops_the_stream(
    vec![begin(STREAM_A, ""), delta(STREAM_A, 1, "x"), delta(STREAM_A, 1, "x")],
    &[]
)]
#[case::late_chunk_after_a_gap_stays_dropped(
    vec![begin(STREAM_A, ""), delta(STREAM_A, 2, "x"), delta(STREAM_A, 1, "y")],
    &[]
)]
#[case::abort_removes_the_entry(
    vec![begin(STREAM_A, ""), delta(STREAM_A, 1, "x"), abort_event(STREAM_A, 2, StreamAbortReason::Cancelled)],
    &[]
)]
#[case::abort_of_an_unknown_stream_is_inert(
    vec![abort_event(STREAM_A, 0, StreamAbortReason::Disconnected), begin(STREAM_B, "b")],
    &[(b(), "b", STREAMING)]
)]
#[case::abort_only_hits_its_own_stream(
    vec![begin(STREAM_A, "a"), begin(STREAM_B, "b"), abort_event(STREAM_A, 1, StreamAbortReason::TimedOut)],
    &[(b(), "b", STREAMING)]
)]
#[case::disconnect_clears_every_stream(
    vec![begin(STREAM_A, "a"), begin(STREAM_B, "b"), TimelineEvent::Disconnected],
    &[]
)]
#[case::chunks_after_a_drop_are_ignored(
    vec![
        begin(STREAM_A, "a"),
        TimelineEvent::Disconnected,
        timeline_connected(&[ROOM_A]),
        delta(STREAM_A, 1, "b"),
    ],
    &[]
)]
#[case::begin_orders_streams(
    vec![begin(STREAM_A, "a"), begin(STREAM_B, "b"), delta(STREAM_B, 1, "2"), delta(STREAM_A, 1, "1")],
    &[(a(), "a1", STREAMING), (b(), "b2", STREAMING)]
)]
#[case::reverse_begin_reverses_order(
    vec![begin(STREAM_B, "b"), begin(STREAM_A, "a"), delta(STREAM_A, 1, "1"), delta(STREAM_B, 1, "2")],
    &[(b(), "b2", STREAMING), (a(), "a1", STREAMING)]
)]
#[case::one_ending_does_not_touch_the_other(
    vec![begin(STREAM_A, "a"), begin(STREAM_B, "b"), end(STREAM_A, 1)],
    &[(a(), "a", FINALIZING), (b(), "b", STREAMING)]
)]
#[case::confirmed_message_replaces_the_entry(
    vec![
        begin(STREAM_A, ""),
        delta(STREAM_A, 1, "hi"),
        end(STREAM_A, 2),
        received(created_frame(steward_message(ROOM_A, 1, STREAM_A, "hi"))),
    ],
    &[(a(), "hi", None)]
)]
#[case::confirmed_before_end_leaves_one_entry(
    vec![
        begin(STREAM_A, ""),
        delta(STREAM_A, 1, "hi"),
        received(created_frame(steward_message(ROOM_A, 1, STREAM_A, "hi"))),
        chunk_event(pulse_protocol::AgentStreamChunk {
            message_id: Some(message_id(1)),
            ..stream_chunk(STREAM_A, StreamPhase::End, 2, "")
        }),
    ],
    &[(a(), "hi", None)]
)]
#[case::other_nonces_do_not_confirm(
    vec![begin(STREAM_A, "a"), received(created_frame(chain_message(ROOM_A, 1)))],
    &[(nonce(1), "peer 1", None), (a(), "a", STREAMING)]
)]
#[case::live_sits_between_confirmed_and_pending(
    vec![
        received(created_frame(chain_message(ROOM_A, 1))),
        send_requested(ROOM_A, NONCE_A, "mine"),
        begin(STREAM_A, "a"),
    ],
    &[(nonce(1), "peer 1", None), (a(), "a", STREAMING), (NONCE_A, "mine", None)]
)]
#[case::other_rooms_stay_out(
    vec![chunk_event(pulse_protocol::AgentStreamChunk { room_id: ROOM_B, ..stream_chunk(STREAM_A, StreamPhase::Begin, 0, "x") })],
    &[]
)]
#[case::foreign_campus_chunks_are_ignored(
    vec![chunk_event(pulse_protocol::AgentStreamChunk { campus_id: CAMPUS_B, ..stream_chunk(STREAM_A, StreamPhase::Begin, 0, "x") })],
    &[]
)]
fn live_entries_follow_the_chunk_stream(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected_entries: &[(ClientNonce, &str, Option<StreamState>)],
) {
    let machine = settled_machine(events);
    assert_eq!(timeline_of(&machine), expected(expected_entries));
}

#[rstest]
#[case::just_before_the_stale_line(1_000 + STREAM_STALE_AFTER_MS - 1, true)]
#[case::on_the_stale_line(1_000 + STREAM_STALE_AFTER_MS, false)]
#[case::long_after(1_000 + 10 * STREAM_STALE_AFTER_MS, false)]
fn silent_streams_expire_by_the_core_clock(#[case] now_ms: u64, #[case] alive: bool) {
    let machine = settled_machine(vec![clock(1_000), begin(STREAM_A, "a"), clock(now_ms)]);
    assert_eq!(timeline_of(&machine).len(), usize::from(alive));
}

#[rstest]
#[case::chunk_refreshes_the_window(40_000, 40_000 + STREAM_STALE_AFTER_MS - 1, true)]
#[case::refreshed_window_still_closes(40_000, 40_000 + STREAM_STALE_AFTER_MS, false)]
fn a_chunk_restarts_the_stale_window(
    #[case] chunk_at_ms: u64,
    #[case] now_ms: u64,
    #[case] alive: bool,
) {
    let machine = settled_machine(vec![
        clock(1_000),
        begin(STREAM_A, "a"),
        clock(chunk_at_ms),
        delta(STREAM_A, 1, "b"),
        clock(now_ms),
    ]);
    assert_eq!(timeline_of(&machine).len(), usize::from(alive));
}

#[rstest]
#[case::no_streams(vec![], None)]
#[case::one_stream(
    vec![clock(1_000), begin(STREAM_A, "a")],
    Some(1_000 + STREAM_STALE_AFTER_MS)
)]
#[case::earliest_of_two(
    vec![clock(1_000), begin(STREAM_A, "a"), clock(2_000), begin(STREAM_B, "b")],
    Some(1_000 + STREAM_STALE_AFTER_MS)
)]
#[case::refresh_moves_the_earliest(
    vec![
        clock(1_000),
        begin(STREAM_A, "a"),
        clock(2_000),
        begin(STREAM_B, "b"),
        clock(3_000),
        delta(STREAM_A, 1, "x"),
    ],
    Some(2_000 + STREAM_STALE_AFTER_MS)
)]
#[case::dropped_streams_leave_no_deadline(
    vec![clock(1_000), begin(STREAM_A, "a"), TimelineEvent::Disconnected],
    None
)]
fn stale_deadlines_reach_the_scheduler(
    #[case] events: Vec<TimelineEvent>,
    #[case] deadline: Option<u64>,
) {
    assert_eq!(settled_machine(events).next_deadline_ms(), deadline);
}

#[test]
fn live_entry_has_the_steward_shape() {
    let machine = settled_machine(vec![begin(STREAM_A, "**hi**")]);
    let entries = machine.snapshot(Some(ROOM_A)).entries;
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    assert_eq!(entry.client_nonce, STREAM_A.client_nonce());
    assert_eq!(entry.message_id, None);
    assert_eq!(entry.author_id, STEWARD_A);
    assert_eq!(entry.author_display_name, STEWARD_NAME);
    assert_eq!(entry.author_mention_token, mention_token(STEWARD_A));
    assert_eq!(entry.delivery, DeliveryState::Confirmed);
    assert!(entry.author_is_steward);
    assert!(!entry.from_self);
    assert_eq!(entry.stream, STREAMING);
    assert_eq!(entry.rejection, None);
}

#[rstest]
#[case::top_level_stream(None, 1, 0)]
#[case::reply_stream_of_the_open_thread(Some(1), 0, 1)]
#[case::reply_stream_of_another_thread(Some(2), 0, 0)]
fn streams_land_where_their_thread_root_says(
    #[case] thread_root: Option<u128>,
    #[case] timeline_streams: usize,
    #[case] thread_streams: usize,
) {
    let machine = settled_machine(vec![
        received(created_frame(chain_message(ROOM_A, 1))),
        open_thread(1, 0),
        chunk_event(pulse_protocol::AgentStreamChunk {
            thread_root_id: thread_root.map(message_id),
            ..stream_chunk(STREAM_A, StreamPhase::Begin, 0, "x")
        }),
    ]);
    let in_timeline = machine
        .snapshot(Some(ROOM_A))
        .entries
        .iter()
        .filter(|entry| entry.stream.is_some())
        .count();
    let in_thread = machine
        .thread_snapshot()
        .replies
        .iter()
        .filter(|entry| entry.stream.is_some())
        .count();
    assert_eq!((in_timeline, in_thread), (timeline_streams, thread_streams));
}
