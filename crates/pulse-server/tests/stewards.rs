mod common;

use std::collections::BTreeSet;

use axum::http::StatusCode;
use common::{
    ClientSocket, Managed, Observed, SCENARIO_DEADLINE, Stage, api_get, campus_with_owner,
    create_steward_request, handshake_outcome, issue_steward, joined_socket, manage_as,
    manage_frame, next_of, send, sibling_state, spawn_server_with, steward_request,
    steward_room_rows, steward_row, sync_stewards_frame,
};
use pulse_protocol::{
    AccountId, AuthToken, ClientNonce, HandshakeRejected, HandshakeRejection, Role, RoomId,
    STEWARD_ROOMS_MAX, STEWARD_TOKEN_PREFIX, STEWARDS_PER_CAMPUS_MAX, ServerFrame, StewardAction,
    StewardChanged, StewardIssued, StewardProfile, StewardRejection, api_path,
};
use pulse_server::auth::token_digest;
use pulse_server::steward::{
    StewardFailure, StewardOutcome, manage_steward, resolve_steward_token,
};
use rstest::rstest;
use tokio::task::JoinSet;
use tokio::time::timeout;

async fn issued_over_socket(
    socket: &mut ClientSocket,
    request: pulse_protocol::ManageSteward,
) -> Result<StewardIssued, pulse_protocol::StewardRejected> {
    match manage_on(socket, request).await {
        Managed::Issued(issued) => Ok(*issued),
        Managed::Rejected(rejected) => Err(rejected),
        Managed::Changed(_) => panic!("creation never answers with a bare change"),
    }
}

