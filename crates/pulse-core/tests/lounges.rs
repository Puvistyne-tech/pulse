mod fixtures;

use fixtures::{
    ACCOUNT_A, ACCOUNT_B, ACCOUNT_C, CAMPUS_B, LOUNGE_A, LOUNGE_B, ROOM_A, client_rooms, clock,
    connect_burst_at, connected_as, error_of, lounge_changed_frame, lounge_joined_frame,
    lounge_rejected_frame, lounge_with, lounges_synced_frame, received, settled_machine, step_all,
    timeline_connected,
};
use pulse_core::{LoungeIntent, TimelineEvent, TimelineMachine};
use pulse_protocol::{
    AccountId, ApiError, ApiErrorCode, ClientFrame, JoinLounge, LOUNGE_HEARTBEAT_MS, LeaveLounge,
    LoungeHeartbeat, LoungeId, LoungeMediaState, LoungeRejection, Role,
};
use rstest::rstest;

#[derive(Debug, PartialEq, Eq)]
struct Seen {
    lounges: Vec<(LoungeId, Vec<AccountId>)>,
    joined: Option<LoungeId>,
    joining: bool,
    ticketed: bool,
    media: LoungeMediaState,
    error: Option<ApiError>,
}

fn seen(machine: &TimelineMachine) -> Seen {
    let snapshot = machine.lounge_snapshot();
    Seen {
        lounges: snapshot
            .lounges
            .iter()
            .map(|entry| {
                (
                    entry.lounge_id,
                    entry
                        .participants
                        .iter()
                        .map(|participant| participant.account_id)
                        .collect(),
                )
            })
            .collect(),
        joined: snapshot.joined_lounge_id,
        joining: snapshot.joining,
        ticketed: snapshot.ticket.is_some(),
        media: snapshot.media,
        error: snapshot.error,
    }
}

fn idle() -> Seen {
    Seen {
        lounges: Vec::new(),
        joined: None,
        joining: false,
        ticketed: false,
        media: LoungeMediaState::Idle,
        error: None,
    }
}

fn join(lounge_id: LoungeId) -> TimelineEvent {
    TimelineEvent::Lounge(LoungeIntent::JoinRequested { lounge_id })
}

fn leave() -> TimelineEvent {
    TimelineEvent::Lounge(LoungeIntent::LeaveRequested)
}

fn media(state: LoungeMediaState) -> TimelineEvent {
    TimelineEvent::Lounge(LoungeIntent::MediaReported(state))
}

fn join_frame(lounge_id: LoungeId) -> ClientFrame {
    ClientFrame::JoinLounge(JoinLounge { lounge_id })
}

fn leave_frame(lounge_id: LoungeId) -> ClientFrame {
    ClientFrame::LeaveLounge(LeaveLounge { lounge_id })
}

fn beat_frame(lounge_id: LoungeId) -> ClientFrame {
    ClientFrame::LoungeHeartbeat(LoungeHeartbeat { lounge_id })
}

fn joined_in(lounge_id: LoungeId) -> TimelineEvent {
    received(lounge_joined_frame(lounge_with(lounge_id, 1, &[])))
}

const LATER: u64 = 100_000;

