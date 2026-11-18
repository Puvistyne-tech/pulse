use pulse_protocol::{
    CampusId, ClientFrame, ClientSnapshot, ConnectionPhase, Hello, IdParseError, Ping, Pong,
    ProtocolVersion, ServerFrame, SessionId, VersionMismatch, Welcome,
};
use rstest::rstest;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::fmt::Debug;
use uuid::Uuid;

const CAMPUS: &str = "0193a5c0-0000-7000-8000-000000000001";
const SESSION: &str = "0193a5c0-0000-7000-8000-000000000002";

fn campus() -> CampusId {
    CAMPUS.parse().expect("valid campus fixture")
}

fn session() -> SessionId {
    SESSION.parse().expect("valid session fixture")
}

fn assert_wire<T>(value: T, expected: Value)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    assert_eq!(serde_json::to_value(&value).expect("serialize"), expected);
    assert_eq!(
        serde_json::from_value::<T>(expected).expect("deserialize"),
        value
    );
}

#[rstest]
#[case::hello(
    ClientFrame::Hello(Hello { protocol_version: ProtocolVersion::CURRENT, campus_id: campus() }),
    json!({"type": "hello", "protocol_version": 1, "campus_id": CAMPUS}),
)]
#[case::ping(ClientFrame::Ping(Ping { nonce: 7 }), json!({"type": "ping", "nonce": 7}))]
fn client_frames_match_wire_format(#[case] frame: ClientFrame, #[case] expected: Value) {
    assert_wire(frame, expected);
}

#[rstest]
#[case::welcome(
    ServerFrame::Welcome(Welcome {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: campus(),
        session_id: session(),
    }),
    json!({
        "type": "welcome",
        "protocol_version": 1,
        "campus_id": CAMPUS,
        "session_id": SESSION,
    }),
)]
#[case::pong(ServerFrame::Pong(Pong { nonce: 9 }), json!({"type": "pong", "nonce": 9}))]
#[case::version_mismatch(
    ServerFrame::VersionMismatch(VersionMismatch {
        supported: ProtocolVersion::CURRENT,
        received: ProtocolVersion(99),
    }),
    json!({"type": "version_mismatch", "supported": 1, "received": 99}),
)]
fn server_frames_match_wire_format(#[case] frame: ServerFrame, #[case] expected: Value) {
    assert_wire(frame, expected);
}

#[rstest]
#[case::disconnected(ConnectionPhase::Disconnected, json!("disconnected"))]
#[case::connecting(ConnectionPhase::Connecting, json!("connecting"))]
#[case::handshaking(ConnectionPhase::Handshaking, json!("handshaking"))]
#[case::connected(ConnectionPhase::Connected, json!("connected"))]
#[case::backoff(ConnectionPhase::Backoff, json!("backoff"))]
fn connection_phase_tags(#[case] phase: ConnectionPhase, #[case] expected: Value) {
    assert_wire(phase, expected);
}

#[rstest]
#[case::backoff_with_retry(
    ClientSnapshot {
        protocol_version: ProtocolVersion::CURRENT,
        connection: ConnectionPhase::Backoff,
        retry_in_ms: Some(1_000),
        campus_id: Some(campus()),
        session_id: None,
        attempt: 2,
        rejection: None,
    },
    json!({
        "protocol_version": 1,
        "connection": "backoff",
        "retry_in_ms": 1000,
        "campus_id": CAMPUS,
        "session_id": null,
        "attempt": 2,
        "rejection": null,
    }),
)]
#[case::connected_without_retry(
    ClientSnapshot {
        protocol_version: ProtocolVersion::CURRENT,
        connection: ConnectionPhase::Connected,
        retry_in_ms: None,
        campus_id: Some(campus()),
        session_id: Some(session()),
        attempt: 0,
        rejection: None,
    },
    json!({
        "protocol_version": 1,
        "connection": "connected",
        "retry_in_ms": null,
        "campus_id": CAMPUS,
        "session_id": SESSION,
        "attempt": 0,
        "rejection": null,
    }),
)]
fn client_snapshot_matches_wire_format(#[case] snapshot: ClientSnapshot, #[case] expected: Value) {
    assert_wire(snapshot, expected);
}

#[test]
fn generated_ids_are_version_seven() {
    assert_eq!(CampusId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(SessionId::generate().as_uuid().get_version_num(), 7);
}

#[rstest]
#[case(CAMPUS)]
#[case(SESSION)]
fn ids_round_trip_through_display(#[case] text: &str) {
    let id: CampusId = text.parse().expect("parse");
    assert_eq!(id.to_string(), text);
    assert_eq!(id.as_uuid(), Uuid::parse_str(text).expect("uuid"));
}

#[rstest]
#[case("")]
#[case("not-a-uuid")]
#[case("0193a5c0-0000-7000-8000-00000000000")]
fn invalid_ids_are_rejected(#[case] text: &str) {
    assert!(matches!(text.parse::<CampusId>(), Err(IdParseError)));
}