async fn changed_of(socket: &mut ClientSocket, steward_id: AccountId) -> StewardProfile {
    next_of(socket, |frame| match frame {
        ServerFrame::StewardChanged(StewardChanged { steward })
            if steward.steward_id == steward_id =>
        {
            Some(steward.clone())
        }
        _ => None,
    })
    .await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn create_issues_one_token_and_stores_only_its_digest() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (mut owner, _) =
            joined_socket(stage.addr, stage.campus.id, &stage.campus.owner.token).await;
        let issued =
            issued_over_socket(&mut owner, create_steward_request("Scribe", &[stage.room]))
                .await
                .expect("issued");
        assert!(issued.token.0.starts_with(STEWARD_TOKEN_PREFIX));
        assert_eq!(issued.steward.display_name, "Scribe");
        assert_eq!(issued.steward.room_ids, vec![stage.room]);
        assert_eq!(issued.steward.credential_revision, 1);
        assert!(!issued.steward.revoked);
        let row = steward_row(&stage.state, stage.campus.id, issued.steward.steward_id)
            .await
            .expect("row");
        assert_eq!(row.token_hash, token_digest(&issued.token));
        assert_ne!(row.token_hash, issued.token.0.as_bytes());
        assert_eq!(row.credential_revision, 1);
        let changed = changed_of(&mut owner, issued.steward.steward_id).await;
        assert_eq!(changed, issued.steward);
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::blank_name("   ", 0, StewardRejection::InvalidDisplayName)]
#[case::empty_name("", 0, StewardRejection::InvalidDisplayName)]
#[case::too_many_rooms("Scribe", STEWARD_ROOMS_MAX + 1, StewardRejection::InvalidRooms)]
#[case::unknown_room("Scribe", 1, StewardRejection::InvalidRooms)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn invalid_creations_are_rejected_without_rows(
    #[case] name: &str,
    #[case] unknown_rooms: usize,
    #[case] reason: StewardRejection,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (mut owner, _) =
            joined_socket(stage.addr, stage.campus.id, &stage.campus.owner.token).await;
        let mut rooms = vec![stage.room];
        if unknown_rooms > 0 {
            rooms = (0..unknown_rooms).map(|_| RoomId::generate()).collect();
        }
        let request = create_steward_request(name, &rooms);
        let nonce = request.request_nonce;
        let rejected = issued_over_socket(&mut owner, request)
            .await
            .expect_err("rejected");
        assert_eq!(rejected.request_nonce, nonce);
        assert_eq!(rejected.action, StewardAction::Create);
        assert_eq!(rejected.reason, reason);
        let listed = pulse_server::steward::steward_profiles(&stage.state.db, stage.campus.id)
            .await
            .expect("profiles");
        assert!(listed.is_empty());
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn the_campus_cap_counts_only_active_stewards() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let owner = stage.campus.owner.account.account_id;
        let mut first = None;
        for index in 0..STEWARDS_PER_CAMPUS_MAX {
            let (profile, _) = issue_steward(
                &stage.state,
                stage.campus.id,
                owner,
                &format!("Steward {index}"),
                &[],
            )
            .await;
            first.get_or_insert(profile.steward_id);
        }
        let over = manage_steward(
            &stage.state.db,
            stage.campus.id,
            owner,
            create_steward_request("One too many", &[]),
        )
        .await;
        assert!(matches!(
            over,
            Err(StewardFailure::Rejected(StewardRejection::TooManyStewards))
        ));
        manage_as(
            &stage.state,
            stage.campus.id,
            owner,
            steward_request(StewardAction::Revoke, first.expect("first"), &[]),
        )
        .await;
        issue_steward(&stage.state, stage.campus.id, owner, "Replacement", &[]).await;
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::rotate(StewardAction::RotateToken)]
#[case::revoke(StewardAction::Revoke)]
#[case::assign(StewardAction::AssignRooms)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn unknown_stewards_are_rejected_and_foreign_campus_stewards_stay_untouched(
    #[case] action: StewardAction,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let other = campus_with_owner(&stage.state, "other@example.org", "south-quad").await;
        let (foreign, foreign_token) = issue_steward(
            &stage.state,
            other.id,
            other.owner.account.account_id,
            "Foreign",
            &[],
        )
        .await;
        let outcome = manage_steward(
            &stage.state.db,
            stage.campus.id,
            stage.campus.owner.account.account_id,
            steward_request(action, foreign.steward_id, &[]),
        )
        .await;
        assert!(matches!(
            outcome,
            Err(StewardFailure::Rejected(StewardRejection::UnknownSteward))
        ));
        let row = steward_row(&stage.state, other.id, foreign.steward_id)
            .await
            .expect("row");
        assert_eq!(row.token_hash, token_digest(&foreign_token));
        assert_eq!(row.credential_revision, 1);
        assert!(!row.revoked);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn rotating_replaces_the_token_and_bumps_the_revision() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let owner = stage.campus.owner.account.account_id;
        let (profile, old) = issue_steward(
            &stage.state,
            stage.campus.id,
            owner,
            "Scribe",
            &[stage.room],
        )
        .await;
        let StewardOutcome::Issued(rotated, fresh) = manage_as(
            &stage.state,
            stage.campus.id,
            owner,
            steward_request(StewardAction::RotateToken, profile.steward_id, &[]),
        )
        .await
        else {
            panic!("rotation must issue a token");
        };
        assert_eq!(rotated.credential_revision, 2);
        assert_eq!(rotated.room_ids, vec![stage.room]);
        assert_ne!(fresh, old);
        let row = steward_row(&stage.state, stage.campus.id, profile.steward_id)
            .await
            .expect("row");
        assert_eq!(row.token_hash, token_digest(&fresh));
        assert!(
            resolve_steward_token(&stage.state.db, stage.campus.id, &old)
                .await
                .expect("resolve")
                .is_none()
        );
        let credential = resolve_steward_token(&stage.state.db, stage.campus.id, &fresh)
            .await
            .expect("resolve")
            .expect("fresh token resolves");
        assert_eq!(credential.steward_id, profile.steward_id);
        assert_eq!(credential.credential_revision, 2);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn revoked_stewards_cannot_rotate_or_be_reassigned() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let owner = stage.campus.owner.account.account_id;
        let (profile, _) = issue_steward(&stage.state, stage.campus.id, owner, "Scribe", &[]).await;
        let StewardOutcome::Changed(revoked) = manage_as(
            &stage.state,
            stage.campus.id,
            owner,
            steward_request(StewardAction::Revoke, profile.steward_id, &[]),
        )
        .await
        else {
            panic!("revocation must change the profile");
        };
        assert!(revoked.revoked);
        assert_eq!(revoked.credential_revision, 2);
        for action in [StewardAction::RotateToken, StewardAction::AssignRooms] {
            let outcome = manage_steward(
                &stage.state.db,
                stage.campus.id,
                owner,
                steward_request(action, profile.steward_id, &[stage.room]),
            )
            .await;
            assert!(matches!(
                outcome,
                Err(StewardFailure::Rejected(StewardRejection::Revoked))
            ));
        }
        let again = manage_as(
            &stage.state,
            stage.campus.id,
            owner,
            steward_request(StewardAction::Revoke, profile.steward_id, &[]),
        )
        .await;
        assert_eq!(again, StewardOutcome::Changed(revoked));
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn assigning_replaces_rooms_without_touching_the_credential() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let owner = stage.campus.owner.account.account_id;
        let rooms = stage.campus.rooms.clone();
        let (profile, token) =
            issue_steward(&stage.state, stage.campus.id, owner, "Scribe", &rooms[..1]).await;
        let mut expected = vec![rooms[1], rooms[0], rooms[1]];
        let StewardOutcome::Changed(changed) = manage_as(
            &stage.state,
            stage.campus.id,
            owner,
            steward_request(StewardAction::AssignRooms, profile.steward_id, &expected),
        )
        .await
        else {
            panic!("assignment must change the profile");
        };
        expected.sort();
        expected.dedup();
        assert_eq!(changed.room_ids, expected);
        assert_eq!(changed.credential_revision, 1);
        assert_eq!(
            steward_room_rows(&stage.state, stage.campus.id, profile.steward_id).await,
            expected
        );
        let row = steward_row(&stage.state, stage.campus.id, profile.steward_id)
            .await
            .expect("row");
        assert_eq!(row.token_hash, token_digest(&token));
        let cleared = manage_as(
            &stage.state,
            stage.campus.id,
            owner,
            steward_request(StewardAction::AssignRooms, profile.steward_id, &[]),
        )
        .await;
        assert_eq!(
            cleared,
            StewardOutcome::Changed(StewardProfile {
                room_ids: Vec::new(),
                ..changed
            })
        );
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::foreign_room(true)]
#[case::unknown_room(false)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn assignments_only_accept_rooms_of_the_same_campus(#[case] foreign: bool) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let other = campus_with_owner(&stage.state, "other@example.org", "south-quad").await;
        let owner = stage.campus.owner.account.account_id;
        let (profile, _) = issue_steward(
            &stage.state,
            stage.campus.id,
            owner,
            "Scribe",
            &[stage.room],
        )
        .await;
        let target = if foreign {
            other.rooms[0]
        } else {
            RoomId::generate()
        };
        let outcome = manage_steward(
            &stage.state.db,
            stage.campus.id,
            owner,
            steward_request(StewardAction::AssignRooms, profile.steward_id, &[target]),
        )
        .await;
        assert!(matches!(
            outcome,
            Err(StewardFailure::Rejected(StewardRejection::InvalidRooms))
        ));
        assert_eq!(
            steward_room_rows(&stage.state, stage.campus.id, profile.steward_id).await,
            vec![stage.room]
        );
    })
    .await
    .expect("scenario deadline");
}

