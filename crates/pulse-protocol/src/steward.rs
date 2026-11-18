use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::auth::AuthToken;
use crate::campus::Room;
use crate::ids::{AccountId, CampusId, ClientNonce, MessageId, RoomId, StreamId, ToolCallId};
use crate::messaging::RoomMessage;

pub const STEWARD_TOKEN_PREFIX: &str = "pst_";
pub const STEWARDS_PER_CAMPUS_MAX: usize = 50;
pub const STEWARD_ROOMS_MAX: usize = 64;
pub const WORK_SYNC_LIMIT: usize = 100;
pub const STREAMS_OPEN_MAX: usize = 4;
pub const STREAM_DELTA_MAX_CHARS: usize = 2_000;
pub const STREAM_IDLE_TIMEOUT_MS: u64 = 30_000;
pub const STREAM_STALE_AFTER_MS: u64 = 45_000;
pub const TOOL_HISTORY_LIMIT_MAX: u32 = 50;
pub const TOOL_ARGUMENTS_MAX_BYTES: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum StewardAction {
    Create,
    RotateToken,
    Revoke,
    AssignRooms,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ManageSteward {
    pub request_nonce: ClientNonce,
    pub action: StewardAction,
    pub steward_id: Option<AccountId>,
    pub display_name: Option<String>,
    #[serde(default)]
    pub room_ids: Vec<RoomId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StewardProfile {
    pub steward_id: AccountId,
    pub display_name: String,
    pub room_ids: Vec<RoomId>,
    pub revoked: bool,
    #[ts(type = "number")]
    pub credential_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StewardIssued {
    pub request_nonce: ClientNonce,
    pub steward: StewardProfile,
    pub token: AuthToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StewardChanged {
    pub steward: StewardProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum StewardRejection {
    Forbidden,
    UnknownSteward,
    InvalidDisplayName,
    InvalidRooms,
    Revoked,
    TooManyStewards,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StewardRejected {
    pub request_nonce: ClientNonce,
    pub action: StewardAction,
    pub reason: StewardRejection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SyncStewards {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StewardsSynced {
    pub stewards: Vec<StewardProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StewardWorkItem {
    pub message: RoomMessage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SyncWork {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct WorkSynced {
    pub items: Vec<StewardWorkItem>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct AckWork {
    pub through: MessageId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum StreamPhase {
    Begin,
    Delta,
    End,
    Abort,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StreamChunk {
    pub stream_id: StreamId,
    pub room_id: RoomId,
    pub thread_root_id: Option<MessageId>,
    pub phase: StreamPhase,
    pub sequence: u32,
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum StreamAbortReason {
    Cancelled,
    Disconnected,
    Revoked,
    Rejected,
    TimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct AgentStreamChunk {
    pub campus_id: CampusId,
    pub stream_id: StreamId,
    pub room_id: RoomId,
    pub thread_root_id: Option<MessageId>,
    pub steward_id: AccountId,
    pub steward_display_name: String,
    pub phase: StreamPhase,
    pub sequence: u32,
    pub text: String,
    pub message_id: Option<MessageId>,
    pub abort_reason: Option<StreamAbortReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum StreamRejection {
    Forbidden,
    UnknownRoom,
    UnknownThread,
    UnknownStream,
    DuplicateStream,
    OutOfOrder,
    TooManyStreams,
    TooLong,
    InvalidBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StreamRejected {
    pub stream_id: StreamId,
    pub reason: StreamRejection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamWriter {
    stream_id: StreamId,
    room_id: RoomId,
    thread_root_id: Option<MessageId>,
    next_sequence: u32,
}

impl StreamWriter {
    pub const fn new(
        stream_id: StreamId,
        room_id: RoomId,
        thread_root_id: Option<MessageId>,
    ) -> Self {
        Self {
            stream_id,
            room_id,
            thread_root_id,
            next_sequence: 0,
        }
    }

    pub fn begin(&mut self) -> StreamChunk {
        self.chunk(StreamPhase::Begin, "")
    }

    pub fn delta(&mut self, text: &str) -> StreamChunk {
        self.chunk(StreamPhase::Delta, text)
    }

    pub fn end(&mut self) -> StreamChunk {
        self.chunk(StreamPhase::End, "")
    }

    pub fn abort(&mut self) -> StreamChunk {
        self.chunk(StreamPhase::Abort, "")
    }

    pub const fn next_sequence(&self) -> u32 {
        self.next_sequence
    }

    fn chunk(&mut self, phase: StreamPhase, text: &str) -> StreamChunk {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        StreamChunk {
            stream_id: self.stream_id,
            room_id: self.room_id,
            thread_root_id: self.thread_root_id,
            phase,
            sequence,
            text: text.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum StewardTool {
    ListRooms,
    ReadRoomHistory,
    ReadThread,
    PostMessage,
}

impl StewardTool {
    pub const ALL: [StewardTool; 4] = [
        StewardTool::ListRooms,
        StewardTool::ReadRoomHistory,
        StewardTool::ReadThread,
        StewardTool::PostMessage,
    ];

    pub const fn name(self) -> &'static str {
        match self {
            StewardTool::ListRooms => "list_rooms",
            StewardTool::ReadRoomHistory => "read_room_history",
            StewardTool::ReadThread => "read_thread",
            StewardTool::PostMessage => "post_message",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            StewardTool::ListRooms => "List the rooms this steward is assigned to.",
            StewardTool::ReadRoomHistory => {
                "Read recent top-level messages of an assigned room, newest first."
            }
            StewardTool::ReadThread => "Read a thread root and its replies in an assigned room.",
            StewardTool::PostMessage => "Post a message to an assigned room or one of its threads.",
        }
    }

    pub const fn input_schema(self) -> &'static str {
        match self {
            StewardTool::ListRooms => r#"{"type":"object","properties":{},"required":[]}"#,
            StewardTool::ReadRoomHistory => {
                r#"{"type":"object","properties":{"room_id":{"type":"string"},"before":{"type":["string","null"]},"limit":{"type":["integer","null"],"minimum":1}},"required":["room_id"]}"#
            }
            StewardTool::ReadThread => {
                r#"{"type":"object","properties":{"room_id":{"type":"string"},"root_message_id":{"type":"string"}},"required":["room_id","root_message_id"]}"#
            }
            StewardTool::PostMessage => {
                r#"{"type":"object","properties":{"room_id":{"type":"string"},"client_nonce":{"type":"string"},"body":{"type":"string"},"thread_root_id":{"type":["string","null"]}},"required":["room_id","client_nonce","body"]}"#
            }
        }
    }

    pub fn from_name(name: &str) -> Option<StewardTool> {
        StewardTool::ALL
            .into_iter()
            .find(|tool| tool.name() == name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ToolDescriptor {
    pub name: String,
    pub description: String,
    pub input_schema: String,
}

pub fn tool_descriptors() -> Vec<ToolDescriptor> {
    StewardTool::ALL
        .into_iter()
        .map(|tool| ToolDescriptor {
            name: tool.name().to_owned(),
            description: tool.description().to_owned(),
            input_schema: tool.input_schema().to_owned(),
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ListTools {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ToolsListed {
    pub tools: Vec<ToolDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct CallTool {
    pub call_id: ToolCallId,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ToolContentKind {
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub kind: ToolContentKind,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ToolResult {
    pub call_id: ToolCallId,
    pub is_error: bool,
    pub content: Vec<ToolContent>,
    pub structured_content: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ToolErrorCode {
    Forbidden,
    UnknownTool,
    InvalidArguments,
    UnknownRoom,
    UnknownThread,
    Rejected,
}

impl ToolErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            ToolErrorCode::Forbidden => "forbidden",
            ToolErrorCode::UnknownTool => "unknown_tool",
            ToolErrorCode::InvalidArguments => "invalid_arguments",
            ToolErrorCode::UnknownRoom => "unknown_room",
            ToolErrorCode::UnknownThread => "unknown_thread",
            ToolErrorCode::Rejected => "rejected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ListRoomsArguments {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ReadRoomHistoryArguments {
    pub room_id: RoomId,
    pub before: Option<MessageId>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ReadThreadArguments {
    pub room_id: RoomId,
    pub root_message_id: MessageId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PostMessageArguments {
    pub room_id: RoomId,
    pub client_nonce: ClientNonce,
    pub body: String,
    pub thread_root_id: Option<MessageId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RoomsListed {
    pub rooms: Vec<Room>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct HistoryRead {
    pub messages: Vec<RoomMessage>,
    pub has_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MessagePosted {
    pub message: RoomMessage,
}
