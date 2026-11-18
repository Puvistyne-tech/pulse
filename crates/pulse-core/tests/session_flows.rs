mod fixtures;

use std::collections::VecDeque;

use fixtures::{
    CAMPUS_A, CAMPUS_B, RecordedCall, ScriptedApi, auth_session, campus_c, campus_list,
    campus_summary, create_campus_request, room, room_list, sign_in_request, signed_in_machine,
    token_a,
};
use futures::executor::block_on;
use pulse_core::{SessionEvent, SessionMachine, drive_session};
use pulse_protocol::{
    ApiError, ApiErrorCode, AuthPhase, CampusCreated, CampusId, CampusList, Role, RoomList,
};
use rstest::rstest;

struct Expect {
    auth: AuthPhase,
    active: Option<CampusId>,
    campuses: Vec<CampusId>,
    rooms: Vec<&'static str>,
    busy: bool,
    error: Option<ApiError>,
    calls: Vec<RecordedCall>,
}

fn failure(code: ApiErrorCode) -> ApiError {
    ApiError::new(code)
}

fn two_campuses() -> CampusList {
    campus_list(vec![
        campus_summary(CAMPUS_A, "alpha", Role::Owner),
        campus_summary(CAMPUS_B, "bravo", Role::Member),
    ])
}

fn gamma_created() -> CampusCreated {
    CampusCreated {
        campus: campus_summary(campus_c(), "gamma", Role::Owner),
        rooms: ["commons", "courtyard"]
            .iter()
            .map(|slug| room(campus_c(), slug))
            .collect(),
    }
}

fn script_sign_in() -> ScriptedApi {
    ScriptedApi {
        sign_ins: VecDeque::from([Ok(auth_session())]),
        campus_lists: VecDeque::from([Ok(two_campuses())]),
        room_lists: VecDeque::from([Ok(room_list(CAMPUS_A, &["commons", "courtyard"]))]),
        ..ScriptedApi::default()
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
        calls: vec![],
    }
}

#[rstest]
#[case::sign_in_selects_first_campus_and_loads_rooms(
    SessionMachine::new(),
    script_sign_in(),
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
        active: Some(campus_c()),
        campuses: vec![CAMPUS_A, CAMPUS_B, campus_c()],
        rooms: vec!["commons", "courtyard"],
        calls: vec![RecordedCall::CreateCampus(token_a(), create_campus_request())],
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
    assert_eq!(api.calls, expect.calls);
    assert_eq!(
        machine.token().is_some(),
        expect.auth == AuthPhase::SignedIn
    );
}
