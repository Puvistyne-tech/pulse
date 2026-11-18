mod fixtures;

use fixtures::{
    ROOM_A, ShelfSpec, chain, clock, message_id, open_thread, received, settled_machine,
    shelf_changed_frame, shelf_event, shelf_rejected_frame, shelf_synced_frame, shelf_update,
    step_all, synced_frame, timeline_run,
};
use pulse_core::{ShelfIntent, TimelineEvent, TimelineMachine};
use pulse_protocol::{
    ApiError, ApiErrorCode, ClientFrame, InputField, MessageId, REMIND_AT_HORIZON_MS,
    ReminderPreset, ShelfAction, ShelfBucket, ShelfEntry, ShelfRejection, UpdateShelf,
};
use rstest::rstest;

fn ids(entries: &[ShelfEntry]) -> Vec<MessageId> {
    entries.iter().map(|entry| entry.message_id).collect()
}

fn indexes(items: &[u128]) -> Vec<MessageId> {
    items.iter().copied().map(message_id).collect()
}

fn shelf_machine(events: Vec<TimelineEvent>) -> TimelineMachine {
    let mut all = vec![received(synced_frame(ROOM_A, chain(ROOM_A, 12), false))];
    all.extend(events);
    settled_machine(all)
}

#[rstest]
#[case::no_reminder(ShelfSpec::new(1), 1_000, &[], &[1], &[], 0)]
#[case::future_reminder(ShelfSpec::new(1).remind(5_000), 1_000, &[], &[1], &[], 0)]
#[case::one_millisecond_early(ShelfSpec::new(1).remind(5_000), 4_999, &[], &[1], &[], 0)]
#[case::exactly_due(ShelfSpec::new(1).remind(5_000), 5_000, &[1], &[], &[], 1)]
#[case::overdue(ShelfSpec::new(1).remind(500), 1_000, &[1], &[], &[], 1)]
#[case::server_delivered_beats_a_future_time(ShelfSpec::new(1).remind(9_000).reminded(800), 1_000, &[1], &[], &[], 1)]
#[case::done_without_reminder(ShelfSpec::new(1).done(900), 1_000, &[], &[], &[1], 0)]
#[case::done_beats_overdue(ShelfSpec::new(1).remind(500).done(900), 1_000, &[], &[], &[1], 0)]
#[case::done_beats_delivered(ShelfSpec::new(1).remind(500).reminded(600).done(900), 1_000, &[], &[], &[1], 0)]
#[case::done_with_future_time(ShelfSpec::new(1).remind(9_000).done(900), 1_000, &[], &[], &[1], 0)]
fn buckets_follow_the_core_clock(
    #[case] spec: ShelfSpec,
    #[case] clock_ms: u64,
    #[case] due: &[u128],
    #[case] upcoming: &[u128],
    #[case] done: &[u128],
    #[case] due_count: u32,
) {
    let machine = shelf_machine(vec![clock(clock_ms), received(shelf_synced_frame(&[spec]))]);
    let snapshot = machine.shelf_snapshot();
    assert_eq!(ids(&snapshot.due), indexes(due));
    assert_eq!(ids(&snapshot.upcoming), indexes(upcoming));
    assert_eq!(ids(&snapshot.done), indexes(done));
    assert_eq!(snapshot.due_count, due_count);
    let entries: Vec<_> = [&snapshot.due, &snapshot.upcoming, &snapshot.done]
        .into_iter()
        .flatten()
        .collect();
    assert_eq!(entries.len(), 1);
    assert!(!entries[0].pending);
    assert_eq!(entries[0].shelf_item_id, Some(fixtures::shelf_item_id(1)));
    let expected_bucket = if !done.is_empty() {
        ShelfBucket::Done
    } else if !due.is_empty() {
        ShelfBucket::Due
    } else {
        ShelfBucket::Upcoming
    };
    assert_eq!(entries[0].bucket, expected_bucket);
}

