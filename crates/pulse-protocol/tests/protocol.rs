use pulse_protocol::fixtures::{
    ACCOUNT_A, ACCOUNT_B, ACCOUNT_C, AUTH_TOKEN_A, CAMPUS_A, CAMPUS_B, EMOJI_A, EMOJI_B, MESSAGE_A,
    MESSAGE_B, MESSAGE_C, NONCE_A, NONCE_B, ROOM_A, ROOM_B, SESSION_A, VALID_PASSWORD, auth_token,
};
use pulse_protocol::{
    AccountId, AccountProfile, ApiError, ApiErrorCode, AuthPhase, AuthSession, BodySegment,
    CampusCreated, CampusId, CampusList, CampusSummary, ClientFrame, ClientNonce, ClientSnapshot,
    ConnectionPhase, ConnectionSnapshot, CreateCampusRequest, CreateRoomRequest, DeliveryState,
    HandshakeRejected, HandshakeRejection, Hello, INBOX_SYNC_LIMIT, IdParseError, InboxItem,
    InboxItemKind, InboxSnapshot, InboxStep, InboxSynced, InputField, MESSAGE_BODY_MAX_CHARS,
    MESSAGE_MENTIONS_MAX, MarkThreadRead, MentionRef, MessageAccepted, MessageCreated, MessageId,
    MessageRejected, MessageRejection, MessageUpdated, Permission, Ping, Pong, ProtocolVersion,
    REACTION_EMOJI_MAX_CHARS, REACTION_KINDS_MAX, REACTION_PALETTE, React, ReactionAction,
    ReactionRejected, ReactionRejection, ReactionSummary, ReactionTally, ReplyPreview, Role, Room,
    RoomCursor, RoomId, RoomList, RoomMessage, RoomSynced, RoomUnavailable, SYNC_MAX_ROOMS,
    SYNC_PAGE_LIMIT, SegmentKind, SendMessage, ServerFrame, SessionId, SessionSnapshot,
    SetThreadSubscription, SetTopicTitle, SignInRequest, SignUpRequest, SubscriptionOrigin,
    SubscriptionState, SyncInbox, SyncRooms, SyncThread, TOPIC_TITLE_MAX_CHARS, ThreadChanged,
    ThreadPreview, ThreadRejected, ThreadRejection, ThreadSnapshot, ThreadSummary, ThreadSynced,
    TimelineEntry, TimelineSnapshot, Validate, VersionMismatch, Welcome, api_path, body_segments,
    close_code, extract_mentions, mention_token, normalize_display_name, normalize_email,
    normalize_message_body, normalize_reaction_emoji, normalize_topic_title, role_permits,
    validate_display_name, validate_password, validate_slug,
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
        active_room_id: None,
        busy: false,
        error: None,
    }
}

fn empty_timeline() -> TimelineSnapshot {
    TimelineSnapshot {
        room_id: None,
        entries: Vec::new(),
        syncing: false,
        error: None,
        reaction_palette: Vec::new(),
    }
}

fn room_message(message_id: MessageId, previous: Option<MessageId>) -> RoomMessage {
    RoomMessage {
        message_id,
        previous_message_id: previous,
        campus_id: CAMPUS_A,
        room_id: ROOM_A,
        author_id: ACCOUNT_A,
        author_display_name: "Ada".to_owned(),
        client_nonce: NONCE_A,
        body: "hello".to_owned(),
        thread_root_id: None,
        topic_title: None,
        reply_count: 0,
        latest_reply_id: None,
        mentions: Vec::new(),
        reactions: Vec::new(),
        activity_seq: 0,
    }
}

fn room_message_json(message_id: MessageId, previous: Option<MessageId>) -> Value {
    json!({
        "message_id": message_id.to_string(),
        "previous_message_id": previous.map(|id| id.to_string()),
        "campus_id": CAMPUS_A.to_string(),
        "room_id": ROOM_A.to_string(),
        "author_id": ACCOUNT_A.to_string(),
        "author_display_name": "Ada",
        "client_nonce": NONCE_A.to_string(),
        "body": "hello",
        "thread_root_id": null,
        "topic_title": null,
        "reply_count": 0,
        "latest_reply_id": null,
        "mentions": [],
        "reactions": [],
        "activity_seq": 0,
    })
}

fn timeline_entry(delivery: DeliveryState, rejection: Option<MessageRejection>) -> TimelineEntry {
    TimelineEntry {
        client_nonce: NONCE_A,
        message_id: matches!(delivery, DeliveryState::Confirmed).then_some(MESSAGE_A),
        author_id: ACCOUNT_A,
        author_display_name: "Ada".to_owned(),
        author_mention_token: mention_token(ACCOUNT_A),
        from_self: true,
        body: "hello".to_owned(),
        segments: vec![text_segment("hello")],
        thread_root_id: None,
        reactions: Vec::new(),
        thread: None,
        delivery,
        rejection,
    }
}

fn text_segment(text: &str) -> BodySegment {
    BodySegment {
        kind: SegmentKind::Text,
        text: text.to_owned(),
        account_id: None,
    }
}

fn timeline_entry_json(
    message_id: Option<MessageId>,
    delivery: &str,
    rejection: Option<&str>,
) -> Value {
    json!({
        "client_nonce": NONCE_A.to_string(),
        "message_id": message_id.map(|id| id.to_string()),
        "author_id": ACCOUNT_A.to_string(),
        "author_display_name": "Ada",
        "author_mention_token": mention_token(ACCOUNT_A),
        "from_self": true,
        "body": "hello",
        "segments": [{"kind": "text", "text": "hello", "account_id": null}],
        "thread_root_id": null,
        "reactions": [],
        "thread": null,
        "delivery": delivery,
        "rejection": rejection,
    })
}

fn react(action: ReactionAction) -> React {
    React {
        room_id: ROOM_A,
        message_id: MESSAGE_A,
        emoji: EMOJI_A.to_owned(),
        action,
    }
}

