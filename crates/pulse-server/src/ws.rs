use std::collections::HashSet;

use axum::extract::ws::{CloseFrame, Message, WebSocket};
use pulse_protocol::{
    AccountId, CampusId, ClientFrame, HandshakeRejected, HandshakeRejection, Hello,
    MessageAccepted, MessageCreated, MessageRejected, Permission, Pong, ProtocolVersion, RoomId,
    RoomMessage, RoomUnavailable, SYNC_MAX_ROOMS, SendMessage, ServerFrame, SessionId, SyncRooms,
    VersionMismatch, Welcome, close_code, role_permits,
};
use tokio::sync::broadcast::error::RecvError;

use crate::auth::resolve_token;
use crate::campus::campus_role;
use crate::message::{PostFailure, PostOutcome, post_message, sync_room};
use crate::state::AppState;

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

async fn publish_created(state: &AppState, message: &RoomMessage) {
    if let Err(error) = state.fanout.publish(message).await {
        tracing::warn!(%error, "message fanout failed");
    }
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
        Ok(PostOutcome::Created(message)) => (
            ServerFrame::MessageAccepted(MessageAccepted {
                message: message.clone(),
            }),
            Some(message),
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
            tracing::warn!(%error, "message post failed");
            return stop_with(socket, close_code::INTERNAL_ERROR).await;
        }
    };
    let flow = reply_with(socket, &reply).await;
    if let Some(message) = created {
        publish_created(state, &message).await;
    }
    flow
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
    let can_view = match campus_role(&state.db, active.campus_id, active.account_id).await {
        Ok(role) => role.is_some_and(|role| role_permits(role, Permission::ViewRooms)),
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
                Err(error) => {
                    tracing::warn!(%error, "room sync failed");
                    return stop_with(socket, close_code::INTERNAL_ERROR).await;
                }
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
        ClientFrame::Hello(_) => stop_with(socket, close_code::POLICY_VIOLATION).await,
    }
}

async fn forward_feed(
    socket: &mut WebSocket,
    active: &Subscriptions,
    received: Result<RoomMessage, RecvError>,
) -> Flow {
    match received {
        Ok(message) => {
            if message.campus_id != active.campus_id || !active.rooms.contains(&message.room_id) {
                return Flow::Continue;
            }
            reply_with(
                socket,
                &ServerFrame::MessageCreated(MessageCreated { message }),
            )
            .await
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
