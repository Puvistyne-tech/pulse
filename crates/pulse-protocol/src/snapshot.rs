use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::frames::VersionMismatch;
use crate::ids::{CampusId, SessionId};
use crate::version::ProtocolVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum ConnectionPhase {
    Disconnected,
    Connecting,
    Handshaking,
    Connected,
    Backoff { retry_in_ms: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ClientSnapshot {
    pub protocol_version: ProtocolVersion,
    pub connection: ConnectionPhase,
    pub campus_id: Option<CampusId>,
    pub session_id: Option<SessionId>,
    pub attempt: u32,
    pub rejection: Option<VersionMismatch>,
}