fn busy_message() -> RoomMessage {
    RoomMessage {
        topic_title: Some("Plans".to_owned()),
        reply_count: 2,
        latest_reply_id: Some(MESSAGE_C),
        mentions: vec![MentionRef {
            account_id: ACCOUNT_B,
            display_name: "Grace".to_owned(),
        }],
        reactions: vec![
            ReactionTally {
                emoji: EMOJI_A.to_owned(),
                account_ids: vec![ACCOUNT_A, ACCOUNT_B],
            },
            ReactionTally {
                emoji: EMOJI_B.to_owned(),
                account_ids: vec![ACCOUNT_C],
            },
        ],
        activity_seq: 9,
        body: format!("hi {}", mention_token(ACCOUNT_B)),
        ..room_message(MESSAGE_A, None)
    }
}

fn busy_message_json() -> Value {
    let mut value = room_message_json(MESSAGE_A, None);
    value["body"] = json!(format!("hi {}", mention_token(ACCOUNT_B)));
    value["topic_title"] = json!("Plans");
    value["reply_count"] = json!(2);
    value["latest_reply_id"] = json!(MESSAGE_C.to_string());
    value["mentions"] = json!([{"account_id": ACCOUNT_B.to_string(), "display_name": "Grace"}]);
    value["reactions"] = json!([
        {"emoji": EMOJI_A, "account_ids": [ACCOUNT_A.to_string(), ACCOUNT_B.to_string()]},
        {"emoji": EMOJI_B, "account_ids": [ACCOUNT_C.to_string()]},
    ]);
    value["activity_seq"] = json!(9);
    value
}

fn reply_message() -> RoomMessage {
    RoomMessage {
        thread_root_id: Some(MESSAGE_A),
        author_id: ACCOUNT_B,
        author_display_name: "Grace".to_owned(),
        body: "reply".to_owned(),
        ..room_message(MESSAGE_C, Some(MESSAGE_B))
    }
}

fn reply_message_json() -> Value {
    let mut value = room_message_json(MESSAGE_C, Some(MESSAGE_B));
    value["thread_root_id"] = json!(MESSAGE_A.to_string());
    value["author_id"] = json!(ACCOUNT_B.to_string());
    value["author_display_name"] = json!("Grace");
    value["body"] = json!("reply");
    value
}

fn thread_summary() -> ThreadSummary {
    ThreadSummary {
        root: busy_message(),
        latest_reply: Some(reply_message()),
        unread_replies: 2,
        unread_mentions: 1,
        subscription: SubscriptionState::Following,
        read_through: Some(MESSAGE_B),
        revision: 4,
    }
}

fn thread_summary_json() -> Value {
    json!({
        "root": busy_message_json(),
        "latest_reply": reply_message_json(),
        "unread_replies": 2,
        "unread_mentions": 1,
        "subscription": "following",
        "read_through": MESSAGE_B.to_string(),
        "revision": 4,
    })
}

fn reply_preview() -> ReplyPreview {
    ReplyPreview {
        message_id: MESSAGE_C,
        author_id: ACCOUNT_B,
        author_display_name: "Grace".to_owned(),
        preview: "reply".to_owned(),
    }
}

fn reply_preview_json() -> Value {
    json!({
        "message_id": MESSAGE_C.to_string(),
        "author_id": ACCOUNT_B.to_string(),
        "author_display_name": "Grace",
        "preview": "reply",
    })
}

fn inbox_item() -> InboxItem {
    InboxItem {
        kind: InboxItemKind::Thread,
        room_id: ROOM_A,
        root_message_id: MESSAGE_A,
        topic_title: Some("Plans".to_owned()),
        root_author_display_name: "Ada".to_owned(),
        root_preview: "hello".to_owned(),
        latest_reply: Some(reply_preview()),
        reply_count: 3,
        unread_replies: 2,
        unread_mentions: 1,
        subscription: SubscriptionState::Following,
        activity_message_id: MESSAGE_C,
    }
}

