use std::io;
use std::sync::Arc;

use pulse_protocol::{
    CampusId, ClientNonce, ClientSnapshot, CreateCampusRequest, InboxStep, MessageId,
    ReactionAction, RoomId, SignInRequest, SignUpRequest, SubscriptionState,
};
use tokio::runtime::{Builder, Runtime};

use crate::client::unavailable_snapshot;
use crate::session::SessionEvent;
use crate::timeline::ThreadIntent;

use super::http::server_url_from_env;
use super::socket::Shared;

const RUNTIME_WORKER_THREADS: usize = 2;

enum Intent {
    Snapshot,
    Session(SessionEvent),
    Send {
        room_id: RoomId,
        client_nonce: ClientNonce,
        body: String,
    },
    Discard(ClientNonce),
    Thread(ThreadIntent),
}

pub struct NativeClient {
    runtime: Option<Runtime>,
    shared: Arc<Shared>,
}

impl NativeClient {
    pub fn new(server_url: impl Into<String>) -> io::Result<Self> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(RUNTIME_WORKER_THREADS)
            .enable_all()
            .build()?;
        let server_url = server_url.into().trim_end_matches('/').to_owned();
        Ok(Self {
            runtime: Some(runtime),
            shared: Arc::new(Shared::new(server_url)),
        })
    }

    pub fn from_env() -> io::Result<Self> {
        Self::new(server_url_from_env())
    }

    async fn run(&self, intent: Intent) -> ClientSnapshot {
        let Some(runtime) = self.runtime.as_ref() else {
            return unavailable_snapshot();
        };
        let shared = Arc::clone(&self.shared);
        let handle = runtime.spawn(async move {
            let snapshot = {
                let mut guard = shared.core.lock().await;
                let (client, api) = &mut *guard;
                let snapshot = match intent {
                    Intent::Snapshot => client.snapshot(),
                    Intent::Session(event) => client.apply(api, event).await,
                    Intent::Send {
                        room_id,
                        client_nonce,
                        body,
                    } => client.send_message(room_id, client_nonce, body),
                    Intent::Discard(client_nonce) => client.discard_message(client_nonce),
                    Intent::Thread(intent) => client.thread_intent(intent),
                };
                shared.publish(client);
                shared.arm_deadline(client.next_deadline_ms()).await;
                shared.sync_link(client.link_target()).await;
                snapshot
            };
            shared.wake.notify_one();
            snapshot
        });
        handle.await.unwrap_or_else(|_| unavailable_snapshot())
    }

    pub async fn snapshot(&self) -> ClientSnapshot {
        self.run(Intent::Snapshot).await
    }

    pub async fn sign_up(&self, request: SignUpRequest) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::SignUpRequested(request)))
            .await
    }

    pub async fn sign_in(&self, request: SignInRequest) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::SignInRequested(request)))
            .await
    }

    pub async fn sign_out(&self) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::SignOutRequested))
            .await
    }

    pub async fn refresh_campuses(&self) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::CampusesRequested))
            .await
    }

    pub async fn create_campus(&self, request: CreateCampusRequest) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::CreateCampusRequested(
            request,
        )))
        .await
    }

    pub async fn select_campus(&self, campus_id: CampusId) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::CampusSelected(campus_id)))
            .await
    }

    pub async fn select_room(&self, room_id: RoomId) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::RoomSelected(room_id)))
            .await
    }

    pub async fn send_message(&self, room_id: RoomId, body: String) -> ClientSnapshot {
        self.run(Intent::Send {
            room_id,
            client_nonce: ClientNonce::generate(),
            body,
        })
        .await
    }

    pub async fn discard_message(&self, client_nonce: ClientNonce) -> ClientSnapshot {
        self.run(Intent::Discard(client_nonce)).await
    }

    pub async fn send_reply(
        &self,
        room_id: RoomId,
        root_message_id: MessageId,
        body: String,
    ) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::ReplyRequested {
            room_id,
            root_message_id,
            client_nonce: ClientNonce::generate(),
            body,
        }))
        .await
    }

    pub async fn react(
        &self,
        room_id: RoomId,
        message_id: MessageId,
        emoji: String,
        action: ReactionAction,
    ) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::ReactRequested {
            room_id,
            message_id,
            emoji,
            action,
        }))
        .await
    }

    pub async fn set_topic_title(
        &self,
        room_id: RoomId,
        root_message_id: MessageId,
        topic_title: String,
    ) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::TopicTitleRequested {
            room_id,
            root_message_id,
            topic_title,
        }))
        .await
    }

    pub async fn set_thread_subscription(
        &self,
        room_id: RoomId,
        root_message_id: MessageId,
        state: SubscriptionState,
    ) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::SubscriptionRequested {
            room_id,
            root_message_id,
            state,
        }))
        .await
    }

    pub async fn open_thread(&self, room_id: RoomId, root_message_id: MessageId) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::ThreadOpened {
            room_id,
            root_message_id,
            now_ms: self.shared.now_ms(),
        }))
        .await
    }

    pub async fn close_thread(&self) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::ThreadClosed)).await
    }

    pub async fn toggle_inbox(&self) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::InboxToggled)).await
    }

    pub async fn move_inbox_selection(&self, step: InboxStep) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::InboxSelectionMoved(step)))
            .await
    }

    pub async fn archive_inbox_selection(&self) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::InboxSelectionArchived))
            .await
    }

    pub async fn start_inbox_reply(&self) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::InboxReplyStarted))
            .await
    }

    pub async fn cancel_inbox_reply(&self) -> ClientSnapshot {
        self.run(Intent::Thread(ThreadIntent::InboxReplyCancelled))
            .await
    }

    pub fn forward_snapshots<F>(&self, mut sink: F)
    where
        F: FnMut(ClientSnapshot) -> bool + Send + 'static,
    {
        let Some(runtime) = self.runtime.as_ref() else {
            return;
        };
        let mut receiver = self.shared.snapshots.subscribe();
        runtime.spawn(async move {
            loop {
                let snapshot = receiver.borrow_and_update().clone();
                if !sink(snapshot) || receiver.changed().await.is_err() {
                    return;
                }
            }
        });
    }
}

impl Drop for NativeClient {
    fn drop(&mut self) {
        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }
    }
}