#[rstest]
#[case::sync_lists_lounges_campus_first(
    vec![received(lounges_synced_frame(vec![
        lounge_with(LOUNGE_B, 1, &[(ACCOUNT_B, LATER)]),
        lounge_with(LOUNGE_A, 1, &[(ACCOUNT_C, LATER)]),
    ]))],
    vec![(LOUNGE_A, vec![ACCOUNT_C]), (LOUNGE_B, vec![ACCOUNT_B])]
)]
#[case::higher_revision_replaces(
    vec![
        received(lounges_synced_frame(vec![lounge_with(LOUNGE_A, 1, &[(ACCOUNT_B, LATER)])])),
        received(lounge_changed_frame(lounge_with(LOUNGE_A, 2, &[(ACCOUNT_C, LATER)]))),
    ],
    vec![(LOUNGE_A, vec![ACCOUNT_C])]
)]
#[case::lower_revision_is_ignored(
    vec![
        received(lounge_changed_frame(lounge_with(LOUNGE_A, 3, &[(ACCOUNT_C, LATER)]))),
        received(lounge_changed_frame(lounge_with(LOUNGE_A, 2, &[(ACCOUNT_B, LATER)]))),
    ],
    vec![(LOUNGE_A, vec![ACCOUNT_C])]
)]
#[case::equal_revision_replays_converge(
    vec![
        received(lounge_changed_frame(lounge_with(LOUNGE_A, 2, &[(ACCOUNT_B, LATER)]))),
        received(lounge_changed_frame(lounge_with(LOUNGE_A, 2, &[(ACCOUNT_C, LATER)]))),
    ],
    vec![(LOUNGE_A, vec![ACCOUNT_C])]
)]
#[case::stale_sync_does_not_regress_a_change(
    vec![
        received(lounge_changed_frame(lounge_with(LOUNGE_A, 5, &[(ACCOUNT_C, LATER)]))),
        received(lounges_synced_frame(vec![lounge_with(LOUNGE_A, 4, &[(ACCOUNT_B, LATER)])])),
    ],
    vec![(LOUNGE_A, vec![ACCOUNT_C])]
)]
#[case::revisions_are_tracked_per_lounge(
    vec![
        received(lounge_changed_frame(lounge_with(LOUNGE_A, 9, &[(ACCOUNT_C, LATER)]))),
        received(lounge_changed_frame(lounge_with(LOUNGE_B, 1, &[(ACCOUNT_B, LATER)]))),
    ],
    vec![(LOUNGE_A, vec![ACCOUNT_C]), (LOUNGE_B, vec![ACCOUNT_B])]
)]
#[case::foreign_campus_changes_are_ignored(
    vec![
        received(lounge_changed_frame(pulse_protocol::Lounge {
            campus_id: CAMPUS_B,
            ..lounge_with(LOUNGE_A, 1, &[(ACCOUNT_B, LATER)])
        })),
        received(lounges_synced_frame(vec![pulse_protocol::Lounge {
            campus_id: CAMPUS_B,
            ..lounge_with(LOUNGE_B, 1, &[(ACCOUNT_B, LATER)])
        }])),
    ],
    vec![]
)]
#[case::participants_present_until_the_clock_reaches_them(
    vec![
        received(lounge_changed_frame(lounge_with(
            LOUNGE_A, 1, &[(ACCOUNT_B, 5_000), (ACCOUNT_C, 9_000)]
        ))),
        clock(4_999),
    ],
    vec![(LOUNGE_A, vec![ACCOUNT_B, ACCOUNT_C])]
)]
#[case::first_expiry_hides_one_participant(
    vec![
        received(lounge_changed_frame(lounge_with(
            LOUNGE_A, 1, &[(ACCOUNT_B, 5_000), (ACCOUNT_C, 9_000)]
        ))),
        clock(5_000),
    ],
    vec![(LOUNGE_A, vec![ACCOUNT_C])]
)]
#[case::last_expiry_empties_the_lounge(
    vec![
        received(lounge_changed_frame(lounge_with(
            LOUNGE_A, 1, &[(ACCOUNT_B, 5_000), (ACCOUNT_C, 9_000)]
        ))),
        clock(9_000),
    ],
    vec![(LOUNGE_A, vec![])]
)]
fn presence_follows_revisions_and_the_clock(
    #[case] events: Vec<TimelineEvent>,
    #[case] lounges: Vec<(LoungeId, Vec<AccountId>)>,
) {
    let machine = settled_machine(events);
    assert_eq!(seen(&machine), Seen { lounges, ..idle() });
}