fn inbox_item_json() -> Value {
    json!({
        "kind": "thread",
        "room_id": ROOM_A.to_string(),
        "root_message_id": MESSAGE_A.to_string(),
        "topic_title": "Plans",
        "root_author_display_name": "Ada",
        "root_preview": "hello",
        "latest_reply": reply_preview_json(),
        "reply_count": 3,
        "unread_replies": 2,
        "unread_mentions": 1,
        "subscription": "following",
        "activity_message_id": MESSAGE_C.to_string(),
    })
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
        "protocol_version": 3,
        "campus_id": campus_text(),
        "auth_token": "fixture-token-a",
    }),
)]
#[case::ping(ClientFrame::Ping(Ping { nonce: 7 }), json!({"type": "ping", "nonce": 7}))]
#[case::send_message(
    ClientFrame::SendMessage(SendMessage {
        room_id: ROOM_A,
        client_nonce: NONCE_A,
        body: "hello".to_owned(),
        thread_root_id: None,
    }),
    json!({
        "type": "send_message",
        "room_id": ROOM_A.to_string(),
        "client_nonce": NONCE_A.to_string(),
        "body": "hello",
        "thread_root_id": null,
    }),
)]
#[case::send_reply(
    ClientFrame::SendMessage(SendMessage {
        room_id: ROOM_A,
        client_nonce: NONCE_B,
        body: "reply".to_owned(),
        thread_root_id: Some(MESSAGE_A),
    }),
    json!({
        "type": "send_message",
        "room_id": ROOM_A.to_string(),
        "client_nonce": NONCE_B.to_string(),
        "body": "reply",
        "thread_root_id": MESSAGE_A.to_string(),
    }),
)]
#[case::sync_rooms(
    ClientFrame::SyncRooms(SyncRooms {
        cursors: vec![
            RoomCursor { room_id: ROOM_A, after: None, activity_after: None },
            RoomCursor { room_id: ROOM_B, after: Some(MESSAGE_A), activity_after: Some(7) },
        ],
    }),
    json!({
        "type": "sync_rooms",
        "cursors": [
            {"room_id": ROOM_A.to_string(), "after": null, "activity_after": null},
            {"room_id": ROOM_B.to_string(), "after": MESSAGE_A.to_string(), "activity_after": 7},
        ],
    }),
)]
#[case::react(
    ClientFrame::React(react(ReactionAction::Add)),
    json!({
        "type": "react",
        "room_id": ROOM_A.to_string(),
        "message_id": MESSAGE_A.to_string(),
        "emoji": EMOJI_A,
        "action": "add",
    }),
)]
#[case::unreact(
    ClientFrame::React(react(ReactionAction::Remove)),
    json!({
        "type": "react",
        "room_id": ROOM_A.to_string(),
        "message_id": MESSAGE_A.to_string(),
        "emoji": EMOJI_A,
        "action": "remove",
    }),
)]
#[case::set_topic_title(
    ClientFrame::SetTopicTitle(SetTopicTitle {
        room_id: ROOM_A,
        root_message_id: MESSAGE_A,
        topic_title: "Plans".to_owned(),
    }),
    json!({
        "type": "set_topic_title",
        "room_id": ROOM_A.to_string(),
        "root_message_id": MESSAGE_A.to_string(),
        "topic_title": "Plans",
    }),
)]
#[case::set_thread_subscription(
    ClientFrame::SetThreadSubscription(SetThreadSubscription {
        room_id: ROOM_A,
        root_message_id: MESSAGE_A,
        state: SubscriptionState::Muted,
        origin: SubscriptionOrigin::Explicit,
    }),
    json!({
        "type": "set_thread_subscription",
        "room_id": ROOM_A.to_string(),
        "root_message_id": MESSAGE_A.to_string(),
        "state": "muted",
        "origin": "explicit",
    }),
)]
#[case::mark_thread_read(
    ClientFrame::MarkThreadRead(MarkThreadRead {
        room_id: ROOM_A,
        root_message_id: MESSAGE_A,
        read_through: MESSAGE_C,
    }),
    json!({
        "type": "mark_thread_read",
        "room_id": ROOM_A.to_string(),
        "root_message_id": MESSAGE_A.to_string(),
        "read_through": MESSAGE_C.to_string(),
    }),
)]
#[case::sync_thread_first_page(
    ClientFrame::SyncThread(SyncThread { room_id: ROOM_A, root_message_id: MESSAGE_A, after: None }),
    json!({
        "type": "sync_thread",
        "room_id": ROOM_A.to_string(),
        "root_message_id": MESSAGE_A.to_string(),
        "after": null,
    }),
)]
#[case::sync_thread_next_page(
    ClientFrame::SyncThread(SyncThread {
        room_id: ROOM_A,
        root_message_id: MESSAGE_A,
        after: Some(MESSAGE_B),
    }),
    json!({
        "type": "sync_thread",
        "room_id": ROOM_A.to_string(),
        "root_message_id": MESSAGE_A.to_string(),
        "after": MESSAGE_B.to_string(),
    }),
)]
#[case::sync_inbox(ClientFrame::SyncInbox(SyncInbox {}), json!({"type": "sync_inbox"}))]
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
        "protocol_version": 3,
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
    json!({"type": "version_mismatch", "supported": 3, "received": 99}),
)]
#[case::handshake_rejected_campus(
    ServerFrame::HandshakeRejected(HandshakeRejected { reason: HandshakeRejection::CampusUnavailable }),
    json!({"type": "handshake_rejected", "reason": "campus_unavailable"}),
)]
#[case::handshake_rejected_auth(
    ServerFrame::HandshakeRejected(HandshakeRejected { reason: HandshakeRejection::Unauthenticated }),
    json!({"type": "handshake_rejected", "reason": "unauthenticated"}),
)]
#[case::message_accepted(
    ServerFrame::MessageAccepted(MessageAccepted { message: room_message(MESSAGE_A, None) }),
    json!({"type": "message_accepted", "message": room_message_json(MESSAGE_A, None)}),
)]
#[case::message_created(
    ServerFrame::MessageCreated(MessageCreated { message: room_message(MESSAGE_B, Some(MESSAGE_A)) }),
    json!({"type": "message_created", "message": room_message_json(MESSAGE_B, Some(MESSAGE_A))}),
)]
#[case::message_rejected(
    ServerFrame::MessageRejected(MessageRejected {
        client_nonce: NONCE_A,
        reason: MessageRejection::Forbidden,
    }),
    json!({"type": "message_rejected", "client_nonce": NONCE_A.to_string(), "reason": "forbidden"}),
)]
#[case::room_synced(
    ServerFrame::RoomSynced(RoomSynced {
        room_id: ROOM_A,
        messages: vec![room_message(MESSAGE_A, None)],
        updates: vec![room_message(MESSAGE_B, Some(MESSAGE_A))],
        activity_seq: 12,
        has_more: true,
    }),
    json!({
        "type": "room_synced",
        "room_id": ROOM_A.to_string(),
        "messages": [room_message_json(MESSAGE_A, None)],
        "updates": [room_message_json(MESSAGE_B, Some(MESSAGE_A))],
        "activity_seq": 12,
        "has_more": true,
    }),
)]
#[case::message_updated(
    ServerFrame::MessageUpdated(MessageUpdated { message: busy_message() }),
    json!({"type": "message_updated", "message": busy_message_json()}),
)]
#[case::reaction_rejected(
    ServerFrame::ReactionRejected(ReactionRejected {
        room_id: ROOM_A,
        message_id: MESSAGE_A,
        emoji: EMOJI_B.to_owned(),
        reason: ReactionRejection::TooManyKinds,
    }),
    json!({
        "type": "reaction_rejected",
        "room_id": ROOM_A.to_string(),
        "message_id": MESSAGE_A.to_string(),
        "emoji": EMOJI_B,
        "reason": "too_many_kinds",
    }),
)]
#[case::thread_rejected(
    ServerFrame::ThreadRejected(ThreadRejected {
        room_id: ROOM_A,
        root_message_id: MESSAGE_A,
        reason: ThreadRejection::UnknownThread,
    }),
    json!({
        "type": "thread_rejected",
        "room_id": ROOM_A.to_string(),
        "root_message_id": MESSAGE_A.to_string(),
        "reason": "unknown_thread",
    }),
)]
#[case::thread_synced(
    ServerFrame::ThreadSynced(ThreadSynced {
        room_id: ROOM_A,
        root_message_id: MESSAGE_A,
        messages: vec![reply_message()],
        has_more: false,
    }),
    json!({
        "type": "thread_synced",
        "room_id": ROOM_A.to_string(),
        "root_message_id": MESSAGE_A.to_string(),
        "messages": [reply_message_json()],
        "has_more": false,
    }),
)]
#[case::inbox_synced(
    ServerFrame::InboxSynced(InboxSynced { threads: vec![thread_summary()] }),
    json!({"type": "inbox_synced", "threads": [thread_summary_json()]}),
)]
#[case::thread_changed(
    ServerFrame::ThreadChanged(ThreadChanged { summary: thread_summary() }),
    json!({"type": "thread_changed", "summary": thread_summary_json()}),
)]
#[case::room_unavailable(
    ServerFrame::RoomUnavailable(RoomUnavailable { room_id: ROOM_B }),
    json!({"type": "room_unavailable", "room_id": ROOM_B.to_string()}),
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
#[case::post_message(Permission::PostMessage, json!("post_message"))]
#[case::react(Permission::React, json!("react"))]
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
#[case::message_body(InputField::MessageBody, json!("message_body"))]
#[case::topic_title(InputField::TopicTitle, json!("topic_title"))]
#[case::reaction_emoji(InputField::ReactionEmoji, json!("reaction_emoji"))]
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
#[case::forbidden(MessageRejection::Forbidden, json!("forbidden"))]
#[case::invalid_body(MessageRejection::InvalidBody, json!("invalid_body"))]
#[case::unknown_room(MessageRejection::UnknownRoom, json!("unknown_room"))]
#[case::unknown_thread(MessageRejection::UnknownThread, json!("unknown_thread"))]
fn message_rejection_tags(#[case] reason: MessageRejection, #[case] expected: Value) {
    assert_wire(reason, expected);
}

