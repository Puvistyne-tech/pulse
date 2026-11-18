use pulse_protocol::{MessageId, RoomId};

pub const THREAD_VIEW_FOLLOW_MS: u64 = 3000;
pub const THREAD_BURIED_DEPTH: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ThreadView {
    pub(crate) room_id: RoomId,
    pub(crate) root_message_id: MessageId,
    pub(crate) opened_at_ms: u64,
    pub(crate) follow_sent: bool,
    pub(crate) pending_follow: bool,
    pub(crate) syncing: bool,
}

impl ThreadView {
    pub(crate) fn new(room_id: RoomId, root_message_id: MessageId, opened_at_ms: u64) -> Self {
        Self {
            room_id,
            root_message_id,
            opened_at_ms,
            follow_sent: false,
            pending_follow: false,
            syncing: true,
        }
    }

    pub(crate) fn follow_deadline_ms(&self) -> Option<u64> {
        (!self.follow_sent).then(|| self.opened_at_ms.saturating_add(THREAD_VIEW_FOLLOW_MS))
    }

    pub(crate) fn follow_due(&self, now_ms: u64) -> bool {
        self.follow_deadline_ms()
            .is_some_and(|deadline| now_ms >= deadline)
    }

    pub(crate) fn shows(&self, room_id: RoomId, root_message_id: MessageId) -> bool {
        self.room_id == room_id && self.root_message_id == root_message_id
    }
}
