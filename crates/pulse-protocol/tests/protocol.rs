use pulse_protocol::fixtures::{
    ACCOUNT_A, AUTH_TOKEN_A, CAMPUS_A, CAMPUS_B, ROOM_A, SESSION_A, VALID_PASSWORD, auth_token,
};
use pulse_protocol::{
    AccountId, AccountProfile, ApiError, ApiErrorCode, AuthPhase, AuthSession, CampusCreated,
    CampusId, CampusList, CampusSummary, ClientFrame, ClientSnapshot, ConnectionPhase,
    ConnectionSnapshot, CreateCampusRequest, CreateRoomRequest, HandshakeRejected,
    HandshakeRejection, Hello, IdParseError, InputField, Permission, Ping, Pong, ProtocolVersion,
    Role, Room, RoomId, RoomList, ServerFrame, SessionId, SessionSnapshot, SignInRequest,
    SignUpRequest, Validate, VersionMismatch, Welcome, api_path, close_code, normalize_email,
    role_permits, validate_display_name, validate_password, validate_slug,
};
use rstest::rstest;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::fmt::Debug;
use uuid::Uuid;

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

fn campus_text() -> String {
    CAMPUS_A.to_string()
}

fn profile() -> AccountProfile {
    AccountProfile {
        account_id: ACCOUNT_A,
        email: "ada@example.org".to_owned(),
        display_name: "Ada".to_owned(),
    }
}

fn summary() -> CampusSummary {
    CampusSummary {
        campus_id: CAMPUS_A,
        slug: "north-quad".to_owned(),
        display_name: "North Quad".to_owned(),
        role: Role::Owner,
    }
}

fn room(slug: &str) -> Room {
    Room {
        room_id: ROOM_A,
        campus_id: CAMPUS_A,
        slug: slug.to_owned(),
    }
}

fn summary_json() -> Value {
    json!({
        "campus_id": CAMPUS_A.to_string(),
        "slug": "north-quad",
        "display_name": "North Quad",
        "role": "owner",
    })
}

fn room_json(slug: &str) -> Value {
    json!({
        "room_id": ROOM_A.to_string(),
        "campus_id": CAMPUS_A.to_string(),
        "slug": slug,
    })
}

fn profile_json() -> Value {
    json!({
        "account_id": ACCOUNT_A.to_string(),
        "email": "ada@example.org",
        "display_name": "Ada",
    })
}

fn signed_out_session() -> SessionSnapshot {
    SessionSnapshot {
        auth: AuthPhase::SignedOut,
        account: None,
        campuses: Vec::new(),
        active_campus_id: None,
        rooms: Vec::new(),
        busy: false,
        error: None,
    }
}

fn disconnected_network() -> ConnectionSnapshot {
    ConnectionSnapshot {
        protocol_version: ProtocolVersion::CURRENT,
        connection: ConnectionPhase::Disconnected,
        retry_in_ms: None,
        campus_id: None,
        session_id: None,
        attempt: 0,
        rejection: None,
        handshake_rejection: None,
    }
}

#[rstest]
#[case::hello(
    ClientFrame::Hello(Hello {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: CAMPUS_A,
        auth_token: auth_token(AUTH_TOKEN_A),
    }),
    json!({
        "type": "hello",
        "protocol_version": 2,
        "campus_id": campus_text(),
        "auth_token": "fixture-token-a",
    }),
)]
#[case::ping(ClientFrame::Ping(Ping { nonce: 7 }), json!({"type": "ping", "nonce": 7}))]
fn client_frames_match_wire_format(#[case] frame: ClientFrame, #[case] expected: Value) {
    assert_wire(frame, expected);
}

