mod fixtures;

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use fixtures::{
    ACCOUNT_A, CAMPUS_A, CAMPUS_B, ROOM_A, ROOM_B, auth_session, campus_list, campus_summary,
    confirmed_key, entry_keys, hello_frame, message_id, pending_key, room_message, send_frame,
    sign_in_request, sync_frame, token_a, welcome_for,
};
use futures_util::{SinkExt, StreamExt};
use pulse_core::native::{NativeClient, websocket_url};
use pulse_protocol::{
    CampusId, ClientFrame, ClientSnapshot, ConnectionPhase, Role, Room, RoomList, ServerFrame,
    api_path,
};
use rstest::rstest;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

const DEADLINE: Duration = Duration::from_secs(20);
const SOCKET_PEEK_BYTES: usize = 8;

type Observed = (usize, ClientFrame);

struct Reaction {
    replies: Vec<ServerFrame>,
    close: bool,
}

fn react(connection: usize, frame: &ClientFrame) -> Reaction {
    match frame {
        ClientFrame::Hello(hello) => Reaction {
            replies: vec![ServerFrame::Welcome(welcome_for(hello.campus_id))],
            close: false,
        },
        ClientFrame::SendMessage(_) if connection == 0 => Reaction {
            replies: vec![],
            close: true,
        },
        ClientFrame::SendMessage(request) => Reaction {
            replies: vec![fixtures::accepted_frame(room_message(
                request.room_id,
                message_id(1),
                None,
                ACCOUNT_A,
                request.client_nonce,
                &request.body,
            ))],
            close: false,
        },
        ClientFrame::Ping(_) | ClientFrame::SyncRooms(_) => Reaction {
            replies: vec![],
            close: false,
        },
    }
}

fn rooms_of(campus_id: CampusId) -> RoomList {
    let ids = if campus_id == CAMPUS_B {
        vec![(ROOM_B, "annex")]
    } else {
        vec![(ROOM_A, "commons"), (ROOM_B, "courtyard")]
    };
    RoomList {
        campus_id,
        rooms: ids
            .into_iter()
            .map(|(room_id, slug)| Room {
                room_id,
                campus_id,
                slug: slug.to_owned(),
            })
            .collect(),
    }
}

fn http_response(status: &str, body: &[u8]) -> Vec<u8> {
    let mut response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    response.extend_from_slice(body);
    response
}

fn route(request_line: &str) -> Vec<u8> {
    let mut parts = request_line.split_whitespace();
    let (method, path) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let json =
        |value: Result<Vec<u8>, serde_json::Error>| http_response("200 OK", &value.expect("json"));
    match (method, path) {
        ("POST", api_path::AUTH_TOKENS) => json(serde_json::to_vec(&auth_session())),
        ("GET", api_path::CAMPUSES) => json(serde_json::to_vec(&campus_list(vec![
            campus_summary(CAMPUS_A, "alpha", Role::Member),
            campus_summary(CAMPUS_B, "bravo", Role::Member),
        ]))),
        ("GET", rooms_path) => {
            let campus = [CAMPUS_A, CAMPUS_B]
                .into_iter()
                .find(|campus_id| api_path::campus_rooms(*campus_id) == rooms_path);
            match campus {
                Some(campus_id) => json(serde_json::to_vec(&rooms_of(campus_id))),
                None => http_response("404 Not Found", br#"{"code":"not_found","field":null}"#),
            }
        }
        _ => http_response("404 Not Found", br#"{"code":"not_found","field":null}"#),
    }
}

fn header_end(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|position| position + 4)
}

fn content_length(head: &str) -> usize {
    head.lines()
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, value)| value.trim().parse().ok())
        .unwrap_or(0)
}

async fn serve_http(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 2048];
    let head = loop {
        let Ok(read) = stream.read(&mut chunk).await else {
            return;
        };
        if read == 0 {
            return;
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(end) = header_end(&buffer) {
            let head = String::from_utf8_lossy(&buffer[..end]).into_owned();
            if buffer.len() >= end + content_length(&head) {
                break head;
            }
        }
    };
    let request_line = head.lines().next().unwrap_or("").to_owned();
    let _ = stream.write_all(&route(&request_line)).await;
    let _ = stream.shutdown().await;
}

async fn serve_socket(stream: TcpStream, connection: usize, observed: UnboundedSender<Observed>) {
    let Ok(mut socket) = accept_async(stream).await else {
        return;
    };
    while let Some(Ok(message)) = socket.next().await {
        let Message::Text(text) = message else {
            continue;
        };
        let frame = serde_json::from_str::<ClientFrame>(text.as_str()).expect("client frame");
        let reaction = react(connection, &frame);
        let _ = observed.send((connection, frame));
        for reply in reaction.replies {
            let text = serde_json::to_string(&reply).expect("server frame");
            if socket.send(Message::text(text)).await.is_err() {
                return;
            }
        }
        if reaction.close {
            let _ = socket.close(None).await;
            return;
        }
    }
}

async fn is_websocket(stream: &TcpStream) -> bool {
    let mut head = [0u8; SOCKET_PEEK_BYTES];
    loop {
        match stream.peek(&mut head).await {
            Ok(read) if read >= SOCKET_PEEK_BYTES => {
                return head.starts_with(format!("GET {} ", api_path::WEBSOCKET).as_bytes());
            }
            Ok(0) | Err(_) => return false,
            Ok(_) => tokio::time::sleep(Duration::from_millis(5)).await,
        }
    }
}

struct FakeServer {
    address: SocketAddr,
    observed: UnboundedReceiver<Observed>,
}

