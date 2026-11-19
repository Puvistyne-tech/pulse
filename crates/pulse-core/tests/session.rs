mod fixtures;

use fixtures::{
    ACCOUNT_A, ACCOUNT_B, ACCOUNT_C, CAMPUS_A, CAMPUS_B, CAMPUS_C, active_room_slug, auth_session,
    campus_invite, campus_list, campus_member, campus_summary, create_campus_request,
    invalid_create_campus_request, invalid_sign_in_request, invite_code, join_campus_request,
    member_list, room, room_list, session_after, session_from, short_password_sign_up_request,
    sign_in_request, sign_up_request, token_a, two_campuses,
};
use pulse_core::{SessionCommand, SessionEvent, SessionMachine, decode_api_response};
use pulse_protocol::{
    AccountId, AddMemberRequest, ApiError, ApiErrorCode, AuthPhase, CampusCreated, CampusId,
    CampusList, DEV_SEED_PASSWORD, DEV_SEED_PEOPLE, InputField, MemberList, Role, RoomId, RoomList,
    SignInRequest, UpdateMemberRequest,
};
use rstest::rstest;

struct Expect {
    auth: AuthPhase,
    has_account: bool,
    campuses: Vec<CampusId>,
    active: Option<CampusId>,
    rooms: Vec<&'static str>,
    active_room: Option<&'static str>,
    invite: Option<CampusId>,
    can_invite: bool,
    members: Vec<(AccountId, Role, bool)>,
    assignable: Vec<Role>,
    busy: bool,
    error: Option<ApiError>,
    command: Option<SessionCommand>,
    has_token: bool,
}

fn ready() -> Expect {
    Expect {
        auth: AuthPhase::SignedIn,
        has_account: true,
        campuses: vec![CAMPUS_A, CAMPUS_B],
        active: Some(CAMPUS_A),
        rooms: vec!["commons", "courtyard"],
        active_room: Some("commons"),
        invite: None,
        can_invite: true,
        members: vec![],
        assignable: vec![Role::Admin, Role::Member, Role::Guest],
        busy: false,
        error: None,
        command: None,
        has_token: true,
    }
}

fn signed_out() -> Expect {
    Expect {
        auth: AuthPhase::SignedOut,
        has_account: false,
        campuses: vec![],
        active: None,
        rooms: vec![],
        active_room: None,
        invite: None,
        can_invite: false,
        members: vec![],
        assignable: vec![],
        busy: false,
        error: None,
        command: None,
        has_token: false,
    }
}

fn authenticating() -> Expect {
    Expect {
        auth: AuthPhase::Authenticating,
        busy: true,
        ..signed_out()
    }
}

fn signed_in_loading() -> Expect {
    Expect {
        auth: AuthPhase::SignedIn,
        has_account: true,
        busy: true,
        has_token: true,
        ..signed_out()
    }
}

fn sign_in_events() -> Vec<SessionEvent> {
    vec![SessionEvent::SignInRequested(sign_in_request())]
}

fn loading_events() -> Vec<SessionEvent> {
    vec![
        SessionEvent::SignInRequested(sign_in_request()),
        SessionEvent::Authenticated(auth_session()),
    ]
}

fn ready_events() -> Vec<SessionEvent> {
    vec![
        SessionEvent::SignInRequested(sign_in_request()),
        SessionEvent::Authenticated(auth_session()),
        SessionEvent::CampusesLoaded(two_campuses()),
        SessionEvent::RoomsLoaded(room_list(CAMPUS_A, &["commons", "courtyard"])),
    ]
}

fn failed_events() -> Vec<SessionEvent> {
    let mut events = ready_events();
    events.push(failure(ApiErrorCode::Unavailable));
    events
}

fn switching_events() -> Vec<SessionEvent> {
    let mut events = ready_events();
    events.push(SessionEvent::CampusSelected(CAMPUS_B));
    events
}

fn invited_events(campus_id: CampusId) -> Vec<SessionEvent> {
    let mut events = ready_events();
    events.push(SessionEvent::InviteRequested);
    events.push(SessionEvent::InviteCreated(campus_invite(campus_id)));
    events
}

fn roster() -> MemberList {
    member_list(
        CAMPUS_A,
        &[
            (ACCOUNT_A, Role::Owner),
            (ACCOUNT_B, Role::Admin),
            (ACCOUNT_C, Role::Member),
        ],
    )
}

