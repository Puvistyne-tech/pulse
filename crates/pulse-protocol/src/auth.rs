use core::fmt;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::AccountId;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
pub struct AuthToken(pub String);

impl fmt::Debug for AuthToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AuthToken(redacted)")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct AccountProfile {
    pub account_id: AccountId,
    pub email: String,
    pub display_name: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SignUpRequest {
    pub email: String,
    pub display_name: String,
    pub password: String,
}

impl fmt::Debug for SignUpRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SignUpRequest")
            .field("email", &"redacted")
            .field("display_name", &self.display_name)
            .field("password", &"redacted")
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

impl fmt::Debug for SignInRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SignInRequest")
            .field("email", &"redacted")
            .field("password", &"redacted")
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct AuthSession {
    pub account: AccountProfile,
    pub token: AuthToken,
}
