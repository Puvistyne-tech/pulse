use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{MessageId, RoomId};

pub const REACTION_KINDS_MAX: usize = 20;
pub const REACTION_PALETTE: [&str; 6] = ["👍", "❤️", "🎉", "👀", "✅", "🙏"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ReactionAction {
    Add,
    Remove,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct React {
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub emoji: String,
    pub action: ReactionAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ReactionRejection {
    Forbidden,
    InvalidEmoji,
    UnknownMessage,
    TooManyKinds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ReactionRejected {
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub emoji: String,
    pub reason: ReactionRejection,
}
