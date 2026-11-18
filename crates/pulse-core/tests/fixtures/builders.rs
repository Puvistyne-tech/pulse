#![allow(dead_code)]

use pulse_protocol::fixtures::{ACCOUNT_A, ACCOUNT_C, CAMPUS_A};
use pulse_protocol::{AccountId, ClientNonce, MessageId, RoomId, RoomMessage};

pub fn indexed_id(base: u128, index: u128) -> String {
    format!("0193a5c0-0000-7000-8000-{:012x}", base + index)
}

pub fn message_id(index: u128) -> MessageId {
    indexed_id(0x1000_0000, index).parse().expect("message id")
}

pub fn nonce(index: u128) -> ClientNonce {
    indexed_id(0x2000_0000, index).parse().expect("nonce")
}

pub fn display_name(account_id: AccountId) -> &'static str {
    match account_id {
        ACCOUNT_A => "Ada",
        ACCOUNT_C => "Linus",
        _ => "Grace",
    }
}

pub fn room_message(
    room_id: RoomId,
    message_id: MessageId,
    previous_message_id: Option<MessageId>,
    author_id: AccountId,
    client_nonce: ClientNonce,
    body: &str,
) -> RoomMessage {
    RoomMessage {
        message_id,
        previous_message_id,
        campus_id: CAMPUS_A,
        room_id,
        author_id,
        author_display_name: display_name(author_id).to_owned(),
        client_nonce,
        body: body.to_owned(),
        thread_root_id: None,
        topic_title: None,
        reply_count: 0,
        latest_reply_id: None,
        mentions: Vec::new(),
        reactions: Vec::new(),
        activity_seq: 0,
        attachments: Vec::new(),
        pin: None,
    }
}