#[rstest]
#[case::welcome(
    ServerFrame::Welcome(Welcome {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: CAMPUS_A,
        session_id: SESSION_A,
        account_id: ACCOUNT_A,
        role: Role::Member,
    }),
    json!({
        "type": "welcome",
        "protocol_version": 2,
        "campus_id": CAMPUS_A.to_string(),
        "session_id": SESSION_A.to_string(),
        "account_id": ACCOUNT_A.to_string(),
        "role": "member",
    }),
)]
#[case::pong(ServerFrame::Pong(Pong { nonce: 9 }), json!({"type": "pong", "nonce": 9}))]
#[case::version_mismatch(
    ServerFrame::VersionMismatch(VersionMismatch {
        supported: ProtocolVersion::CURRENT,
        received: ProtocolVersion(99),
    }),
    json!({"type": "version_mismatch", "supported": 2, "received": 99}),
)]
#[case::handshake_rejected_campus(
    ServerFrame::HandshakeRejected(HandshakeRejected { reason: HandshakeRejection::CampusUnavailable }),
    json!({"type": "handshake_rejected", "reason": "campus_unavailable"}),
)]
#[case::handshake_rejected_auth(
    ServerFrame::HandshakeRejected(HandshakeRejected { reason: HandshakeRejection::Unauthenticated }),
    json!({"type": "handshake_rejected", "reason": "unauthenticated"}),
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
#[case::signed_out(AuthPhase::SignedOut, json!("signed_out"))]
#[case::authenticating(AuthPhase::Authenticating, json!("authenticating"))]
#[case::signed_in(AuthPhase::SignedIn, json!("signed_in"))]
fn auth_phase_tags(#[case] phase: AuthPhase, #[case] expected: Value) {
    assert_wire(phase, expected);
}

#[rstest]
#[case::owner(Role::Owner, json!("owner"))]
#[case::admin(Role::Admin, json!("admin"))]
#[case::member(Role::Member, json!("member"))]
#[case::guest(Role::Guest, json!("guest"))]
fn role_tags(#[case] role: Role, #[case] expected: Value) {
    assert_wire(role, expected);
}

#[rstest]
#[case::view_campus(Permission::ViewCampus, json!("view_campus"))]
#[case::view_rooms(Permission::ViewRooms, json!("view_rooms"))]
#[case::create_room(Permission::CreateRoom, json!("create_room"))]
#[case::manage_members(Permission::ManageMembers, json!("manage_members"))]
#[case::manage_campus(Permission::ManageCampus, json!("manage_campus"))]
fn permission_tags(#[case] permission: Permission, #[case] expected: Value) {
    assert_wire(permission, expected);
}

#[rstest]
#[case::invalid_input(ApiErrorCode::InvalidInput, json!("invalid_input"), 400)]
#[case::unauthenticated(ApiErrorCode::Unauthenticated, json!("unauthenticated"), 401)]
#[case::forbidden(ApiErrorCode::Forbidden, json!("forbidden"), 403)]
#[case::not_found(ApiErrorCode::NotFound, json!("not_found"), 404)]
#[case::conflict(ApiErrorCode::Conflict, json!("conflict"), 409)]
#[case::unavailable(ApiErrorCode::Unavailable, json!("unavailable"), 503)]
#[case::internal(ApiErrorCode::Internal, json!("internal"), 500)]
fn api_error_codes(#[case] code: ApiErrorCode, #[case] wire: Value, #[case] status: u16) {
    assert_wire(code, wire);
    assert_eq!(code.http_status(), status);
    assert_eq!(ApiErrorCode::from_http_status(status), code);
}

#[rstest]
#[case::teapot(418)]
#[case::bad_gateway(502)]
#[case::ok(200)]
fn unknown_statuses_map_to_internal(#[case] status: u16) {
    assert_eq!(
        ApiErrorCode::from_http_status(status),
        ApiErrorCode::Internal
    );
}

#[rstest]
#[case::email(InputField::Email, json!("email"))]
#[case::display_name(InputField::DisplayName, json!("display_name"))]
#[case::password(InputField::Password, json!("password"))]
#[case::slug(InputField::Slug, json!("slug"))]
fn input_field_tags(#[case] field: InputField, #[case] expected: Value) {
    assert_wire(field, expected);
}

