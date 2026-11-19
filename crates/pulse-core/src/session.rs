use pulse_protocol::{
    AccountId, AccountProfile, AddMemberRequest, ApiError, ApiErrorCode, AuthPhase, AuthSession,
    AuthToken, CampusCreated, CampusId, CampusInvite, CampusList, CampusMember, CampusSummary,
    CreateCampusRequest, DEV_SEED_PASSWORD, DEV_SEED_PEOPLE, InputField, JoinCampusRequest,
    MemberEntry, MemberList, Note, NoteEntry, NoteList, Permission, Room, RoomId, RoomList,
    SessionSnapshot, SignInRequest, SignUpRequest, StartNoteRequest, UpdateMemberRequest, Validate,
    assignable_roles, dev_accounts, member_manageable, normalize_note_members, note_title,
    role_permits,
};

use crate::api::ApiTransport;

const NOTE_PAIR_SIZE: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionEvent {
    SignUpRequested(SignUpRequest),
    SignInRequested(SignInRequest),
    Authenticated(AuthSession),
    CampusesRequested,
    CampusesLoaded(CampusList),
    CreateCampusRequested(CreateCampusRequest),
    CampusCreated(CampusCreated),
    JoinCampusRequested(JoinCampusRequest),
    InviteRequested,
    InviteCreated(CampusInvite),
    MembersRequested,
    MembersLoaded(MemberList),
    AddMemberRequested(AddMemberRequest),
    UpdateMemberRequested {
        account_id: AccountId,
        request: UpdateMemberRequest,
    },
    RemoveMemberRequested(AccountId),
    MemberSaved(CampusMember),
    MemberRemoved {
        campus_id: CampusId,
        account_id: AccountId,
    },
    CampusSelected(CampusId),
    RoomsLoaded(RoomList),
    RoomSelected(RoomId),
    RequestFailed(ApiError),
    SignOutRequested,
    QuickSignInRequested(String),
    NotesLoaded(NoteList),
    StartNoteRequested(StartNoteRequest),
    NoteOpened(Note),
    NoteStarted(Note),
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
    JoinCampus {
        token: AuthToken,
        request: JoinCampusRequest,
    },
    CreateInvite {
        token: AuthToken,
        campus_id: CampusId,
    },
    LoadMembers {
        token: AuthToken,
        campus_id: CampusId,
    },
    AddMember {
        token: AuthToken,
        campus_id: CampusId,
        request: AddMemberRequest,
    },
    UpdateMember {
        token: AuthToken,
        campus_id: CampusId,
        account_id: AccountId,
        request: UpdateMemberRequest,
    },
    RemoveMember {
        token: AuthToken,
        campus_id: CampusId,
        account_id: AccountId,
    },
    LoadNotes {
        token: AuthToken,
        campus_id: CampusId,
    },
    StartNote {
        token: AuthToken,
        campus_id: CampusId,
        request: StartNoteRequest,
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
    active_room_id: Option<RoomId>,
    invite: Option<CampusInvite>,
    members: Vec<CampusMember>,
    notes: Vec<Note>,
    quick_sign_in: bool,
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
            active_room_id: None,
            invite: None,
            members: Vec::new(),
            notes: Vec::new(),
            quick_sign_in: false,
            busy: false,
            error: None,
        }
    }
}