#[rstest]
#[case::join_asks_the_server(
    vec![join(LOUNGE_A)],
    vec![join_frame(LOUNGE_A)],
    Seen { joined: Some(LOUNGE_A), joining: true, ..idle() }
)]
#[case::joined_frame_brings_the_ticket(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A)],
    vec![join_frame(LOUNGE_A)],
    Seen {
        lounges: vec![(LOUNGE_A, vec![])],
        joined: Some(LOUNGE_A),
        ticketed: true,
        ..idle()
    }
)]
#[case::unrequested_joined_frame_is_ignored(
    vec![joined_in(LOUNGE_B)],
    vec![],
    Seen { lounges: vec![(LOUNGE_B, vec![])], ..idle() }
)]
#[case::joined_frame_of_a_stale_request_is_ignored(
    vec![join(LOUNGE_A), joined_in(LOUNGE_B)],
    vec![join_frame(LOUNGE_A)],
    Seen {
        lounges: vec![(LOUNGE_B, vec![])],
        joined: Some(LOUNGE_A),
        joining: true,
        ..idle()
    }
)]
#[case::joining_the_same_lounge_twice_sends_once(
    vec![join(LOUNGE_A), join(LOUNGE_A)],
    vec![join_frame(LOUNGE_A)],
    Seen { joined: Some(LOUNGE_A), joining: true, ..idle() }
)]
#[case::switching_leaves_then_joins(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), join(LOUNGE_B)],
    vec![join_frame(LOUNGE_A), leave_frame(LOUNGE_A), join_frame(LOUNGE_B)],
    Seen {
        lounges: vec![(LOUNGE_A, vec![])],
        joined: Some(LOUNGE_B),
        joining: true,
        ..idle()
    }
)]
#[case::leave_clears_the_join(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), leave()],
    vec![join_frame(LOUNGE_A), leave_frame(LOUNGE_A)],
    Seen { lounges: vec![(LOUNGE_A, vec![])], ..idle() }
)]
#[case::leave_without_a_join_sends_nothing(vec![leave()], vec![], idle())]
#[case::leave_before_the_ticket_still_cancels(
    vec![join(LOUNGE_A), leave()],
    vec![join_frame(LOUNGE_A), leave_frame(LOUNGE_A)],
    idle()
)]
#[case::rejection_of_the_pending_join_clears_it(
    vec![join(LOUNGE_A), received(lounge_rejected_frame(LOUNGE_A, LoungeRejection::Full))],
    vec![join_frame(LOUNGE_A)],
    Seen { error: error_of(ApiErrorCode::Conflict), ..idle() }
)]
#[case::rejection_of_the_joined_lounge_drops_the_ticket(
    vec![
        join(LOUNGE_A),
        joined_in(LOUNGE_A),
        media(LoungeMediaState::Connected),
        received(lounge_rejected_frame(LOUNGE_A, LoungeRejection::Forbidden)),
    ],
    vec![join_frame(LOUNGE_A)],
    Seen {
        lounges: vec![(LOUNGE_A, vec![])],
        error: error_of(ApiErrorCode::Forbidden),
        ..idle()
    }
)]
#[case::rejection_of_another_lounge_keeps_the_join(
    vec![
        join(LOUNGE_A),
        joined_in(LOUNGE_A),
        received(lounge_rejected_frame(LOUNGE_B, LoungeRejection::UnknownLounge)),
    ],
    vec![join_frame(LOUNGE_A)],
    Seen {
        lounges: vec![(LOUNGE_A, vec![])],
        joined: Some(LOUNGE_A),
        ticketed: true,
        error: error_of(ApiErrorCode::NotFound),
        ..idle()
    }
)]
#[case::rejection_without_a_join_only_reports(
    vec![received(lounge_rejected_frame(LOUNGE_A, LoungeRejection::Unavailable))],
    vec![],
    Seen { error: error_of(ApiErrorCode::Unavailable), ..idle() }
)]
#[case::a_new_join_clears_the_error(
    vec![
        received(lounge_rejected_frame(LOUNGE_A, LoungeRejection::Unavailable)),
        join(LOUNGE_A),
    ],
    vec![join_frame(LOUNGE_A)],
    Seen { joined: Some(LOUNGE_A), joining: true, ..idle() }
)]
fn join_state_follows_intents_and_server_answers(
    #[case] events: Vec<TimelineEvent>,
    #[case] frames: Vec<ClientFrame>,
    #[case] expected: Seen,
) {
    let (machine, emitted) = step_all(settled_machine(vec![]), events);
    assert_eq!(emitted, frames);
    assert_eq!(seen(&machine), expected);
}

#[rstest]
#[case::no_ticket_means_idle(
    vec![media(LoungeMediaState::Connected)],
    LoungeMediaState::Idle
)]
#[case::pending_join_means_idle(
    vec![join(LOUNGE_A), media(LoungeMediaState::Connecting)],
    LoungeMediaState::Idle
)]
#[case::connecting(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), media(LoungeMediaState::Connecting)],
    LoungeMediaState::Connecting
)]
#[case::connected(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), media(LoungeMediaState::Connected)],
    LoungeMediaState::Connected
)]
#[case::failed(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), media(LoungeMediaState::Failed)],
    LoungeMediaState::Failed
)]
#[case::leaving_resets(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), media(LoungeMediaState::Connected), leave()],
    LoungeMediaState::Idle
)]
#[case::switching_resets(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), media(LoungeMediaState::Connected), join(LOUNGE_B)],
    LoungeMediaState::Idle
)]
#[case::late_report_after_leaving_is_idle(
    vec![
        join(LOUNGE_A),
        joined_in(LOUNGE_A),
        leave(),
        media(LoungeMediaState::Connected),
    ],
    LoungeMediaState::Idle
)]
fn media_reports_only_count_while_a_ticket_is_held(
    #[case] events: Vec<TimelineEvent>,
    #[case] state: LoungeMediaState,
) {
    let (machine, _) = step_all(settled_machine(vec![]), events);
    assert_eq!(machine.lounge_snapshot().media, state);
}

