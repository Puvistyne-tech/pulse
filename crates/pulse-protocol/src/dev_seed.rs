use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::rbac::Role;

pub const DEV_SEED_PASSWORD: &str = "pulse-dev-password";
pub const DEV_SEED_CAMPUS_SLUG: &str = "demo-campus";
pub const DEV_SEED_CAMPUS_NAME: &str = "Demo Campus";
pub const DEV_SEED_PEOPLE: [(&str, &str, Role); 5] = [
    ("ada@pulse.test", "Ada Lovelace", Role::Owner),
    ("grace@pulse.test", "Grace Hopper", Role::Admin),
    ("linus@pulse.test", "Linus Pauling", Role::Member),
    ("margaret@pulse.test", "Margaret Hamilton", Role::Member),
    ("alan@pulse.test", "Alan Kay", Role::Guest),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct DevAccount {
    pub email: String,
    pub display_name: String,
    pub role: Role,
}

pub fn dev_accounts() -> Vec<DevAccount> {
    DEV_SEED_PEOPLE
        .into_iter()
        .map(|(email, display_name, role)| DevAccount {
            email: email.to_owned(),
            display_name: display_name.to_owned(),
            role,
        })
        .collect()
}
