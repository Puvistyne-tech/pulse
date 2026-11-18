pub mod api;
pub mod auth;
pub mod campus;
#[cfg(feature = "fixtures")]
pub mod fixtures;
pub mod frames;
pub mod ids;
pub mod messaging;
pub mod rbac;
pub mod snapshot;
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
pub use messaging::{
    MessageAccepted, MessageCreated, MessageRejected, MessageRejection, RoomCursor, RoomMessage,
    RoomSynced, RoomUnavailable, SYNC_MAX_ROOMS, SYNC_PAGE_LIMIT, SendMessage, SyncRooms,
};
pub use rbac::{Permission, Role, RoleParseError, role_permits};
pub use snapshot::{
    AuthPhase, ClientSnapshot, ConnectionPhase, ConnectionSnapshot, DeliveryState, SessionSnapshot,
    TimelineEntry, TimelineSnapshot,
};
pub use validation::{
    DISPLAY_NAME_MAX_CHARS, EMAIL_MAX_CHARS, MESSAGE_BODY_MAX_CHARS, PASSWORD_MAX_CHARS,
    PASSWORD_MIN_CHARS, SLUG_MAX_CHARS, SLUG_MIN_CHARS, Validate, normalize_display_name,
    normalize_email, normalize_message_body, validate_display_name, validate_password,
    validate_slug,
};
pub use version::ProtocolVersion;
