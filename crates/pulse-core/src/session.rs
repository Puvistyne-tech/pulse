use pulse_protocol::{
    AccountProfile, ApiError, ApiErrorCode, AuthPhase, AuthSession, AuthToken, CampusCreated,
    CampusId, CampusList, CampusSummary, CreateCampusRequest, Room, RoomList, SessionSnapshot,
    SignInRequest, SignUpRequest, Validate,
};

use crate::api::ApiTransport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionEvent {
    SignUpRequested(SignUpRequest),
    SignInRequested(SignInRequest),
    Authenticated(AuthSession),
    CampusesRequested,
    CampusesLoaded(CampusList),
    CreateCampusRequested(CreateCampusRequest),
    CampusCreated(CampusCreated),
    CampusSelected(CampusId),
    RoomsLoaded(RoomList),
    RequestFailed(ApiError),
    SignOutRequested,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionCommand {
    SignUp(SignUpRequest),
    SignIn(SignInRequest),
    SignOut(AuthToken),
    LoadCampuses(AuthToken),
    CreateCampus {
        token: AuthToken,
        request: CreateCampusRequest,
    },
    LoadRooms {
        token: AuthToken,
        campus_id: CampusId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionMachine {
    auth: AuthPhase,
    token: Option<AuthToken>,
    account: Option<AccountProfile>,
    campuses: Vec<CampusSummary>,
    active_campus_id: Option<CampusId>,
    rooms: Vec<Room>,
    busy: bool,
    error: Option<ApiError>,
}

impl Default for SessionMachine {
    fn default() -> Self {
        Self {
            auth: AuthPhase::SignedOut,
            token: None,
            account: None,
            campuses: Vec::new(),
            active_campus_id: None,
            rooms: Vec::new(),
            busy: false,
            error: None,
        }
    }
}

impl SessionMachine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token(&self) -> Option<&AuthToken> {
        self.token.as_ref()
    }

    pub fn snapshot(&self) -> SessionSnapshot {
        SessionSnapshot {
            auth: self.auth,
            account: self.account.clone(),
            campuses: self.campuses.clone(),
            active_campus_id: self.active_campus_id,
            rooms: self.rooms.clone(),
            busy: self.busy,
            error: self.error,
        }
    }

    fn begin_authentication(
        mut self,
        validation: Result<(), ApiError>,
        command: SessionCommand,
    ) -> (Self, Option<SessionCommand>) {
        match validation {
            Ok(()) => {
                self.auth = AuthPhase::Authenticating;
                self.busy = true;
                self.error = None;
                (self, Some(command))
            }
            Err(error) => {
                self.error = Some(error);
                (self, None)
            }
        }
    }

    fn load_rooms_for(&mut self, campus_id: CampusId) -> Option<SessionCommand> {
        let token = self.token.clone()?;
        self.active_campus_id = Some(campus_id);
        self.rooms.clear();
        self.busy = true;
        Some(SessionCommand::LoadRooms { token, campus_id })
    }

    fn with_token(
        mut self,
        build: impl FnOnce(AuthToken) -> SessionCommand,
    ) -> (Self, Option<SessionCommand>) {
        self.error = None;
        let command = self.token.clone().map(build);
        self.busy = command.is_some();
        (self, command)
    }

    fn signed_in_step(mut self, event: SessionEvent) -> (Self, Option<SessionCommand>) {
        match event {
            SessionEvent::CampusesRequested => self.with_token(SessionCommand::LoadCampuses),
            SessionEvent::CampusesLoaded(list) => {
                self.campuses = list.campuses;
                let active_is_listed = self.active_campus_id.is_some_and(|active| {
                    self.campuses
                        .iter()
                        .any(|campus| campus.campus_id == active)
                });
                if !active_is_listed {
                    self.active_campus_id = None;
                    self.rooms.clear();
                }
                let first = self.campuses.first().map(|campus| campus.campus_id);
                let command = match (self.active_campus_id, first) {
                    (None, Some(campus_id)) => self.load_rooms_for(campus_id),
                    _ => None,
                };
                if command.is_none() {
                    self.busy = false;
                }
                (self, command)
            }
            SessionEvent::CreateCampusRequested(request) => match request.validate() {
                Ok(()) => self.with_token(|token| SessionCommand::CreateCampus { token, request }),
                Err(error) => {
                    self.error = Some(error);
                    (self, None)
                }
            },
            SessionEvent::CampusCreated(created) => {
                if !self
                    .campuses
                    .iter()
                    .any(|campus| campus.campus_id == created.campus.campus_id)
                {
                    self.campuses.push(created.campus.clone());
                }
                self.active_campus_id = Some(created.campus.campus_id);
                self.rooms = created.rooms;
                self.busy = false;
                (self, None)
            }
            SessionEvent::CampusSelected(campus_id) => {
                let listed = self
                    .campuses
                    .iter()
                    .any(|campus| campus.campus_id == campus_id);
                if !listed {
                    return (self, None);
                }
                self.error = None;
                let command = self.load_rooms_for(campus_id);
                (self, command)
            }
            SessionEvent::RoomsLoaded(list) => {
                if Some(list.campus_id) == self.active_campus_id {
                    self.rooms = list.rooms;
                    self.busy = false;
                }
                (self, None)
            }
            SessionEvent::RequestFailed(error) => {
                if error.code == ApiErrorCode::Unauthenticated {
                    let mut reset = Self::new();
                    reset.error = Some(error);
                    (reset, None)
                } else {
                    self.busy = false;
                    self.error = Some(error);
                    (self, None)
                }
            }
            _ => (self, None),
        }
    }

    pub fn step(mut self, event: SessionEvent) -> (Self, Option<SessionCommand>) {
        if matches!(event, SessionEvent::SignOutRequested) {
            let command = match self.auth {
                AuthPhase::SignedIn => self.token.take().map(SessionCommand::SignOut),
                AuthPhase::Authenticating | AuthPhase::SignedOut => None,
            };
            return match self.auth {
                AuthPhase::SignedOut => (self, None),
                _ => (Self::new(), command),
            };
        }
        match (self.auth, event) {
            (AuthPhase::SignedOut, SessionEvent::SignUpRequested(request)) => {
                let validation = request.validate();
                self.begin_authentication(validation, SessionCommand::SignUp(request))
            }
            (AuthPhase::SignedOut, SessionEvent::SignInRequested(request)) => {
                let validation = request.validate();
                self.begin_authentication(validation, SessionCommand::SignIn(request))
            }
            (AuthPhase::Authenticating, SessionEvent::Authenticated(session)) => {
                self.auth = AuthPhase::SignedIn;
                self.account = Some(session.account);
                self.busy = true;
                let command = SessionCommand::LoadCampuses(session.token.clone());
                self.token = Some(session.token);
                (self, Some(command))
            }
            (AuthPhase::Authenticating, SessionEvent::RequestFailed(error)) => {
                self.auth = AuthPhase::SignedOut;
                self.busy = false;
                self.error = Some(error);
                (self, None)
            }
            (AuthPhase::SignedIn, event) => self.signed_in_step(event),
            _ => (self, None),
        }
    }
}

pub async fn drive_session<A: ApiTransport>(
    machine: SessionMachine,
    api: &mut A,
    event: SessionEvent,
) -> SessionMachine {
    let (mut machine, mut command) = machine.step(event);
    while let Some(current) = command.take() {
        let next_event = match current {
            SessionCommand::SignOut(token) => {
                let _ = api.sign_out(token).await;
                break;
            }
            SessionCommand::SignUp(request) => api
                .sign_up(request)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::Authenticated),
            SessionCommand::SignIn(request) => api
                .sign_in(request)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::Authenticated),
            SessionCommand::LoadCampuses(token) => api
                .list_campuses(token)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::CampusesLoaded),
            SessionCommand::CreateCampus { token, request } => api
                .create_campus(token, request)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::CampusCreated),
            SessionCommand::LoadRooms { token, campus_id } => api
                .list_rooms(token, campus_id)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::RoomsLoaded),
        };
        let (next, next_command) = machine.step(next_event);
        machine = next;
        command = next_command;
    }
    machine
}
