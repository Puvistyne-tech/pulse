use std::collections::HashSet;
use std::time::Duration;

use axum::extract::ws::{CloseFrame, Message, WebSocket};
use pulse_protocol::{
    AccountId, AckWork, AgentStreamChunk, CallTool, CampusId, ClientFrame, HandshakeRejected,
    HandshakeRejection, Hello, INBOX_SYNC_LIMIT, InboxSynced, JoinLounge, LeaveLounge,
    LoungeChanged, LoungeHeartbeat, LoungeId, LoungeRejected, LoungeRejection, LoungesSynced,
    ManageSteward, MarkThreadRead, MessageAccepted, MessageCreated, MessageId, MessageRejected,
    MessageRejection, MessageUpdated, Permission, PinRejected, Pong, ProtocolVersion, React,
    ReactionRejected, ReactionRejection, Role, RoomId, RoomUnavailable, SHELF_SYNC_LIMIT,
    STEWARD_TOKEN_PREFIX, SYNC_MAX_ROOMS, SendMessage, ServerFrame, SessionId, SetPin,
    SetThreadSubscription, SetTopicTitle, ShelfChanged, ShelfItem, ShelfRejected, ShelfSynced,
    StewardChanged, StewardIssued, StewardRejected, StewardsSynced, StreamAbortReason, StreamChunk,
    StreamPhase, StreamRejected, StreamRejection, SyncRooms, SyncShelf, SyncThread, SyncVault,
    ThreadChanged, ThreadRejected, ThreadRejection, ThreadSummary, ToolsListed, UpdateShelf,
    VaultRejected, VaultRejection, VersionMismatch, WORK_SYNC_LIMIT, Welcome, WorkSynced,
    close_code, role_permits, tool_descriptors,
};
use tokio::sync::broadcast::error::RecvError;

use crate::auth::{resolve_steward_token, resolve_token};
use crate::campus::campus_role;
use crate::fanout::FanoutEnvelope;
use crate::lounge::{LoungeFailure, heartbeat_lounge, join_lounge, leave_lounge, lounges_of};
use crate::message::{
    PostFailure, PostOutcome, is_top_level, post_message, send_message, sync_room, sync_thread,
};
use crate::pin::{PinFailure, PinOutcome, set_pin};
use crate::publish::{publish_created, publish_frame, publish_summaries};
use crate::reaction::{ReactionFailure, ReactionOutcome, apply_reaction};
use crate::shelf::{ShelfFailure, ShelfOutcome, current_item, shelf_items, update_shelf};
use crate::state::AppState;
use crate::steward::{
    StewardFailure, StewardOutcome, ack_work, credential_current, display_name_of, manage_steward,
    steward_profiles, steward_room_assigned, work_items,
};
use crate::stream::{OpenStream, OpenStreams, StreamStep};
use crate::thread::{
    ThreadFailure, mark_read, set_subscription, set_topic_title, thread_summaries,
};
use crate::time::now_ms;
use crate::tool::call_tool;
use crate::vault::sync_vault;

struct StewardSession {
    credential_revision: u64,
    display_name: String,
}

struct Subscriptions {
    campus_id: CampusId,
    account_id: AccountId,
    session_id: SessionId,
    steward: Option<StewardSession>,
    rooms: HashSet<RoomId>,
    streams: OpenStreams,
    lounges: HashSet<LoungeId>,
}

#[derive(Clone, Copy)]
struct StreamRef {
    stream_id: pulse_protocol::StreamId,
    room_id: RoomId,
    thread_root_id: Option<MessageId>,
    sequence: u32,
}

enum Flow {
    Continue,
    Stop,
}

enum Incoming {
    Frame(ClientFrame),
    Skip,
    Reject(u16),
    Stop,
}

async fn send_frame(socket: &mut WebSocket, frame: &ServerFrame) -> bool {
    match serde_json::to_string(frame) {
        Ok(text) => socket.send(Message::Text(text.into())).await.is_ok(),
        Err(_) => false,
    }
}

async fn close_with(socket: &mut WebSocket, code: u16) {
    let frame = CloseFrame {
        code,
        reason: "".into(),
    };
    let _ = socket.send(Message::Close(Some(frame))).await;
}

async fn reply_with(socket: &mut WebSocket, frame: &ServerFrame) -> Flow {
    if send_frame(socket, frame).await {
        Flow::Continue
    } else {
        Flow::Stop
    }
}

async fn stop_with(socket: &mut WebSocket, code: u16) -> Flow {
    close_with(socket, code).await;
    Flow::Stop
}

async fn reject(socket: &mut WebSocket, reason: HandshakeRejection) {
    let rejected = ServerFrame::HandshakeRejected(HandshakeRejected { reason });
    if send_frame(socket, &rejected).await {
        close_with(socket, close_code::POLICY_VIOLATION).await;
    }
}

