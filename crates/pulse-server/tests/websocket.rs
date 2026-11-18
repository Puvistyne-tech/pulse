mod common;

use common::{
    Observed, Outgoing, campus_for, connect, fresh_state, hello, observe, ping, send, send_message,
    signed_up, spawn_server, spawn_server_with, sync_rooms,
};
use pulse_protocol::fixtures::{
    AUTH_TOKEN_A, AUTH_TOKEN_B, CAMPUS_A, CAMPUS_B, NONCE_A, ROOM_A, auth_token,
};
use pulse_protocol::{
    AuthToken, CampusId, HandshakeRejected, HandshakeRejection, Pong, ProtocolVersion, Role,
    ServerFrame, VersionMismatch, close_code,
};
use rstest::rstest;

#[allow(clippy::large_enum_variant)]
enum Expect {
    Welcome(Role),
    Exact(Observed),
}

fn pong(nonce: u32) -> Expect {
    Expect::Exact(Observed::Frame(ServerFrame::Pong(Pong { nonce })))
}

fn close(code: u16) -> Expect {
    Expect::Exact(Observed::Close(code))
}

fn rejected(reason: HandshakeRejection) -> Expect {
    Expect::Exact(Observed::Frame(ServerFrame::HandshakeRejected(
        HandshakeRejected { reason },
    )))
}

fn check(actual: &Observed, wanted: &Expect, campus_id: CampusId) {
    match wanted {
        Expect::Welcome(role) => {
            let Observed::Frame(ServerFrame::Welcome(welcome)) = actual else {
                panic!("expected welcome, got {actual:?}");
            };
            assert_eq!(welcome.protocol_version, ProtocolVersion::CURRENT);
            assert_eq!(welcome.campus_id, campus_id);
            assert_eq!(welcome.role, *role);
        }
        Expect::Exact(exact) => assert_eq!(actual, exact),
    }
}

#[rstest]
#[case::version_mismatch(
    vec![hello(CAMPUS_A, 99, &auth_token(AUTH_TOKEN_A))],
    vec![
        Expect::Exact(Observed::Frame(ServerFrame::VersionMismatch(VersionMismatch {
            supported: ProtocolVersion::CURRENT,
            received: ProtocolVersion(99),
        }))),
        close(close_code::PROTOCOL_ERROR),
    ],
)]
#[case::ping_before_hello(vec![ping(1)], vec![close(close_code::POLICY_VIOLATION)])]
#[case::send_message_before_hello(
    vec![send_message(ROOM_A, NONCE_A, "early")],
    vec![close(close_code::POLICY_VIOLATION)],
)]
#[case::sync_rooms_before_hello(
    vec![sync_rooms(&[(ROOM_A, None)])],
    vec![close(close_code::POLICY_VIOLATION)],
)]
#[case::unparseable_text(
    vec![Outgoing::Raw("not json".to_owned())],
    vec![close(close_code::INVALID_PAYLOAD)],
)]
#[case::hello_missing_auth_token(
    vec![Outgoing::Raw(format!(
        r#"{{"type":"hello","protocol_version":2,"campus_id":"{CAMPUS_A}"}}"#
    ))],
    vec![close(close_code::INVALID_PAYLOAD)],
)]
#[tokio::test]
async fn lazy_websocket_scenarios(#[case] sends: Vec<Outgoing>, #[case] expected: Vec<Expect>) {
    let addr = spawn_server().await;
    let mut socket = connect(addr).await;
    for outgoing in &sends {
        send(&mut socket, outgoing).await;
    }
    for wanted in &expected {
        check(&observe(&mut socket).await, wanted, CAMPUS_A);
    }
}

#[derive(Clone, Copy)]
enum Credentials {
    Member,
    UnknownToken,
    UnknownCampus,
    NonMember,
}

enum Step {
    Hello(Credentials),
    Ping(u32),
}

#[rstest]
#[case::valid_hello(vec![Step::Hello(Credentials::Member)], vec![Expect::Welcome(Role::Owner)])]
#[case::pings_echo_nonces(
    vec![Step::Hello(Credentials::Member), Step::Ping(7), Step::Ping(9)],
    vec![Expect::Welcome(Role::Owner), pong(7), pong(9)],
)]
#[case::unknown_token(
    vec![Step::Hello(Credentials::UnknownToken)],
    vec![rejected(HandshakeRejection::Unauthenticated), close(close_code::POLICY_VIOLATION)],
)]
#[case::unknown_campus(
    vec![Step::Hello(Credentials::UnknownCampus)],
    vec![rejected(HandshakeRejection::CampusUnavailable), close(close_code::POLICY_VIOLATION)],
)]
#[case::non_member_campus(
    vec![Step::Hello(Credentials::NonMember)],
    vec![rejected(HandshakeRejection::CampusUnavailable), close(close_code::POLICY_VIOLATION)],
)]
#[case::second_hello(
    vec![Step::Hello(Credentials::Member), Step::Hello(Credentials::Member)],
    vec![Expect::Welcome(Role::Owner), close(close_code::POLICY_VIOLATION)],
)]
#[tokio::test]
async fn database_websocket_scenarios(#[case] steps: Vec<Step>, #[case] expected: Vec<Expect>) {
    let state = fresh_state().await;
    let owner = signed_up(&state, "owner@example.org").await;
    let outsider = signed_up(&state, "outsider@example.org").await;
    let created = campus_for(&state, &owner.token, "north-quad").await;
    let campus_id = created.campus.campus_id;
    let addr = spawn_server_with(state).await;
    let credentials = |kind: Credentials| -> (CampusId, AuthToken) {
        match kind {
            Credentials::Member => (campus_id, owner.token.clone()),
            Credentials::UnknownToken => (campus_id, auth_token(AUTH_TOKEN_B)),
            Credentials::UnknownCampus => (CAMPUS_B, owner.token.clone()),
            Credentials::NonMember => (campus_id, outsider.token.clone()),
        }
    };
    let mut socket = connect(addr).await;
    for step in &steps {
        let outgoing = match step {
            Step::Hello(kind) => {
                let (campus, token) = credentials(*kind);
                hello(campus, ProtocolVersion::CURRENT.0, &token)
            }
            Step::Ping(nonce) => ping(*nonce),
        };
        send(&mut socket, &outgoing).await;
    }
    for wanted in &expected {
        check(&observe(&mut socket).await, wanted, campus_id);
    }
}
