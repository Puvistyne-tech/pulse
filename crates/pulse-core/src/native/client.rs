use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use pulse_protocol::{
    AccountId, AddMemberRequest, CampusId, ClientNonce, ClientSnapshot, CreateCampusRequest,
    DraftId, InboxStep, JoinCampusRequest, LoungeId, LoungeMediaState, MessageId, ReactionAction,
    ReminderPreset, RoomId, ShelfAction, SignInRequest, SignUpRequest, SubscriptionState,
    UpdateMemberRequest, VaultTab,
};
use tokio::runtime::{Builder, Runtime};

use crate::client::unavailable_snapshot;
use crate::lounge::LoungeIntent;
use crate::media::MediaIntent;
use crate::search::MemorySearchIndex;
use crate::search_view::SearchIntent;
use crate::session::SessionEvent;
use crate::shelf::ShelfIntent;
use crate::steward_view::StewardIntent;
use crate::timeline::ThreadIntent;

use super::http::{quick_sign_in_from_env, server_url_from_env};
use super::search_index::{NativeSearchIndex, SqliteSearchIndex};
use super::socket::Shared;

const DATA_DIR_VARIABLE: &str = "PULSE_DATA_DIR";
const SEARCH_INDEX_FILE: &str = "pulse-search.sqlite3";

const RUNTIME_WORKER_THREADS: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientStorage {
    Memory,
    Directory(PathBuf),
}

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
    Search(SearchIntent),
    Media(MediaIntent),
    Attach {
        draft_id: DraftId,
        room_id: RoomId,
        file_name: String,
        bytes: Vec<u8>,
    },
    Shelf(ShelfIntent),
    Lounge(LoungeIntent),
    Steward(StewardIntent),
    OpenSearchHit(Option<MessageId>),
}

pub struct NativeClient {
    runtime: Option<Runtime>,
    shared: Arc<Shared>,
}

