#![allow(dead_code)]

use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use pulse_protocol::{CampusId, ClientFrame, Hello, Ping, ProtocolVersion, ServerFrame};
use pulse_server::{AppState, Config, router, state};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub const CAMPUS_A: &str = "0193a5c0-0000-7000-8000-00000000000a";
pub const CAMPUS_B: &str = "0193a5c0-0000-7000-8000-00000000000b";

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

pub fn campus(text: &str) -> CampusId {
    text.parse().expect("valid campus fixture")
}

pub fn hello(campus_text: &str, version: u16) -> Outgoing {
    Outgoing::Frame(ClientFrame::Hello(Hello {
        protocol_version: ProtocolVersion(version),
        campus_id: campus(campus_text),
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

pub async fn spawn_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    let app = router(lazy_state());
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    addr
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
