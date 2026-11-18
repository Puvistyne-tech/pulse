mod fixtures;

use fixtures::{
    ACCOUNT_A, ACCOUNT_B, ACCOUNT_C, CAMPUS_B, EMOJI_A, EMOJI_B, EntryKey, NONCE_A, NONCE_B,
    ROOM_A, ROOM_B, accepted_frame, chain, chain_message, confirmed_key, created_frame, entry_keys,
    failed_key, message_id, nonce, own_message, peer_reply, pending_key, reacted, received,
    rejected_frame, room_message, send_frame, send_requested, sync_frame, sync_frame_at,
    sync_inbox_frame, sync_shelf_frame, synced_frame, synced_frame_with, timeline_after,
    timeline_connected, timeline_run, titled, unavailable_frame, updated_frame, with_replies,
};
use pulse_core::TimelineEvent;
use pulse_protocol::{
    ApiError, ClientFrame, DeliveryState, InputField, MessageId, MessageRejection, RoomId,
    RoomMessage, SYNC_MAX_ROOMS,
};
use rstest::rstest;

struct Expect {
    frames: Vec<ClientFrame>,
    entries: Vec<EntryKey>,
    syncing: bool,
    error: Option<ApiError>,
    cursor: Option<MessageId>,
}

fn own() -> RoomMessage {
    own_message(ROOM_A, 1, NONCE_A)
}

fn peer(index: u128) -> RoomMessage {
    chain_message(ROOM_A, index)
}

fn connected_a() -> TimelineEvent {
    timeline_connected(&[ROOM_A])
}

fn initial_sync() -> ClientFrame {
    sync_frame(&[(ROOM_A, None)])
}

fn resumed(after: Option<MessageId>) -> ClientFrame {
    sync_frame_at(&[(ROOM_A, after, Some(0))])
}

fn settled() -> TimelineEvent {
    received(synced_frame(ROOM_A, vec![], false))
}

fn rejected_nonce_a(reason: MessageRejection) -> TimelineEvent {
    received(rejected_frame(NONCE_A, reason))
}

fn foreign(message: RoomMessage) -> RoomMessage {
    RoomMessage {
        campus_id: CAMPUS_B,
        ..message
    }
}

