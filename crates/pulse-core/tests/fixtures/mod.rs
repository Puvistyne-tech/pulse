#![allow(dead_code, unused_imports)]

use std::collections::VecDeque;

use futures::executor::block_on;
use pulse_core::{
    ApiTransport, BackoffPolicy, Client, ConnectionEvent, ConnectionMachine, NetworkTransport,
    SessionEvent, SessionMachine, ThreadIntent, TimelineEvent, TimelineMachine, TransportError,
};
use pulse_protocol::fixtures::auth_token;
pub use pulse_protocol::fixtures::{
    ACCOUNT_A, ACCOUNT_B, ACCOUNT_C, AUTH_TOKEN_A, AUTH_TOKEN_B, CAMPUS_A, CAMPUS_B, CAMPUS_C,
    EMOJI_A, EMOJI_B, NONCE_A, NONCE_B, NONCE_C, ROOM_A, ROOM_B, SESSION_A, VALID_PASSWORD,
};
use pulse_protocol::{
    AccountId, AccountProfile, ApiError, AuthSession, AuthToken, CampusCreated, CampusId,
    CampusList, CampusSummary, ClientFrame, ClientNonce, CreateCampusRequest, DeliveryState,
    HandshakeRejected, HandshakeRejection, Hello, InboxSnapshot, InboxStep, InboxSynced,
    MarkThreadRead, MentionRef, MessageAccepted, MessageCreated, MessageId, MessageRejected,
    MessageRejection, MessageUpdated, ProtocolVersion, React, ReactionAction, ReactionRejected,
    ReactionRejection, ReactionTally, Role, Room, RoomCursor, RoomId, RoomList, RoomMessage,
    RoomSynced, RoomUnavailable, SendMessage, ServerFrame, SessionSnapshot, SetThreadSubscription,
    SetTopicTitle, SignInRequest, SignUpRequest, SubscriptionOrigin, SubscriptionState, SyncInbox,
    SyncLounges, SyncRooms, SyncShelf, SyncStewards, SyncThread, ThreadChanged, ThreadRejected,
    ThreadRejection, ThreadSummary, ThreadSynced, TimelineSnapshot, Welcome,
};

pub fn token_a() -> AuthToken {
    auth_token(AUTH_TOKEN_A)
}

pub fn token_b() -> AuthToken {
    auth_token(AUTH_TOKEN_B)
}

pub fn welcome_for(campus_id: CampusId) -> Welcome {
    Welcome {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id,
        session_id: SESSION_A,
        account_id: ACCOUNT_A,
        role: Role::Member,
    }
}

pub fn connect_for(campus_id: CampusId) -> ConnectionEvent {
    ConnectionEvent::ConnectRequested {
        campus_id,
        auth_token: token_a(),
    }
}

pub fn connect() -> ConnectionEvent {
    connect_for(CAMPUS_A)
}

pub fn welcome(campus_id: CampusId) -> ConnectionEvent {
    ConnectionEvent::WelcomeReceived(welcome_for(campus_id))
}

pub fn rejected(reason: HandshakeRejection) -> HandshakeRejected {
    HandshakeRejected { reason }
}

pub fn fresh_machine() -> ConnectionMachine {
    ConnectionMachine::new(BackoffPolicy::default())
}

pub fn machine_after(events: Vec<ConnectionEvent>) -> ConnectionMachine {
    events
        .into_iter()
        .fold(fresh_machine(), |machine, event| machine.step(event).0)
}

pub fn hello_frame(campus_id: CampusId, token: AuthToken) -> ClientFrame {
    ClientFrame::Hello(Hello {
        protocol_version: ProtocolVersion::CURRENT,
        campus_id,
        auth_token: token,
    })
}

pub fn welcome_frame(campus_id: CampusId) -> Result<ServerFrame, TransportError> {
    Ok(ServerFrame::Welcome(welcome_for(campus_id)))
}

pub fn refused() -> Result<(), TransportError> {
    Err(TransportError::Failure("refused".into()))
}

#[derive(Debug, Default)]
pub struct ScriptedTransport {
    pub open_results: VecDeque<Result<(), TransportError>>,
    pub incoming: VecDeque<Result<ServerFrame, TransportError>>,
    pub sent: Vec<ClientFrame>,
    pub closes: u32,
}

impl ScriptedTransport {
    pub fn scripted(
        open_results: Vec<Result<(), TransportError>>,
        incoming: Vec<Result<ServerFrame, TransportError>>,
    ) -> Self {
        Self {
            open_results: open_results.into(),
            incoming: incoming.into(),
            ..Self::default()
        }
    }
}

impl NetworkTransport for ScriptedTransport {
    async fn open(&mut self) -> Result<(), TransportError> {
        self.open_results.pop_front().unwrap_or(Ok(()))
    }

    async fn send(&mut self, frame: ClientFrame) -> Result<(), TransportError> {
        self.sent.push(frame);
        Ok(())
    }

    async fn receive(&mut self) -> Result<ServerFrame, TransportError> {
        self.incoming
            .pop_front()
            .unwrap_or(Err(TransportError::Closed))
    }

