use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{MessageId, RoomId};
use crate::messaging::RoomMessage;

pub const INBOX_SYNC_LIMIT: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionState {
    Following,
    Muted,
    Archived,
}

#[derive(Debug, thiserror::Error)]
#[error("unknown subscription state")]
pub struct SubscriptionStateParseError;

impl SubscriptionState {
    pub const ALL: [SubscriptionState; 3] = [
        SubscriptionState::Following,
        SubscriptionState::Muted,
        SubscriptionState::Archived,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            SubscriptionState::Following => "following",
            SubscriptionState::Muted => "muted",
            SubscriptionState::Archived => "archived",
        }
    }
}

impl core::str::FromStr for SubscriptionState {
    type Err = SubscriptionStateParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        SubscriptionState::ALL
            .into_iter()
            .find(|state| state.as_str() == value)
            .ok_or(SubscriptionStateParseError)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionOrigin {
    Explicit,
    Viewed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SetThreadSubscription {
    pub room_id: RoomId,
    pub root_message_id: MessageId,
    pub state: SubscriptionState,
    pub origin: SubscriptionOrigin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MarkThreadRead {
    pub room_id: RoomId,
    pub root_message_id: MessageId,
    pub read_through: MessageId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SetTopicTitle {
    pub room_id: RoomId,
    pub root_message_id: MessageId,
    pub topic_title: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SyncThread {
    pub room_id: RoomId,
    pub root_message_id: MessageId,
    pub after: Option<MessageId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ThreadSynced {
    pub room_id: RoomId,
    pub root_message_id: MessageId,
    pub messages: Vec<RoomMessage>,
    pub has_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SyncInbox {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ThreadSummary {
    pub root: RoomMessage,
    pub latest_reply: Option<RoomMessage>,
    pub unread_replies: u32,
    pub unread_mentions: u32,
    pub subscription: SubscriptionState,
    pub read_through: Option<MessageId>,
    #[ts(type = "number")]
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct InboxSynced {
    pub threads: Vec<ThreadSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ThreadChanged {
    pub summary: ThreadSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ThreadRejection {
    Forbidden,
    UnknownThread,
    InvalidTopicTitle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ThreadRejected {
    pub room_id: RoomId,
    pub root_message_id: MessageId,
    pub reason: ThreadRejection,
}
