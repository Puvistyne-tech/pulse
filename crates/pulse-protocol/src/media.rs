use core::str::FromStr;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{AccountId, AttachmentId, MessageId, RoomId};
use crate::messaging::RoomMessage;

pub const ATTACHMENT_MAX_BYTES: u64 = 26_214_400;
pub const MESSAGE_ATTACHMENTS_MAX: usize = 10;
pub const FILE_NAME_MAX_CHARS: usize = 255;
pub const CONTENT_TYPE_MAX_CHARS: usize = 127;
pub const DOWNLOAD_TICKETS_MAX: usize = 50;
pub const ROOM_PINS_MAX: usize = 50;
pub const VAULT_PAGE_LIMIT: usize = 50;

const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";

const EXTENSION_CONTENT_TYPES: [(&str, &str); 24] = [
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("gif", "image/gif"),
    ("webp", "image/webp"),
    ("svg", "image/svg+xml"),
    ("mp4", "video/mp4"),
    ("mov", "video/quicktime"),
    ("webm", "video/webm"),
    ("mp3", "audio/mpeg"),
    ("wav", "audio/wav"),
    ("ogg", "audio/ogg"),
    ("m4a", "audio/mp4"),
    ("pdf", "application/pdf"),
    ("zip", "application/zip"),
    ("json", "application/json"),
    ("txt", "text/plain"),
    ("md", "text/markdown"),
    ("csv", "text/csv"),
    ("html", "text/html"),
    ("doc", "application/msword"),
    (
        "docx",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    ),
    ("xls", "application/vnd.ms-excel"),
    (
        "xlsx",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentKind {
    Image,
    Video,
    Audio,
    File,
}

#[derive(Debug, thiserror::Error)]
#[error("unknown attachment kind")]
pub struct AttachmentKindParseError;

impl AttachmentKind {
    pub const ALL: [AttachmentKind; 4] = [
        AttachmentKind::Image,
        AttachmentKind::Video,
        AttachmentKind::Audio,
        AttachmentKind::File,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            AttachmentKind::Image => "image",
            AttachmentKind::Video => "video",
            AttachmentKind::Audio => "audio",
            AttachmentKind::File => "file",
        }
    }

    pub fn from_content_type(content_type: &str) -> AttachmentKind {
        let major = content_type.split('/').next().unwrap_or_default();
        match major.trim().to_lowercase().as_str() {
            "image" => AttachmentKind::Image,
            "video" => AttachmentKind::Video,
            "audio" => AttachmentKind::Audio,
            _ => AttachmentKind::File,
        }
    }
}

impl FromStr for AttachmentKind {
    type Err = AttachmentKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        AttachmentKind::ALL
            .into_iter()
            .find(|kind| kind.as_str() == value)
            .ok_or(AttachmentKindParseError)
    }
}

pub fn guess_content_type(file_name: &str) -> &'static str {
    let Some((_, extension)) = file_name.rsplit_once('.') else {
        return DEFAULT_CONTENT_TYPE;
    };
    let extension = extension.to_lowercase();
    EXTENSION_CONTENT_TYPES
        .iter()
        .find(|(known, _)| *known == extension)
        .map_or(DEFAULT_CONTENT_TYPE, |(_, content_type)| content_type)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Attachment {
    pub attachment_id: AttachmentId,
    pub file_name: String,
    pub content_type: String,
    pub kind: AttachmentKind,
    #[ts(type = "number")]
    pub byte_size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PinMark {
    pub pinned_by: AccountId,
    #[ts(type = "number")]
    pub pinned_at_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct CreateUploadRequest {
    pub file_name: String,
    pub content_type: String,
    #[ts(type = "number")]
    pub byte_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct UploadTicket {
    pub attachment_id: AttachmentId,
    pub put_url: String,
    pub content_type: String,
    #[ts(type = "number")]
    pub expires_at_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct DownloadTicketsRequest {
    pub attachment_ids: Vec<AttachmentId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct DownloadTicket {
    pub attachment_id: AttachmentId,
    pub url: String,
    #[ts(type = "number")]
    pub expires_at_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct DownloadTickets {
    pub tickets: Vec<DownloadTicket>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SetPin {
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub pinned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum PinRejection {
    Forbidden,
    UnknownMessage,
    LimitReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct PinRejected {
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub reason: PinRejection,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum VaultTab {
    #[default]
    Media,
    Files,
    Links,
    Pinned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SyncVault {
    pub room_id: RoomId,
    pub tab: VaultTab,
    pub before: Option<MessageId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct VaultItem {
    pub message: RoomMessage,
    pub attachment: Option<Attachment>,
    pub link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct VaultSynced {
    pub room_id: RoomId,
    pub tab: VaultTab,
    pub items: Vec<VaultItem>,
    pub next_before: Option<MessageId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum VaultRejection {
    Forbidden,
    UnknownRoom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct VaultRejected {
    pub room_id: RoomId,
    pub tab: VaultTab,
    pub reason: VaultRejection,
}