#[rstest]
#[case::joined_keeps_intent_and_ticket(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), TimelineEvent::Disconnected],
    Seen {
        lounges: vec![(LOUNGE_A, vec![])],
        joined: Some(LOUNGE_A),
        joining: true,
        ticketed: true,
        ..idle()
    }
)]
#[case::not_joined_stays_idle(
    vec![received(lounges_synced_frame(vec![lounge_with(LOUNGE_A, 1, &[])])), TimelineEvent::Disconnected],
    Seen { lounges: vec![(LOUNGE_A, vec![])], ..idle() }
)]
#[case::offline_join_records_the_intent(
    vec![TimelineEvent::Disconnected, join(LOUNGE_B)],
    Seen { joined: Some(LOUNGE_B), joining: true, ..idle() }
)]
#[case::offline_leave_clears_it(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), TimelineEvent::Disconnected, leave()],
    Seen { lounges: vec![(LOUNGE_A, vec![])], ..idle() }
)]
fn drops_keep_the_join_intent(#[case] events: Vec<TimelineEvent>, #[case] expected: Seen) {
    let (machine, _) = step_all(settled_machine(vec![]), events);
    assert_eq!(seen(&machine), expected);
}

#[rstest]
#[case::joined_before_the_drop(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), TimelineEvent::Disconnected],
    Some(LOUNGE_A)
)]
#[case::offline_join(vec![TimelineEvent::Disconnected, join(LOUNGE_B)], Some(LOUNGE_B))]
#[case::never_joined(vec![TimelineEvent::Disconnected], None)]
#[case::left_before_the_drop(
    vec![join(LOUNGE_A), joined_in(LOUNGE_A), leave(), TimelineEvent::Disconnected],
    None
)]
fn reconnect_burst_ends_with_the_join(
    #[case] events: Vec<TimelineEvent>,
    #[case] rejoined: Option<LoungeId>,
) {
    let (machine, _) = step_all(settled_machine(vec![]), events);
    let (machine, frames) = step_all(machine, vec![timeline_connected(&[ROOM_A])]);
    let expected = [
        connect_burst_at(&[(ROOM_A, None, Some(0))]),
        rejoined.map(join_frame).into_iter().collect(),
    ]
    .concat();
    assert_eq!(frames, expected);
    let joining = machine.lounge_snapshot().joining;
    assert_eq!(joining, rejoined.is_some());
}

#[test]
fn a_fresh_ticket_after_reconnect_ends_the_joining_state() {
    let (machine, _) = step_all(
        settled_machine(vec![]),
        vec![
            join(LOUNGE_A),
            joined_in(LOUNGE_A),
            TimelineEvent::Disconnected,
            timeline_connected(&[ROOM_A]),
            joined_in(LOUNGE_A),
        ],
    );
    let snapshot = machine.lounge_snapshot();
    assert!(!snapshot.joining);
    assert_eq!(snapshot.joined_lounge_id, Some(LOUNGE_A));
    assert!(snapshot.ticket.is_some());
}

#[rstest]
#[case::nothing_before_the_interval(
    vec![clock(1_000), join(LOUNGE_A), joined_in(LOUNGE_A), clock(1_000 + LOUNGE_HEARTBEAT_MS - 1)],
    vec![join_frame(LOUNGE_A)]
)]
#[case::beat_at_the_interval(
    vec![clock(1_000), join(LOUNGE_A), joined_in(LOUNGE_A), clock(1_000 + LOUNGE_HEARTBEAT_MS)],
    vec![join_frame(LOUNGE_A), beat_frame(LOUNGE_A)]
)]
#[case::beats_repeat_each_interval(
    vec![
        clock(1_000),
        join(LOUNGE_A),
        joined_in(LOUNGE_A),
        clock(1_000 + LOUNGE_HEARTBEAT_MS),
        clock(1_000 + 2 * LOUNGE_HEARTBEAT_MS - 1),
        clock(1_000 + 2 * LOUNGE_HEARTBEAT_MS),
    ],
    vec![join_frame(LOUNGE_A), beat_frame(LOUNGE_A), beat_frame(LOUNGE_A)]
)]
#[case::a_late_tick_beats_once(
    vec![clock(1_000), join(LOUNGE_A), joined_in(LOUNGE_A), clock(1_000 + 5 * LOUNGE_HEARTBEAT_MS)],
    vec![join_frame(LOUNGE_A), beat_frame(LOUNGE_A)]
)]
#[case::no_beat_before_the_ticket(
    vec![clock(1_000), join(LOUNGE_A), clock(1_000 + 3 * LOUNGE_HEARTBEAT_MS)],
    vec![join_frame(LOUNGE_A)]
)]
#[case::no_beat_after_leaving(
    vec![
        clock(1_000),
        join(LOUNGE_A),
        joined_in(LOUNGE_A),
        leave(),
        clock(1_000 + 3 * LOUNGE_HEARTBEAT_MS),
    ],
    vec![join_frame(LOUNGE_A), leave_frame(LOUNGE_A)]
)]
#[case::no_beat_while_offline(
    vec![
        clock(1_000),
        join(LOUNGE_A),
        joined_in(LOUNGE_A),
        TimelineEvent::Disconnected,
        clock(1_000 + 3 * LOUNGE_HEARTBEAT_MS),
    ],
    vec![join_frame(LOUNGE_A)]
)]
fn heartbeats_follow_the_core_clock(
    #[case] events: Vec<TimelineEvent>,
    #[case] frames: Vec<ClientFrame>,
) {
    let (_, emitted) = step_all(settled_machine(vec![]), events);
    assert_eq!(emitted, frames);
}

