use core::str::FromStr;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Member,
    Guest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    ViewCampus,
    ViewRooms,
    CreateRoom,
    ManageMembers,
    ManageCampus,
}

#[derive(Debug, thiserror::Error)]
#[error("unknown role")]
pub struct RoleParseError;

impl Role {
    pub const ALL: [Role; 4] = [Role::Owner, Role::Admin, Role::Member, Role::Guest];

    pub const fn as_str(self) -> &'static str {
        match self {
            Role::Owner => "owner",
            Role::Admin => "admin",
            Role::Member => "member",
            Role::Guest => "guest",
        }
    }
}

impl FromStr for Role {
    type Err = RoleParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Role::ALL
            .into_iter()
            .find(|role| role.as_str() == value)
            .ok_or(RoleParseError)
    }
}

impl Permission {
    pub const ALL: [Permission; 5] = [
        Permission::ViewCampus,
        Permission::ViewRooms,
        Permission::CreateRoom,
        Permission::ManageMembers,
        Permission::ManageCampus,
    ];
}

pub const fn role_permits(role: Role, permission: Permission) -> bool {
    match permission {
        Permission::ViewCampus | Permission::ViewRooms => true,
        Permission::CreateRoom => !matches!(role, Role::Guest),
        Permission::ManageMembers => matches!(role, Role::Owner | Role::Admin),
        Permission::ManageCampus => matches!(role, Role::Owner),
    }
}
