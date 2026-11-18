mod fixtures;

use fixtures::{
    CAMPUS_A, CAMPUS_B, SESSION_A, ScriptedTransport, campus, hello_frame, refused, session,
    welcome_for,
};
use futures::executor::block_on;
use pulse_core::{
    BackoffPolicy, ConnectionCommand, ConnectionEvent, ConnectionMachine, TransportError,
    handshake, initial_snapshot,
};
use pulse_protocol::{
    ClientFrame, ConnectionPhase, Hello, Pong, ProtocolVersion, ServerFrame, VersionMismatch,
};
use rstest::rstest;

fn connect() -> ConnectionEvent {
    ConnectionEvent::ConnectRequested {
        campus_id: campus(CAMPUS_A),
    }
}

fn welcome(campus_text: &str) -> ConnectionEvent {
    ConnectionEvent::WelcomeReceived(welcome_for(campus_text))
}

fn mismatch() -> VersionMismatch {
    VersionMismatch {
        supported: ProtocolVersion::CURRENT,
        received: ProtocolVersion(99),
    }
}

fn hello() -> ConnectionCommand {
    ConnectionCommand::SendHello(Hello {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: campus(CAMPUS_A),
    })
}

fn machine_after(events: Vec<ConnectionEvent>) -> ConnectionMachine {
    events.into_iter().fold(
        ConnectionMachine::new(BackoffPolicy::default()),
        |machine, event| machine.step(event).0,
    )
}

