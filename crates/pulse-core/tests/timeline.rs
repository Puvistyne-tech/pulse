mod fixtures;

use fixtures::{
    ACCOUNT_A, ACCOUNT_B, CAMPUS_B, EntryKey, NONCE_A, NONCE_B, ROOM_A, ROOM_B, accepted_frame,
    chain, chain_message, confirmed_key, created_frame, entry_keys, failed_key, message_id, nonce,
    own_message, pending_key, received, rejected_frame, room_message, send_frame, send_requested,
    sync_frame, synced_frame, timeline_after, timeline_connected, timeline_run, unavailable_frame,
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "hi")],
        entries: vec![pending_key(NONCE_A)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::queued_while_offline_then_sent_on_connect(
    vec![send_requested(ROOM_A, NONCE_A, "hello"), connected_a()],
    Expect {
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "hello")],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "hi")],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "hi!")],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "hi?")],
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
            initial_sync(),
            send_frame(ROOM_A, NONCE_A, "again"),
            initial_sync(),
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
        frames: vec![initial_sync(), sync_frame(&[(ROOM_A, None)])],
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
        frames: vec![initial_sync(), sync_frame(&[(ROOM_A, Some(message_id(1)))])],
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
        frames: vec![initial_sync(), sync_frame(&[(ROOM_A, Some(message_id(1)))])],
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
        frames: vec![initial_sync(), sync_frame(&[(ROOM_A, Some(message_id(2)))])],
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
        frames: vec![initial_sync(), sync_frame(&[(ROOM_A, Some(message_id(2)))])],
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
        frames: vec![initial_sync()],
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
        frames: vec![initial_sync()],
        entries: vec![confirmed_key(&peer(1)), confirmed_key(&peer(2)), confirmed_key(&peer(3))],
        syncing: false,
        error: None,
        cursor: Some(message_id(3)),
    },
)]
#[case::duplicate_created_is_one_entry(
    vec![connected_a(), settled(), received(created_frame(peer(1))), received(created_frame(peer(1)))],
    Expect {
        frames: vec![initial_sync()],
        entries: vec![confirmed_key(&peer(1))],
        syncing: false,
        error: None,
        cursor: Some(message_id(1)),
    },
)]
#[case::foreign_campus_created_dropped(
    vec![connected_a(), settled(), received(created_frame(foreign(peer(1))))],
    Expect {
        frames: vec![initial_sync()],
        entries: vec![],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::foreign_campus_sync_page_dropped(
    vec![connected_a(), received(synced_frame(ROOM_A, vec![foreign(peer(1))], false))],
    Expect {
        frames: vec![initial_sync()],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "kept")],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "mine")],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "nope"), initial_sync()],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "lost")],
        entries: vec![failed_key(NONCE_A, MessageRejection::UnknownRoom)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::rejection_for_unknown_nonce_ignored(
    vec![connected_a(), received(rejected_frame(NONCE_B, MessageRejection::Forbidden))],
    Expect {
        frames: vec![initial_sync()],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "bye")],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "stay")],
        entries: vec![pending_key(NONCE_A)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::blank_body_sets_error_and_emits_nothing(
    vec![connected_a(), send_requested(ROOM_A, NONCE_A, "   ")],
    Expect {
        frames: vec![initial_sync()],
        entries: vec![],
        syncing: true,
        error: Some(ApiError::invalid(InputField::MessageBody)),
        cursor: None,
    },
)]
#[case::control_body_sets_error(
    vec![connected_a(), send_requested(ROOM_A, NONCE_A, "a\u{7}")],
    Expect {
        frames: vec![initial_sync()],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_B, "fine")],
        entries: vec![pending_key(NONCE_B)],
        syncing: true,
        error: None,
        cursor: None,
    },
)]
#[case::unavailable_room_clears_syncing(
    vec![connected_a(), received(unavailable_frame(ROOM_A))],
    Expect {
        frames: vec![initial_sync()],
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
        frames: vec![initial_sync(), send_frame(ROOM_A, NONCE_A, "orphan")],
        entries: vec![failed_key(NONCE_A, MessageRejection::UnknownRoom)],
        syncing: false,
        error: None,
        cursor: None,
    },
)]
#[case::disconnect_clears_syncing(
    vec![connected_a(), TimelineEvent::Disconnected],
    Expect {
        frames: vec![initial_sync()],
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
        frames: vec![initial_sync(), sync_frame(&[(ROOM_A, Some(message_id(2)))])],
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
        frames: vec![initial_sync()],
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
        frames: vec![initial_sync(), sync_frame(&[(ROOM_B, None)])],
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
#[case::connected_without_rooms_emits_nothing(
    vec![timeline_connected(&[])],
    Expect {
        frames: vec![],
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
    let sizes = frames
        .iter()
        .map(|frame| match frame {
            ClientFrame::SyncRooms(sync) => sync.cursors.len(),
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
        frames[frames.len() - 3..],
        [
            sync_frame(&[(ROOM_A, None), (ROOM_B, None)]),
            send_frame(ROOM_B, NONCE_A, "a"),
            send_frame(ROOM_A, NONCE_B, "b"),
        ]
    );
}