#[rstest]
#[case::pending(DeliveryState::Pending, json!("pending"))]
#[case::confirmed(DeliveryState::Confirmed, json!("confirmed"))]
#[case::failed(DeliveryState::Failed, json!("failed"))]
fn delivery_state_tags(#[case] state: DeliveryState, #[case] expected: Value) {
    assert_wire(state, expected);
}

#[rstest]
#[case::root(room_message(MESSAGE_A, None), room_message_json(MESSAGE_A, None))]
#[case::chained(
    room_message(MESSAGE_B, Some(MESSAGE_A)),
    room_message_json(MESSAGE_B, Some(MESSAGE_A))
)]
fn room_message_wire_format(#[case] message: RoomMessage, #[case] expected: Value) {
    assert_wire(message, expected);
}

#[rstest]
#[case::pending(
    timeline_entry(DeliveryState::Pending, None),
    timeline_entry_json(None, "pending", None)
)]
#[case::confirmed(
    timeline_entry(DeliveryState::Confirmed, None),
    timeline_entry_json(Some(MESSAGE_A), "confirmed", None)
)]
#[case::failed(
    timeline_entry(DeliveryState::Failed, Some(MessageRejection::InvalidBody)),
    timeline_entry_json(None, "failed", Some("invalid_body"))
)]
fn timeline_entry_wire_format(#[case] entry: TimelineEntry, #[case] expected: Value) {
    assert_wire(entry, expected);
}

