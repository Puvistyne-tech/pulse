mod common;

use common::{
    CAMPUS_A, Observed, Outgoing, campus, connect, hello, observe, ping, send, spawn_server,
};
use pulse_protocol::{Pong, ProtocolVersion, ServerFrame, VersionMismatch, close_code};
use rstest::rstest;

enum Expect {
    Welcome,
    Exact(Observed),
}

fn pong(nonce: u32) -> Expect {
    Expect::Exact(Observed::Frame(ServerFrame::Pong(Pong { nonce })))
}

fn close(code: u16) -> Expect {
    Expect::Exact(Observed::Close(code))
}

fn is_welcome(observed: &Observed) -> bool {
    let Observed::Frame(ServerFrame::Welcome(welcome)) = observed else {
        return false;
    };
    welcome.protocol_version == ProtocolVersion::CURRENT && welcome.campus_id == campus(CAMPUS_A)
}

#[rstest]
#[case::valid_hello(vec![hello(CAMPUS_A, 1)], vec![Expect::Welcome])]
#[case::pings_echo_nonces(
    vec![hello(CAMPUS_A, 1), ping(7), ping(9)],
    vec![Expect::Welcome, pong(7), pong(9)],
)]
#[case::version_mismatch(
    vec![hello(CAMPUS_A, 99)],
    vec![
        Expect::Exact(Observed::Frame(ServerFrame::VersionMismatch(VersionMismatch {
            supported: ProtocolVersion::CURRENT,
            received: ProtocolVersion(99),
        }))),
        close(close_code::PROTOCOL_ERROR),
    ],
)]
#[case::ping_before_hello(vec![ping(1)], vec![close(close_code::POLICY_VIOLATION)])]
#[case::unparseable_text(
    vec![Outgoing::Raw("not json".to_owned())],
    vec![close(close_code::INVALID_PAYLOAD)],
)]
#[case::second_hello(
    vec![hello(CAMPUS_A, 1), hello(CAMPUS_A, 1)],
    vec![Expect::Welcome, close(close_code::POLICY_VIOLATION)],
)]
#[tokio::test]
async fn websocket_scenarios(#[case] sends: Vec<Outgoing>, #[case] expected: Vec<Expect>) {
    let addr = spawn_server().await;
    let mut socket = connect(addr).await;
    for outgoing in &sends {
        send(&mut socket, outgoing).await;
    }
    for wanted in &expected {
        let actual = observe(&mut socket).await;
        match wanted {
            Expect::Welcome => assert!(is_welcome(&actual), "unexpected {actual:?}"),
            Expect::Exact(exact) => assert_eq!(&actual, exact),
        }
    }
}
