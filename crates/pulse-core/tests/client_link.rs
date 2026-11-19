mod fixtures;

use fixtures::{
    CAMPUS_A, CAMPUS_B, NONCE_A, NONCE_B, Start, chain, chain_message, client_rooms, connect_burst,
    connect_burst_at, connect_for, connected_client, created_frame, message_id, room_list,
    send_frame, signed_in_client, start_events, sync_frame, synced_frame, token_a, welcome,
    welcome_for,
};
use futures::executor::block_on;
use pulse_core::{
    Client, ConnectionEvent, LinkTarget, SessionEvent, THREAD_VIEW_FOLLOW_MS, ThreadIntent,
};
use pulse_protocol::{
    CampusId, ClientFrame, ClientNonce, ConnectionPhase, InboxSnapshot, Pong, RoomId, ServerFrame,
    SetThreadSubscription, SubscriptionOrigin, SubscriptionState, SyncThread, ThreadSnapshot,
};
use rstest::rstest;

fn expected_target(campus_id: CampusId) -> Option<LinkTarget> {
    Some(LinkTarget {
        campus_id,
        auth_token: token_a(),
    })
}

fn bring_to(client: &mut Client, start: Start) {
    start_events(start).into_iter().for_each(|event| {
        client.connection_step(event);
    });
}

fn connect_to(client: &mut Client, campus_id: CampusId) {
    [
        connect_for(campus_id),
        ConnectionEvent::TransportOpened,
        welcome(campus_id),
    ]
    .into_iter()
    .for_each(|event| {
        client.connection_step(event);
    });
}

fn switch_to_campus_b(client: &mut Client, api: &mut fixtures::ScriptedApi) {
    api.room_lists
        .push_back(Ok(room_list(CAMPUS_B, &["lab", "annex"])));
    block_on(client.apply(api, SessionEvent::CampusSelected(CAMPUS_B)));
}

#[rstest]
#[case::fresh(false, None, None)]
#[case::signed_in(true, None, expected_target(CAMPUS_A))]
#[case::switched(
    true,
    Some(SessionEvent::CampusSelected(CAMPUS_B)),
    expected_target(CAMPUS_B)
)]
#[case::signed_out(true, Some(SessionEvent::SignOutRequested), None)]
fn link_target_tracks_session_state(
    #[case] signed_in: bool,
    #[case] event: Option<SessionEvent>,
    #[case] expected: Option<LinkTarget>,
) {
    let mut client = Client::default();
    let mut api = fixtures::signed_in_api();
    api.room_lists.push_back(Ok(room_list(CAMPUS_B, &["lab"])));
    if signed_in {
        block_on(client.apply(
            &mut api,
            SessionEvent::SignInRequested(fixtures::sign_in_request()),
        ));
    }
    if let Some(event) = event {
        block_on(client.apply(&mut api, event));
    }
    assert_eq!(client.link_target(), expected);
}

#[test]
fn rejected_sign_in_has_no_link_target() {
    let mut api = fixtures::ScriptedApi::default();
    let mut client = Client::default();
    block_on(client.apply(
        &mut api,
        SessionEvent::SignInRequested(fixtures::sign_in_request()),
    ));
    assert_eq!(client.link_target(), None);
    assert_eq!(client.snapshot().timeline.room_id, None);
}

#[rstest]
#[case::nothing_pending(vec![], vec![])]
#[case::one_pending(vec![(NONCE_A, "first")], vec![(NONCE_A, "first")])]
#[case::pending_in_nonce_order(
    vec![(NONCE_B, "second"), (NONCE_A, "first")],
    vec![(NONCE_A, "first"), (NONCE_B, "second")],
)]
fn welcome_queues_sync_then_pending_sends(
    #[case] queued: Vec<(ClientNonce, &str)>,
    #[case] expected_sends: Vec<(ClientNonce, &str)>,
) {
    let (mut client, _) = signed_in_client();
    let rooms = client_rooms(&client);
    queued.iter().for_each(|(nonce, body)| {
        client.send_message(rooms[0], *nonce, (*body).to_owned());
    });
    assert!(client.drain_outbound().is_empty());
    bring_to(&mut client, Start::Connected);
    let mut expected = connect_burst(&rooms);
    expected.extend(
        expected_sends
            .iter()
            .map(|(nonce, body)| send_frame(rooms[0], *nonce, body)),
    );
    assert_eq!(client.drain_outbound(), expected);
    assert!(client.drain_outbound().is_empty());
}

#[test]
fn welcome_frame_connects_and_queues_sync() {
    let (mut client, _) = signed_in_client();
    let rooms = client_rooms(&client);
    bring_to(&mut client, Start::Handshaking);
    client.receive_frame(ServerFrame::Welcome(welcome_for(CAMPUS_A)));
    assert_eq!(
        client.snapshot().network.connection,
        ConnectionPhase::Connected
    );
    assert_eq!(client.drain_outbound(), connect_burst(&rooms));
}