#[test]
fn an_upcoming_reminder_turns_due_when_the_clock_reaches_it() {
    let machine = shelf_machine(vec![
        clock(1_000),
        received(shelf_synced_frame(&[ShelfSpec::new(1).remind(5_000)])),
    ]);
    assert_eq!(machine.shelf_snapshot().due_count, 0);
    let (machine, _) = step_all(machine, vec![clock(5_000)]);
    let snapshot = machine.shelf_snapshot();
    assert_eq!(snapshot.due_count, 1);
    assert_eq!(ids(&snapshot.due), indexes(&[1]));
    let (machine, _) = step_all(machine, vec![clock(100)]);
    assert_eq!(machine.shelf_snapshot().due_count, 1);
}

#[test]
fn buckets_are_sorted_by_reminder_then_recency() {
    let specs = [
        ShelfSpec::new(1).remind(300),
        ShelfSpec::new(2).remind(100),
        ShelfSpec::new(3).remind(200).reminded(250),
        ShelfSpec::new(4).remind(5_000).saved(10),
        ShelfSpec::new(5).remind(3_000).saved(10),
        ShelfSpec::new(6).saved(50),
        ShelfSpec::new(7).saved(70),
        ShelfSpec::new(8).remind(3_000).saved(30),
        ShelfSpec::new(9).done(400),
        ShelfSpec::new(10).done(900),
        ShelfSpec::new(11).done(600),
    ];
    let machine = shelf_machine(vec![clock(1_000), received(shelf_synced_frame(&specs))]);
    let snapshot = machine.shelf_snapshot();
    assert_eq!(ids(&snapshot.due), indexes(&[2, 3, 1]));
    assert_eq!(ids(&snapshot.upcoming), indexes(&[8, 5, 4, 7, 6]));
    assert_eq!(ids(&snapshot.done), indexes(&[10, 11, 9]));
    assert_eq!(snapshot.due_count, 3);
    assert_eq!(snapshot.presets, ReminderPreset::ALL.to_vec());
}

type Remind = Vec<(u128, Option<u64>)>;

fn reminders(machine: &TimelineMachine) -> Remind {
    let snapshot = machine.shelf_snapshot();
    let mut found: Remind = [&snapshot.due, &snapshot.upcoming, &snapshot.done]
        .into_iter()
        .flatten()
        .map(|entry| {
            let index = (1..=12)
                .find(|index| message_id(*index) == entry.message_id)
                .expect("known message");
            (index, entry.remind_at_ms)
        })
        .collect();
    found.sort();
    found
}

#[rstest]
#[case::newer_revision_wins(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(100), false), shelf_changed_frame(ShelfSpec::new(1).remind(200).revision(2), false)],
    vec![(1, Some(200))]
)]
#[case::stale_revision_is_ignored(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(200).revision(2), false), shelf_changed_frame(ShelfSpec::new(1).remind(100), false)],
    vec![(1, Some(200))]
)]
#[case::equal_revision_replaces(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(200).revision(2), false), shelf_changed_frame(ShelfSpec::new(1).remind(300).revision(2), false)],
    vec![(1, Some(300))]
)]
#[case::removal_applies(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(100), false), shelf_changed_frame(ShelfSpec::new(1).revision(2), true)],
    vec![]
)]
#[case::stale_removal_is_ignored(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(100).revision(3), false), shelf_changed_frame(ShelfSpec::new(1).revision(2), true)],
    vec![(1, Some(100))]
)]
#[case::sync_replaces_the_list(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(100), false), shelf_synced_frame(&[ShelfSpec::new(2).remind(50)])],
    vec![(2, Some(50))]
)]
#[case::sync_keeps_a_newer_local_item(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(300).revision(3), false), shelf_synced_frame(&[ShelfSpec::new(1).remind(100)])],
    vec![(1, Some(300))]
)]
#[case::sync_takes_a_newer_server_item(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(100), false), shelf_synced_frame(&[ShelfSpec::new(1).remind(200).revision(2)])],
    vec![(1, Some(200))]
)]
#[case::empty_sync_clears_everything(
    vec![shelf_changed_frame(ShelfSpec::new(1).remind(100), false), shelf_synced_frame(&[])],
    vec![]
)]
fn revisions_decide_which_item_wins(
    #[case] frames: Vec<pulse_protocol::ServerFrame>,
    #[case] expected: Remind,
) {
    let machine = shelf_machine(frames.into_iter().map(received).collect());
    assert_eq!(reminders(&machine), expected);
}