    async fn close(&mut self) {
        self.closes += 1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordedCall {
    SignUp(SignUpRequest),
    SignIn(SignInRequest),
    SignOut(AuthToken),
    ListCampuses(AuthToken),
    CreateCampus(AuthToken, CreateCampusRequest),
    ListRooms(AuthToken, CampusId),
}

#[derive(Debug, Default)]
pub struct ScriptedApi {
    pub sign_ups: VecDeque<Result<AuthSession, ApiError>>,
    pub sign_ins: VecDeque<Result<AuthSession, ApiError>>,
    pub sign_outs: VecDeque<Result<(), ApiError>>,
    pub campus_lists: VecDeque<Result<CampusList, ApiError>>,
    pub campus_creations: VecDeque<Result<CampusCreated, ApiError>>,
    pub room_lists: VecDeque<Result<RoomList, ApiError>>,
    pub calls: Vec<RecordedCall>,
}

fn unavailable() -> ApiError {
    ApiError::new(pulse_protocol::ApiErrorCode::Unavailable)
}

impl ApiTransport for ScriptedApi {
    async fn sign_up(&mut self, request: SignUpRequest) -> Result<AuthSession, ApiError> {
        self.calls.push(RecordedCall::SignUp(request));
        self.sign_ups.pop_front().unwrap_or(Err(unavailable()))
    }

    async fn sign_in(&mut self, request: SignInRequest) -> Result<AuthSession, ApiError> {
        self.calls.push(RecordedCall::SignIn(request));
        self.sign_ins.pop_front().unwrap_or(Err(unavailable()))
    }

    async fn sign_out(&mut self, token: AuthToken) -> Result<(), ApiError> {
        self.calls.push(RecordedCall::SignOut(token));
        self.sign_outs.pop_front().unwrap_or(Ok(()))
    }

    async fn list_campuses(&mut self, token: AuthToken) -> Result<CampusList, ApiError> {
        self.calls.push(RecordedCall::ListCampuses(token));
        self.campus_lists.pop_front().unwrap_or(Err(unavailable()))
    }

    async fn create_campus(
        &mut self,
        token: AuthToken,
        request: CreateCampusRequest,
    ) -> Result<CampusCreated, ApiError> {
        self.calls.push(RecordedCall::CreateCampus(token, request));
        self.campus_creations
            .pop_front()
            .unwrap_or(Err(unavailable()))
    }

    async fn list_rooms(
        &mut self,
        token: AuthToken,
        campus_id: CampusId,
    ) -> Result<RoomList, ApiError> {
        self.calls.push(RecordedCall::ListRooms(token, campus_id));
        self.room_lists.pop_front().unwrap_or(Err(unavailable()))
    }
}

pub fn profile() -> AccountProfile {
    AccountProfile {
        account_id: ACCOUNT_A,
        email: "ada@example.org".to_owned(),
        display_name: "Ada".to_owned(),
    }
}

pub fn auth_session() -> AuthSession {
    AuthSession {
        account: profile(),
        token: token_a(),
    }
}

pub fn campus_summary(campus_id: CampusId, slug: &str, role: Role) -> CampusSummary {
    CampusSummary {
        campus_id,
        slug: slug.to_owned(),
        display_name: slug.to_owned(),
        role,
    }
}

pub fn room(campus_id: CampusId, slug: &str) -> Room {
    Room {
        room_id: RoomId::generate(),
        campus_id,
        slug: slug.to_owned(),
    }
}

pub fn campus_list(campuses: Vec<CampusSummary>) -> CampusList {
    CampusList { campuses }
}

pub fn room_list(campus_id: CampusId, slugs: &[&str]) -> RoomList {
    RoomList {
        campus_id,
        rooms: slugs.iter().map(|slug| room(campus_id, slug)).collect(),
    }
}

pub fn two_campuses() -> CampusList {
    campus_list(vec![
        campus_summary(CAMPUS_A, "alpha", Role::Owner),
        campus_summary(CAMPUS_B, "bravo", Role::Member),
    ])
}

pub fn sign_in_request() -> SignInRequest {
    SignInRequest {
        email: "ada@example.org".to_owned(),
        password: VALID_PASSWORD.to_owned(),
    }
}

pub fn sign_up_request() -> SignUpRequest {
    SignUpRequest {
        email: "ada@example.org".to_owned(),
        display_name: "Ada".to_owned(),
        password: VALID_PASSWORD.to_owned(),
    }
}

pub fn session_after(events: Vec<SessionEvent>) -> SessionMachine {
    events
        .into_iter()
        .fold(SessionMachine::new(), |machine, event| {
            machine.step(event).0
        })
}

pub fn signed_in_machine() -> SessionMachine {
    session_after(vec![
        SessionEvent::SignInRequested(sign_in_request()),
        SessionEvent::Authenticated(auth_session()),
        SessionEvent::CampusesLoaded(campus_list(vec![
            campus_summary(CAMPUS_A, "alpha", Role::Owner),
            campus_summary(CAMPUS_B, "bravo", Role::Member),
        ])),
        SessionEvent::RoomsLoaded(room_list(CAMPUS_A, &["commons", "courtyard"])),
    ])
}

pub fn invalid_sign_in_request() -> SignInRequest {
    SignInRequest {
        email: "not-an-email".to_owned(),
        password: VALID_PASSWORD.to_owned(),
    }
}

pub fn short_password_sign_up_request() -> SignUpRequest {
    SignUpRequest {
        password: "short".to_owned(),
        ..sign_up_request()
    }
}

pub fn invalid_create_campus_request() -> CreateCampusRequest {
    CreateCampusRequest {
        slug: "-".to_owned(),
        display_name: "Gamma".to_owned(),
    }
}

pub fn create_campus_request() -> CreateCampusRequest {
    CreateCampusRequest {
        slug: "gamma".to_owned(),
        display_name: "Gamma".to_owned(),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Start {
    Fresh,
    Backoff,
    Connecting,
    Handshaking,
    Connected,
}

pub fn start_events(start: Start) -> Vec<ConnectionEvent> {
    match start {
        Start::Fresh => vec![],
        Start::Backoff => vec![connect(), ConnectionEvent::TransportFailed],
        Start::Connecting => vec![connect()],
        Start::Handshaking => vec![connect(), ConnectionEvent::TransportOpened],
        Start::Connected => vec![
            connect(),
            ConnectionEvent::TransportOpened,
            welcome(CAMPUS_A),
        ],
    }
}

pub fn started(start: Start) -> ConnectionMachine {
    machine_after(start_events(start))
}

pub fn signed_in_api() -> ScriptedApi {
    ScriptedApi {
        sign_ins: VecDeque::from([Ok(auth_session())]),
        campus_lists: VecDeque::from([Ok(two_campuses())]),
        room_lists: VecDeque::from([Ok(room_list(CAMPUS_A, &["commons", "courtyard"]))]),
        ..ScriptedApi::default()
    }
}

pub fn signed_in_client() -> (Client, ScriptedApi) {
    let mut api = signed_in_api();
    let mut client = Client::default();
    block_on(client.apply(&mut api, SessionEvent::SignInRequested(sign_in_request())));
    (client, api)
}

pub fn connected_client() -> Client {
    let (mut client, _) = signed_in_client();
    start_events(Start::Connected)
        .into_iter()
        .for_each(|event| {
            client.connection_step(event);
        });
    client
}

pub fn client_rooms(client: &Client) -> Vec<RoomId> {
    client
        .snapshot()
        .session
        .rooms
        .iter()
        .map(|room| room.room_id)
        .collect()
}

#[path = "builders.rs"]
mod builders;
pub use builders::{display_name, indexed_id, message_id, nonce, room_message};

pub fn message_ids(indexes: &[u128]) -> Vec<MessageId> {
    indexes.iter().copied().map(message_id).collect()
}

pub fn sorted_message_ids(indexes: &[u128]) -> Vec<MessageId> {
    let mut ids = message_ids(indexes);
    ids.sort();
    ids
}

pub fn foreign(message: RoomMessage) -> RoomMessage {
    RoomMessage {
        campus_id: CAMPUS_B,
        ..message
    }
}

fn previous_in_chain(index: u128) -> Option<MessageId> {
    index
        .checked_sub(1)
        .filter(|previous| *previous > 0)
        .map(message_id)
}

pub fn chain_message(room_id: RoomId, index: u128) -> RoomMessage {
    room_message(
        room_id,
        message_id(index),
        previous_in_chain(index),
        ACCOUNT_B,
        nonce(index),
        &format!("peer {index}"),
    )
}

pub fn chain(room_id: RoomId, count: u128) -> Vec<RoomMessage> {
    (1..=count)
        .map(|index| chain_message(room_id, index))
        .collect()
}

pub fn own_message(room_id: RoomId, index: u128, client_nonce: ClientNonce) -> RoomMessage {
    room_message(
        room_id,
        message_id(index),
        previous_in_chain(index),
        ACCOUNT_A,
        client_nonce,
        "hi",
    )
}

pub fn accepted_frame(message: RoomMessage) -> ServerFrame {
    ServerFrame::MessageAccepted(MessageAccepted { message })
}

pub fn created_frame(message: RoomMessage) -> ServerFrame {
    ServerFrame::MessageCreated(MessageCreated { message })
}

pub fn synced_frame(room_id: RoomId, messages: Vec<RoomMessage>, has_more: bool) -> ServerFrame {
    synced_frame_with(room_id, messages, vec![], 0, has_more)
}

pub fn synced_frame_with(
    room_id: RoomId,
    messages: Vec<RoomMessage>,
    updates: Vec<RoomMessage>,
    activity_seq: u64,
    has_more: bool,
) -> ServerFrame {
    ServerFrame::RoomSynced(RoomSynced {
        room_id,
        messages,
        updates,
        activity_seq,
        has_more,
    })
}

pub fn rejected_frame(client_nonce: ClientNonce, reason: MessageRejection) -> ServerFrame {
    ServerFrame::MessageRejected(MessageRejected {
        client_nonce,
        reason,
    })
}

pub fn unavailable_frame(room_id: RoomId) -> ServerFrame {
    ServerFrame::RoomUnavailable(RoomUnavailable { room_id })
}

pub fn sync_frame(cursors: &[(RoomId, Option<MessageId>)]) -> ClientFrame {
    sync_frame_at(
        &cursors
            .iter()
            .map(|(room_id, after)| (*room_id, *after, None))
            .collect::<Vec<_>>(),
    )
}

pub fn sync_frame_at(cursors: &[(RoomId, Option<MessageId>, Option<u64>)]) -> ClientFrame {
    ClientFrame::SyncRooms(SyncRooms {
        cursors: cursors
            .iter()
            .map(|(room_id, after, activity_after)| RoomCursor {
                room_id: *room_id,
                after: *after,
                activity_after: *activity_after,
            })
            .collect(),
    })
}

pub fn send_frame(room_id: RoomId, client_nonce: ClientNonce, body: &str) -> ClientFrame {
    ClientFrame::SendMessage(SendMessage {
        room_id,
        client_nonce,
        body: body.to_owned(),
        thread_root_id: None,
        attachment_ids: Vec::new(),
    })
}

pub fn received(frame: ServerFrame) -> TimelineEvent {
    TimelineEvent::FrameReceived(frame)
}

pub fn timeline_connected(rooms: &[RoomId]) -> TimelineEvent {
    TimelineEvent::Connected {
        rooms: rooms.to_vec(),
    }
}

pub fn send_requested(room_id: RoomId, client_nonce: ClientNonce, body: &str) -> TimelineEvent {
    TimelineEvent::SendRequested {
        room_id,
        client_nonce,
        body: body.to_owned(),
    }
}

pub fn timeline_run(events: Vec<TimelineEvent>) -> (TimelineMachine, Vec<ClientFrame>) {
    events.into_iter().fold(
        (TimelineMachine::new(CAMPUS_A, profile()), Vec::new()),
        |(machine, mut frames), event| {
            let (machine, emitted) = machine.step(event);
            frames.extend(emitted);
            (machine, frames)
        },
    )
}

pub fn timeline_after(events: Vec<TimelineEvent>) -> TimelineMachine {
    timeline_run(events).0
}

pub type EntryKey = (
    Option<MessageId>,
    ClientNonce,
    DeliveryState,
    Option<MessageRejection>,
);

pub fn entry_keys(snapshot: &TimelineSnapshot) -> Vec<EntryKey> {
    snapshot
        .entries
        .iter()
        .map(|entry| {
            (
                entry.message_id,
                entry.client_nonce,
                entry.delivery,
                entry.rejection,
            )
        })
        .collect()
}

pub fn confirmed_key(message: &RoomMessage) -> EntryKey {
    (
        Some(message.message_id),
        message.client_nonce,
        DeliveryState::Confirmed,
        None,
    )
}

pub fn pending_key(client_nonce: ClientNonce) -> EntryKey {
    (None, client_nonce, DeliveryState::Pending, None)
}

pub fn failed_key(client_nonce: ClientNonce, reason: MessageRejection) -> EntryKey {
    (None, client_nonce, DeliveryState::Failed, Some(reason))
}

pub fn active_room_slug(snapshot: &SessionSnapshot) -> Option<&str> {
    snapshot
        .rooms
        .iter()
        .find(|room| Some(room.room_id) == snapshot.active_room_id)
        .map(|room| room.slug.as_str())
}

pub fn reply_message(
    room_id: RoomId,
    index: u128,
    root_index: u128,
    author_id: AccountId,
) -> RoomMessage {
    RoomMessage {
        thread_root_id: Some(message_id(root_index)),
        body: format!("reply {index}"),
        author_display_name: display_name(author_id).to_owned(),
        author_id,
        ..chain_message(room_id, index)
    }
}

pub fn peer_reply(index: u128, root_index: u128) -> RoomMessage {
    reply_message(ROOM_A, index, root_index, ACCOUNT_B)
}

pub fn own_reply(index: u128, root_index: u128) -> RoomMessage {
    reply_message(ROOM_A, index, root_index, ACCOUNT_A)
}

pub fn mentioning(message: RoomMessage, accounts: &[AccountId]) -> RoomMessage {
    RoomMessage {
        mentions: accounts
            .iter()
            .map(|account_id| MentionRef {
                account_id: *account_id,
                display_name: display_name(*account_id).to_owned(),
            })
            .collect(),
        body: accounts
            .iter()
            .map(|account_id| pulse_protocol::mention_token(*account_id))
            .collect::<Vec<_>>()
            .join(" "),
        ..message
    }
}

pub fn reacted(
    message: RoomMessage,
    reactions: &[(&str, &[AccountId])],
    activity_seq: u64,
) -> RoomMessage {
    RoomMessage {
        reactions: reactions
            .iter()
            .map(|(emoji, accounts)| ReactionTally {
                emoji: (*emoji).to_owned(),
                account_ids: accounts.to_vec(),
            })
            .collect(),
        activity_seq,
        ..message
    }
}

pub fn titled(message: RoomMessage, title: &str, activity_seq: u64) -> RoomMessage {
    RoomMessage {
        topic_title: Some(title.to_owned()),
        activity_seq,
        ..message
    }
}

pub fn with_replies(message: RoomMessage, reply_count: u32, latest: Option<u128>) -> RoomMessage {
    RoomMessage {
        reply_count,
        latest_reply_id: latest.map(message_id),
        ..message
    }
}

#[derive(Debug, Clone)]
pub struct SummarySpec {
    pub root: u128,
    pub latest: Option<u128>,
    pub replies: u32,
    pub unread_replies: u32,
    pub unread_mentions: u32,
    pub state: SubscriptionState,
    pub read_through: Option<u128>,
    pub revision: u64,
}

impl SummarySpec {
    pub fn new(root: u128) -> Self {
        Self {
            root,
            latest: None,
            replies: 0,
            unread_replies: 0,
            unread_mentions: 0,
            state: SubscriptionState::Following,
            read_through: None,
            revision: 1,
        }
    }

    pub fn replied(root: u128, latest: u128, unread_replies: u32) -> Self {
        Self {
            latest: Some(latest),
            replies: unread_replies,
            unread_replies,
            ..Self::new(root)
        }
    }
}

pub fn summary(spec: &SummarySpec) -> ThreadSummary {
    ThreadSummary {
        root: with_replies(chain_message(ROOM_A, spec.root), spec.replies, spec.latest),
        latest_reply: spec.latest.map(|latest| peer_reply(latest, spec.root)),
        unread_replies: spec.unread_replies,
        unread_mentions: spec.unread_mentions,
        subscription: spec.state,
        read_through: spec.read_through.map(message_id),
        revision: spec.revision,
    }
}

pub fn inbox_synced_frame(specs: &[SummarySpec]) -> ServerFrame {
    ServerFrame::InboxSynced(InboxSynced {
        threads: specs.iter().map(summary).collect(),
    })
}

pub fn thread_changed_frame(spec: &SummarySpec) -> ServerFrame {
    ServerFrame::ThreadChanged(ThreadChanged {
        summary: summary(spec),
    })
}

pub fn thread_synced_frame(
    root_index: u128,
    messages: Vec<RoomMessage>,
    has_more: bool,
) -> ServerFrame {
    ServerFrame::ThreadSynced(ThreadSynced {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        messages,
        has_more,
    })
}

pub fn updated_frame(message: RoomMessage) -> ServerFrame {
    ServerFrame::MessageUpdated(MessageUpdated { message })
}

pub fn reaction_rejected_frame(message_index: u128, reason: ReactionRejection) -> ServerFrame {
    ServerFrame::ReactionRejected(ReactionRejected {
        room_id: ROOM_A,
        message_id: message_id(message_index),
        emoji: EMOJI_A.to_owned(),
        reason,
    })
}

pub fn thread_rejected_frame(root_index: u128, reason: ThreadRejection) -> ServerFrame {
    ServerFrame::ThreadRejected(ThreadRejected {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        reason,
    })
}

pub fn sync_inbox_frame() -> ClientFrame {
    ClientFrame::SyncInbox(SyncInbox {})
}

pub fn sync_shelf_frame() -> ClientFrame {
    ClientFrame::SyncShelf(SyncShelf {})
}

pub fn sync_stewards_frame() -> ClientFrame {
    ClientFrame::SyncStewards(SyncStewards {})
}

pub fn sync_lounges_frame() -> ClientFrame {
    ClientFrame::SyncLounges(SyncLounges {})
}

pub fn connect_burst(rooms: &[RoomId]) -> Vec<ClientFrame> {
    connect_burst_at(
        &rooms
            .iter()
            .map(|room_id| (*room_id, None, None))
            .collect::<Vec<_>>(),
    )
}

pub fn connect_burst_at(cursors: &[(RoomId, Option<MessageId>, Option<u64>)]) -> Vec<ClientFrame> {
    let rooms = (!cursors.is_empty()).then(|| sync_frame_at(cursors));
    rooms
        .into_iter()
        .chain([
            sync_inbox_frame(),
            sync_shelf_frame(),
            sync_stewards_frame(),
            sync_lounges_frame(),
        ])
        .collect()
}

pub fn sync_thread_frame(root_index: u128, after: Option<u128>) -> ClientFrame {
    ClientFrame::SyncThread(SyncThread {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        after: after.map(message_id),
    })
}

pub fn subscription_frame(
    root_index: u128,
    state: SubscriptionState,
    origin: SubscriptionOrigin,
) -> ClientFrame {
    ClientFrame::SetThreadSubscription(SetThreadSubscription {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        state,
        origin,
    })
}

pub fn mark_read_frame(root_index: u128, read_through: u128) -> ClientFrame {
    ClientFrame::MarkThreadRead(MarkThreadRead {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        read_through: message_id(read_through),
    })
}

pub fn react_frame(message_index: u128, emoji: &str, action: ReactionAction) -> ClientFrame {
    ClientFrame::React(React {
        room_id: ROOM_A,
        message_id: message_id(message_index),
        emoji: emoji.to_owned(),
        action,
    })
}

pub fn topic_title_frame(root_index: u128, title: &str) -> ClientFrame {
    ClientFrame::SetTopicTitle(SetTopicTitle {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        topic_title: title.to_owned(),
    })
}

pub fn reply_frame(root_index: u128, client_nonce: ClientNonce, body: &str) -> ClientFrame {
    ClientFrame::SendMessage(SendMessage {
        room_id: ROOM_A,
        client_nonce,
        body: body.to_owned(),
        thread_root_id: Some(message_id(root_index)),
        attachment_ids: Vec::new(),
    })
}

pub fn intent(intent: ThreadIntent) -> TimelineEvent {
    TimelineEvent::Thread(intent)
}

pub fn open_thread(root_index: u128, now_ms: u64) -> TimelineEvent {
    intent(ThreadIntent::ThreadOpened {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        now_ms,
    })
}

pub fn close_thread() -> TimelineEvent {
    intent(ThreadIntent::ThreadClosed)
}

pub fn clock(now_ms: u64) -> TimelineEvent {
    intent(ThreadIntent::ClockAdvanced { now_ms })
}

pub fn subscribe(root_index: u128, state: SubscriptionState) -> TimelineEvent {
    intent(ThreadIntent::SubscriptionRequested {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        state,
    })
}

pub fn react_intent(message_index: u128, emoji: &str, action: ReactionAction) -> TimelineEvent {
    intent(ThreadIntent::ReactRequested {
        room_id: ROOM_A,
        message_id: message_id(message_index),
        emoji: emoji.to_owned(),
        action,
    })
}

pub fn reply_intent(root_index: u128, client_nonce: ClientNonce, body: &str) -> TimelineEvent {
    intent(ThreadIntent::ReplyRequested {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        client_nonce,
        body: body.to_owned(),
    })
}

pub fn title_intent(root_index: u128, title: &str) -> TimelineEvent {
    intent(ThreadIntent::TopicTitleRequested {
        room_id: ROOM_A,
        root_message_id: message_id(root_index),
        topic_title: title.to_owned(),
    })
}

pub fn toggle_inbox() -> TimelineEvent {
    intent(ThreadIntent::InboxToggled)
}

pub fn move_selection(step: InboxStep) -> TimelineEvent {
    intent(ThreadIntent::InboxSelectionMoved(step))
}

pub fn archive_selection() -> TimelineEvent {
    intent(ThreadIntent::InboxSelectionArchived)
}

pub fn start_inbox_reply() -> TimelineEvent {
    intent(ThreadIntent::InboxReplyStarted)
}

pub fn cancel_inbox_reply() -> TimelineEvent {
    intent(ThreadIntent::InboxReplyCancelled)
}

pub fn thread_roots(snapshot: &InboxSnapshot) -> Vec<MessageId> {
    snapshot
        .items
        .iter()
        .map(|item| item.root_message_id)
        .collect()
}

pub fn settled_connection() -> Vec<TimelineEvent> {
    vec![
        timeline_connected(&[ROOM_A]),
        received(synced_frame(ROOM_A, vec![], false)),
    ]
}

pub fn settled_machine(events: Vec<TimelineEvent>) -> TimelineMachine {
    let mut all = settled_connection();
    all.extend(events);
    timeline_after(all)
}

pub fn step_all(
    machine: TimelineMachine,
    events: Vec<TimelineEvent>,
) -> (TimelineMachine, Vec<ClientFrame>) {
    events
        .into_iter()
        .fold((machine, Vec::new()), |(machine, mut frames), event| {
            let (machine, emitted) = machine.step(event);
            frames.extend(emitted);
            (machine, frames)
        })
}

pub fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    (0..items.len())
        .flat_map(|index| {
            let mut rest = items.to_vec();
            let head = rest.remove(index);
            permutations(&rest)
                .into_iter()
                .map(move |mut tail| {
                    tail.insert(0, head.clone());
                    tail
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn attachment_id(index: u128) -> pulse_protocol::AttachmentId {
    indexed_id(0x3000_0000, index)
        .parse()
        .expect("attachment id")
}

pub fn draft_id(index: u128) -> pulse_protocol::DraftId {
    indexed_id(0x4000_0000, index).parse().expect("draft id")
}

pub fn shelf_item_id(index: u128) -> pulse_protocol::ShelfItemId {
    indexed_id(0x5000_0000, index)
        .parse()
        .expect("shelf item id")
}

pub fn attachment_of(
    index: u128,
    file_name: &str,
    kind: pulse_protocol::AttachmentKind,
) -> pulse_protocol::Attachment {
    let content_type = match kind {
        pulse_protocol::AttachmentKind::Image => "image/png",
        pulse_protocol::AttachmentKind::Video => "video/mp4",
        pulse_protocol::AttachmentKind::Audio => "audio/mpeg",
        pulse_protocol::AttachmentKind::File => "application/pdf",
    };
    pulse_protocol::Attachment {
        attachment_id: attachment_id(index),
        file_name: file_name.to_owned(),
        content_type: content_type.to_owned(),
        kind,
        byte_size: 1024,
    }
}

pub fn with_attachments(
    mut message: RoomMessage,
    attachments: Vec<pulse_protocol::Attachment>,
) -> RoomMessage {
    message.attachments = attachments;
    message
}

pub fn pinned(mut message: RoomMessage, pinned_at_ms: u64) -> RoomMessage {
    message.pin = Some(pulse_protocol::PinMark {
        pinned_by: ACCOUNT_B,
        pinned_at_ms,
    });
    message
}

#[derive(Debug, Clone, Copy)]
pub struct ShelfSpec {
    pub message: u128,
    pub remind_at_ms: Option<u64>,
    pub reminded_at_ms: Option<u64>,
    pub done_at_ms: Option<u64>,
    pub saved_at_ms: u64,
    pub revision: u64,
}

impl ShelfSpec {
    pub fn new(message: u128) -> Self {
        Self {
            message,
            remind_at_ms: None,
            reminded_at_ms: None,
            done_at_ms: None,
            saved_at_ms: 100,
            revision: 1,
        }
    }

    pub fn remind(self, remind_at_ms: u64) -> Self {
        Self {
            remind_at_ms: Some(remind_at_ms),
            ..self
        }
    }

    pub fn reminded(self, reminded_at_ms: u64) -> Self {
        Self {
            reminded_at_ms: Some(reminded_at_ms),
            ..self
        }
    }

    pub fn done(self, done_at_ms: u64) -> Self {
        Self {
            done_at_ms: Some(done_at_ms),
            ..self
        }
    }

    pub fn saved(self, saved_at_ms: u64) -> Self {
        Self {
            saved_at_ms,
            ..self
        }
    }

    pub fn revision(self, revision: u64) -> Self {
        Self { revision, ..self }
    }
}

pub fn shelf_item(spec: ShelfSpec) -> pulse_protocol::ShelfItem {
    pulse_protocol::ShelfItem {
        shelf_item_id: shelf_item_id(spec.message),
        message: chain_message(ROOM_A, spec.message),
        remind_at_ms: spec.remind_at_ms,
        reminded_at_ms: spec.reminded_at_ms,
        done_at_ms: spec.done_at_ms,
        saved_at_ms: spec.saved_at_ms,
        revision: spec.revision,
    }
}

pub fn shelf_synced_frame(specs: &[ShelfSpec]) -> ServerFrame {
    ServerFrame::ShelfSynced(pulse_protocol::ShelfSynced {
        items: specs.iter().copied().map(shelf_item).collect(),
    })
}

pub fn shelf_changed_frame(spec: ShelfSpec, removed: bool) -> ServerFrame {
    ServerFrame::ShelfChanged(pulse_protocol::ShelfChanged {
        item: shelf_item(spec),
        removed,
    })
}

pub fn shelf_rejected_frame(
    message_index: u128,
    action: pulse_protocol::ShelfAction,
    reason: pulse_protocol::ShelfRejection,
) -> ServerFrame {
    ServerFrame::ShelfRejected(pulse_protocol::ShelfRejected {
        room_id: ROOM_A,
        message_id: message_id(message_index),
        action,
        reason,
    })
}

pub fn vault_item(
    message: RoomMessage,
    attachment: Option<pulse_protocol::Attachment>,
    link: Option<&str>,
) -> pulse_protocol::VaultItem {
    pulse_protocol::VaultItem {
        message,
        attachment,
        link: link.map(str::to_owned),
    }
}

pub fn vault_synced_frame(
    room_id: RoomId,
    tab: pulse_protocol::VaultTab,
    items: Vec<pulse_protocol::VaultItem>,
    next_before: Option<u128>,
) -> ServerFrame {
    ServerFrame::VaultSynced(pulse_protocol::VaultSynced {
        room_id,
        tab,
        items,
        next_before: next_before.map(message_id),
    })
}

pub fn media(intent: pulse_core::MediaIntent) -> TimelineEvent {
    TimelineEvent::Media(intent)
}

pub fn search_event(intent: pulse_core::SearchIntent) -> TimelineEvent {
    TimelineEvent::Search(intent)
}

pub fn shelf_event(intent: pulse_core::ShelfIntent) -> TimelineEvent {
    TimelineEvent::Shelf(intent)
}

pub fn shelf_update(
    message_index: u128,
    action: pulse_protocol::ShelfAction,
    preset: Option<pulse_protocol::ReminderPreset>,
    remind_at_ms: Option<u64>,
) -> TimelineEvent {
    shelf_event(pulse_core::ShelfIntent::UpdateRequested {
        room_id: ROOM_A,
        message_id: message_id(message_index),
        action,
        preset,
        remind_at_ms,
    })
}

pub fn attach_draft(index: u128, file_name: &str, byte_size: u64) -> TimelineEvent {
    media(pulse_core::MediaIntent::DraftAttached {
        draft_id: draft_id(index),
        room_id: ROOM_A,
        file_name: file_name.to_owned(),
        byte_size,
    })
}

pub fn upload_finished(
    draft_index: u128,
    result: Result<pulse_protocol::AttachmentId, ApiError>,
) -> TimelineEvent {
    TimelineEvent::EffectFinished(pulse_core::EffectOutcome::UploadFinished {
        draft_id: draft_id(draft_index),
        result,
    })
}

pub fn unavailable_error() -> ApiError {
    ApiError::new(pulse_protocol::ApiErrorCode::Unavailable)
}

pub fn welcome_as(role: Role) -> ConnectionEvent {
    ConnectionEvent::WelcomeReceived(Welcome {
        role,
        ..welcome_for(CAMPUS_A)
    })
}

pub fn connected_as(role: Role) -> Client {
    let (mut client, _) = signed_in_client();
    [
        connect(),
        ConnectionEvent::TransportOpened,
        welcome_as(role),
    ]
    .into_iter()
    .for_each(|event| {
        client.connection_step(event);
    });
    client
}

pub fn settled_client() -> Client {
    let mut client = connected_client();
    let rooms = client_rooms(&client);
    rooms.iter().for_each(|room_id| {
        client.receive_frame(synced_frame(*room_id, vec![], false));
    });
    client.drain_outbound();
    client.take_effects();
    client
}

pub use pulse_protocol::fixtures::{LOUNGE_A, LOUNGE_B, STEWARD_A, STREAM_A, STREAM_B};
use pulse_protocol::{
    AgentStreamChunk, Lounge, LoungeChanged, LoungeId, LoungeJoined, LoungeParticipant,
    LoungeRejected, LoungeRejection, LoungeScope, LoungeTicket, LoungesSynced, ManageSteward,
    StewardAction, StewardChanged, StewardIssued, StewardProfile, StewardRejected,
    StewardRejection, StewardsSynced, StreamAbortReason, StreamId, StreamPhase,
};

pub const STEWARD_NAME: &str = "Scribe";

pub fn stream_chunk(
    stream_id: StreamId,
    phase: StreamPhase,
    sequence: u32,
    text: &str,
) -> AgentStreamChunk {
    AgentStreamChunk {
        campus_id: CAMPUS_A,
        stream_id,
        room_id: ROOM_A,
        thread_root_id: None,
        steward_id: STEWARD_A,
        steward_display_name: STEWARD_NAME.to_owned(),
        phase,
        sequence,
        text: text.to_owned(),
        message_id: None,
        abort_reason: None,
    }
}

pub fn chunk_event(chunk: AgentStreamChunk) -> TimelineEvent {
    received(ServerFrame::AgentStreamChunk(chunk))
}

pub fn stream_event(
    stream_id: StreamId,
    phase: StreamPhase,
    sequence: u32,
    text: &str,
) -> TimelineEvent {
    chunk_event(stream_chunk(stream_id, phase, sequence, text))
}

pub fn abort_event(stream_id: StreamId, sequence: u32, reason: StreamAbortReason) -> TimelineEvent {
    chunk_event(AgentStreamChunk {
        abort_reason: Some(reason),
        ..stream_chunk(stream_id, StreamPhase::Abort, sequence, "")
    })
}

pub fn steward_message(
    room_id: RoomId,
    index: u128,
    stream_id: StreamId,
    body: &str,
) -> RoomMessage {
    room_message(
        room_id,
        message_id(index),
        None,
        STEWARD_A,
        stream_id.client_nonce(),
        body,
    )
}

pub fn lounge_with(
    lounge_id: LoungeId,
    revision: u64,
    participants: &[(AccountId, u64)],
) -> Lounge {
    Lounge {
        lounge_id,
        campus_id: CAMPUS_A,
        scope: if lounge_id == LOUNGE_A {
            LoungeScope::Campus
        } else {
            LoungeScope::Room
        },
        room_id: (lounge_id != LOUNGE_A).then_some(ROOM_A),
        participants: participants
            .iter()
            .map(|(account_id, present_until_ms)| LoungeParticipant {
                account_id: *account_id,
                display_name: display_name(*account_id).to_owned(),
                can_speak: true,
                present_until_ms: *present_until_ms,
            })
            .collect(),
        revision,
    }
}

pub fn lounge_ticket(lounge_id: LoungeId) -> LoungeTicket {
    LoungeTicket {
        lounge_id,
        livekit_url: "ws://127.0.0.1:7880".to_owned(),
        room_name: format!("pulse-{CAMPUS_A}-{lounge_id}"),
        token: "ticket-token".to_owned(),
        expires_at_ms: 600_000,
    }
}

pub fn lounges_synced_frame(lounges: Vec<Lounge>) -> ServerFrame {
    ServerFrame::LoungesSynced(LoungesSynced { lounges })
}

pub fn lounge_changed_frame(lounge: Lounge) -> ServerFrame {
    ServerFrame::LoungeChanged(LoungeChanged { lounge })
}

pub fn lounge_joined_frame(lounge: Lounge) -> ServerFrame {
    let ticket = lounge_ticket(lounge.lounge_id);
    ServerFrame::LoungeJoined(LoungeJoined { lounge, ticket })
}

pub fn lounge_rejected_frame(lounge_id: LoungeId, reason: LoungeRejection) -> ServerFrame {
    ServerFrame::LoungeRejected(LoungeRejected { lounge_id, reason })
}

pub fn steward_profile(
    steward_id: AccountId,
    name: &str,
    room_ids: &[RoomId],
    revoked: bool,
) -> StewardProfile {
    StewardProfile {
        steward_id,
        display_name: name.to_owned(),
        room_ids: room_ids.to_vec(),
        revoked,
        credential_revision: 1,
    }
}

pub fn stewards_synced_frame(stewards: Vec<StewardProfile>) -> ServerFrame {
    ServerFrame::StewardsSynced(StewardsSynced { stewards })
}

pub fn steward_changed_frame(steward: StewardProfile) -> ServerFrame {
    ServerFrame::StewardChanged(StewardChanged { steward })
}

pub fn steward_issued_frame(request_nonce: ClientNonce, steward: StewardProfile) -> ServerFrame {
    ServerFrame::StewardIssued(StewardIssued {
        request_nonce,
        steward,
        token: auth_token("pst_issued-token"),
    })
}

pub fn steward_rejected_frame(
    request_nonce: ClientNonce,
    action: StewardAction,
    reason: StewardRejection,
) -> ServerFrame {
    ServerFrame::StewardRejected(StewardRejected {
        request_nonce,
        action,
        reason,
    })
}

pub fn manage_frame(
    request_nonce: ClientNonce,
    action: StewardAction,
    steward_id: Option<AccountId>,
    display_name: Option<&str>,
    room_ids: &[RoomId],
) -> ClientFrame {
    ClientFrame::ManageSteward(ManageSteward {
        request_nonce,
        action,
        steward_id,
        display_name: display_name.map(str::to_owned),
        room_ids: room_ids.to_vec(),
    })
}
