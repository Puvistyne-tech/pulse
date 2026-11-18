use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use pulse_protocol::{ClientFrame, ClientSnapshot, Hello, ServerFrame, api_path};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify, watch};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::client::{Client, LinkTarget};
use crate::connection::{ConnectionCommand, ConnectionEvent};

use super::http::HttpApiTransport;

type Link = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub fn websocket_url(server_url: &str) -> String {
    let trimmed = server_url.trim_end_matches('/');
    let base = if let Some(rest) = trimmed.strip_prefix("https://") {
        format!("wss://{rest}")
    } else if let Some(rest) = trimmed.strip_prefix("http://") {
        format!("ws://{rest}")
    } else {
        trimmed.to_owned()
    };
    format!("{base}{}", api_path::WEBSOCKET)
}

struct RunningLink {
    target: LinkTarget,
    task: JoinHandle<()>,
}

pub(super) struct Shared {
    pub(super) core: Mutex<(Client, HttpApiTransport)>,
    pub(super) snapshots: watch::Sender<ClientSnapshot>,
    pub(super) wake: Notify,
    server_url: String,
    link: Mutex<Option<RunningLink>>,
}

impl Shared {
    pub(super) fn new(server_url: String) -> Self {
        let client = Client::default();
        let (snapshots, _) = watch::channel(client.snapshot());
        Self {
            core: Mutex::new((client, HttpApiTransport::new(server_url.clone()))),
            snapshots,
            wake: Notify::new(),
            server_url,
            link: Mutex::new(None),
        }
    }

    pub(super) fn publish(&self, client: &Client) {
        self.snapshots.send_replace(client.snapshot());
    }

    pub(super) async fn sync_link(self: &Arc<Self>, target: Option<LinkTarget>) {
        let mut link = self.link.lock().await;
        if link.as_ref().map(|running| &running.target) == target.as_ref() {
            return;
        }
        if let Some(running) = link.take() {
            running.task.abort();
        }
        if let Some(target) = target {
            let task = tokio::spawn(run_link(Arc::clone(self), target.clone()));
            *link = Some(RunningLink { target, task });
        }
    }

    async fn step(&self, event: ConnectionEvent) -> Option<ConnectionCommand> {
        let mut guard = self.core.lock().await;
        let command = guard.0.connection_step(event);
        self.publish(&guard.0);
        command
    }

    async fn deliver(&self, frame: ServerFrame) -> (Option<ConnectionCommand>, Vec<ClientFrame>) {
        let mut guard = self.core.lock().await;
        let command = guard.0.receive_frame(frame);
        let outbound = guard.0.drain_outbound();
        self.publish(&guard.0);
        (command, outbound)
    }

    async fn drain(&self) -> Vec<ClientFrame> {
        self.core.lock().await.0.drain_outbound()
    }
}

async fn send_frame(link: &mut Link, frame: &ClientFrame) -> bool {
    match serde_json::to_string(frame) {
        Ok(text) => link.send(Message::text(text)).await.is_ok(),
        Err(_) => false,
    }
}

async fn send_all(link: &mut Link, frames: &[ClientFrame]) -> bool {
    for frame in frames {
        if !send_frame(link, frame).await {
            return false;
        }
    }
    true
}

async fn pump(shared: &Shared, link: &mut Link, hello: Hello) {
    if !send_frame(link, &ClientFrame::Hello(hello)).await {
        return;
    }
    loop {
        let outbound = tokio::select! {
            incoming = link.next() => match incoming {
                Some(Ok(Message::Text(text))) => {
                    let Ok(frame) = serde_json::from_str::<ServerFrame>(text.as_str()) else {
                        return;
                    };
                    let (command, outbound) = shared.deliver(frame).await;
                    if matches!(command, Some(ConnectionCommand::CloseTransport)) {
                        return;
                    }
                    outbound
                }
                Some(Ok(Message::Close(_))) | Some(Err(_)) | None => return,
                Some(Ok(_)) => continue,
            },
            () = shared.wake.notified() => shared.drain().await,
        };
        if !send_all(link, &outbound).await {
            return;
        }
    }
}

async fn run_link(shared: Arc<Shared>, target: LinkTarget) {
    let url = websocket_url(&shared.server_url);
    shared.step(ConnectionEvent::DisconnectRequested).await;
    let mut command = shared
        .step(ConnectionEvent::ConnectRequested {
            campus_id: target.campus_id,
            auth_token: target.auth_token,
        })
        .await;
    loop {
        let event = match command {
            Some(ConnectionCommand::OpenTransport) => match connect_async(url.as_str()).await {
                Ok((mut link, _)) => {
                    let opened = shared.step(ConnectionEvent::TransportOpened).await;
                    if let Some(ConnectionCommand::SendHello(hello)) = opened {
                        pump(&shared, &mut link, hello).await;
                    }
                    ConnectionEvent::TransportFailed
                }
                Err(_) => ConnectionEvent::TransportFailed,
            },
            Some(ConnectionCommand::ScheduleRetry { delay_ms }) => {
                tokio::time::sleep(Duration::from_millis(u64::from(delay_ms))).await;
                ConnectionEvent::BackoffElapsed
            }
            Some(ConnectionCommand::SendHello(_) | ConnectionCommand::CloseTransport) | None => {
                return;
            }
        };
        command = shared.step(event).await;
    }
}