fn roster_events() -> Vec<SessionEvent> {
    let mut events = ready_events();
    events.push(SessionEvent::MembersLoaded(roster()));
    events
}

fn roster_view() -> Vec<(AccountId, Role, bool)> {
    vec![
        (ACCOUNT_A, Role::Owner, false),
        (ACCOUNT_B, Role::Admin, true),
        (ACCOUNT_C, Role::Member, true),
    ]
}

fn add_request(email: &str) -> AddMemberRequest {
    AddMemberRequest {
        email: email.to_owned(),
        role: Role::Member,
    }
}

fn failure(code: ApiErrorCode) -> SessionEvent {
    SessionEvent::RequestFailed(ApiError::new(code))
}

fn load_rooms(campus_id: CampusId) -> Option<SessionCommand> {
    Some(SessionCommand::LoadRooms {
        token: token_a(),
        campus_id,
    })
}

fn created(campus_id: CampusId, slug: &str, rooms: &[&str]) -> SessionEvent {
    SessionEvent::CampusCreated(CampusCreated {
        campus: campus_summary(campus_id, slug, Role::Owner),
        rooms: rooms.iter().map(|name| room(campus_id, name)).collect(),
    })
}

#[rstest]
#[case::sign_in_valid(
    vec![],
    SessionEvent::SignInRequested(sign_in_request()),
    Expect { command: Some(SessionCommand::SignIn(sign_in_request())), ..authenticating() },
)]
#[case::sign_up_valid(
    vec![],
    SessionEvent::SignUpRequested(sign_up_request()),
    Expect { command: Some(SessionCommand::SignUp(sign_up_request())), ..authenticating() },
)]
#[case::sign_in_invalid_email(
    vec![],
    SessionEvent::SignInRequested(invalid_sign_in_request()),
    Expect { error: Some(ApiError::invalid(InputField::Email)), ..signed_out() },
)]
#[case::sign_up_invalid_password(
    vec![],
    SessionEvent::SignUpRequested(short_password_sign_up_request()),
    Expect { error: Some(ApiError::invalid(InputField::Password)), ..signed_out() },
)]
#[case::authenticated(
    sign_in_events(),
    SessionEvent::Authenticated(auth_session()),
    Expect { command: Some(SessionCommand::LoadCampuses(token_a())), ..signed_in_loading() },
)]
#[case::authentication_failed(
    sign_in_events(),
    failure(ApiErrorCode::Unauthenticated),
    Expect { error: Some(ApiError::new(ApiErrorCode::Unauthenticated)), ..signed_out() },
)]
#[case::campuses_loaded_selects_first(
    loading_events(),
    SessionEvent::CampusesLoaded(two_campuses()),
    Expect {
        campuses: vec![CAMPUS_A, CAMPUS_B],
        active: Some(CAMPUS_A),
        can_invite: true,
        assignable: vec![Role::Admin, Role::Member, Role::Guest],
        command: load_rooms(CAMPUS_A),
        ..signed_in_loading()
    },
)]
#[case::campuses_loaded_empty(
    loading_events(),
    SessionEvent::CampusesLoaded(campus_list(vec![])),
    Expect { busy: false, ..signed_in_loading() },
)]
#[case::campuses_loaded_keeps_active(
    ready_events(),
    SessionEvent::CampusesLoaded(two_campuses()),
    ready()
)]
#[case::campuses_loaded_replaces_missing_active(
    ready_events(),
    SessionEvent::CampusesLoaded(campus_list(vec![campus_summary(CAMPUS_B, "bravo", Role::Member)])),
    Expect {
        campuses: vec![CAMPUS_B],
        active: Some(CAMPUS_B),
        can_invite: false,
        assignable: vec![],
        rooms: vec![],
        active_room: None,
        busy: true,
        command: load_rooms(CAMPUS_B),
        ..ready()
    },
)]
#[case::campuses_requested(
    ready_events(),
    SessionEvent::CampusesRequested,
    Expect { busy: true, command: Some(SessionCommand::LoadCampuses(token_a())), ..ready() },
)]
#[case::campuses_requested_clears_error(
    failed_events(),
    SessionEvent::CampusesRequested,
    Expect { busy: true, command: Some(SessionCommand::LoadCampuses(token_a())), ..ready() },
)]
#[case::create_campus_valid(
    ready_events(),
    SessionEvent::CreateCampusRequested(create_campus_request()),
    Expect {
        busy: true,
        command: Some(SessionCommand::CreateCampus { token: token_a(), request: create_campus_request() }),
        ..ready()
    },
)]
#[case::create_campus_invalid(
    ready_events(),
    SessionEvent::CreateCampusRequested(invalid_create_campus_request()),
    Expect { error: Some(ApiError::invalid(InputField::Slug)), ..ready() },
)]
#[case::join_campus_valid(
    ready_events(),
    SessionEvent::JoinCampusRequested(join_campus_request(&invite_code(CAMPUS_C))),
    Expect {
        busy: true,
        command: Some(SessionCommand::JoinCampus {
            token: token_a(),
            request: join_campus_request(&invite_code(CAMPUS_C)),
        }),
        ..ready()
    },
)]
#[case::join_campus_invalid(
    ready_events(),
    SessionEvent::JoinCampusRequested(join_campus_request("nope")),
    Expect { error: Some(ApiError::invalid(InputField::InviteCode)), ..ready() },
)]
#[case::invite_requested(
    ready_events(),
    SessionEvent::InviteRequested,
    Expect {
        busy: true,
        command: Some(SessionCommand::CreateInvite { token: token_a(), campus_id: CAMPUS_A }),
        ..ready()
    },
)]
#[case::invite_requested_without_campus(
    vec![
        SessionEvent::SignInRequested(sign_in_request()),
        SessionEvent::Authenticated(auth_session()),
        SessionEvent::CampusesLoaded(campus_list(vec![])),
    ],
    SessionEvent::InviteRequested,
    Expect { auth: AuthPhase::SignedIn, has_account: true, has_token: true, ..signed_out() },
)]
#[case::invite_created_for_active(
    ready_events(),
    SessionEvent::InviteCreated(campus_invite(CAMPUS_A)),
    Expect { invite: Some(CAMPUS_A), ..ready() },
)]
#[case::invite_created_for_stale_campus_ignored(
    ready_events(),
    SessionEvent::InviteCreated(campus_invite(CAMPUS_B)),
    ready()
)]
#[case::invite_hidden_after_campus_switch(
    invited_events(CAMPUS_A),
    SessionEvent::CampusSelected(CAMPUS_B),
    Expect {
        active: Some(CAMPUS_B),
        can_invite: false,
        assignable: vec![],
        rooms: vec![],
        active_room: None,
        busy: true,
        command: load_rooms(CAMPUS_B),
        ..ready()
    },
)]
#[case::members_requested(
    ready_events(),
    SessionEvent::MembersRequested,
    Expect {
        busy: true,
        command: Some(SessionCommand::LoadMembers { token: token_a(), campus_id: CAMPUS_A }),
        ..ready()
    },
)]
#[case::members_loaded(ready_events(), SessionEvent::MembersLoaded(roster()), Expect { members: roster_view(), ..ready() })]
#[case::members_loaded_for_stale_campus_ignored(
    ready_events(),
    SessionEvent::MembersLoaded(member_list(CAMPUS_B, &[(ACCOUNT_B, Role::Member)])),
    ready(),
)]
#[case::add_member_valid(
    ready_events(),
    SessionEvent::AddMemberRequested(add_request("grace@example.org")),
    Expect {
        busy: true,
        command: Some(SessionCommand::AddMember {
            token: token_a(),
            campus_id: CAMPUS_A,
            request: add_request("grace@example.org"),
        }),
        ..ready()
    },
)]
#[case::add_member_invalid_email(
    ready_events(),
    SessionEvent::AddMemberRequested(add_request("grace")),
    Expect { error: Some(ApiError::invalid(InputField::Email)), ..ready() },
)]
#[case::update_member_valid(
    ready_events(),
    SessionEvent::UpdateMemberRequested { account_id: ACCOUNT_B, request: UpdateMemberRequest { role: Role::Guest } },
    Expect {
        busy: true,
        command: Some(SessionCommand::UpdateMember {
            token: token_a(),
            campus_id: CAMPUS_A,
            account_id: ACCOUNT_B,
            request: UpdateMemberRequest { role: Role::Guest },
        }),
        ..ready()
    },
)]
#[case::update_member_invalid_role(
    ready_events(),
    SessionEvent::UpdateMemberRequested { account_id: ACCOUNT_B, request: UpdateMemberRequest { role: Role::Owner } },
    Expect { error: Some(ApiError::invalid(InputField::Role)), ..ready() },
)]
#[case::remove_member(
    ready_events(),
    SessionEvent::RemoveMemberRequested(ACCOUNT_C),
    Expect {
        busy: true,
        command: Some(SessionCommand::RemoveMember { token: token_a(), campus_id: CAMPUS_A, account_id: ACCOUNT_C }),
        ..ready()
    },
)]
#[case::member_saved_replaces_listed(
    roster_events(),
    SessionEvent::MemberSaved(campus_member(CAMPUS_A, ACCOUNT_C, Role::Guest)),
    Expect {
        members: vec![(ACCOUNT_A, Role::Owner, false), (ACCOUNT_B, Role::Admin, true), (ACCOUNT_C, Role::Guest, true)],
        ..ready()
    },
)]
#[case::member_saved_appends_new(
    ready_events(),
    SessionEvent::MemberSaved(campus_member(CAMPUS_A, ACCOUNT_B, Role::Admin)),
    Expect { members: vec![(ACCOUNT_B, Role::Admin, true)], ..ready() },
)]
#[case::member_removed(
    roster_events(),
    SessionEvent::MemberRemoved { campus_id: CAMPUS_A, account_id: ACCOUNT_B },
    Expect { members: vec![(ACCOUNT_A, Role::Owner, false), (ACCOUNT_C, Role::Member, true)], ..ready() },
)]
#[case::member_removed_from_stale_campus_ignored(
    roster_events(),
    SessionEvent::MemberRemoved { campus_id: CAMPUS_B, account_id: ACCOUNT_B },
    Expect { members: roster_view(), ..ready() },
)]
#[case::members_cleared_on_campus_switch(
    roster_events(),
    SessionEvent::CampusSelected(CAMPUS_B),
    Expect {
        active: Some(CAMPUS_B),
        can_invite: false,
        assignable: vec![],
        rooms: vec![],
        active_room: None,
        busy: true,
        command: load_rooms(CAMPUS_B),
        ..ready()
    },
)]
#[case::campus_created(
    ready_events(),
    created(CAMPUS_C, "gamma", &["commons", "courtyard"]),
    Expect {
        campuses: vec![CAMPUS_A, CAMPUS_B, CAMPUS_C],
        active: Some(CAMPUS_C),
        ..ready()
    },
)]
#[case::campus_created_already_listed(
    ready_events(),
    created(CAMPUS_A, "alpha", &["commons"]),
    Expect { rooms: vec!["commons"], active_room: Some("commons"), ..ready() },
)]
#[case::campus_selected(
    ready_events(),
    SessionEvent::CampusSelected(CAMPUS_B),
    Expect {
        active: Some(CAMPUS_B),
        can_invite: false,
        assignable: vec![],
        rooms: vec![],
        active_room: None,
        busy: true,
        command: load_rooms(CAMPUS_B),
        ..ready()
    },
)]
#[case::campus_selected_clears_error(
    failed_events(),
    SessionEvent::CampusSelected(CAMPUS_B),
    Expect {
        active: Some(CAMPUS_B),
        can_invite: false,
        assignable: vec![],
        rooms: vec![],
        active_room: None,
        busy: true,
        command: load_rooms(CAMPUS_B),
        ..ready()
    },
)]
#[case::campus_selected_unknown_ignored(
    ready_events(),
    SessionEvent::CampusSelected(CAMPUS_C),
    ready()
)]
#[case::rooms_loaded_for_active(
    switching_events(),
    SessionEvent::RoomsLoaded(room_list(CAMPUS_B, &["lab"])),
    Expect {
        active: Some(CAMPUS_B),
        can_invite: false,
        assignable: vec![],
        rooms: vec!["lab"],
        active_room: Some("lab"),
        ..ready()
    },
)]
#[case::rooms_loaded_for_stale_campus_ignored(
    switching_events(),
    SessionEvent::RoomsLoaded(room_list(CAMPUS_A, &["commons"])),
    Expect {
        active: Some(CAMPUS_B),
        can_invite: false,
        assignable: vec![],
        rooms: vec![],
        active_room: None,
        busy: true,
        ..ready()
    },
)]
#[case::request_failed_keeps_session(
    ready_events(),
    failure(ApiErrorCode::Conflict),
    Expect { error: Some(ApiError::new(ApiErrorCode::Conflict)), ..ready() },
)]
#[case::request_failed_unauthenticated_resets(
    ready_events(),
    failure(ApiErrorCode::Unauthenticated),
    Expect { error: Some(ApiError::new(ApiErrorCode::Unauthenticated)), ..signed_out() },
)]
#[case::sign_out_signed_in(
    ready_events(),
    SessionEvent::SignOutRequested,
    Expect { command: Some(SessionCommand::SignOut(token_a())), ..signed_out() },
)]
#[case::sign_out_authenticating(sign_in_events(), SessionEvent::SignOutRequested, signed_out())]
#[case::sign_out_signed_out(vec![], SessionEvent::SignOutRequested, signed_out())]
#[case::authenticated_ignored_when_signed_out(
    vec![],
    SessionEvent::Authenticated(auth_session()),
    signed_out(),
)]
#[case::campuses_loaded_ignored_when_signed_out(
    vec![],
    SessionEvent::CampusesLoaded(two_campuses()),
    signed_out(),
)]
#[case::sign_in_ignored_when_signed_in(
    ready_events(),
    SessionEvent::SignInRequested(sign_in_request()),
    ready()
)]
fn step_follows_transition_table(
    #[case] setup: Vec<SessionEvent>,
    #[case] event: SessionEvent,
    #[case] expected: Expect,
) {
    let (machine, command) = session_after(setup).step(event);
    let snapshot = machine.snapshot();
    assert_eq!(snapshot.auth, expected.auth);
    assert_eq!(snapshot.account.is_some(), expected.has_account);
    assert_eq!(
        snapshot
            .campuses
            .iter()
            .map(|campus| campus.campus_id)
            .collect::<Vec<_>>(),
        expected.campuses
    );
    assert_eq!(snapshot.active_campus_id, expected.active);
    assert_eq!(
        snapshot
            .rooms
            .iter()
            .map(|room| room.slug.as_str())
            .collect::<Vec<_>>(),
        expected.rooms
    );
    assert_eq!(active_room_slug(&snapshot), expected.active_room);
    assert_eq!(
        snapshot.invite.map(|invite| invite.campus_id),
        expected.invite
    );
    assert_eq!(snapshot.can_invite, expected.can_invite);
    assert_eq!(
        snapshot
            .members
            .iter()
            .map(|member| (member.account_id, member.role, member.manageable))
            .collect::<Vec<_>>(),
        expected.members
    );
    assert_eq!(snapshot.assignable_roles, expected.assignable);
    assert_eq!(snapshot.busy, expected.busy);
    assert_eq!(snapshot.error, expected.error);
    assert_eq!(command, expected.command);
    assert_eq!(machine.token().is_some(), expected.has_token);
}

