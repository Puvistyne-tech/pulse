#![allow(dead_code)]

use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use futures_util::{SinkExt, StreamExt};
use http_body_util::BodyExt;
use pulse_protocol::fixtures::VALID_PASSWORD;
use pulse_protocol::{
    AccountId, ApiError, AttachmentId, AuthSession, AuthToken, CampusCreated, CampusId, CampusList,
    ClientFrame, ClientNonce, CreateCampusRequest, CreateUploadRequest, DownloadTickets,
    DownloadTicketsRequest, Hello, InboxSynced, MarkThreadRead, MessageId, Ping, Pong,
    ProtocolVersion, React, ReactionAction, Role, RoomCursor, RoomId, RoomList, RoomMessage,
    RoomSynced, SendMessage, ServerFrame, SetPin, SetThreadSubscription, SetTopicTitle,
    ShelfAction, SignUpRequest, SubscriptionOrigin, SubscriptionState, SyncInbox, SyncRooms,
    SyncShelf, SyncThread, SyncVault, ThreadSummary, ThreadSynced, UpdateShelf, UploadTicket,
    VaultTab, Welcome, api_path,
};
use pulse_server::auth::token_digest;
use pulse_server::config::DEFAULT_DATABASE_URL;
use pulse_server::{AppState, Config, MIGRATOR, router, state};
use serde::Serialize;
use serde_json::Value;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::{JoinHandle, JoinSet};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tower::ServiceExt;
use uuid::Uuid;

const DEFAULT_VALKEY_URL: &str = "redis://127.0.0.1:6379";
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(5);
const START_POSTGRES: &str = "start Postgres: docker compose up -d postgres";

pub type ClientSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Observed {
    Frame(ServerFrame),
    Close(u16),
}

pub enum Outgoing {
    Frame(ClientFrame),
    Raw(String),
}

pub fn hello(campus_id: CampusId, version: u16, token: &AuthToken) -> Outgoing {
    Outgoing::Frame(ClientFrame::Hello(Hello {
        protocol_version: ProtocolVersion(version),
        campus_id,
        auth_token: token.clone(),
    }))
}

pub fn ping(nonce: u32) -> Outgoing {
    Outgoing::Frame(ClientFrame::Ping(Ping { nonce }))
}

pub fn send_message(room_id: RoomId, client_nonce: ClientNonce, body: &str) -> Outgoing {
    Outgoing::Frame(ClientFrame::SendMessage(SendMessage {
        room_id,
        client_nonce,
        body: body.to_owned(),
        thread_root_id: None,
        attachment_ids: Vec::new(),
    }))
}

pub fn send_reply(
    room_id: RoomId,
    root: MessageId,
    client_nonce: ClientNonce,
    body: &str,
) -> Outgoing {
    Outgoing::Frame(ClientFrame::SendMessage(SendMessage {
        room_id,
        client_nonce,
        body: body.to_owned(),
        thread_root_id: Some(root),
        attachment_ids: Vec::new(),
    }))
}

pub fn sync_rooms(cursors: &[(RoomId, Option<MessageId>)]) -> Outgoing {
    sync_rooms_at(
        &cursors
            .iter()
            .map(|(room_id, after)| (*room_id, *after, None))
            .collect::<Vec<_>>(),
    )
}

pub fn sync_rooms_at(cursors: &[(RoomId, Option<MessageId>, Option<u64>)]) -> Outgoing {
    Outgoing::Frame(ClientFrame::SyncRooms(SyncRooms {
        cursors: cursors
            .iter()
            .map(|(room_id, after, activity_after)| RoomCursor {
                room_id: *room_id,
                after: *after,
                activity_after: *activity_after,
            })
            .collect(),
    }))
}

pub fn react(
    room_id: RoomId,
    message_id: MessageId,
    emoji: &str,
    action: ReactionAction,
) -> Outgoing {
    Outgoing::Frame(ClientFrame::React(React {
        room_id,
        message_id,
        emoji: emoji.to_owned(),
        action,
    }))
}

pub fn set_title(room_id: RoomId, root: MessageId, title: &str) -> Outgoing {
    Outgoing::Frame(ClientFrame::SetTopicTitle(SetTopicTitle {
        room_id,
        root_message_id: root,
        topic_title: title.to_owned(),
    }))
}

pub fn set_subscription(
    room_id: RoomId,
    root: MessageId,
    state: SubscriptionState,
    origin: SubscriptionOrigin,
) -> Outgoing {
    Outgoing::Frame(ClientFrame::SetThreadSubscription(SetThreadSubscription {
        room_id,
        root_message_id: root,
        state,
        origin,
    }))
}

pub fn mark_read(room_id: RoomId, root: MessageId, read_through: MessageId) -> Outgoing {
    Outgoing::Frame(ClientFrame::MarkThreadRead(MarkThreadRead {
        room_id,
        root_message_id: root,
        read_through,
    }))
}

pub fn sync_inbox() -> Outgoing {
    Outgoing::Frame(ClientFrame::SyncInbox(SyncInbox {}))
}

pub fn sync_thread(room_id: RoomId, root: MessageId, after: Option<MessageId>) -> Outgoing {
    Outgoing::Frame(ClientFrame::SyncThread(SyncThread {
        room_id,
        root_message_id: root,
        after,
    }))
}

pub fn lazy_state() -> AppState {
    let config = Config {
        bind_addr: SocketAddr::from(([127, 0, 0, 1], 0)),
        database_url: "postgres://pulse:pulse@127.0.0.1:1/pulse".to_owned(),
        valkey_url: "redis://127.0.0.1:1".to_owned(),
    };
    state::lazy(&config).expect("lazy state")
}