async fn accept_hello(
    socket: &mut WebSocket,
    state: &AppState,
    hello: Hello,
) -> Option<Subscriptions> {
    if hello.protocol_version != ProtocolVersion::CURRENT {
        let mismatch = ServerFrame::VersionMismatch(VersionMismatch {
            supported: ProtocolVersion::CURRENT,
            received: hello.protocol_version,
        });
        if send_frame(socket, &mismatch).await {
            close_with(socket, close_code::PROTOCOL_ERROR).await;
        }
        return None;
    }
    let credential = if hello.auth_token.0.starts_with(STEWARD_TOKEN_PREFIX) {
        match resolve_steward_token(&state.db, hello.campus_id, &hello.auth_token).await {
            Ok(credential) => credential,
            Err(_) => {
                close_with(socket, close_code::INTERNAL_ERROR).await;
                return None;
            }
        }
    } else {
        None
    };
    let (account_id, steward) = match credential {
        Some(credential) => {
            let name =
                match display_name_of(&state.db, hello.campus_id, credential.steward_id).await {
                    Ok(Some(name)) => name,
                    _ => {
                        close_with(socket, close_code::INTERNAL_ERROR).await;
                        return None;
                    }
                };
            (
                credential.steward_id,
                Some(StewardSession {
                    credential_revision: credential.credential_revision,
                    display_name: name,
                }),
            )
        }
        None => match resolve_token(&state.db, &hello.auth_token).await {
            Ok(Some(account_id)) => (account_id, None),
            Ok(None) => {
                reject(socket, HandshakeRejection::Unauthenticated).await;
                return None;
            }
            Err(_) => {
                close_with(socket, close_code::INTERNAL_ERROR).await;
                return None;
            }
        },
    };
    let role = match campus_role(&state.db, hello.campus_id, account_id).await {
        Ok(role) => role.filter(|role| role_permits(*role, Permission::ViewCampus)),
        Err(_) => {
            close_with(socket, close_code::INTERNAL_ERROR).await;
            return None;
        }
    };
    let Some(role) = role else {
        reject(socket, HandshakeRejection::CampusUnavailable).await;
        return None;
    };
    let session_id = SessionId::generate();
    let welcome = ServerFrame::Welcome(Welcome {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: hello.campus_id,
        session_id,
        account_id,
        role,
    });
    send_frame(socket, &welcome).await.then_some(Subscriptions {
        campus_id: hello.campus_id,
        account_id,
        session_id,
        steward,
        rooms: HashSet::new(),
        streams: OpenStreams::new(state.streams.idle_timeout_ms),
        lounges: HashSet::new(),
    })
}

async fn read_frame(socket: &mut WebSocket) -> Incoming {
    let text = match socket.recv().await {
        Some(Ok(Message::Text(text))) => text,
        Some(Ok(Message::Ping(_) | Message::Pong(_))) => return Incoming::Skip,
        Some(Ok(Message::Binary(_))) => return Incoming::Reject(close_code::INVALID_PAYLOAD),
        Some(Ok(Message::Close(_)) | Err(_)) | None => return Incoming::Stop,
    };
    match serde_json::from_str::<ClientFrame>(text.as_str()) {
        Ok(frame) => Incoming::Frame(frame),
        Err(_) => Incoming::Reject(close_code::INVALID_PAYLOAD),
    }
}

async fn await_hello(socket: &mut WebSocket, state: &AppState) -> Option<Subscriptions> {
    loop {
        match read_frame(socket).await {
            Incoming::Frame(ClientFrame::Hello(hello)) => {
                return accept_hello(socket, state, hello).await;
            }
            Incoming::Frame(_) => {
                close_with(socket, close_code::POLICY_VIOLATION).await;
                return None;
            }
            Incoming::Skip => {}
            Incoming::Reject(code) => {
                close_with(socket, code).await;
                return None;
            }
            Incoming::Stop => return None,
        }
    }
}

async fn room_open(
    state: &AppState,
    active: &Subscriptions,
    room_id: RoomId,
) -> Result<bool, sqlx::Error> {
    match active.steward {
        None => Ok(true),
        Some(_) => {
            steward_room_assigned(&state.db, active.campus_id, active.account_id, room_id).await
        }
    }
}

async fn permitted(
    state: &AppState,
    active: &Subscriptions,
    permission: Permission,
) -> Result<bool, sqlx::Error> {
    Ok(campus_role(&state.db, active.campus_id, active.account_id)
        .await?
        .is_some_and(|role| role_permits(role, permission)))
}

fn thread_rejected(
    room_id: RoomId,
    root_message_id: MessageId,
    reason: ThreadRejection,
) -> ServerFrame {
    ServerFrame::ThreadRejected(ThreadRejected {
        room_id,
        root_message_id,
        reason,
    })
}

async fn database_failure(socket: &mut WebSocket, context: &str, error: sqlx::Error) -> Flow {
    tracing::warn!(%error, context, "request failed");
    stop_with(socket, close_code::INTERNAL_ERROR).await
}

async fn handle_send(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: SendMessage,
) -> Flow {
    let client_nonce = request.client_nonce;
    match room_open(state, active, request.room_id).await {
        Ok(true) => {}
        Ok(false) => {
            let rejected = ServerFrame::MessageRejected(MessageRejected {
                client_nonce,
                reason: MessageRejection::Forbidden,
            });
            return reply_with(socket, &rejected).await;
        }
        Err(error) => return database_failure(socket, "room assignment", error).await,
    }
    let outcome = send_message(state, active.campus_id, active.account_id, request).await;
    let (reply, created) = match outcome {
        Ok(PostOutcome::Created(message, affected)) => (
            ServerFrame::MessageAccepted(MessageAccepted {
                message: message.clone(),
            }),
            Some((message, affected)),
        ),
        Ok(PostOutcome::Replayed(message)) => (
            ServerFrame::MessageAccepted(MessageAccepted { message }),
            None,
        ),
        Err(PostFailure::Rejected(reason)) => (
            ServerFrame::MessageRejected(MessageRejected {
                client_nonce,
                reason,
            }),
            None,
        ),
        Err(PostFailure::Database(error)) => {
            return database_failure(socket, "message post", error).await;
        }
    };
    let flow = reply_with(socket, &reply).await;
    if let Some((message, affected)) = created {
        publish_created(state, active.campus_id, message, &affected).await;
    }
    flow
}

