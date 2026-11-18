use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::auth::AuthToken;
use crate::ids::{AccountId, CampusId, SessionId};
use crate::messaging::{
    MessageAccepted, MessageCreated, MessageRejected, MessageUpdated, RoomSynced, RoomUnavailable,
    SendMessage, SyncRooms,
};
use crate::rbac::Role;
use crate::reaction::{React, ReactionRejected};
use crate::thread::{
    InboxSynced, MarkThreadRead, SetThreadSubscription, SetTopicTitle, SyncInbox, SyncThread,
    ThreadChanged, ThreadRejected, ThreadSynced,
};
use crate::version::ProtocolVersion;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Hello {
    pub protocol_version: ProtocolVersion,
    pub campus_id: CampusId,
    pub auth_token: AuthToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Welcome {
    pub protocol_version: ProtocolVersion,
    pub campus_id: CampusId,
    pub session_id: SessionId,
    pub account_id: AccountId,
    pub role: Role,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum HandshakeRejection {
    Unauthenticated,
    CampusUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct HandshakeRejected {
    pub reason: HandshakeRejection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientFrame {
    Hello(Hello),
    Ping(Ping),
    SendMessage(SendMessage),
    SyncRooms(SyncRooms),
    React(React),
    SetTopicTitle(SetTopicTitle),
    SetThreadSubscription(SetThreadSubscription),
    MarkThreadRead(MarkThreadRead),
    SyncThread(SyncThread),
    SyncInbox(SyncInbox),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ServerFrame {
    Welcome(Welcome),
    Pong(Pong),
    VersionMismatch(VersionMismatch),
    HandshakeRejected(HandshakeRejected),
    MessageAccepted(MessageAccepted),
    MessageCreated(MessageCreated),
    MessageRejected(MessageRejected),
    RoomSynced(RoomSynced),
    RoomUnavailable(RoomUnavailable),
    MessageUpdated(MessageUpdated),
    ReactionRejected(ReactionRejected),
    ThreadRejected(ThreadRejected),
    ThreadSynced(ThreadSynced),
    InboxSynced(InboxSynced),
    ThreadChanged(ThreadChanged),
}

pub mod close_code {
    pub const PROTOCOL_ERROR: u16 = 1002;
    pub const INVALID_PAYLOAD: u16 = 1007;
    pub const POLICY_VIOLATION: u16 = 1008;
    pub const INTERNAL_ERROR: u16 = 1011;
    pub const TRY_AGAIN_LATER: u16 = 1013;
}
