use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;

use pulse_protocol::{
    AccountId, InboxItem, InboxItemKind, InboxSnapshot, InboxStep, MessageId, ReplyPreview,
    RoomMessage, SubscriptionState, ThreadSummary, body_segments,
};

const PREVIEW_MAX_CHARS: usize = 140;

pub(crate) fn preview_text(message: &RoomMessage) -> String {
    body_segments(&message.body, &message.mentions)
        .iter()
        .map(|segment| segment.text.as_str())
        .collect::<String>()
        .chars()
        .take(PREVIEW_MAX_CHARS)
        .collect()
}

pub(crate) fn reply_preview(message: &RoomMessage) -> ReplyPreview {
    ReplyPreview {
        message_id: message.message_id,
        author_id: message.author_id,
        author_display_name: message.author_display_name.clone(),
        preview: preview_text(message),
    }
}

fn freshness(message: &RoomMessage) -> (u64, Option<MessageId>) {
    (message.activity_seq, message.latest_reply_id)
}

pub(crate) fn is_newer(incoming: &RoomMessage, existing: &RoomMessage) -> bool {
    freshness(incoming) > freshness(existing)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ThreadTrack {
    baseline: Option<ThreadSummary>,
    root: Option<RoomMessage>,
    latest: Option<RoomMessage>,
    live_replies: BTreeSet<MessageId>,
    live_unread: BTreeSet<MessageId>,
    live_mentions: BTreeSet<MessageId>,
    read_through: Option<MessageId>,
    confirmed: Option<(SubscriptionState, u64)>,
    requested: Option<SubscriptionState>,
}

fn count_after(live: &BTreeSet<MessageId>, floor: MessageId) -> u32 {
    u32::try_from(
        live.range((Bound::Excluded(floor), Bound::Unbounded))
            .count(),
    )
    .unwrap_or(u32::MAX)
}

impl ThreadTrack {
    fn watermark(&self) -> Option<MessageId> {
        self.baseline.as_ref().map(|baseline| {
            baseline
                .root
                .latest_reply_id
                .unwrap_or(baseline.root.message_id)
        })
    }

    fn state(&self) -> Option<SubscriptionState> {
        self.requested
            .or_else(|| self.confirmed.map(|(state, _)| state))
    }

    fn unread(&self, base: u32, live: &BTreeSet<MessageId>) -> u32 {
        let Some(watermark) = self.watermark() else {
            return 0;
        };
        let covered = self.read_through.is_some_and(|read| read >= watermark);
        let floor = self
            .read_through
            .map_or(watermark, |read| read.max(watermark));
        (if covered { 0 } else { base }).saturating_add(count_after(live, floor))
    }

    fn unread_replies(&self) -> u32 {
        let base = self
            .baseline
            .as_ref()
            .map_or(0, |baseline| baseline.unread_replies);
        self.unread(base, &self.live_unread)
    }

    fn unread_mentions(&self) -> u32 {
        let base = self
            .baseline
            .as_ref()
            .map_or(0, |baseline| baseline.unread_mentions);
        self.unread(base, &self.live_mentions)
    }

    fn advance_read(&mut self, through: MessageId) -> bool {
        if self.read_through.is_some_and(|read| read >= through) {
            return false;
        }
        self.read_through = Some(through);
        self.live_unread.retain(|id| *id > through);
        self.live_mentions.retain(|id| *id > through);
        true
    }

    fn note_latest(&mut self, message: &RoomMessage) {
        let replace = self.latest.as_ref().is_none_or(|latest| {
            message.message_id > latest.message_id
                || (message.message_id == latest.message_id && is_newer(message, latest))
        });
        if replace {
            self.latest = Some(message.clone());
        }
    }

    fn note_root(&mut self, message: &RoomMessage) {
        if self
            .root
            .as_ref()
            .is_none_or(|root| is_newer(message, root))
        {
            self.root = Some(message.clone());
        }
    }

    fn item(&self) -> Option<InboxItem> {
        let baseline = self.baseline.as_ref()?;
        let state = self.state()?;
        let root = self.root.as_ref().unwrap_or(&baseline.root);
        let watermark = self.watermark()?;
        let latest = self.latest.as_ref().or(baseline.latest_reply.as_ref());
        Some(InboxItem {
            kind: InboxItemKind::Thread,
            room_id: root.room_id,
            root_message_id: root.message_id,
            topic_title: root.topic_title.clone(),
            root_author_display_name: root.author_display_name.clone(),
            root_preview: preview_text(root),
            latest_reply: latest.map(reply_preview),
            reply_count: baseline
                .root
                .reply_count
                .saturating_add(count_after(&self.live_replies, watermark)),
            unread_replies: self.unread_replies(),
            unread_mentions: self.unread_mentions(),
            subscription: state,
            activity_message_id: latest.map_or(root.message_id, |latest| {
                latest.message_id.max(root.message_id)
            }),
        })
    }
}

fn has_unread(item: &InboxItem) -> bool {
    item.unread_mentions > 0
        || (item.subscription != SubscriptionState::Muted && item.unread_replies > 0)
}

fn is_visible(item: &InboxItem) -> bool {
    match item.subscription {
        SubscriptionState::Following => true,
        SubscriptionState::Archived => has_unread(item),
        SubscriptionState::Muted => item.unread_mentions > 0,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct InboxTracker {
    tracks: BTreeMap<MessageId, ThreadTrack>,
    selection: Option<(MessageId, usize)>,
    open: bool,
    replying: bool,
}

impl InboxTracker {
    pub(crate) fn observe(&mut self, message: &RoomMessage, self_id: AccountId) {
        let own = message.author_id == self_id;
        let mentions_self = !own
            && message
                .mentions
                .iter()
                .any(|mention| mention.account_id == self_id);
        let message_id = message.message_id;
        match message.thread_root_id {
            Some(root) => {
                let track = self.tracks.entry(root).or_default();
                track.live_replies.insert(message_id);
                if own {
                    track.advance_read(message_id);
                } else {
                    if track.read_through.is_none_or(|read| message_id > read) {
                        track.live_unread.insert(message_id);
                    }
                    if mentions_self && track.read_through.is_none_or(|read| message_id > read) {
                        track.live_mentions.insert(message_id);
                    }
                }
                track.note_latest(message);
            }
            None => {
                if mentions_self {
                    self.tracks
                        .entry(message_id)
                        .or_default()
                        .live_mentions
                        .insert(message_id);
                }
                if let Some(track) = self.tracks.get_mut(&message_id) {
                    track.note_root(message);
                }
            }
        }
    }

    pub(crate) fn apply_summary(&mut self, summary: ThreadSummary) {
        let root_id = summary.root.message_id;
        let track = self.tracks.entry(root_id).or_default();
        let incoming_watermark = summary
            .root
            .latest_reply_id
            .unwrap_or(summary.root.message_id);
        let current_revision = track.confirmed.map(|(_, revision)| revision);
        let accepted = match (current_revision, track.watermark()) {
            (Some(current), Some(watermark)) => {
                summary.revision > current
                    || (summary.revision == current && incoming_watermark >= watermark)
            }
            _ => true,
        };
        if !accepted {
            return;
        }
        if current_revision.is_none_or(|current| summary.revision >= current) {
            track.confirmed = Some((summary.subscription, summary.revision));
            if track.requested == Some(summary.subscription) {
                track.requested = None;
            }
        }
        if let Some(read) = summary.read_through {
            track.advance_read(read);
        }
        track.note_root(&summary.root);
        if let Some(latest) = &summary.latest_reply {
            track.note_latest(latest);
        }
        track.baseline = Some(summary);
        let watermark = track.watermark();
        if let Some(watermark) = watermark {
            track.live_replies.retain(|id| *id > watermark);
            track.live_unread.retain(|id| *id > watermark);
            track.live_mentions.retain(|id| *id > watermark);
        }
    }

    pub(crate) fn request_state(&mut self, root: MessageId, state: SubscriptionState) {
        self.tracks.entry(root).or_default().requested = Some(state);
    }

    pub(crate) fn reject(&mut self, root: MessageId) {
        if let Some(track) = self.tracks.get_mut(&root) {
            track.requested = None;
        }
    }

    pub(crate) fn clear_requests(&mut self) {
        self.tracks
            .values_mut()
            .for_each(|track| track.requested = None);
    }

    pub(crate) fn mark_read(&mut self, root: MessageId, through: MessageId) -> bool {
        self.tracks.entry(root).or_default().advance_read(through)
    }

    pub(crate) fn thread_state(&self, root: MessageId) -> Option<(SubscriptionState, u32)> {
        let track = self.tracks.get(&root)?;
        track.state().map(|state| (state, track.unread_replies()))
    }

    pub(crate) fn root_copy(&self, root: MessageId) -> Option<&RoomMessage> {
        let track = self.tracks.get(&root)?;
        track
            .root
            .as_ref()
            .or_else(|| track.baseline.as_ref().map(|baseline| &baseline.root))
    }

    fn items(&self) -> Vec<InboxItem> {
        let mut items = self
            .tracks
            .values()
            .filter_map(ThreadTrack::item)
            .filter(is_visible)
            .collect::<Vec<_>>();
        items.sort_by(|left, right| {
            right
                .activity_message_id
                .cmp(&left.activity_message_id)
                .then(right.root_message_id.cmp(&left.root_message_id))
        });
        items
    }

    pub(crate) fn selected_item(&self) -> Option<InboxItem> {
        let (root, _) = self.selection?;
        self.items()
            .into_iter()
            .find(|item| item.root_message_id == root)
    }

    pub(crate) fn toggle(&mut self) {
        self.open = !self.open;
        self.replying = false;
        self.selection = None;
        self.refresh();
    }

    pub(crate) fn step_selection(&mut self, step: InboxStep) {
        if !self.open {
            return;
        }
        let items = self.items();
        let Some(last) = items.len().checked_sub(1) else {
            return;
        };
        let next = match (self.selection, step) {
            (Some((_, index)), InboxStep::Next) => (index + 1).min(last),
            (Some((_, index)), InboxStep::Previous) => index.saturating_sub(1),
            (None, _) => 0,
        };
        self.replying = false;
        self.selection = Some((items[next].root_message_id, next));
    }

    pub(crate) fn start_reply(&mut self) {
        self.replying = self.open && self.selection.is_some();
    }

    pub(crate) fn cancel_reply(&mut self) {
        self.replying = false;
    }

    pub(crate) fn refresh(&mut self) {
        if !self.open {
            self.selection = None;
            self.replying = false;
            return;
        }
        let items = self.items();
        let previous = self.selection;
        self.selection = match (previous, items.len().checked_sub(1)) {
            (_, None) => None,
            (None, Some(_)) => Some((items[0].root_message_id, 0)),
            (Some((root, index)), Some(last)) => {
                match items.iter().position(|item| item.root_message_id == root) {
                    Some(position) => Some((root, position)),
                    None => {
                        let clamped = index.min(last);
                        Some((items[clamped].root_message_id, clamped))
                    }
                }
            }
        };
        if self.selection.map(|(root, _)| root) != previous.map(|(root, _)| root) {
            self.replying = false;
        }
    }

    pub(crate) fn snapshot(&self) -> InboxSnapshot {
        let items = self.items();
        let unread_threads =
            u32::try_from(items.iter().filter(|item| has_unread(item)).count()).unwrap_or(u32::MAX);
        InboxSnapshot {
            open: self.open,
            items,
            unread_threads,
            selected_root_message_id: self.selection.map(|(root, _)| root),
            replying: self.replying,
        }
    }
}
