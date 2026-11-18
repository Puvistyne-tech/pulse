use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{AccountId, CampusId, ClientNonce, MessageId, RoomId};

pub const SYNC_PAGE_LIMIT: usize = 100;
pub const SYNC_MAX_ROOMS: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RoomMessage {
    pub message_id: MessageId,
    pub previous_message_id: Option<MessageId>,
    pub campus_id: CampusId,
    pub room_id: RoomId,
    pub author_id: AccountId,
    pub author_display_name: String,
    pub client_nonce: ClientNonce,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SendMessage {
    pub room_id: RoomId,
    pub client_nonce: ClientNonce,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RoomCursor {
    pub room_id: RoomId,
    pub after: Option<MessageId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SyncRooms {
    pub cursors: Vec<RoomCursor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MessageAccepted {
    pub message: RoomMessage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MessageCreated {
    pub message: RoomMessage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum MessageRejection {
    Forbidden,
    InvalidBody,
    UnknownRoom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MessageRejected {
    pub client_nonce: ClientNonce,
    pub reason: MessageRejection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RoomSynced {
    pub room_id: RoomId,
    pub messages: Vec<RoomMessage>,
    pub has_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RoomUnavailable {
    pub room_id: RoomId,
}