async fn handle_react(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: React,
) -> Flow {
    let (room_id, message_id, emoji) = (request.room_id, request.message_id, request.emoji.clone());
    match room_open(state, active, room_id).await {
        Ok(true) => {}
        Ok(false) => {
            let rejected = ServerFrame::ReactionRejected(ReactionRejected {
                room_id,
                message_id,
                emoji,
                reason: ReactionRejection::Forbidden,
            });
            return reply_with(socket, &rejected).await;
        }
        Err(error) => return database_failure(socket, "room assignment", error).await,
    }
    match apply_reaction(&state.db, active.campus_id, active.account_id, request).await {
        Ok(ReactionOutcome::Changed(message, affected)) => {
            let root = message.thread_root_id.unwrap_or(message.message_id);
            let frame = ServerFrame::MessageUpdated(MessageUpdated { message });
            let flow = reply_with(socket, &frame).await;
            publish_frame(state, active.campus_id, None, frame).await;
            publish_summaries(state, active.campus_id, &affected, root).await;
            flow
        }
        Ok(ReactionOutcome::Unchanged(message)) => {
            reply_with(
                socket,
                &ServerFrame::MessageUpdated(MessageUpdated { message }),
            )
            .await
        }
        Err(ReactionFailure::Rejected(reason)) => {
            let rejected = ServerFrame::ReactionRejected(ReactionRejected {
                room_id,
                message_id,
                emoji,
                reason,
            });
            reply_with(socket, &rejected).await
        }
        Err(ReactionFailure::Database(error)) => database_failure(socket, "reaction", error).await,
    }
}

async fn handle_pin(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: SetPin,
) -> Flow {
    let (room_id, message_id) = (request.room_id, request.message_id);
    match set_pin(&state.db, active.campus_id, active.account_id, request).await {
        Ok(PinOutcome::Changed(message)) => {
            let frame = ServerFrame::MessageUpdated(MessageUpdated { message });
            let flow = reply_with(socket, &frame).await;
            publish_frame(state, active.campus_id, None, frame).await;
            flow
        }
        Ok(PinOutcome::Unchanged(message)) => {
            reply_with(
                socket,
                &ServerFrame::MessageUpdated(MessageUpdated { message }),
            )
            .await
        }
        Err(PinFailure::Rejected(reason)) => {
            let rejected = ServerFrame::PinRejected(PinRejected {
                room_id,
                message_id,
                reason,
            });
            reply_with(socket, &rejected).await
        }
        Err(PinFailure::Database(error)) => database_failure(socket, "pin", error).await,
    }
}

async fn handle_sync_vault(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: SyncVault,
) -> Flow {
    let rejected = |reason| {
        ServerFrame::VaultRejected(VaultRejected {
            room_id: request.room_id,
            tab: request.tab,
            reason,
        })
    };
    let reply = match permitted(state, active, Permission::ViewRooms).await {
        Ok(true) => match sync_vault(&state.db, active.campus_id, &request).await {
            Ok(Some(synced)) => ServerFrame::VaultSynced(synced),
            Ok(None) => rejected(VaultRejection::UnknownRoom),
            Err(error) => return database_failure(socket, "vault sync", error).await,
        },
        Ok(false) => rejected(VaultRejection::Forbidden),
        Err(error) => return database_failure(socket, "role lookup", error).await,
    };
    reply_with(socket, &reply).await
}

async fn publish_shelf_change(
    state: &AppState,
    active: &Subscriptions,
    item: ShelfItem,
    removed: bool,
) -> ServerFrame {
    let frame = ServerFrame::ShelfChanged(ShelfChanged { item, removed });
    publish_frame(
        state,
        active.campus_id,
        Some(active.account_id),
        frame.clone(),
    )
    .await;
    frame
}

async fn handle_update_shelf(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: UpdateShelf,
) -> Flow {
    let (room_id, message_id, action) = (request.room_id, request.message_id, request.action);
    let outcome = update_shelf(
        &state.db,
        active.campus_id,
        active.account_id,
        request,
        now_ms(),
    )
    .await;
    match outcome {
        Ok(ShelfOutcome::Changed(item)) => {
            let frame = publish_shelf_change(state, active, item, false).await;
            reply_with(socket, &frame).await
        }
        Ok(ShelfOutcome::Removed(item)) => {
            let frame = publish_shelf_change(state, active, item, true).await;
            reply_with(socket, &frame).await
        }
        Ok(ShelfOutcome::Unchanged) => {
            match current_item(&state.db, active.campus_id, active.account_id, message_id).await {
                Ok(Some(item)) => {
                    let frame = ServerFrame::ShelfChanged(ShelfChanged {
                        item,
                        removed: false,
                    });
                    reply_with(socket, &frame).await
                }
                Ok(None) => Flow::Continue,
                Err(error) => database_failure(socket, "shelf item", error).await,
            }
        }
        Err(ShelfFailure::Rejected(reason)) => {
            let rejected = ServerFrame::ShelfRejected(ShelfRejected {
                room_id,
                message_id,
                action,
                reason,
            });
            reply_with(socket, &rejected).await
        }
        Err(ShelfFailure::Database(error)) => database_failure(socket, "shelf update", error).await,
    }
}

async fn handle_sync_shelf(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
) -> Flow {
    let items = match permitted(state, active, Permission::ViewRooms).await {
        Ok(true) => {
            let mut connection = match state.db.acquire().await {
                Ok(connection) => connection,
                Err(error) => return database_failure(socket, "shelf connection", error).await,
            };
            match shelf_items(
                &mut connection,
                active.campus_id,
                active.account_id,
                None,
                SHELF_SYNC_LIMIT,
            )
            .await
            {
                Ok(items) => items,
                Err(error) => return database_failure(socket, "shelf sync", error).await,
            }
        }
        Ok(false) => Vec::new(),
        Err(error) => return database_failure(socket, "role lookup", error).await,
    };
    reply_with(socket, &ServerFrame::ShelfSynced(ShelfSynced { items })).await
}

