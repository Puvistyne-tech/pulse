use pulse_protocol::{
    ApiError, ApiErrorCode, AuthToken, CampusId, ClientFrame, ClientNonce, ClientSnapshot,
    ConnectionPhase, DraftId, InboxSnapshot, LoungeSnapshot, MessageId, Permission, RoomId,
    SearchSnapshot, ServerFrame, ShelfSnapshot, StewardSnapshot, ThreadSnapshot, VaultSnapshot,
    role_permits,
};

use crate::api::ApiTransport;
use crate::backoff::BackoffPolicy;
use crate::connection::{ConnectionCommand, ConnectionEvent, ConnectionMachine};
use crate::effects::{Effect, EffectOutcome};
use crate::media::MediaIntent;
use crate::search_view::SearchIntent;
use crate::session::{SessionEvent, SessionMachine, drive_session};
use crate::shelf::ShelfIntent;
use crate::timeline::{ThreadIntent, TimelineEvent, TimelineMachine, empty_timeline};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkTarget {
    pub campus_id: CampusId,
    pub auth_token: AuthToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    connection: ConnectionMachine,
    session: SessionMachine,
    timeline: Option<TimelineMachine>,
    outbound: Vec<ClientFrame>,
    sequence: u64,
    generation_floor: u64,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(BackoffPolicy::default())
    }
}

impl Client {
    pub fn new(policy: BackoffPolicy) -> Self {
        Self {
            connection: ConnectionMachine::new(policy),
            session: SessionMachine::new(),
            timeline: None,
            outbound: Vec::new(),
            sequence: 0,
            generation_floor: 0,
        }
    }

    pub fn snapshot(&self) -> ClientSnapshot {
        let active_room_id = self.session.snapshot().active_room_id;
        let mut timeline = self.timeline.as_ref().map_or_else(
            || empty_timeline(None),
            |machine| machine.snapshot(active_room_id),
        );
        let role = self.connection.role();
        timeline.can_pin = role.is_some_and(|role| role_permits(role, Permission::PinMessage));
        timeline.can_upload =
            role.is_some_and(|role| role_permits(role, Permission::UploadAttachment));
        let thread = self
            .timeline
            .as_ref()
            .map_or_else(ThreadSnapshot::default, TimelineMachine::thread_snapshot);
        let inbox = self
            .timeline
            .as_ref()
            .map_or_else(InboxSnapshot::default, TimelineMachine::inbox_snapshot);
        ClientSnapshot {
            sequence: self.sequence,
            network: self.connection.snapshot(),
            session: self.session.snapshot(),
            timeline,
            thread,
            inbox,
            search: self
                .timeline
                .as_ref()
                .map_or_else(SearchSnapshot::default, TimelineMachine::search_snapshot),
            vault: self
                .timeline
                .as_ref()
                .map_or_else(VaultSnapshot::default, TimelineMachine::vault_snapshot),
            shelf: self
                .timeline
                .as_ref()
                .map_or_else(ShelfSnapshot::default, TimelineMachine::shelf_snapshot),
            lounges: LoungeSnapshot::default(),
            stewards: StewardSnapshot::default(),
        }
    }

    pub async fn apply<A: ApiTransport>(
        &mut self,
        api: &mut A,
        event: SessionEvent,
    ) -> ClientSnapshot {
        self.sequence += 1;
        let rooms_before = self.room_ids();
        let machine = core::mem::take(&mut self.session);
        self.session = drive_session(machine, api, event).await;
        self.reconcile(rooms_before);
        self.snapshot()
    }

    pub fn link_target(&self) -> Option<LinkTarget> {
        let campus_id = self.session.active_campus_id()?;
        let auth_token = self.session.token()?.clone();
        Some(LinkTarget {
            campus_id,
            auth_token,
        })
    }

    pub fn connection_step(&mut self, event: ConnectionEvent) -> Option<ConnectionCommand> {
        self.sequence += 1;
        self.step_connection(event)
    }

    pub fn receive_frame(&mut self, frame: ServerFrame) -> Option<ConnectionCommand> {
        self.sequence += 1;
        match frame {
            ServerFrame::Welcome(welcome) => {
                self.step_connection(ConnectionEvent::WelcomeReceived(welcome))
            }
            ServerFrame::VersionMismatch(mismatch) => {
                self.step_connection(ConnectionEvent::VersionRejected(mismatch))
            }
            ServerFrame::HandshakeRejected(rejected) => {
                self.step_connection(ConnectionEvent::HandshakeRejected(rejected))
            }
            ServerFrame::Pong(_) => None,
            frame => {
                self.feed_timeline(TimelineEvent::FrameReceived(frame));
                None
            }
        }
    }

    pub fn send_message(
        &mut self,
        room_id: RoomId,
        client_nonce: ClientNonce,
        body: String,
    ) -> ClientSnapshot {
        self.sequence += 1;
        self.feed_timeline(TimelineEvent::SendRequested {
            room_id,
            client_nonce,
            body,
        });
        self.snapshot()
    }

    pub fn discard_message(&mut self, client_nonce: ClientNonce) -> ClientSnapshot {
        self.sequence += 1;
        self.feed_timeline(TimelineEvent::DiscardRequested { client_nonce });
        self.snapshot()
    }

    pub fn thread_intent(&mut self, intent: ThreadIntent) -> ClientSnapshot {
        self.sequence += 1;
        self.feed_timeline(TimelineEvent::Thread(intent));
        self.snapshot()
    }

