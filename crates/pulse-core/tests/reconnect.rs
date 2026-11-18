mod fixtures;

use fixtures::{
    CAMPUS_A, CAMPUS_B, SESSION_A, ScriptedTransport, campus, hello_frame, refused, session,
    welcome_for, welcome_frame,
};
use futures::executor::block_on;
use pulse_core::{BackoffPolicy, ConnectionEvent, ConnectionMachine, TransportError, handshake};
use pulse_protocol::{ConnectionPhase, ServerFrame};
use rstest::rstest;

type Opens = Vec<Result<(), TransportError>>;
type Incoming = Vec<Result<ServerFrame, TransportError>>;

enum Step {
    Run { opens: Opens, incoming: Incoming },
    Drop(ConnectionEvent),
}

struct Expected {
    phase: ConnectionPhase,
    retry_in_ms: Option<u32>,
    attempt: u32,
    has_session: bool,
}

fn run(opens: Opens, incoming: Incoming) -> Step {
    Step::Run { opens, incoming }
}

fn backoff(retry_in_ms: u32, attempt: u32) -> Expected {
    Expected {
        phase: ConnectionPhase::Backoff,
        retry_in_ms: Some(retry_in_ms),
        attempt,
        has_session: false,
    }
}

fn connected() -> Expected {
    Expected {
        phase: ConnectionPhase::Connected,
        retry_in_ms: None,
        attempt: 0,
        has_session: true,
    }
}

fn disconnected(attempt: u32) -> Expected {
    Expected {
        phase: ConnectionPhase::Disconnected,
        retry_in_ms: None,
        attempt,
        has_session: false,
    }
}

fn fresh() -> ConnectionMachine {
    ConnectionMachine::new(BackoffPolicy::default())
}

enum Start {
    Backoff,
    Connecting,
    Handshaking,
    Connected,
}

fn started(start: Start) -> ConnectionMachine {
    let connect = ConnectionEvent::ConnectRequested {
        campus_id: campus(CAMPUS_A),
    };
    let events = match start {
        Start::Backoff => vec![connect, ConnectionEvent::TransportFailed],
        Start::Connecting => vec![connect],
        Start::Handshaking => vec![connect, ConnectionEvent::TransportOpened],
        Start::Connected => vec![
            connect,
            ConnectionEvent::TransportOpened,
            ConnectionEvent::WelcomeReceived(welcome_for(CAMPUS_A)),
        ],
    };
    events
        .into_iter()
        .fold(fresh(), |machine, event| machine.step(event).0)
}