async fn handle_title(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: SetTopicTitle,
) -> Flow {
    let (room_id, root_message_id) = (request.room_id, request.root_message_id);
    match set_topic_title(&state.db, active.campus_id, active.account_id, request).await {
        Ok(message) => {
            let frame = ServerFrame::MessageUpdated(MessageUpdated { message });
            let flow = reply_with(socket, &frame).await;
            publish_frame(state, active.campus_id, None, frame).await;
            flow
        }
        Err(ThreadFailure::Rejected(reason)) => {
            reply_with(socket, &thread_rejected(room_id, root_message_id, reason)).await
        }
        Err(ThreadFailure::Database(error)) => database_failure(socket, "topic title", error).await,
    }
}

async fn reply_summary(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    room_id: RoomId,
    root_message_id: MessageId,
    outcome: Result<Option<ThreadSummary>, ThreadFailure>,
) -> Flow {
    match outcome {
        Ok(Some(summary)) => {
            let frame = ServerFrame::ThreadChanged(ThreadChanged { summary });
            let flow = reply_with(socket, &frame).await;
            publish_frame(state, active.campus_id, Some(active.account_id), frame).await;
            flow
        }
        Ok(None) => Flow::Continue,
        Err(ThreadFailure::Rejected(reason)) => {
            reply_with(socket, &thread_rejected(room_id, root_message_id, reason)).await
        }
        Err(ThreadFailure::Database(error)) => {
            database_failure(socket, "thread subscription", error).await
        }
    }
}

async fn handle_subscription(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: SetThreadSubscription,
) -> Flow {
    let (room_id, root_message_id) = (request.room_id, request.root_message_id);
    let outcome = match permitted(state, active, Permission::ViewRooms).await {
        Ok(true) => set_subscription(&state.db, active.campus_id, active.account_id, request).await,
        Ok(false) => Err(ThreadFailure::Rejected(ThreadRejection::Forbidden)),
        Err(error) => return database_failure(socket, "role lookup", error).await,
    };
    reply_summary(socket, state, active, room_id, root_message_id, outcome).await
}

async fn handle_mark_read(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: MarkThreadRead,
) -> Flow {
    let (room_id, root_message_id) = (request.room_id, request.root_message_id);
    let outcome = match permitted(state, active, Permission::ViewRooms).await {
        Ok(true) => mark_read(&state.db, active.campus_id, active.account_id, request).await,
        Ok(false) => Err(ThreadFailure::Rejected(ThreadRejection::Forbidden)),
        Err(error) => return database_failure(socket, "role lookup", error).await,
    };
    reply_summary(socket, state, active, room_id, root_message_id, outcome).await
}

async fn handle_sync_thread(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: SyncThread,
) -> Flow {
    let assigned = match room_open(state, active, request.room_id).await {
        Ok(assigned) => assigned,
        Err(error) => return database_failure(socket, "room assignment", error).await,
    };
    let reply = match permitted(state, active, Permission::ViewRooms).await {
        Ok(true) if !assigned => thread_rejected(
            request.room_id,
            request.root_message_id,
            ThreadRejection::Forbidden,
        ),
        Ok(true) => match sync_thread(&state.db, active.campus_id, &request).await {
            Ok(Some(synced)) => ServerFrame::ThreadSynced(synced),
            Ok(None) => thread_rejected(
                request.room_id,
                request.root_message_id,
                ThreadRejection::UnknownThread,
            ),
            Err(error) => return database_failure(socket, "thread sync", error).await,
        },
        Ok(false) => thread_rejected(
            request.room_id,
            request.root_message_id,
            ThreadRejection::Forbidden,
        ),
        Err(error) => return database_failure(socket, "role lookup", error).await,
    };
    reply_with(socket, &reply).await
}

async fn handle_sync_inbox(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
) -> Flow {
    let threads = match permitted(state, active, Permission::ViewRooms).await {
        Ok(true) => {
            let mut connection = match state.db.acquire().await {
                Ok(connection) => connection,
                Err(error) => return database_failure(socket, "inbox connection", error).await,
            };
            match thread_summaries(
                &mut connection,
                active.campus_id,
                active.account_id,
                None,
                INBOX_SYNC_LIMIT,
            )
            .await
            {
                Ok(threads) => threads,
                Err(error) => return database_failure(socket, "inbox sync", error).await,
            }
        }
        Ok(false) => Vec::new(),
        Err(error) => return database_failure(socket, "role lookup", error).await,
    };
    reply_with(socket, &ServerFrame::InboxSynced(InboxSynced { threads })).await
}

async fn handle_sync(
    socket: &mut WebSocket,
    state: &AppState,
    active: &mut Subscriptions,
    request: SyncRooms,
) -> Flow {
    if request.cursors.len() > SYNC_MAX_ROOMS {
        return stop_with(socket, close_code::POLICY_VIOLATION).await;
    }
    let can_view = match permitted(state, active, Permission::ViewRooms).await {
        Ok(can_view) => can_view,
        Err(_) => return stop_with(socket, close_code::INTERNAL_ERROR).await,
    };
    let mut seen = HashSet::new();
    for cursor in request.cursors {
        if !seen.insert(cursor.room_id) {
            continue;
        }
        let unavailable = ServerFrame::RoomUnavailable(RoomUnavailable {
            room_id: cursor.room_id,
        });
        let assigned = if can_view {
            match room_open(state, active, cursor.room_id).await {
                Ok(assigned) => assigned,
                Err(error) => return database_failure(socket, "room assignment", error).await,
            }
        } else {
            false
        };
        let reply = if assigned {
            active.rooms.insert(cursor.room_id);
            match sync_room(&state.db, active.campus_id, &cursor).await {
                Ok(Some(synced)) => ServerFrame::RoomSynced(synced),
                Ok(None) => {
                    active.rooms.remove(&cursor.room_id);
                    unavailable
                }
                Err(error) => return database_failure(socket, "room sync", error).await,
            }
        } else {
            unavailable
        };
        if matches!(reply_with(socket, &reply).await, Flow::Stop) {
            return Flow::Stop;
        }
    }
    Flow::Continue
}