#[rstest]
#[case::ok_body(200, br#"{"campuses":[]}"#.to_vec(), Ok(campus_list(vec![])))]
#[case::created_body(201, br#"{"campuses":[]}"#.to_vec(), Ok(campus_list(vec![])))]
#[case::success_with_garbage(200, b"nope".to_vec(), Err(ApiError::new(ApiErrorCode::Internal)))]
#[case::conflict_body(
    409,
    br#"{"code":"conflict","field":"slug"}"#.to_vec(),
    Err(ApiError::conflict(InputField::Slug)),
)]
#[case::unauthenticated_body(
    401,
    br#"{"code":"unauthenticated","field":null}"#.to_vec(),
    Err(ApiError::new(ApiErrorCode::Unauthenticated)),
)]
#[case::empty_not_found(404, Vec::new(), Err(ApiError::new(ApiErrorCode::NotFound)))]
#[case::html_bad_gateway(502, b"<html>".to_vec(), Err(ApiError::new(ApiErrorCode::Internal)))]
#[case::garbage_bad_request(400, b"{".to_vec(), Err(ApiError::new(ApiErrorCode::InvalidInput)))]
fn decode_api_response_maps_status_and_body(
    #[case] status: u16,
    #[case] body: Vec<u8>,
    #[case] expected: Result<CampusList, ApiError>,
) {
    assert_eq!(decode_api_response::<CampusList>(status, &body), expected);
}