    pub fn search_intent(&mut self, intent: SearchIntent) -> ClientSnapshot {
        self.sequence += 1;
        self.feed_timeline(TimelineEvent::Search(intent));
        self.snapshot()
    }

    pub fn media_intent(&mut self, intent: MediaIntent) -> ClientSnapshot {
        self.sequence += 1;
        self.feed_timeline(TimelineEvent::Media(intent));
        self.snapshot()
    }

    pub fn shelf_intent(&mut self, intent: ShelfIntent) -> ClientSnapshot {
        self.sequence += 1;
        self.feed_timeline(TimelineEvent::Shelf(intent));
        self.snapshot()
    }

    pub async fn open_search_hit<A: ApiTransport>(
        &mut self,
        api: &mut A,
        message_id: MessageId,
        now_ms: u64,
    ) -> ClientSnapshot {
        let hit = self
            .timeline
            .as_ref()
            .and_then(|machine| machine.search_hit(message_id));
        let Some((room_id, thread_root_id)) = hit else {
            return self.snapshot();
        };
        self.apply(api, SessionEvent::RoomSelected(room_id)).await;
        self.sequence += 1;
        if let Some(root_message_id) = thread_root_id {
            self.feed_timeline(TimelineEvent::Thread(ThreadIntent::ThreadOpened {
                room_id,
                root_message_id,
                now_ms,
            }));
        }
        self.feed_timeline(TimelineEvent::FocusRequested { message_id });
        self.snapshot()
    }

    pub fn selected_search_hit(&self) -> Option<MessageId> {
        self.timeline
            .as_ref()
            .and_then(TimelineMachine::selected_search_hit)
    }

    pub fn live_draft_ids(&self) -> Vec<DraftId> {
        self.timeline
            .as_ref()
            .map_or_else(Vec::new, TimelineMachine::live_draft_ids)
    }

    pub fn take_effects(&mut self) -> Vec<Effect> {
        self.timeline
            .as_mut()
            .map_or_else(Vec::new, TimelineMachine::take_effects)
    }

    pub fn effect_finished(&mut self, outcome: EffectOutcome) -> ClientSnapshot {
        self.sequence += 1;
        self.feed_timeline(TimelineEvent::EffectFinished(outcome));
        self.snapshot()
    }

    pub fn next_deadline_ms(&self) -> Option<u64> {
        self.timeline
            .as_ref()
            .and_then(TimelineMachine::next_deadline_ms)
    }

    pub fn advance_clock(&mut self, now_ms: u64) {
        self.sequence += 1;
        self.feed_timeline(TimelineEvent::Thread(ThreadIntent::ClockAdvanced {
            now_ms,
        }));
    }

    pub fn drain_outbound(&mut self) -> Vec<ClientFrame> {
        self.sequence += 1;
        core::mem::take(&mut self.outbound)
    }

    fn room_ids(&self) -> Vec<RoomId> {
        self.session
            .snapshot()
            .rooms
            .iter()
            .map(|room| room.room_id)
            .collect()
    }

    fn feed_timeline(&mut self, event: TimelineEvent) {
        let Some(machine) = self.timeline.take() else {
            return;
        };
        let (machine, frames) = machine.step(event);
        self.timeline = Some(machine);
        if matches!(self.connection.phase(), ConnectionPhase::Connected) {
            self.outbound.extend(frames);
        }
    }

    fn step_connection(&mut self, event: ConnectionEvent) -> Option<ConnectionCommand> {
        let was_connected = matches!(self.connection.phase(), ConnectionPhase::Connected);
        let machine = core::mem::replace(
            &mut self.connection,
            ConnectionMachine::new(BackoffPolicy::default()),
        );
        let (machine, command) = machine.step(event);
        self.connection = machine;
        let is_connected = matches!(self.connection.phase(), ConnectionPhase::Connected);
        match (was_connected, is_connected) {
            (false, true) => {
                let rooms = self.room_ids();
                self.feed_timeline(TimelineEvent::Connected { rooms });
            }
            (true, false) => self.feed_timeline(TimelineEvent::Disconnected),
            _ => {}
        }
        command
    }

    fn reset_connection(&mut self) {
        self.step_connection(ConnectionEvent::DisconnectRequested);
        self.outbound.clear();
    }

    fn replace_timeline(&mut self, next: Option<TimelineMachine>) {
        if let Some(previous) = self.timeline.take() {
            self.generation_floor = self.generation_floor.max(previous.generation());
        }
        self.timeline = next.map(|mut machine| {
            machine.seed_generation(self.generation_floor);
            machine
        });
    }

    fn reconcile(&mut self, rooms_before: Vec<RoomId>) {
        let account = self.session.account().cloned();
        let campus_id = self.session.active_campus_id();
        let Some((account, campus_id)) = account.zip(campus_id) else {
            self.reset_connection();
            self.replace_timeline(None);
            return;
        };
        let current = self.timeline.as_ref().map(TimelineMachine::campus_id);
        if current != Some(campus_id) {
            self.reset_connection();
            self.replace_timeline(Some(TimelineMachine::new(campus_id, account)));
            return;
        }
        let rooms = self.room_ids();
        if rooms != rooms_before {
            self.feed_timeline(TimelineEvent::RoomsChanged { rooms });
        }
    }
}

pub fn initial_snapshot() -> ClientSnapshot {
    Client::default().snapshot()
}

pub fn unavailable_snapshot() -> ClientSnapshot {
    let mut snapshot = initial_snapshot();
    snapshot.session.error = Some(ApiError::new(ApiErrorCode::Unavailable));
    snapshot
}