fn steward_frame_allowed(frame: &ClientFrame) -> bool {
    matches!(
        frame,
        ClientFrame::Ping(_)
            | ClientFrame::SyncRooms(_)
            | ClientFrame::SendMessage(_)
            | ClientFrame::React(_)
            | ClientFrame::SyncThread(_)
            | ClientFrame::StreamChunk(_)
            | ClientFrame::ListTools(_)
            | ClientFrame::CallTool(_)
            | ClientFrame::SyncWork(_)
            | ClientFrame::AckWork(_)
    )
}

fn agent_chunk(
    active: &Subscriptions,
    base: StreamRef,
    phase: StreamPhase,
    text: String,
    message_id: Option<MessageId>,
    abort_reason: Option<StreamAbortReason>,
) -> ServerFrame {
    ServerFrame::AgentStreamChunk(AgentStreamChunk {
        campus_id: active.campus_id,
        stream_id: base.stream_id,
        room_id: base.room_id,
        thread_root_id: base.thread_root_id,
        steward_id: active.account_id,
        steward_display_name: active
            .steward
            .as_ref()
            .map(|steward| steward.display_name.clone())
            .unwrap_or_default(),
        phase,
        sequence: base.sequence,
        text,
        message_id,
        abort_reason,
    })
}

async fn publish_abort(
    state: &AppState,
    active: &Subscriptions,
    base: StreamRef,
    reason: StreamAbortReason,
) {
    let frame = agent_chunk(
        active,
        base,
        StreamPhase::Abort,
        String::new(),
        None,
        Some(reason),
    );
    publish_frame(state, active.campus_id, None, frame).await;
}

async fn abort_streams(
    state: &AppState,
    active: &Subscriptions,
    streams: Vec<(pulse_protocol::StreamId, OpenStream)>,
    reason: StreamAbortReason,
) {
    for (stream_id, open) in streams {
        let base = StreamRef {
            stream_id,
            room_id: open.room_id,
            thread_root_id: open.thread_root_id,
            sequence: open.next_sequence,
        };
        publish_abort(state, active, base, reason).await;
    }
}

async fn admit_stream(
    state: &AppState,
    active: &Subscriptions,
    room_id: RoomId,
    thread_root_id: Option<MessageId>,
) -> Result<Option<StreamRejection>, sqlx::Error> {
    if !permitted(state, active, Permission::StreamReply).await?
        || !room_open(state, active, room_id).await?
    {
        return Ok(Some(StreamRejection::Forbidden));
    }
    if let Some(root) = thread_root_id {
        let mut connection = state.db.acquire().await?;
        if !is_top_level(&mut connection, active.campus_id, room_id, root).await? {
            return Ok(Some(StreamRejection::UnknownThread));
        }
    }
    Ok(None)
}

async fn finalize_stream(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    base: StreamRef,
    open: OpenStream,
) -> Flow {
    let reject = |reason| {
        ServerFrame::StreamRejected(StreamRejected {
            stream_id: base.stream_id,
            reason,
        })
    };
    let assigned = match room_open(state, active, open.room_id).await {
        Ok(assigned) => assigned,
        Err(error) => return database_failure(socket, "room assignment", error).await,
    };
    if !assigned {
        publish_abort(state, active, base, StreamAbortReason::Rejected).await;
        return reply_with(socket, &reject(StreamRejection::Forbidden)).await;
    }
    let request = SendMessage {
        room_id: open.room_id,
        client_nonce: base.stream_id.client_nonce(),
        body: open.body,
        thread_root_id: open.thread_root_id,
        attachment_ids: Vec::new(),
    };
    let message = match post_message(&state.db, active.campus_id, active.account_id, request).await
    {
        Ok(PostOutcome::Created(message, affected)) => {
            publish_created(state, active.campus_id, message.clone(), &affected).await;
            message
        }
        Ok(PostOutcome::Replayed(message)) => message,
        Err(PostFailure::Rejected(reason)) => {
            let (rejection, abort) = match reason {
                MessageRejection::Forbidden => {
                    (StreamRejection::Forbidden, StreamAbortReason::Revoked)
                }
                MessageRejection::UnknownRoom => {
                    (StreamRejection::UnknownRoom, StreamAbortReason::Rejected)
                }
                MessageRejection::UnknownThread => {
                    (StreamRejection::UnknownThread, StreamAbortReason::Rejected)
                }
                _ => (StreamRejection::InvalidBody, StreamAbortReason::Rejected),
            };
            publish_abort(state, active, base, abort).await;
            return reply_with(socket, &reject(rejection)).await;
        }
        Err(PostFailure::Database(error)) => {
            publish_abort(state, active, base, StreamAbortReason::Rejected).await;
            return database_failure(socket, "stream finalize", error).await;
        }
    };
    let frame = agent_chunk(
        active,
        base,
        StreamPhase::End,
        String::new(),
        Some(message.message_id),
        None,
    );
    publish_frame(state, active.campus_id, None, frame).await;
    Flow::Continue
}

