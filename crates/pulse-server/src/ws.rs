use std::collections::HashSet;

use axum::extract::ws::{CloseFrame, Message, WebSocket};
use pulse_protocol::{
    AccountId, CampusId, ClientFrame, HandshakeRejected, HandshakeRejection, Hello,
    INBOX_SYNC_LIMIT, InboxSynced, MarkThreadRead, MessageAccepted, MessageCreated, MessageId,
    MessageRejected, MessageUpdated, Permission, Pong, ProtocolVersion, React, ReactionRejected,
    RoomId, RoomUnavailable, SYNC_MAX_ROOMS, SendMessage, ServerFrame, SessionId,
    SetThreadSubscription, SetTopicTitle, SyncRooms, SyncThread, ThreadChanged, ThreadRejected,
    ThreadRejection, ThreadSummary, VersionMismatch, Welcome, close_code, role_permits,
};
use tokio::sync::broadcast::error::RecvError;

use crate::auth::resolve_token;
use crate::campus::campus_role;
use crate::fanout::FanoutEnvelope;
use crate::message::{PostFailure, PostOutcome, post_message, sync_room, sync_thread};
use crate::reaction::{ReactionFailure, ReactionOutcome, apply_reaction};
use crate::state::AppState;
use crate::thread::{
    ThreadFailure, mark_read, set_subscription, set_topic_title, thread_summaries,
};

struct Subscriptions {
    campus_id: CampusId,
    account_id: AccountId,
    rooms: HashSet<RoomId>,
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
    let account_id = match resolve_token(&state.db, &hello.auth_token).await {
        Ok(Some(account_id)) => account_id,
        Ok(None) => {
            reject(socket, HandshakeRejection::Unauthenticated).await;
            return None;
        }
        Err(_) => {
            close_with(socket, close_code::INTERNAL_ERROR).await;
            return None;
        }
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
    let welcome = ServerFrame::Welcome(Welcome {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id: hello.campus_id,
        session_id: SessionId::generate(),
        account_id,
        role,
    });
    send_frame(socket, &welcome).await.then_some(Subscriptions {
        campus_id: hello.campus_id,
        account_id,
        rooms: HashSet::new(),
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

async fn publish_frame(
    state: &AppState,
    campus_id: CampusId,
    account_id: Option<AccountId>,
    frame: ServerFrame,
) {
    let envelope = FanoutEnvelope {
        campus_id,
        account_id,
        frame,
    };
    if let Err(error) = state.fanout.publish(&envelope).await {
        tracing::warn!(%error, "fanout failed");
    }
}

async fn publish_summaries(
    state: &AppState,
    campus_id: CampusId,
    accounts: &[AccountId],
    root: MessageId,
) {
    if accounts.is_empty() {
        return;
    }
    let mut connection = match state.db.acquire().await {
        Ok(connection) => connection,
        Err(error) => {
            tracing::warn!(%error, "thread summary connection failed");
            return;
        }
    };
    for account_id in accounts {
        match thread_summaries(&mut connection, campus_id, *account_id, Some(&[root]), 1).await {
            Ok(summaries) => {
                for summary in summaries {
                    publish_frame(
                        state,
                        campus_id,
                        Some(*account_id),
                        ServerFrame::ThreadChanged(ThreadChanged { summary }),
                    )
                    .await;
                }
            }
            Err(error) => tracing::warn!(%error, "thread summary failed"),
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
    let outcome = post_message(&state.db, active.campus_id, active.account_id, request).await;
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
        let root = message.thread_root_id.unwrap_or(message.message_id);
        publish_frame(
            state,
            active.campus_id,
            None,
            ServerFrame::MessageCreated(MessageCreated { message }),
        )
        .await;
        publish_summaries(state, active.campus_id, &affected, root).await;
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
    let reply = match permitted(state, active, Permission::ViewRooms).await {
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
        let reply = if can_view {
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

async fn handle_frame(
    socket: &mut WebSocket,
    state: &AppState,
    active: &mut Subscriptions,
    frame: ClientFrame,
) -> Flow {
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
        ServerFrame::ThreadChanged(_) => envelope.account_id == Some(active.account_id),
        _ => false,
    }
}

async fn forward_feed(
    socket: &mut WebSocket,
    active: &Subscriptions,
    received: Result<FanoutEnvelope, RecvError>,
) -> Flow {
    match received {
        Ok(envelope) => {
            if !deliverable(active, &envelope) {
                return Flow::Continue;
            }
            reply_with(socket, &envelope.frame).await
        }
        Err(_) => stop_with(socket, close_code::TRY_AGAIN_LATER).await,
    }
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
    loop {
        let flow = tokio::select! {
            incoming = read_frame(&mut socket) => match incoming {
                Incoming::Frame(frame) => {
                    handle_frame(&mut socket, &state, &mut active, frame).await
                }
                Incoming::Skip => Flow::Continue,
                Incoming::Reject(code) => stop_with(&mut socket, code).await,
                Incoming::Stop => Flow::Stop,
            },
            received = feed.recv() => forward_feed(&mut socket, &active, received).await,
        };
        if matches!(flow, Flow::Stop) {
            return;
        }
    }
}