impl NativeClient {
    pub fn new(server_url: impl Into<String>, storage: ClientStorage) -> io::Result<Self> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(RUNTIME_WORKER_THREADS)
            .enable_all()
            .build()?;
        let server_url = server_url.into().trim_end_matches('/').to_owned();
        let index = match storage {
            ClientStorage::Memory => NativeSearchIndex::Memory(MemorySearchIndex::default()),
            ClientStorage::Directory(directory) => {
                std::fs::create_dir_all(&directory)?;
                let index = SqliteSearchIndex::open(&directory.join(SEARCH_INDEX_FILE))
                    .map_err(io::Error::other)?;
                NativeSearchIndex::Sqlite(index)
            }
        };
        Ok(Self {
            runtime: Some(runtime),
            shared: Arc::new(Shared::new(server_url, index, quick_sign_in_from_env())),
        })
    }

    pub fn from_env() -> io::Result<Self> {
        let storage = std::env::var_os(DATA_DIR_VARIABLE)
            .map_or(ClientStorage::Memory, |directory| {
                ClientStorage::Directory(PathBuf::from(directory))
            });
        Self::new(server_url_from_env(), storage)
    }

    pub fn from_env_in(data_dir: PathBuf) -> io::Result<Self> {
        Self::new(server_url_from_env(), ClientStorage::Directory(data_dir))
    }

    async fn run(&self, intent: Intent) -> ClientSnapshot {
        let Some(runtime) = self.runtime.as_ref() else {
            return unavailable_snapshot();
        };
        let shared = Arc::clone(&self.shared);
        let handle = runtime.spawn(async move {
            let reset_drafts = matches!(intent, Intent::Session(_));
            let snapshot = {
                let mut guard = shared.core.lock().await;
                let (client, api) = &mut *guard;
                client.advance_clock(shared.now_ms());
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
                    Intent::Search(intent) => client.search_intent(intent),
                    Intent::Media(intent) => client.media_intent(intent),
                    Intent::Attach {
                        draft_id,
                        room_id,
                        file_name,
                        bytes,
                    } => {
                        let byte_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
                        shared.stash_bytes(draft_id, bytes);
                        client.media_intent(MediaIntent::DraftAttached {
                            draft_id,
                            room_id,
                            file_name,
                            byte_size,
                        })
                    }
                    Intent::Shelf(intent) => client.shelf_intent(intent),
                    Intent::Lounge(intent) => client.lounge_intent(intent),
                    Intent::Steward(intent) => client.steward_intent(intent),
                    Intent::OpenSearchHit(target) => {
                        match target.or_else(|| client.selected_search_hit()) {
                            Some(message_id) => {
                                client
                                    .open_search_hit(api, message_id, shared.now_ms())
                                    .await
                            }
                            None => client.snapshot(),
                        }
                    }
                };
                if reset_drafts {
                    shared.retain_stash(&client.live_draft_ids());
                }
                shared.publish(client);
                shared.arm_deadline(client.next_deadline_ms()).await;
                shared.sync_link(client.link_target()).await;
                snapshot
            };
            shared.process_effects().await;
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

    pub async fn quick_sign_in(&self, email: String) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::QuickSignInRequested(email)))
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

    pub async fn join_campus(&self, request: JoinCampusRequest) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::JoinCampusRequested(request)))
            .await
    }

    pub async fn create_invite(&self) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::InviteRequested))
            .await
    }

    pub async fn load_members(&self) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::MembersRequested))
            .await
    }

    pub async fn add_member(&self, request: AddMemberRequest) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::AddMemberRequested(request)))
            .await
    }

    pub async fn update_member(
        &self,
        account_id: AccountId,
        request: UpdateMemberRequest,
    ) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::UpdateMemberRequested {
            account_id,
            request,
        }))
        .await
    }

    pub async fn remove_member(&self, account_id: AccountId) -> ClientSnapshot {
        self.run(Intent::Session(SessionEvent::RemoveMemberRequested(
            account_id,
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

    pub async fn toggle_search(&self) -> ClientSnapshot {
        self.run(Intent::Search(SearchIntent::Toggled)).await
    }

    pub async fn search(&self, query: String) -> ClientSnapshot {
        self.run(Intent::Search(SearchIntent::QueryChanged(query)))
            .await
    }

    pub async fn move_search_selection(&self, step: InboxStep) -> ClientSnapshot {
        self.run(Intent::Search(SearchIntent::SelectionMoved(step)))
            .await
    }

    pub async fn open_search_selection(&self) -> ClientSnapshot {
        self.run(Intent::OpenSearchHit(None)).await
    }

    pub async fn open_search_hit(&self, message_id: MessageId) -> ClientSnapshot {
        self.run(Intent::OpenSearchHit(Some(message_id))).await
    }

    pub async fn attach_file(
        &self,
        room_id: RoomId,
        file_name: String,
        bytes: Vec<u8>,
    ) -> ClientSnapshot {
        self.run(Intent::Attach {
            draft_id: DraftId::generate(),
            room_id,
            file_name,
            bytes,
        })
        .await
    }

    pub async fn discard_draft(&self, draft_id: DraftId) -> ClientSnapshot {
        self.run(Intent::Media(MediaIntent::DraftDiscarded(draft_id)))
            .await
    }

    pub async fn retry_draft(&self, draft_id: DraftId) -> ClientSnapshot {
        self.run(Intent::Media(MediaIntent::DraftRetried(draft_id)))
            .await
    }

    pub async fn set_pin(
        &self,
        room_id: RoomId,
        message_id: MessageId,
        pinned: bool,
    ) -> ClientSnapshot {
        self.run(Intent::Media(MediaIntent::PinRequested {
            room_id,
            message_id,
            pinned,
        }))
        .await
    }

    pub async fn open_vault(&self, room_id: RoomId, tab: VaultTab) -> ClientSnapshot {
        self.run(Intent::Media(MediaIntent::VaultOpened { room_id, tab }))
            .await
    }

    pub async fn load_more_vault(&self) -> ClientSnapshot {
        self.run(Intent::Media(MediaIntent::VaultMoreRequested))
            .await
    }

    pub async fn close_vault(&self) -> ClientSnapshot {
        self.run(Intent::Media(MediaIntent::VaultClosed)).await
    }

    pub async fn toggle_shelf(&self) -> ClientSnapshot {
        self.run(Intent::Shelf(ShelfIntent::Toggled)).await
    }

    pub async fn update_shelf(
        &self,
        room_id: RoomId,
        message_id: MessageId,
        action: ShelfAction,
        preset: Option<ReminderPreset>,
        remind_at_ms: Option<u64>,
    ) -> ClientSnapshot {
        self.run(Intent::Shelf(ShelfIntent::UpdateRequested {
            room_id,
            message_id,
            action,
            preset,
            remind_at_ms,
        }))
        .await
    }

    pub async fn dismiss_reminder(&self) -> ClientSnapshot {
        self.run(Intent::Shelf(ShelfIntent::ReminderDismissed))
            .await
    }

    pub async fn join_lounge(&self, lounge_id: LoungeId) -> ClientSnapshot {
        self.run(Intent::Lounge(LoungeIntent::JoinRequested { lounge_id }))
            .await
    }

    pub async fn leave_lounge(&self) -> ClientSnapshot {
        self.run(Intent::Lounge(LoungeIntent::LeaveRequested)).await
    }

    pub async fn report_lounge_media(&self, state: LoungeMediaState) -> ClientSnapshot {
        self.run(Intent::Lounge(LoungeIntent::MediaReported(state)))
            .await
    }

    pub async fn toggle_stewards(&self) -> ClientSnapshot {
        self.run(Intent::Steward(StewardIntent::Toggled)).await
    }

    pub async fn create_steward(
        &self,
        display_name: String,
        room_ids: Vec<RoomId>,
    ) -> ClientSnapshot {
        self.run(Intent::Steward(StewardIntent::CreateRequested {
            request_nonce: ClientNonce::generate(),
            display_name,
            room_ids,
        }))
        .await
    }

    pub async fn rotate_steward_token(&self, steward_id: AccountId) -> ClientSnapshot {
        self.run(Intent::Steward(StewardIntent::RotateRequested {
            request_nonce: ClientNonce::generate(),
            steward_id,
        }))
        .await
    }

    pub async fn revoke_steward(&self, steward_id: AccountId) -> ClientSnapshot {
        self.run(Intent::Steward(StewardIntent::RevokeRequested {
            request_nonce: ClientNonce::generate(),
            steward_id,
        }))
        .await
    }

    pub async fn assign_steward_rooms(
        &self,
        steward_id: AccountId,
        room_ids: Vec<RoomId>,
    ) -> ClientSnapshot {
        self.run(Intent::Steward(StewardIntent::RoomsAssigned {
            request_nonce: ClientNonce::generate(),
            steward_id,
            room_ids,
        }))
        .await
    }

    pub async fn dismiss_steward_token(&self) -> ClientSnapshot {
        self.run(Intent::Steward(StewardIntent::IssuedTokenDismissed))
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
