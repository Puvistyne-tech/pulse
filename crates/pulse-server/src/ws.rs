use axum::extract::ws::{CloseFrame, Message, WebSocket};
use pulse_protocol::{
    CampusId, ClientFrame, HandshakeRejected, HandshakeRejection, Hello, Permission, Pong,
    ProtocolVersion, ServerFrame, SessionId, VersionMismatch, Welcome, close_code, role_permits,
};

use crate::auth::resolve_token;
use crate::campus::campus_role;
use crate::state::AppState;

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

async fn reject(socket: &mut WebSocket, reason: HandshakeRejection) {
    let rejected = ServerFrame::HandshakeRejected(HandshakeRejected { reason });
    if send_frame(socket, &rejected).await {
        close_with(socket, close_code::POLICY_VIOLATION).await;
    }
}

async fn accept_hello(socket: &mut WebSocket, state: &AppState, hello: Hello) -> Option<CampusId> {
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
    send_frame(socket, &welcome)
        .await
        .then_some(hello.campus_id)
}

pub async fn serve_socket(mut socket: WebSocket, state: AppState) {
    let mut campus: Option<CampusId> = None;
    while let Some(Ok(message)) = socket.recv().await {
        let text = match message {
            Message::Text(text) => text,
            Message::Close(_) => return,
            Message::Ping(_) | Message::Pong(_) => continue,
            Message::Binary(_) => {
                close_with(&mut socket, close_code::INVALID_PAYLOAD).await;
                return;
            }
        };
        let Ok(frame) = serde_json::from_str::<ClientFrame>(text.as_str()) else {
            close_with(&mut socket, close_code::INVALID_PAYLOAD).await;
            return;
        };
        match (frame, campus) {
            (ClientFrame::Hello(hello), None) => {
                campus = accept_hello(&mut socket, &state, hello).await;
                if campus.is_none() {
                    return;
                }
            }
            (ClientFrame::Ping(ping), Some(_)) => {
                let pong = ServerFrame::Pong(Pong { nonce: ping.nonce });
                if !send_frame(&mut socket, &pong).await {
                    return;
                }
            }
            (ClientFrame::Hello(_), Some(_)) | (ClientFrame::Ping(_), None) => {
                close_with(&mut socket, close_code::POLICY_VIOLATION).await;
                return;
            }
        }
    }
}
