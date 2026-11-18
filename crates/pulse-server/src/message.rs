use pulse_protocol::{
    AccountId, CampusId, ClientNonce, MessageId, MessageRejection, Permission, Role, RoomCursor,
    RoomId, RoomMessage, RoomSynced, SYNC_PAGE_LIMIT, SendMessage, normalize_message_body,
    role_permits,
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::campus::campus_role;

type MessageRow = (Uuid, Option<Uuid>, Uuid, Uuid, String, Uuid, String);

macro_rules! select_messages {
    ($tail:literal) => {
        concat!(
            "SELECT m.message_id, m.previous_message_id, m.room_id, m.author_id, \
             a.display_name, m.client_nonce, m.body \
             FROM room_messages m JOIN accounts a ON a.account_id = m.author_id ",
            $tail
        )
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostOutcome {
    Created(RoomMessage),
    Replayed(RoomMessage),
}

#[derive(Debug)]
pub enum PostFailure {
    Rejected(MessageRejection),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for PostFailure {
    fn from(error: sqlx::Error) -> Self {
        PostFailure::Database(error)
    }
}

fn message_from_row(campus_id: CampusId, row: MessageRow) -> RoomMessage {
    let (message_id, previous_message_id, room_id, author_id, author_display_name, nonce, body) =
        row;
    RoomMessage {
        message_id: MessageId(message_id),
        previous_message_id: previous_message_id.map(MessageId),
        campus_id,
        room_id: RoomId(room_id),
        author_id: AccountId(author_id),
        author_display_name,
        client_nonce: ClientNonce(nonce),
        body,
    }
}

pub fn next_message_id(previous: Option<MessageId>) -> MessageId {
    let candidate = Uuid::now_v7();
    match previous {
        Some(previous) if candidate <= previous.as_uuid() => MessageId(Uuid::from_u128(
            previous.as_uuid().as_u128().saturating_add(1),
        )),
        _ => MessageId(candidate),
    }
}

async fn find_by_nonce(
    connection: &mut PgConnection,
    campus_id: CampusId,
    author_id: AccountId,
    client_nonce: ClientNonce,
) -> Result<Option<RoomMessage>, sqlx::Error> {
    let row = sqlx::query_as::<_, MessageRow>(select_messages!(
        "WHERE m.campus_id = $1 AND m.author_id = $2 AND m.client_nonce = $3"
    ))
    .bind(campus_id.as_uuid())
    .bind(author_id.as_uuid())
    .bind(client_nonce.as_uuid())
    .fetch_optional(connection)
    .await?;
    Ok(row.map(|row| message_from_row(campus_id, row)))
}

pub async fn post_message(
    db: &PgPool,
    campus_id: CampusId,
    author_id: AccountId,
    request: SendMessage,
) -> Result<PostOutcome, PostFailure> {
    let body = normalize_message_body(&request.body)
        .map_err(|_| PostFailure::Rejected(MessageRejection::InvalidBody));
    let allowed =
        |role: Option<Role>| role.is_some_and(|role| role_permits(role, Permission::PostMessage));
    if !allowed(campus_role(db, campus_id, author_id).await?) {
        return Err(PostFailure::Rejected(MessageRejection::Forbidden));
    }
    let body = body?;
    let mut transaction = db.begin().await?;
    let room_locked = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM rooms WHERE campus_id = $1 AND room_id = $2 FOR NO KEY UPDATE",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .fetch_optional(&mut *transaction)
    .await?;
    if room_locked.is_none() {
        return Err(PostFailure::Rejected(MessageRejection::UnknownRoom));
    }
    let role_text = sqlx::query_scalar::<_, String>(
        "SELECT role FROM campus_members WHERE campus_id = $1 AND account_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(author_id.as_uuid())
    .fetch_optional(&mut *transaction)
    .await?;
    let role = role_text.and_then(|text| text.parse::<Role>().ok());
    if !allowed(role) {
        return Err(PostFailure::Rejected(MessageRejection::Forbidden));
    }
    if let Some(original) =
        find_by_nonce(&mut transaction, campus_id, author_id, request.client_nonce).await?
    {
        return Ok(PostOutcome::Replayed(original));
    }
    let previous = sqlx::query_scalar::<_, Uuid>(
        "SELECT message_id FROM room_messages WHERE campus_id = $1 AND room_id = $2 \
         ORDER BY message_id DESC LIMIT 1",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .fetch_optional(&mut *transaction)
    .await?
    .map(MessageId);
    let message_id = next_message_id(previous);
    let inserted = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO room_messages \
         (campus_id, room_id, message_id, previous_message_id, author_id, client_nonce, body) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) \
         ON CONFLICT (campus_id, author_id, client_nonce) DO NOTHING \
         RETURNING message_id",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(message_id.as_uuid())
    .bind(previous.map(|previous| previous.as_uuid()))
    .bind(author_id.as_uuid())
    .bind(request.client_nonce.as_uuid())
    .bind(&body)
    .fetch_optional(&mut *transaction)
    .await?;
    if inserted.is_none() {
        transaction.rollback().await?;
        let mut connection = db.acquire().await?;
        return find_by_nonce(&mut connection, campus_id, author_id, request.client_nonce)
            .await?
            .map(PostOutcome::Replayed)
            .ok_or(PostFailure::Database(sqlx::Error::RowNotFound));
    }
    let created = find_by_nonce(&mut transaction, campus_id, author_id, request.client_nonce)
        .await?
        .ok_or(PostFailure::Database(sqlx::Error::RowNotFound))?;
    transaction.commit().await?;
    Ok(PostOutcome::Created(created))
}

pub async fn sync_room(
    db: &PgPool,
    campus_id: CampusId,
    cursor: &RoomCursor,
) -> Result<Option<RoomSynced>, sqlx::Error> {
    let room_exists =
        sqlx::query_scalar::<_, i32>("SELECT 1 FROM rooms WHERE campus_id = $1 AND room_id = $2")
            .bind(campus_id.as_uuid())
            .bind(cursor.room_id.as_uuid())
            .fetch_optional(db)
            .await?;
    if room_exists.is_none() {
        return Ok(None);
    }
    let limit = i64::try_from(SYNC_PAGE_LIMIT).unwrap_or(i64::MAX);
    let (rows, has_more) = match cursor.after {
        Some(after) => {
            let mut rows = sqlx::query_as::<_, MessageRow>(select_messages!(
                "WHERE m.campus_id = $1 AND m.room_id = $2 AND m.message_id > $3 \
                 ORDER BY m.message_id LIMIT $4"
            ))
            .bind(campus_id.as_uuid())
            .bind(cursor.room_id.as_uuid())
            .bind(after.as_uuid())
            .bind(limit.saturating_add(1))
            .fetch_all(db)
            .await?;
            let has_more = rows.len() > SYNC_PAGE_LIMIT;
            rows.truncate(SYNC_PAGE_LIMIT);
            (rows, has_more)
        }
        None => {
            let mut rows = sqlx::query_as::<_, MessageRow>(select_messages!(
                "WHERE m.campus_id = $1 AND m.room_id = $2 \
                 ORDER BY m.message_id DESC LIMIT $3"
            ))
            .bind(campus_id.as_uuid())
            .bind(cursor.room_id.as_uuid())
            .bind(limit)
            .fetch_all(db)
            .await?;
            rows.reverse();
            (rows, false)
        }
    };
    Ok(Some(RoomSynced {
        room_id: cursor.room_id,
        messages: rows
            .into_iter()
            .map(|row| message_from_row(campus_id, row))
            .collect(),
        has_more,
    }))
}