#[rstest]
#[case::just_before_the_tolerance(699_999, true)]
#[case::at_the_tolerance(700_000, false)]
fn past_times_beyond_the_tolerance_are_refused(#[case] remind_at_ms: u64, #[case] refused: bool) {
    let (machine, frames) = run_online(vec![
        clock(1_000_000),
        shelf_update(2, ShelfAction::Save, None, Some(remind_at_ms)),
    ]);
    assert_eq!(frames.is_empty(), refused);
    assert_eq!(
        machine.shelf_snapshot().error,
        refused.then(|| ApiError::invalid(InputField::RemindAt))
    );
}

fn update_frame(index: u128, action: ShelfAction, remind_at_ms: Option<u64>) -> ClientFrame {
    ClientFrame::UpdateShelf(UpdateShelf {
        room_id: ROOM_A,
        message_id: message_id(index),
        action,
        remind_at_ms,
    })
}

fn shelf_frames(frames: Vec<ClientFrame>) -> Vec<ClientFrame> {
    frames
        .into_iter()
        .filter(|frame| matches!(frame, ClientFrame::UpdateShelf(_)))
        .collect()
}

fn run_online(events: Vec<TimelineEvent>) -> (TimelineMachine, Vec<ClientFrame>) {
    let machine = shelf_machine(vec![]);
    let (machine, frames) = step_all(machine, events);
    (machine, shelf_frames(frames))
}

#[rstest]
#[case::save(shelf_update(2, ShelfAction::Save, None, None), vec![update_frame(2, ShelfAction::Save, None)], None, None)]
#[case::save_with_time(shelf_update(2, ShelfAction::Save, None, Some(9_000)), vec![update_frame(2, ShelfAction::Save, Some(9_000))], Some(9_000), None)]
#[case::preset_is_added_to_the_clock(
    shelf_update(2, ShelfAction::Save, Some(ReminderPreset::InOneHour), None),
    vec![update_frame(2, ShelfAction::Save, Some(1_000 + 3_600_000))],
    Some(1_000 + 3_600_000),
    None
)]
#[case::explicit_time_beats_a_preset(
    shelf_update(2, ShelfAction::Save, Some(ReminderPreset::InOneHour), Some(7_000)),
    vec![update_frame(2, ShelfAction::Save, Some(7_000))],
    Some(7_000),
    None
)]
#[case::reschedule_without_a_time_is_refused(
    shelf_update(2, ShelfAction::Reschedule, None, None),
    vec![],
    None,
    Some(ApiError::invalid(InputField::RemindAt))
)]
#[case::complete_with_a_time_is_refused(
    shelf_update(2, ShelfAction::Complete, None, Some(9_000)),
    vec![],
    None,
    Some(ApiError::invalid(InputField::RemindAt))
)]
#[case::time_inside_the_past_tolerance_is_sent(
    shelf_update(2, ShelfAction::Save, None, Some(0)),
    vec![update_frame(2, ShelfAction::Save, Some(0))],
    Some(0),
    None
)]
#[case::time_beyond_the_horizon_is_refused(
    shelf_update(2, ShelfAction::Save, None, Some(1_000 + REMIND_AT_HORIZON_MS + 1)),
    vec![],
    None,
    Some(ApiError::invalid(InputField::RemindAt))
)]
#[case::unknown_message_is_refused(
    shelf_update(99, ShelfAction::Save, None, None),
    vec![],
    None,
    Some(ApiError::new(ApiErrorCode::NotFound))
)]
fn requests_are_validated_before_they_leave_the_core(
    #[case] request: TimelineEvent,
    #[case] expected_frames: Vec<ClientFrame>,
    #[case] expected_reminder: Option<u64>,
    #[case] expected_error: Option<ApiError>,
) {
    let (machine, frames) = run_online(vec![clock(1_000), request]);
    assert_eq!(frames, expected_frames);
    let snapshot = machine.shelf_snapshot();
    assert_eq!(snapshot.error, expected_error);
    if expected_frames.is_empty() {
        assert!(snapshot.upcoming.is_empty() && snapshot.due.is_empty());
    } else {
        let entry = [&snapshot.due, &snapshot.upcoming]
            .into_iter()
            .flatten()
            .next()
            .expect("optimistic entry");
        assert_eq!(entry.remind_at_ms, expected_reminder);
        assert!(entry.pending);
        assert_eq!(entry.shelf_item_id, None);
    }
}