impl FakeServer {
    async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let address = listener.local_addr().expect("address");
        let (sender, observed) = unbounded_channel();
        let connections = Arc::new(AtomicUsize::new(0));
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let sender = sender.clone();
                let connections = Arc::clone(&connections);
                tokio::spawn(async move {
                    if is_websocket(&stream).await {
                        let connection = connections.fetch_add(1, Ordering::SeqCst);
                        serve_socket(stream, connection, sender).await;
                    } else {
                        serve_http(stream).await;
                    }
                });
            }
        });
        Self { address, observed }
    }

    fn url(&self) -> String {
        format!("http://{}", self.address)
    }

    async fn next_frame(&mut self) -> Observed {
        within(self.observed.recv()).await.expect("frame")
    }
}

async fn within<T>(future: impl Future<Output = T>) -> T {
    tokio::time::timeout(DEADLINE, future)
        .await
        .expect("deadline exceeded")
}

fn watch(client: &NativeClient) -> UnboundedReceiver<ClientSnapshot> {
    let (sender, receiver) = unbounded_channel();
    client.forward_snapshots(move |snapshot| sender.send(snapshot).is_ok());
    receiver
}

async fn snapshot_where(
    receiver: &mut UnboundedReceiver<ClientSnapshot>,
    predicate: impl Fn(&ClientSnapshot) -> bool,
) -> ClientSnapshot {
    within(async {
        loop {
            let snapshot = receiver.recv().await.expect("snapshot stream");
            if predicate(&snapshot) {
                return snapshot;
            }
        }
    })
    .await
}

#[rstest]
#[case::http("http://127.0.0.1:8080", "ws://127.0.0.1:8080/ws")]
#[case::https("https://pulse.example.org", "wss://pulse.example.org/ws")]
#[case::trailing_slash("http://localhost:3000/", "ws://localhost:3000/ws")]
#[case::many_trailing_slashes("http://localhost:3000///", "ws://localhost:3000/ws")]
#[case::bare_host("localhost:9000", "localhost:9000/ws")]
fn websocket_url_follows_server_scheme(#[case] server_url: &str, #[case] expected: &str) {
    assert_eq!(websocket_url(server_url), expected);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn link_completes_handshake_then_resends_pending_nonce_after_drop() {
    let mut server = FakeServer::start().await;
    let client = NativeClient::new(server.url()).expect("client");
    let mut snapshots = watch(&client);
    let signed_in = within(client.sign_in(sign_in_request())).await;
    assert_eq!(signed_in.session.active_campus_id, Some(CAMPUS_A));
    assert_eq!(signed_in.session.active_room_id, Some(ROOM_A));
    let all_rooms = sync_frame(&[(ROOM_A, None), (ROOM_B, None)]);
    assert_eq!(
        server.next_frame().await,
        (0, hello_frame(CAMPUS_A, token_a()))
    );
    assert_eq!(server.next_frame().await, (0, all_rooms.clone()));
    snapshot_where(&mut snapshots, |snapshot| {
        snapshot.network.connection == ConnectionPhase::Connected
    })
    .await;
    let sent = within(client.send_message(ROOM_A, "hello".to_owned())).await;
    let client_nonce = sent.timeline.entries[0].client_nonce;
    assert_eq!(entry_keys(&sent.timeline), vec![pending_key(client_nonce)]);
    assert_eq!(
        server.next_frame().await,
        (0, send_frame(ROOM_A, client_nonce, "hello"))
    );
    assert_eq!(
        server.next_frame().await,
        (1, hello_frame(CAMPUS_A, token_a()))
    );
    assert_eq!(server.next_frame().await, (1, all_rooms));
    assert_eq!(
        server.next_frame().await,
        (1, send_frame(ROOM_A, client_nonce, "hello"))
    );
    let confirmed = room_message(
        ROOM_A,
        message_id(1),
        None,
        ACCOUNT_A,
        client_nonce,
        "hello",
    );
    let settled = snapshot_where(&mut snapshots, |snapshot| {
        snapshot.timeline.entries.len() == 1 && snapshot.timeline.entries[0].message_id.is_some()
    })
    .await;
    assert_eq!(
        entry_keys(&settled.timeline),
        vec![confirmed_key(&confirmed)]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn campus_switch_replaces_the_link_and_resets_timeline() {
    let mut server = FakeServer::start().await;
    let client = NativeClient::new(server.url()).expect("client");
    let mut snapshots = watch(&client);
    within(client.sign_in(sign_in_request())).await;
    assert_eq!(
        server.next_frame().await,
        (0, hello_frame(CAMPUS_A, token_a()))
    );
    snapshot_where(&mut snapshots, |snapshot| {
        snapshot.network.connection == ConnectionPhase::Connected
            && snapshot.network.campus_id == Some(CAMPUS_A)
    })
    .await;
    assert_eq!(
        server.next_frame().await,
        (0, sync_frame(&[(ROOM_A, None), (ROOM_B, None)]))
    );
    let switched = within(client.select_campus(CAMPUS_B)).await;
    assert_eq!(switched.session.active_campus_id, Some(CAMPUS_B));
    assert!(switched.timeline.entries.is_empty());
    assert_eq!(
        server.next_frame().await,
        (1, hello_frame(CAMPUS_B, token_a()))
    );
    assert_eq!(
        server.next_frame().await,
        (1, sync_frame(&[(ROOM_B, None)]))
    );
    let settled = snapshot_where(&mut snapshots, |snapshot| {
        snapshot.network.connection == ConnectionPhase::Connected
            && snapshot.network.campus_id == Some(CAMPUS_B)
    })
    .await;
    assert_eq!(settled.session.active_room_id, Some(ROOM_B));
    assert!(settled.timeline.entries.is_empty());
}
