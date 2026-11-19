mod fixtures;

use fixtures::{
    ACCOUNT_A, ACCOUNT_B, NONCE_A, NONCE_B, ROOM_A, ROOM_B, STEWARD_A, STREAM_A, connected_as,
    created_frame, error_of, manage_frame, received, settled_machine, step_all,
    steward_changed_frame, steward_issued_frame, steward_message, steward_profile,
    steward_rejected_frame, stewards_synced_frame, stream_event,
};
use pulse_core::{StewardIntent, TimelineEvent, TimelineMachine};
use pulse_protocol::{
    AccountId, ApiError, ApiErrorCode, ClientFrame, ClientNonce, DISPLAY_NAME_MAX_CHARS,
    InputField, Role, RoomId, STEWARD_ROOMS_MAX, StewardAction, StewardRejection, StewardSnapshot,
    StreamPhase, mention_token,
};
use rstest::rstest;

const OTHER_STEWARD: AccountId = ACCOUNT_B;

#[derive(Debug, PartialEq, Eq)]
struct Seen {
    open: bool,
    stewards: Vec<(AccountId, String, Vec<RoomId>, bool)>,
    pending: bool,
    issued_for: Option<AccountId>,
    error: Option<ApiError>,
}

fn seen_of(snapshot: &StewardSnapshot) -> Seen {
    Seen {
        open: snapshot.open,
        stewards: snapshot
            .stewards
            .iter()
            .map(|entry| {
                assert_eq!(entry.mention_token, mention_token(entry.steward_id));
                (
                    entry.steward_id,
                    entry.display_name.clone(),
                    entry.room_ids.clone(),
                    entry.revoked,
                )
            })
            .collect(),
        pending: snapshot.pending,
        issued_for: snapshot.issued.as_ref().map(|issued| issued.steward_id),
        error: snapshot.error,
    }
}

fn seen(machine: &TimelineMachine) -> Seen {
    seen_of(&machine.steward_snapshot())
}

fn idle() -> Seen {
    Seen {
        open: false,
        stewards: Vec::new(),
        pending: false,
        issued_for: None,
        error: None,
    }
}

fn steward(intent: StewardIntent) -> TimelineEvent {
    TimelineEvent::Steward(intent)
}

fn create(request_nonce: ClientNonce, name: &str, room_ids: Vec<RoomId>) -> TimelineEvent {
    steward(StewardIntent::CreateRequested {
        request_nonce,
        display_name: name.to_owned(),
        room_ids,
    })
}

fn rotate(request_nonce: ClientNonce, steward_id: AccountId) -> TimelineEvent {
    steward(StewardIntent::RotateRequested {
        request_nonce,
        steward_id,
    })
}

fn revoke(request_nonce: ClientNonce, steward_id: AccountId) -> TimelineEvent {
    steward(StewardIntent::RevokeRequested {
        request_nonce,
        steward_id,
    })
}

fn assign(
    request_nonce: ClientNonce,
    steward_id: AccountId,
    room_ids: Vec<RoomId>,
) -> TimelineEvent {
    steward(StewardIntent::RoomsAssigned {
        request_nonce,
        steward_id,
        room_ids,
    })
}

fn listed(steward_id: AccountId, name: &str, revoked: bool) -> TimelineEvent {
    received(stewards_synced_frame(vec![steward_profile(
        steward_id,
        name,
        &[ROOM_A],
        revoked,
    )]))
}

fn too_many_rooms() -> Vec<RoomId> {
    (0..=STEWARD_ROOMS_MAX)
        .map(|_| RoomId::generate())
        .collect()
}

#[rstest]
#[case::sync_fills_the_directory(
    vec![listed(STEWARD_A, "Scribe", false)],
    vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_A], false)]
)]
#[case::sync_replaces_the_directory(
    vec![listed(STEWARD_A, "Scribe", false), listed(OTHER_STEWARD, "Herald", false)],
    vec![(OTHER_STEWARD, "Herald".to_owned(), vec![ROOM_A], false)]
)]
#[case::change_inserts(
    vec![received(steward_changed_frame(steward_profile(STEWARD_A, "Scribe", &[ROOM_A, ROOM_B], false)))],
    vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_A, ROOM_B], false)]
)]
#[case::change_updates_in_place(
    vec![
        listed(STEWARD_A, "Scribe", false),
        received(steward_changed_frame(steward_profile(STEWARD_A, "Scribe", &[ROOM_B], false))),
    ],
    vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_B], false)]
)]
#[case::revocation_is_shown(
    vec![
        listed(STEWARD_A, "Scribe", false),
        received(steward_changed_frame(steward_profile(STEWARD_A, "Scribe", &[ROOM_A], true))),
    ],
    vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_A], true)]
)]
fn directory_follows_server_frames(
    #[case] events: Vec<TimelineEvent>,
    #[case] stewards: Vec<(AccountId, String, Vec<RoomId>, bool)>,
) {
    let machine = settled_machine(events);
    assert_eq!(seen(&machine), Seen { stewards, ..idle() });
}

