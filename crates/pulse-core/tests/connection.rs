mod fixtures;

use fixtures::{
    CAMPUS_A, CAMPUS_B, SESSION_A, ScriptedTransport, connect, hello_frame, machine_after, refused,
    rejected, token_a, token_b, welcome, welcome_frame,
};
use futures::executor::block_on;
use pulse_core::{
    BackoffPolicy, ConnectionCommand, ConnectionEvent, ConnectionMachine, TransportError,
    handshake, initial_snapshot,
};
use pulse_protocol::{
    AuthPhase, ClientFrame, ConnectionPhase, HandshakeRejection, Hello, Pong, ProtocolVersion,
    ServerFrame, VersionMismatch,
};
use rstest::rstest;

fn mismatch() -> VersionMismatch {
    VersionMismatch {
        supported: ProtocolVersion::CURRENT,
        received: ProtocolVersion(99),
    }
}

fn hello() -> ConnectionCommand {
    ConnectionCommand::SendHello(Hello {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: CAMPUS_A,
        auth_token: token_a(),
    })
}

#[rstest]
#[case::connect_requested(vec![], connect(), ConnectionPhase::Connecting, None, Some(ConnectionCommand::OpenTransport), 0, false)]
#[case::transport_opened(vec![connect()], ConnectionEvent::TransportOpened, ConnectionPhase::Handshaking, None, Some(hello()), 0, false)]
#[case::failed_while_connecting(
    vec![connect()],
    ConnectionEvent::TransportFailed,
    ConnectionPhase::Backoff, Some(500),
    Some(ConnectionCommand::ScheduleRetry { delay_ms: 500 }),
    1,
    false,
)]
#[case::failed_while_handshaking(
    vec![connect(), ConnectionEvent::TransportOpened],
    ConnectionEvent::TransportFailed,
    ConnectionPhase::Backoff, Some(500),
    Some(ConnectionCommand::ScheduleRetry { delay_ms: 500 }),
    1,
    false,
)]
#[case::second_failure_doubles_delay(
    vec![connect(), ConnectionEvent::TransportFailed, ConnectionEvent::BackoffElapsed],
    ConnectionEvent::TransportFailed,
    ConnectionPhase::Backoff, Some(1000),
    Some(ConnectionCommand::ScheduleRetry { delay_ms: 1000 }),
    2,
    false,
)]
#[case::welcome_matching_campus(vec![connect(), ConnectionEvent::TransportOpened], welcome(CAMPUS_A), ConnectionPhase::Connected, None, None, 0, true)]
#[case::welcome_other_campus(
    vec![connect(), ConnectionEvent::TransportOpened],
    welcome(CAMPUS_B),
    ConnectionPhase::Disconnected, None,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::version_rejected(
    vec![connect(), ConnectionEvent::TransportOpened],
    ConnectionEvent::VersionRejected(mismatch()),
    ConnectionPhase::Disconnected, None,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::handshake_rejected_unauthenticated(
    vec![connect(), ConnectionEvent::TransportOpened],
    ConnectionEvent::HandshakeRejected(rejected(HandshakeRejection::Unauthenticated)),
    ConnectionPhase::Disconnected, None,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::ignored_rejection_when_connected(
    vec![connect(), ConnectionEvent::TransportOpened, welcome(CAMPUS_A)],
    ConnectionEvent::HandshakeRejected(rejected(HandshakeRejection::CampusUnavailable)),
    ConnectionPhase::Connected, None,
    None,
    0,
    true,
)]
#[case::connected_transport_failed(
    vec![connect(), ConnectionEvent::TransportOpened, welcome(CAMPUS_A)],
    ConnectionEvent::TransportFailed,
    ConnectionPhase::Backoff, Some(500),
    Some(ConnectionCommand::ScheduleRetry { delay_ms: 500 }),
    1,
    false,
)]
#[case::backoff_elapsed(
    vec![connect(), ConnectionEvent::TransportFailed],
    ConnectionEvent::BackoffElapsed,
    ConnectionPhase::Connecting, None,
    Some(ConnectionCommand::OpenTransport),
    1,
    false,
)]
#[case::disconnect_while_connecting(vec![connect()], ConnectionEvent::DisconnectRequested, ConnectionPhase::Disconnected, None, Some(ConnectionCommand::CloseTransport), 0, false)]
#[case::disconnect_while_connected(
    vec![connect(), ConnectionEvent::TransportOpened, welcome(CAMPUS_A)],
    ConnectionEvent::DisconnectRequested,
    ConnectionPhase::Disconnected, None,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::disconnect_while_backoff(
    vec![connect(), ConnectionEvent::TransportFailed],
    ConnectionEvent::DisconnectRequested,
    ConnectionPhase::Disconnected, None,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::ignored_opened_when_backoff(
    vec![connect(), ConnectionEvent::TransportFailed],
    ConnectionEvent::TransportOpened,
    ConnectionPhase::Backoff,
    Some(500),
    None,
    1,
    false,
)]
#[case::ignored_disconnect_when_disconnected(vec![], ConnectionEvent::DisconnectRequested, ConnectionPhase::Disconnected, None, None, 0, false)]
#[case::ignored_opened_when_disconnected(vec![], ConnectionEvent::TransportOpened, ConnectionPhase::Disconnected, None, None, 0, false)]
#[case::ignored_connect_when_connecting(vec![connect()], connect(), ConnectionPhase::Connecting, None, None, 0, false)]
#[case::ignored_welcome_when_connecting(vec![connect()], welcome(CAMPUS_A), ConnectionPhase::Connecting, None, None, 0, false)]
#[case::ignored_elapsed_when_connected(
    vec![connect(), ConnectionEvent::TransportOpened, welcome(CAMPUS_A)],
    ConnectionEvent::BackoffElapsed,
    ConnectionPhase::Connected, None,
    None,
    0,
    true,
)]
#[case::ignored_failure_when_disconnected(vec![], ConnectionEvent::TransportFailed, ConnectionPhase::Disconnected, None, None, 0, false)]
fn step_follows_transition_table(
    #[case] setup: Vec<ConnectionEvent>,
    #[case] event: ConnectionEvent,
    #[case] expected_phase: ConnectionPhase,
    #[case] expected_retry: Option<u32>,
    #[case] expected_command: Option<ConnectionCommand>,
    #[case] expected_attempt: u32,
    #[case] has_session: bool,
) {
    let (machine, command) = machine_after(setup).step(event);
    let snapshot = machine.snapshot();
    assert_eq!(*machine.phase(), expected_phase);
    assert_eq!(snapshot.connection, expected_phase);
    assert_eq!(command, expected_command);
    assert_eq!(snapshot.retry_in_ms, expected_retry);
    assert_eq!(snapshot.attempt, expected_attempt);
    assert_eq!(snapshot.session_id.is_some(), has_session);
    if has_session {
        assert_eq!(snapshot.session_id, Some(SESSION_A));
    }
}

