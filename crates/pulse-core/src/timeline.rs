use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;

use pulse_protocol::{
    AccountProfile, ApiError, ApiErrorCode, CampusId, ClientFrame, ClientNonce, DeliveryState,
    InboxSnapshot, InboxStep, InputField, MarkThreadRead, MessageId, MessageRejection,
    REACTION_PALETTE, React, ReactionAction, ReactionRejection, ReactionSummary, RoomCursor,
    RoomId, RoomMessage, SYNC_MAX_ROOMS, SendMessage, ServerFrame, SetThreadSubscription,
    SetTopicTitle, SubscriptionOrigin, SubscriptionState, SyncInbox, SyncRooms, SyncThread,
    ThreadPreview, ThreadRejection, ThreadSnapshot, ThreadSynced, TimelineEntry, TimelineSnapshot,
    body_segments, mention_token, normalize_message_body, normalize_reaction_emoji,
    normalize_topic_title,
};

use crate::inbox::{InboxTracker, is_newer, reply_preview};
use crate::thread_view::{THREAD_BURIED_DEPTH, ThreadView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadIntent {
    ReplyRequested {
        room_id: RoomId,
        root_message_id: MessageId,
        client_nonce: ClientNonce,
        body: String,
    },
    ReactRequested {
        room_id: RoomId,
        message_id: MessageId,
        emoji: String,
        action: ReactionAction,
    },
    TopicTitleRequested {
        room_id: RoomId,
        root_message_id: MessageId,
        topic_title: String,
    },
    SubscriptionRequested {
        room_id: RoomId,
        root_message_id: MessageId,
        state: SubscriptionState,
    },
    ThreadOpened {
        room_id: RoomId,
        root_message_id: MessageId,
        now_ms: u64,
    },
    ThreadClosed,
    ClockAdvanced {
        now_ms: u64,
    },
    InboxToggled,
    InboxSelectionMoved(InboxStep),
    InboxSelectionArchived,
    InboxReplyStarted,
    InboxReplyCancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum TimelineEvent {
    Connected {
        rooms: Vec<RoomId>,
    },
    RoomsChanged {
        rooms: Vec<RoomId>,
    },
    Disconnected,
    SendRequested {
        room_id: RoomId,
        client_nonce: ClientNonce,
        body: String,
    },
    DiscardRequested {
        client_nonce: ClientNonce,
    },
    FrameReceived(ServerFrame),
    Thread(ThreadIntent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Outgoing {
    room_id: RoomId,
    body: String,
    thread_root_id: Option<MessageId>,
    failed: Option<MessageRejection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct RoomLog {
    messages: BTreeMap<MessageId, RoomMessage>,
    cursor: Option<MessageId>,
    activity: Option<u64>,
    window_start: Option<MessageId>,
    syncing: bool,
}

impl RoomLog {
    fn advance_chain(&mut self) {
        loop {
            let lower = self.cursor.map_or(Bound::Unbounded, Bound::Excluded);
            let next = self
                .messages
                .range((lower, Bound::Unbounded))
                .next()
                .filter(|(_, message)| message.previous_message_id == self.cursor);
            let Some((message_id, _)) = next else {
                return;
            };
            self.cursor = Some(*message_id);
        }
    }

    fn has_gap(&self) -> bool {
        let lower = self.cursor.map_or(Bound::Unbounded, Bound::Excluded);
        self.messages
            .range((lower, Bound::Unbounded))
            .next()
            .is_some()
    }

    fn cursor_request(&self, room_id: RoomId) -> RoomCursor {
        RoomCursor {
            room_id,
            after: self.cursor,
            activity_after: self.activity,
        }
    }

    fn in_stream(&self, message: &RoomMessage) -> bool {
        message.thread_root_id.is_none()
            && self
                .window_start
                .is_none_or(|start| message.message_id >= start)
    }

    fn replies_to(&self, root: MessageId) -> Vec<&RoomMessage> {
        self.messages
            .range((Bound::Excluded(root), Bound::Unbounded))
            .map(|(_, message)| message)
            .filter(|message| message.thread_root_id == Some(root))
            .collect()
    }

    fn buried_depth_between(&self, root: MessageId, latest: MessageId) -> usize {
        self.messages
            .range((Bound::Excluded(root), Bound::Excluded(latest)))
            .filter(|(_, message)| self.in_stream(message))
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelineMachine {
    campus_id: CampusId,
    account: AccountProfile,
    connected: bool,
    synced: BTreeSet<RoomId>,
    logs: BTreeMap<RoomId, RoomLog>,
    outbox: BTreeMap<ClientNonce, Outgoing>,
    error: Option<ApiError>,
    inbox: InboxTracker,
    thread_view: Option<ThreadView>,
    clock_ms: u64,
}

fn sync_frames(cursors: Vec<RoomCursor>) -> Vec<ClientFrame> {
    cursors
        .chunks(SYNC_MAX_ROOMS)
        .map(|chunk| {
            ClientFrame::SyncRooms(SyncRooms {
                cursors: chunk.to_vec(),
            })
        })
        .collect()
}

fn reaction_palette() -> Vec<String> {
    REACTION_PALETTE
        .iter()
        .map(|emoji| (*emoji).to_owned())
        .collect()
}

pub(crate) fn empty_timeline(error: Option<ApiError>) -> TimelineSnapshot {
    TimelineSnapshot {
        room_id: None,
        entries: Vec::new(),
        syncing: false,
        error,
        reaction_palette: reaction_palette(),
    }
}

fn reaction_error(reason: ReactionRejection) -> ApiError {
    match reason {
        ReactionRejection::Forbidden => ApiError::new(ApiErrorCode::Forbidden),
        ReactionRejection::InvalidEmoji => ApiError::invalid(InputField::ReactionEmoji),
        ReactionRejection::UnknownMessage => ApiError::new(ApiErrorCode::NotFound),
        ReactionRejection::TooManyKinds => ApiError::conflict(InputField::ReactionEmoji),
    }
}

fn thread_error(reason: ThreadRejection) -> ApiError {
    match reason {
        ThreadRejection::Forbidden => ApiError::new(ApiErrorCode::Forbidden),
        ThreadRejection::UnknownThread => ApiError::new(ApiErrorCode::NotFound),
        ThreadRejection::InvalidTopicTitle => ApiError::invalid(InputField::TopicTitle),
    }
}

impl TimelineMachine {
    pub fn new(campus_id: CampusId, account: AccountProfile) -> Self {
        Self {
            campus_id,
            account,
            connected: false,
            synced: BTreeSet::new(),
            logs: BTreeMap::new(),
            outbox: BTreeMap::new(),
            error: None,
            inbox: InboxTracker::default(),
            thread_view: None,
            clock_ms: 0,
        }
    }

    pub fn campus_id(&self) -> CampusId {
        self.campus_id
    }

    pub fn cursor(&self, room_id: RoomId) -> Option<MessageId> {
        self.logs.get(&room_id).and_then(|log| log.cursor)
    }

    pub fn next_deadline_ms(&self) -> Option<u64> {
        self.thread_view
            .as_ref()
            .and_then(ThreadView::follow_deadline_ms)
    }

    fn guarded(&self, frames: Vec<ClientFrame>) -> Vec<ClientFrame> {
        if self.connected { frames } else { Vec::new() }
    }

    fn begin_sync(&mut self, rooms: &[RoomId]) -> Vec<ClientFrame> {
        let cursors = rooms
            .iter()
            .map(|room_id| {
                let log = self.logs.entry(*room_id).or_default();
                log.syncing = true;
                log.cursor_request(*room_id)
            })
            .collect::<Vec<_>>();
        self.synced.extend(rooms.iter().copied());
        sync_frames(cursors)
    }

    fn resend_frames(&self) -> Vec<ClientFrame> {
        self.outbox
            .iter()
            .filter(|(_, outgoing)| outgoing.failed.is_none())
            .map(|(client_nonce, outgoing)| {
                ClientFrame::SendMessage(SendMessage {
                    room_id: outgoing.room_id,
                    client_nonce: *client_nonce,
                    body: outgoing.body.clone(),
                    thread_root_id: outgoing.thread_root_id,
                })
            })
            .collect()
    }

    fn gap_frames(&mut self, room_id: RoomId) -> Vec<ClientFrame> {
        let connected = self.connected;
        let Some(log) = self.logs.get_mut(&room_id) else {
            return Vec::new();
        };
        if !connected || log.syncing || !log.has_gap() {
            return Vec::new();
        }
        log.syncing = true;
        sync_frames(vec![log.cursor_request(room_id)])
    }

    fn store(&mut self, message: &RoomMessage, allow_new: bool) -> bool {
        let Some(log) = (if allow_new {
            Some(self.logs.entry(message.room_id).or_default())
        } else {
            self.logs.get_mut(&message.room_id)
        }) else {
            return false;
        };
        let first = match log.messages.get(&message.message_id) {
            Some(existing) => {
                if is_newer(message, existing) {
                    log.messages.insert(message.message_id, message.clone());
                }
                false
            }
            None if allow_new => {
                log.messages.insert(message.message_id, message.clone());
                true
            }
            None => false,
        };
        log.advance_chain();
        first
    }

    fn absorb(&mut self, message: &RoomMessage, allow_new: bool) -> bool {
        self.forget_own_outgoing(message);
        let first = self.store(message, allow_new);
        self.inbox.observe(message, self.account.account_id);
        first
    }

    fn thread_latest(&self, room_id: RoomId, root: MessageId) -> Option<MessageId> {
        let log = self.logs.get(&room_id);
        let known_root = self.best_root(room_id, root);
        let newest_reply =
            log.and_then(|log| log.replies_to(root).last().map(|reply| reply.message_id));
        [
            known_root.and_then(|message| message.latest_reply_id),
            newest_reply,
            known_root.map(|message| message.message_id),
        ]
        .into_iter()
        .flatten()
        .max()
    }

    fn best_root(&self, room_id: RoomId, root: MessageId) -> Option<&RoomMessage> {
        let logged = self
            .logs
            .get(&room_id)
            .and_then(|log| log.messages.get(&root));
        match (logged, self.inbox.root_copy(root)) {
            (Some(logged), Some(tracked)) => Some(if is_newer(tracked, logged) {
                tracked
            } else {
                logged
            }),
            (logged, tracked) => logged.or(tracked),
        }
    }

    fn mark_read_frame(&mut self, room_id: RoomId, root: MessageId) -> Option<ClientFrame> {
        if !self.connected || self.inbox.thread_state(root).is_none() {
            return None;
        }
        let latest = self.thread_latest(room_id, root)?;
        if !self.inbox.mark_read(root, latest) {
            return None;
        }
        Some(ClientFrame::MarkThreadRead(MarkThreadRead {
            room_id,
            root_message_id: root,
            read_through: latest,
        }))
    }

    fn ingest(&mut self, message: RoomMessage) -> Vec<ClientFrame> {
        if message.campus_id != self.campus_id {
            return Vec::new();
        }
        let room_id = message.room_id;
        let first = self.absorb(&message, true);
        let mut frames = Vec::new();
        let watching = message.thread_root_id.is_some_and(|root| {
            self.thread_view
                .as_ref()
                .is_some_and(|view| view.shows(room_id, root))
        });
        if first
            && watching
            && message.author_id != self.account.account_id
            && let Some(root) = message.thread_root_id
            && self.connected
            && self.inbox.thread_state(root).is_some()
            && self.inbox.mark_read(root, message.message_id)
        {
            frames.push(ClientFrame::MarkThreadRead(MarkThreadRead {
                room_id,
                root_message_id: root,
                read_through: message.message_id,
            }));
        }
        frames.extend(self.gap_frames(room_id));
        frames
    }

    fn receive_page(
        &mut self,
        room_id: RoomId,
        messages: Vec<RoomMessage>,
        updates: Vec<RoomMessage>,
        activity_seq: u64,
        has_more: bool,
    ) -> Vec<ClientFrame> {
        let campus_id = self.campus_id;
        let belongs =
            |message: &RoomMessage| message.campus_id == campus_id && message.room_id == room_id;
        let accepted = messages.into_iter().filter(belongs).collect::<Vec<_>>();
        let changed = updates.into_iter().filter(belongs).collect::<Vec<_>>();
        let last = accepted.iter().map(|message| message.message_id).max();
        let first_page = self
            .logs
            .get(&room_id)
            .is_none_or(|log| log.cursor.is_none());
        for message in &accepted {
            self.absorb(message, true);
        }
        for message in &changed {
            self.absorb(message, false);
        }
        let log = self.logs.entry(room_id).or_default();
        if first_page && log.window_start.is_none() {
            log.window_start = accepted.iter().map(|message| message.message_id).min();
        }
        if let Some(last) = last
            && log.cursor.is_none_or(|cursor| last > cursor)
        {
            log.cursor = Some(last);
        }
        if log.activity.is_none_or(|current| activity_seq > current) {
            log.activity = Some(activity_seq);
        }
        log.advance_chain();
        if has_more {
            return sync_frames(vec![log.cursor_request(room_id)]);
        }
        log.syncing = false;
        self.gap_frames(room_id)
    }

    fn receive_thread(&mut self, synced: ThreadSynced) -> Vec<ClientFrame> {
        let campus_id = self.campus_id;
        let room_id = synced.room_id;
        let root = synced.root_message_id;
        let accepted = synced
            .messages
            .into_iter()
            .filter(|message| message.campus_id == campus_id && message.room_id == room_id)
            .collect::<Vec<_>>();
        let last = accepted.iter().map(|message| message.message_id).max();
        for message in &accepted {
            self.absorb(message, true);
        }
        let mut frames = Vec::new();
        let showing = self
            .thread_view
            .as_ref()
            .is_some_and(|view| view.shows(room_id, root));
        if showing {
            match last.filter(|_| synced.has_more) {
                Some(last) => frames.push(ClientFrame::SyncThread(SyncThread {
                    room_id,
                    root_message_id: root,
                    after: Some(last),
                })),
                None => {
                    if let Some(view) = self.thread_view.as_mut() {
                        view.syncing = false;
                    }
                    frames.extend(self.mark_read_frame(room_id, root));
                }
            }
        }
        frames.extend(self.gap_frames(room_id));
        frames
    }

    fn forget_own_outgoing(&mut self, message: &RoomMessage) {
        if message.author_id == self.account.account_id {
            self.outbox.remove(&message.client_nonce);
        }
    }

    fn receive_frame(&mut self, frame: ServerFrame) -> Vec<ClientFrame> {
        match frame {
            ServerFrame::MessageAccepted(accepted) => self.ingest(accepted.message),
            ServerFrame::MessageCreated(created) => self.ingest(created.message),
            ServerFrame::MessageUpdated(updated) => self.ingest(updated.message),
            ServerFrame::RoomSynced(synced) => self.receive_page(
                synced.room_id,
                synced.messages,
                synced.updates,
                synced.activity_seq,
                synced.has_more,
            ),
            ServerFrame::RoomUnavailable(unavailable) => {
                if let Some(log) = self.logs.get_mut(&unavailable.room_id) {
                    log.syncing = false;
                }
                self.outbox
                    .values_mut()
                    .filter(|outgoing| {
                        outgoing.room_id == unavailable.room_id && outgoing.failed.is_none()
                    })
                    .for_each(|outgoing| outgoing.failed = Some(MessageRejection::UnknownRoom));
                Vec::new()
            }
            ServerFrame::MessageRejected(rejected) => {
                if let Some(outgoing) = self.outbox.get_mut(&rejected.client_nonce) {
                    outgoing.failed = Some(rejected.reason);
                }
                Vec::new()
            }
            ServerFrame::ReactionRejected(rejected) => {
                self.error = Some(reaction_error(rejected.reason));
                Vec::new()
            }
            ServerFrame::ThreadRejected(rejected) => {
                self.error = Some(thread_error(rejected.reason));
                self.inbox.reject(rejected.root_message_id);
                if let Some(view) = self.thread_view.as_mut()
                    && view.shows(rejected.room_id, rejected.root_message_id)
                {
                    view.syncing = false;
                }
                Vec::new()
            }
            ServerFrame::ThreadSynced(synced) => self.receive_thread(synced),
            ServerFrame::InboxSynced(synced) => {
                for summary in synced.threads {
                    if summary.root.campus_id == self.campus_id {
                        self.inbox.apply_summary(summary);
                    }
                }
                Vec::new()
            }
            ServerFrame::ThreadChanged(changed) => {
                if changed.summary.root.campus_id == self.campus_id {
                    self.inbox.apply_summary(changed.summary);
                }
                Vec::new()
            }
            ServerFrame::Welcome(_)
            | ServerFrame::Pong(_)
            | ServerFrame::VersionMismatch(_)
            | ServerFrame::HandshakeRejected(_) => Vec::new(),
        }
    }

    fn queue_message(
        &mut self,
        room_id: RoomId,
        client_nonce: ClientNonce,
        body: &str,
        thread_root_id: Option<MessageId>,
    ) -> Vec<ClientFrame> {
        match normalize_message_body(body) {
            Err(field) => {
                self.error = Some(ApiError::invalid(field));
                Vec::new()
            }
            Ok(body) => {
                self.error = None;
                let frame = ClientFrame::SendMessage(SendMessage {
                    room_id,
                    client_nonce,
                    body: body.clone(),
                    thread_root_id,
                });
                self.outbox.insert(
                    client_nonce,
                    Outgoing {
                        room_id,
                        body,
                        thread_root_id,
                        failed: None,
                    },
                );
                self.guarded(vec![frame])
            }
        }
    }

    fn follow_frame(&mut self) -> Option<ClientFrame> {
        let view = self.thread_view.as_mut()?;
        let (room_id, root) = (view.room_id, view.root_message_id);
        if self.inbox.thread_state(root).is_some() {
            return None;
        }
        self.inbox.request_state(root, SubscriptionState::Following);
        Some(ClientFrame::SetThreadSubscription(SetThreadSubscription {
            room_id,
            root_message_id: root,
            state: SubscriptionState::Following,
            origin: SubscriptionOrigin::Viewed,
        }))
    }

    fn advance_clock(&mut self, now_ms: u64) -> Vec<ClientFrame> {
        self.clock_ms = self.clock_ms.max(now_ms);
        let due = self
            .thread_view
            .as_ref()
            .is_some_and(|view| view.follow_due(self.clock_ms));
        if !due {
            return Vec::new();
        }
        if let Some(view) = self.thread_view.as_mut() {
            view.follow_sent = true;
        }
        if self.connected {
            self.follow_frame().into_iter().collect()
        } else {
            if let Some(view) = self.thread_view.as_mut() {
                view.pending_follow = true;
            }
            Vec::new()
        }
    }

    fn open_thread(&mut self, room_id: RoomId, root: MessageId, now_ms: u64) -> Vec<ClientFrame> {
        self.clock_ms = self.clock_ms.max(now_ms);
        self.thread_view = Some(ThreadView::new(room_id, root, now_ms));
        let mut frames = vec![ClientFrame::SyncThread(SyncThread {
            room_id,
            root_message_id: root,
            after: None,
        })];
        frames.extend(self.mark_read_frame(room_id, root));
        self.guarded(frames)
    }

    fn archive_selected(&mut self) -> Vec<ClientFrame> {
        let Some(item) = self.inbox.selected_item() else {
            return Vec::new();
        };
        if !self.connected {
            return Vec::new();
        }
        let (room_id, root) = (item.room_id, item.root_message_id);
        let latest = item.activity_message_id;
        self.inbox.request_state(root, SubscriptionState::Archived);
        self.inbox.mark_read(root, latest);
        vec![
            ClientFrame::SetThreadSubscription(SetThreadSubscription {
                room_id,
                root_message_id: root,
                state: SubscriptionState::Archived,
                origin: SubscriptionOrigin::Explicit,
            }),
            ClientFrame::MarkThreadRead(MarkThreadRead {
                room_id,
                root_message_id: root,
                read_through: latest,
            }),
        ]
    }

    fn apply_intent(&mut self, intent: ThreadIntent) -> Vec<ClientFrame> {
        match intent {
            ThreadIntent::ReplyRequested {
                room_id,
                root_message_id,
                client_nonce,
                body,
            } => self.queue_message(room_id, client_nonce, &body, Some(root_message_id)),
            ThreadIntent::ReactRequested {
                room_id,
                message_id,
                emoji,
                action,
            } => match normalize_reaction_emoji(&emoji) {
                Err(field) => {
                    self.error = Some(ApiError::invalid(field));
                    Vec::new()
                }
                Ok(emoji) => {
                    self.error = None;
                    self.guarded(vec![ClientFrame::React(React {
                        room_id,
                        message_id,
                        emoji,
                        action,
                    })])
                }
            },
            ThreadIntent::TopicTitleRequested {
                room_id,
                root_message_id,
                topic_title,
            } => match normalize_topic_title(&topic_title) {
                Err(field) => {
                    self.error = Some(ApiError::invalid(field));
                    Vec::new()
                }
                Ok(title) => {
                    self.error = None;
                    self.guarded(vec![ClientFrame::SetTopicTitle(SetTopicTitle {
                        room_id,
                        root_message_id,
                        topic_title: title.unwrap_or_default(),
                    })])
                }
            },
            ThreadIntent::SubscriptionRequested {
                room_id,
                root_message_id,
                state,
            } => {
                if !self.connected {
                    return Vec::new();
                }
                self.inbox.request_state(root_message_id, state);
                vec![ClientFrame::SetThreadSubscription(SetThreadSubscription {
                    room_id,
                    root_message_id,
                    state,
                    origin: SubscriptionOrigin::Explicit,
                })]
            }
            ThreadIntent::ThreadOpened {
                room_id,
                root_message_id,
                now_ms,
            } => self.open_thread(room_id, root_message_id, now_ms),
            ThreadIntent::ThreadClosed => {
                self.thread_view = None;
                Vec::new()
            }
            ThreadIntent::ClockAdvanced { now_ms } => self.advance_clock(now_ms),
            ThreadIntent::InboxToggled => {
                self.inbox.toggle();
                Vec::new()
            }
            ThreadIntent::InboxSelectionMoved(step) => {
                self.inbox.step_selection(step);
                Vec::new()
            }
            ThreadIntent::InboxSelectionArchived => self.archive_selected(),
            ThreadIntent::InboxReplyStarted => {
                self.inbox.start_reply();
                Vec::new()
            }
            ThreadIntent::InboxReplyCancelled => {
                self.inbox.cancel_reply();
                Vec::new()
            }
        }
    }

    fn on_connected(&mut self, rooms: Vec<RoomId>) -> Vec<ClientFrame> {
        self.connected = true;
        self.synced.clear();
        let mut frames = self.begin_sync(&rooms);
        frames.push(ClientFrame::SyncInbox(SyncInbox {}));
        frames.extend(self.resend_frames());
        let pending = self
            .thread_view
            .as_mut()
            .is_some_and(|view| core::mem::take(&mut view.pending_follow));
        if pending {
            frames.extend(self.follow_frame());
        }
        if let Some(view) = self.thread_view.as_ref().filter(|view| view.syncing) {
            frames.push(ClientFrame::SyncThread(SyncThread {
                room_id: view.room_id,
                root_message_id: view.root_message_id,
                after: None,
            }));
        }
        frames
    }

    pub fn step(mut self, event: TimelineEvent) -> (Self, Vec<ClientFrame>) {
        let frames = match event {
            TimelineEvent::Connected { rooms } => self.on_connected(rooms),
            TimelineEvent::RoomsChanged { rooms } => {
                if self.connected {
                    let fresh = rooms
                        .into_iter()
                        .filter(|room_id| !self.synced.contains(room_id))
                        .collect::<Vec<_>>();
                    self.begin_sync(&fresh)
                } else {
                    Vec::new()
                }
            }
            TimelineEvent::Disconnected => {
                self.connected = false;
                self.synced.clear();
                self.logs.values_mut().for_each(|log| log.syncing = false);
                self.inbox.clear_requests();
                Vec::new()
            }
            TimelineEvent::SendRequested {
                room_id,
                client_nonce,
                body,
            } => self.queue_message(room_id, client_nonce, &body, None),
            TimelineEvent::DiscardRequested { client_nonce } => {
                let failed = self
                    .outbox
                    .get(&client_nonce)
                    .is_some_and(|outgoing| outgoing.failed.is_some());
                if failed {
                    self.outbox.remove(&client_nonce);
                }
                Vec::new()
            }
            TimelineEvent::FrameReceived(frame) => self.receive_frame(frame),
            TimelineEvent::Thread(intent) => self.apply_intent(intent),
        };
        self.inbox.refresh();
        (self, frames)
    }

    fn reaction_summaries(&self, message: &RoomMessage) -> Vec<ReactionSummary> {
        message
            .reactions
            .iter()
            .map(|tally| ReactionSummary {
                emoji: tally.emoji.clone(),
                count: u32::try_from(tally.account_ids.len()).unwrap_or(u32::MAX),
                reacted_by_self: tally.account_ids.contains(&self.account.account_id),
            })
            .collect()
    }

    fn message_entry(&self, message: &RoomMessage, thread: Option<ThreadPreview>) -> TimelineEntry {
        TimelineEntry {
            client_nonce: message.client_nonce,
            message_id: Some(message.message_id),
            author_id: message.author_id,
            author_display_name: message.author_display_name.clone(),
            author_mention_token: mention_token(message.author_id),
            from_self: message.author_id == self.account.account_id,
            body: message.body.clone(),
            segments: body_segments(&message.body, &message.mentions),
            thread_root_id: message.thread_root_id,
            reactions: self.reaction_summaries(message),
            thread,
            delivery: DeliveryState::Confirmed,
            rejection: None,
        }
    }

    fn pending_entry(&self, client_nonce: ClientNonce, outgoing: &Outgoing) -> TimelineEntry {
        TimelineEntry {
            client_nonce,
            message_id: None,
            author_id: self.account.account_id,
            author_display_name: self.account.display_name.clone(),
            author_mention_token: mention_token(self.account.account_id),
            from_self: true,
            body: outgoing.body.clone(),
            segments: body_segments(&outgoing.body, &[]),
            thread_root_id: outgoing.thread_root_id,
            reactions: Vec::new(),
            thread: None,
            delivery: if outgoing.failed.is_some() {
                DeliveryState::Failed
            } else {
                DeliveryState::Pending
            },
            rejection: outgoing.failed,
        }
    }

    fn thread_preview(&self, log: &RoomLog, root: &RoomMessage) -> Option<ThreadPreview> {
        let replies = log.replies_to(root.message_id);
        let fresh_replies = replies
            .iter()
            .filter(|reply| {
                root.latest_reply_id
                    .is_none_or(|latest| reply.message_id > latest)
            })
            .count();
        let reply_count = root
            .reply_count
            .saturating_add(u32::try_from(fresh_replies).unwrap_or(u32::MAX));
        if reply_count == 0 && root.topic_title.is_none() {
            return None;
        }
        let latest = replies.last();
        let state = self.inbox.thread_state(root.message_id);
        Some(ThreadPreview {
            topic_title: root.topic_title.clone(),
            reply_count,
            latest_reply: latest.map(|reply| reply_preview(reply)),
            show_latest_reply: latest.is_some_and(|reply| {
                log.buried_depth_between(root.message_id, reply.message_id) >= THREAD_BURIED_DEPTH
            }),
            subscription: state.map(|(subscription, _)| subscription),
            unread_replies: state.map_or(0, |(_, unread)| unread),
        })
    }

    pub fn thread_snapshot(&self) -> ThreadSnapshot {
        let Some(view) = &self.thread_view else {
            return ThreadSnapshot::default();
        };
        let root = self.best_root(view.room_id, view.root_message_id);
        let confirmed = self
            .logs
            .get(&view.room_id)
            .into_iter()
            .flat_map(|log| log.replies_to(view.root_message_id))
            .map(|reply| self.message_entry(reply, None));
        let unconfirmed = self
            .outbox
            .iter()
            .filter(|(_, outgoing)| {
                outgoing.room_id == view.room_id
                    && outgoing.thread_root_id == Some(view.root_message_id)
            })
            .map(|(client_nonce, outgoing)| self.pending_entry(*client_nonce, outgoing));
        ThreadSnapshot {
            room_id: Some(view.room_id),
            root_message_id: Some(view.root_message_id),
            root: root.map(|message| self.message_entry(message, None)),
            replies: confirmed.chain(unconfirmed).collect(),
            topic_title: root.and_then(|message| message.topic_title.clone()),
            subscription: self
                .inbox
                .thread_state(view.root_message_id)
                .map(|(state, _)| state),
            syncing: view.syncing,
        }
    }

    pub fn inbox_snapshot(&self) -> InboxSnapshot {
        self.inbox.snapshot()
    }

    pub fn snapshot(&self, room_id: Option<RoomId>) -> TimelineSnapshot {
        let Some(room_id) = room_id else {
            return empty_timeline(self.error);
        };
        let confirmed = self.logs.get(&room_id).into_iter().flat_map(|log| {
            log.messages
                .values()
                .filter(|message| log.in_stream(message))
                .map(|message| self.message_entry(message, self.thread_preview(log, message)))
        });
        let unconfirmed = self
            .outbox
            .iter()
            .filter(|(_, outgoing)| {
                outgoing.room_id == room_id && outgoing.thread_root_id.is_none()
            })
            .map(|(client_nonce, outgoing)| self.pending_entry(*client_nonce, outgoing));
        TimelineSnapshot {
            room_id: Some(room_id),
            entries: confirmed.chain(unconfirmed).collect(),
            syncing: self.logs.get(&room_id).is_some_and(|log| log.syncing),
            error: self.error,
            reaction_palette: reaction_palette(),
        }
    }
}
