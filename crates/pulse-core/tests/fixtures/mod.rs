#![allow(dead_code)]

use std::collections::VecDeque;

use pulse_core::{
    ApiTransport, BackoffPolicy, ConnectionEvent, ConnectionMachine, NetworkTransport,
    SessionEvent, SessionMachine, TransportError,
};
use pulse_protocol::fixtures::auth_token;
pub use pulse_protocol::fixtures::{
    ACCOUNT_A, AUTH_TOKEN_A, AUTH_TOKEN_B, CAMPUS_A, CAMPUS_B, SESSION_A, VALID_PASSWORD,
};
use pulse_protocol::{
    AccountProfile, ApiError, AuthSession, AuthToken, CampusCreated, CampusId, CampusList,
    CampusSummary, ClientFrame, CreateCampusRequest, HandshakeRejected, HandshakeRejection, Hello,
    ProtocolVersion, Role, Room, RoomId, RoomList, ServerFrame, SignInRequest, SignUpRequest,
    Welcome,
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

pub fn connect() -> ConnectionEvent {
    ConnectionEvent::ConnectRequested {
        campus_id: CAMPUS_A,
        auth_token: token_a(),
    }
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

pub fn started(start: Start) -> ConnectionMachine {
    let events = match start {
        Start::Fresh => vec![],
        Start::Backoff => vec![connect(), ConnectionEvent::TransportFailed],
        Start::Connecting => vec![connect()],
        Start::Handshaking => vec![connect(), ConnectionEvent::TransportOpened],
        Start::Connected => vec![
            connect(),
            ConnectionEvent::TransportOpened,
            welcome(CAMPUS_A),
        ],
    };
    machine_after(events)
}