#[derive(Debug, Clone, Copy)]
enum Selection {
    Listed(usize),
    Unlisted,
}

#[derive(Debug, Clone, Copy)]
enum Followup {
    Nothing,
    SameRoomsReloaded,
    FreshRoomsReloaded,
    CampusSwitched,
    SignedOut,
}

fn followup_event(followup: Followup, machine: &SessionMachine) -> Option<SessionEvent> {
    let snapshot = machine.snapshot();
    match followup {
        Followup::Nothing => None,
        Followup::SameRoomsReloaded => Some(SessionEvent::RoomsLoaded(RoomList {
            campus_id: CAMPUS_A,
            rooms: snapshot.rooms,
        })),
        Followup::FreshRoomsReloaded => Some(SessionEvent::RoomsLoaded(room_list(
            CAMPUS_A,
            &["lab", "commons"],
        ))),
        Followup::CampusSwitched => Some(SessionEvent::CampusSelected(CAMPUS_B)),
        Followup::SignedOut => Some(SessionEvent::SignOutRequested),
    }
}

#[rstest]
#[case::listed_second(Selection::Listed(1), Followup::Nothing, Some("courtyard"))]
#[case::listed_first(Selection::Listed(0), Followup::Nothing, Some("commons"))]
#[case::unlisted_ignored(Selection::Unlisted, Followup::Nothing, Some("commons"))]
#[case::kept_when_still_listed(
    Selection::Listed(1),
    Followup::SameRoomsReloaded,
    Some("courtyard")
)]
#[case::falls_back_to_first_when_unlisted(
    Selection::Listed(1),
    Followup::FreshRoomsReloaded,
    Some("lab")
)]
#[case::cleared_on_campus_switch(Selection::Listed(1), Followup::CampusSwitched, None)]
#[case::cleared_on_sign_out(Selection::Listed(1), Followup::SignedOut, None)]
fn room_selection_follows_listing(
    #[case] selection: Selection,
    #[case] followup: Followup,
    #[case] expected: Option<&str>,
) {
    let machine = session_after(ready_events());
    let target = match selection {
        Selection::Listed(index) => machine.snapshot().rooms[index].room_id,
        Selection::Unlisted => RoomId::generate(),
    };
    let (machine, _) = machine.step(SessionEvent::RoomSelected(target));
    let machine = match followup_event(followup, &machine) {
        Some(event) => machine.step(event).0,
        None => machine,
    };
    let snapshot = machine.snapshot();
    assert_eq!(active_room_slug(&snapshot), expected);
    assert_eq!(snapshot.active_room_id.is_some(), expected.is_some());
}

