use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{CampusId, RoomId};
use crate::rbac::Role;

pub const DEFAULT_ROOM_SLUGS: [&str; 2] = ["commons", "courtyard"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct CampusSummary {
    pub campus_id: CampusId,
    pub slug: String,
    pub display_name: String,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct CreateCampusRequest {
    pub slug: String,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Room {
    pub room_id: RoomId,
    pub campus_id: CampusId,
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct CampusCreated {
    pub campus: CampusSummary,
    pub rooms: Vec<Room>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct CampusList {
    pub campuses: Vec<CampusSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RoomList {
    pub campus_id: CampusId,
    pub rooms: Vec<Room>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct CreateRoomRequest {
    pub slug: String,
}
