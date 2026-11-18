use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{CampusId, SessionId};
use crate::version::ProtocolVersion;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Hello {
    pub protocol_version: ProtocolVersion,
    pub campus_id: CampusId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Welcome {
    pub protocol_version: ProtocolVersion,
    pub campus_id: CampusId,
    pub session_id: SessionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Ping {
    pub nonce: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Pong {
    pub nonce: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct VersionMismatch {
    pub supported: ProtocolVersion,
    pub received: ProtocolVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientFrame {
    Hello(Hello),
    Ping(Ping),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerFrame {
    Welcome(Welcome),
    Pong(Pong),
    VersionMismatch(VersionMismatch),
}

pub mod close_code {
    pub const PROTOCOL_ERROR: u16 = 1002;
    pub const INVALID_PAYLOAD: u16 = 1007;
    pub const POLICY_VIOLATION: u16 = 1008;
}