#[test]
fn version_rejection_is_recorded_and_cleared_on_next_connect() {
    let rejected = machine_after(vec![
        connect(),
        ConnectionEvent::TransportOpened,
        ConnectionEvent::VersionRejected(mismatch()),
    ]);
    assert_eq!(rejected.snapshot().rejection, Some(mismatch()));
    let (retried, _) = rejected.step(connect());
    assert_eq!(retried.snapshot().rejection, None);
}

#[rstest]
#[case::unauthenticated(HandshakeRejection::Unauthenticated)]
#[case::campus_unavailable(HandshakeRejection::CampusUnavailable)]
fn handshake_rejection_is_recorded_and_cleared_on_next_connect(#[case] reason: HandshakeRejection) {
    let denied = machine_after(vec![
        connect(),
        ConnectionEvent::TransportOpened,
        ConnectionEvent::HandshakeRejected(rejected(reason)),
    ]);
    assert_eq!(denied.snapshot().handshake_rejection, Some(reason));
    let (retried, _) = denied.step(connect());
    assert_eq!(retried.snapshot().handshake_rejection, None);
}

#[test]
fn initial_snapshot_is_signed_out_and_disconnected_at_current_version() {
    let full = initial_snapshot();
    assert_eq!(full.session.auth, AuthPhase::SignedOut);
    assert_eq!(full.session.account, None);
    assert!(full.session.campuses.is_empty());
    assert_eq!(full.session.active_campus_id, None);
    assert!(full.session.rooms.is_empty());
    assert!(!full.session.busy);
    assert_eq!(full.session.error, None);
    let snapshot = full.network;
    assert_eq!(snapshot.handshake_rejection, None);
    assert_eq!(snapshot.connection, ConnectionPhase::Disconnected);
    assert_eq!(snapshot.retry_in_ms, None);
    assert_eq!(snapshot.protocol_version, ProtocolVersion::CURRENT);
    assert_eq!(snapshot.campus_id, None);
    assert_eq!(snapshot.session_id, None);
    assert_eq!(snapshot.attempt, 0);
    assert_eq!(snapshot.rejection, None);
}

