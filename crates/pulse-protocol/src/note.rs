use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{AccountId, CampusId, RoomId};

pub const NOTE_MEMBERS_MAX: usize = 8;
pub const NOTE_SLUG_PREFIX: &str = "note-";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct NoteMember {
    pub account_id: AccountId,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct Note {
    pub campus_id: CampusId,
    pub room_id: RoomId,
    pub members: Vec<NoteMember>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct NoteList {
    pub campus_id: CampusId,
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct StartNoteRequest {
    pub account_ids: Vec<AccountId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct NoteStarted {
    pub note: Note,
}

pub fn note_member_key(members: &[AccountId]) -> String {
    let mut sorted: Vec<String> = members.iter().map(ToString::to_string).collect();
    sorted.sort();
    sorted.dedup();
    sorted.join(",")
}

pub fn note_title(note: &Note, viewer: AccountId) -> String {
    let others: Vec<&str> = note
        .members
        .iter()
        .filter(|member| member.account_id != viewer)
        .map(|member| member.display_name.as_str())
        .collect();
    if others.is_empty() {
        note.members
            .iter()
            .map(|member| member.display_name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        others.join(", ")
    }
}