#[rstest]
#[case::signed_out(
    ClientSnapshot {
        sequence: 0,
        network: disconnected_network(),
        session: signed_out_session(),
        timeline: empty_timeline(),
        thread: ThreadSnapshot::default(),
        inbox: InboxSnapshot::default(),
    },
    json!({
        "sequence": 0,
        "network": {
            "protocol_version": 3,
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
            "active_room_id": null,
            "busy": false,
            "error": null,
        },
        "timeline": {
            "room_id": null,
            "entries": [],
            "syncing": false,
            "error": null,
            "reaction_palette": [],
        },
        "thread": {
            "room_id": null,
            "root_message_id": null,
            "root": null,
            "replies": [],
            "topic_title": null,
            "subscription": null,
            "syncing": false,
        },
        "inbox": {
            "open": false,
            "items": [],
            "unread_threads": 0,
            "selected_root_message_id": null,
            "replying": false,
        },
    }),
)]
#[case::signed_in_rejected(
    ClientSnapshot {
        sequence: 42,
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
            active_room_id: Some(ROOM_A),
            busy: true,
            error: Some(ApiError::invalid(InputField::Slug)),
        },
        timeline: TimelineSnapshot {
            room_id: Some(ROOM_A),
            entries: vec![
                timeline_entry(DeliveryState::Confirmed, None),
                timeline_entry(DeliveryState::Failed, Some(MessageRejection::Forbidden)),
            ],
            syncing: true,
            error: Some(ApiError::invalid(InputField::MessageBody)),
            reaction_palette: vec![EMOJI_A.to_owned(), EMOJI_B.to_owned()],
        },
        thread: ThreadSnapshot {
            room_id: Some(ROOM_A),
            root_message_id: Some(MESSAGE_A),
            root: Some(timeline_entry(DeliveryState::Confirmed, None)),
            replies: vec![timeline_entry(DeliveryState::Pending, None)],
            topic_title: Some("Plans".to_owned()),
            subscription: Some(SubscriptionState::Archived),
            syncing: true,
        },
        inbox: InboxSnapshot {
            open: true,
            items: vec![inbox_item()],
            unread_threads: 1,
            selected_root_message_id: Some(MESSAGE_A),
            replying: true,
        },
    },
    json!({
        "sequence": 42,
        "network": {
            "protocol_version": 3,
            "connection": "backoff",
            "retry_in_ms": 1000,
            "campus_id": CAMPUS_A.to_string(),
            "session_id": SESSION_A.to_string(),
            "attempt": 2,
            "rejection": {"supported": 3, "received": 1},
            "handshake_rejection": "campus_unavailable",
        },
        "session": {
            "auth": "signed_in",
            "account": profile_json(),
            "campuses": [summary_json()],
            "active_campus_id": CAMPUS_A.to_string(),
            "rooms": [room_json("commons")],
            "active_room_id": ROOM_A.to_string(),
            "busy": true,
            "error": {"code": "invalid_input", "field": "slug"},
        },
        "timeline": {
            "room_id": ROOM_A.to_string(),
            "entries": [
                timeline_entry_json(Some(MESSAGE_A), "confirmed", None),
                timeline_entry_json(None, "failed", Some("forbidden")),
            ],
            "syncing": true,
            "error": {"code": "invalid_input", "field": "message_body"},
            "reaction_palette": [EMOJI_A, EMOJI_B],
        },
        "thread": {
            "room_id": ROOM_A.to_string(),
            "root_message_id": MESSAGE_A.to_string(),
            "root": timeline_entry_json(Some(MESSAGE_A), "confirmed", None),
            "replies": [timeline_entry_json(None, "pending", None)],
            "topic_title": "Plans",
            "subscription": "archived",
            "syncing": true,
        },
        "inbox": {
            "open": true,
            "items": [inbox_item_json()],
            "unread_threads": 1,
            "selected_root_message_id": MESSAGE_A.to_string(),
            "replying": true,
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
#[case::owner_post_message(Role::Owner, Permission::PostMessage, true)]
#[case::owner_react(Role::Owner, Permission::React, true)]
#[case::owner_manage_members(Role::Owner, Permission::ManageMembers, true)]
#[case::owner_manage_campus(Role::Owner, Permission::ManageCampus, true)]
#[case::admin_view_campus(Role::Admin, Permission::ViewCampus, true)]
#[case::admin_view_rooms(Role::Admin, Permission::ViewRooms, true)]
#[case::admin_create_room(Role::Admin, Permission::CreateRoom, true)]
#[case::admin_post_message(Role::Admin, Permission::PostMessage, true)]
#[case::admin_react(Role::Admin, Permission::React, true)]
#[case::admin_manage_members(Role::Admin, Permission::ManageMembers, true)]
#[case::admin_manage_campus(Role::Admin, Permission::ManageCampus, false)]
#[case::member_view_campus(Role::Member, Permission::ViewCampus, true)]
#[case::member_view_rooms(Role::Member, Permission::ViewRooms, true)]
#[case::member_create_room(Role::Member, Permission::CreateRoom, true)]
#[case::member_post_message(Role::Member, Permission::PostMessage, true)]
#[case::member_react(Role::Member, Permission::React, true)]
#[case::member_manage_members(Role::Member, Permission::ManageMembers, false)]
#[case::member_manage_campus(Role::Member, Permission::ManageCampus, false)]
#[case::guest_view_campus(Role::Guest, Permission::ViewCampus, true)]
#[case::guest_view_rooms(Role::Guest, Permission::ViewRooms, true)]
#[case::guest_create_room(Role::Guest, Permission::CreateRoom, false)]
#[case::guest_post_message(Role::Guest, Permission::PostMessage, false)]
#[case::guest_react(Role::Guest, Permission::React, true)]
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
    assert_eq!(Permission::ALL.len(), 7);
}

#[rstest]
#[case::trims_padding("  hello  ", Ok("hello".to_owned()))]
#[case::empty("", Err(InputField::MessageBody))]
#[case::blank(" \n\t ", Err(InputField::MessageBody))]
#[case::at_limit(&"a".repeat(MESSAGE_BODY_MAX_CHARS), Ok("a".repeat(MESSAGE_BODY_MAX_CHARS)))]
#[case::over_limit(&"a".repeat(MESSAGE_BODY_MAX_CHARS + 1), Err(InputField::MessageBody))]
#[case::multibyte_at_limit(&"é".repeat(MESSAGE_BODY_MAX_CHARS), Ok("é".repeat(MESSAGE_BODY_MAX_CHARS)))]
#[case::multibyte_over_limit(&"é".repeat(MESSAGE_BODY_MAX_CHARS + 1), Err(InputField::MessageBody))]
#[case::inner_newline_kept("a\nb", Ok("a\nb".to_owned()))]
#[case::inner_tab_kept("a\tb", Ok("a\tb".to_owned()))]
#[case::bell_rejected("a\u{7}b", Err(InputField::MessageBody))]
#[case::null_rejected("a\0b", Err(InputField::MessageBody))]
fn message_body_normalization(#[case] raw: &str, #[case] expected: Result<String, InputField>) {
    assert_eq!(normalize_message_body(raw), expected);
}

#[rstest]
#[case::valid("hello", None)]
#[case::empty("", Some(InputField::MessageBody))]
#[case::control("a\u{7}", Some(InputField::MessageBody))]
fn send_message_validation(#[case] body: &str, #[case] field: Option<InputField>) {
    let request = SendMessage {
        room_id: ROOM_A,
        client_nonce: NONCE_A,
        body: body.to_owned(),
        thread_root_id: None,
    };
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
}

#[test]
fn sync_limits_are_stable() {
    assert_eq!(SYNC_PAGE_LIMIT, 100);
    assert_eq!(SYNC_MAX_ROOMS, 256);
    assert_eq!(INBOX_SYNC_LIMIT, 200);
    assert_eq!(MESSAGE_MENTIONS_MAX, 16);
    assert_eq!(REACTION_KINDS_MAX, 20);
    assert_eq!(REACTION_EMOJI_MAX_CHARS, 16);
    assert_eq!(TOPIC_TITLE_MAX_CHARS, 120);
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
#[case::single_char("A", Some("A".to_owned()))]
#[case::padded("  Ada  ", Some("Ada".to_owned()))]
#[case::padded_campus("  North  ", Some("North".to_owned()))]
#[case::at_limit(&"a".repeat(64), Some("a".repeat(64)))]
#[case::over_limit(&"a".repeat(65), None)]
#[case::empty("", None)]
#[case::blank("   ", None)]
#[case::control("Ada\nLovelace", None)]
#[case::multibyte_at_limit(&"é".repeat(64), Some("é".repeat(64)))]
fn display_name_rules(#[case] raw: &str, #[case] normalized: Option<String>) {
    let expected = normalized.ok_or(InputField::DisplayName);
    assert_eq!(normalize_display_name(raw), expected);
    assert_eq!(validate_display_name(raw).is_ok(), expected.is_ok());
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
#[case::password_at_max("ada@example.org", &"p".repeat(128), None)]
#[case::password_over_max("ada@example.org", &"p".repeat(129), Some(InputField::Password))]
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
    assert_eq!(close_code::TRY_AGAIN_LATER, 1013);
}

#[test]
fn generated_ids_are_version_seven() {
    assert_eq!(CampusId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(SessionId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(AccountId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(RoomId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(MessageId::generate().as_uuid().get_version_num(), 7);
    assert_eq!(ClientNonce::generate().as_uuid().get_version_num(), 7);
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

#[test]
fn protocol_version_is_three() {
    assert_eq!(ProtocolVersion::CURRENT, ProtocolVersion(3));
}

#[rstest]
#[case::following(SubscriptionState::Following, json!("following"))]
#[case::muted(SubscriptionState::Muted, json!("muted"))]
#[case::archived(SubscriptionState::Archived, json!("archived"))]
fn subscription_state_tags(#[case] state: SubscriptionState, #[case] expected: Value) {
    assert_wire(state, expected.clone());
    let text = expected.as_str().expect("text");
    assert_eq!(state.as_str(), text);
    assert_eq!(text.parse::<SubscriptionState>().ok(), Some(state));
}

#[rstest]
#[case::empty("")]
#[case::capitalised("Following")]
#[case::unknown("snoozed")]
#[case::padded(" muted")]
fn unknown_subscription_text_is_rejected(#[case] text: &str) {
    assert!(text.parse::<SubscriptionState>().is_err());
}

#[test]
fn subscription_states_are_complete() {
    assert_eq!(
        SubscriptionState::ALL,
        [
            SubscriptionState::Following,
            SubscriptionState::Muted,
            SubscriptionState::Archived,
        ]
    );
}

#[rstest]
#[case::explicit(SubscriptionOrigin::Explicit, json!("explicit"))]
#[case::viewed(SubscriptionOrigin::Viewed, json!("viewed"))]
fn subscription_origin_tags(#[case] origin: SubscriptionOrigin, #[case] expected: Value) {
    assert_wire(origin, expected);
}

#[rstest]
#[case::forbidden(ThreadRejection::Forbidden, json!("forbidden"))]
#[case::unknown_thread(ThreadRejection::UnknownThread, json!("unknown_thread"))]
#[case::invalid_title(ThreadRejection::InvalidTopicTitle, json!("invalid_topic_title"))]
fn thread_rejection_tags(#[case] reason: ThreadRejection, #[case] expected: Value) {
    assert_wire(reason, expected);
}

#[rstest]
#[case::forbidden(ReactionRejection::Forbidden, json!("forbidden"))]
#[case::invalid_emoji(ReactionRejection::InvalidEmoji, json!("invalid_emoji"))]
#[case::unknown_message(ReactionRejection::UnknownMessage, json!("unknown_message"))]
#[case::too_many_kinds(ReactionRejection::TooManyKinds, json!("too_many_kinds"))]
fn reaction_rejection_tags(#[case] reason: ReactionRejection, #[case] expected: Value) {
    assert_wire(reason, expected);
}

#[rstest]
#[case::add(ReactionAction::Add, json!("add"))]
#[case::remove(ReactionAction::Remove, json!("remove"))]
fn reaction_action_tags(#[case] action: ReactionAction, #[case] expected: Value) {
    assert_wire(action, expected);
}

#[rstest]
#[case::text(SegmentKind::Text, json!("text"))]
#[case::mention(SegmentKind::Mention, json!("mention"))]
fn segment_kind_tags(#[case] kind: SegmentKind, #[case] expected: Value) {
    assert_wire(kind, expected);
}

#[rstest]
#[case::next(InboxStep::Next, json!("next"))]
#[case::previous(InboxStep::Previous, json!("previous"))]
fn inbox_step_tags(#[case] step: InboxStep, #[case] expected: Value) {
    assert_wire(step, expected);
}

#[test]
fn inbox_item_kind_tag() {
    assert_wire(InboxItemKind::Thread, json!("thread"));
}

#[rstest]
#[case::room_message_with_mentions_and_reactions(busy_message(), busy_message_json())]
#[case::reply(reply_message(), reply_message_json())]
fn rich_room_messages_match_wire_format(#[case] message: RoomMessage, #[case] expected: Value) {
    assert_wire(message, expected);
}

#[rstest]
#[case::reaction_summary(
    ReactionSummary { emoji: EMOJI_A.to_owned(), count: 2, reacted_by_self: true },
    json!({"emoji": EMOJI_A, "count": 2, "reacted_by_self": true}),
)]
#[case::reply_preview(reply_preview(), reply_preview_json())]
#[case::thread_preview(
    ThreadPreview {
        topic_title: Some("Plans".to_owned()),
        reply_count: 3,
        latest_reply: Some(reply_preview()),
        show_latest_reply: true,
        subscription: Some(SubscriptionState::Muted),
        unread_replies: 2,
    },
    json!({
        "topic_title": "Plans",
        "reply_count": 3,
        "latest_reply": reply_preview_json(),
        "show_latest_reply": true,
        "subscription": "muted",
        "unread_replies": 2,
    }),
)]
#[case::bare_thread_preview(
    ThreadPreview {
        topic_title: None,
        reply_count: 0,
        latest_reply: None,
        show_latest_reply: false,
        subscription: None,
        unread_replies: 0,
    },
    json!({
        "topic_title": null,
        "reply_count": 0,
        "latest_reply": null,
        "show_latest_reply": false,
        "subscription": null,
        "unread_replies": 0,
    }),
)]
#[case::inbox_item(inbox_item(), inbox_item_json())]
#[case::thread_summary(thread_summary(), thread_summary_json())]
#[case::unread_free_summary(
    ThreadSummary {
        latest_reply: None,
        unread_replies: 0,
        unread_mentions: 0,
        subscription: SubscriptionState::Archived,
        read_through: None,
        revision: 1,
        ..thread_summary()
    },
    {
        let mut value = thread_summary_json();
        value["latest_reply"] = Value::Null;
        value["unread_replies"] = json!(0);
        value["unread_mentions"] = json!(0);
        value["subscription"] = json!("archived");
        value["read_through"] = Value::Null;
        value["revision"] = json!(1);
        value
    },
)]
fn thread_types_match_wire_format<T>(#[case] value: T, #[case] expected: Value)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    assert_wire(value, expected);
}