#[rstest]
#[case::invalid_slug(
    ApiError::invalid(InputField::Slug),
    json!({"code": "invalid_input", "field": "slug"}),
)]
#[case::bare_forbidden(
    ApiError::new(ApiErrorCode::Forbidden),
    json!({"code": "forbidden", "field": null}),
)]
#[case::conflict_email(
    ApiError::conflict(InputField::Email),
    json!({"code": "conflict", "field": "email"}),
)]
fn api_error_wire_format(#[case] error: ApiError, #[case] expected: Value) {
    assert_wire(error, expected);
}

#[rstest]
#[case::sign_up(
    serde_json::to_value(SignUpRequest {
        email: "ada@example.org".to_owned(),
        display_name: "Ada".to_owned(),
        password: VALID_PASSWORD.to_owned(),
    }).expect("serialize"),
    json!({"email": "ada@example.org", "display_name": "Ada", "password": "correct-horse-battery"}),
)]
#[case::sign_in(
    serde_json::to_value(SignInRequest {
        email: "ada@example.org".to_owned(),
        password: VALID_PASSWORD.to_owned(),
    }).expect("serialize"),
    json!({"email": "ada@example.org", "password": "correct-horse-battery"}),
)]
fn credential_requests_match_wire_format(#[case] actual: Value, #[case] expected: Value) {
    assert_eq!(actual, expected);
}

#[rstest]
#[case::auth_session(
    AuthSession { account: profile(), token: auth_token(AUTH_TOKEN_A) },
    json!({"account": profile_json(), "token": "fixture-token-a"}),
)]
#[case::create_campus_request(
    CreateCampusRequest { slug: "north-quad".to_owned(), display_name: "North Quad".to_owned() },
    json!({"slug": "north-quad", "display_name": "North Quad"}),
)]
#[case::create_room_request(
    CreateRoomRequest { slug: "lab".to_owned() },
    json!({"slug": "lab"}),
)]
#[case::campus_created(
    CampusCreated { campus: summary(), rooms: vec![room("commons"), room("courtyard")] },
    json!({"campus": summary_json(), "rooms": [room_json("commons"), room_json("courtyard")]}),
)]
#[case::campus_list(
    CampusList { campuses: vec![summary()] },
    json!({"campuses": [summary_json()]}),
)]
#[case::room_list(
    RoomList { campus_id: CAMPUS_A, rooms: vec![room("commons")] },
    json!({"campus_id": CAMPUS_A.to_string(), "rooms": [room_json("commons")]}),
)]
fn directory_types_match_wire_format<T>(#[case] value: T, #[case] expected: Value)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    assert_wire(value, expected);
}

#[rstest]
#[case::signed_out(
    ClientSnapshot { network: disconnected_network(), session: signed_out_session() },
    json!({
        "network": {
            "protocol_version": 2,
            "connection": "disconnected",
            "retry_in_ms": null,
            "campus_id": null,
            "session_id": null,
            "attempt": 0,
            "rejection": null,
            "handshake_rejection": null,
        },
        "session": {
            "auth": "signed_out",
            "account": null,
            "campuses": [],
            "active_campus_id": null,
            "rooms": [],
            "busy": false,
            "error": null,
        },
    }),
)]
#[case::signed_in_rejected(
    ClientSnapshot {
        network: ConnectionSnapshot {
            protocol_version: ProtocolVersion::CURRENT,
            connection: ConnectionPhase::Backoff,
            retry_in_ms: Some(1_000),
            campus_id: Some(CAMPUS_A),
            session_id: Some(SESSION_A),
            attempt: 2,
            rejection: Some(VersionMismatch {
                supported: ProtocolVersion::CURRENT,
                received: ProtocolVersion(1),
            }),
            handshake_rejection: Some(HandshakeRejection::CampusUnavailable),
        },
        session: SessionSnapshot {
            auth: AuthPhase::SignedIn,
            account: Some(profile()),
            campuses: vec![summary()],
            active_campus_id: Some(CAMPUS_A),
            rooms: vec![room("commons")],
            busy: true,
            error: Some(ApiError::invalid(InputField::Slug)),
        },
    },
    json!({
        "network": {
            "protocol_version": 2,
            "connection": "backoff",
            "retry_in_ms": 1000,
            "campus_id": CAMPUS_A.to_string(),
            "session_id": SESSION_A.to_string(),
            "attempt": 2,
            "rejection": {"supported": 2, "received": 1},
            "handshake_rejection": "campus_unavailable",
        },
        "session": {
            "auth": "signed_in",
            "account": profile_json(),
            "campuses": [summary_json()],
            "active_campus_id": CAMPUS_A.to_string(),
            "rooms": [room_json("commons")],
            "busy": true,
            "error": {"code": "invalid_input", "field": "slug"},
        },
    }),
)]
fn client_snapshot_matches_wire_format(#[case] snapshot: ClientSnapshot, #[case] expected: Value) {
    assert_wire(snapshot, expected);
}