#[test]
fn every_preset_adds_its_own_offset_to_the_core_clock() {
    for preset in ReminderPreset::ALL {
        let (_, frames) = run_online(vec![
            clock(5_000),
            shelf_update(2, ShelfAction::Save, Some(preset), None),
        ]);
        assert_eq!(
            frames,
            vec![update_frame(
                2,
                ShelfAction::Save,
                Some(5_000 + preset.offset_ms())
            )]
        );
    }
}

#[test]
fn requests_while_disconnected_report_unavailable_and_send_nothing() {
    let (machine, frames) = timeline_run(vec![
        received(synced_frame(ROOM_A, chain(ROOM_A, 3), false)),
        shelf_update(2, ShelfAction::Save, None, None),
    ]);
    assert!(frames.is_empty());
    let snapshot = machine.shelf_snapshot();
    assert_eq!(
        snapshot.error,
        Some(ApiError::new(ApiErrorCode::Unavailable))
    );
    assert!(snapshot.upcoming.is_empty());
}

#[test]
fn the_server_echo_confirms_an_optimistic_save() {
    let (machine, _) = run_online(vec![
        shelf_update(2, ShelfAction::Save, None, None),
        received(shelf_changed_frame(ShelfSpec::new(2), false)),
    ]);
    let snapshot = machine.shelf_snapshot();
    assert_eq!(ids(&snapshot.upcoming), indexes(&[2]));
    assert!(!snapshot.upcoming[0].pending);
    assert_eq!(
        snapshot.upcoming[0].shelf_item_id,
        Some(fixtures::shelf_item_id(2))
    );
    assert_eq!(snapshot.error, None);
}

#[test]
fn a_stale_echo_does_not_confirm_a_newer_optimistic_change() {
    let machine = shelf_machine(vec![received(shelf_synced_frame(&[
        ShelfSpec::new(2).revision(5)
    ]))]);
    let (machine, _) = step_all(
        machine,
        vec![
            shelf_update(2, ShelfAction::Complete, None, None),
            received(shelf_changed_frame(ShelfSpec::new(2).revision(4), false)),
        ],
    );
    let snapshot = machine.shelf_snapshot();
    assert_eq!(ids(&snapshot.done), indexes(&[2]));
    assert!(snapshot.done[0].pending);
}

