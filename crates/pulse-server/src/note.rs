use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use pulse_protocol::{
    AccountId, ApiError, CampusId, InputField, NOTE_SLUG_PREFIX, Note, NoteList, NoteMember,
    NoteStarted, Permission, Role, RoomId, ServerFrame, StartNoteRequest, Validate,
    normalize_note_members, note_member_key,
};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::auth::Authenticated;
use crate::campus::{parse_campus_id, require_permission};
use crate::error::{ApiFailure, ApiJson, is_unique_violation};
use crate::publish::publish_frame;
use crate::state::AppState;

pub async fn room_accessible(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    room_id: RoomId,
    account_id: AccountId,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        "SELECT NOT EXISTS (SELECT 1 FROM direct_notes WHERE campus_id = $1 AND room_id = $2) \
         OR EXISTS (SELECT 1 FROM direct_note_members \
                    WHERE campus_id = $1 AND room_id = $2 AND account_id = $3)",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .bind(account_id.as_uuid())
    .fetch_one(executor)
    .await
}

fn invalid_members() -> ApiFailure {
    ApiFailure(ApiError::invalid(InputField::NoteMembers))
}

async fn notes_where(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    filter: &str,
    key: Uuid,
) -> Result<Vec<Note>, sqlx::Error> {
    let query = format!(
        "SELECT n.room_id, a.account_id, a.display_name FROM direct_notes n \
         JOIN direct_note_members m ON m.campus_id = n.campus_id AND m.room_id = n.room_id \
         JOIN accounts a ON a.account_id = m.account_id \
         WHERE n.campus_id = $1 AND {filter} \
         ORDER BY n.created_at, n.room_id, lower(a.display_name), a.account_id"
    );
    let rows = sqlx::query_as::<_, (Uuid, Uuid, String)>(sqlx::AssertSqlSafe(query))
        .bind(campus_id.as_uuid())
        .bind(key)
        .fetch_all(executor)
        .await?;
    let mut order: Vec<RoomId> = Vec::new();
    let mut members: HashMap<RoomId, Vec<NoteMember>> = HashMap::new();
    for (room_id, account_id, display_name) in rows {
        let room_id = RoomId(room_id);
        if !members.contains_key(&room_id) {
            order.push(room_id);
        }
        members.entry(room_id).or_default().push(NoteMember {
            account_id: AccountId(account_id),
            display_name,
        });
    }
    Ok(order
        .into_iter()
        .map(|room_id| Note {
            campus_id,
            room_id,
            members: members.remove(&room_id).unwrap_or_default(),
        })
        .collect())
}

const MEMBER_OF: &str = "EXISTS (SELECT 1 FROM direct_note_members mine \
     WHERE mine.campus_id = n.campus_id AND mine.room_id = n.room_id AND mine.account_id = $2)";
const ROOM_IS: &str = "n.room_id = $2";

pub async fn list_notes(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path(raw_campus_id): Path<String>,
) -> Result<Json<NoteList>, ApiFailure> {
    let campus_id = parse_campus_id(&raw_campus_id)?;
    require_permission(
        &state.db,
        campus_id,
        authenticated.account_id,
        Permission::ViewRooms,
    )
    .await?;
    let notes = notes_where(
        &state.db,
        campus_id,
        MEMBER_OF,
        authenticated.account_id.as_uuid(),
    )
    .await?;
    Ok(Json(NoteList { campus_id, notes }))
}

async fn eligible_count(
    connection: &mut PgConnection,
    campus_id: CampusId,
    members: &[AccountId],
) -> Result<usize, sqlx::Error> {
    let ids: Vec<Uuid> = members
        .iter()
        .map(|account_id| account_id.as_uuid())
        .collect();
    let found = sqlx::query_scalar::<_, i64>(
        "SELECT count(*) FROM campus_members WHERE campus_id = $1 AND account_id = ANY($2) \
         AND role <> $3",
    )
    .bind(campus_id.as_uuid())
    .bind(&ids)
    .bind(Role::Steward.as_str())
    .fetch_one(connection)
    .await?;
    Ok(usize::try_from(found).unwrap_or(0))
}