pub async fn fresh_state() -> AppState {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned());
    let valkey_url = std::env::var("VALKEY_URL").unwrap_or_else(|_| DEFAULT_VALKEY_URL.to_owned());
    let options = PgConnectOptions::from_str(&database_url).expect("database url");
    let admin = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(ACQUIRE_TIMEOUT)
        .connect_with(options.clone())
        .await
        .expect(START_POSTGRES);
    let name = format!("pulse_test_{}", Uuid::now_v7().simple());
    sqlx::query(sqlx::AssertSqlSafe(format!("CREATE DATABASE \"{name}\"")))
        .execute(&admin)
        .await
        .expect("create test database");
    admin.close().await;
    let db = PgPoolOptions::new()
        .acquire_timeout(ACQUIRE_TIMEOUT)
        .connect_with(options.database(&name))
        .await
        .expect(START_POSTGRES);
    MIGRATOR.run(&db).await.expect("run migrations");
    AppState::new(
        db,
        redis::Client::open(valkey_url.as_str()).expect("valkey client"),
    )
}

pub fn sibling_state(state: &AppState) -> AppState {
    AppState::new(state.db.clone(), state.valkey.clone())
}

pub async fn spawn_server_with(state: AppState) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    let app = router(state);
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    addr
}

pub async fn spawn_server() -> SocketAddr {
    spawn_server_with(lazy_state()).await
}

pub async fn api_raw(
    state: &AppState,
    method: Method,
    path: &str,
    token: Option<&AuthToken>,
    body: Option<String>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(path);
    if let Some(token) = token {
        builder = builder.header(header::AUTHORIZATION, format!("Bearer {}", token.0));
    }
    let request = match body {
        Some(text) => builder
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(text)),
        None => builder.body(Body::empty()),
    }
    .expect("request");
    let response = timeout(
        Duration::from_secs(30),
        router(state.clone()).oneshot(request),
    )
    .await
    .expect("request deadline")
    .expect("response");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).expect("json body")
    };
    (status, value)
}

pub async fn api<B: Serialize>(
    state: &AppState,
    method: Method,
    path: &str,
    token: Option<&AuthToken>,
    body: Option<&B>,
) -> (StatusCode, Value) {
    let text = body.map(|body| serde_json::to_string(body).expect("serialize body"));
    api_raw(state, method, path, token, text).await
}

