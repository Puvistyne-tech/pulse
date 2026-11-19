use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{AccountId, CampusId};
use crate::rbac::{Permission, Role, role_permits};

pub const ASSIGNABLE_ROLES: [Role; 3] = [Role::Admin, Role::Member, Role::Guest];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct CampusMember {
    pub campus_id: CampusId,
    pub account_id: AccountId,
    pub display_name: String,
    pub email: String,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MemberList {
    pub campus_id: CampusId,
    pub members: Vec<CampusMember>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct AddMemberRequest {
    pub email: String,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct UpdateMemberRequest {
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct MembershipRevoked {
    pub campus_id: CampusId,
    pub account_id: AccountId,
}

pub const fn member_manageable(actor: Role, target: Role) -> bool {
    role_permits(actor, Permission::ManageMembers)
        && match target {
            Role::Owner | Role::Steward => false,
            Role::Admin => matches!(actor, Role::Owner),
            Role::Member | Role::Guest => true,
        }
}

pub const fn role_assignable(actor: Role, role: Role) -> bool {
    member_manageable(actor, role)
}

pub fn assignable_roles(actor: Role) -> Vec<Role> {
    ASSIGNABLE_ROLES
        .into_iter()
        .filter(|role| role_assignable(actor, *role))
        .collect()
}