#[rstest]
#[case::owner_view_campus(Role::Owner, Permission::ViewCampus, true)]
#[case::owner_view_rooms(Role::Owner, Permission::ViewRooms, true)]
#[case::owner_create_room(Role::Owner, Permission::CreateRoom, true)]
#[case::owner_manage_members(Role::Owner, Permission::ManageMembers, true)]
#[case::owner_manage_campus(Role::Owner, Permission::ManageCampus, true)]
#[case::admin_view_campus(Role::Admin, Permission::ViewCampus, true)]
#[case::admin_view_rooms(Role::Admin, Permission::ViewRooms, true)]
#[case::admin_create_room(Role::Admin, Permission::CreateRoom, true)]
#[case::admin_manage_members(Role::Admin, Permission::ManageMembers, true)]
#[case::admin_manage_campus(Role::Admin, Permission::ManageCampus, false)]
#[case::member_view_campus(Role::Member, Permission::ViewCampus, true)]
#[case::member_view_rooms(Role::Member, Permission::ViewRooms, true)]
#[case::member_create_room(Role::Member, Permission::CreateRoom, true)]
#[case::member_manage_members(Role::Member, Permission::ManageMembers, false)]
#[case::member_manage_campus(Role::Member, Permission::ManageCampus, false)]
#[case::guest_view_campus(Role::Guest, Permission::ViewCampus, true)]
#[case::guest_view_rooms(Role::Guest, Permission::ViewRooms, true)]
#[case::guest_create_room(Role::Guest, Permission::CreateRoom, false)]
#[case::guest_manage_members(Role::Guest, Permission::ManageMembers, false)]
#[case::guest_manage_campus(Role::Guest, Permission::ManageCampus, false)]
fn role_permission_matrix(
    #[case] role: Role,
    #[case] permission: Permission,
    #[case] expected: bool,
) {
    assert_eq!(role_permits(role, permission), expected);
}

#[test]
fn role_and_permission_lists_are_complete() {
    assert_eq!(Role::ALL.len(), 4);
    assert_eq!(Permission::ALL.len(), 5);
}

#[rstest]
#[case("owner", Some(Role::Owner))]
#[case("admin", Some(Role::Admin))]
#[case("member", Some(Role::Member))]
#[case("guest", Some(Role::Guest))]
#[case("Owner", None)]
#[case("", None)]
fn roles_parse_from_storage_text(#[case] text: &str, #[case] expected: Option<Role>) {
    assert_eq!(text.parse::<Role>().ok(), expected);
    if let Some(role) = expected {
        assert_eq!(role.as_str(), text);
    }
}