#[derive(Clone, Copy, Debug)]
enum Presented {
    Active,
    RotatedOut,
    Revoked,
    WrongCampus,
    Unknown,
    Truncated,
}

#[rstest]
#[case::active(Presented::Active, None)]
#[case::rotated_out(Presented::RotatedOut, Some(HandshakeRejection::Unauthenticated))]
#[case::revoked(Presented::Revoked, Some(HandshakeRejection::Unauthenticated))]
#[case::wrong_campus(Presented::WrongCampus, Some(HandshakeRejection::Unauthenticated))]
#[case::unknown(Presented::Unknown, Some(HandshakeRejection::Unauthenticated))]
#[case::truncated(Presented::Truncated, Some(HandshakeRejection::Unauthenticated))]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn handshake_honours_only_the_current_token_of_its_campus(
    #[case] presented: Presented,
    #[case] rejection: Option<HandshakeRejection>,
) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let other = campus_with_owner(&stage.state, "other@example.org", "south-quad").await;
        let owner = stage.campus.owner.account.account_id;
        let (profile, original) = issue_steward(
            &stage.state,
            stage.campus.id,
            owner,
            "Scribe",
            &[stage.room],
        )
        .await;
        let StewardOutcome::Issued(_, current) = manage_as(
            &stage.state,
            stage.campus.id,
            owner,
            steward_request(StewardAction::RotateToken, profile.steward_id, &[]),
        )
        .await
        else {
            panic!("rotation must issue a token");
        };
        let (campus_id, token) = match presented {
            Presented::Active => (stage.campus.id, current),
            Presented::RotatedOut => (stage.campus.id, original),
            Presented::Revoked => {
                manage_as(
                    &stage.state,
                    stage.campus.id,
                    owner,
                    steward_request(StewardAction::Revoke, profile.steward_id, &[]),
                )
                .await;
                (stage.campus.id, current)
            }
            Presented::WrongCampus => (other.id, current),
            Presented::Unknown => (stage.campus.id, AuthToken("pst_never-issued".to_owned())),
            Presented::Truncated => (
                stage.campus.id,
                AuthToken(current.0[..STEWARD_TOKEN_PREFIX.len() + 4].to_owned()),
            ),
        };
        let outcome = handshake_outcome(stage.addr, campus_id, &token).await;
        match rejection {
            None => match outcome {
                Observed::Frame(ServerFrame::Welcome(welcome)) => {
                    assert_eq!(welcome.account_id, profile.steward_id);
                    assert_eq!(welcome.role, Role::Steward);
                    assert_eq!(welcome.campus_id, stage.campus.id);
                }
                other => panic!("expected welcome, got {other:?}"),
            },
            Some(reason) => assert_eq!(
                outcome,
                Observed::Frame(ServerFrame::HandshakeRejected(HandshakeRejected { reason }))
            ),
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn steward_tokens_are_refused_on_http() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let (_, token) = issue_steward(
            &stage.state,
            stage.campus.id,
            stage.campus.owner.account.account_id,
            "Scribe",
            &[stage.room],
        )
        .await;
        for path in [
            api_path::CAMPUSES.to_owned(),
            api_path::campus_rooms(stage.campus.id),
        ] {
            let (status, _) = api_get(&stage.state, &path, Some(&token)).await;
            assert_eq!(status, StatusCode::UNAUTHORIZED, "{path}");
        }
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn changes_fan_out_to_every_member_of_the_campus_only() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let other = campus_with_owner(&stage.state, "other@example.org", "south-quad").await;
        let (mut owner, _) =
            joined_socket(stage.addr, stage.campus.id, &stage.campus.owner.token).await;
        let (mut ada, _) = joined_socket(stage.addr, stage.campus.id, &stage.ada.token).await;
        let (mut guest, _) = joined_socket(stage.addr, stage.campus.id, &stage.guest.token).await;
        let (mut outsider, _) = joined_socket(stage.addr, other.id, &other.owner.token).await;
        let issued =
            issued_over_socket(&mut owner, create_steward_request("Scribe", &[stage.room]))
                .await
                .expect("issued");
        for socket in [&mut owner, &mut ada, &mut guest] {
            assert_eq!(
                changed_of(socket, issued.steward.steward_id).await,
                issued.steward
            );
        }
        let own_nonce = ClientNonce::generate();
        send(
            &mut outsider,
            &common::send_message(other.rooms[0], own_nonce, "ping"),
        )
        .await;
        let seen = common::frames_until_pong(&mut outsider, 3).await;
        assert!(
            seen.iter()
                .all(|frame| !matches!(frame, ServerFrame::StewardChanged(_)))
        );
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn members_list_the_directory_over_a_socket() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let owner = stage.campus.owner.account.account_id;
        let (first, _) = issue_steward(
            &stage.state,
            stage.campus.id,
            owner,
            "Scribe",
            &[stage.room],
        )
        .await;
        let (second, _) = issue_steward(&stage.state, stage.campus.id, owner, "Herald", &[]).await;
        manage_as(
            &stage.state,
            stage.campus.id,
            owner,
            steward_request(StewardAction::Revoke, second.steward_id, &[]),
        )
        .await;
        let (mut ada, _) = joined_socket(stage.addr, stage.campus.id, &stage.ada.token).await;
        send(&mut ada, &sync_stewards_frame()).await;
        let listed = next_of(&mut ada, |frame| match frame {
            ServerFrame::StewardsSynced(synced) => Some(synced.stewards.clone()),
            _ => None,
        })
        .await;
        let ids = listed
            .iter()
            .map(|profile| (profile.steward_id, profile.revoked))
            .collect::<BTreeSet<_>>();
        assert_eq!(
            ids,
            BTreeSet::from([(first.steward_id, false), (second.steward_id, true)])
        );
    })
    .await
    .expect("scenario deadline");
}