#[rstest]
#[case::pending_while_connected(
    vec![connected_a(), send_requested(ROOM_A, NONCE_A, "  hi  ")],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "hi")],
        entries: vec![pending_key(NONCE_A)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::queued_while_offline_then_sent_on_connect(
    vec![send_requested(ROOM_A, NONCE_A, "hello"), connected_a()],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "hello")],
        entries: vec![pending_key(NONCE_A)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::pending_offline_emits_nothing(
    vec![send_requested(ROOM_A, NONCE_A, "offline")],
    Expect {
        frames: vec![],
        entries: vec![pending_key(NONCE_A)],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::accepted_confirms_pending(
    vec![connected_a(), send_requested(ROOM_A, NONCE_A, "hi"), settled(), received(accepted_frame(own()))],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "hi")],
        entries: vec![confirmed_key(&own())],
        syncing: false,
        error: None,
        cursor: Some(message_id(1)),
    },
)]
#[case::accepted_then_created_echo_is_one_entry(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "hi!"),
        settled(),
        received(accepted_frame(own())),
        received(created_frame(own())),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "hi!")],
        entries: vec![confirmed_key(&own())],
        syncing: false,
        error: None,
        cursor: Some(message_id(1)),
    },
)]
#[case::created_echo_then_accepted_is_one_entry(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "hi?"),
        settled(),
        received(created_frame(own())),
        received(accepted_frame(own())),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "hi?")],
        entries: vec![confirmed_key(&own())],
        syncing: false,
        error: None,
        cursor: Some(message_id(1)),
    },
)]
#[case::resend_after_reconnect_reuses_nonce_and_replay_is_one_entry(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "again"),
        TimelineEvent::Disconnected,
        connected_a(),
        settled(),
        received(accepted_frame(own())),
        received(accepted_frame(own())),
    ],
    Expect {
        frames: vec![
            initial_sync(), sync_inbox_frame(), sync_shelf_frame(),
            send_frame(ROOM_A, NONCE_A, "again"),
            initial_sync(), sync_inbox_frame(), sync_shelf_frame(),
            send_frame(ROOM_A, NONCE_A, "again"),
        ],
        entries: vec![confirmed_key(&own())],
        syncing: false,
        error: None,
        cursor: Some(message_id(1)),
    },
)]
#[case::out_of_order_created_converges_with_single_gap_sync(
    vec![
        connected_a(),
        settled(),
        received(created_frame(peer(3))),
        received(created_frame(peer(1))),
        received(created_frame(peer(2))),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), resumed(None)],
        entries: vec![confirmed_key(&peer(1)), confirmed_key(&peer(2)), confirmed_key(&peer(3))],
        syncing: true,
        error: None,
        cursor: Some(message_id(3)),
    },
)]
#[case::gap_syncs_once_from_chain_cursor(
    vec![
        connected_a(),
        settled(),
        received(created_frame(peer(1))),
        received(created_frame(peer(3))),
        received(created_frame(peer(4))),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), resumed(Some(message_id(1)))],
        entries: vec![confirmed_key(&peer(1)), confirmed_key(&peer(3)), confirmed_key(&peer(4))],
        syncing: true,
        error: None,
        cursor: Some(message_id(1)),
    },
)]
#[case::gap_filled_by_sync_page_closes_gap(
    vec![
        connected_a(),
        settled(),
        received(created_frame(peer(1))),
        received(created_frame(peer(3))),
        received(synced_frame(ROOM_A, vec![peer(2), peer(3)], false)),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), resumed(Some(message_id(1)))],
        entries: vec![confirmed_key(&peer(1)), confirmed_key(&peer(2)), confirmed_key(&peer(3))],
        syncing: false,
        error: None,
        cursor: Some(message_id(3)),
    },
)]
#[case::has_more_requests_follow_up_from_page_tip(
    vec![
        connected_a(),
        received(synced_frame(ROOM_A, chain(ROOM_A, 2), true)),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), resumed(Some(message_id(2)))],
        entries: vec![confirmed_key(&peer(1)), confirmed_key(&peer(2))],
        syncing: true,
        error: None,
        cursor: Some(message_id(2)),
    },
)]
#[case::final_page_ends_paging(
    vec![
        connected_a(),
        received(synced_frame(ROOM_A, chain(ROOM_A, 2), true)),
        received(synced_frame(ROOM_A, vec![peer(3)], false)),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), resumed(Some(message_id(2)))],
        entries: vec![confirmed_key(&peer(1)), confirmed_key(&peer(2)), confirmed_key(&peer(3))],
        syncing: false,
        error: None,
        cursor: Some(message_id(3)),
    },
)]
#[case::latest_page_without_cursor_anchors_at_tip(
    vec![
        connected_a(),
        received(synced_frame(ROOM_A, vec![peer(5), peer(6)], false)),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![confirmed_key(&peer(5)), confirmed_key(&peer(6))],
        syncing: false,
        error: None,
        cursor: Some(message_id(6)),
    },
)]
#[case::stale_page_never_moves_cursor_back(
    vec![
        connected_a(),
        settled(),
        received(created_frame(peer(1))),
        received(created_frame(peer(2))),
        received(created_frame(peer(3))),
        received(synced_frame(ROOM_A, vec![peer(1)], false)),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![confirmed_key(&peer(1)), confirmed_key(&peer(2)), confirmed_key(&peer(3))],
        syncing: false,
        error: None,
        cursor: Some(message_id(3)),
    },
)]
#[case::duplicate_created_is_one_entry(
    vec![connected_a(), settled(), received(created_frame(peer(1))), received(created_frame(peer(1)))],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![confirmed_key(&peer(1))],
        syncing: false,
        error: None,
        cursor: Some(message_id(1)),
    },
)]
#[case::foreign_campus_created_dropped(
    vec![connected_a(), settled(), received(created_frame(foreign(peer(1))))],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::foreign_campus_sync_page_dropped(
    vec![connected_a(), received(synced_frame(ROOM_A, vec![foreign(peer(1))], false))],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::foreign_campus_accepted_keeps_pending(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "kept"),
        received(accepted_frame(foreign(own()))),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "kept")],
        entries: vec![pending_key(NONCE_A)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::peer_message_with_same_nonce_keeps_pending(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "mine"),
        settled(),
        received(created_frame(room_message(ROOM_A, message_id(1), None, ACCOUNT_B, NONCE_A, "theirs"))),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "mine")],
        entries: vec![
            confirmed_key(&room_message(ROOM_A, message_id(1), None, ACCOUNT_B, NONCE_A, "theirs")),
            pending_key(NONCE_A),
        ],
        syncing: false,
        error: None,
        cursor: Some(message_id(1)),
    },
)]
#[case::rejected_becomes_failed_and_is_not_resent(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "nope"),
        rejected_nonce_a(MessageRejection::Forbidden),
        TimelineEvent::Disconnected,
        connected_a(),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "nope"), initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![failed_key(NONCE_A, MessageRejection::Forbidden)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::rejected_unknown_room_is_failed(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "lost"),
        rejected_nonce_a(MessageRejection::UnknownRoom),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "lost")],
        entries: vec![failed_key(NONCE_A, MessageRejection::UnknownRoom)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::rejection_for_unknown_nonce_ignored(
    vec![connected_a(), received(rejected_frame(NONCE_B, MessageRejection::Forbidden))],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::discard_removes_failed(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "bye"),
        rejected_nonce_a(MessageRejection::InvalidBody),
        TimelineEvent::DiscardRequested { client_nonce: NONCE_A },
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "bye")],
        entries: vec![],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::discard_ignores_pending(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "stay"),
        TimelineEvent::DiscardRequested { client_nonce: NONCE_A },
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "stay")],
        entries: vec![pending_key(NONCE_A)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::blank_body_sets_error_and_emits_nothing(
    vec![connected_a(), send_requested(ROOM_A, NONCE_A, "   ")],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![],
        syncing: true,
        error: Some(ApiError::invalid(InputField::MessageBody)),
        cursor: None,
    },
)]
#[case::control_body_sets_error(
    vec![connected_a(), send_requested(ROOM_A, NONCE_A, "a\u{7}")],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![],
        syncing: true,
        error: Some(ApiError::invalid(InputField::MessageBody)),
        cursor: None,
    },
)]
#[case::valid_send_clears_error(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "\u{0}"),
        send_requested(ROOM_A, NONCE_B, "fine"),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_B, "fine")],
        entries: vec![pending_key(NONCE_B)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::unavailable_room_clears_syncing(
    vec![connected_a(), received(unavailable_frame(ROOM_A))],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::unavailable_room_fails_pending_sends(
    vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_A, "orphan"),
        received(unavailable_frame(ROOM_A)),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), send_frame(ROOM_A, NONCE_A, "orphan")],
        entries: vec![failed_key(NONCE_A, MessageRejection::UnknownRoom)],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::disconnect_clears_syncing(
    vec![connected_a(), TimelineEvent::Disconnected],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::reconnect_resumes_from_chain_tip(
    vec![
        connected_a(),
        received(synced_frame(ROOM_A, chain(ROOM_A, 2), false)),
        TimelineEvent::Disconnected,
        connected_a(),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), resumed(Some(message_id(2))), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![confirmed_key(&peer(1)), confirmed_key(&peer(2))],
        syncing: true,
        error: None,
        cursor: Some(message_id(2)),
    },
)]
#[case::created_while_disconnected_defers_gap_sync(
    vec![
        connected_a(),
        settled(),
        TimelineEvent::Disconnected,
        received(created_frame(peer(2))),
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![confirmed_key(&peer(2))],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::rooms_changed_syncs_only_new_rooms(
    vec![
        connected_a(),
        TimelineEvent::RoomsChanged { rooms: vec![ROOM_A, ROOM_B] },
    ],
    Expect {
        frames: vec![initial_sync(), sync_inbox_frame(), sync_shelf_frame(), sync_frame(&[(ROOM_B, None)])],
        entries: vec![],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::rooms_changed_while_offline_emits_nothing(
    vec![TimelineEvent::RoomsChanged { rooms: vec![ROOM_A] }],
    Expect {
        frames: vec![],
        entries: vec![],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::connected_without_rooms_only_syncs_inbox(
    vec![timeline_connected(&[])],
    Expect {
        frames: vec![sync_inbox_frame(), sync_shelf_frame()],
        entries: vec![],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
fn timeline_flows_follow_the_protocol(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Expect,
) {
    let (machine, frames) = timeline_run(events);
    let snapshot = machine.snapshot(Some(ROOM_A));
    assert_eq!(frames, expected.frames);
    assert_eq!(entry_keys(&snapshot), expected.entries);
    assert_eq!(snapshot.syncing, expected.syncing);
    assert_eq!(snapshot.error, expected.error);
    assert_eq!(snapshot.room_id, Some(ROOM_A));
    assert_eq!(machine.cursor(ROOM_A), expected.cursor);
}

#[rstest]
#[case::empty(0, vec![])]
#[case::one_chunk(SYNC_MAX_ROOMS, vec![SYNC_MAX_ROOMS])]
#[case::spills_into_second_chunk(SYNC_MAX_ROOMS + 1, vec![SYNC_MAX_ROOMS, 1])]
#[case::three_chunks(SYNC_MAX_ROOMS * 2 + 5, vec![SYNC_MAX_ROOMS, SYNC_MAX_ROOMS, 5])]
fn connected_chunks_cursors_by_sync_limit(
    #[case] room_count: usize,
    #[case] chunk_sizes: Vec<usize>,
) {
    let rooms = (0..room_count)
        .map(|_| RoomId::generate())
        .collect::<Vec<_>>();
    let (_, frames) = timeline_run(vec![timeline_connected(&rooms)]);
    assert_eq!(frames.last(), Some(&sync_shelf_frame()));
    let sizes = frames
        .iter()
        .filter_map(|frame| match frame {
            ClientFrame::SyncRooms(sync) => Some(sync.cursors.len()),
            ClientFrame::SyncInbox(_) | ClientFrame::SyncShelf(_) => None,
            other => panic!("unexpected frame {other:?}"),
        })
        .collect::<Vec<_>>();
    assert_eq!(sizes, chunk_sizes);
}

#[test]
fn snapshot_orders_confirmed_by_id_then_unconfirmed_by_nonce() {
    let machine = timeline_after(vec![
        connected_a(),
        settled(),
        send_requested(ROOM_A, nonce(9), "late"),
        send_requested(ROOM_A, NONCE_B, "second"),
        send_requested(ROOM_A, NONCE_A, "first"),
        rejected_nonce_a(MessageRejection::Forbidden),
        received(created_frame(peer(2))),
        received(created_frame(peer(1))),
    ]);
    assert_eq!(
        entry_keys(&machine.snapshot(Some(ROOM_A))),
        vec![
            confirmed_key(&peer(1)),
            confirmed_key(&peer(2)),
            failed_key(NONCE_A, MessageRejection::Forbidden),
            pending_key(NONCE_B),
            pending_key(nonce(9)),
        ]
    );
}

#[test]
fn snapshot_describes_authorship() {
    let machine = timeline_after(vec![
        connected_a(),
        settled(),
        send_requested(ROOM_A, NONCE_B, "draft"),
        received(created_frame(peer(1))),
        received(accepted_frame(own_message(ROOM_A, 2, NONCE_A))),
    ]);
    let entries = machine.snapshot(Some(ROOM_A)).entries;
    let described = entries
        .iter()
        .map(|entry| {
            (
                entry.author_id,
                entry.author_display_name.as_str(),
                entry.from_self,
                entry.delivery,
                entry.body.as_str(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        described,
        vec![
            (
                ACCOUNT_B,
                "Grace",
                false,
                DeliveryState::Confirmed,
                "peer 1"
            ),
            (ACCOUNT_A, "Ada", true, DeliveryState::Confirmed, "hi"),
            (ACCOUNT_A, "Ada", true, DeliveryState::Pending, "draft"),
        ]
    );
}

#[rstest]
#[case::no_room(None, vec![])]
#[case::other_room(Some(ROOM_B), vec![])]
#[case::message_room(Some(ROOM_A), vec![confirmed_key(&peer(1))])]
fn snapshot_is_scoped_to_requested_room(
    #[case] room_id: Option<RoomId>,
    #[case] expected: Vec<EntryKey>,
) {
    let machine = timeline_after(vec![
        connected_a(),
        settled(),
        received(created_frame(peer(1))),
    ]);
    let snapshot = machine.snapshot(room_id);
    assert_eq!(snapshot.room_id, room_id);
    assert_eq!(entry_keys(&snapshot), expected);
}

#[test]
fn outbox_survives_reconnect_in_nonce_order() {
    let (_, frames) = timeline_run(vec![
        connected_a(),
        send_requested(ROOM_A, NONCE_B, "b"),
        send_requested(ROOM_B, NONCE_A, "a"),
        TimelineEvent::Disconnected,
        timeline_connected(&[ROOM_A, ROOM_B]),
    ]);
    assert_eq!(
        frames[frames.len() - 5..],
        [
            sync_frame(&[(ROOM_A, None), (ROOM_B, None)]),
            sync_inbox_frame(),
            sync_shelf_frame(),
            send_frame(ROOM_B, NONCE_A, "a"),
            send_frame(ROOM_A, NONCE_B, "b"),
        ]
    );
}

fn after_settled(events: Vec<TimelineEvent>) -> pulse_core::TimelineMachine {
    let mut all = vec![connected_a(), settled()];
    all.extend(events);
    timeline_after(all)
}

fn stream_ids(machine: &pulse_core::TimelineMachine) -> Vec<MessageId> {
    machine
        .snapshot(Some(ROOM_A))
        .entries
        .iter()
        .filter_map(|entry| entry.message_id)
        .collect()
}

#[rstest]
#[case::reply_is_hidden_from_main_stream(
    vec![received(created_frame(peer(1))), received(created_frame(peer_reply(2, 1)))],
    vec![message_id(1)],
)]
#[case::reply_arriving_first_is_still_hidden(
    vec![received(created_frame(peer_reply(2, 1))), received(created_frame(peer(1)))],
    vec![message_id(1)],
)]
#[case::pending_reply_is_hidden_from_main_stream(
    vec![
        received(created_frame(peer(1))),
        fixtures::reply_intent(1, NONCE_A, "pending reply"),
    ],
    vec![message_id(1)],
)]
#[case::top_level_messages_stay_in_id_order(
    vec![
        received(created_frame(peer(1))),
        received(created_frame(peer_reply(2, 1))),
        received(created_frame(peer(3))),
    ],
    vec![message_id(1), message_id(3)],
)]
fn replies_never_appear_in_the_main_stream(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Vec<MessageId>,
) {
    let machine = after_settled(events);
    assert_eq!(stream_ids(&machine), expected);
    assert_eq!(machine.snapshot(Some(ROOM_A)).entries.len(), expected.len());
}

#[rstest]
#[case::first_page_sets_window_start(
    vec![received(synced_frame(ROOM_A, vec![peer(5), peer(6)], false))],
    vec![message_id(5), message_id(6)],
)]
#[case::older_message_after_window_is_hidden(
    vec![
        received(synced_frame(ROOM_A, vec![peer(5), peer(6)], false)),
        received(created_frame(peer(3))),
    ],
    vec![message_id(5), message_id(6)],
)]
#[case::newer_message_after_window_is_shown(
    vec![
        received(synced_frame(ROOM_A, vec![peer(5), peer(6)], false)),
        received(created_frame(peer(7))),
    ],
    vec![message_id(5), message_id(6), message_id(7)],
)]
#[case::later_page_does_not_move_window_start(
    vec![
        received(synced_frame(ROOM_A, vec![peer(5), peer(6)], false)),
        received(synced_frame(ROOM_A, vec![peer(4)], false)),
    ],
    vec![message_id(5), message_id(6)],
)]
fn window_start_is_set_by_the_first_page(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Vec<MessageId>,
) {
    let mut all = vec![connected_a()];
    all.extend(events);
    assert_eq!(stream_ids(&timeline_after(all)), expected);
}