#[rstest]
#[case::create_normalizes_name_and_rooms(
    create(NONCE_A, "  Scribe  ", vec![ROOM_B, ROOM_A, ROOM_B]),
    manage_frame(NONCE_A, StewardAction::Create, None, Some("Scribe"), &[ROOM_A, ROOM_B])
)]
#[case::create_without_rooms(
    create(NONCE_A, "Scribe", vec![]),
    manage_frame(NONCE_A, StewardAction::Create, None, Some("Scribe"), &[])
)]
#[case::rotate(
    rotate(NONCE_A, STEWARD_A),
    manage_frame(NONCE_A, StewardAction::RotateToken, Some(STEWARD_A), None, &[])
)]
#[case::revoke(
    revoke(NONCE_A, STEWARD_A),
    manage_frame(NONCE_A, StewardAction::Revoke, Some(STEWARD_A), None, &[])
)]
#[case::assign_normalizes_rooms(
    assign(NONCE_A, STEWARD_A, vec![ROOM_B, ROOM_A, ROOM_A]),
    manage_frame(NONCE_A, StewardAction::AssignRooms, Some(STEWARD_A), None, &[ROOM_A, ROOM_B])
)]
#[case::assign_nothing_clears_rooms(
    assign(NONCE_A, STEWARD_A, vec![]),
    manage_frame(NONCE_A, StewardAction::AssignRooms, Some(STEWARD_A), None, &[])
)]
fn admin_intents_become_manage_frames(#[case] event: TimelineEvent, #[case] frame: ClientFrame) {
    let (machine, frames) = step_all(settled_machine(vec![]), vec![event]);
    assert_eq!(frames, vec![frame]);
    assert_eq!(
        seen(&machine),
        Seen {
            pending: true,
            ..idle()
        }
    );
}

#[rstest]
#[case::empty_name(create(NONCE_A, "", vec![]), InputField::DisplayName)]
#[case::blank_name(create(NONCE_A, "   ", vec![]), InputField::DisplayName)]
#[case::control_character_name(create(NONCE_A, "Sc\nribe", vec![]), InputField::DisplayName)]
#[case::overlong_name(
    create(NONCE_A, &"x".repeat(DISPLAY_NAME_MAX_CHARS + 1), vec![]),
    InputField::DisplayName
)]
#[case::too_many_rooms_on_create(create(NONCE_A, "Scribe", too_many_rooms()), InputField::Rooms)]
#[case::too_many_rooms_on_assign(assign(NONCE_A, STEWARD_A, too_many_rooms()), InputField::Rooms)]
fn invalid_requests_never_leave_the_core(#[case] event: TimelineEvent, #[case] field: InputField) {
    let (machine, frames) = step_all(settled_machine(vec![]), vec![event]);
    assert!(frames.is_empty());
    assert_eq!(
        seen(&machine),
        Seen {
            error: Some(ApiError::invalid(field)),
            ..idle()
        }
    );
}

#[rstest]
#[case::create(create(NONCE_A, "Scribe", vec![ROOM_A]))]
#[case::rotate(rotate(NONCE_A, STEWARD_A))]
#[case::revoke(revoke(NONCE_A, STEWARD_A))]
#[case::assign(assign(NONCE_A, STEWARD_A, vec![ROOM_A]))]
fn offline_requests_report_unavailable(#[case] event: TimelineEvent) {
    let (machine, frames) = step_all(
        settled_machine(vec![TimelineEvent::Disconnected]),
        vec![event],
    );
    assert!(frames.is_empty());
    assert_eq!(
        seen(&machine),
        Seen {
            error: error_of(ApiErrorCode::Unavailable),
            ..idle()
        }
    );
}

