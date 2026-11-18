use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    InvalidInput,
    Unauthenticated,
    Forbidden,
    NotFound,
    Conflict,
    Unavailable,
    Internal,
}

impl ApiErrorCode {
    pub const fn http_status(self) -> u16 {
        match self {
            ApiErrorCode::InvalidInput => 400,
            ApiErrorCode::Unauthenticated => 401,
            ApiErrorCode::Forbidden => 403,
            ApiErrorCode::NotFound => 404,
            ApiErrorCode::Conflict => 409,
            ApiErrorCode::Unavailable => 503,
            ApiErrorCode::Internal => 500,
        }
    }

    pub const fn from_http_status(status: u16) -> ApiErrorCode {
        match status {
            400 => ApiErrorCode::InvalidInput,
            401 => ApiErrorCode::Unauthenticated,
            403 => ApiErrorCode::Forbidden,
            404 => ApiErrorCode::NotFound,
            409 => ApiErrorCode::Conflict,
            503 => ApiErrorCode::Unavailable,
            _ => ApiErrorCode::Internal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum InputField {
    Email,
    DisplayName,
    Password,
    Slug,
    MessageBody,
    TopicTitle,
    ReactionEmoji,
    FileName,
    ContentType,
    ByteSize,
    Attachments,
    RemindAt,
    Rooms,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub field: Option<InputField>,
}

impl ApiError {
    pub const fn new(code: ApiErrorCode) -> Self {
        Self { code, field: None }
    }

    pub const fn invalid(field: InputField) -> Self {
        Self {
            code: ApiErrorCode::InvalidInput,
            field: Some(field),
        }
    }

    pub const fn conflict(field: InputField) -> Self {
        Self {
            code: ApiErrorCode::Conflict,
            field: Some(field),
        }
    }
}

pub mod api_path {
    use crate::ids::{CampusId, RoomId};

    pub const ACCOUNTS: &str = "/v1/accounts";
    pub const AUTH_TOKENS: &str = "/v1/auth-tokens";
    pub const CURRENT_AUTH_TOKEN: &str = "/v1/auth-tokens/current";
    pub const CAMPUSES: &str = "/v1/campuses";
    pub const CAMPUS_ROOMS_ROUTE: &str = "/v1/campuses/{campus_id}/rooms";
    pub const CAMPUS_ROOM_UPLOADS_ROUTE: &str = "/v1/campuses/{campus_id}/rooms/{room_id}/uploads";
    pub const CAMPUS_DOWNLOADS_ROUTE: &str = "/v1/campuses/{campus_id}/downloads";
    pub const WEBSOCKET: &str = "/ws";

    pub fn campus_rooms(campus_id: CampusId) -> String {
        format!("{CAMPUSES}/{campus_id}/rooms")
    }

    pub fn campus_room_uploads(campus_id: CampusId, room_id: RoomId) -> String {
        format!("{CAMPUSES}/{campus_id}/rooms/{room_id}/uploads")
    }

    pub fn campus_downloads(campus_id: CampusId) -> String {
        format!("{CAMPUSES}/{campus_id}/downloads")
    }
}