#[rstest]
#[case::trims_padding("  Plans  ", Ok(Some("Plans".to_owned())))]
#[case::empty_clears("", Ok(None))]
#[case::blank_clears(" \t\n ", Ok(None))]
#[case::inner_space_kept("Q3 plans", Ok(Some("Q3 plans".to_owned())))]
#[case::at_limit(&"t".repeat(TOPIC_TITLE_MAX_CHARS), Ok(Some("t".repeat(TOPIC_TITLE_MAX_CHARS))))]
#[case::over_limit(&"t".repeat(TOPIC_TITLE_MAX_CHARS + 1), Err(InputField::TopicTitle))]
#[case::multibyte_at_limit(&"é".repeat(TOPIC_TITLE_MAX_CHARS), Ok(Some("é".repeat(TOPIC_TITLE_MAX_CHARS))))]
#[case::multibyte_over_limit(&"é".repeat(TOPIC_TITLE_MAX_CHARS + 1), Err(InputField::TopicTitle))]
#[case::inner_newline_rejected("a\nb", Err(InputField::TopicTitle))]
#[case::inner_tab_rejected("a\tb", Err(InputField::TopicTitle))]
#[case::bell_rejected("a\u{7}b", Err(InputField::TopicTitle))]
#[case::null_rejected("a\0b", Err(InputField::TopicTitle))]
fn topic_title_normalization(
    #[case] raw: &str,
    #[case] expected: Result<Option<String>, InputField>,
) {
    assert_eq!(normalize_topic_title(raw), expected);
}

