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
    Steward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    ViewCampus,
    ViewRooms,
    CreateRoom,
    ManageMembers,
    ManageCampus,
    PostMessage,
    React,
    PinMessage,
    UploadAttachment,
    ManageStewards,
    StreamReply,
    UseStewardTools,
    JoinLounge,
    SpeakInLounge,
}

#[derive(Debug, thiserror::Error)]
#[error("unknown role")]
pub struct RoleParseError;

impl Role {
    pub const ALL: [Role; 5] = [
        Role::Owner,
        Role::Admin,
        Role::Member,
        Role::Guest,
        Role::Steward,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Role::Owner => "owner",
            Role::Admin => "admin",
            Role::Member => "member",
            Role::Guest => "guest",
            Role::Steward => "steward",
        }
    }

    pub const fn is_steward(self) -> bool {
        matches!(self, Role::Steward)
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
    pub const ALL: [Permission; 14] = [
        Permission::ViewCampus,
        Permission::ViewRooms,
        Permission::CreateRoom,
        Permission::ManageMembers,
        Permission::ManageCampus,
        Permission::PostMessage,
        Permission::React,
        Permission::PinMessage,
        Permission::UploadAttachment,
        Permission::ManageStewards,
        Permission::StreamReply,
        Permission::UseStewardTools,
        Permission::JoinLounge,
        Permission::SpeakInLounge,
    ];
}

pub const fn role_permits(role: Role, permission: Permission) -> bool {
    match permission {
        Permission::ViewCampus | Permission::ViewRooms | Permission::React => true,
        Permission::PostMessage => !matches!(role, Role::Guest),
        Permission::CreateRoom | Permission::PinMessage | Permission::UploadAttachment => {
            matches!(role, Role::Owner | Role::Admin | Role::Member)
        }
        Permission::ManageMembers | Permission::ManageStewards => {
            matches!(role, Role::Owner | Role::Admin)
        }
        Permission::ManageCampus => matches!(role, Role::Owner),
        Permission::StreamReply | Permission::UseStewardTools => matches!(role, Role::Steward),
        Permission::JoinLounge => !matches!(role, Role::Steward),
        Permission::SpeakInLounge => matches!(role, Role::Owner | Role::Admin | Role::Member),
    }
}
