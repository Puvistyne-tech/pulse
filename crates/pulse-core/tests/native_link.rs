mod fixtures;

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use fixtures::{
    ACCOUNT_A, CAMPUS_A, CAMPUS_B, ROOM_A, ROOM_B, attachment_id, auth_session, campus_list,
    campus_summary, chain_message, confirmed_key, entry_keys, hello_frame, message_id, pending_key,
    room_message, send_frame, sign_in_request, subscription_frame, sync_frame, sync_inbox_frame,
    sync_shelf_frame, sync_thread_frame, token_a, vault_item, vault_synced_frame, welcome_for,
};
use futures_util::{SinkExt, StreamExt};
use pulse_core::THREAD_VIEW_FOLLOW_MS;
use pulse_core::native::{ClientStorage, NativeClient, websocket_url};
use pulse_protocol::{
    CampusId, ClientFrame, ClientSnapshot, ConnectionPhase, Role, Room, RoomList, ServerFrame,
    SubscriptionOrigin, SubscriptionState, UploadState, UploadTicket, VaultTab, api_path,
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
        ClientFrame::SyncVault(request) => Reaction {
            replies: vec![vault_synced_frame(
                request.room_id,
                request.tab,
                (1..=2)
                    .map(|index| {
                        vault_item(
                            chain_message(request.room_id, index),
                            None,
                            Some("https://example.org/a"),
                        )
                    })
                    .collect(),
                None,
            )],
            close: false,
        },
        _ => Reaction {
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

fn route(request_line: &str, body: &str, address: SocketAddr) -> Vec<u8> {
    let mut parts = request_line.split_whitespace();
    let (method, path) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
    let json =
        |value: Result<Vec<u8>, serde_json::Error>| http_response("200 OK", &value.expect("json"));
    let uploads = api_path::campus_room_uploads(CAMPUS_A, ROOM_A);
    match (method, path) {
        ("POST", uploads_path) if uploads_path == uploads => {
            let target = if body.contains("denied") {
                "denied"
            } else {
                "accepted"
            };
            let ticket = UploadTicket {
                attachment_id: attachment_id(1),
                put_url: format!("http://{address}/objects/{target}"),
                content_type: "image/png".to_owned(),
                expires_at_ms: 1,
            };
            http_response("201 Created", &serde_json::to_vec(&ticket).expect("json"))
        }
        ("PUT", "/objects/accepted") => http_response("200 OK", b"{}"),
        ("PUT", _) => http_response("403 Forbidden", br#"{"code":"forbidden","field":null}"#),
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

async fn serve_http(mut stream: TcpStream, address: SocketAddr) {
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 2048];
    let (head, body) = loop {
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
                let body = String::from_utf8_lossy(&buffer[end..]).into_owned();
                break (head, body);
            }
        }
    };
    let request_line = head.lines().next().unwrap_or("").to_owned();
    let _ = stream
        .write_all(&route(&request_line, &body, address))
        .await;
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
                        serve_http(stream, address).await;
                    }
                });
            }
        });
        Self { address, observed }
    }

    fn url(&self) -> String {
        format!("http://{}", self.address)
    }

    fn buffered(&mut self) -> Vec<Observed> {
        std::iter::from_fn(|| self.observed.try_recv().ok()).collect()
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
    let client = NativeClient::new(server.url(), ClientStorage::Memory).expect("client");
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
    assert_eq!(server.next_frame().await, (0, sync_inbox_frame()));
    assert_eq!(server.next_frame().await, (0, sync_shelf_frame()));
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
    assert_eq!(server.next_frame().await, (1, sync_inbox_frame()));
    assert_eq!(server.next_frame().await, (1, sync_shelf_frame()));
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
    let client = NativeClient::new(server.url(), ClientStorage::Memory).expect("client");
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
    assert_eq!(server.next_frame().await, (0, sync_inbox_frame()));
    assert_eq!(server.next_frame().await, (0, sync_shelf_frame()));
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
    assert_eq!(server.next_frame().await, (1, sync_inbox_frame()));
    assert_eq!(server.next_frame().await, (1, sync_shelf_frame()));
    let settled = snapshot_where(&mut snapshots, |snapshot| {
        snapshot.network.connection == ConnectionPhase::Connected
            && snapshot.network.campus_id == Some(CAMPUS_B)
    })
    .await;
    assert_eq!(settled.session.active_room_id, Some(ROOM_B));
    assert!(settled.timeline.entries.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_intents_during_campus_switch_leave_only_the_new_link() {
    let mut server = FakeServer::start().await;
    let client = NativeClient::new(server.url(), ClientStorage::Memory).expect("client");
    let mut snapshots = watch(&client);
    within(client.sign_in(sign_in_request())).await;
    snapshot_where(&mut snapshots, |snapshot| {
        snapshot.network.connection == ConnectionPhase::Connected
            && snapshot.network.campus_id == Some(CAMPUS_A)
    })
    .await;
    let (switched, _, _, _, _, _, _) = within(async {
        tokio::join!(
            client.select_campus(CAMPUS_B),
            client.snapshot(),
            client.send_message(ROOM_A, "one".to_owned()),
            client.snapshot(),
            client.send_message(ROOM_A, "two".to_owned()),
            client.send_message(ROOM_B, "three".to_owned()),
            client.snapshot(),
        )
    })
    .await;
    assert_eq!(switched.session.active_campus_id, Some(CAMPUS_B));
    let settled = snapshot_where(&mut snapshots, |snapshot| {
        snapshot.network.connection == ConnectionPhase::Connected
            && snapshot.network.campus_id == Some(CAMPUS_B)
    })
    .await;
    assert_eq!(settled.session.active_campus_id, Some(CAMPUS_B));
    tokio::time::sleep(Duration::from_millis(1500)).await;
    let hellos = server
        .buffered()
        .into_iter()
        .filter_map(|(connection, frame)| match frame {
            ClientFrame::Hello(hello) => Some((connection, hello.campus_id)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let (last_connection, last_campus) = *hellos.last().expect("hello");
    assert_eq!(last_campus, CAMPUS_B);
    assert!(
        hellos
            .iter()
            .filter(|(_, campus_id)| *campus_id == CAMPUS_A)
            .all(|(connection, _)| *connection < last_connection)
    );
    let final_snapshot = within(client.snapshot()).await;
    assert_eq!(
        final_snapshot.network.connection,
        ConnectionPhase::Connected
    );
    assert_eq!(final_snapshot.network.campus_id, Some(CAMPUS_B));
}

const FOLLOW_MARGIN: Duration = Duration::from_millis(2500);
const FOLLOW_DELAY: Duration = Duration::from_millis(THREAD_VIEW_FOLLOW_MS);
const FOLLOW_EARLIEST: Duration = Duration::from_millis(THREAD_VIEW_FOLLOW_MS - 100);

async fn connected_watching(
    server: &mut FakeServer,
) -> (NativeClient, UnboundedReceiver<ClientSnapshot>) {
    let client = NativeClient::new(server.url(), ClientStorage::Memory).expect("client");
    let mut snapshots = watch(&client);
    within(client.sign_in(sign_in_request())).await;
    snapshot_where(&mut snapshots, |snapshot| {
        snapshot.network.connection == ConnectionPhase::Connected
    })
    .await;
    assert_eq!(
        server.next_frame().await,
        (0, hello_frame(CAMPUS_A, token_a()))
    );
    assert_eq!(
        server.next_frame().await,
        (0, sync_frame(&[(ROOM_A, None), (ROOM_B, None)]))
    );
    assert_eq!(server.next_frame().await, (0, sync_inbox_frame()));
    assert_eq!(server.next_frame().await, (0, sync_shelf_frame()));
    (client, snapshots)
}

async fn connected_client(server: &mut FakeServer) -> NativeClient {
    connected_watching(server).await.0
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn opened_thread_syncs_then_follows_after_the_view_delay() {
    let mut server = FakeServer::start().await;
    let client = connected_client(&mut server).await;
    let started = tokio::time::Instant::now();
    let opened = within(client.open_thread(ROOM_A, message_id(1))).await;
    assert_eq!(opened.thread.root_message_id, Some(message_id(1)));
    assert_eq!(server.next_frame().await, (0, sync_thread_frame(1, None)));
    let follow = tokio::time::timeout(FOLLOW_DELAY + FOLLOW_MARGIN, server.next_frame())
        .await
        .expect("follow within the delay plus margin");
    assert_eq!(
        follow,
        (
            0,
            subscription_frame(1, SubscriptionState::Following, SubscriptionOrigin::Viewed)
        )
    );
    assert!(started.elapsed() >= FOLLOW_EARLIEST);
    let settled = within(client.snapshot()).await;
    assert_eq!(
        settled.thread.subscription,
        Some(SubscriptionState::Following)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn closing_before_the_view_delay_sends_no_follow() {
    let mut server = FakeServer::start().await;
    let client = connected_client(&mut server).await;
    within(client.open_thread(ROOM_A, message_id(1))).await;
    assert_eq!(server.next_frame().await, (0, sync_thread_frame(1, None)));
    let closed = within(client.close_thread()).await;
    assert_eq!(closed.thread.root_message_id, None);
    let silent = tokio::time::timeout(FOLLOW_DELAY + FOLLOW_MARGIN, server.next_frame()).await;
    assert!(silent.is_err());
    assert_eq!(server.buffered(), vec![]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn vault_pages_feed_offline_search_through_the_effect_runner() {
    let mut server = FakeServer::start().await;
    let (client, mut snapshots) = connected_watching(&mut server).await;
    within(client.open_vault(ROOM_A, VaultTab::Links)).await;
    snapshot_where(&mut snapshots, |snapshot| {
        snapshot.vault.entries.len() == 2 && !snapshot.vault.loading
    })
    .await;
    within(client.toggle_search()).await;
    within(client.search("peer".to_owned())).await;
    let found = snapshot_where(&mut snapshots, |snapshot| {
        snapshot.search.hits.len() == 2 && !snapshot.search.searching
    })
    .await;
    assert_eq!(
        found
            .search
            .hits
            .iter()
            .map(|hit| hit.message_id)
            .collect::<Vec<_>>(),
        vec![message_id(2), message_id(1)]
    );
    assert_eq!(found.search.selected_index, Some(0));
    let none = within(client.search("zzz".to_owned())).await;
    assert!(none.search.query == "zzz");
    let empty = snapshot_where(&mut snapshots, |snapshot| {
        snapshot.search.query == "zzz" && !snapshot.search.searching
    })
    .await;
    assert!(empty.search.hits.is_empty());
    let observed = server.buffered();
    assert!(
        observed
            .iter()
            .any(|(_, frame)| matches!(frame, ClientFrame::SyncVault(_)))
    );
}

#[rstest]
#[case::accepted_upload("a.png", UploadState::Uploaded, true)]
#[case::storage_refuses_the_put("denied.png", UploadState::Failed, false)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn attached_files_upload_before_the_message_is_sent(
    #[case] file_name: &str,
    #[case] expected: UploadState,
    #[case] message_sent: bool,
) {
    let mut server = FakeServer::start().await;
    let (client, mut snapshots) = connected_watching(&mut server).await;
    within(client.attach_file(ROOM_A, file_name.to_owned(), vec![1, 2, 3])).await;
    let settled = snapshot_where(&mut snapshots, |snapshot| {
        snapshot
            .timeline
            .drafts
            .iter()
            .any(|draft| draft.upload != UploadState::Uploading)
    })
    .await;
    assert_eq!(settled.timeline.drafts.len(), 1);
    assert_eq!(settled.timeline.drafts[0].upload, expected);
    assert_eq!(settled.timeline.drafts[0].byte_size, 3);
    if !message_sent {
        let sent = within(client.send_message(ROOM_A, String::new())).await;
        assert_eq!(sent.timeline.entries.len(), 1);
        assert_eq!(
            sent.timeline.entries[0].rejection,
            Some(pulse_protocol::MessageRejection::InvalidAttachment)
        );
        return;
    }
    within(client.send_message(ROOM_A, String::new())).await;
    let frame = within(async {
        loop {
            let (_, frame) = server.next_frame().await;
            if matches!(frame, ClientFrame::SendMessage(_)) {
                return frame;
            }
        }
    })
    .await;
    match frame {
        ClientFrame::SendMessage(request) => {
            assert_eq!(request.attachment_ids, vec![attachment_id(1)]);
            assert_eq!(request.room_id, ROOM_A);
        }
        other => panic!("unexpected frame {other:?}"),
    }
}
