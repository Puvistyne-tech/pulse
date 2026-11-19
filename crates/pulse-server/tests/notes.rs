mod common;

use std::time::Duration;

use axum::http::{Method, StatusCode};
use common::{
    ClientSocket, api, campus_for, expect_frame, frames_until_pong, fresh_state, grant_role,
    issue_steward, joined_socket, mention_rows, mentioning, next_of, posted, rooms_of, send,
    signed_up, spawn_server_with, sync_rooms,
};
use futures_util::future::join_all;
use pulse_protocol::{
    AccountId, AuthSession, CampusId, ClientNonce, Note, NoteList, Role, RoomId, ServerFrame,
    StartNoteRequest, api_path,
};
use pulse_server::AppState;
use rstest::rstest;
use serde_json::{Value, json};
use tokio::time::timeout;

const CONCURRENT_STARTS: usize = 8;
const DEADLINE: Duration = Duration::from_secs(30);

struct Cast {
    state: AppState,
    campus_id: CampusId,
    rooms: Vec<RoomId>,
    ada: AuthSession,
    grace: AuthSession,
    linus: AuthSession,
    alan: AuthSession,
    stranger: AuthSession,
}

impl Cast {
    async fn open() -> Self {
        let state = fresh_state().await;
        let ada = signed_up(&state, "ada@example.org").await;
        let grace = signed_up(&state, "grace@example.org").await;
        let linus = signed_up(&state, "linus@example.org").await;
        let alan = signed_up(&state, "alan@example.org").await;
        let stranger = signed_up(&state, "stranger@example.org").await;
        let created = campus_for(&state, &ada.token, "north-quad").await;
        let campus_id = created.campus.campus_id;
        grant_role(&state, campus_id, grace.account.account_id, Role::Member).await;
        grant_role(&state, campus_id, linus.account.account_id, Role::Member).await;
        grant_role(&state, campus_id, alan.account.account_id, Role::Guest).await;
        Self {
            state,
            campus_id,
            rooms: created.rooms.iter().map(|room| room.room_id).collect(),
            ada,
            grace,
            linus,
            alan,
            stranger,
        }
    }

    async fn start(&self, starter: &AuthSession, with: &[AccountId]) -> (StatusCode, Value) {
        let request = StartNoteRequest {
            account_ids: with.to_vec(),
        };
        api(
            &self.state,
            Method::POST,
            &api_path::campus_notes(self.campus_id),
            Some(&starter.token),
            Some(&request),
        )
        .await
    }

    async fn started(&self, starter: &AuthSession, with: &[AccountId]) -> Note {
        let (status, body) = self.start(starter, with).await;
        assert!(status.is_success(), "{status} {body}");
        serde_json::from_value(body).expect("note")
    }