#[rstest]
#[case::create_settles_on_issue(
    vec![create(NONCE_A, "Scribe", vec![ROOM_A]), received(steward_issued_frame(NONCE_A, steward_profile(STEWARD_A, "Scribe", &[ROOM_A], false)))],
    Seen {
        stewards: vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_A], false)],
        issued_for: Some(STEWARD_A),
        ..idle()
    }
)]
#[case::rotate_settles_on_issue(
    vec![rotate(NONCE_A, STEWARD_A), received(steward_issued_frame(NONCE_A, steward_profile(STEWARD_A, "Scribe", &[ROOM_A], false)))],
    Seen {
        stewards: vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_A], false)],
        issued_for: Some(STEWARD_A),
        ..idle()
    }
)]
#[case::foreign_issue_is_ignored(
    vec![create(NONCE_A, "Scribe", vec![]), received(steward_issued_frame(NONCE_B, steward_profile(STEWARD_A, "Scribe", &[], false)))],
    Seen { pending: true, ..idle() }
)]
#[case::unsolicited_issue_is_ignored(
    vec![received(steward_issued_frame(NONCE_A, steward_profile(STEWARD_A, "Scribe", &[], false)))],
    idle()
)]
#[case::rotate_is_not_settled_by_a_change(
    vec![rotate(NONCE_A, STEWARD_A), received(steward_changed_frame(steward_profile(STEWARD_A, "Scribe", &[ROOM_A], false)))],
    Seen {
        stewards: vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_A], false)],
        pending: true,
        ..idle()
    }
)]
#[case::revoke_settles_on_its_change(
    vec![revoke(NONCE_A, STEWARD_A), received(steward_changed_frame(steward_profile(STEWARD_A, "Scribe", &[ROOM_A], true)))],
    Seen {
        stewards: vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_A], true)],
        ..idle()
    }
)]
#[case::assign_settles_on_its_change(
    vec![assign(NONCE_A, STEWARD_A, vec![ROOM_B]), received(steward_changed_frame(steward_profile(STEWARD_A, "Scribe", &[ROOM_B], false)))],
    Seen {
        stewards: vec![(STEWARD_A, "Scribe".to_owned(), vec![ROOM_B], false)],
        ..idle()
    }
)]
#[case::another_stewards_change_keeps_waiting(
    vec![revoke(NONCE_A, STEWARD_A), received(steward_changed_frame(steward_profile(OTHER_STEWARD, "Herald", &[], false)))],
    Seen {
        stewards: vec![(OTHER_STEWARD, "Herald".to_owned(), vec![], false)],
        pending: true,
        ..idle()
    }
)]
#[case::a_drop_forgets_the_pending_request(
    vec![revoke(NONCE_A, STEWARD_A), TimelineEvent::Disconnected],
    idle()
)]
fn requests_settle_on_their_answers(#[case] events: Vec<TimelineEvent>, #[case] expected: Seen) {
    let (machine, _) = step_all(settled_machine(vec![]), events);
    assert_eq!(seen(&machine), expected);
}

#[rstest]
#[case::forbidden(StewardRejection::Forbidden, ApiError::new(ApiErrorCode::Forbidden))]
#[case::unknown_steward(
    StewardRejection::UnknownSteward,
    ApiError::new(ApiErrorCode::NotFound)
)]
#[case::invalid_name(
    StewardRejection::InvalidDisplayName,
    ApiError::invalid(InputField::DisplayName)
)]
#[case::invalid_rooms(StewardRejection::InvalidRooms, ApiError::invalid(InputField::Rooms))]
#[case::revoked(StewardRejection::Revoked, ApiError::new(ApiErrorCode::Conflict))]
#[case::too_many(
    StewardRejection::TooManyStewards,
    ApiError::new(ApiErrorCode::Conflict)
)]
fn rejections_map_to_errors(#[case] reason: StewardRejection, #[case] error: ApiError) {
    let (machine, _) = step_all(
        settled_machine(vec![]),
        vec![
            rotate(NONCE_A, STEWARD_A),
            received(steward_rejected_frame(
                NONCE_A,
                StewardAction::RotateToken,
                reason,
            )),
        ],
    );
    assert_eq!(
        seen(&machine),
        Seen {
            error: Some(error),
            ..idle()
        }
    );
}

#[test]
fn a_rejection_for_another_request_changes_nothing() {
    let (machine, _) = step_all(
        settled_machine(vec![]),
        vec![
            rotate(NONCE_A, STEWARD_A),
            received(steward_rejected_frame(
                NONCE_B,
                StewardAction::RotateToken,
                StewardRejection::Forbidden,
            )),
        ],
    );
    assert_eq!(
        seen(&machine),
        Seen {
            pending: true,
            ..idle()
        }
    );
}

