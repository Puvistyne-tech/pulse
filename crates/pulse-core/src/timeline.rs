use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;

use pulse_protocol::{
    AccountProfile, ApiError, CampusId, ClientFrame, ClientNonce, DeliveryState, MessageId,
    MessageRejection, RoomCursor, RoomId, RoomMessage, SYNC_MAX_ROOMS, SendMessage, ServerFrame,
    SyncRooms, TimelineEntry, TimelineSnapshot, normalize_message_body,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Outgoing {
    room_id: RoomId,
    body: String,
    failed: Option<MessageRejection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct RoomLog {
    messages: BTreeMap<MessageId, RoomMessage>,
    cursor: Option<MessageId>,
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
        }
    }

    pub fn campus_id(&self) -> CampusId {
        self.campus_id
    }

    pub fn cursor(&self, room_id: RoomId) -> Option<MessageId> {
        self.logs.get(&room_id).and_then(|log| log.cursor)
    }

    fn begin_sync(&mut self, rooms: &[RoomId]) -> Vec<ClientFrame> {
        let cursors = rooms
            .iter()
            .map(|room_id| {
                let log = self.logs.entry(*room_id).or_default();
                log.syncing = true;
                RoomCursor {
                    room_id: *room_id,
                    after: log.cursor,
                }
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
        sync_frames(vec![RoomCursor {
            room_id,
            after: log.cursor,
        }])
    }

    fn ingest(&mut self, message: RoomMessage) -> Vec<ClientFrame> {
        if message.campus_id != self.campus_id {
            return Vec::new();
        }
        let room_id = message.room_id;
        self.forget_own_outgoing(&message);
        let log = self.logs.entry(room_id).or_default();
        log.messages.entry(message.message_id).or_insert(message);
        log.advance_chain();
        self.gap_frames(room_id)
    }

    fn receive_page(
        &mut self,
        room_id: RoomId,
        messages: Vec<RoomMessage>,
        has_more: bool,
    ) -> Vec<ClientFrame> {
        let campus_id = self.campus_id;
        let accepted = messages
            .into_iter()
            .filter(|message| message.campus_id == campus_id && message.room_id == room_id)
            .collect::<Vec<_>>();
        let last = accepted.iter().map(|message| message.message_id).max();
        for message in &accepted {
            self.forget_own_outgoing(message);
        }
        let log = self.logs.entry(room_id).or_default();
        for message in accepted {
            log.messages.entry(message.message_id).or_insert(message);
        }
        if let Some(last) = last
            && log.cursor.is_none_or(|cursor| last > cursor)
        {
            log.cursor = Some(last);
        }
        log.advance_chain();
        if has_more {
            return sync_frames(vec![RoomCursor {
                room_id,
                after: log.cursor,
            }]);
        }
        log.syncing = false;
        self.gap_frames(room_id)
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
            ServerFrame::RoomSynced(synced) => {
                self.receive_page(synced.room_id, synced.messages, synced.has_more)
            }
            ServerFrame::RoomUnavailable(unavailable) => {
                if let Some(log) = self.logs.get_mut(&unavailable.room_id) {
                    log.syncing = false;
                }
                Vec::new()
            }
            ServerFrame::MessageRejected(rejected) => {
                if let Some(outgoing) = self.outbox.get_mut(&rejected.client_nonce) {
                    outgoing.failed = Some(rejected.reason);
                }
                Vec::new()
            }
            ServerFrame::Welcome(_)
            | ServerFrame::Pong(_)
            | ServerFrame::VersionMismatch(_)
            | ServerFrame::HandshakeRejected(_) => Vec::new(),
        }
    }

    pub fn step(mut self, event: TimelineEvent) -> (Self, Vec<ClientFrame>) {
        let frames = match event {
            TimelineEvent::Connected { rooms } => {
                self.connected = true;
                self.synced.clear();
                let mut frames = self.begin_sync(&rooms);
                frames.extend(self.resend_frames());
                frames
            }
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
                Vec::new()
            }
            TimelineEvent::SendRequested {
                room_id,
                client_nonce,
                body,
            } => match normalize_message_body(&body) {
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
                    });
                    self.outbox.insert(
                        client_nonce,
                        Outgoing {
                            room_id,
                            body,
                            failed: None,
                        },
                    );
                    if self.connected {
                        vec![frame]
                    } else {
                        Vec::new()
                    }
                }
            },
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
        };
        (self, frames)
    }

    pub fn snapshot(&self, room_id: Option<RoomId>) -> TimelineSnapshot {
        let Some(room_id) = room_id else {
            return TimelineSnapshot {
                room_id: None,
                entries: Vec::new(),
                syncing: false,
                error: self.error,
            };
        };
        let confirmed = self.logs.get(&room_id).into_iter().flat_map(|log| {
            log.messages.values().map(|message| TimelineEntry {
                client_nonce: message.client_nonce,
                message_id: Some(message.message_id),
                author_id: message.author_id,
                author_display_name: message.author_display_name.clone(),
                from_self: message.author_id == self.account.account_id,
                body: message.body.clone(),
                delivery: DeliveryState::Confirmed,
                rejection: None,
            })
        });
        let unconfirmed = self
            .outbox
            .iter()
            .filter(|(_, outgoing)| outgoing.room_id == room_id)
            .map(|(client_nonce, outgoing)| TimelineEntry {
                client_nonce: *client_nonce,
                message_id: None,
                author_id: self.account.account_id,
                author_display_name: self.account.display_name.clone(),
                from_self: true,
                body: outgoing.body.clone(),
                delivery: if outgoing.failed.is_some() {
                    DeliveryState::Failed
                } else {
                    DeliveryState::Pending
                },
                rejection: outgoing.failed,
            });
        TimelineSnapshot {
            room_id: Some(room_id),
            entries: confirmed.chain(unconfirmed).collect(),
            syncing: self.logs.get(&room_id).is_some_and(|log| log.syncing),
            error: self.error,
        }
    }
}