async fn handle_stream(
    socket: &mut WebSocket,
    state: &AppState,
    active: &mut Subscriptions,
    chunk: StreamChunk,
) -> Flow {
    let stream_id = chunk.stream_id;
    let reject = |reason| ServerFrame::StreamRejected(StreamRejected { stream_id, reason });
    if active.steward.is_none() {
        return reply_with(socket, &reject(StreamRejection::Forbidden)).await;
    }
    let admission = match (chunk.phase, active.streams.peek(stream_id)) {
        (StreamPhase::Begin, _) => {
            Some(admit_stream(state, active, chunk.room_id, chunk.thread_root_id).await)
        }
        (StreamPhase::Abort, _) | (_, None) => None,
        (_, Some(open)) => Some(admit_stream(state, active, open.room_id, None).await),
    };
    match admission {
        None | Some(Ok(None)) => {}
        Some(Ok(Some(reason))) => {
            let streams = active.streams.take_all();
            abort_streams(state, active, streams, StreamAbortReason::Revoked).await;
            return reply_with(socket, &reject(reason)).await;
        }
        Some(Err(error)) => return database_failure(socket, "stream admission", error).await,
    }
    let before = active.streams.peek(stream_id).map(|open| StreamRef {
        stream_id,
        room_id: open.room_id,
        thread_root_id: open.thread_root_id,
        sequence: open.next_sequence,
    });
    let base = StreamRef {
        stream_id,
        room_id: chunk.room_id,
        thread_root_id: chunk.thread_root_id,
        sequence: chunk.sequence,
    };
    match active.streams.accept(&chunk, now_ms()) {
        Ok(StreamStep::Opened | StreamStep::Appended) => {
            let Some(base) = active.streams.peek(stream_id).map(|open| StreamRef {
                room_id: open.room_id,
                thread_root_id: open.thread_root_id,
                ..base
            }) else {
                return Flow::Continue;
            };
            let frame = agent_chunk(active, base, chunk.phase, chunk.text, None, None);
            if publish_frame(state, active.campus_id, None, frame).await {
                Flow::Continue
            } else {
                active.streams.discard(stream_id);
                reply_with(socket, &reject(StreamRejection::UnknownStream)).await
            }
        }
        Ok(StreamStep::Cancelled(open)) => {
            let base = StreamRef {
                room_id: open.room_id,
                thread_root_id: open.thread_root_id,
                ..base
            };
            publish_abort(state, active, base, StreamAbortReason::Cancelled).await;
            Flow::Continue
        }
        Ok(StreamStep::Closing(open)) => {
            let base = StreamRef {
                room_id: open.room_id,
                thread_root_id: open.thread_root_id,
                ..base
            };
            finalize_stream(socket, state, active, base, open).await
        }
        Err(reason) => {
            let flow = reply_with(socket, &reject(reason)).await;
            if let Some(base) = before
                && active.streams.peek(stream_id).is_none()
            {
                publish_abort(state, active, base, StreamAbortReason::Rejected).await;
            }
            flow
        }
    }
}

async fn handle_call_tool(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: CallTool,
) -> Flow {
    let result = call_tool(state, active.campus_id, active.account_id, request).await;
    reply_with(socket, &ServerFrame::ToolResult(result)).await
}

async fn handle_sync_work(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
) -> Flow {
    let synced = if active.steward.is_some() {
        match work_items(
            &state.db,
            active.campus_id,
            active.account_id,
            WORK_SYNC_LIMIT,
        )
        .await
        {
            Ok(synced) => synced,
            Err(error) => return database_failure(socket, "work sync", error).await,
        }
    } else {
        WorkSynced {
            items: Vec::new(),
            has_more: false,
        }
    };
    reply_with(socket, &ServerFrame::WorkSynced(synced)).await
}

async fn handle_ack_work(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: AckWork,
) -> Flow {
    if active.steward.is_none() {
        return Flow::Continue;
    }
    match ack_work(
        &state.db,
        active.campus_id,
        active.account_id,
        request.through,
    )
    .await
    {
        Ok(()) => Flow::Continue,
        Err(error) => database_failure(socket, "work ack", error).await,
    }
}

async fn handle_manage_steward(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: ManageSteward,
) -> Flow {
    let (request_nonce, action) = (request.request_nonce, request.action);
    match manage_steward(&state.db, active.campus_id, active.account_id, request).await {
        Ok(StewardOutcome::Issued(steward, token)) => {
            let issued = ServerFrame::StewardIssued(StewardIssued {
                request_nonce,
                steward: steward.clone(),
                token,
            });
            let flow = reply_with(socket, &issued).await;
            let changed = ServerFrame::StewardChanged(StewardChanged { steward });
            publish_frame(state, active.campus_id, None, changed).await;
            flow
        }
        Ok(StewardOutcome::Changed(steward)) => {
            let changed = ServerFrame::StewardChanged(StewardChanged { steward });
            publish_frame(state, active.campus_id, None, changed).await;
            Flow::Continue
        }
        Err(StewardFailure::Rejected(reason)) => {
            let rejected = ServerFrame::StewardRejected(StewardRejected {
                request_nonce,
                action,
                reason,
            });
            reply_with(socket, &rejected).await
        }
        Err(StewardFailure::Database(error)) => {
            database_failure(socket, "steward management", error).await
        }
    }
}

async fn handle_sync_stewards(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
) -> Flow {
    let stewards = match permitted(state, active, Permission::ViewCampus).await {
        Ok(true) => match steward_profiles(&state.db, active.campus_id).await {
            Ok(stewards) => stewards,
            Err(error) => return database_failure(socket, "steward sync", error).await,
        },
        Ok(false) => Vec::new(),
        Err(error) => return database_failure(socket, "role lookup", error).await,
    };
    reply_with(
        socket,
        &ServerFrame::StewardsSynced(StewardsSynced { stewards }),
    )
    .await
}

async fn handle_sync_lounges(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
) -> Flow {
    let allowed = match permitted(state, active, Permission::ViewCampus).await {
        Ok(allowed) => allowed,
        Err(error) => return database_failure(socket, "role lookup", error).await,
    };
    let lounges = if allowed {
        match lounges_of(state, active.campus_id).await {
            Ok(lounges) => lounges,
            Err(LoungeFailure::Rejected(_)) => Vec::new(),
            Err(LoungeFailure::Database(error)) => {
                return database_failure(socket, "lounge sync", error).await;
            }
        }
    } else {
        Vec::new()
    };
    reply_with(
        socket,
        &ServerFrame::LoungesSynced(LoungesSynced { lounges }),
    )
    .await
}

fn lounge_rejected(lounge_id: LoungeId, reason: LoungeRejection) -> ServerFrame {
    ServerFrame::LoungeRejected(LoungeRejected { lounge_id, reason })
}