    async fn notes_of(&self, session: &AuthSession) -> Vec<RoomId> {
        let nothing: Option<&()> = None;
        let (status, body) = api(
            &self.state,
            Method::GET,
            &api_path::campus_notes(self.campus_id),
            Some(&session.token),
            nothing,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let list: NoteList = serde_json::from_value(body).expect("note list");
        list.notes.iter().map(|note| note.room_id).collect()
    }
}

fn id(session: &AuthSession) -> AccountId {
    session.account.account_id
}

#[tokio::test]
async fn starting_the_same_set_twice_reuses_one_note_visible_only_to_its_members() {
    let cast = Cast::open().await;
    let (status, body) = cast.start(&cast.ada, &[id(&cast.grace)]).await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    let first: Note = serde_json::from_value(body).expect("note");
    let (again_status, again) = cast
        .start(&cast.grace, &[id(&cast.ada), id(&cast.grace)])
        .await;
    assert_eq!(again_status, StatusCode::OK, "{again}");
    assert_eq!(again["room_id"], json!(first.room_id.to_string()));
    let mut members: Vec<AccountId> = first
        .members
        .iter()
        .map(|member| member.account_id)
        .collect();
    members.sort_by_key(ToString::to_string);
    let mut expected = vec![id(&cast.ada), id(&cast.grace)];
    expected.sort_by_key(ToString::to_string);
    assert_eq!(members, expected);
    assert_eq!(cast.notes_of(&cast.ada).await, vec![first.room_id]);
    assert_eq!(cast.notes_of(&cast.grace).await, vec![first.room_id]);
    assert!(cast.notes_of(&cast.linus).await.is_empty());
    let listed = rooms_of(&cast.state, &cast.ada.token, cast.campus_id).await;
    assert_eq!(listed.rooms.len(), cast.rooms.len());
}

#[derive(Clone, Copy, Debug)]
enum Pick {
    Nobody,
    OnlySelf,
    Stranger,
    Steward,
    TooMany,
}

#[rstest]
#[case::nobody(Pick::Nobody, StatusCode::BAD_REQUEST)]
#[case::only_self(Pick::OnlySelf, StatusCode::BAD_REQUEST)]
#[case::stranger(Pick::Stranger, StatusCode::BAD_REQUEST)]
#[case::steward(Pick::Steward, StatusCode::BAD_REQUEST)]
#[case::too_many(Pick::TooMany, StatusCode::BAD_REQUEST)]
#[tokio::test]
async fn note_members_must_be_campus_people(#[case] pick: Pick, #[case] status: StatusCode) {
    let cast = Cast::open().await;
    let (steward, _) = issue_steward(
        &cast.state,
        cast.campus_id,
        id(&cast.ada),
        "Scribe",
        &cast.rooms,
    )
    .await;
    let with = match pick {
        Pick::Nobody => vec![],
        Pick::OnlySelf => vec![id(&cast.ada)],
        Pick::Stranger => vec![id(&cast.stranger)],
        Pick::Steward => vec![steward.steward_id],
        Pick::TooMany => (0..10).map(|_| AccountId::generate()).collect(),
    };
    let (actual, body) = cast.start(&cast.ada, &with).await;
    assert_eq!(actual, status, "{body}");
    assert_eq!(
        body,
        json!({"code": "invalid_input", "field": "note_members"})
    );
    assert!(cast.notes_of(&cast.ada).await.is_empty());
}

#[tokio::test]
async fn guests_cannot_start_notes() {
    let cast = Cast::open().await;
    let (status, body) = cast.start(&cast.alan, &[id(&cast.ada)]).await;
    assert_eq!(status, StatusCode::FORBIDDEN, "{body}");
}

async fn synced(socket: &mut ClientSocket, room_id: RoomId) -> ServerFrame {
    send(socket, &sync_rooms(&[(room_id, None)])).await;
    expect_frame(socket).await
}

#[tokio::test]
async fn note_messages_reach_members_only() {
    timeout(DEADLINE, async {
        let cast = Cast::open().await;
        let addr = spawn_server_with(cast.state.clone()).await;
        let note = cast.started(&cast.ada, &[id(&cast.grace)]).await;
        let (mut ada, _) = joined_socket(addr, cast.campus_id, &cast.ada.token).await;
        let (mut grace, _) = joined_socket(addr, cast.campus_id, &cast.grace.token).await;
        let (mut linus, _) = joined_socket(addr, cast.campus_id, &cast.linus.token).await;
        assert!(matches!(
            synced(&mut ada, note.room_id).await,
            ServerFrame::RoomSynced(_)
        ));
        assert!(matches!(
            synced(&mut grace, note.room_id).await,
            ServerFrame::RoomSynced(_)
        ));
        assert!(matches!(
            synced(&mut linus, note.room_id).await,
            ServerFrame::RoomUnavailable(_)
        ));
        let body = format!(
            "{} {}",
            mentioning(id(&cast.linus), "psst"),
            mentioning(id(&cast.grace), "hi")
        );
        let message = posted(&mut ada, note.room_id, ClientNonce::generate(), &body).await;
        next_of(&mut grace, |frame| match frame {
            ServerFrame::MessageCreated(created)
                if created.message.message_id == message.message_id =>
            {
                Some(())
            }
            _ => None,
        })
        .await;
        let leaked = frames_until_pong(&mut linus, 2).await;
        assert!(leaked.iter().all(|frame| !matches!(
            frame,
            ServerFrame::MessageCreated(_) | ServerFrame::ThreadChanged(_)
        )));
        assert_eq!(
            mention_rows(&cast.state, cast.campus_id, message.message_id).await,
            vec![id(&cast.grace)]
        );
    })
    .await
    .expect("note delivery settles");
}

#[tokio::test]
async fn new_notes_are_announced_to_their_members() {
    timeout(DEADLINE, async {
        let cast = Cast::open().await;
        let addr = spawn_server_with(cast.state.clone()).await;
        let (mut grace, _) = joined_socket(addr, cast.campus_id, &cast.grace.token).await;
        let (mut linus, _) = joined_socket(addr, cast.campus_id, &cast.linus.token).await;
        let note = cast.started(&cast.ada, &[id(&cast.grace)]).await;
        next_of(&mut grace, |frame| match frame {
            ServerFrame::NoteStarted(started) if started.note.room_id == note.room_id => Some(()),
            _ => None,
        })
        .await;
        let unrelated = frames_until_pong(&mut linus, 2).await;
        assert!(
            unrelated
                .iter()
                .all(|frame| !matches!(frame, ServerFrame::NoteStarted(_)))
        );
    })
    .await
    .expect("announcement settles");
}

#[tokio::test]
async fn concurrent_starts_of_one_set_share_a_single_note() {
    let cast = Cast::open().await;
    let with = [id(&cast.grace), id(&cast.linus)];
    let attempts = (0..CONCURRENT_STARTS).map(|_| cast.start(&cast.ada, &with));
    let outcomes = timeout(DEADLINE, join_all(attempts))
        .await
        .expect("starts settle");
    let rooms: Vec<Value> = outcomes
        .iter()
        .map(|(status, body)| {
            assert!(status.is_success(), "{status} {body}");
            body["room_id"].clone()
        })
        .collect();
    assert!(rooms.iter().all(|room| *room == rooms[0]));
    assert_eq!(
        outcomes
            .iter()
            .filter(|(status, _)| *status == StatusCode::CREATED)
            .count(),
        1
    );
    assert_eq!(
        common::count_rows(&cast.state, "direct_notes", cast.campus_id).await,
        1
    );
    assert_eq!(
        common::count_rows(&cast.state, "direct_note_members", cast.campus_id).await,
        3
    );
}