pub async fn signed_up(state: &AppState, email: &str) -> AuthSession {
    let request = SignUpRequest {
        email: email.to_owned(),
        display_name: "Fixture".to_owned(),
        password: VALID_PASSWORD.to_owned(),
    };
    let (status, body) = api(
        state,
        Method::POST,
        api_path::ACCOUNTS,
        None,
        Some(&request),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    serde_json::from_value(body).expect("auth session")
}

pub async fn campus_for(state: &AppState, token: &AuthToken, slug: &str) -> CampusCreated {
    let request = CreateCampusRequest {
        slug: slug.to_owned(),
        display_name: slug.to_owned(),
    };
    let (status, body) = api(
        state,
        Method::POST,
        api_path::CAMPUSES,
        Some(token),
        Some(&request),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    serde_json::from_value(body).expect("campus created")
}

pub async fn grant_role(state: &AppState, campus_id: CampusId, account_id: AccountId, role: Role) {
    sqlx::query(
        "INSERT INTO campus_members (campus_id, account_id, role) VALUES ($1, $2, $3) \
         ON CONFLICT (campus_id, account_id) DO UPDATE SET role = EXCLUDED.role",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(role.as_str())
    .execute(&state.db)
    .await
    .expect("grant role");
}

pub async fn connect(addr: SocketAddr) -> ClientSocket {
    let (socket, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/ws"))
        .await
        .expect("connect");
    socket
}

pub async fn send(socket: &mut ClientSocket, outgoing: &Outgoing) {
    let text = match outgoing {
        Outgoing::Frame(frame) => serde_json::to_string(frame).expect("serialize client frame"),
        Outgoing::Raw(text) => text.clone(),
    };
    socket
        .send(Message::Text(text.into()))
        .await
        .expect("send text");
}

pub async fn observe(socket: &mut ClientSocket) -> Observed {
    loop {
        match socket.next().await {
            Some(Ok(Message::Text(text))) => {
                return Observed::Frame(serde_json::from_str(text.as_str()).expect("server frame"));
            }
            Some(Ok(Message::Close(frame))) => {
                return Observed::Close(frame.map(|close| u16::from(close.code)).unwrap_or(1005));
            }
            Some(Ok(_)) => continue,
            Some(Err(_)) | None => return Observed::Close(1006),
        }
    }
}

pub const SCENARIO_DEADLINE: Duration = Duration::from_secs(60);
pub const AUTH_DEADLINE: Duration = Duration::from_secs(30);
pub const HANDSHAKE_DEADLINE: Duration = Duration::from_secs(10);
const OBSERVE_DEADLINE: Duration = Duration::from_secs(20);

pub async fn observe_until(
    socket: &mut ClientSocket,
    mut accept: impl FnMut(&Observed) -> bool,
) -> Observed {
    timeout(OBSERVE_DEADLINE, async {
        loop {
            let observed = observe(socket).await;
            if accept(&observed) || matches!(observed, Observed::Close(_)) {
                return observed;
            }
        }
    })
    .await
    .expect("observe deadline")
}

pub async fn expect_frame(socket: &mut ClientSocket) -> ServerFrame {
    match timeout(OBSERVE_DEADLINE, observe(socket))
        .await
        .expect("observe deadline")
    {
        Observed::Frame(frame) => frame,
        Observed::Close(code) => panic!("expected frame, socket closed with {code}"),
    }
}

pub async fn joined_socket(
    addr: SocketAddr,
    campus_id: CampusId,
    token: &AuthToken,
) -> (ClientSocket, Welcome) {
    let mut socket = connect(addr).await;
    send(
        &mut socket,
        &hello(campus_id, ProtocolVersion::CURRENT.0, token),
    )
    .await;
    match expect_frame(&mut socket).await {
        ServerFrame::Welcome(welcome) => (socket, welcome),
        other => panic!("expected welcome, got {other:?}"),
    }
}

pub async fn subscribed_socket(
    addr: SocketAddr,
    campus_id: CampusId,
    token: &AuthToken,
    rooms: &[RoomId],
) -> ClientSocket {
    let (mut socket, _) = joined_socket(addr, campus_id, token).await;
    let cursors = rooms
        .iter()
        .map(|room_id| (*room_id, None))
        .collect::<Vec<_>>();
    send(&mut socket, &sync_rooms(&cursors)).await;
    for _ in rooms {
        match expect_frame(&mut socket).await {
            ServerFrame::RoomSynced(_) => {}
            other => panic!("expected room sync, got {other:?}"),
        }
    }
    socket
}

pub async fn posted(
    socket: &mut ClientSocket,
    room_id: RoomId,
    client_nonce: ClientNonce,
    body: &str,
) -> RoomMessage {
    send(socket, &send_message(room_id, client_nonce, body)).await;
    let observed = observe_until(socket, |observed| {
        matches!(observed, Observed::Frame(ServerFrame::MessageAccepted(_)))
    })
    .await;
    match observed {
        Observed::Frame(ServerFrame::MessageAccepted(accepted)) => accepted.message,
        other => panic!("expected accepted, got {other:?}"),
    }
}

pub struct Campus {
    pub id: CampusId,
    pub rooms: Vec<RoomId>,
    pub owner: AuthSession,
}

pub async fn campus_with_owner(state: &AppState, email: &str, slug: &str) -> Campus {
    let owner = signed_up(state, email).await;
    let created = campus_for(state, &owner.token, slug).await;
    Campus {
        id: created.campus.campus_id,
        rooms: created.rooms.iter().map(|room| room.room_id).collect(),
        owner,
    }
}

pub async fn frames_until_pong(socket: &mut ClientSocket, nonce: u32) -> Vec<ServerFrame> {
    send(socket, &ping(nonce)).await;
    let mut frames = Vec::new();
    loop {
        let frame = expect_frame(socket).await;
        if frame == ServerFrame::Pong(Pong { nonce }) {
            return frames;
        }
        frames.push(frame);
    }
}

pub async fn stored_bodies(state: &AppState, campus_id: CampusId) -> Vec<String> {
    sqlx::query_scalar::<_, String>(
        "SELECT body FROM room_messages WHERE campus_id = $1 ORDER BY message_id",
    )
    .bind(campus_id.as_uuid())
    .fetch_all(&state.db)
    .await
    .expect("stored bodies")
}

pub async fn synced_page(
    socket: &mut ClientSocket,
    room_id: RoomId,
    after: Option<MessageId>,
) -> RoomSynced {
    send(socket, &sync_rooms(&[(room_id, after)])).await;
    match expect_frame(socket).await {
        ServerFrame::RoomSynced(page) => page,
        other => panic!("expected room sync, got {other:?}"),
    }
}

pub struct ValkeyProxy {
    pub addr: SocketAddr,
    links: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl ValkeyProxy {
    pub async fn start() -> Self {
        let valkey_url =
            std::env::var("VALKEY_URL").unwrap_or_else(|_| DEFAULT_VALKEY_URL.to_owned());
        let upstream = valkey_url
            .strip_prefix("redis://")
            .unwrap_or(valkey_url.as_str())
            .to_owned();
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind proxy");
        let addr = listener.local_addr().expect("proxy address");
        let links = Arc::new(Mutex::new(Vec::new()));
        let tracked = Arc::clone(&links);
        tokio::spawn(async move {
            while let Ok((mut client, _)) = listener.accept().await {
                let upstream = upstream.clone();
                let link = tokio::spawn(async move {
                    if let Ok(mut server) = TcpStream::connect(upstream).await {
                        let _ = tokio::io::copy_bidirectional(&mut client, &mut server).await;
                    }
                });
                tracked.lock().expect("links").push(link);
            }
        });
        Self { addr, links }
    }

    pub fn cut(&self) {
        self.links
            .lock()
            .expect("links")
            .drain(..)
            .for_each(|link| link.abort());
    }
}

pub fn proxied_state(state: &AppState, proxy: &ValkeyProxy) -> AppState {
    AppState::new(
        state.db.clone(),
        redis::Client::open(format!("redis://{}", proxy.addr)).expect("proxied valkey client"),
    )
}

pub fn assert_chain(messages: &[RoomMessage], start: Option<MessageId>) {
    let mut tip = start;
    for message in messages {
        assert_eq!(message.previous_message_id, tip);
        assert!(tip.is_none_or(|tip| message.message_id > tip));
        tip = Some(message.message_id);
    }
}

pub async fn created_messages(socket: &mut ClientSocket, count: usize) -> Vec<RoomMessage> {
    let mut messages = Vec::new();
    while messages.len() < count {
        match expect_frame(socket).await {
            ServerFrame::MessageCreated(created) => messages.push(created.message),
            other => panic!("expected created, got {other:?}"),
        }
    }
    messages
}

pub async fn seed_messages(
    state: &AppState,
    campus_id: CampusId,
    room_id: RoomId,
    author_id: AccountId,
    count: usize,
) -> Vec<MessageId> {
    let base = Uuid::now_v7().as_u128();
    let ids = (0..count)
        .map(|offset| {
            MessageId(Uuid::from_u128(
                base + u128::try_from(offset).expect("offset"),
            ))
        })
        .collect::<Vec<_>>();
    for (index, id) in ids.iter().enumerate() {
        let previous = index.checked_sub(1).map(|before| ids[before].as_uuid());
        sqlx::query(
            "INSERT INTO room_messages \
             (campus_id, room_id, message_id, previous_message_id, author_id, client_nonce, body) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(id.as_uuid())
        .bind(previous)
        .bind(author_id.as_uuid())
        .bind(ClientNonce::generate().as_uuid())
        .bind(format!("seed {index}"))
        .execute(&state.db)
        .await
        .expect("seed message");
    }
    ids
}

pub async fn message_count(state: &AppState, campus_id: CampusId) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT count(*) FROM room_messages WHERE campus_id = $1")
        .bind(campus_id.as_uuid())
        .fetch_one(&state.db)
        .await
        .expect("message count")
}

pub async fn member_session(
    state: &AppState,
    email: &str,
    campus_id: CampusId,
    role: Role,
) -> AuthSession {
    let session = signed_up(state, email).await;
    grant_role(state, campus_id, session.account.account_id, role).await;
    session
}

pub async fn api_get(
    state: &AppState,
    path: &str,
    token: Option<&AuthToken>,
) -> (StatusCode, Value) {
    api_raw(state, Method::GET, path, token, None).await
}

pub async fn sign_in_as(state: &AppState, email: &str) -> AuthSession {
    let body = serde_json::json!({"email": email, "password": VALID_PASSWORD}).to_string();
    let (status, body) =
        api_raw(state, Method::POST, api_path::AUTH_TOKENS, None, Some(body)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    serde_json::from_value(body).expect("auth session")
}

pub async fn campuses_of(state: &AppState, token: &AuthToken) -> CampusList {
    let (status, body) = api_get(state, api_path::CAMPUSES, Some(token)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    serde_json::from_value(body).expect("campus list")
}

pub async fn rooms_of(state: &AppState, token: &AuthToken, campus_id: CampusId) -> RoomList {
    let (status, body) = api_get(state, &api_path::campus_rooms(campus_id), Some(token)).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    serde_json::from_value(body).expect("room list")
}

pub async fn tally_race(
    mut attempts: JoinSet<(StatusCode, Value)>,
    conflict_body: Value,
) -> (usize, usize) {
    let mut created = 0;
    let mut conflicts = 0;
    while let Some(done) = attempts.join_next().await {
        let (status, body) = done.expect("attempt task");
        match status {
            StatusCode::CREATED => created += 1,
            StatusCode::CONFLICT => {
                conflicts += 1;
                assert_eq!(body, conflict_body);
            }
            other => panic!("unexpected status {other}: {body}"),
        }
    }
    (created, conflicts)
}

pub async fn directory_counts(state: &AppState) -> (i64, i64, i64) {
    sqlx::query_as::<_, (i64, i64, i64)>(
        "SELECT (SELECT count(*) FROM campuses), \
                (SELECT count(*) FROM campus_members), \
                (SELECT count(*) FROM rooms)",
    )
    .fetch_one(&state.db)
    .await
    .expect("directory counts")
}

pub async fn expire_token(state: &AppState, token: &AuthToken) {
    let affected = sqlx::query(
        "UPDATE auth_tokens SET expires_at = now() - interval '1 second' WHERE token_hash = $1",
    )
    .bind(token_digest(token))
    .execute(&state.db)
    .await
    .expect("expire token")
    .rows_affected();
    assert_eq!(affected, 1);
}

pub async fn handshake_outcome(
    addr: SocketAddr,
    campus_id: CampusId,
    token: &AuthToken,
) -> Observed {
    let mut socket = connect(addr).await;
    send(
        &mut socket,
        &hello(campus_id, ProtocolVersion::CURRENT.0, token),
    )
    .await;
    observe(&mut socket).await
}

pub async fn next_of<T>(
    socket: &mut ClientSocket,
    mut pick: impl FnMut(&ServerFrame) -> Option<T>,
) -> T {
    timeout(OBSERVE_DEADLINE, async {
        loop {
            match observe(socket).await {
                Observed::Frame(frame) => {
                    if let Some(found) = pick(&frame) {
                        return found;
                    }
                }
                Observed::Close(code) => panic!("socket closed with {code}"),
            }
        }
    })
    .await
    .expect("observe deadline")
}

pub async fn replied(
    socket: &mut ClientSocket,
    room_id: RoomId,
    root: MessageId,
    client_nonce: ClientNonce,
    body: &str,
) -> RoomMessage {
    send(socket, &send_reply(room_id, root, client_nonce, body)).await;
    next_of(socket, |frame| match frame {
        ServerFrame::MessageAccepted(accepted) if accepted.message.client_nonce == client_nonce => {
            Some(accepted.message.clone())
        }
        _ => None,
    })
    .await
}

pub async fn inbox_of(socket: &mut ClientSocket) -> Vec<ThreadSummary> {
    send(socket, &sync_inbox()).await;
    let InboxSynced { threads } = next_of(socket, |frame| match frame {
        ServerFrame::InboxSynced(synced) => Some(synced.clone()),
        _ => None,
    })
    .await;
    threads
}

pub async fn thread_page(
    socket: &mut ClientSocket,
    room_id: RoomId,
    root: MessageId,
    after: Option<MessageId>,
) -> ThreadSynced {
    send(socket, &sync_thread(room_id, root, after)).await;
    next_of(socket, |frame| match frame {
        ServerFrame::ThreadSynced(synced) => Some(synced.clone()),
        _ => None,
    })
    .await
}

pub async fn synced_page_at(
    socket: &mut ClientSocket,
    room_id: RoomId,
    after: Option<MessageId>,
    activity_after: Option<u64>,
) -> RoomSynced {
    send(socket, &sync_rooms_at(&[(room_id, after, activity_after)])).await;
    next_of(socket, |frame| match frame {
        ServerFrame::RoomSynced(page) if page.room_id == room_id => Some(page.clone()),
        _ => None,
    })
    .await
}

pub async fn frames_through_created(
    socket: &mut ClientSocket,
    client_nonce: ClientNonce,
) -> Vec<ServerFrame> {
    let mut frames = Vec::new();
    loop {
        let frame = expect_frame(socket).await;
        let done = matches!(
            &frame,
            ServerFrame::MessageCreated(created) if created.message.client_nonce == client_nonce
        );
        frames.push(frame);
        if done {
            return frames;
        }
    }
}

pub async fn sentinel_frames(
    socket: &mut ClientSocket,
    poster: &mut ClientSocket,
    room_id: RoomId,
) -> Vec<ServerFrame> {
    let nonce = ClientNonce::generate();
    posted(poster, room_id, nonce, "sentinel").await;
    frames_through_created(socket, nonce).await
}

pub struct Stage {
    pub state: AppState,
    pub addr: SocketAddr,
    pub campus: Campus,
    pub room: RoomId,
    pub ada: AuthSession,
    pub grace: AuthSession,
    pub linus: AuthSession,
    pub guest: AuthSession,
}

impl Stage {
    pub async fn open() -> Self {
        let state = fresh_state().await;
        let campus = campus_with_owner(&state, "owner@example.org", "north-quad").await;
        let ada = member_session(&state, "ada@example.org", campus.id, Role::Member).await;
        let grace = member_session(&state, "grace@example.org", campus.id, Role::Member).await;
        let linus = member_session(&state, "linus@example.org", campus.id, Role::Member).await;
        let guest = member_session(&state, "guest@example.org", campus.id, Role::Guest).await;
        let addr = spawn_server_with(state.clone()).await;
        let room = campus.rooms[0];
        Self {
            state,
            addr,
            campus,
            room,
            ada,
            grace,
            linus,
            guest,
        }
    }

    pub async fn socket(&self, session: &AuthSession) -> ClientSocket {
        subscribed_socket(self.addr, self.campus.id, &session.token, &[self.room]).await
    }

    pub async fn root(&self, socket: &mut ClientSocket, body: &str) -> RoomMessage {
        posted(socket, self.room, ClientNonce::generate(), body).await
    }

    pub fn account(session: &AuthSession) -> AccountId {
        session.account.account_id
    }

    pub async fn subscription(
        &self,
        root: MessageId,
        session: &AuthSession,
    ) -> Option<(String, Option<MessageId>, i64)> {
        subscription_row(&self.state, self.campus.id, root, Self::account(session)).await
    }
}

pub async fn subscription_row(
    state: &AppState,
    campus_id: CampusId,
    root: MessageId,
    account_id: AccountId,
) -> Option<(String, Option<MessageId>, i64)> {
    sqlx::query_as::<_, (String, Option<Uuid>, i64)>(
        "SELECT state, read_through, revision FROM thread_subscriptions \
         WHERE campus_id = $1 AND root_message_id = $2 AND account_id = $3",
    )
    .bind(campus_id.as_uuid())
    .bind(root.as_uuid())
    .bind(account_id.as_uuid())
    .fetch_optional(&state.db)
    .await
    .expect("subscription row")
    .map(|(state, read, revision)| (state, read.map(MessageId), revision))
}

pub async fn count_rows(state: &AppState, table: &'static str, campus_id: CampusId) -> i64 {
    sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(format!(
        "SELECT count(*) FROM {table} WHERE campus_id = $1"
    )))
    .bind(campus_id.as_uuid())
    .fetch_one(&state.db)
    .await
    .expect("row count")
}

pub async fn reaction_rows(
    state: &AppState,
    campus_id: CampusId,
    message_id: MessageId,
) -> Vec<(String, AccountId)> {
    sqlx::query_as::<_, (String, Uuid)>(
        "SELECT emoji, account_id FROM message_reactions \
         WHERE campus_id = $1 AND message_id = $2 ORDER BY emoji, account_id",
    )
    .bind(campus_id.as_uuid())
    .bind(message_id.as_uuid())
    .fetch_all(&state.db)
    .await
    .expect("reaction rows")
    .into_iter()
    .map(|(emoji, account_id)| (emoji, AccountId(account_id)))
    .collect()
}

pub async fn mention_rows(
    state: &AppState,
    campus_id: CampusId,
    message_id: MessageId,
) -> Vec<AccountId> {
    sqlx::query_scalar::<_, Uuid>(
        "SELECT account_id FROM message_mentions \
         WHERE campus_id = $1 AND message_id = $2 ORDER BY account_id",
    )
    .bind(campus_id.as_uuid())
    .bind(message_id.as_uuid())
    .fetch_all(&state.db)
    .await
    .expect("mention rows")
    .into_iter()
    .map(AccountId)
    .collect()
}

pub async fn room_activity(state: &AppState, campus_id: CampusId, room_id: RoomId) -> i64 {
    sqlx::query_scalar::<_, i64>(
        "SELECT activity_seq FROM rooms WHERE campus_id = $1 AND room_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .fetch_one(&state.db)
    .await
    .expect("room activity")
}

pub fn summary_of(summaries: &[ThreadSummary], root: MessageId) -> &ThreadSummary {
    summaries
        .iter()
        .find(|summary| summary.root.message_id == root)
        .expect("summary for root")
}

pub fn changed_summaries(frames: &[ServerFrame]) -> Vec<ThreadSummary> {
    frames
        .iter()
        .filter_map(|frame| match frame {
            ServerFrame::ThreadChanged(changed) => Some(changed.summary.clone()),
            _ => None,
        })
        .collect()
}

pub async fn seed_replies(
    state: &AppState,
    campus_id: CampusId,
    room_id: RoomId,
    root: MessageId,
    author_id: AccountId,
    tip: MessageId,
    count: usize,
) -> Vec<MessageId> {
    let mut previous = tip;
    let mut ids = Vec::new();
    for index in 0..count {
        let id = MessageId(Uuid::from_u128(previous.as_uuid().as_u128() + 1));
        sqlx::query(
            "INSERT INTO room_messages \
             (campus_id, room_id, message_id, previous_message_id, author_id, client_nonce, body, \
             thread_root_id) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(id.as_uuid())
        .bind(previous.as_uuid())
        .bind(author_id.as_uuid())
        .bind(ClientNonce::generate().as_uuid())
        .bind(format!("seeded reply {index}"))
        .bind(root.as_uuid())
        .execute(&state.db)
        .await
        .expect("seed reply");
        ids.push(id);
        previous = id;
    }
    ids
}

pub async fn title_outcome(socket: &mut ClientSocket) -> ServerFrame {
    next_of(socket, |frame| match frame {
        ServerFrame::ThreadRejected(_) | ServerFrame::MessageUpdated(_) => Some(frame.clone()),
        _ => None,
    })
    .await
}

pub async fn reaction_outcome(socket: &mut ClientSocket) -> ServerFrame {
    next_of(socket, |frame| match frame {
        ServerFrame::ReactionRejected(_) | ServerFrame::MessageUpdated(_) => Some(frame.clone()),
        _ => None,
    })
    .await
}

pub async fn plain_socket(stage: &Stage, session: &AuthSession) -> ClientSocket {
    joined_socket(stage.addr, stage.campus.id, &session.token)
        .await
        .0
}

pub async fn settled_frames(
    socket: &mut ClientSocket,
    poster: &mut ClientSocket,
    room_id: RoomId,
) -> Vec<ServerFrame> {
    let mut frames = frames_until_pong(socket, 9).await;
    frames.extend(sentinel_frames(socket, poster, room_id).await);
    frames
}

pub fn rejections_of(frames: &[ServerFrame]) -> Vec<pulse_protocol::ThreadRejection> {
    frames
        .iter()
        .filter_map(|frame| match frame {
            ServerFrame::ThreadRejected(rejected) => Some(rejected.reason),
            _ => None,
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Person {
    Ada,
    Grace,
    Linus,
    Guest,
}

impl Stage {
    pub fn session(&self, person: Person) -> &AuthSession {
        match person {
            Person::Ada => &self.ada,
            Person::Grace => &self.grace,
            Person::Linus => &self.linus,
            Person::Guest => &self.guest,
        }
    }

    pub fn id_of(&self, person: Person) -> AccountId {
        Self::account(self.session(person))
    }

    pub async fn plain(&self, person: Person) -> ClientSocket {
        plain_socket(self, self.session(person)).await
    }

    pub async fn watching(&self, person: Person) -> ClientSocket {
        self.socket(self.session(person)).await
    }

    pub async fn poster(&self) -> ClientSocket {
        plain_socket(self, &self.campus.owner).await
    }

    pub async fn row_of(
        &self,
        root: MessageId,
        person: Person,
    ) -> Option<(String, Option<MessageId>, i64)> {
        self.subscription(root, self.session(person)).await
    }
}

impl Stage {
    pub async fn root_in(
        &self,
        socket: &mut ClientSocket,
        room_id: RoomId,
        body: &str,
    ) -> RoomMessage {
        posted(socket, room_id, ClientNonce::generate(), body).await
    }
}

pub async fn settle_all(
    sockets: &mut [ClientSocket],
    poster: &mut ClientSocket,
    room_id: RoomId,
) -> Vec<Vec<ServerFrame>> {
    let mut frames = Vec::new();
    for socket in sockets.iter_mut() {
        frames.push(frames_until_pong(socket, 11).await);
    }
    let nonce = ClientNonce::generate();
    posted(poster, room_id, nonce, "sentinel").await;
    for (socket, seen) in sockets.iter_mut().zip(frames.iter_mut()) {
        seen.extend(frames_through_created(socket, nonce).await);
    }
    frames
}

pub async fn extra_members(
    state: &AppState,
    campus_id: CampusId,
    count: usize,
) -> Vec<AuthSession> {
    let mut sessions = Vec::new();
    for index in 0..count {
        sessions.push(
            member_session(
                state,
                &format!("extra-{index}@example.org"),
                campus_id,
                Role::Member,
            )
            .await,
        );
    }
    sessions
}

#[derive(Clone, Copy, Debug)]
pub enum Aim {
    Top,
    Reply,
    Unknown,
    Sibling,
}

pub fn aimed(aim: Aim, top: MessageId, reply: MessageId, sibling: MessageId) -> MessageId {
    match aim {
        Aim::Top => top,
        Aim::Reply => reply,
        Aim::Unknown => MessageId::generate(),
        Aim::Sibling => sibling,
    }
}

pub struct Roots {
    pub top: MessageId,
    pub reply: MessageId,
    pub sibling: MessageId,
}

impl Roots {
    pub fn pick(&self, aim: Aim) -> MessageId {
        aimed(aim, self.top, self.reply, self.sibling)
    }
}

impl Stage {
    pub async fn roots(&self) -> Roots {
        let mut ada = self.plain(Person::Ada).await;
        let top = self.root(&mut ada, "root").await.message_id;
        let reply = replied(&mut ada, self.room, top, ClientNonce::generate(), "reply")
            .await
            .message_id;
        let sibling = self
            .root_in(&mut ada, self.campus.rooms[1], "elsewhere")
            .await
            .message_id;
        Roots {
            top,
            reply,
            sibling,
        }
    }
}

pub use pulse_server::time::now_ms;

pub fn ms_from_now(offset_ms: i64) -> u64 {
    now_ms().saturating_add_signed(offset_ms)
}

pub async fn storage_ready(state: &AppState) {
    state
        .storage
        .ensure_bucket()
        .await
        .expect("start MinIO: docker start pulse-dev-minio");
}

pub async fn request_upload(
    state: &AppState,
    token: &AuthToken,
    campus_id: CampusId,
    room_id: RoomId,
    request: &CreateUploadRequest,
) -> (StatusCode, Value) {
    api(
        state,
        Method::POST,
        &api_path::campus_room_uploads(campus_id, room_id),
        Some(token),
        Some(request),
    )
    .await
}

pub fn upload_request(file_name: &str, content_type: &str, byte_size: u64) -> CreateUploadRequest {
    CreateUploadRequest {
        file_name: file_name.to_owned(),
        content_type: content_type.to_owned(),
        byte_size,
    }
}

pub async fn put_bytes(url: &str, content_type: &str, bytes: Vec<u8>) -> StatusCode {
    let response = timeout(
        HANDSHAKE_DEADLINE,
        reqwest::Client::new()
            .put(url)
            .header(header::CONTENT_TYPE, content_type)
            .body(bytes)
            .send(),
    )
    .await
    .expect("put deadline")
    .expect("put request");
    response.status()
}

pub async fn get_bytes(url: &str) -> (StatusCode, Vec<u8>) {
    let response = timeout(HANDSHAKE_DEADLINE, reqwest::get(url))
        .await
        .expect("get deadline")
        .expect("get request");
    let status = response.status();
    (status, response.bytes().await.expect("get body").to_vec())
}

pub async fn uploaded(
    state: &AppState,
    session: &AuthSession,
    campus_id: CampusId,
    room_id: RoomId,
    file_name: &str,
    content_type: &str,
    bytes: &[u8],
) -> UploadTicket {
    let request = upload_request(file_name, content_type, bytes.len() as u64);
    let (status, body) = request_upload(state, &session.token, campus_id, room_id, &request).await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    let ticket: UploadTicket = serde_json::from_value(body).expect("upload ticket");
    assert_eq!(
        put_bytes(&ticket.put_url, &ticket.content_type, bytes.to_vec()).await,
        StatusCode::OK
    );
    ticket
}

pub async fn download_tickets_of(
    state: &AppState,
    token: &AuthToken,
    campus_id: CampusId,
    ids: Vec<AttachmentId>,
) -> (StatusCode, Value) {
    api(
        state,
        Method::POST,
        &api_path::campus_downloads(campus_id),
        Some(token),
        Some(&DownloadTicketsRequest {
            attachment_ids: ids,
        }),
    )
    .await
}

pub fn parsed_tickets(body: Value) -> DownloadTickets {
    serde_json::from_value(body).expect("download tickets")
}

pub fn api_error_of(body: Value) -> ApiError {
    serde_json::from_value(body).expect("api error")
}

pub fn send_attached(
    room_id: RoomId,
    client_nonce: ClientNonce,
    body: &str,
    attachment_ids: &[AttachmentId],
) -> Outgoing {
    Outgoing::Frame(ClientFrame::SendMessage(SendMessage {
        room_id,
        client_nonce,
        body: body.to_owned(),
        thread_root_id: None,
        attachment_ids: attachment_ids.to_vec(),
    }))
}

pub async fn send_outcome(
    socket: &mut ClientSocket,
    room_id: RoomId,
    body: &str,
    attachment_ids: &[AttachmentId],
) -> ServerFrame {
    let nonce = ClientNonce::generate();
    send(socket, &send_attached(room_id, nonce, body, attachment_ids)).await;
    match observe_until(socket, |observed| {
        matches!(
            observed,
            Observed::Frame(ServerFrame::MessageAccepted(_) | ServerFrame::MessageRejected(_))
        )
    })
    .await
    {
        Observed::Frame(frame) => frame,
        Observed::Close(code) => panic!("socket closed with {code}"),
    }
}

pub fn pin_frame(room_id: RoomId, message_id: MessageId, pinned: bool) -> Outgoing {
    Outgoing::Frame(ClientFrame::SetPin(SetPin {
        room_id,
        message_id,
        pinned,
    }))
}

pub fn vault_frame(room_id: RoomId, tab: VaultTab, before: Option<MessageId>) -> Outgoing {
    Outgoing::Frame(ClientFrame::SyncVault(SyncVault {
        room_id,
        tab,
        before,
    }))
}

pub fn shelf_update_request(
    room_id: RoomId,
    message_id: MessageId,
    action: ShelfAction,
    remind_at_ms: Option<u64>,
) -> UpdateShelf {
    UpdateShelf {
        room_id,
        message_id,
        action,
        remind_at_ms,
    }
}

pub fn shelf_frame(
    room_id: RoomId,
    message_id: MessageId,
    action: ShelfAction,
    remind_at_ms: Option<u64>,
) -> Outgoing {
    Outgoing::Frame(ClientFrame::UpdateShelf(shelf_update_request(
        room_id,
        message_id,
        action,
        remind_at_ms,
    )))
}

pub fn sync_shelf_frame() -> Outgoing {
    Outgoing::Frame(ClientFrame::SyncShelf(SyncShelf {}))
}

pub async fn seed_attachments(
    state: &AppState,
    campus_id: CampusId,
    room_id: RoomId,
    uploader_id: AccountId,
    placements: &[(MessageId, &str, &str)],
) {
    for (message_id, content_type, kind) in placements {
        let attachment_id = AttachmentId::generate();
        sqlx::query(
            "INSERT INTO attachments \
             (campus_id, room_id, attachment_id, uploader_id, object_key, file_name, \
             content_type, kind, byte_size, message_id, position) \
             VALUES ($1, $2, $3, $4, 'campuses/' || $1::text || '/rooms/' || $2::text \
             || '/attachments/' || $3::text, 'seed', $5, $6, 10, $7, 0)",
        )
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(attachment_id.as_uuid())
        .bind(uploader_id.as_uuid())
        .bind(content_type)
        .bind(kind)
        .bind(message_id.as_uuid())
        .execute(&state.db)
        .await
        .expect("seed attachment");
    }
}

pub async fn seed_links(
    state: &AppState,
    campus_id: CampusId,
    room_id: RoomId,
    message_ids: &[MessageId],
) {
    for message_id in message_ids {
        sqlx::query(
            "INSERT INTO message_links (campus_id, room_id, message_id, position, url) \
             VALUES ($1, $2, $3, 0, 'https://example.org/seed')",
        )
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(message_id.as_uuid())
        .execute(&state.db)
        .await
        .expect("seed link");
    }
}

pub async fn seed_pins(
    state: &AppState,
    campus_id: CampusId,
    room_id: RoomId,
    pinned_by: AccountId,
    message_ids: &[MessageId],
) {
    for message_id in message_ids {
        sqlx::query(
            "INSERT INTO message_pins (campus_id, room_id, message_id, pinned_by) \
             VALUES ($1, $2, $3, $4)",
        )
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(message_id.as_uuid())
        .bind(pinned_by.as_uuid())
        .execute(&state.db)
        .await
        .expect("seed pin");
    }
}

pub async fn seed_shelf(
    state: &AppState,
    campus_id: CampusId,
    room_id: RoomId,
    account_id: AccountId,
    message_ids: &[MessageId],
    remind_offset_ms: Option<i64>,
) {
    for message_id in message_ids {
        sqlx::query(
            "INSERT INTO shelf_items \
             (campus_id, account_id, shelf_item_id, room_id, message_id, remind_at) \
             VALUES ($1, $2, $3, $4, $5, \
             CASE WHEN $6::bigint IS NULL THEN NULL \
             ELSE now() + ($6::bigint || ' milliseconds')::interval END)",
        )
        .bind(campus_id.as_uuid())
        .bind(account_id.as_uuid())
        .bind(Uuid::now_v7())
        .bind(room_id.as_uuid())
        .bind(message_id.as_uuid())
        .bind(remind_offset_ms)
        .execute(&state.db)
        .await
        .expect("seed shelf item");
    }
}

pub async fn pin_count(state: &AppState, campus_id: CampusId, room_id: RoomId) -> i64 {
    sqlx::query_scalar::<_, i64>(
        "SELECT count(*) FROM message_pins WHERE campus_id = $1 AND room_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .fetch_one(&state.db)
    .await
    .expect("pin count")
}

pub async fn shelf_state_of(
    state: &AppState,
    campus_id: CampusId,
    account_id: AccountId,
    message_id: MessageId,
) -> Option<(bool, bool, bool, i64)> {
    sqlx::query_as::<_, (bool, bool, bool, i64)>(
        "SELECT remind_at IS NOT NULL, reminded_at IS NOT NULL, done_at IS NOT NULL, revision \
         FROM shelf_items WHERE campus_id = $1 AND account_id = $2 AND message_id = $3",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(message_id.as_uuid())
    .fetch_optional(&state.db)
    .await
    .expect("shelf state")
}

pub async fn attachment_binding(
    state: &AppState,
    campus_id: CampusId,
    attachment_id: AttachmentId,
) -> Option<(Option<MessageId>, Option<i16>)> {
    sqlx::query_as::<_, (Option<Uuid>, Option<i16>)>(
        "SELECT message_id, position FROM attachments WHERE campus_id = $1 AND attachment_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(attachment_id.as_uuid())
    .fetch_optional(&state.db)
    .await
    .expect("attachment binding")
    .map(|(message_id, position)| (message_id.map(MessageId), position))
}

pub async fn collect_until<T>(
    socket: &mut ClientSocket,
    mut pick: impl FnMut(ServerFrame) -> Option<T>,
    wanted: usize,
) -> Vec<T> {
    let mut found = Vec::new();
    while found.len() < wanted {
        if let Some(item) = pick(expect_frame(socket).await) {
            found.push(item);
        }
    }
    found
}

pub async fn object_key_of(
    state: &AppState,
    campus_id: CampusId,
    attachment_id: AttachmentId,
) -> String {
    sqlx::query_scalar::<_, String>(
        "SELECT object_key FROM attachments WHERE campus_id = $1 AND attachment_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(attachment_id.as_uuid())
    .fetch_one(&state.db)
    .await
    .expect("object key")
}

pub async fn bound_attachment_count(state: &AppState, campus_id: CampusId) -> i64 {
    sqlx::query_scalar::<_, i64>(
        "SELECT count(*) FROM attachments WHERE campus_id = $1 AND message_id IS NOT NULL",
    )
    .bind(campus_id.as_uuid())
    .fetch_one(&state.db)
    .await
    .expect("bound count")
}