#[rstest]
#[case::four(4)]
#[case::eight(8)]
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_rotations_leave_exactly_one_valid_token(#[case] attempts: usize) {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let owner = stage.campus.owner.account.account_id;
        let (profile, original) = issue_steward(
            &stage.state,
            stage.campus.id,
            owner,
            "Scribe",
            &[stage.room],
        )
        .await;
        let mut rotating = JoinSet::new();
        for _ in 0..attempts {
            let state = sibling_state(&stage.state);
            let campus_id = stage.campus.id;
            let steward_id = profile.steward_id;
            rotating.spawn(async move {
                manage_as(
                    &state,
                    campus_id,
                    owner,
                    steward_request(StewardAction::RotateToken, steward_id, &[]),
                )
                .await
            });
        }
        let mut tokens = vec![original];
        let mut revisions = BTreeSet::new();
        while let Some(done) = rotating.join_next().await {
            match done.expect("rotation task") {
                StewardOutcome::Issued(rotated, token) => {
                    revisions.insert(rotated.credential_revision);
                    tokens.push(token);
                }
                other => panic!("expected an issued token, got {other:?}"),
            }
        }
        let expected =
            (2..=u64::try_from(attempts).expect("attempts") + 1).collect::<BTreeSet<_>>();
        assert_eq!(revisions, expected);
        let row = steward_row(&stage.state, stage.campus.id, profile.steward_id)
            .await
            .expect("row");
        assert_eq!(
            row.credential_revision,
            i64::try_from(attempts).expect("attempts") + 1
        );
        let mut valid = 0;
        for token in &tokens {
            let resolved = resolve_steward_token(&stage.state.db, stage.campus.id, token)
                .await
                .expect("resolve");
            if resolved.is_some() {
                valid += 1;
                assert_eq!(row.token_hash, token_digest(token));
            }
        }
        assert_eq!(valid, 1);
    })
    .await
    .expect("scenario deadline");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn campus_members_without_the_permission_cannot_manage() {
    timeout(SCENARIO_DEADLINE, async {
        let stage = Stage::open().await;
        let addr = spawn_server_with(sibling_state(&stage.state)).await;
        let (mut ada, _) = joined_socket(addr, stage.campus.id, &stage.ada.token).await;
        let rejected =
            issued_over_socket(&mut ada, create_steward_request("Scribe", &[stage.room]))
                .await
                .expect_err("rejected");
        assert_eq!(rejected.reason, StewardRejection::Forbidden);
        assert!(
            pulse_server::steward::steward_profiles(&stage.state.db, stage.campus.id)
                .await
                .expect("profiles")
                .is_empty()
        );
    })
    .await
    .expect("scenario deadline");
}
