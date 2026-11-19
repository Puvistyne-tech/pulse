mod fixtures;

use std::collections::VecDeque;

use fixtures::{
    ACCOUNT_B, CAMPUS_A, CAMPUS_B, CAMPUS_C, RecordedCall, ScriptedApi, campus_invite,
    campus_member, campus_summary, create_campus_request, invite_code, join_campus_request, room,
    room_list, sign_in_request, signed_in_api, signed_in_machine, token_a,
};
use futures::executor::block_on;
use pulse_core::{SessionEvent, SessionMachine, drive_session};
use pulse_protocol::{
    AccountId, AddMemberRequest, ApiError, ApiErrorCode, AuthPhase, CampusCreated, CampusId, Role,
    RoomList,
};
use rstest::rstest;

struct Expect {
    auth: AuthPhase,
    active: Option<CampusId>,
    campuses: Vec<CampusId>,
    rooms: Vec<&'static str>,
    busy: bool,
    error: Option<ApiError>,
    invite: Option<CampusId>,
    members: Vec<AccountId>,
    calls: Vec<RecordedCall>,
}

fn failure(code: ApiErrorCode) -> ApiError {
    ApiError::new(code)
}

fn gamma_created() -> CampusCreated {
    CampusCreated {
        campus: campus_summary(CAMPUS_C, "gamma", Role::Owner),
        rooms: ["commons", "courtyard"]
            .iter()
            .map(|slug| room(CAMPUS_C, slug))
            .collect(),
    }
}

fn script_rejected_sign_in() -> ScriptedApi {
    ScriptedApi {
        sign_ins: VecDeque::from([Err(failure(ApiErrorCode::Unauthenticated))]),
        ..ScriptedApi::default()
    }
}

fn script_create_campus() -> ScriptedApi {
    ScriptedApi {
        campus_creations: VecDeque::from([Ok(gamma_created())]),
        ..ScriptedApi::default()
    }
}

fn script_join_campus() -> ScriptedApi {
    let mut joined = gamma_created();
    joined.campus.role = Role::Member;
    ScriptedApi {
        joins: VecDeque::from([Ok(joined)]),
        ..ScriptedApi::default()
    }
}

fn script_invite() -> ScriptedApi {
    ScriptedApi {
        invites: VecDeque::from([Ok(campus_invite(CAMPUS_A))]),
        ..ScriptedApi::default()
    }
}

fn script_member_added() -> ScriptedApi {
    ScriptedApi {
        member_saves: VecDeque::from([Ok(campus_member(CAMPUS_A, ACCOUNT_B, Role::Member))]),
        ..ScriptedApi::default()
    }
}

fn added_grace() -> AddMemberRequest {
    AddMemberRequest {
        email: "grace@example.org".to_owned(),
        role: Role::Member,
    }
}

fn script_expired_session() -> ScriptedApi {
    ScriptedApi {
        campus_lists: VecDeque::from([Err(failure(ApiErrorCode::Unauthenticated))]),
        ..ScriptedApi::default()
    }
}

fn script_stale_rooms() -> ScriptedApi {
    ScriptedApi {
        room_lists: VecDeque::from([Ok::<RoomList, ApiError>(room_list(CAMPUS_A, &["commons"]))]),
        ..ScriptedApi::default()
    }
}

fn script_sign_out_failing() -> ScriptedApi {
    ScriptedApi {
        sign_outs: VecDeque::from([Err(failure(ApiErrorCode::Unavailable))]),
        ..ScriptedApi::default()
    }
}

fn signed_in_expect() -> Expect {
    Expect {
        auth: AuthPhase::SignedIn,
        active: Some(CAMPUS_A),
        campuses: vec![CAMPUS_A, CAMPUS_B],
        rooms: vec!["commons", "courtyard"],
        busy: false,
        error: None,
        invite: None,
        members: vec![],
        calls: vec![],
    }
}

fn signed_out_expect() -> Expect {
    Expect {
        auth: AuthPhase::SignedOut,
        active: None,
        campuses: vec![],
        rooms: vec![],
        busy: false,
        error: None,
        invite: None,
        members: vec![],
        calls: vec![],
    }
}

