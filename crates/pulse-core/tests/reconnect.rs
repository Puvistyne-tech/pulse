mod fixtures;

use fixtures::{
    CAMPUS_A, CAMPUS_B, SESSION_A, ScriptedTransport, campus, hello_frame, refused, session,
    welcome_frame,
};
use futures::executor::block_on;
use pulse_core::{BackoffPolicy, ConnectionEvent, ConnectionMachine, TransportError, handshake};
use pulse_protocol::{ConnectionPhase, ServerFrame};
use rstest::rstest;

type Opens = Vec<Result<(), TransportError>>;
type Incoming = Vec<Result<ServerFrame, TransportError>>;

enum Step {
    Run {
        campus: &'static str,
        opens: Opens,
        incoming: Incoming,
    },
    Drop(ConnectionEvent),
}

struct Expected {
    phase: ConnectionPhase,
    attempt: u32,
    has_session: bool,
}

fn run(opens: Opens, incoming: Incoming) -> Step {
    Step::Run {
        campus: CAMPUS_A,
        opens,
        incoming,
    }
}

fn backoff(retry_in_ms: u32, attempt: u32) -> Expected {
    Expected {
        phase: ConnectionPhase::Backoff { retry_in_ms },
        attempt,
        has_session: false,
    }
}

fn connected() -> Expected {
    Expected {
        phase: ConnectionPhase::Connected,
        attempt: 0,
        has_session: true,
    }
}

fn disconnected(attempt: u32) -> Expected {
    Expected {
        phase: ConnectionPhase::Disconnected,
        attempt,
        has_session: false,
    }
}

fn fresh() -> ConnectionMachine {
    ConnectionMachine::new(BackoffPolicy::default())
}

fn backed_off() -> ConnectionMachine {
    let mut transport = ScriptedTransport::with_open_results(vec![refused()]);
    block_on(handshake(fresh(), &mut transport, campus(CAMPUS_A)))
}

#[rstest]
#[case::failures_then_success(vec![
    (run(vec![refused()], vec![]), backoff(500, 1)),
    (run(vec![refused()], vec![]), backoff(1_000, 2)),
    (run(vec![Ok(())], vec![welcome_frame(CAMPUS_A)]), connected()),
])]
#[case::delay_escalates_to_cap(vec![
    (run(vec![refused()], vec![]), backoff(500, 1)),
    (run(vec![refused()], vec![]), backoff(1_000, 2)),
    (run(vec![refused()], vec![]), backoff(2_000, 3)),
    (run(vec![refused()], vec![]), backoff(4_000, 4)),
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
            Step::Run {
                campus: target,
                opens,
                incoming,
            } => {
                let mut transport = ScriptedTransport::scripted(opens, incoming);
                block_on(handshake(machine, &mut transport, campus(target)))
            }
            Step::Drop(event) => machine.step(event).0,
        };
        let snapshot = machine.snapshot();
        assert_eq!(snapshot.connection, expected.phase, "phase at step {index}");
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
#[case::same_campus_argument(CAMPUS_A, CAMPUS_A, ConnectionPhase::Connected, true)]
#[case::other_campus_argument_is_ignored(CAMPUS_B, CAMPUS_A, ConnectionPhase::Connected, true)]
#[case::other_campus_argument_welcome_rejected(
    CAMPUS_B,
    CAMPUS_B,
    ConnectionPhase::Disconnected,
    false
)]
fn resume_from_backoff_uses_stored_campus(
    #[case] argument: &str,
    #[case] welcome_campus: &str,
    #[case] expected_phase: ConnectionPhase,
    #[case] has_session: bool,
) {
    let mut transport =
        ScriptedTransport::scripted(vec![Ok(())], vec![welcome_frame(welcome_campus)]);
    let resumed = block_on(handshake(backed_off(), &mut transport, campus(argument)));
    let snapshot = resumed.snapshot();
    assert_eq!(snapshot.connection, expected_phase);
    assert_eq!(snapshot.campus_id, Some(campus(CAMPUS_A)));
    assert_eq!(snapshot.session_id.is_some(), has_session);
    assert_eq!(transport.sent, vec![hello_frame(CAMPUS_A)]);
}