#[rstest]
#[case::nothing_scheduled(vec![], None)]
#[case::participant_expiry(
    vec![received(lounge_changed_frame(lounge_with(LOUNGE_A, 1, &[(ACCOUNT_B, 6_000), (ACCOUNT_C, 8_000)])))],
    Some(6_000)
)]
#[case::heartbeat_without_participants(
    vec![clock(1_000), join(LOUNGE_A), joined_in(LOUNGE_A)],
    Some(1_000 + LOUNGE_HEARTBEAT_MS)
)]
#[case::heartbeat_beats_a_later_expiry(
    vec![
        clock(1_000),
        join(LOUNGE_A),
        received(lounge_joined_frame(lounge_with(LOUNGE_A, 1, &[(ACCOUNT_B, 50_000)]))),
    ],
    Some(1_000 + LOUNGE_HEARTBEAT_MS)
)]
#[case::expiry_beats_a_later_heartbeat(
    vec![
        clock(1_000),
        join(LOUNGE_A),
        received(lounge_joined_frame(lounge_with(LOUNGE_A, 1, &[(ACCOUNT_B, 6_000)]))),
    ],
    Some(6_000)
)]
#[case::expired_participants_leave_the_schedule(
    vec![
        received(lounge_changed_frame(lounge_with(LOUNGE_A, 1, &[(ACCOUNT_B, 6_000), (ACCOUNT_C, 8_000)]))),
        clock(6_000),
    ],
    Some(8_000)
)]
fn deadlines_cover_expiry_and_heartbeat(
    #[case] events: Vec<TimelineEvent>,
    #[case] deadline: Option<u64>,
) {
    assert_eq!(settled_machine(events).next_deadline_ms(), deadline);
}

#[rstest]
#[case::owner(Role::Owner, true, true)]
#[case::admin(Role::Admin, true, true)]
#[case::member(Role::Member, true, true)]
#[case::guest(Role::Guest, true, false)]
#[case::steward(Role::Steward, false, false)]
fn permissions_come_from_the_role(
    #[case] role: Role,
    #[case] can_join: bool,
    #[case] can_speak: bool,
) {
    let snapshot = connected_as(role).snapshot().lounges;
    assert_eq!(
        (snapshot.can_join, snapshot.can_speak),
        (can_join, can_speak)
    );
}

#[test]
fn snapshot_marks_self_and_names_room_lounges() {
    let mut client = connected_as(Role::Member);
    let rooms = client_rooms(&client);
    let slugs = client
        .snapshot()
        .session
        .rooms
        .iter()
        .map(|room| room.slug.clone())
        .collect::<Vec<_>>();
    let room_lounge = pulse_protocol::Lounge {
        room_id: Some(rooms[1]),
        ..lounge_with(LOUNGE_B, 1, &[(ACCOUNT_A, LATER)])
    };
    client.receive_frame(lounges_synced_frame(vec![
        room_lounge,
        lounge_with(LOUNGE_A, 1, &[(ACCOUNT_B, LATER)]),
    ]));
    let lounges = client.snapshot().lounges.lounges;
    assert_eq!(lounges.len(), 2);
    assert_eq!(lounges[0].room_slug, None);
    assert_eq!(lounges[1].room_slug, Some(slugs[1].clone()));
    assert!(!lounges[0].participants[0].from_self);
    assert!(lounges[1].participants[0].from_self);
}
