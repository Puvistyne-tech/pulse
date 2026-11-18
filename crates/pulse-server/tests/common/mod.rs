#![allow(dead_code)]

use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use futures_util::{SinkExt, StreamExt};
use http_body_util::BodyExt;
use pulse_protocol::fixtures::VALID_PASSWORD;
use pulse_protocol::{
    AccountId, AuthSession, AuthToken, CampusCreated, CampusId, CampusList, ClientFrame,
    CreateCampusRequest, Hello, Ping, ProtocolVersion, Role, RoomList, ServerFrame, SignUpRequest,
    api_path,
};
use pulse_server::auth::token_digest;
use pulse_server::{AppState, Config, MIGRATOR, router, state};
use serde::Serialize;
use serde_json::Value;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinSet;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tower::ServiceExt;
use uuid::Uuid;

const DEFAULT_DATABASE_URL: &str = "postgres://pulse:pulse@127.0.0.1:5432/pulse";
const DEFAULT_VALKEY_URL: &str = "redis://127.0.0.1:6379";
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(5);
const START_POSTGRES: &str = "start Postgres: docker compose up -d postgres";

pub type ClientSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, PartialEq)]
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
    AppState {
        db,
        valkey: redis::Client::open(valkey_url.as_str()).expect("valkey client"),
    }
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