async fn current_role(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    lounge_id: LoungeId,
) -> Result<Role, Flow> {
    match campus_role(&state.db, active.campus_id, active.account_id).await {
        Ok(Some(role)) => Ok(role),
        Ok(None) => Err(reply_with(
            socket,
            &lounge_rejected(lounge_id, LoungeRejection::Forbidden),
        )
        .await),
        Err(error) => Err(database_failure(socket, "role lookup", error).await),
    }
}

async fn publish_lounge(state: &AppState, active: &Subscriptions, lounge: pulse_protocol::Lounge) {
    let frame = ServerFrame::LoungeChanged(LoungeChanged { lounge });
    publish_frame(state, active.campus_id, None, frame).await;
}

async fn handle_join_lounge(
    socket: &mut WebSocket,
    state: &AppState,
    active: &mut Subscriptions,
    request: JoinLounge,
) -> Flow {
    let lounge_id = request.lounge_id;
    let role = match current_role(socket, state, active, lounge_id).await {
        Ok(role) => role,
        Err(flow) => return flow,
    };
    match join_lounge(
        state,
        active.campus_id,
        active.account_id,
        active.session_id,
        role,
        lounge_id,
    )
    .await
    {
        Ok(joined) => {
            active.lounges.insert(lounge_id);
            let lounge = joined.lounge.clone();
            let flow = reply_with(socket, &ServerFrame::LoungeJoined(joined)).await;
            publish_lounge(state, active, lounge).await;
            flow
        }
        Err(LoungeFailure::Rejected(reason)) => {
            reply_with(socket, &lounge_rejected(lounge_id, reason)).await
        }
        Err(LoungeFailure::Database(error)) => database_failure(socket, "lounge join", error).await,
    }
}

async fn handle_lounge_heartbeat(
    socket: &mut WebSocket,
    state: &AppState,
    active: &Subscriptions,
    request: LoungeHeartbeat,
) -> Flow {
    let lounge_id = request.lounge_id;
    if !active.lounges.contains(&lounge_id) {
        return reply_with(
            socket,
            &lounge_rejected(lounge_id, LoungeRejection::UnknownLounge),
        )
        .await;
    }
    let role = match current_role(socket, state, active, lounge_id).await {
        Ok(role) => role,
        Err(flow) => return flow,
    };
    match heartbeat_lounge(
        state,
        active.campus_id,
        active.account_id,
        active.session_id,
        role,
        lounge_id,
    )
    .await
    {
        Ok(lounge) => {
            publish_lounge(state, active, lounge).await;
            Flow::Continue
        }
        Err(LoungeFailure::Rejected(reason)) => {
            reply_with(socket, &lounge_rejected(lounge_id, reason)).await
        }
        Err(LoungeFailure::Database(error)) => {
            database_failure(socket, "lounge heartbeat", error).await
        }
    }
}

async fn handle_leave_lounge(
    socket: &mut WebSocket,
    state: &AppState,
    active: &mut Subscriptions,
    request: LeaveLounge,
) -> Flow {
    let lounge_id = request.lounge_id;
    if !active.lounges.remove(&lounge_id) {
        return Flow::Continue;
    }
    match leave_lounge(
        state,
        active.campus_id,
        active.account_id,
        active.session_id,
        lounge_id,
    )
    .await
    {
        Ok(lounge) => {
            publish_lounge(state, active, lounge).await;
            Flow::Continue
        }
        Err(LoungeFailure::Rejected(reason)) => {
            reply_with(socket, &lounge_rejected(lounge_id, reason)).await
        }
        Err(LoungeFailure::Database(error)) => {
            database_failure(socket, "lounge leave", error).await
        }
    }
}

async fn release_socket(state: &AppState, active: &mut Subscriptions) {
    let streams = active.streams.take_all();
    abort_streams(state, active, streams, StreamAbortReason::Disconnected).await;
    let lounges: Vec<LoungeId> = active.lounges.drain().collect();
    for lounge_id in lounges {
        if let Ok(lounge) = leave_lounge(
            state,
            active.campus_id,
            active.account_id,
            active.session_id,
            lounge_id,
        )
        .await
        {
            publish_lounge(state, active, lounge).await;
        }
    }
}

async fn expire_streams(state: &AppState, active: &mut Subscriptions) {
    let expired = active.streams.expire(now_ms());
    abort_streams(state, active, expired, StreamAbortReason::TimedOut).await;
}

async fn stream_deadline(deadline_ms: Option<u64>) {
    match deadline_ms {
        Some(deadline) => {
            tokio::time::sleep(Duration::from_millis(deadline.saturating_sub(now_ms()))).await;
        }
        None => std::future::pending::<()>().await,
    }
}

