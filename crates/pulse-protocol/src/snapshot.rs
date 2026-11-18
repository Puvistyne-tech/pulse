use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::api::ApiError;
use crate::auth::AccountProfile;
use crate::campus::{CampusSummary, Room};
use crate::frames::{HandshakeRejection, VersionMismatch};
use crate::ids::{AccountId, CampusId, ClientNonce, MessageId, RoomId, SessionId};
use crate::messaging::MessageRejection;
use crate::version::ProtocolVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionPhase {
    Disconnected,
    Connecting,
    Handshaking,
    Connected,
    Backoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum AuthPhase {
    SignedOut,
    Authenticating,
    SignedIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ConnectionSnapshot {
    pub protocol_version: ProtocolVersion,
    pub connection: ConnectionPhase,
    pub retry_in_ms: Option<u32>,
    pub campus_id: Option<CampusId>,
    pub session_id: Option<SessionId>,
    pub attempt: u32,
    pub rejection: Option<VersionMismatch>,
    pub handshake_rejection: Option<HandshakeRejection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SessionSnapshot {
    pub auth: AuthPhase,
    pub account: Option<AccountProfile>,
    pub campuses: Vec<CampusSummary>,
    pub active_campus_id: Option<CampusId>,
    pub rooms: Vec<Room>,
    pub active_room_id: Option<RoomId>,
    pub busy: bool,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryState {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct TimelineEntry {
    pub client_nonce: ClientNonce,
    pub message_id: Option<MessageId>,
    pub author_id: AccountId,
    pub author_display_name: String,
    pub from_self: bool,
    pub body: String,
    pub delivery: DeliveryState,
    pub rejection: Option<MessageRejection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct TimelineSnapshot {
    pub room_id: Option<RoomId>,
    pub entries: Vec<TimelineEntry>,
    pub syncing: bool,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ClientSnapshot {
    #[ts(type = "number")]
    pub sequence: u64,
    pub network: ConnectionSnapshot,
    pub session: SessionSnapshot,
    pub timeline: TimelineSnapshot,
}
