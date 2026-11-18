use uuid::Uuid;

use crate::auth::AuthToken;
use crate::ids::{AccountId, CampusId, RoomId, SessionId};

pub const CAMPUS_A: CampusId = CampusId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_00000000000a));
pub const CAMPUS_B: CampusId = CampusId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_00000000000b));
pub const SESSION_A: SessionId = SessionId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000a1));
pub const ACCOUNT_A: AccountId = AccountId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000c1));
pub const ACCOUNT_B: AccountId = AccountId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000c2));
pub const ROOM_A: RoomId = RoomId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000d1));

pub const AUTH_TOKEN_A: &str = "fixture-token-a";
pub const AUTH_TOKEN_B: &str = "fixture-token-b";
pub const VALID_PASSWORD: &str = "correct-horse-battery";

pub fn auth_token(text: &str) -> AuthToken {
    AuthToken(text.to_owned())
}