struct Observed {
    reactions: Vec<(String, u32, bool)>,
    topic_title: Option<String>,
    reply_count: Option<u32>,
    entries: usize,
}

fn tally(emoji: &str, count: u32, by_self: bool) -> (String, u32, bool) {
    (emoji.to_owned(), count, by_self)
}

#[rstest]
#[case::newer_activity_replaces(
    vec![
        received(created_frame(peer(1))),
        received(updated_frame(reacted(peer(1), &[(EMOJI_A, &[ACCOUNT_B])], 1))),
    ],
    Observed { reactions: vec![tally(EMOJI_A, 1, false)], topic_title: None, reply_count: None, entries: 1 },
)]
#[case::older_activity_is_ignored(
    vec![
        received(created_frame(peer(1))),
        received(updated_frame(reacted(peer(1), &[(EMOJI_A, &[ACCOUNT_B])], 2))),
        received(updated_frame(reacted(peer(1), &[(EMOJI_B, &[ACCOUNT_B])], 1))),
    ],
    Observed { reactions: vec![tally(EMOJI_A, 1, false)], topic_title: None, reply_count: None, entries: 1 },
)]
#[case::equal_activity_is_ignored(
    vec![
        received(created_frame(peer(1))),
        received(updated_frame(reacted(peer(1), &[(EMOJI_A, &[ACCOUNT_B])], 2))),
        received(updated_frame(reacted(peer(1), &[(EMOJI_B, &[ACCOUNT_B])], 2))),
    ],
    Observed { reactions: vec![tally(EMOJI_A, 1, false)], topic_title: None, reply_count: None, entries: 1 },
)]
#[case::stale_created_echo_does_not_revert(
    vec![
        received(created_frame(peer(1))),
        received(updated_frame(reacted(peer(1), &[(EMOJI_A, &[ACCOUNT_B, ACCOUNT_C])], 4))),
        received(created_frame(peer(1))),
    ],
    Observed { reactions: vec![tally(EMOJI_A, 2, false)], topic_title: None, reply_count: None, entries: 1 },
)]
#[case::reacted_by_self_follows_tally(
    vec![
        received(created_frame(peer(1))),
        received(updated_frame(reacted(peer(1), &[(EMOJI_A, &[ACCOUNT_A, ACCOUNT_B]), (EMOJI_B, &[ACCOUNT_B])], 1))),
    ],
    Observed {
        reactions: vec![tally(EMOJI_A, 2, true), tally(EMOJI_B, 1, false)],
        topic_title: None,
        reply_count: None,
        entries: 1,
    },
)]
#[case::sync_update_replaces_known_message(
    vec![
        received(created_frame(peer(1))),
        received(synced_frame_with(ROOM_A, vec![], vec![reacted(peer(1), &[(EMOJI_A, &[ACCOUNT_B])], 3)], 3, false)),
    ],
    Observed { reactions: vec![tally(EMOJI_A, 1, false)], topic_title: None, reply_count: None, entries: 1 },
)]
#[case::sync_update_for_unknown_message_is_not_inserted(
    vec![received(synced_frame_with(ROOM_A, vec![], vec![reacted(peer(7), &[(EMOJI_A, &[ACCOUNT_B])], 3)], 3, false))],
    Observed { reactions: vec![], topic_title: None, reply_count: None, entries: 0 },
)]
#[case::title_update_shows_preview(
    vec![
        received(created_frame(peer(1))),
        received(updated_frame(titled(peer(1), "Plans", 1))),
    ],
    Observed { reactions: vec![], topic_title: Some("Plans".to_owned()), reply_count: Some(0), entries: 1 },
)]
#[case::latest_reply_breaks_equal_activity(
    vec![
        received(created_frame(peer(1))),
        received(updated_frame(with_replies(peer(1), 2, Some(3)))),
    ],
    Observed { reactions: vec![], topic_title: None, reply_count: Some(2), entries: 1 },
)]
#[case::logged_replies_beyond_root_stats_are_counted(
    vec![
        received(created_frame(with_replies(peer(1), 3, Some(4)))),
        received(created_frame(peer_reply(4, 1))),
        received(created_frame(peer_reply(5, 1))),
    ],
    Observed { reactions: vec![], topic_title: None, reply_count: Some(4), entries: 1 },
)]
fn newer_messages_replace_and_older_are_ignored(
    #[case] events: Vec<TimelineEvent>,
    #[case] expected: Observed,
) {
    let snapshot = after_settled(events).snapshot(Some(ROOM_A));
    assert_eq!(snapshot.entries.len(), expected.entries);
    let entry = snapshot.entries.first();
    let reactions = entry
        .map(|entry| {
            entry
                .reactions
                .iter()
                .map(|summary| {
                    (
                        summary.emoji.clone(),
                        summary.count,
                        summary.reacted_by_self,
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let preview = entry.and_then(|entry| entry.thread.as_ref());
    assert_eq!(reactions, expected.reactions);
    assert_eq!(
        preview.and_then(|thread| thread.topic_title.clone()),
        expected.topic_title
    );
    assert_eq!(
        preview.map(|thread| thread.reply_count),
        expected.reply_count
    );
}
