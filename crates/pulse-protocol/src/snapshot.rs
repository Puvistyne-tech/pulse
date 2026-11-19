use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::api::ApiError;
use crate::auth::AccountProfile;
use crate::auth::AuthToken;
use crate::campus::{CampusInvite, CampusSummary, Room};
use crate::dev_seed::DevAccount;
use crate::frames::{HandshakeRejection, VersionMismatch};
use crate::ids::{
    AccountId, AttachmentId, CampusId, ClientNonce, DraftId, LoungeId, MessageId, RoomId,
    SessionId, ShelfItemId,
};
use crate::lounge::{LoungeScope, LoungeTicket};
use crate::media::{AttachmentKind, VaultTab};
use crate::mention::BodySegment;
use crate::messaging::MessageRejection;
use crate::rbac::Role;
use crate::shelf::{ReminderPreset, ShelfBucket};
use crate::thread::SubscriptionState;
use crate::version::ProtocolVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionPhase {
    Disconnected,
    Connecting,
    Handshaking,
    Connected,
    Backoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum AuthPhase {
    SignedOut,
    Authenticating,
    SignedIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ConnectionSnapshot {
    pub protocol_version: ProtocolVersion,
    pub connection: ConnectionPhase,
    pub retry_in_ms: Option<u32>,
    pub campus_id: Option<CampusId>,
    pub session_id: Option<SessionId>,
    pub attempt: u32,
    pub rejection: Option<VersionMismatch>,
    pub handshake_rejection: Option<HandshakeRejection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MemberEntry {
    pub account_id: AccountId,
    pub display_name: String,
    pub email: String,
    pub role: Role,
    pub manageable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SessionSnapshot {
    pub auth: AuthPhase,
    pub account: Option<AccountProfile>,
    pub campuses: Vec<CampusSummary>,
    pub active_campus_id: Option<CampusId>,
    pub rooms: Vec<Room>,
    pub active_room_id: Option<RoomId>,
    pub invite: Option<CampusInvite>,
    pub can_invite: bool,
    pub members: Vec<MemberEntry>,
    pub assignable_roles: Vec<Role>,
    pub quick_accounts: Vec<DevAccount>,
    pub busy: bool,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryState {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum UploadState {
    Uploading,
    Uploaded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct AttachmentView {
    pub attachment_id: AttachmentId,
    pub file_name: String,
    pub content_type: String,
    pub kind: AttachmentKind,
    #[ts(type = "number")]
    pub byte_size: u64,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct DraftAttachmentView {
    pub draft_id: DraftId,
    pub room_id: RoomId,
    pub file_name: String,
    pub kind: AttachmentKind,
    #[ts(type = "number")]
    pub byte_size: u64,
    pub upload: UploadState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ReactionSummary {
    pub emoji: String,
    pub count: u32,
    pub reacted_by_self: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ReplyPreview {
    pub message_id: MessageId,
    pub author_id: AccountId,
    pub author_display_name: String,
    pub preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ThreadPreview {
    pub topic_title: Option<String>,
    pub reply_count: u32,
    pub latest_reply: Option<ReplyPreview>,
    pub show_latest_reply: bool,
    pub subscription: Option<SubscriptionState>,
    pub unread_replies: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum StreamState {
    Streaming,
    Finalizing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct TimelineEntry {
    pub client_nonce: ClientNonce,
    pub message_id: Option<MessageId>,
    pub author_id: AccountId,
    pub author_display_name: String,
    pub author_mention_token: String,
    pub from_self: bool,
    pub body: String,
    pub segments: Vec<BodySegment>,
    pub thread_root_id: Option<MessageId>,
    pub reactions: Vec<ReactionSummary>,
    pub thread: Option<ThreadPreview>,
    pub delivery: DeliveryState,
    pub rejection: Option<MessageRejection>,
    pub attachments: Vec<AttachmentView>,
    pub pending_drafts: Vec<DraftAttachmentView>,
    pub pinned: bool,
    pub shelved: bool,
    pub author_is_steward: bool,
    pub stream: Option<StreamState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct TimelineSnapshot {
    pub room_id: Option<RoomId>,
    pub entries: Vec<TimelineEntry>,
    pub syncing: bool,
    pub error: Option<ApiError>,
    pub reaction_palette: Vec<String>,
    pub drafts: Vec<DraftAttachmentView>,
    pub focus_message_id: Option<MessageId>,
    pub can_pin: bool,
    pub can_upload: bool,
    pub mentionable_stewards: Vec<StewardEntry>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ThreadSnapshot {
    pub room_id: Option<RoomId>,
    pub root_message_id: Option<MessageId>,
    pub root: Option<TimelineEntry>,
    pub replies: Vec<TimelineEntry>,
    pub topic_title: Option<String>,
    pub subscription: Option<SubscriptionState>,
    pub syncing: bool,
    pub mentionable_stewards: Vec<StewardEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum InboxItemKind {
    Thread,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum InboxStep {
    Next,
    Previous,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct InboxItem {
    pub kind: InboxItemKind,
    pub room_id: RoomId,
    pub root_message_id: MessageId,
    pub topic_title: Option<String>,
    pub root_author_display_name: String,
    pub root_preview: String,
    pub latest_reply: Option<ReplyPreview>,
    pub reply_count: u32,
    pub unread_replies: u32,
    pub unread_mentions: u32,
    pub subscription: SubscriptionState,
    pub activity_message_id: MessageId,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct InboxSnapshot {
    pub open: bool,
    pub items: Vec<InboxItem>,
    pub unread_threads: u32,
    pub selected_root_message_id: Option<MessageId>,
    pub replying: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct HighlightSpan {
    pub text: String,
    pub matched: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SearchHit {
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub thread_root_id: Option<MessageId>,
    pub author_display_name: String,
    pub topic_title: Option<String>,
    pub snippet: Vec<HighlightSpan>,
    pub score: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SearchSnapshot {
    pub open: bool,
    pub query: String,
    pub hits: Vec<SearchHit>,
    pub selected_index: Option<u32>,
    pub searching: bool,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct VaultEntry {
    pub tab: VaultTab,
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub author_display_name: String,
    pub preview: String,
    pub attachment: Option<AttachmentView>,
    pub link: Option<String>,
    #[ts(type = "number | null")]
    pub pinned_at_ms: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct VaultSnapshot {
    pub open: bool,
    pub room_id: Option<RoomId>,
    pub tab: VaultTab,
    pub entries: Vec<VaultEntry>,
    pub loading: bool,
    pub has_more: bool,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ShelfEntry {
    pub shelf_item_id: Option<ShelfItemId>,
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub author_display_name: String,
    pub preview: String,
    #[ts(type = "number | null")]
    pub remind_at_ms: Option<u64>,
    #[ts(type = "number | null")]
    pub done_at_ms: Option<u64>,
    pub bucket: ShelfBucket,
    pub pending: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ShelfSnapshot {
    pub open: bool,
    pub due: Vec<ShelfEntry>,
    pub upcoming: Vec<ShelfEntry>,
    pub done: Vec<ShelfEntry>,
    pub due_count: u32,
    pub latest_reminder: Option<ShelfEntry>,
    pub presets: Vec<ReminderPreset>,
    #[ts(type = "number | null")]
    pub picker_anchor_ms: Option<u64>,
    #[ts(type = "number | null")]
    pub picker_latest_ms: Option<u64>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum LoungeMediaState {
    #[default]
    Idle,
    Connecting,
    Connected,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeParticipantView {
    pub account_id: AccountId,
    pub display_name: String,
    pub from_self: bool,
    pub can_speak: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeEntry {
    pub lounge_id: LoungeId,
    pub scope: LoungeScope,
    pub room_id: Option<RoomId>,
    pub room_slug: Option<String>,
    pub participants: Vec<LoungeParticipantView>,
    pub joined: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct LoungeSnapshot {
    pub lounges: Vec<LoungeEntry>,
    pub joined_lounge_id: Option<LoungeId>,
    pub joining: bool,
    pub ticket: Option<LoungeTicket>,
    pub media: LoungeMediaState,
    pub can_join: bool,
    pub can_speak: bool,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StewardEntry {
    pub steward_id: AccountId,
    pub display_name: String,
    pub mention_token: String,
    pub room_ids: Vec<RoomId>,
    pub revoked: bool,
    pub manageable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct IssuedStewardToken {
    pub steward_id: AccountId,
    pub display_name: String,
    pub token: AuthToken,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StewardSnapshot {
    pub open: bool,
    pub stewards: Vec<StewardEntry>,
    pub can_manage: bool,
    pub issued: Option<IssuedStewardToken>,
    pub pending: bool,
    pub error: Option<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ClientSnapshot {
    #[ts(type = "number")]
    pub sequence: u64,
    pub network: ConnectionSnapshot,
    pub session: SessionSnapshot,
    pub timeline: TimelineSnapshot,
    pub thread: ThreadSnapshot,
    pub inbox: InboxSnapshot,
    pub search: SearchSnapshot,
    pub vault: VaultSnapshot,
    pub shelf: ShelfSnapshot,
    pub lounges: LoungeSnapshot,
    pub stewards: StewardSnapshot,
}