#[rstest]
#[case::forbidden(ShelfRejection::Forbidden, ApiError::new(ApiErrorCode::Forbidden))]
#[case::unknown_message(ShelfRejection::UnknownMessage, ApiError::new(ApiErrorCode::NotFound))]
#[case::unknown_item(ShelfRejection::UnknownItem, ApiError::new(ApiErrorCode::NotFound))]
#[case::invalid_time(
    ShelfRejection::InvalidRemindAt,
    ApiError::invalid(InputField::RemindAt)
)]
#[case::shelf_full(ShelfRejection::ShelfFull, ApiError::new(ApiErrorCode::Conflict))]
fn a_rejected_save_rolls_back_and_reports(
    #[case] reason: ShelfRejection,
    #[case] expected: ApiError,
) {
    let (machine, _) = run_online(vec![
        shelf_update(2, ShelfAction::Save, None, None),
        received(shelf_rejected_frame(2, ShelfAction::Save, reason)),
    ]);
    let snapshot = machine.shelf_snapshot();
    assert!(snapshot.upcoming.is_empty() && snapshot.due.is_empty() && snapshot.done.is_empty());
    assert_eq!(snapshot.error, Some(expected));
}

#[rstest]
#[case::complete(ShelfAction::Complete, vec![], vec![2])]
#[case::remove(ShelfAction::Remove, vec![], vec![])]
fn optimistic_changes_to_a_saved_item_show_at_once_and_roll_back(
    #[case] action: ShelfAction,
    #[case] expected_upcoming_during: Vec<u128>,
    #[case] expected_done_during: Vec<u128>,
) {
    let saved = shelf_machine(vec![received(shelf_synced_frame(&[ShelfSpec::new(2)]))]);
    let (during, frames) = step_all(saved, vec![shelf_update(2, action, None, None)]);
    assert_eq!(shelf_frames(frames), vec![update_frame(2, action, None)]);
    let snapshot = during.shelf_snapshot();
    assert_eq!(ids(&snapshot.upcoming), indexes(&expected_upcoming_during));
    assert_eq!(ids(&snapshot.done), indexes(&expected_done_during));
    let (after, _) = step_all(
        during,
        vec![received(shelf_rejected_frame(
            2,
            action,
            ShelfRejection::Forbidden,
        ))],
    );
    let restored = after.shelf_snapshot();
    assert_eq!(ids(&restored.upcoming), indexes(&[2]));
    assert!(restored.done.is_empty());
    assert!(!restored.upcoming[0].pending);
}

#[test]
fn a_dropped_connection_discards_unconfirmed_changes() {
    let (machine, _) = run_online(vec![shelf_update(2, ShelfAction::Save, None, None)]);
    assert_eq!(machine.shelf_snapshot().upcoming.len(), 1);
    let (machine, _) = step_all(machine, vec![TimelineEvent::Disconnected]);
    assert!(machine.shelf_snapshot().upcoming.is_empty());
}

#[test]
fn saved_messages_are_flagged_in_the_timeline_even_while_unconfirmed() {
    let flags = |machine: &TimelineMachine| {
        machine
            .snapshot(Some(ROOM_A))
            .entries
            .iter()
            .map(|entry| entry.shelved)
            .collect::<Vec<_>>()
    };
    let machine = shelf_machine(vec![]);
    assert_eq!(flags(&machine), vec![false; 12]);
    let (machine, _) = step_all(
        machine,
        vec![shelf_update(2, ShelfAction::Save, None, None)],
    );
    assert_eq!(flags(&machine).iter().filter(|flag| **flag).count(), 1);
    let (machine, _) = step_all(
        machine,
        vec![
            received(shelf_changed_frame(ShelfSpec::new(2), false)),
            shelf_update(2, ShelfAction::Remove, None, None),
        ],
    );
    assert_eq!(flags(&machine), vec![false; 12]);
}

#[test]
fn toggling_the_shelf_flips_visibility_and_clears_the_error() {
    let (machine, _) = timeline_run(vec![shelf_update(2, ShelfAction::Save, None, None)]);
    assert!(machine.shelf_snapshot().error.is_some());
    let (machine, _) = step_all(machine, vec![shelf_event(ShelfIntent::Toggled)]);
    let opened = machine.shelf_snapshot();
    assert!(opened.open);
    assert_eq!(opened.error, None);
    let (machine, _) = step_all(machine, vec![shelf_event(ShelfIntent::Toggled)]);
    assert!(!machine.shelf_snapshot().open);
}

