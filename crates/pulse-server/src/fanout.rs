use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex, MutexGuard, PoisonError};

use futures_util::StreamExt;
use pulse_protocol::{CampusId, RoomMessage};
use redis::AsyncCommands;
use redis::aio::{MultiplexedConnection, PubSubSink, PubSubStream};
use tokio::sync::{Mutex, broadcast};

const FEED_CAPACITY: usize = 1024;

#[derive(Debug, thiserror::Error)]
pub enum FanoutError {
    #[error("valkey: {0}")]
    Valkey(#[from] redis::RedisError),
    #[error("payload: {0}")]
    Payload(#[from] serde_json::Error),
}

pub fn campus_channel(campus_id: CampusId) -> String {
    format!("pulse:campus:{campus_id}:messages")
}

struct CampusEntry {
    sender: broadcast::Sender<RoomMessage>,
    generation: u64,
    feeds: usize,
}

struct Inner {
    valkey: redis::Client,
    publisher: Mutex<Option<MultiplexedConnection>>,
    subscriber: Mutex<Option<PubSubSink>>,
    campuses: StdMutex<HashMap<CampusId, CampusEntry>>,
    generations: AtomicU64,
}

impl Inner {
    fn campuses(&self) -> MutexGuard<'_, HashMap<CampusId, CampusEntry>> {
        self.campuses.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn deliver(&self, channel: &str, payload: &[u8]) {
        let Ok(message) = serde_json::from_slice::<RoomMessage>(payload) else {
            return;
        };
        if channel != campus_channel(message.campus_id) {
            return;
        }
        let sender = self
            .campuses()
            .get(&message.campus_id)
            .map(|entry| entry.sender.clone());
        if let Some(sender) = sender {
            let _ = sender.send(message);
        }
    }
}

#[derive(Clone)]
pub struct Fanout {
    inner: Arc<Inner>,
}

pub struct CampusFeed {
    receiver: broadcast::Receiver<RoomMessage>,
    campus_id: CampusId,
    generation: u64,
    inner: Arc<Inner>,
}

async fn dispatch(inner: Arc<Inner>, mut stream: PubSubStream) {
    while let Some(message) = stream.next().await {
        inner.deliver(message.get_channel_name(), message.get_payload_bytes());
    }
    let mut subscriber = inner.subscriber.lock().await;
    *subscriber = None;
    inner.campuses().clear();
}

impl Fanout {
    pub fn new(valkey: redis::Client) -> Self {
        Self {
            inner: Arc::new(Inner {
                valkey,
                publisher: Mutex::new(None),
                subscriber: Mutex::new(None),
                campuses: StdMutex::new(HashMap::new()),
                generations: AtomicU64::new(0),
            }),
        }
    }

    pub async fn publish(&self, message: &RoomMessage) -> Result<(), FanoutError> {
        let payload = serde_json::to_string(message)?;
        let channel = campus_channel(message.campus_id);
        let mut publisher = self.inner.publisher.lock().await;
        let mut connection = match publisher.take() {
            Some(connection) => connection,
            None => self.inner.valkey.get_multiplexed_async_connection().await?,
        };
        let outcome = connection.publish::<_, _, i64>(channel, payload).await;
        if outcome.is_ok() {
            *publisher = Some(connection);
        }
        outcome?;
        Ok(())
    }

    pub async fn join(&self, campus_id: CampusId) -> Result<CampusFeed, FanoutError> {
        let mut subscriber = self.inner.subscriber.lock().await;
        if subscriber.is_none() {
            let (sink, stream) = self.inner.valkey.get_async_pubsub().await?.split();
            tokio::spawn(dispatch(Arc::clone(&self.inner), stream));
            *subscriber = Some(sink);
        }
        let (generation, receiver, first) = {
            let mut campuses = self.inner.campuses();
            let entry = campuses.entry(campus_id).or_insert_with(|| CampusEntry {
                sender: broadcast::channel(FEED_CAPACITY).0,
                generation: self.inner.generations.fetch_add(1, Ordering::Relaxed),
                feeds: 0,
            });
            entry.feeds += 1;
            (entry.generation, entry.sender.subscribe(), entry.feeds == 1)
        };
        let feed = CampusFeed {
            receiver,
            campus_id,
            generation,
            inner: Arc::clone(&self.inner),
        };
        if first && let Some(sink) = subscriber.as_mut() {
            sink.subscribe(campus_channel(campus_id)).await?;
        }
        Ok(feed)
    }
}

impl CampusFeed {
    pub async fn recv(&mut self) -> Result<RoomMessage, broadcast::error::RecvError> {
        self.receiver.recv().await
    }
}

impl Drop for CampusFeed {
    fn drop(&mut self) {
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            return;
        };
        let inner = Arc::clone(&self.inner);
        let campus_id = self.campus_id;
        let generation = self.generation;
        handle.spawn(async move {
            let mut subscriber = inner.subscriber.lock().await;
            let last = {
                let mut campuses = inner.campuses();
                match campuses.get_mut(&campus_id) {
                    Some(entry) if entry.generation == generation => {
                        entry.feeds = entry.feeds.saturating_sub(1);
                        if entry.feeds == 0 {
                            campuses.remove(&campus_id);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            };
            if last && let Some(sink) = subscriber.as_mut() {
                let _ = sink.unsubscribe(campus_channel(campus_id)).await;
            }
        });
    }
}
