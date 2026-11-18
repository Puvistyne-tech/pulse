pub mod api;
pub mod auth;
pub mod campus;
#[cfg(feature = "fixtures")]
pub mod fixtures;
pub mod frames;
pub mod ids;
pub mod mention;
pub mod messaging;
pub mod rbac;
pub mod reaction;
pub mod snapshot;
pub mod thread;
pub mod validation;
pub mod version;

pub use api::{ApiError, ApiErrorCode, InputField, api_path};
pub use auth::{AccountProfile, AuthSession, AuthToken, SignInRequest, SignUpRequest};
pub use campus::{
    CampusCreated, CampusList, CampusSummary, CreateCampusRequest, CreateRoomRequest,
    DEFAULT_ROOM_SLUGS, Room, RoomList,
};
pub use frames::{
    ClientFrame, HandshakeRejected, HandshakeRejection, Hello, Ping, Pong, ServerFrame,
    VersionMismatch, Welcome, close_code,
};
pub use ids::{AccountId, CampusId, ClientNonce, IdParseError, MessageId, RoomId, SessionId};
pub use mention::{
    BodySegment, MESSAGE_MENTIONS_MAX, SegmentKind, body_segments, extract_mentions, mention_token,
};
pub use messaging::{
    MentionRef, MessageAccepted, MessageCreated, MessageRejected, MessageRejection, MessageUpdated,
    ReactionTally, RoomCursor, RoomMessage, RoomSynced, RoomUnavailable, SYNC_MAX_ROOMS,
    SYNC_PAGE_LIMIT, SendMessage, SyncRooms,
};
pub use rbac::{Permission, Role, RoleParseError, role_permits};
pub use reaction::{
    REACTION_KINDS_MAX, REACTION_PALETTE, React, ReactionAction, ReactionRejected,
    ReactionRejection,
};
pub use snapshot::{
    AuthPhase, ClientSnapshot, ConnectionPhase, ConnectionSnapshot, DeliveryState, InboxItem,
    InboxItemKind, InboxSnapshot, InboxStep, ReactionSummary, ReplyPreview, SessionSnapshot,
    ThreadPreview, ThreadSnapshot, TimelineEntry, TimelineSnapshot,
};
pub use thread::{
    INBOX_SYNC_LIMIT, InboxSynced, MarkThreadRead, SetThreadSubscription, SetTopicTitle,
    SubscriptionOrigin, SubscriptionState, SyncInbox, SyncThread, ThreadChanged, ThreadRejected,
    ThreadRejection, ThreadSummary, ThreadSynced,
};
pub use validation::{
    DISPLAY_NAME_MAX_CHARS, EMAIL_MAX_CHARS, MESSAGE_BODY_MAX_CHARS, PASSWORD_MAX_CHARS,
    PASSWORD_MIN_CHARS, REACTION_EMOJI_MAX_CHARS, SLUG_MAX_CHARS, SLUG_MIN_CHARS,
    TOPIC_TITLE_MAX_CHARS, Validate, normalize_display_name, normalize_email,
    normalize_message_body, normalize_reaction_emoji, normalize_topic_title, validate_display_name,
    validate_password, validate_slug,
};
pub use version::ProtocolVersion;