#[rstest]
#[case::dismiss_clears_the_token(
    vec![
        create(NONCE_A, "Scribe", vec![]),
        received(steward_issued_frame(NONCE_A, steward_profile(STEWARD_A, "Scribe", &[], false))),
        steward(StewardIntent::IssuedTokenDismissed),
    ],
    None
)]
#[case::token_stays_until_dismissed(
    vec![
        create(NONCE_A, "Scribe", vec![]),
        received(steward_issued_frame(NONCE_A, steward_profile(STEWARD_A, "Scribe", &[], false))),
        steward(StewardIntent::Toggled),
        listed(STEWARD_A, "Scribe", false),
    ],
    Some(STEWARD_A)
)]
#[case::drop_keeps_the_token(
    vec![
        create(NONCE_A, "Scribe", vec![]),
        received(steward_issued_frame(NONCE_A, steward_profile(STEWARD_A, "Scribe", &[], false))),
        TimelineEvent::Disconnected,
    ],
    Some(STEWARD_A)
)]
fn the_issued_token_lives_until_dismissed(
    #[case] events: Vec<TimelineEvent>,
    #[case] issued_for: Option<AccountId>,
) {
    let (machine, _) = step_all(settled_machine(vec![]), events);
    assert_eq!(
        machine
            .steward_snapshot()
            .issued
            .map(|issued| issued.steward_id),
        issued_for
    );
}

#[test]
fn the_issued_token_carries_its_steward_and_secret() {
    let (machine, _) = step_all(
        settled_machine(vec![]),
        vec![
            create(NONCE_A, "Scribe", vec![]),
            received(steward_issued_frame(
                NONCE_A,
                steward_profile(STEWARD_A, "Scribe", &[], false),
            )),
        ],
    );
    let issued = machine.steward_snapshot().issued.expect("issued");
    assert_eq!(issued.display_name, "Scribe");
    assert_eq!(
        issued.token,
        pulse_protocol::fixtures::auth_token("pst_issued-token")
    );
}

#[rstest]
#[case::toggle_opens(vec![steward(StewardIntent::Toggled)], true)]
#[case::toggle_twice_closes(vec![steward(StewardIntent::Toggled), steward(StewardIntent::Toggled)], false)]
fn toggling_flips_visibility(#[case] events: Vec<TimelineEvent>, #[case] open: bool) {
    let (machine, frames) = step_all(settled_machine(vec![]), events);
    assert!(frames.is_empty());
    assert_eq!(seen(&machine), Seen { open, ..idle() });
}

#[test]
fn toggling_clears_a_stale_error() {
    let (machine, _) = step_all(
        settled_machine(vec![]),
        vec![create(NONCE_A, "", vec![]), steward(StewardIntent::Toggled)],
    );
    assert_eq!(
        seen(&machine),
        Seen {
            open: true,
            ..idle()
        }
    );
}

#[rstest]
#[case::unknown_author(ACCOUNT_B, vec![], false)]
#[case::listed_steward(STEWARD_A, vec![listed(STEWARD_A, "Scribe", false)], true)]
#[case::listed_other_account(ACCOUNT_A, vec![listed(STEWARD_A, "Scribe", false)], false)]
#[case::observed_through_a_stream(STEWARD_A, vec![stream_event(STREAM_A, StreamPhase::Begin, 0, "x")], true)]
fn messages_of_stewards_are_flagged(
    #[case] author: AccountId,
    #[case] before: Vec<TimelineEvent>,
    #[case] flagged: bool,
) {
    let message = pulse_protocol::RoomMessage {
        author_id: author,
        ..steward_message(ROOM_A, 1, STREAM_A, "hi")
    };
    let machine = settled_machine([before, vec![received(created_frame(message))]].concat());
    let entries = machine.snapshot(Some(ROOM_A)).entries;
    let confirmed = entries
        .iter()
        .find(|entry| entry.message_id.is_some())
        .expect("confirmed entry");
    assert_eq!(confirmed.author_is_steward, flagged);
}

#[rstest]
#[case::owner(Role::Owner, true)]
#[case::admin(Role::Admin, true)]
#[case::member(Role::Member, false)]
#[case::guest(Role::Guest, false)]
#[case::steward(Role::Steward, false)]
fn managing_is_for_owners_and_admins(#[case] role: Role, #[case] can_manage: bool) {
    assert_eq!(
        connected_as(role).snapshot().stewards.can_manage,
        can_manage
    );
}
