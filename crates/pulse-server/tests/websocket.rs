mod common;

use common::{CAMPUS_A, Observed, connect, observe, send_text, spawn_server};
use pulse_protocol::close_code;
use rstest::rstest;
use serde_json::json;

enum Expect {
    Welcome,
    Exact(Observed),
}

fn hello(version: u16) -> String {
    json!({"type": "hello", "protocol_version": version, "campus_id": CAMPUS_A}).to_string()
}

fn ping(nonce: u32) -> String {
    json!({"type": "ping", "nonce": nonce}).to_string()
}

fn pong(nonce: u32) -> Expect {
    Expect::Exact(Observed::Json(json!({"type": "pong", "nonce": nonce})))
}

fn close(code: u16) -> Expect {
    Expect::Exact(Observed::Close(code))
}

fn is_welcome(observed: &Observed) -> bool {
    let Observed::Json(value) = observed else {
        return false;
    };
    value["type"] == "welcome"
        && value["protocol_version"] == 1
        && value["campus_id"] == CAMPUS_A
        && value["session_id"]
            .as_str()
            .is_some_and(|id| !id.is_empty())
}

#[rstest]
#[case::valid_hello(vec![hello(1)], vec![Expect::Welcome])]
#[case::pings_echo_nonces(
    vec![hello(1), ping(7), ping(9)],
    vec![Expect::Welcome, pong(7), pong(9)],
)]
#[case::version_mismatch(
    vec![hello(99)],
    vec![
        Expect::Exact(Observed::Json(
            json!({"type": "version_mismatch", "supported": 1, "received": 99}),
        )),
        close(close_code::PROTOCOL_ERROR),
    ],
)]
#[case::ping_before_hello(vec![ping(1)], vec![close(close_code::POLICY_VIOLATION)])]
#[case::unparseable_text(
    vec!["not json".to_owned()],
    vec![close(close_code::INVALID_PAYLOAD)],
)]
#[case::second_hello(
    vec![hello(1), hello(1)],
    vec![Expect::Welcome, close(close_code::POLICY_VIOLATION)],
)]
#[tokio::test]
async fn websocket_scenarios(#[case] sends: Vec<String>, #[case] expected: Vec<Expect>) {
    let addr = spawn_server().await;
    let mut socket = connect(addr).await;
    for text in &sends {
        send_text(&mut socket, text).await;
    }
    for wanted in &expected {
        let actual = observe(&mut socket).await;
        match wanted {
            Expect::Welcome => assert!(is_welcome(&actual), "unexpected {actual:?}"),
            Expect::Exact(exact) => assert_eq!(&actual, exact),
        }
    }
}