#[rstest]
#[case::connect_requested(vec![], connect(), ConnectionPhase::Connecting, Some(ConnectionCommand::OpenTransport), 0, false)]
#[case::transport_opened(vec![connect()], ConnectionEvent::TransportOpened, ConnectionPhase::Handshaking, Some(hello()), 0, false)]
#[case::failed_while_connecting(
    vec![connect()],
    ConnectionEvent::TransportFailed,
    ConnectionPhase::Backoff { retry_in_ms: 500 },
    Some(ConnectionCommand::ScheduleRetry { delay_ms: 500 }),
    1,
    false,
)]
#[case::failed_while_handshaking(
    vec![connect(), ConnectionEvent::TransportOpened],
    ConnectionEvent::TransportFailed,
    ConnectionPhase::Backoff { retry_in_ms: 500 },
    Some(ConnectionCommand::ScheduleRetry { delay_ms: 500 }),
    1,
    false,
)]
#[case::second_failure_doubles_delay(
    vec![connect(), ConnectionEvent::TransportFailed, ConnectionEvent::BackoffElapsed],
    ConnectionEvent::TransportFailed,
    ConnectionPhase::Backoff { retry_in_ms: 1000 },
    Some(ConnectionCommand::ScheduleRetry { delay_ms: 1000 }),
    2,
    false,
)]
#[case::welcome_matching_campus(vec![connect(), ConnectionEvent::TransportOpened], welcome(CAMPUS_A), ConnectionPhase::Connected, None, 0, true)]
#[case::welcome_other_campus(
    vec![connect(), ConnectionEvent::TransportOpened],
    welcome(CAMPUS_B),
    ConnectionPhase::Disconnected,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::version_rejected(
    vec![connect(), ConnectionEvent::TransportOpened],
    ConnectionEvent::VersionRejected(mismatch()),
    ConnectionPhase::Disconnected,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::connected_transport_failed(
    vec![connect(), ConnectionEvent::TransportOpened, welcome(CAMPUS_A)],
    ConnectionEvent::TransportFailed,
    ConnectionPhase::Backoff { retry_in_ms: 500 },
    Some(ConnectionCommand::ScheduleRetry { delay_ms: 500 }),
    1,
    false,
)]
#[case::backoff_elapsed(
    vec![connect(), ConnectionEvent::TransportFailed],
    ConnectionEvent::BackoffElapsed,
    ConnectionPhase::Connecting,
    Some(ConnectionCommand::OpenTransport),
    1,
    false,
)]
#[case::disconnect_while_connecting(vec![connect()], ConnectionEvent::DisconnectRequested, ConnectionPhase::Disconnected, Some(ConnectionCommand::CloseTransport), 0, false)]
#[case::disconnect_while_connected(
    vec![connect(), ConnectionEvent::TransportOpened, welcome(CAMPUS_A)],
    ConnectionEvent::DisconnectRequested,
    ConnectionPhase::Disconnected,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::disconnect_while_backoff(
    vec![connect(), ConnectionEvent::TransportFailed],
    ConnectionEvent::DisconnectRequested,
    ConnectionPhase::Disconnected,
    Some(ConnectionCommand::CloseTransport),
    0,
    false,
)]
#[case::ignored_disconnect_when_disconnected(vec![], ConnectionEvent::DisconnectRequested, ConnectionPhase::Disconnected, None, 0, false)]
#[case::ignored_opened_when_disconnected(vec![], ConnectionEvent::TransportOpened, ConnectionPhase::Disconnected, None, 0, false)]
#[case::ignored_connect_when_connecting(vec![connect()], connect(), ConnectionPhase::Connecting, None, 0, false)]
#[case::ignored_welcome_when_connecting(vec![connect()], welcome(CAMPUS_A), ConnectionPhase::Connecting, None, 0, false)]
#[case::ignored_elapsed_when_connected(
    vec![connect(), ConnectionEvent::TransportOpened, welcome(CAMPUS_A)],
    ConnectionEvent::BackoffElapsed,
    ConnectionPhase::Connected,
    None,
    0,
    true,
)]
#[case::ignored_failure_when_disconnected(vec![], ConnectionEvent::TransportFailed, ConnectionPhase::Disconnected, None, 0, false)]
fn step_follows_transition_table(
    #[case] setup: Vec<ConnectionEvent>,
    #[case] event: ConnectionEvent,
    #[case] expected_phase: ConnectionPhase,
    #[case] expected_command: Option<ConnectionCommand>,
    #[case] expected_attempt: u32,
    #[case] has_session: bool,
) {
    let (machine, command) = machine_after(setup).step(event);
    let snapshot = machine.snapshot();
    assert_eq!(*machine.phase(), expected_phase);
    assert_eq!(snapshot.connection, expected_phase);
    assert_eq!(command, expected_command);
    assert_eq!(snapshot.attempt, expected_attempt);
    assert_eq!(snapshot.session_id.is_some(), has_session);
    if has_session {
        assert_eq!(snapshot.session_id, Some(session(SESSION_A)));
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

#[test]
fn initial_snapshot_is_disconnected_at_current_version() {
    let snapshot = initial_snapshot();
    assert_eq!(snapshot.connection, ConnectionPhase::Disconnected);
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
    ScriptedTransport::with_incoming(vec![Ok(ServerFrame::Welcome(welcome_for(CAMPUS_A)))]),
    ConnectionPhase::Connected,
    true,
    0,
    vec![hello_frame(CAMPUS_A)],
)]
#[case::version_mismatch(
    ScriptedTransport::with_incoming(vec![Ok(ServerFrame::VersionMismatch(mismatch()))]),
    ConnectionPhase::Disconnected,
    false,
    1,
    vec![hello_frame(CAMPUS_A)],
)]
#[case::open_failure(
    ScriptedTransport::with_open_results(vec![refused()]),
    ConnectionPhase::Backoff { retry_in_ms: 500 },
    false,
    0,
    vec![],
)]
#[case::receive_failure(
    ScriptedTransport::with_incoming(vec![Err(TransportError::Closed)]),
    ConnectionPhase::Backoff { retry_in_ms: 500 },
    false,
    0,
    vec![hello_frame(CAMPUS_A)],
)]
#[case::unexpected_frame(
    ScriptedTransport::with_incoming(vec![Ok(ServerFrame::Pong(Pong { nonce: 1 }))]),
    ConnectionPhase::Backoff { retry_in_ms: 500 },
    false,
    0,
    vec![hello_frame(CAMPUS_A)],
)]
#[case::wrong_campus(
    ScriptedTransport::with_incoming(vec![Ok(ServerFrame::Welcome(welcome_for(CAMPUS_B)))]),
    ConnectionPhase::Disconnected,
    false,
    1,
    vec![hello_frame(CAMPUS_A)],
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
        campus(CAMPUS_A),
    ));
    let snapshot = machine.snapshot();
    assert_eq!(snapshot.connection, expected_phase);
    assert_eq!(snapshot.session_id.is_some(), has_session);
    assert_eq!(transport.closes, expected_closes);
    assert_eq!(transport.sent, expected_sent);
    assert_eq!(snapshot.campus_id, Some(campus(CAMPUS_A)));
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
        campus(CAMPUS_A),
    ));
    assert_eq!(unchanged, connected);
    assert!(transport.sent.is_empty());
}
