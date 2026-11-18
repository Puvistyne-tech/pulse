use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
#[error("invalid identifier")]
pub struct IdParseError;

macro_rules! define_ids {
    ($($name:ident),+ $(,)?) => {$(
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, TS)]
        #[ts(type = "string")]
        pub struct $name(pub Uuid);

        impl $name {
            pub fn generate() -> Self {
                Self(Uuid::now_v7())
            }

            pub const fn as_uuid(&self) -> Uuid {
                self.0
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "{}", self.0.hyphenated())
            }
        }

        impl core::str::FromStr for $name {
            type Err = IdParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Uuid::parse_str(value).map(Self).map_err(|_| IdParseError)
            }
        }
    )+};
}

define_ids!(CampusId, SessionId, AccountId, RoomId);