#[rstest]
#[case(0, 500)]
#[case(1, 1_000)]
#[case(2, 2_000)]
#[case(5, 16_000)]
#[case(40, 30_000)]
fn backoff_doubles_until_cap(#[case] attempt: u32, #[case] expected: u32) {
    assert_eq!(BackoffPolicy::default().delay_ms(attempt), expected);
}

#[rstest]
#[case::welcome(
    ScriptedTransport::scripted(vec![], vec![welcome_frame(CAMPUS_A)]),
    ConnectionPhase::Connected,
    true,
    0,
    vec![hello_frame(CAMPUS_A, token_a())],
)]
#[case::handshake_rejected(
    ScriptedTransport::scripted(vec![], vec![Ok(ServerFrame::HandshakeRejected(rejected(HandshakeRejection::CampusUnavailable)))]),
    ConnectionPhase::Disconnected,
    false,
    1,
    vec![hello_frame(CAMPUS_A, token_a())],
)]
#[case::version_mismatch(
    ScriptedTransport::scripted(vec![], vec![Ok(ServerFrame::VersionMismatch(mismatch()))]),
    ConnectionPhase::Disconnected,
    false,
    1,
    vec![hello_frame(CAMPUS_A, token_a())],
)]
#[case::open_failure(
    ScriptedTransport::scripted(vec![refused()], vec![]),
    ConnectionPhase::Backoff,
    false,
    0,
    vec![],
)]
#[case::receive_failure(
    ScriptedTransport::scripted(vec![], vec![Err(TransportError::Closed)]),
    ConnectionPhase::Backoff,
    false,
    0,
    vec![hello_frame(CAMPUS_A, token_a())],
)]
#[case::unexpected_frame(
    ScriptedTransport::scripted(vec![], vec![Ok(ServerFrame::Pong(Pong { nonce: 1 }))]),
    ConnectionPhase::Backoff,
    false,
    0,
    vec![hello_frame(CAMPUS_A, token_a())],
)]
#[case::wrong_campus(
    ScriptedTransport::scripted(vec![], vec![welcome_frame(CAMPUS_B)]),
    ConnectionPhase::Disconnected,
    false,
    1,
    vec![hello_frame(CAMPUS_A, token_a())],
)]
fn handshake_scenarios(
    #[case] mut transport: ScriptedTransport,
    #[case] expected_phase: ConnectionPhase,
    #[case] has_session: bool,
    #[case] expected_closes: u32,
    #[case] expected_sent: Vec<ClientFrame>,
) {
    let machine = block_on(handshake(
        ConnectionMachine::new(BackoffPolicy::default()),
        &mut transport,
        CAMPUS_A,
        token_a(),
    ));
    let snapshot = machine.snapshot();
    assert_eq!(snapshot.connection, expected_phase);
    let expected_retry = (expected_phase == ConnectionPhase::Backoff).then_some(500);
    assert_eq!(snapshot.retry_in_ms, expected_retry);
    assert_eq!(snapshot.session_id.is_some(), has_session);
    assert_eq!(transport.closes, expected_closes);
    assert_eq!(transport.sent, expected_sent);
    assert_eq!(snapshot.campus_id, Some(CAMPUS_A));
}

#[test]
fn handshake_with_different_token_restarts_connected_machine() {
    let connected = machine_after(vec![
        connect(),
        ConnectionEvent::TransportOpened,
        welcome(CAMPUS_A),
    ]);
    let mut transport = ScriptedTransport::scripted(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]);
    let restarted = block_on(handshake(connected, &mut transport, CAMPUS_A, token_b()));
    assert_eq!(restarted.snapshot().connection, ConnectionPhase::Connected);
    assert_eq!(transport.closes, 1);
    assert_eq!(transport.sent, vec![hello_frame(CAMPUS_A, token_b())]);
}

#[test]
fn handshake_leaves_connected_machine_untouched() {
    let connected = machine_after(vec![
        connect(),
        ConnectionEvent::TransportOpened,
        welcome(CAMPUS_A),
    ]);
    let mut transport = ScriptedTransport::default();
    let unchanged = block_on(handshake(
        connected.clone(),
        &mut transport,
        CAMPUS_A,
        token_a(),
    ));
    assert_eq!(unchanged, connected);
    assert!(transport.sent.is_empty());
}