#[rstest]
#[case::transport_failed(ConnectionEvent::TransportFailed, ConnectionEvent::BackoffElapsed)]
#[case::disconnect_requested(
    ConnectionEvent::DisconnectRequested,
    ConnectionEvent::ConnectRequested { campus_id: CAMPUS_A, auth_token: token_a() },
)]
fn reconnect_cursors_equal_chain_tips(
    #[case] drop_event: ConnectionEvent,
    #[case] resume_event: ConnectionEvent,
) {
    let mut client = connected_client();
    let rooms = client_rooms(&client);
    client.drain_outbound();
    client.receive_frame(synced_frame(rooms[0], chain(rooms[0], 3), false));
    client.receive_frame(created_frame(chain_message(rooms[1], 1)));
    client.connection_step(drop_event);
    assert!(client.drain_outbound().is_empty());
    [
        resume_event,
        ConnectionEvent::TransportOpened,
        ConnectionEvent::WelcomeReceived(welcome_for(CAMPUS_A)),
    ]
    .into_iter()
    .for_each(|event| {
        client.connection_step(event);
    });
    assert_eq!(
        client.drain_outbound(),
        connect_burst_at(&[
            (rooms[0], Some(message_id(3)), Some(0)),
            (rooms[1], Some(message_id(1)), None)
        ])
    );
}

#[test]
fn thread_intents_sequence_frames_and_snapshots() {
    let mut client = connected_client();
    let rooms = client_rooms(&client);
    client.drain_outbound();
    client.receive_frame(created_frame(chain_message(rooms[0], 1)));
    let opened = client.thread_intent(ThreadIntent::ThreadOpened {
        room_id: rooms[0],
        root_message_id: message_id(1),
        now_ms: 10,
    });
    assert_eq!(opened.thread.root_message_id, Some(message_id(1)));
    assert!(opened.thread.syncing);
    assert_eq!(client.next_deadline_ms(), Some(10 + THREAD_VIEW_FOLLOW_MS));
    assert_eq!(
        client.drain_outbound(),
        vec![ClientFrame::SyncThread(SyncThread {
            room_id: rooms[0],
            root_message_id: message_id(1),
            after: None
        })]
    );
    let ticked = client.thread_intent(ThreadIntent::ClockAdvanced {
        now_ms: 10 + THREAD_VIEW_FOLLOW_MS,
    });
    assert_eq!(
        ticked.thread.subscription,
        Some(SubscriptionState::Following)
    );
    assert_eq!(client.next_deadline_ms(), None);
    assert_eq!(
        client.drain_outbound(),
        vec![ClientFrame::SetThreadSubscription(SetThreadSubscription {
            room_id: rooms[0],
            root_message_id: message_id(1),
            state: SubscriptionState::Following,
            origin: SubscriptionOrigin::Viewed,
        })]
    );
    let closed = client.thread_intent(ThreadIntent::ThreadClosed);
    assert_eq!(closed.thread, ThreadSnapshot::default());
    assert!(client.drain_outbound().is_empty());
}

#[test]
fn thread_intents_before_sign_in_change_nothing() {
    let mut client = Client::default();
    let snapshot = client.thread_intent(ThreadIntent::InboxToggled);
    assert_eq!(snapshot.inbox, InboxSnapshot::default());
    assert_eq!(client.next_deadline_ms(), None);
    assert!(client.drain_outbound().is_empty());
}

#[rstest]
#[case::fresh(Start::Fresh)]
#[case::connecting(Start::Connecting)]
#[case::handshaking(Start::Handshaking)]
#[case::backoff(Start::Backoff)]
#[case::connected(Start::Connected)]
fn campus_switch_resets_timeline_and_connection(#[case] start: Start) {
    let (mut client, mut api) = signed_in_client();
    let rooms = client_rooms(&client);
    bring_to(&mut client, start);
    client.send_message(rooms[0], NONCE_A, "stranded".to_owned());
    client.receive_frame(created_frame(chain_message(rooms[0], 1)));
    switch_to_campus_b(&mut client, &mut api);
    let snapshot = client.snapshot();
    assert_eq!(snapshot.session.active_campus_id, Some(CAMPUS_B));
    assert_eq!(snapshot.network.connection, ConnectionPhase::Disconnected);
    assert_eq!(snapshot.timeline.room_id, snapshot.session.active_room_id);
    assert!(snapshot.timeline.entries.is_empty());
    assert!(client.drain_outbound().is_empty());
    assert_eq!(client.link_target(), expected_target(CAMPUS_B));
}

#[test]
fn campus_switch_then_reconnect_syncs_only_new_campus_rooms() {
    let (mut client, mut api) = signed_in_client();
    bring_to(&mut client, Start::Connected);
    switch_to_campus_b(&mut client, &mut api);
    let rooms = client_rooms(&client);
    connect_to(&mut client, CAMPUS_B);
    assert_eq!(client.drain_outbound(), connect_burst(&rooms));
}

