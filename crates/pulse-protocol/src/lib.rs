pub mod api;
pub mod auth;
pub mod campus;
pub mod dev_seed;
#[cfg(feature = "fixtures")]
pub mod fixtures;
pub mod frames;
pub mod ids;
pub mod link;
pub mod lounge;
pub mod media;
pub mod member;
pub mod mention;
pub mod messaging;
pub mod note;
pub mod rbac;
pub mod reaction;
pub mod search;
pub mod shelf;
pub mod snapshot;
pub mod steward;
pub mod thread;
pub mod validation;
pub mod version;

pub use api::{ApiError, ApiErrorCode, InputField, api_path};
pub use auth::{AccountProfile, AuthSession, AuthToken, SignInRequest, SignUpRequest};
pub use campus::{
    CampusCreated, CampusInvite, CampusList, CampusSummary, CreateCampusRequest, CreateRoomRequest,
    DEFAULT_ROOM_SLUGS, INVITE_CODE_SEPARATOR, INVITE_TTL_DAYS, InviteCode, JoinCampusRequest,
    Room, RoomList,
};
pub use dev_seed::{
    DEV_SEED_CAMPUS_NAME, DEV_SEED_CAMPUS_SLUG, DEV_SEED_PASSWORD, DEV_SEED_PEOPLE, DevAccount,
    dev_accounts,
};
pub use frames::{
    ClientFrame, HandshakeRejected, HandshakeRejection, Hello, Ping, Pong, ServerFrame,
    VersionMismatch, Welcome, close_code,
};
pub use ids::{
    AccountId, AttachmentId, CampusId, ClientNonce, DraftId, IdParseError, LoungeId, MessageId,
    RoomId, SessionId, ShelfItemId, StreamId, ToolCallId,
};
pub use link::{LINK_MAX_CHARS, MESSAGE_LINKS_MAX, extract_links};
pub use lounge::{
    JoinLounge, LOUNGE_HEARTBEAT_MS, LOUNGE_PARTICIPANTS_MAX, LOUNGE_PRESENCE_TTL_MS,
    LOUNGE_TICKET_TTL_MS, LeaveLounge, Lounge, LoungeChanged, LoungeHeartbeat, LoungeJoined,
    LoungeParticipant, LoungeRejected, LoungeRejection, LoungeScope, LoungeTicket, LoungesSynced,
    SyncLounges,
};
pub use media::{
    ATTACHMENT_MAX_BYTES, Attachment, AttachmentKind, AttachmentKindParseError,
    CONTENT_TYPE_MAX_CHARS, CreateUploadRequest, DOWNLOAD_TICKETS_MAX, DownloadTicket,
    DownloadTickets, DownloadTicketsRequest, FILE_NAME_MAX_CHARS, MESSAGE_ATTACHMENTS_MAX, PinMark,
    PinRejected, PinRejection, ROOM_PINS_MAX, SetPin, SyncVault, UploadTicket, VAULT_PAGE_LIMIT,
    VaultItem, VaultRejected, VaultRejection, VaultSynced, VaultTab, guess_content_type,
};
pub use member::{
    ASSIGNABLE_ROLES, AddMemberRequest, CampusMember, MemberList, MembershipRevoked,
    UpdateMemberRequest, assignable_roles, member_manageable, role_assignable,
};
pub use mention::{
    BodySegment, MESSAGE_MENTIONS_MAX, SegmentKind, body_segments, extract_mentions, mention_token,
};
pub use messaging::{
    MentionRef, MessageAccepted, MessageCreated, MessageRejected, MessageRejection, MessageUpdated,
    ReactionTally, RoomCursor, RoomMessage, RoomSynced, RoomUnavailable, SYNC_MAX_ROOMS,
    SYNC_PAGE_LIMIT, SendMessage, SyncRooms,
};
pub use note::{
    NOTE_MEMBERS_MAX, NOTE_SLUG_PREFIX, Note, NoteList, NoteMember, NoteStarted, StartNoteRequest,
    note_member_key, note_title,
};
pub use rbac::{Permission, Role, RoleParseError, role_permits};
pub use reaction::{
    REACTION_KINDS_MAX, REACTION_PALETTE, React, ReactionAction, ReactionRejected,
    ReactionRejection,
};
pub use search::{
    SEARCH_CANDIDATE_LIMIT, SEARCH_HITS_LIMIT, SEARCH_QUERY_MAX_CHARS, SEARCH_SNIPPET_CHARS,
    SEARCH_TERM_MAX_CHARS, SEARCH_TERMS_MAX, search_terms,
};
pub use shelf::{
    REMIND_AT_HORIZON_MS, REMIND_AT_PAST_TOLERANCE_MS, REMINDER_PICKER_STEP_MS, ReminderPreset,
    SHELF_ITEMS_MAX, SHELF_SYNC_LIMIT, ShelfAction, ShelfBucket, ShelfChanged, ShelfItem,
    ShelfRejected, ShelfRejection, ShelfSynced, SyncShelf, UpdateShelf, picker_anchor_ms,
    picker_latest_ms,
};
pub use snapshot::{
    AttachmentView, AuthPhase, ClientSnapshot, ConnectionPhase, ConnectionSnapshot, DeliveryState,
    DraftAttachmentView, HighlightSpan, InboxItem, InboxItemKind, InboxSnapshot, InboxStep,
    IssuedStewardToken, LoungeEntry, LoungeMediaState, LoungeParticipantView, LoungeSnapshot,
    MemberEntry, NoteEntry, ReactionSummary, ReplyPreview, SearchHit, SearchSnapshot,
    SessionSnapshot, ShelfEntry, ShelfSnapshot, StewardEntry, StewardSnapshot, StreamState,
    ThreadPreview, ThreadSnapshot, TimelineEntry, TimelineSnapshot, UploadState, VaultEntry,
    VaultSnapshot,
};
pub use steward::{
    AckWork, AgentStreamChunk, CallTool, HistoryRead, ListRoomsArguments, ListTools, ManageSteward,
    MessagePosted, PostMessageArguments, ReadRoomHistoryArguments, ReadThreadArguments,
    RoomsListed, STEWARD_ROOMS_MAX, STEWARD_TOKEN_PREFIX, STEWARDS_PER_CAMPUS_MAX,
    STREAM_DELTA_MAX_CHARS, STREAM_IDLE_TIMEOUT_MS, STREAM_STALE_AFTER_MS, STREAMS_OPEN_MAX,
    StewardAction, StewardChanged, StewardIssued, StewardProfile, StewardRejected,
    StewardRejection, StewardTool, StewardWorkItem, StewardsSynced, StreamAbortReason, StreamChunk,
    StreamPhase, StreamRejected, StreamRejection, StreamWriter, SyncStewards, SyncWork,
    TOOL_ARGUMENTS_MAX_BYTES, TOOL_HISTORY_LIMIT_MAX, ToolContent, ToolContentKind, ToolDescriptor,
    ToolErrorCode, ToolResult, ToolsListed, WORK_SYNC_LIMIT, WorkSynced, tool_descriptors,
};
pub use thread::{
    INBOX_SYNC_LIMIT, InboxSynced, MarkThreadRead, SetThreadSubscription, SetTopicTitle,
    SubscriptionOrigin, SubscriptionState, SubscriptionStateParseError, SyncInbox, SyncThread,
    ThreadChanged, ThreadRejected, ThreadRejection, ThreadSummary, ThreadSynced,
};
pub use validation::{
    DISPLAY_NAME_MAX_CHARS, EMAIL_MAX_CHARS, INVITE_SECRET_CHARS, MESSAGE_BODY_MAX_CHARS,
    PASSWORD_MAX_CHARS, PASSWORD_MIN_CHARS, REACTION_EMOJI_MAX_CHARS, SLUG_MAX_CHARS,
    SLUG_MIN_CHARS, TOPIC_TITLE_MAX_CHARS, Validate, normalize_attachment_ids,
    normalize_content_type, normalize_display_name, normalize_email, normalize_file_name,
    normalize_invite_code, normalize_message_body, normalize_message_content,
    normalize_note_members, normalize_reaction_emoji, normalize_room_ids, normalize_search_query,
    normalize_topic_title, validate_byte_size, validate_display_name, validate_password,
    validate_remind_at, validate_slug,
};
pub use version::ProtocolVersion;