async fn handle_frame(
    socket: &mut WebSocket,
    state: &AppState,
    active: &mut Subscriptions,
    frame: ClientFrame,
) -> Flow {
    if active.steward.is_some() && !steward_frame_allowed(&frame) {
        return stop_with(socket, close_code::POLICY_VIOLATION).await;
    }
    match frame {
        ClientFrame::Ping(ping) => {
            reply_with(socket, &ServerFrame::Pong(Pong { nonce: ping.nonce })).await
        }
        ClientFrame::SendMessage(request) => handle_send(socket, state, active, request).await,
        ClientFrame::SyncRooms(request) => handle_sync(socket, state, active, request).await,
        ClientFrame::React(request) => handle_react(socket, state, active, request).await,
        ClientFrame::SetTopicTitle(request) => handle_title(socket, state, active, request).await,
        ClientFrame::SetThreadSubscription(request) => {
            handle_subscription(socket, state, active, request).await
        }
        ClientFrame::MarkThreadRead(request) => {
            handle_mark_read(socket, state, active, request).await
        }
        ClientFrame::SyncThread(request) => {
            handle_sync_thread(socket, state, active, request).await
        }
        ClientFrame::SyncInbox(_) => handle_sync_inbox(socket, state, active).await,
        ClientFrame::SetPin(request) => handle_pin(socket, state, active, request).await,
        ClientFrame::SyncVault(request) => handle_sync_vault(socket, state, active, request).await,
        ClientFrame::UpdateShelf(request) => {
            handle_update_shelf(socket, state, active, request).await
        }
        ClientFrame::SyncShelf(SyncShelf {}) => handle_sync_shelf(socket, state, active).await,
        ClientFrame::StreamChunk(chunk) => handle_stream(socket, state, active, chunk).await,
        ClientFrame::ListTools(_) => {
            let listed = ServerFrame::ToolsListed(ToolsListed {
                tools: tool_descriptors(),
            });
            reply_with(socket, &listed).await
        }
        ClientFrame::CallTool(request) => handle_call_tool(socket, state, active, request).await,
        ClientFrame::SyncWork(_) => handle_sync_work(socket, state, active).await,
        ClientFrame::AckWork(request) => handle_ack_work(socket, state, active, request).await,
        ClientFrame::ManageSteward(request) => {
            handle_manage_steward(socket, state, active, request).await
        }
        ClientFrame::SyncStewards(_) => handle_sync_stewards(socket, state, active).await,
        ClientFrame::SyncLounges(_) => handle_sync_lounges(socket, state, active).await,
        ClientFrame::JoinLounge(request) => {
            handle_join_lounge(socket, state, active, request).await
        }
        ClientFrame::LeaveLounge(request) => {
            handle_leave_lounge(socket, state, active, request).await
        }
        ClientFrame::LoungeHeartbeat(request) => {
            handle_lounge_heartbeat(socket, state, active, request).await
        }
        ClientFrame::Hello(_) => stop_with(socket, close_code::POLICY_VIOLATION).await,
    }
}

fn deliverable(active: &Subscriptions, envelope: &FanoutEnvelope) -> bool {
    if envelope.campus_id != active.campus_id {
        return false;
    }
    match &envelope.frame {
        ServerFrame::MessageCreated(MessageCreated { message })
        | ServerFrame::MessageUpdated(MessageUpdated { message }) => {
            message.campus_id == active.campus_id && active.rooms.contains(&message.room_id)
        }
        ServerFrame::ThreadChanged(_) | ServerFrame::ShelfChanged(_) => {
            envelope.account_id == Some(active.account_id)
        }
        ServerFrame::StewardWorkItem(item) => {
            item.message.campus_id == active.campus_id
                && envelope.account_id == Some(active.account_id)
        }
        ServerFrame::AgentStreamChunk(chunk) => {
            chunk.campus_id == active.campus_id && active.rooms.contains(&chunk.room_id)
        }
        ServerFrame::StewardChanged(_) => true,
        ServerFrame::LoungeChanged(changed) => {
            active.steward.is_none() && changed.lounge.campus_id == active.campus_id
        }
        _ => false,
    }
}

async fn forward_feed(
    socket: &mut WebSocket,
    state: &AppState,
    active: &mut Subscriptions,
    received: Result<FanoutEnvelope, RecvError>,
) -> Flow {
    let Ok(envelope) = received else {
        return stop_with(socket, close_code::TRY_AGAIN_LATER).await;
    };
    if !deliverable(active, &envelope) {
        return Flow::Continue;
    }
    let flow = reply_with(socket, &envelope.frame).await;
    if matches!(flow, Flow::Stop) {
        return flow;
    }
    let (ServerFrame::StewardChanged(changed), Some(session)) =
        (&envelope.frame, active.steward.as_ref())
    else {
        return flow;
    };
    if changed.steward.steward_id != active.account_id {
        return flow;
    }
    if changed.steward.revoked || changed.steward.credential_revision > session.credential_revision
    {
        let streams = active.streams.take_all();
        abort_streams(state, active, streams, StreamAbortReason::Revoked).await;
        return stop_with(socket, close_code::POLICY_VIOLATION).await;
    }
    active
        .rooms
        .retain(|room_id| changed.steward.room_ids.contains(room_id));
    let pruned = active.streams.take_outside(&changed.steward.room_ids);
    abort_streams(state, active, pruned, StreamAbortReason::Rejected).await;
    flow
}

pub async fn serve_socket(mut socket: WebSocket, state: AppState) {
    let Some(mut active) = await_hello(&mut socket, &state).await else {
        return;
    };
    let mut feed = match state.fanout.join(active.campus_id).await {
        Ok(feed) => feed,
        Err(error) => {
            tracing::warn!(%error, "campus feed unavailable");
            close_with(&mut socket, close_code::INTERNAL_ERROR).await;
            return;
        }
    };
    if let Some(session) = active.steward.as_ref() {
        let current = credential_current(
            &state.db,
            active.campus_id,
            active.account_id,
            session.credential_revision,
        )
        .await;
        if !matches!(current, Ok(true)) {
            close_with(&mut socket, close_code::POLICY_VIOLATION).await;
            return;
        }
    }
    loop {
        let deadline = active.streams.next_deadline_ms();
        let flow = tokio::select! {
            incoming = read_frame(&mut socket) => match incoming {
                Incoming::Frame(frame) => {
                    handle_frame(&mut socket, &state, &mut active, frame).await
                }
                Incoming::Skip => Flow::Continue,
                Incoming::Reject(code) => stop_with(&mut socket, code).await,
                Incoming::Stop => Flow::Stop,
            },
            received = feed.recv() => {
                forward_feed(&mut socket, &state, &mut active, received).await
            }
            () = stream_deadline(deadline) => {
                expire_streams(&state, &mut active).await;
                Flow::Continue
            }
        };
        if matches!(flow, Flow::Stop) {
            break;
        }
    }
    release_socket(&state, &mut active).await;
}