#[rstest]
#[case::sign_in_selects_first_campus_and_loads_rooms(
    SessionMachine::new(),
    signed_in_api(),
    SessionEvent::SignInRequested(sign_in_request()),
    Expect {
        calls: vec![
            RecordedCall::SignIn(sign_in_request()),
            RecordedCall::ListCampuses(token_a()),
            RecordedCall::ListRooms(token_a(), CAMPUS_A),
        ],
        ..signed_in_expect()
    },
)]
#[case::rejected_sign_in_stays_signed_out(
    SessionMachine::new(),
    script_rejected_sign_in(),
    SessionEvent::SignInRequested(sign_in_request()),
    Expect {
        error: Some(failure(ApiErrorCode::Unauthenticated)),
        calls: vec![RecordedCall::SignIn(sign_in_request())],
        ..signed_out_expect()
    },
)]
#[case::create_campus_publishes_rooms(
    signed_in_machine(),
    script_create_campus(),
    SessionEvent::CreateCampusRequested(create_campus_request()),
    Expect {
        active: Some(CAMPUS_C),
        campuses: vec![CAMPUS_A, CAMPUS_B, CAMPUS_C],
        rooms: vec!["commons", "courtyard"],
        calls: vec![RecordedCall::CreateCampus(token_a(), create_campus_request())],
        ..signed_in_expect()
    },
)]
#[case::join_campus_switches_to_joined_campus(
    signed_in_machine(),
    script_join_campus(),
    SessionEvent::JoinCampusRequested(join_campus_request(&invite_code(CAMPUS_C))),
    Expect {
        active: Some(CAMPUS_C),
        campuses: vec![CAMPUS_A, CAMPUS_B, CAMPUS_C],
        rooms: vec!["commons", "courtyard"],
        calls: vec![RecordedCall::JoinCampus(
            token_a(),
            join_campus_request(&invite_code(CAMPUS_C)),
        )],
        ..signed_in_expect()
    },
)]
#[case::invite_requested_publishes_code(
    signed_in_machine(),
    script_invite(),
    SessionEvent::InviteRequested,
    Expect {
        invite: Some(CAMPUS_A),
        calls: vec![RecordedCall::CreateInvite(token_a(), CAMPUS_A)],
        ..signed_in_expect()
    },
)]
#[case::add_member_lists_new_member(
    signed_in_machine(),
    script_member_added(),
    SessionEvent::AddMemberRequested(added_grace()),
    Expect {
        members: vec![ACCOUNT_B],
        calls: vec![RecordedCall::AddMember(token_a(), CAMPUS_A, added_grace())],
        ..signed_in_expect()
    },
)]
#[case::unauthenticated_mid_session_resets(
    signed_in_machine(),
    script_expired_session(),
    SessionEvent::CampusesRequested,
    Expect {
        error: Some(failure(ApiErrorCode::Unauthenticated)),
        calls: vec![RecordedCall::ListCampuses(token_a())],
        ..signed_out_expect()
    },
)]
#[case::stale_rooms_are_ignored(
    signed_in_machine(),
    script_stale_rooms(),
    SessionEvent::CampusSelected(CAMPUS_B),
    Expect {
        active: Some(CAMPUS_B),
        rooms: vec![],
        busy: true,
        calls: vec![RecordedCall::ListRooms(token_a(), CAMPUS_B)],
        ..signed_in_expect()
    },
)]
#[case::sign_out_calls_api_once(
    signed_in_machine(),
    ScriptedApi::default(),
    SessionEvent::SignOutRequested,
    Expect {
        calls: vec![RecordedCall::SignOut(token_a())],
        ..signed_out_expect()
    },
)]
#[case::failed_sign_out_still_signs_out_once(
    signed_in_machine(),
    script_sign_out_failing(),
    SessionEvent::SignOutRequested,
    Expect {
        calls: vec![RecordedCall::SignOut(token_a())],
        ..signed_out_expect()
    },
)]
fn drive_session_runs_command_chains(
    #[case] start: SessionMachine,
    #[case] mut api: ScriptedApi,
    #[case] event: SessionEvent,
    #[case] expect: Expect,
) {
    let machine = block_on(drive_session(start, &mut api, event));
    let snapshot = machine.snapshot();
    assert_eq!(snapshot.auth, expect.auth);
    assert_eq!(snapshot.active_campus_id, expect.active);
    assert_eq!(
        snapshot
            .campuses
            .iter()
            .map(|campus| campus.campus_id)
            .collect::<Vec<_>>(),
        expect.campuses
    );
    assert_eq!(
        snapshot
            .rooms
            .iter()
            .map(|room| room.slug.as_str())
            .collect::<Vec<_>>(),
        expect.rooms
    );
    assert_eq!(snapshot.busy, expect.busy);
    assert_eq!(snapshot.error, expect.error);
    assert_eq!(
        snapshot.invite.map(|invite| invite.campus_id),
        expect.invite
    );
    assert_eq!(
        snapshot
            .members
            .iter()
            .map(|member| member.account_id)
            .collect::<Vec<_>>(),
        expect.members
    );
    assert_eq!(api.calls, expect.calls);
    assert_eq!(
        machine.token().is_some(),
        expect.auth == AuthPhase::SignedIn
    );
}
