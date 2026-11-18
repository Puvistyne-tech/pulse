use std::collections::{BTreeMap, HashMap};

use pulse_protocol::{
    AccountId, CampusId, ClientNonce, MentionRef, MessageId, MessageRejection, Permission,
    ReactionTally, RoomCursor, RoomId, RoomMessage, RoomSynced, SYNC_PAGE_LIMIT, SendMessage,
    SyncThread, ThreadSynced, extract_mentions, normalize_message_body, role_permits,
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::campus::campus_role;
use crate::thread::{advance_read, auto_follow};

#[derive(Debug, sqlx::FromRow)]
pub struct MessageRow {
    message_id: Uuid,
    previous_message_id: Option<Uuid>,
    room_id: Uuid,
    author_id: Uuid,
    author_display_name: String,
    client_nonce: Uuid,
    body: String,
    thread_root_id: Option<Uuid>,
    topic_title: Option<String>,
    activity_seq: i64,
}

macro_rules! select_messages {
    ($tail:literal) => {
        concat!(
            "SELECT m.message_id, m.previous_message_id, m.room_id, m.author_id, \
             a.display_name AS author_display_name, m.client_nonce, m.body, m.thread_root_id, \
             t.topic_title, m.activity_seq \
             FROM room_messages m JOIN accounts a ON a.account_id = m.author_id \
             LEFT JOIN threads t ON t.campus_id = m.campus_id AND t.room_id = m.room_id \
             AND t.root_message_id = m.message_id ",
            $tail
        )
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostOutcome {
    Created(RoomMessage, Vec<AccountId>),
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

pub fn next_message_id(previous: Option<MessageId>) -> MessageId {
    let candidate = Uuid::now_v7();
    match previous {
        Some(previous) if candidate <= previous.as_uuid() => MessageId(Uuid::from_u128(
            previous.as_uuid().as_u128().saturating_add(1),
        )),
        _ => MessageId(candidate),
    }
}

fn uuids(ids: &[MessageId]) -> Vec<Uuid> {
    ids.iter().map(|id| id.as_uuid()).collect()
}

pub async fn hydrate_messages(
    executor: &mut PgConnection,
    campus_id: CampusId,
    rows: Vec<MessageRow>,
) -> Result<Vec<RoomMessage>, sqlx::Error> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let ids: Vec<Uuid> = rows.iter().map(|row| row.message_id).collect();
    let mention_rows = sqlx::query_as::<_, (Uuid, Uuid, String)>(
        "SELECT mm.message_id, mm.account_id, a.display_name \
         FROM message_mentions mm JOIN accounts a ON a.account_id = mm.account_id \
         WHERE mm.campus_id = $1 AND mm.message_id = ANY($2) \
         ORDER BY mm.message_id, mm.account_id",
    )
    .bind(campus_id.as_uuid())
    .bind(&ids)
    .fetch_all(&mut *executor)
    .await?;
    let reaction_rows = sqlx::query_as::<_, (Uuid, String, Uuid)>(
        "SELECT message_id, emoji, account_id FROM message_reactions \
         WHERE campus_id = $1 AND message_id = ANY($2)",
    )
    .bind(campus_id.as_uuid())
    .bind(&ids)
    .fetch_all(&mut *executor)
    .await?;
    let reply_rows = sqlx::query_as::<_, (Uuid, Uuid, i64)>(
        "SELECT DISTINCT ON (thread_root_id) thread_root_id, message_id, \
         count(*) OVER (PARTITION BY thread_root_id) \
         FROM room_messages \
         WHERE campus_id = $1 AND thread_root_id = ANY($2) \
         ORDER BY thread_root_id, message_id DESC",
    )
    .bind(campus_id.as_uuid())
    .bind(&ids)
    .fetch_all(&mut *executor)
    .await?;
    let mut mentions: HashMap<Uuid, Vec<MentionRef>> = HashMap::new();
    for (message_id, account_id, display_name) in mention_rows {
        mentions.entry(message_id).or_default().push(MentionRef {
            account_id: AccountId(account_id),
            display_name,
        });
    }
    let mut reactions: HashMap<Uuid, BTreeMap<String, Vec<AccountId>>> = HashMap::new();
    for (message_id, emoji, account_id) in reaction_rows {
        reactions
            .entry(message_id)
            .or_default()
            .entry(emoji)
            .or_default()
            .push(AccountId(account_id));
    }
    let replies: HashMap<Uuid, (MessageId, u32)> = reply_rows
        .into_iter()
        .map(|(root, latest, count)| {
            (
                root,
                (MessageId(latest), u32::try_from(count).unwrap_or(u32::MAX)),
            )
        })
        .collect();
    Ok(rows
        .into_iter()
        .map(|row| {
            let (latest_reply_id, reply_count) = replies
                .get(&row.message_id)
                .map_or((None, 0), |(latest, count)| (Some(*latest), *count));
            RoomMessage {
                message_id: MessageId(row.message_id),
                previous_message_id: row.previous_message_id.map(MessageId),
                campus_id,
                room_id: RoomId(row.room_id),
                author_id: AccountId(row.author_id),
                author_display_name: row.author_display_name,
                client_nonce: ClientNonce(row.client_nonce),
                body: row.body,
                thread_root_id: row.thread_root_id.map(MessageId),
                topic_title: row.topic_title,
                reply_count,
                latest_reply_id,
                mentions: mentions.remove(&row.message_id).unwrap_or_default(),
                reactions: reactions
                    .remove(&row.message_id)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(emoji, mut account_ids)| {
                        account_ids.sort();
                        ReactionTally { emoji, account_ids }
                    })
                    .collect(),
                activity_seq: u64::try_from(row.activity_seq).unwrap_or(0),
            }
        })
        .collect())
}

pub(crate) async fn lock_room(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
) -> Result<bool, sqlx::Error> {
    let locked = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM rooms WHERE campus_id = $1 AND room_id = $2 FOR NO KEY UPDATE",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .fetch_optional(connection)
    .await?;
    Ok(locked.is_some())
}

pub(crate) async fn message_root(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    message_id: MessageId,
) -> Result<Option<Option<MessageId>>, sqlx::Error> {
    let root = sqlx::query_scalar::<_, Option<Uuid>>(
        "SELECT thread_root_id FROM room_messages \
         WHERE campus_id = $1 AND room_id = $2 AND message_id = $3",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .bind(message_id.as_uuid())
    .fetch_optional(connection)
    .await?;
    Ok(root.map(|root| root.map(MessageId)))
}

pub(crate) async fn is_top_level(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    message_id: MessageId,
) -> Result<bool, sqlx::Error> {
    Ok(message_root(connection, campus_id, room_id, message_id)
        .await?
        .is_some_and(|thread_root| thread_root.is_none()))
}

pub(crate) async fn latest_thread_message(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    root_message_id: MessageId,
) -> Result<MessageId, sqlx::Error> {
    let latest = sqlx::query_scalar::<_, Uuid>(
        "SELECT message_id FROM room_messages \
         WHERE campus_id = $1 AND room_id = $2 AND (message_id = $3 OR thread_root_id = $3) \
         ORDER BY message_id DESC LIMIT 1",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .bind(root_message_id.as_uuid())
    .fetch_one(connection)
    .await?;
    Ok(MessageId(latest))
}

pub(crate) async fn stamp_activity(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    message_id: MessageId,
) -> Result<(), sqlx::Error> {
    let sequence = sqlx::query_scalar::<_, i64>(
        "UPDATE rooms SET activity_seq = activity_seq + 1 \
         WHERE campus_id = $1 AND room_id = $2 RETURNING activity_seq",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .fetch_one(&mut *connection)
    .await?;
    sqlx::query(
        "UPDATE room_messages SET activity_seq = $4 \
         WHERE campus_id = $1 AND room_id = $2 AND message_id = $3",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .bind(message_id.as_uuid())
    .bind(sequence)
    .execute(connection)
    .await?;
    Ok(())
}

pub(crate) async fn fetch_messages(
    connection: &mut PgConnection,
    campus_id: CampusId,
    ids: &[MessageId],
) -> Result<Vec<RoomMessage>, sqlx::Error> {
    let rows = sqlx::query_as::<_, MessageRow>(select_messages!(
        "WHERE m.campus_id = $1 AND m.message_id = ANY($2) ORDER BY m.message_id"
    ))
    .bind(campus_id.as_uuid())
    .bind(uuids(ids))
    .fetch_all(&mut *connection)
    .await?;
    hydrate_messages(connection, campus_id, rows).await
}

pub(crate) async fn fetch_message(
    connection: &mut PgConnection,
    campus_id: CampusId,
    message_id: MessageId,
) -> Result<Option<RoomMessage>, sqlx::Error> {
    Ok(fetch_messages(connection, campus_id, &[message_id])
        .await?
        .into_iter()
        .next())
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
    .fetch_optional(&mut *connection)
    .await?;
    match row {
        Some(row) => Ok(hydrate_messages(connection, campus_id, vec![row])
            .await?
            .into_iter()
            .next()),
        None => Ok(None),
    }
}

async fn thread_has_replies(
    connection: &mut PgConnection,
    campus_id: CampusId,
    root: MessageId,
) -> Result<bool, sqlx::Error> {
    let found = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM room_messages WHERE campus_id = $1 AND thread_root_id = $2 LIMIT 1",
    )
    .bind(campus_id.as_uuid())
    .bind(root.as_uuid())
    .fetch_optional(connection)
    .await?;
    Ok(found.is_some())
}

async fn message_author(
    connection: &mut PgConnection,
    campus_id: CampusId,
    message_id: MessageId,
) -> Result<Option<AccountId>, sqlx::Error> {
    let author = sqlx::query_scalar::<_, Uuid>(
        "SELECT author_id FROM room_messages WHERE campus_id = $1 AND message_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(message_id.as_uuid())
    .fetch_optional(connection)
    .await?;
    Ok(author.map(AccountId))
}

async fn record_mentions(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    message_id: MessageId,
    body: &str,
) -> Result<Vec<AccountId>, sqlx::Error> {
    let mut recorded = Vec::new();
    for account_id in extract_mentions(body) {
        if campus_role(&mut *connection, campus_id, account_id)
            .await?
            .is_none()
        {
            continue;
        }
        sqlx::query(
            "INSERT INTO message_mentions (campus_id, room_id, message_id, account_id) \
             VALUES ($1, $2, $3, $4)",
        )
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(message_id.as_uuid())
        .bind(account_id.as_uuid())
        .execute(&mut *connection)
        .await?;
        recorded.push(account_id);
    }
    Ok(recorded)
}

struct Followers {
    campus_id: CampusId,
    room_id: RoomId,
    root: MessageId,
    changed: Vec<AccountId>,
}

impl Followers {
    async fn follow(
        &mut self,
        connection: &mut PgConnection,
        account_id: AccountId,
        read_through: Option<MessageId>,
    ) -> Result<(), sqlx::Error> {
        let inserted = auto_follow(
            connection,
            self.campus_id,
            self.room_id,
            self.root,
            account_id,
            read_through,
        )
        .await?;
        let advanced = match read_through {
            Some(read_through) if !inserted => {
                advance_read(
                    connection,
                    self.campus_id,
                    self.room_id,
                    self.root,
                    account_id,
                    read_through,
                )
                .await?
            }
            _ => false,
        };
        if (inserted || advanced) && !self.changed.contains(&account_id) {
            self.changed.push(account_id);
        }
        Ok(())
    }
}

pub async fn post_message(
    db: &PgPool,
    campus_id: CampusId,
    author_id: AccountId,
    request: SendMessage,
) -> Result<PostOutcome, PostFailure> {
    let mut transaction = db.begin().await?;
    if !lock_room(&mut transaction, campus_id, request.room_id).await? {
        return Err(PostFailure::Rejected(MessageRejection::UnknownRoom));
    }
    let permitted = campus_role(&mut *transaction, campus_id, author_id)
        .await?
        .is_some_and(|role| role_permits(role, Permission::PostMessage));
    if !permitted {
        return Err(PostFailure::Rejected(MessageRejection::Forbidden));
    }
    let body = normalize_message_body(&request.body)
        .map_err(|_| PostFailure::Rejected(MessageRejection::InvalidBody))?;
    if let Some(original) =
        find_by_nonce(&mut transaction, campus_id, author_id, request.client_nonce).await?
    {
        return Ok(PostOutcome::Replayed(original));
    }
    let mut first_reply = false;
    if let Some(root) = request.thread_root_id {
        if !is_top_level(&mut transaction, campus_id, request.room_id, root).await? {
            return Err(PostFailure::Rejected(MessageRejection::UnknownThread));
        }
        first_reply = !thread_has_replies(&mut transaction, campus_id, root).await?;
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
         (campus_id, room_id, message_id, previous_message_id, author_id, client_nonce, body, \
         thread_root_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
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
    .bind(request.thread_root_id.map(|root| root.as_uuid()))
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
    let mentioned = record_mentions(
        &mut transaction,
        campus_id,
        request.room_id,
        message_id,
        &body,
    )
    .await?;
    let mut followers = Followers {
        campus_id,
        room_id: request.room_id,
        root: request.thread_root_id.unwrap_or(message_id),
        changed: Vec::new(),
    };
    if let Some(root) = request.thread_root_id {
        sqlx::query(
            "INSERT INTO threads (campus_id, room_id, root_message_id) VALUES ($1, $2, $3) \
             ON CONFLICT (campus_id, room_id, root_message_id) DO NOTHING",
        )
        .bind(campus_id.as_uuid())
        .bind(request.room_id.as_uuid())
        .bind(root.as_uuid())
        .execute(&mut *transaction)
        .await?;
        followers
            .follow(&mut transaction, author_id, Some(message_id))
            .await?;
        if first_reply
            && let Some(root_author) = message_author(&mut transaction, campus_id, root).await?
            && root_author != author_id
        {
            followers
                .follow(&mut transaction, root_author, Some(root))
                .await?;
        }
    }
    for account_id in mentioned
        .into_iter()
        .filter(|account| *account != author_id)
    {
        followers.follow(&mut transaction, account_id, None).await?;
    }
    let created = fetch_message(&mut transaction, campus_id, message_id)
        .await?
        .ok_or(PostFailure::Database(sqlx::Error::RowNotFound))?;
    transaction.commit().await?;
    Ok(PostOutcome::Created(created, followers.changed))
}

pub async fn sync_room(
    db: &PgPool,
    campus_id: CampusId,
    cursor: &RoomCursor,
) -> Result<Option<RoomSynced>, sqlx::Error> {
    let mut transaction = db
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ")
        .await?;
    let room_sequence = sqlx::query_scalar::<_, i64>(
        "SELECT activity_seq FROM rooms WHERE campus_id = $1 AND room_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(cursor.room_id.as_uuid())
    .fetch_optional(&mut *transaction)
    .await?;
    let Some(room_sequence) = room_sequence else {
        return Ok(None);
    };
    let limit = i64::try_from(SYNC_PAGE_LIMIT).unwrap_or(i64::MAX);
    let mut activity_seq = u64::try_from(room_sequence).unwrap_or(0);
    let mut updates = Vec::new();
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
            .fetch_all(&mut *transaction)
            .await?;
            let has_more = rows.len() > SYNC_PAGE_LIMIT;
            rows.truncate(SYNC_PAGE_LIMIT);
            let mut update_rows = sqlx::query_as::<_, MessageRow>(select_messages!(
                "WHERE m.campus_id = $1 AND m.room_id = $2 AND m.message_id <= $3 \
                 AND m.activity_seq > $4 ORDER BY m.activity_seq LIMIT $5"
            ))
            .bind(campus_id.as_uuid())
            .bind(cursor.room_id.as_uuid())
            .bind(after.as_uuid())
            .bind(i64::try_from(cursor.activity_after.unwrap_or(0)).unwrap_or(i64::MAX))
            .bind(limit.saturating_add(1))
            .fetch_all(&mut *transaction)
            .await?;
            let updates_truncated = update_rows.len() > SYNC_PAGE_LIMIT;
            if updates_truncated {
                update_rows.truncate(SYNC_PAGE_LIMIT);
                activity_seq = update_rows.last().map_or(activity_seq, |row| {
                    u64::try_from(row.activity_seq).unwrap_or(0)
                });
            }
            updates = hydrate_messages(&mut transaction, campus_id, update_rows).await?;
            (rows, has_more || updates_truncated)
        }
        None => {
            let mut rows = sqlx::query_as::<_, MessageRow>(select_messages!(
                "WHERE m.campus_id = $1 AND m.room_id = $2 \
                 ORDER BY m.message_id DESC LIMIT $3"
            ))
            .bind(campus_id.as_uuid())
            .bind(cursor.room_id.as_uuid())
            .bind(limit)
            .fetch_all(&mut *transaction)
            .await?;
            rows.reverse();
            (rows, false)
        }
    };
    let messages = hydrate_messages(&mut transaction, campus_id, rows).await?;
    Ok(Some(RoomSynced {
        room_id: cursor.room_id,
        messages,
        updates,
        activity_seq,
        has_more,
    }))
}

pub async fn sync_thread(
    db: &PgPool,
    campus_id: CampusId,
    request: &SyncThread,
) -> Result<Option<ThreadSynced>, sqlx::Error> {
    let mut transaction = db
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ")
        .await?;
    let top_level = is_top_level(
        &mut transaction,
        campus_id,
        request.room_id,
        request.root_message_id,
    )
    .await?;
    if !top_level {
        return Ok(None);
    }
    let limit = i64::try_from(SYNC_PAGE_LIMIT).unwrap_or(i64::MAX);
    let mut rows = sqlx::query_as::<_, MessageRow>(select_messages!(
        "WHERE m.campus_id = $1 AND m.room_id = $2 \
         AND (m.message_id = $3 OR m.thread_root_id = $3) \
         AND ($4::uuid IS NULL OR m.message_id > $4) \
         ORDER BY m.message_id LIMIT $5"
    ))
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(request.root_message_id.as_uuid())
    .bind(request.after.map(|after| after.as_uuid()))
    .bind(limit.saturating_add(1))
    .fetch_all(&mut *transaction)
    .await?;
    let has_more = rows.len() > SYNC_PAGE_LIMIT;
    rows.truncate(SYNC_PAGE_LIMIT);
    let messages = hydrate_messages(&mut transaction, campus_id, rows).await?;
    Ok(Some(ThreadSynced {
        room_id: request.room_id,
        root_message_id: request.root_message_id,
        messages,
        has_more,
    }))
}