#[rstest]
#[case::delay_doubles_to_cap_then_plateaus(vec![
    (run(vec![refused()], vec![]), backoff(500, 1)),
    (run(vec![refused()], vec![]), backoff(1_000, 2)),
    (run(vec![refused()], vec![]), backoff(2_000, 3)),
    (run(vec![refused()], vec![]), backoff(4_000, 4)),
    (run(vec![refused()], vec![]), backoff(8_000, 5)),
    (run(vec![refused()], vec![]), backoff(16_000, 6)),
    (run(vec![refused()], vec![]), backoff(30_000, 7)),
    (run(vec![refused()], vec![]), backoff(30_000, 8)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
])]
#[case::receive_failure_then_success(vec![
    (run(vec![Ok(())], vec![Err(TransportError::Closed)]), backoff(500, 1)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
])]
#[case::drop_clears_session_then_reconnects(vec![
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
    (Step::Drop(ConnectionEvent::TransportFailed), backoff(500, 1)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
])]
#[case::drop_then_failed_reconnect_escalates(vec![
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
    (Step::Drop(ConnectionEvent::TransportFailed), backoff(500, 1)),
    (run(vec![refused()], vec![]), backoff(1_000, 2)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
])]
#[case::requested_disconnect_then_connect(vec![
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
    (Step::Drop(ConnectionEvent::DisconnectRequested), disconnected(0)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
])]
#[case::wrong_campus_welcome_never_connects(vec![
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_B)]), disconnected(0)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
])]
#[case::wrong_campus_welcome_after_backoff(vec![
    (run(vec![refused()], vec![]), backoff(500, 1)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_B)]), disconnected(1)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
])]
#[case::wrong_campus_welcome_after_drop(vec![
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
    (Step::Drop(ConnectionEvent::TransportFailed), backoff(500, 1)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_B)]), disconnected(1)),
])]
fn reconnect_sequences(#[case] script: Vec<(Step, Expected)>) {
    let mut machine = fresh();
    for (index, (step, expected)) in script.into_iter().enumerate() {
        machine = match step {
            Step::Run { opens, incoming } => {
                let mut transport = ScriptedTransport::scripted(opens, incoming);
                block_on(handshake(machine, &mut transport, campus(CAMPUS_A)))
            }
            Step::Drop(event) => machine.step(event).0,
        };
        let snapshot = machine.snapshot();
        assert_eq!(snapshot.connection, expected.phase, "phase at step {index}");
        assert_eq!(
            snapshot.retry_in_ms, expected.retry_in_ms,
            "retry at step {index}"
        );
        assert_eq!(
            snapshot.attempt, expected.attempt,
            "attempt at step {index}"
        );
        assert_eq!(
            snapshot.session_id.is_some(),
            expected.has_session,
            "session at step {index}"
        );
        if expected.has_session {
            assert_eq!(snapshot.session_id, Some(session(SESSION_A)));
        }
    }
}

#[rstest]
#[case::backoff_same_campus_resumes(
    Start::Backoff,
    CAMPUS_A,
    CAMPUS_A,
    ConnectionPhase::Connected,
    true,
    0,
    0
)]
#[case::backoff_other_campus_restarts(
    Start::Backoff,
    CAMPUS_B,
    CAMPUS_B,
    ConnectionPhase::Connected,
    true,
    0,
    1
)]
#[case::backoff_other_campus_rejects_stale_welcome(
    Start::Backoff,
    CAMPUS_B,
    CAMPUS_A,
    ConnectionPhase::Disconnected,
    false,
    0,
    2
)]
#[case::connecting_other_campus_restarts(
    Start::Connecting,
    CAMPUS_B,
    CAMPUS_B,
    ConnectionPhase::Connected,
    true,
    0,
    1
)]
#[case::handshaking_other_campus_restarts(
    Start::Handshaking,
    CAMPUS_B,
    CAMPUS_B,
    ConnectionPhase::Connected,
    true,
    0,
    1
)]
#[case::connected_other_campus_restarts(
    Start::Connected,
    CAMPUS_B,
    CAMPUS_B,
    ConnectionPhase::Connected,
    true,
    0,
    1
)]
#[case::connected_other_campus_rejects_stale_welcome(
    Start::Connected,
    CAMPUS_B,
    CAMPUS_A,
    ConnectionPhase::Disconnected,
    false,
    0,
    2
)]
fn resume_targets_argument_campus(
    #[case] start: Start,
    #[case] argument: &str,
    #[case] welcome_campus: &str,
    #[case] expected_phase: ConnectionPhase,
    #[case] has_session: bool,
    #[case] expected_attempt: u32,
    #[case] expected_closes: u32,
) {
    let mut transport =
        ScriptedTransport::scripted(vec![Ok(())], vec![welcome_frame(welcome_campus)]);
    let resumed = block_on(handshake(started(start), &mut transport, campus(argument)));
    let snapshot = resumed.snapshot();
    assert_eq!(snapshot.connection, expected_phase);
    assert_eq!(snapshot.retry_in_ms, None);
    assert_eq!(snapshot.campus_id, Some(campus(argument)));
    assert_eq!(snapshot.session_id.is_some(), has_session);
    assert_eq!(snapshot.attempt, expected_attempt);
    assert_eq!(transport.sent, vec![hello_frame(argument)]);
    assert_eq!(transport.closes, expected_closes);
}