async fn existing_note(
    connection: &mut PgConnection,
    campus_id: CampusId,
    member_key: &str,
) -> Result<Option<RoomId>, sqlx::Error> {
    let room_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT room_id FROM direct_notes WHERE campus_id = $1 AND member_key = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(member_key)
    .fetch_optional(connection)
    .await?;
    Ok(room_id.map(RoomId))
}

async fn enroll(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    members: &[AccountId],
) -> Result<(), sqlx::Error> {
    for account_id in members {
        sqlx::query(
            "INSERT INTO direct_note_members (campus_id, room_id, account_id) VALUES ($1, $2, $3) \
             ON CONFLICT (campus_id, room_id, account_id) DO NOTHING",
        )
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(account_id.as_uuid())
        .execute(&mut *connection)
        .await?;
    }
    Ok(())
}

async fn insert_note(
    connection: &mut PgConnection,
    campus_id: CampusId,
    member_key: &str,
    starter: AccountId,
) -> Result<RoomId, sqlx::Error> {
    let room_id = RoomId::generate();
    sqlx::query("INSERT INTO rooms (campus_id, room_id, slug) VALUES ($1, $2, $3)")
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(format!("{NOTE_SLUG_PREFIX}{}", room_id.as_uuid().simple()))
        .execute(&mut *connection)
        .await?;
    sqlx::query(
        "INSERT INTO direct_notes (campus_id, room_id, member_key, created_by) \
         VALUES ($1, $2, $3, $4)",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .bind(member_key)
    .bind(starter.as_uuid())
    .execute(&mut *connection)
    .await?;
    Ok(room_id)
}

async fn settle_note(
    state: &AppState,
    campus_id: CampusId,
    member_key: &str,
    starter: AccountId,
    members: &[AccountId],
) -> Result<(RoomId, bool), sqlx::Error> {
    let mut transaction = state.db.begin().await?;
    if let Some(room_id) = existing_note(&mut transaction, campus_id, member_key).await? {
        enroll(&mut transaction, campus_id, room_id, members).await?;
        transaction.commit().await?;
        return Ok((room_id, false));
    }
    match insert_note(&mut transaction, campus_id, member_key, starter).await {
        Ok(room_id) => {
            enroll(&mut transaction, campus_id, room_id, members).await?;
            transaction.commit().await?;
            Ok((room_id, true))
        }
        Err(error) if is_unique_violation(&error) => {
            transaction.rollback().await?;
            let mut retry = state.db.begin().await?;
            let room_id = existing_note(&mut retry, campus_id, member_key)
                .await?
                .ok_or(sqlx::Error::RowNotFound)?;
            enroll(&mut retry, campus_id, room_id, members).await?;
            retry.commit().await?;
            Ok((room_id, false))
        }
        Err(error) => Err(error),
    }
}

pub async fn start_note(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path(raw_campus_id): Path<String>,
    ApiJson(request): ApiJson<StartNoteRequest>,
) -> Result<(StatusCode, Json<Note>), ApiFailure> {
    let campus_id = parse_campus_id(&raw_campus_id)?;
    let starter = authenticated.account_id;
    require_permission(&state.db, campus_id, starter, Permission::PostMessage).await?;
    request.validate()?;
    let others =
        normalize_note_members(&request.account_ids, starter).map_err(ApiError::invalid)?;
    let mut members = others.clone();
    members.push(starter);
    let mut connection = state.db.acquire().await?;
    if eligible_count(&mut connection, campus_id, &others).await? != others.len() {
        return Err(invalid_members());
    }
    drop(connection);
    let member_key = note_member_key(&members);
    let (room_id, created) = settle_note(&state, campus_id, &member_key, starter, &members).await?;
    let note = notes_where(&state.db, campus_id, ROOM_IS, room_id.as_uuid())
        .await?
        .into_iter()
        .next()
        .ok_or_else(invalid_members)?;
    if created {
        for member in &note.members {
            let frame = ServerFrame::NoteStarted(NoteStarted { note: note.clone() });
            publish_frame(&state, campus_id, Some(member.account_id), frame).await;
        }
    }
    let status = if created {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(note)))
}
