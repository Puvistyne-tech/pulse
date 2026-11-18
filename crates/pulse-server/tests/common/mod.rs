#![allow(dead_code)]

use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use pulse_server::{AppState, Config, router, state};
use serde_json::Value;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub const CAMPUS_A: &str = "0193a5c0-0000-7000-8000-00000000000a";

pub type ClientSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, PartialEq)]
pub enum Observed {
    Json(Value),
    Close(u16),
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

pub async fn send_text(socket: &mut ClientSocket, text: &str) {
    socket
        .send(Message::Text(text.into()))
        .await
        .expect("send text");
}

pub async fn observe(socket: &mut ClientSocket) -> Observed {
    loop {
        match socket.next().await {
            Some(Ok(Message::Text(text))) => {
                return Observed::Json(serde_json::from_str(text.as_str()).expect("json frame"));
            }
            Some(Ok(Message::Close(frame))) => {
                return Observed::Close(frame.map(|close| u16::from(close.code)).unwrap_or(1005));
            }
            Some(Ok(_)) => continue,
            Some(Err(_)) | None => return Observed::Close(1006),
        }
    }
}
