use core::fmt;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{AccountId, CampusId, LoungeId, RoomId};

pub const LOUNGE_HEARTBEAT_MS: u64 = 10_000;
pub const LOUNGE_PRESENCE_TTL_MS: u64 = 30_000;
pub const LOUNGE_TICKET_TTL_MS: u64 = 600_000;
pub const LOUNGE_PARTICIPANTS_MAX: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum LoungeScope {
    Campus,
    Room,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeParticipant {
    pub account_id: AccountId,
    pub display_name: String,
    pub can_speak: bool,
    #[ts(type = "number")]
    pub present_until_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Lounge {
    pub lounge_id: LoungeId,
    pub campus_id: CampusId,
    pub scope: LoungeScope,
    pub room_id: Option<RoomId>,
    pub participants: Vec<LoungeParticipant>,
    #[ts(type = "number")]
    pub revision: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SyncLounges {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungesSynced {
    pub lounges: Vec<Lounge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct JoinLounge {
    pub lounge_id: LoungeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LeaveLounge {
    pub lounge_id: LoungeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeHeartbeat {
    pub lounge_id: LoungeId,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeTicket {
    pub lounge_id: LoungeId,
    pub livekit_url: String,
    pub room_name: String,
    pub token: String,
    #[ts(type = "number")]
    pub expires_at_ms: u64,
}

impl fmt::Debug for LoungeTicket {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LoungeTicket")
            .field("lounge_id", &self.lounge_id)
            .field("livekit_url", &self.livekit_url)
            .field("room_name", &self.room_name)
            .field("token", &"redacted")
            .field("expires_at_ms", &self.expires_at_ms)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeJoined {
    pub lounge: Lounge,
    pub ticket: LoungeTicket,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeChanged {
    pub lounge: Lounge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum LoungeRejection {
    Forbidden,
    UnknownLounge,
    Full,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeRejected {
    pub lounge_id: LoungeId,
    pub reason: LoungeRejection,
}
