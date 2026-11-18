use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{AccountId, CampusId, ClientNonce, MessageId, RoomId};

pub const SYNC_PAGE_LIMIT: usize = 100;
pub const SYNC_MAX_ROOMS: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MentionRef {
    pub account_id: AccountId,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ReactionTally {
    pub emoji: String,
    pub account_ids: Vec<AccountId>,
}

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
    pub thread_root_id: Option<MessageId>,
    pub topic_title: Option<String>,
    pub reply_count: u32,
    pub latest_reply_id: Option<MessageId>,
    pub mentions: Vec<MentionRef>,
    pub reactions: Vec<ReactionTally>,
    #[ts(type = "number")]
    pub activity_seq: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SendMessage {
    pub room_id: RoomId,
    pub client_nonce: ClientNonce,
    pub body: String,
    pub thread_root_id: Option<MessageId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RoomCursor {
    pub room_id: RoomId,
    pub after: Option<MessageId>,
    #[ts(type = "number | null")]
    pub activity_after: Option<u64>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MessageUpdated {
    pub message: RoomMessage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum MessageRejection {
    Forbidden,
    InvalidBody,
    UnknownRoom,
    UnknownThread,
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
    pub updates: Vec<RoomMessage>,
    #[ts(type = "number")]
    pub activity_seq: u64,
    pub has_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RoomUnavailable {
    pub room_id: RoomId,
}