impl SessionMachine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_quick_sign_in(mut self, enabled: bool) -> Self {
        self.quick_sign_in = enabled;
        self
    }

    fn reset(&self) -> Self {
        Self::new().with_quick_sign_in(self.quick_sign_in)
    }

    fn quick_sign_in_request(&self, email: &str) -> Option<SignInRequest> {
        if !self.quick_sign_in {
            return None;
        }
        DEV_SEED_PEOPLE
            .iter()
            .find(|(seeded, _, _)| *seeded == email)
            .map(|(seeded, _, _)| SignInRequest {
                email: (*seeded).to_owned(),
                password: DEV_SEED_PASSWORD.to_owned(),
            })
    }

    pub fn token(&self) -> Option<&AuthToken> {
        self.token.as_ref()
    }

    pub fn account(&self) -> Option<&AccountProfile> {
        self.account.as_ref()
    }

    pub fn active_campus_id(&self) -> Option<CampusId> {
        self.active_campus_id
    }

    fn active_campus(&self) -> Option<&CampusSummary> {
        self.campuses
            .iter()
            .find(|campus| Some(campus.campus_id) == self.active_campus_id)
    }

    fn room_listed(&self, room_id: RoomId) -> bool {
        self.rooms.iter().any(|room| room.room_id == room_id)
            || self.notes.iter().any(|note| note.room_id == room_id)
    }

    pub fn note_room_ids(&self) -> impl Iterator<Item = RoomId> + '_ {
        self.notes.iter().map(|note| note.room_id)
    }

    fn keep_note(&mut self, note: Note) -> bool {
        if Some(note.campus_id) != self.active_campus_id {
            return false;
        }
        match self
            .notes
            .iter_mut()
            .find(|listed| listed.room_id == note.room_id)
        {
            Some(listed) => *listed = note,
            None => self.notes.push(note),
        }
        true
    }

    fn settle_active_room(&mut self) {
        let still_listed = self
            .active_room_id
            .is_some_and(|active| self.room_listed(active));
        if !still_listed {
            self.active_room_id = self.rooms.first().map(|room| room.room_id);
        }
    }

    pub fn snapshot(&self) -> SessionSnapshot {
        SessionSnapshot {
            auth: self.auth,
            account: self.account.clone(),
            campuses: self.campuses.clone(),
            active_campus_id: self.active_campus_id,
            rooms: self.rooms.clone(),
            active_room_id: self.active_room_id,
            invite: self
                .invite
                .clone()
                .filter(|invite| Some(invite.campus_id) == self.active_campus_id),
            can_invite: self
                .active_campus()
                .is_some_and(|campus| role_permits(campus.role, Permission::ManageMembers)),
            members: self.member_entries(),
            assignable_roles: self
                .active_campus()
                .map(|campus| assignable_roles(campus.role))
                .unwrap_or_default(),
            notes: self.note_entries(),
            note_candidates: self
                .member_entries()
                .into_iter()
                .filter(|member| !member.is_self)
                .collect(),
            quick_accounts: if self.quick_sign_in && self.auth == AuthPhase::SignedOut {
                dev_accounts()
            } else {
                Vec::new()
            },
            busy: self.busy,
            error: self.error,
        }
    }

    fn note_entries(&self) -> Vec<NoteEntry> {
        let Some(viewer) = self.account.as_ref().map(|account| account.account_id) else {
            return Vec::new();
        };
        self.notes
            .iter()
            .filter(|note| Some(note.campus_id) == self.active_campus_id)
            .map(|note| NoteEntry {
                room_id: note.room_id,
                title: note_title(note, viewer),
                member_count: u32::try_from(note.members.len()).unwrap_or(u32::MAX),
                group: note.members.len() > NOTE_PAIR_SIZE,
            })
            .collect()
    }

    fn member_entries(&self) -> Vec<MemberEntry> {
        let actor = self.active_campus().map(|campus| campus.role);
        let self_id = self.account.as_ref().map(|account| account.account_id);
        self.members
            .iter()
            .filter(|member| Some(member.campus_id) == self.active_campus_id)
            .map(|member| MemberEntry {
                account_id: member.account_id,
                display_name: member.display_name.clone(),
                email: member.email.clone(),
                role: member.role,
                is_self: Some(member.account_id) == self_id,
                manageable: Some(member.account_id) != self_id
                    && actor.is_some_and(|actor| member_manageable(actor, member.role)),
            })
            .collect()
    }

    fn active_command(
        mut self,
        build: impl FnOnce(AuthToken, CampusId) -> SessionCommand,
    ) -> (Self, Option<SessionCommand>) {
        match self.active_campus_id {
            Some(campus_id) => self.with_token(|token| build(token, campus_id)),
            None => {
                self.busy = false;
                (self, None)
            }
        }
    }

    fn validated_command(
        mut self,
        validation: Result<(), ApiError>,
        build: impl FnOnce(AuthToken, CampusId) -> SessionCommand,
    ) -> (Self, Option<SessionCommand>) {
        match validation {
            Ok(()) => self.active_command(build),
            Err(error) => {
                self.error = Some(error);
                (self, None)
            }
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
        self.members.clear();
        self.notes.clear();
        self.active_room_id = None;
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
                    self.active_room_id = None;
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
            SessionEvent::JoinCampusRequested(request) => match request.validate() {
                Ok(()) => self.with_token(|token| SessionCommand::JoinCampus { token, request }),
                Err(error) => {
                    self.error = Some(error);
                    (self, None)
                }
            },
            SessionEvent::InviteRequested => self.active_command(|token, campus_id| {
                SessionCommand::CreateInvite { token, campus_id }
            }),
            SessionEvent::MembersRequested => self.active_command(|token, campus_id| {
                SessionCommand::LoadMembers { token, campus_id }
            }),
            SessionEvent::MembersLoaded(list) => {
                if Some(list.campus_id) == self.active_campus_id {
                    self.members = list.members;
                }
                self.busy = false;
                (self, None)
            }
            SessionEvent::AddMemberRequested(request) => {
                let validation = request.validate();
                self.validated_command(validation, |token, campus_id| SessionCommand::AddMember {
                    token,
                    campus_id,
                    request,
                })
            }
            SessionEvent::UpdateMemberRequested {
                account_id,
                request,
            } => {
                let validation = request.validate();
                self.validated_command(validation, |token, campus_id| {
                    SessionCommand::UpdateMember {
                        token,
                        campus_id,
                        account_id,
                        request,
                    }
                })
            }
            SessionEvent::RemoveMemberRequested(account_id) => {
                self.active_command(|token, campus_id| SessionCommand::RemoveMember {
                    token,
                    campus_id,
                    account_id,
                })
            }
            SessionEvent::MemberSaved(member) => {
                self.busy = false;
                if Some(member.campus_id) == self.active_campus_id {
                    match self
                        .members
                        .iter_mut()
                        .find(|listed| listed.account_id == member.account_id)
                    {
                        Some(listed) => *listed = member,
                        None => self.members.push(member),
                    }
                }
                (self, None)
            }
            SessionEvent::MemberRemoved {
                campus_id,
                account_id,
            } => {
                self.busy = false;
                if Some(campus_id) == self.active_campus_id {
                    self.members
                        .retain(|member| member.account_id != account_id);
                }
                (self, None)
            }
            SessionEvent::InviteCreated(invite) => {
                self.busy = false;
                if Some(invite.campus_id) == self.active_campus_id {
                    self.invite = Some(invite);
                }
                (self, None)
            }
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
                self.settle_active_room();
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
                if Some(list.campus_id) != self.active_campus_id {
                    return (self, None);
                }
                self.rooms = list.rooms;
                self.settle_active_room();
                self.active_command(|token, campus_id| SessionCommand::LoadNotes {
                    token,
                    campus_id,
                })
            }
            SessionEvent::NotesLoaded(list) => {
                if Some(list.campus_id) == self.active_campus_id {
                    self.notes = list.notes;
                    self.settle_active_room();
                    self.busy = false;
                }
                (self, None)
            }
            SessionEvent::StartNoteRequested(request) => {
                let starter = self.account.as_ref().map(|account| account.account_id);
                let validation = starter
                    .ok_or(InputField::NoteMembers)
                    .and_then(|starter| normalize_note_members(&request.account_ids, starter))
                    .map(|_| ())
                    .map_err(ApiError::invalid);
                self.validated_command(validation, |token, campus_id| SessionCommand::StartNote {
                    token,
                    campus_id,
                    request,
                })
            }
            SessionEvent::NoteOpened(note) => {
                self.busy = false;
                let room_id = note.room_id;
                if self.keep_note(note) {
                    self.active_room_id = Some(room_id);
                }
                (self, None)
            }
            SessionEvent::NoteStarted(note) => {
                self.keep_note(note);
                (self, None)
            }
            SessionEvent::RoomSelected(room_id) => {
                if self.room_listed(room_id) {
                    self.active_room_id = Some(room_id);
                }
                (self, None)
            }
            SessionEvent::RequestFailed(error) => {
                if error.code == ApiErrorCode::Unauthenticated {
                    let mut reset = self.reset();
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
                _ => (self.reset(), command),
            };
        }
        match (self.auth, event) {
            (AuthPhase::SignedOut, SessionEvent::SignUpRequested(request)) => {
                let validation = request.validate();
                self.begin_authentication(validation, SessionCommand::SignUp(request))
            }
            (AuthPhase::SignedOut, SessionEvent::QuickSignInRequested(email)) => {
                match self.quick_sign_in_request(&email) {
                    Some(request) => {
                        self.begin_authentication(Ok(()), SessionCommand::SignIn(request))
                    }
                    None => (self, None),
                }
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
            SessionCommand::JoinCampus { token, request } => api
                .join_campus(token, request)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::CampusCreated),
            SessionCommand::CreateInvite { token, campus_id } => api
                .create_invite(token, campus_id)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::InviteCreated),
            SessionCommand::LoadMembers { token, campus_id } => api
                .list_members(token, campus_id)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::MembersLoaded),
            SessionCommand::AddMember {
                token,
                campus_id,
                request,
            } => api
                .add_member(token, campus_id, request)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::MemberSaved),
            SessionCommand::UpdateMember {
                token,
                campus_id,
                account_id,
                request,
            } => api
                .update_member(token, campus_id, account_id, request)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::MemberSaved),
            SessionCommand::LoadNotes { token, campus_id } => api
                .list_notes(token, campus_id)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::NotesLoaded),
            SessionCommand::StartNote {
                token,
                campus_id,
                request,
            } => api
                .start_note(token, campus_id, request)
                .await
                .map_or_else(SessionEvent::RequestFailed, SessionEvent::NoteOpened),
            SessionCommand::RemoveMember {
                token,
                campus_id,
                account_id,
            } => api
                .remove_member(token, campus_id, account_id)
                .await
                .map_or_else(SessionEvent::RequestFailed, |()| {
                    SessionEvent::MemberRemoved {
                        campus_id,
                        account_id,
                    }
                }),
        };
        let (next, next_command) = machine.step(next_event);
        machine = next;
        command = next_command;
    }
    machine
}
