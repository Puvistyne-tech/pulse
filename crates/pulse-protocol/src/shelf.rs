use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ids::{MessageId, RoomId, ShelfItemId};
use crate::messaging::RoomMessage;

pub const SHELF_ITEMS_MAX: usize = 500;
pub const SHELF_SYNC_LIMIT: usize = 500;
pub const REMIND_AT_PAST_TOLERANCE_MS: u64 = 300_000;
pub const REMIND_AT_HORIZON_MS: u64 = 31_622_400_000;
pub const REMINDER_PICKER_STEP_MS: u64 = 300_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ShelfAction {
    Save,
    Reschedule,
    Complete,
    Reopen,
    Remove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ReminderPreset {
    InTwentyMinutes,
    InOneHour,
    InThreeHours,
    Tomorrow,
    NextWeek,
}

impl ReminderPreset {
    pub const ALL: [ReminderPreset; 5] = [
        ReminderPreset::InTwentyMinutes,
        ReminderPreset::InOneHour,
        ReminderPreset::InThreeHours,
        ReminderPreset::Tomorrow,
        ReminderPreset::NextWeek,
    ];

    pub const fn offset_ms(self) -> u64 {
        match self {
            ReminderPreset::InTwentyMinutes => 20 * 60 * 1000,
            ReminderPreset::InOneHour => 60 * 60 * 1000,
            ReminderPreset::InThreeHours => 3 * 60 * 60 * 1000,
            ReminderPreset::Tomorrow => 24 * 60 * 60 * 1000,
            ReminderPreset::NextWeek => 7 * 24 * 60 * 60 * 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ShelfBucket {
    Due,
    Upcoming,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct UpdateShelf {
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub action: ShelfAction,
    #[ts(type = "number | null")]
    pub remind_at_ms: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct SyncShelf {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ShelfItem {
    pub shelf_item_id: ShelfItemId,
    pub message: RoomMessage,
    #[ts(type = "number | null")]
    pub remind_at_ms: Option<u64>,
    #[ts(type = "number | null")]
    pub reminded_at_ms: Option<u64>,
    #[ts(type = "number | null")]
    pub done_at_ms: Option<u64>,
    #[ts(type = "number")]
    pub saved_at_ms: u64,
    #[ts(type = "number")]
    pub revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ShelfSynced {
    pub items: Vec<ShelfItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ShelfChanged {
    pub item: ShelfItem,
    pub removed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ShelfRejection {
    Forbidden,
    UnknownMessage,
    UnknownItem,
    InvalidRemindAt,
    ShelfFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ShelfRejected {
    pub room_id: RoomId,
    pub message_id: MessageId,
    pub action: ShelfAction,
    pub reason: ShelfRejection,
}

pub fn picker_anchor_ms(clock_ms: u64) -> Option<u64> {
    if clock_ms == 0 {
        return None;
    }
    clock_ms
        .div_ceil(REMINDER_PICKER_STEP_MS)
        .checked_mul(REMINDER_PICKER_STEP_MS)
}

pub fn picker_latest_ms(clock_ms: u64) -> Option<u64> {
    if clock_ms == 0 {
        return None;
    }
    clock_ms.checked_add(REMIND_AT_HORIZON_MS)
}