#[rstest]
#[case::thumbs_up("👍", Ok("👍".to_owned()))]
#[case::trims_padding("  🎉 ", Ok("🎉".to_owned()))]
#[case::variation_selector("❤️", Ok("❤️".to_owned()))]
#[case::joined_sequence("👨\u{200d}👩\u{200d}👧", Ok("👨\u{200d}👩\u{200d}👧".to_owned()))]
#[case::at_limit(&"👍".repeat(REACTION_EMOJI_MAX_CHARS), Ok("👍".repeat(REACTION_EMOJI_MAX_CHARS)))]
#[case::over_limit(&"👍".repeat(REACTION_EMOJI_MAX_CHARS + 1), Err(InputField::ReactionEmoji))]
#[case::empty("", Err(InputField::ReactionEmoji))]
#[case::blank("  \t", Err(InputField::ReactionEmoji))]
#[case::ascii_letter("a", Err(InputField::ReactionEmoji))]
#[case::ascii_digit("1", Err(InputField::ReactionEmoji))]
#[case::ascii_punctuation(":)", Err(InputField::ReactionEmoji))]
#[case::emoji_then_ascii("👍a", Err(InputField::ReactionEmoji))]
#[case::inner_space("👍 🎉", Err(InputField::ReactionEmoji))]
#[case::no_break_space("👍\u{a0}🎉", Err(InputField::ReactionEmoji))]
#[case::control("👍\u{7}", Err(InputField::ReactionEmoji))]
#[case::newline("👍\n🎉", Err(InputField::ReactionEmoji))]
fn reaction_emoji_normalization(#[case] raw: &str, #[case] expected: Result<String, InputField>) {
    assert_eq!(normalize_reaction_emoji(raw), expected);
}

#[test]
fn palette_emoji_are_all_valid_reactions() {
    REACTION_PALETTE.iter().for_each(|emoji| {
        assert_eq!(normalize_reaction_emoji(emoji), Ok((*emoji).to_owned()));
    });
    assert_eq!(REACTION_PALETTE.len(), 6);
}

#[rstest]
#[case::valid("👍", None)]
#[case::empty("", Some(InputField::ReactionEmoji))]
#[case::ascii("a", Some(InputField::ReactionEmoji))]
fn react_validation(#[case] emoji: &str, #[case] field: Option<InputField>) {
    let request = React {
        emoji: emoji.to_owned(),
        ..react(ReactionAction::Add)
    };
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
}

#[rstest]
#[case::title("Plans", None)]
#[case::clearing("", None)]
#[case::control("a\u{7}", Some(InputField::TopicTitle))]
#[case::over_limit(&"t".repeat(TOPIC_TITLE_MAX_CHARS + 1), Some(InputField::TopicTitle))]
fn set_topic_title_validation(#[case] title: &str, #[case] field: Option<InputField>) {
    let request = SetTopicTitle {
        room_id: ROOM_A,
        root_message_id: MESSAGE_A,
        topic_title: title.to_owned(),
    };
    assert_eq!(
        request.validate().err().map(|error| error.field),
        field.map(Some)
    );
}

fn account(index: u128) -> AccountId {
    AccountId(Uuid::from_u128(
        0x0193a5c0_0000_7000_8000_000000000000 + index,
    ))
}

