use std::collections::{BTreeMap, BTreeSet};

use pulse_protocol::{
    ApiError, ApiErrorCode, InputField, MessageId, ReminderPreset, RoomId, RoomMessage,
    ShelfAction, ShelfBucket, ShelfChanged, ShelfEntry, ShelfItem, ShelfItemId, ShelfRejected,
    ShelfRejection, ShelfSnapshot, ShelfSynced,
};

use crate::inbox::preview_text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfIntent {
    Toggled,
    UpdateRequested {
        room_id: RoomId,
        message_id: MessageId,
        action: ShelfAction,
        preset: Option<ReminderPreset>,
        remind_at_ms: Option<u64>,
    },
    ReminderDismissed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pending {
    action: ShelfAction,
    remind_at_ms: Option<u64>,
    message: RoomMessage,
    base_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ShelfView {
    shelf_item_id: Option<ShelfItemId>,
    message: RoomMessage,
    remind_at_ms: Option<u64>,
    reminded_at_ms: Option<u64>,
    done_at_ms: Option<u64>,
    saved_at_ms: u64,
    pending: bool,
}

impl ShelfView {
    fn confirmed(item: &ShelfItem) -> Self {
        Self {
            shelf_item_id: Some(item.shelf_item_id),
            message: item.message.clone(),
            remind_at_ms: item.remind_at_ms,
            reminded_at_ms: item.reminded_at_ms,
            done_at_ms: item.done_at_ms,
            saved_at_ms: item.saved_at_ms,
            pending: false,
        }
    }

    fn bucket(&self, clock_ms: u64) -> ShelfBucket {
        if self.done_at_ms.is_some() {
            ShelfBucket::Done
        } else if self.reminded_at_ms.is_some()
            || self.remind_at_ms.is_some_and(|remind| remind <= clock_ms)
        {
            ShelfBucket::Due
        } else {
            ShelfBucket::Upcoming
        }
    }

    fn entry(&self, clock_ms: u64) -> ShelfEntry {
        ShelfEntry {
            shelf_item_id: self.shelf_item_id,
            room_id: self.message.room_id,
            message_id: self.message.message_id,
            author_display_name: self.message.author_display_name.clone(),
            preview: preview_text(&self.message),
            remind_at_ms: self.remind_at_ms,
            done_at_ms: self.done_at_ms,
            bucket: self.bucket(clock_ms),
            pending: self.pending,
        }
    }
}

fn apply_pending(
    pending: &Pending,
    existing: Option<&ShelfItem>,
    clock_ms: u64,
) -> Option<ShelfView> {
    let mut view = existing.map_or_else(
        || ShelfView {
            shelf_item_id: None,
            message: pending.message.clone(),
            remind_at_ms: None,
            reminded_at_ms: None,
            done_at_ms: None,
            saved_at_ms: clock_ms,
            pending: true,
        },
        ShelfView::confirmed,
    );
    view.pending = true;
    match pending.action {
        ShelfAction::Save | ShelfAction::Reschedule => {
            view.remind_at_ms = pending.remind_at_ms;
            view.reminded_at_ms = None;
            view.done_at_ms = None;
            Some(view)
        }
        ShelfAction::Complete => existing.map(|_| {
            view.done_at_ms = Some(clock_ms);
            view
        }),
        ShelfAction::Reopen => existing.map(|_| {
            view.done_at_ms = None;
            view
        }),
        ShelfAction::Remove => None,
    }
}

fn rejection_error(reason: ShelfRejection) -> ApiError {
    match reason {
        ShelfRejection::Forbidden => ApiError::new(ApiErrorCode::Forbidden),
        ShelfRejection::UnknownMessage | ShelfRejection::UnknownItem => {
            ApiError::new(ApiErrorCode::NotFound)
        }
        ShelfRejection::InvalidRemindAt => ApiError::invalid(InputField::RemindAt),
        ShelfRejection::ShelfFull => ApiError::new(ApiErrorCode::Conflict),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ShelfTracker {
    open: bool,
    items: BTreeMap<MessageId, ShelfItem>,
    pending: BTreeMap<MessageId, Pending>,
    latest_reminder: Option<MessageId>,
    error: Option<ApiError>,
}

impl ShelfTracker {
    pub(crate) fn toggle(&mut self) {
        self.open = !self.open;
        self.error = None;
    }

    pub(crate) fn dismiss_reminder(&mut self) {
        self.latest_reminder = None;
    }

    pub(crate) fn set_error(&mut self, error: Option<ApiError>) {
        self.error = error;
    }

    pub(crate) fn request(
        &mut self,
        message: RoomMessage,
        action: ShelfAction,
        remind_at_ms: Option<u64>,
    ) {
        let base_revision = self
            .items
            .get(&message.message_id)
            .map_or(0, |item| item.revision);
        self.error = None;
        self.pending.insert(
            message.message_id,
            Pending {
                action,
                remind_at_ms,
                message,
                base_revision,
            },
        );
    }

    pub(crate) fn clear_pending(&mut self) {
        self.pending.clear();
    }

    pub(crate) fn apply_changed(&mut self, changed: ShelfChanged) {
        let message_id = changed.item.message.message_id;
        let existing = self.items.get(&message_id);
        let accepted = existing.is_none_or(|current| changed.item.revision >= current.revision);
        if self
            .pending
            .get(&message_id)
            .is_some_and(|pending| pending.base_revision <= changed.item.revision)
        {
            self.pending.remove(&message_id);
        }
        if !accepted {
            return;
        }
        if changed.removed {
            self.items.remove(&message_id);
            return;
        }
        let newly_reminded = changed.item.reminded_at_ms.is_some()
            && existing.is_none_or(|current| current.reminded_at_ms.is_none());
        if newly_reminded {
            self.latest_reminder = Some(message_id);
        }
        self.items.insert(message_id, changed.item);
    }

    pub(crate) fn apply_synced(&mut self, synced: ShelfSynced) {
        let mut next = BTreeMap::new();
        for item in synced.items {
            let message_id = item.message.message_id;
            let keep_local = self
                .items
                .get(&message_id)
                .is_some_and(|local| local.revision > item.revision);
            let chosen = if keep_local {
                self.items.remove(&message_id)
            } else {
                Some(item)
            };
            if let Some(chosen) = chosen {
                next.insert(message_id, chosen);
            }
        }
        self.items = next;
        self.pending.clear();
    }

    pub(crate) fn apply_rejected(&mut self, rejected: &ShelfRejected) {
        self.pending.remove(&rejected.message_id);
        self.error = Some(rejection_error(rejected.reason));
    }

    pub(crate) fn messages(&self) -> impl Iterator<Item = &RoomMessage> {
        self.items.values().map(|item| &item.message)
    }

    fn views(&self, clock_ms: u64) -> Vec<ShelfView> {
        let confirmed =
            self.items
                .iter()
                .filter_map(|(message_id, item)| match self.pending.get(message_id) {
                    None => Some(ShelfView::confirmed(item)),
                    Some(pending) => apply_pending(pending, Some(item), clock_ms),
                });
        let fresh = self
            .pending
            .iter()
            .filter(|(message_id, _)| !self.items.contains_key(message_id))
            .filter_map(|(_, pending)| apply_pending(pending, None, clock_ms));
        confirmed.chain(fresh).collect()
    }

    pub(crate) fn shelved_ids(&self) -> BTreeSet<MessageId> {
        let mut ids = self.items.keys().copied().collect::<BTreeSet<_>>();
        for (message_id, pending) in &self.pending {
            match pending.action {
                ShelfAction::Remove => {
                    ids.remove(message_id);
                }
                ShelfAction::Save | ShelfAction::Reschedule => {
                    ids.insert(*message_id);
                }
                ShelfAction::Complete | ShelfAction::Reopen => {}
            }
        }
        ids
    }

    pub(crate) fn next_deadline_ms(&self, clock_ms: u64) -> Option<u64> {
        self.views(clock_ms)
            .iter()
            .filter(|view| view.done_at_ms.is_none() && view.reminded_at_ms.is_none())
            .filter_map(|view| view.remind_at_ms)
            .filter(|remind| *remind > clock_ms)
            .min()
    }

    pub(crate) fn snapshot(&self, clock_ms: u64) -> ShelfSnapshot {
        let views = self.views(clock_ms);
        let mut due = Vec::new();
        let mut upcoming = Vec::new();
        let mut done = Vec::new();
        for view in &views {
            match view.bucket(clock_ms) {
                ShelfBucket::Due => due.push(view),
                ShelfBucket::Upcoming => upcoming.push(view),
                ShelfBucket::Done => done.push(view),
            }
        }
        due.sort_by_key(|view| view.remind_at_ms.unwrap_or(0));
        upcoming.sort_by(|left, right| {
            let key = |view: &ShelfView| (view.remind_at_ms.is_none(), view.remind_at_ms);
            key(left)
                .cmp(&key(right))
                .then(right.saved_at_ms.cmp(&left.saved_at_ms))
        });
        done.sort_by_key(|view| std::cmp::Reverse(view.done_at_ms));
        let entries = |views: Vec<&ShelfView>| {
            views
                .into_iter()
                .map(|view| view.entry(clock_ms))
                .collect::<Vec<_>>()
        };
        let due_count = u32::try_from(due.len()).unwrap_or(u32::MAX);
        let latest_reminder = self.latest_reminder.and_then(|message_id| {
            views
                .iter()
                .find(|view| view.message.message_id == message_id)
                .map(|view| view.entry(clock_ms))
        });
        ShelfSnapshot {
            open: self.open,
            due: entries(due),
            upcoming: entries(upcoming),
            done: entries(done),
            due_count,
            latest_reminder,
            presets: ReminderPreset::ALL.to_vec(),
            error: self.error,
        }
    }
}
