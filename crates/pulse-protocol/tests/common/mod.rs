#![allow(dead_code)]

use pulse_protocol::fixtures::{ACCOUNT_A, CAMPUS_A, NONCE_A, ROOM_A};
use pulse_protocol::{MessageId, RoomMessage};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::fmt::Debug;

pub fn assert_wire<T>(value: T, expected: Value)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    assert_eq!(serde_json::to_value(&value).expect("serialize"), expected);
    assert_eq!(
        serde_json::from_value::<T>(expected).expect("deserialize"),
        value
    );
}

pub fn room_message(message_id: MessageId, previous: Option<MessageId>) -> RoomMessage {
    RoomMessage {
        message_id,
        previous_message_id: previous,
        campus_id: CAMPUS_A,
        room_id: ROOM_A,
        author_id: ACCOUNT_A,
        author_display_name: "Ada".to_owned(),
        client_nonce: NONCE_A,
        body: "hello".to_owned(),
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

pub fn room_message_json(message_id: MessageId, previous: Option<MessageId>) -> Value {
    json!({
        "message_id": message_id.to_string(),
        "previous_message_id": previous.map(|id| id.to_string()),
        "campus_id": CAMPUS_A.to_string(),
        "room_id": ROOM_A.to_string(),
        "author_id": ACCOUNT_A.to_string(),
        "author_display_name": "Ada",
        "client_nonce": NONCE_A.to_string(),
        "body": "hello",
        "thread_root_id": null,
        "topic_title": null,
        "reply_count": 0,
        "latest_reply_id": null,
        "mentions": [],
        "reactions": [],
        "activity_seq": 0,
        "attachments": [],
        "pin": null,
    })
}
