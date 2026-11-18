use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use futures_util::{SinkExt, StreamExt};
use pulse_protocol::{ClientFrame, ClientSnapshot, Hello, ServerFrame, api_path};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify, watch};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::client::{Client, LinkTarget};
use crate::connection::{ConnectionCommand, ConnectionEvent};
use crate::timeline::ThreadIntent;

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

struct Stale;

struct RunningLink {
    target: LinkTarget,
    task: JoinHandle<()>,
}

pub(super) struct Shared {
    pub(super) core: Mutex<(Client, HttpApiTransport)>,
    pub(super) snapshots: watch::Sender<ClientSnapshot>,
    pub(super) wake: Notify,
    server_url: String,
    generation: AtomicU64,
    link: Mutex<Option<RunningLink>>,
    started: Instant,
    deadline_generation: AtomicU64,
    deadline: Mutex<Option<JoinHandle<()>>>,
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
            generation: AtomicU64::new(0),
            link: Mutex::new(None),
            started: Instant::now(),
            deadline_generation: AtomicU64::new(0),
            deadline: Mutex::new(None),
        }
    }

    pub(super) fn now_ms(&self) -> u64 {
        u64::try_from(self.started.elapsed().as_millis()).unwrap_or(u64::MAX)
    }

    pub(super) async fn arm_deadline(self: &Arc<Self>, deadline_ms: Option<u64>) {
        let mut slot = self.deadline.lock().await;
        if let Some(task) = slot.take() {
            task.abort();
        }
        let generation = self.deadline_generation.fetch_add(1, Ordering::SeqCst) + 1;
        if let Some(deadline_ms) = deadline_ms {
            *slot = Some(tokio::spawn(run_deadline(
                Arc::clone(self),
                deadline_ms,
                generation,
            )));
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
        let generation = self.generation.fetch_add(1, Ordering::SeqCst) + 1;
        if let Some(target) = target {
            let task = tokio::spawn(run_link(Arc::clone(self), target.clone(), generation));
            *link = Some(RunningLink { target, task });
        }
    }

    fn is_current(&self, generation: u64) -> bool {
        self.generation.load(Ordering::SeqCst) == generation
    }

    async fn step(
        &self,
        generation: u64,
        event: ConnectionEvent,
    ) -> Result<Option<ConnectionCommand>, Stale> {
        let mut guard = self.core.lock().await;
        if !self.is_current(generation) {
            return Err(Stale);
        }
        let command = guard.0.connection_step(event);
        self.publish(&guard.0);
        Ok(command)
    }

    async fn deliver(
        &self,
        generation: u64,
        frame: ServerFrame,
    ) -> Result<(Option<ConnectionCommand>, Vec<ClientFrame>), Stale> {
        let mut guard = self.core.lock().await;
        if !self.is_current(generation) {
            return Err(Stale);
        }
        let command = guard.0.receive_frame(frame);
        let outbound = guard.0.drain_outbound();
        self.publish(&guard.0);
        Ok((command, outbound))
    }

    async fn drain(&self, generation: u64) -> Result<Vec<ClientFrame>, Stale> {
        let mut guard = self.core.lock().await;
        if !self.is_current(generation) {
            return Err(Stale);
        }
        Ok(guard.0.drain_outbound())
    }
}

async fn run_deadline(shared: Arc<Shared>, first_deadline_ms: u64, generation: u64) {
    let mut deadline_ms = first_deadline_ms;
    loop {
        let wait = deadline_ms.saturating_sub(shared.now_ms());
        tokio::time::sleep(Duration::from_millis(wait)).await;
        let next = {
            let mut guard = shared.core.lock().await;
            if shared.deadline_generation.load(Ordering::SeqCst) != generation {
                return;
            }
            let now_ms = shared.now_ms();
            guard
                .0
                .thread_intent(ThreadIntent::ClockAdvanced { now_ms });
            shared.publish(&guard.0);
            guard.0.next_deadline_ms()
        };
        shared.wake.notify_one();
        match next {
            Some(next) => deadline_ms = next,
            None => return,
        }
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

async fn pump(shared: &Shared, generation: u64, link: &mut Link, hello: Hello) {
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
                    let Ok((command, outbound)) = shared.deliver(generation, frame).await else {
                        return;
                    };
                    if matches!(command, Some(ConnectionCommand::CloseTransport)) {
                        return;
                    }
                    outbound
                }
                Some(Ok(Message::Close(_))) | Some(Err(_)) | None => return,
                Some(Ok(_)) => continue,
            },
            () = shared.wake.notified() => match shared.drain(generation).await {
                Ok(outbound) => outbound,
                Err(Stale) => return,
            },
        };
        if !send_all(link, &outbound).await {
            return;
        }
    }
}

async fn run_link(shared: Arc<Shared>, target: LinkTarget, generation: u64) {
    let url = websocket_url(&shared.server_url);
    if shared
        .step(generation, ConnectionEvent::DisconnectRequested)
        .await
        .is_err()
    {
        return;
    }
    let Ok(mut command) = shared
        .step(
            generation,
            ConnectionEvent::ConnectRequested {
                campus_id: target.campus_id,
                auth_token: target.auth_token,
            },
        )
        .await
    else {
        return;
    };
    loop {
        let event = match command {
            Some(ConnectionCommand::OpenTransport) => match connect_async(url.as_str()).await {
                Ok((mut link, _)) => {
                    let Ok(opened) = shared
                        .step(generation, ConnectionEvent::TransportOpened)
                        .await
                    else {
                        return;
                    };
                    if let Some(ConnectionCommand::SendHello(hello)) = opened {
                        pump(&shared, generation, &mut link, hello).await;
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
        let Ok(next) = shared.step(generation, event).await else {
            return;
        };
        command = next;
    }
}