#[rstest]
#[case::trims_and_lowercases("  Ada@Example.ORG ", Ok("ada@example.org"))]
#[case::plain("ada@example.org", Ok("ada@example.org"))]
#[case::missing_at("ada.example.org", Err(InputField::Email))]
#[case::two_ats("a@b@example.org", Err(InputField::Email))]
#[case::empty_local("@example.org", Err(InputField::Email))]
#[case::empty_domain("ada@", Err(InputField::Email))]
#[case::domain_without_dot("ada@example", Err(InputField::Email))]
#[case::domain_leading_dot("ada@.org", Err(InputField::Email))]
#[case::domain_trailing_dot("ada@example.", Err(InputField::Email))]
#[case::inner_whitespace("a da@example.org", Err(InputField::Email))]
#[case::empty("   ", Err(InputField::Email))]
fn email_normalization(#[case] raw: &str, #[case] expected: Result<&str, InputField>) {
    assert_eq!(normalize_email(raw), expected.map(str::to_owned));
}

#[test]
fn email_length_boundary() {
    let at_limit = format!("{}@example.org", "a".repeat(254 - "@example.org".len()));
    let over_limit = format!("{}@example.org", "a".repeat(255 - "@example.org".len()));
    assert!(normalize_email(&at_limit).is_ok());
    assert_eq!(normalize_email(&over_limit), Err(InputField::Email));
}

#[rstest]
#[case::single_char("A", true)]
#[case::padded("  Ada  ", true)]
#[case::at_limit(&"a".repeat(64), true)]
#[case::over_limit(&"a".repeat(65), false)]
#[case::empty("", false)]
#[case::blank("   ", false)]
#[case::control("Ada\nLovelace", false)]
#[case::multibyte_at_limit(&"é".repeat(64), true)]
fn display_name_rules(#[case] raw: &str, #[case] valid: bool) {
    assert_eq!(validate_display_name(raw).is_ok(), valid);
}

#[rstest]
#[case::below_min(&"p".repeat(11), false)]
#[case::at_min(&"p".repeat(12), true)]
#[case::at_max(&"p".repeat(128), true)]
#[case::above_max(&"p".repeat(129), false)]
#[case::multibyte_at_min(&"é".repeat(12), true)]
fn password_rules(#[case] raw: &str, #[case] valid: bool) {
    assert_eq!(validate_password(raw).is_ok(), valid);
}

#[rstest]
#[case::below_min("ab", false)]
#[case::at_min("abc", true)]
#[case::at_max(&"a".repeat(32), true)]
#[case::above_max(&"a".repeat(33), false)]
#[case::hyphen_inside("north-quad", true)]
#[case::digits("room-42", true)]
#[case::leading_hyphen("-abc", false)]
#[case::trailing_hyphen("abc-", false)]
#[case::uppercase("Abc", false)]
#[case::underscore("a_b", false)]
#[case::space("a b", false)]
#[case::empty("", false)]
fn slug_rules(#[case] raw: &str, #[case] valid: bool) {
    assert_eq!(validate_slug(raw).is_ok(), valid);
}

fn sign_up(email: &str, display_name: &str, password: &str) -> SignUpRequest {
    SignUpRequest {
        email: email.to_owned(),
        display_name: display_name.to_owned(),
        password: password.to_owned(),
    }
}

#[rstest]
#[case::valid(sign_up("ada@example.org", "Ada", VALID_PASSWORD), None)]
#[case::email_first(sign_up("bad", "", "short"), Some(InputField::Email))]
#[case::display_name_second(sign_up("ada@example.org", "", "short"), Some(InputField::DisplayName))]
#[case::password_last(sign_up("ada@example.org", "Ada", "short"), Some(InputField::Password))]
fn sign_up_validation_order(#[case] request: SignUpRequest, #[case] field: Option<InputField>) {
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
    assert_eq!(
        request.validate().err().map(|error| error.code),
        field.map(|_| ApiErrorCode::InvalidInput)
    );
}