fn many_tokens(count: u128) -> String {
    (1..=count)
        .map(|index| mention_token(account(index)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn accounts(range: std::ops::RangeInclusive<u128>) -> Vec<AccountId> {
    range.map(account).collect()
}

#[rstest]
#[case::no_tokens("hello there", vec![])]
#[case::empty("", vec![])]
#[case::one_token(&mention_token(ACCOUNT_A), vec![ACCOUNT_A])]
#[case::inside_text(&format!("hi {} bye", mention_token(ACCOUNT_B)), vec![ACCOUNT_B])]
#[case::first_occurrence_order(
    &format!("{} {} {}", mention_token(ACCOUNT_C), mention_token(ACCOUNT_A), mention_token(ACCOUNT_B)),
    vec![ACCOUNT_C, ACCOUNT_A, ACCOUNT_B],
)]
#[case::duplicates_collapse(
    &format!("{} {} {}", mention_token(ACCOUNT_A), mention_token(ACCOUNT_B), mention_token(ACCOUNT_A)),
    vec![ACCOUNT_A, ACCOUNT_B],
)]
#[case::adjacent_tokens(
    &format!("{}{}", mention_token(ACCOUNT_A), mention_token(ACCOUNT_B)),
    vec![ACCOUNT_A, ACCOUNT_B],
)]
#[case::missing_close("<@0193a5c0-0000-7000-8000-0000000000c1", vec![])]
#[case::short_id("<@1234>", vec![])]
#[case::not_a_uuid("<@zzzzzzzz-zzzz-zzzz-zzzz-zzzzzzzzzzzz>", vec![])]
#[case::unhyphenated("<@0193a5c000007000800000000000c1>", vec![])]
#[case::braced("{@0193a5c0-0000-7000-8000-0000000000c1}", vec![])]
#[case::space_inside("<@ 0193a5c0-0000-7000-8000-0000000000c1>", vec![])]
#[case::bare_uuid("0193a5c0-0000-7000-8000-0000000000c1", vec![])]
#[case::truncates_at_limit(&many_tokens(MESSAGE_MENTIONS_MAX as u128 + 4), accounts(1..=MESSAGE_MENTIONS_MAX as u128))]
#[case::exactly_at_limit(&many_tokens(MESSAGE_MENTIONS_MAX as u128), accounts(1..=MESSAGE_MENTIONS_MAX as u128))]
fn mention_extraction(#[case] body: &str, #[case] expected: Vec<AccountId>) {
    assert_eq!(extract_mentions(body), expected);
}

#[test]
fn mention_token_is_the_hyphenated_form() {
    assert_eq!(
        mention_token(ACCOUNT_A),
        format!("<@{}>", ACCOUNT_A.0.hyphenated())
    );
    assert_eq!(extract_mentions(&mention_token(ACCOUNT_A)), vec![ACCOUNT_A]);
}

fn mention_of(account_id: AccountId, name: &str) -> MentionRef {
    MentionRef {
        account_id,
        display_name: name.to_owned(),
    }
}

fn segment(kind: SegmentKind, text: &str, account_id: Option<AccountId>) -> BodySegment {
    BodySegment {
        kind,
        text: text.to_owned(),
        account_id,
    }
}

#[rstest]
#[case::empty_body("", vec![], vec![])]
#[case::plain_text("hello", vec![], vec![text_segment("hello")])]
#[case::resolved_token(
    &mention_token(ACCOUNT_A),
    vec![mention_of(ACCOUNT_A, "Ada")],
    vec![segment(SegmentKind::Mention, "@Ada", Some(ACCOUNT_A))],
)]
#[case::token_between_text(
    &format!("hi {} bye", mention_token(ACCOUNT_A)),
    vec![mention_of(ACCOUNT_A, "Ada")],
    vec![
        text_segment("hi "),
        segment(SegmentKind::Mention, "@Ada", Some(ACCOUNT_A)),
        text_segment(" bye"),
    ],
)]
#[case::unresolved_token_stays_text(
    &format!("hi {}", mention_token(ACCOUNT_B)),
    vec![mention_of(ACCOUNT_A, "Ada")],
    vec![text_segment(&format!("hi {}", mention_token(ACCOUNT_B)))],
)]
#[case::unresolved_between_text_merges_into_one(
    &format!("a {} b", mention_token(ACCOUNT_B)),
    vec![],
    vec![text_segment(&format!("a {} b", mention_token(ACCOUNT_B)))],
)]
#[case::malformed_token_stays_text("a <@1234> b", vec![], vec![text_segment("a <@1234> b")])]
#[case::lone_opener_stays_text("<@", vec![], vec![text_segment("<@")])]
#[case::adjacent_tokens(
    &format!("{}{}", mention_token(ACCOUNT_A), mention_token(ACCOUNT_B)),
    vec![mention_of(ACCOUNT_A, "Ada"), mention_of(ACCOUNT_B, "Grace")],
    vec![
        segment(SegmentKind::Mention, "@Ada", Some(ACCOUNT_A)),
        segment(SegmentKind::Mention, "@Grace", Some(ACCOUNT_B)),
    ],
)]
#[case::repeated_token_resolves_each_time(
    &format!("{0} {0}", mention_token(ACCOUNT_A)),
    vec![mention_of(ACCOUNT_A, "Ada")],
    vec![
        segment(SegmentKind::Mention, "@Ada", Some(ACCOUNT_A)),
        text_segment(" "),
        segment(SegmentKind::Mention, "@Ada", Some(ACCOUNT_A)),
    ],
)]
#[case::trailing_close_bracket_is_text(
    &format!("{}>", mention_token(ACCOUNT_A)),
    vec![mention_of(ACCOUNT_A, "Ada")],
    vec![segment(SegmentKind::Mention, "@Ada", Some(ACCOUNT_A)), text_segment(">")],
)]
#[case::multibyte_text_and_name(
    &format!("héllo {} ünï", mention_token(ACCOUNT_A)),
    vec![mention_of(ACCOUNT_A, "Zoë Ångström")],
    vec![
        text_segment("héllo "),
        segment(SegmentKind::Mention, "@Zoë Ångström", Some(ACCOUNT_A)),
        text_segment(" ünï"),
    ],
)]
#[case::malformed_then_valid(
    &format!("<@ {}", mention_token(ACCOUNT_A)),
    vec![mention_of(ACCOUNT_A, "Ada")],
    vec![text_segment("<@ "), segment(SegmentKind::Mention, "@Ada", Some(ACCOUNT_A))],
)]
fn body_segmentation(
    #[case] body: &str,
    #[case] mentions: Vec<MentionRef>,
    #[case] expected: Vec<BodySegment>,
) {
    assert_eq!(body_segments(body, &mentions), expected);
}
