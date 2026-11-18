use uuid::Uuid;

use crate::auth::AuthToken;
use crate::ids::{
    AccountId, AttachmentId, CampusId, ClientNonce, DraftId, LoungeId, MessageId, RoomId,
    SessionId, ShelfItemId, StreamId, ToolCallId,
};

pub const CAMPUS_A: CampusId = CampusId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_00000000000a));
pub const CAMPUS_B: CampusId = CampusId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_00000000000b));
pub const CAMPUS_C: CampusId = CampusId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_00000000000c));
pub const SESSION_A: SessionId = SessionId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000a1));
pub const ACCOUNT_A: AccountId = AccountId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000c1));
pub const ACCOUNT_B: AccountId = AccountId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000c2));
pub const ACCOUNT_C: AccountId = AccountId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000c3));
pub const ROOM_A: RoomId = RoomId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000d1));
pub const ROOM_B: RoomId = RoomId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000d2));
pub const MESSAGE_A: MessageId = MessageId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000e1));
pub const MESSAGE_B: MessageId = MessageId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000e2));
pub const MESSAGE_C: MessageId = MessageId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000e3));
pub const MESSAGE_D: MessageId = MessageId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000e4));
pub const NONCE_A: ClientNonce =
    ClientNonce(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000f1));
pub const NONCE_B: ClientNonce =
    ClientNonce(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000f2));
pub const NONCE_C: ClientNonce =
    ClientNonce(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000f3));

pub const ATTACHMENT_A: AttachmentId =
    AttachmentId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000a01));
pub const ATTACHMENT_B: AttachmentId =
    AttachmentId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000a02));
pub const SHELF_ITEM_A: ShelfItemId =
    ShelfItemId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000b01));
pub const DRAFT_A: DraftId = DraftId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000c01));

pub const STREAM_A: StreamId = StreamId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000d01));
pub const STREAM_B: StreamId = StreamId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000d02));
pub const LOUNGE_A: LoungeId = LoungeId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000e01));
pub const LOUNGE_B: LoungeId = LoungeId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000e02));
pub const STEWARD_A: AccountId = AccountId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_0000000000c4));
pub const TOOL_CALL_A: ToolCallId =
    ToolCallId(Uuid::from_u128(0x0193a5c0_0000_7000_8000_000000000f01));

pub const LINK_A: &str = "https://example.org/a";

pub const EMOJI_A: &str = "👍";
pub const EMOJI_B: &str = "🎉";

pub const AUTH_TOKEN_A: &str = "fixture-token-a";
pub const AUTH_TOKEN_B: &str = "fixture-token-b";
pub const STEWARD_TOKEN_A: &str = "pst_fixture-steward-a";
pub const VALID_PASSWORD: &str = "correct-horse-battery";

pub fn auth_token(text: &str) -> AuthToken {
    AuthToken(text.to_owned())
}