#[rstest]
#[case::empty_shelf(vec![], None, None)]
#[case::single_upcoming(vec![ShelfSpec::new(1).remind(5_000)], None, Some(5_000))]
#[case::earliest_of_several(
    vec![ShelfSpec::new(1).remind(7_000), ShelfSpec::new(2).remind(3_000), ShelfSpec::new(3)],
    None,
    Some(3_000)
)]
#[case::overdue_has_no_deadline(vec![ShelfSpec::new(1).remind(500)], None, None)]
#[case::reminder_due_exactly_now_has_no_deadline(vec![ShelfSpec::new(1).remind(1_000)], None, None)]
#[case::delivered_has_no_deadline(vec![ShelfSpec::new(1).remind(9_000).reminded(800)], None, None)]
#[case::done_has_no_deadline(vec![ShelfSpec::new(1).remind(9_000).done(800)], None, None)]
#[case::thread_follow_comes_first(vec![ShelfSpec::new(1).remind(5_000)], Some(0), Some(3_000))]
#[case::reminder_comes_before_thread_follow(vec![ShelfSpec::new(1).remind(2_000)], Some(0), Some(2_000))]
#[case::thread_follow_alone(vec![], Some(0), Some(3_000))]
fn the_next_deadline_includes_upcoming_reminders(
    #[case] specs: Vec<ShelfSpec>,
    #[case] thread_opened_at: Option<u64>,
    #[case] expected: Option<u64>,
) {
    let mut events = vec![clock(1_000), received(shelf_synced_frame(&specs))];
    if let Some(now) = thread_opened_at {
        events.insert(0, open_thread(1, now));
    }
    let machine = shelf_machine(events);
    assert_eq!(machine.next_deadline_ms(), expected);
}

#[rstest]
#[case::delivered_reminder_notifies(
    vec![received(shelf_changed_frame(ShelfSpec::new(1).remind(100).reminded(150).revision(2), false))],
    Some(1)
)]
#[case::plain_change_does_not_notify(vec![received(shelf_changed_frame(ShelfSpec::new(1).remind(100), false))], None)]
#[case::sync_does_not_notify(
    vec![received(shelf_synced_frame(&[ShelfSpec::new(1).remind(100).reminded(150)]))],
    None
)]
#[case::repeat_of_a_delivered_item_keeps_the_first_notice(
    vec![
        received(shelf_changed_frame(ShelfSpec::new(1).remind(100).reminded(150).revision(2), false)),
        received(shelf_changed_frame(ShelfSpec::new(2).remind(100).reminded(150).revision(2), false)),
        received(shelf_changed_frame(ShelfSpec::new(1).remind(100).reminded(150).revision(3), false)),
    ],
    Some(2)
)]
#[case::later_reminder_replaces_the_notice(
    vec![
        received(shelf_changed_frame(ShelfSpec::new(1).remind(100).reminded(150).revision(2), false)),
        received(shelf_changed_frame(ShelfSpec::new(2).remind(100).reminded(160).revision(2), false)),
    ],
    Some(2)
)]
#[case::dismissed_notice_is_gone(
    vec![
        received(shelf_changed_frame(ShelfSpec::new(1).remind(100).reminded(150).revision(2), false)),
        shelf_event(ShelfIntent::ReminderDismissed),
    ],
    None
)]
#[case::removed_item_has_no_notice(
    vec![
        received(shelf_changed_frame(ShelfSpec::new(1).remind(100).reminded(150).revision(2), false)),
        received(shelf_changed_frame(ShelfSpec::new(1).revision(3), true)),
    ],
    None
)]
fn the_latest_reminder_tracks_newly_delivered_items(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Option<u128>,
) {
    let machine = shelf_machine(events);
    let snapshot = machine.shelf_snapshot();
    assert_eq!(
        snapshot.latest_reminder.map(|entry| entry.message_id),
        expected.map(message_id)
    );
}