#[test]
fn sign_out_clears_timeline_and_connection() {
    let mut client = connected_client();
    let rooms = client_rooms(&client);
    client.send_message(rooms[0], NONCE_A, "pending".to_owned());
    block_on(client.apply(
        &mut fixtures::ScriptedApi::default(),
        SessionEvent::SignOutRequested,
    ));
    let snapshot = client.snapshot();
    assert_eq!(snapshot.network.connection, ConnectionPhase::Disconnected);
    assert_eq!(snapshot.timeline.room_id, None);
    assert!(snapshot.timeline.entries.is_empty());
    assert_eq!(client.link_target(), None);
}

#[test]
fn rooms_reloaded_syncs_only_unseen_rooms() {
    let mut client = connected_client();
    client.drain_outbound();
    let known = client_rooms(&client);
    let reloaded = room_list(CAMPUS_A, &["lab"]);
    let fresh = reloaded.rooms[0].room_id;
    block_on(client.apply(
        &mut fixtures::ScriptedApi::default(),
        SessionEvent::RoomsLoaded(reloaded),
    ));
    assert!(!known.contains(&fresh));
    assert_eq!(client.drain_outbound(), vec![sync_frame(&[(fresh, None)])]);
}

#[test]
fn room_selection_switches_timeline() {
    let mut client = connected_client();
    let rooms = client_rooms(&client);
    client.receive_frame(created_frame(chain_message(rooms[0], 1)));
    client.receive_frame(created_frame(chain_message(rooms[1], 1)));
    client.send_message(rooms[1], NONCE_A, "in courtyard".to_owned());
    let mut api = fixtures::ScriptedApi::default();
    let selections = [(rooms[0], 1), (rooms[1], 2), (RoomId::generate(), 2)];
    let observed = selections
        .iter()
        .map(|(room_id, _)| {
            let snapshot = block_on(client.apply(&mut api, SessionEvent::RoomSelected(*room_id)));
            (snapshot.timeline.room_id, snapshot.timeline.entries.len())
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed,
        vec![
            (Some(rooms[0]), 1),
            (Some(rooms[1]), 2),
            (Some(rooms[1]), 2)
        ]
    );
}

#[test]
fn send_to_inactive_room_is_hidden_from_snapshot_but_still_sent() {
    let mut client = connected_client();
    let rooms = client_rooms(&client);
    client.drain_outbound();
    let snapshot = client.send_message(rooms[1], NONCE_A, "elsewhere".to_owned());
    assert_eq!(snapshot.timeline.room_id, Some(rooms[0]));
    assert!(snapshot.timeline.entries.is_empty());
    assert_eq!(
        client.drain_outbound(),
        vec![send_frame(rooms[1], NONCE_A, "elsewhere")]
    );
}

enum Mutation {
    Apply(SessionEvent),
    ConnectionStep(ConnectionEvent),
    ReceivePong,
    ReceiveCreated,
    Send,
    Discard,
    Drain,
    Thread,
}

#[rstest]
#[case::apply(Mutation::Apply(SessionEvent::CampusesRequested))]
#[case::connection_step(Mutation::ConnectionStep(ConnectionEvent::TransportFailed))]
#[case::receive_pong(Mutation::ReceivePong)]
#[case::receive_created(Mutation::ReceiveCreated)]
#[case::send(Mutation::Send)]
#[case::discard(Mutation::Discard)]
#[case::drain(Mutation::Drain)]
#[case::thread(Mutation::Thread)]
fn every_mutator_increases_sequence(#[case] mutation: Mutation) {
    let mut client = connected_client();
    let rooms = client_rooms(&client);
    let mut api = fixtures::ScriptedApi {
        campus_lists: [Ok(fixtures::two_campuses())].into(),
        ..fixtures::ScriptedApi::default()
    };
    let before = client.snapshot().sequence;
    match mutation {
        Mutation::Apply(event) => {
            block_on(client.apply(&mut api, event));
        }
        Mutation::ConnectionStep(event) => {
            client.connection_step(event);
        }
        Mutation::ReceivePong => {
            client.receive_frame(ServerFrame::Pong(Pong { nonce: 1 }));
        }
        Mutation::ReceiveCreated => {
            client.receive_frame(created_frame(chain_message(rooms[0], 1)));
        }
        Mutation::Send => {
            client.send_message(rooms[0], NONCE_A, "tick".to_owned());
        }
        Mutation::Discard => {
            client.discard_message(NONCE_A);
        }
        Mutation::Drain => {
            client.drain_outbound();
        }
        Mutation::Thread => {
            client.thread_intent(ThreadIntent::InboxToggled);
        }
    }
    assert!(client.snapshot().sequence > before);
}