#[rstest]
#[case::valid("ada@example.org", "x", None)]
#[case::email_first("bad", "", Some(InputField::Email))]
#[case::empty_password("ada@example.org", "", Some(InputField::Password))]
fn sign_in_validation(
    #[case] email: &str,
    #[case] password: &str,
    #[case] field: Option<InputField>,
) {
    let request = SignInRequest {
        email: email.to_owned(),
        password: password.to_owned(),
    };
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
}

#[rstest]
#[case::valid("north-quad", "North Quad", None)]
#[case::slug_first("-", "", Some(InputField::Slug))]
#[case::display_name_second("north-quad", "", Some(InputField::DisplayName))]
fn create_campus_validation(
    #[case] slug: &str,
    #[case] display_name: &str,
    #[case] field: Option<InputField>,
) {
    let request = CreateCampusRequest {
        slug: slug.to_owned(),
        display_name: display_name.to_owned(),
    };
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
}

#[rstest]
#[case::valid("lab", None)]
#[case::invalid("Lab", Some(InputField::Slug))]
fn create_room_validation(#[case] slug: &str, #[case] field: Option<InputField>) {
    let request = CreateRoomRequest {
        slug: slug.to_owned(),
    };
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
}

#[rstest]
#[case::token(format!("{:?}", auth_token(AUTH_TOKEN_A)), "AuthToken(redacted)", &[AUTH_TOKEN_A])]
#[case::sign_up(
    format!("{:?}", sign_up("ada@example.org", "Ada", VALID_PASSWORD)),
    "redacted",
    &[VALID_PASSWORD, "ada@example.org"],
)]
#[case::sign_in(
    format!("{:?}", SignInRequest { email: "ada@example.org".to_owned(), password: VALID_PASSWORD.to_owned() }),
    "redacted",
    &[VALID_PASSWORD, "ada@example.org"],
)]
fn secrets_are_redacted_in_debug(
    #[case] rendered: String,
    #[case] expected_fragment: &str,
    #[case] forbidden: &[&str],
) {
    assert!(rendered.contains(expected_fragment));
    forbidden
        .iter()
        .for_each(|secret| assert!(!rendered.contains(secret)));
}

#[test]
fn api_paths_are_stable() {
    assert_eq!(api_path::ACCOUNTS, "/v1/accounts");
    assert_eq!(api_path::AUTH_TOKENS, "/v1/auth-tokens");
    assert_eq!(api_path::CURRENT_AUTH_TOKEN, "/v1/auth-tokens/current");
    assert_eq!(api_path::CAMPUSES, "/v1/campuses");
    assert_eq!(
        api_path::CAMPUS_ROOMS_ROUTE,
        "/v1/campuses/{campus_id}/rooms"
    );
    assert_eq!(api_path::WEBSOCKET, "/ws");
    assert_eq!(
        api_path::campus_rooms(CAMPUS_B),
        format!("/v1/campuses/{CAMPUS_B}/rooms")
    );
}

#[test]
fn close_codes_are_stable() {
    assert_eq!(close_code::PROTOCOL_ERROR, 1002);
    assert_eq!(close_code::INVALID_PAYLOAD, 1007);
    assert_eq!(close_code::POLICY_VIOLATION, 1008);
    assert_eq!(close_code::INTERNAL_ERROR, 1011);
}

#[test]
fn generated_ids_are_version_seven() {
    assert_eq!(CampusId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(SessionId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(AccountId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(RoomId::generate().as_uuid().get_version_num(), 7);
}

#[rstest]
#[case(CAMPUS_A.to_string())]
#[case(SESSION_A.to_string())]
fn ids_round_trip_through_display(#[case] text: String) {
    let id: CampusId = text.parse().expect("parse");
    assert_eq!(id.to_string(), text);
    assert_eq!(id.as_uuid(), Uuid::parse_str(&text).expect("uuid"));
}

#[rstest]
#[case("")]
#[case("not-a-uuid")]
#[case("0193a5c0-0000-7000-8000-00000000000")]
fn invalid_ids_are_rejected(#[case] text: &str) {
    assert!(matches!(text.parse::<CampusId>(), Err(IdParseError)));
}