fn quick_sign_in_of(email: &str) -> Option<SessionCommand> {
    Some(SessionCommand::SignIn(SignInRequest {
        email: email.to_owned(),
        password: DEV_SEED_PASSWORD.to_owned(),
    }))
}

#[rstest]
#[case::enabled_seeded(true, vec![], DEV_SEED_PEOPLE[1].0, DEV_SEED_PEOPLE.len(), quick_sign_in_of(DEV_SEED_PEOPLE[1].0))]
#[case::enabled_unknown(true, vec![], "nobody@pulse.test", DEV_SEED_PEOPLE.len(), None)]
#[case::disabled(false, vec![], DEV_SEED_PEOPLE[1].0, 0, None)]
#[case::kept_after_sign_out(
    true,
    vec![
        SessionEvent::SignInRequested(sign_in_request()),
        SessionEvent::Authenticated(auth_session()),
        SessionEvent::SignOutRequested,
    ],
    DEV_SEED_PEOPLE[0].0,
    DEV_SEED_PEOPLE.len(),
    quick_sign_in_of(DEV_SEED_PEOPLE[0].0),
)]
#[case::hidden_when_signed_in(true, ready_events(), DEV_SEED_PEOPLE[0].0, 0, None)]
fn quick_sign_in_uses_seeded_accounts(
    #[case] enabled: bool,
    #[case] setup: Vec<SessionEvent>,
    #[case] email: &str,
    #[case] offered: usize,
    #[case] command: Option<SessionCommand>,
) {
    let machine = session_from(SessionMachine::new().with_quick_sign_in(enabled), setup);
    assert_eq!(machine.snapshot().quick_accounts.len(), offered);
    let (machine, actual) = machine.step(SessionEvent::QuickSignInRequested(email.to_owned()));
    let authenticating = command.is_some();
    assert_eq!(actual, command);
    assert_eq!(
        machine.snapshot().auth == AuthPhase::Authenticating,
        authenticating
    );
}
