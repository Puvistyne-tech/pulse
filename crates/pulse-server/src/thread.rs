use std::collections::HashMap;

use pulse_protocol::{
    AccountId, CampusId, MarkThreadRead, MessageId, Permission, RoomId, RoomMessage,
    SetThreadSubscription, SetTopicTitle, SubscriptionOrigin, SubscriptionState, ThreadRejection,
    ThreadSummary, normalize_topic_title, role_permits,
};
use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;

use crate::campus::campus_role;
use crate::message::{
    fetch_message, fetch_messages, is_top_level, latest_thread_message, lock_room, stamp_activity,
};

#[derive(Debug)]
pub enum ThreadFailure {
    Rejected(ThreadRejection),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for ThreadFailure {
    fn from(error: sqlx::Error) -> Self {
        ThreadFailure::Database(error)
    }
}

type SummaryRow = (
    Uuid,
    Uuid,
    String,
    Option<Uuid>,
    i64,
    Option<Uuid>,
    i64,
    i64,
);

fn count(value: i64) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

pub async fn thread_summaries(
    executor: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
    roots: Option<&[MessageId]>,
    limit: usize,
) -> Result<Vec<ThreadSummary>, sqlx::Error> {
    let mut transaction = executor
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ")
        .await?;
    let summaries =
        snapshot_summaries(&mut transaction, campus_id, account_id, roots, limit).await?;
    transaction.commit().await?;
    Ok(summaries)
}

async fn snapshot_summaries(
    executor: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
    roots: Option<&[MessageId]>,
    limit: usize,
) -> Result<Vec<ThreadSummary>, sqlx::Error> {
    let rows = sqlx::query_as::<_, SummaryRow>(
        "SELECT s.room_id, s.root_message_id, s.state, s.read_through, s.revision, \
         latest.message_id, \
         (SELECT count(*) FROM room_messages m \
          WHERE m.campus_id = s.campus_id AND m.thread_root_id = s.root_message_id \
          AND m.author_id <> s.account_id \
          AND (s.read_through IS NULL OR m.message_id > s.read_through)), \
         (SELECT count(*) FROM message_mentions x \
          WHERE x.campus_id = s.campus_id AND x.account_id = s.account_id \
          AND (s.read_through IS NULL OR x.message_id > s.read_through) \
          AND (x.message_id = s.root_message_id OR EXISTS ( \
              SELECT 1 FROM room_messages r \
              WHERE r.campus_id = s.campus_id AND r.message_id = x.message_id \
              AND r.thread_root_id = s.root_message_id))) \
         FROM thread_subscriptions s \
         LEFT JOIN LATERAL ( \
             SELECT m.message_id FROM room_messages m \
             WHERE m.campus_id = s.campus_id AND m.thread_root_id = s.root_message_id \
             ORDER BY m.message_id DESC LIMIT 1) latest ON true \
         WHERE s.campus_id = $1 AND s.account_id = $2 \
         AND ($3::uuid[] IS NULL OR s.root_message_id = ANY($3)) \
         ORDER BY COALESCE(latest.message_id, s.root_message_id) DESC, s.root_message_id DESC \
         LIMIT $4",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(roots.map(|roots| {
        roots
            .iter()
            .map(|root| root.as_uuid())
            .collect::<Vec<Uuid>>()
    }))
    .bind(i64::try_from(limit).unwrap_or(i64::MAX))
    .fetch_all(&mut *executor)
    .await?;
    let ids: Vec<MessageId> = rows
        .iter()
        .flat_map(|row| [Some(row.1), row.5])
        .flatten()
        .map(MessageId)
        .collect();
    let messages: HashMap<MessageId, RoomMessage> = fetch_messages(executor, campus_id, &ids)
        .await?
        .into_iter()
        .map(|message| (message.message_id, message))
        .collect();
    rows.into_iter()
        .map(
            |(_, root, state, read_through, revision, latest, unread_replies, unread_mentions)| {
                let root_message = messages
                    .get(&MessageId(root))
                    .cloned()
                    .ok_or(sqlx::Error::RowNotFound)?;
                Ok(ThreadSummary {
                    root: root_message,
                    latest_reply: latest.and_then(|id| messages.get(&MessageId(id)).cloned()),
                    unread_replies: count(unread_replies),
                    unread_mentions: count(unread_mentions),
                    subscription: state
                        .parse()
                        .map_err(|error| sqlx::Error::Decode(Box::new(error)))?,
                    read_through: read_through.map(MessageId),
                    revision: u64::try_from(revision).unwrap_or(0),
                })
            },
        )
        .collect()
}

async fn summary_for(
    connection: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
    root: MessageId,
) -> Result<Option<ThreadSummary>, sqlx::Error> {
    Ok(
        thread_summaries(connection, campus_id, account_id, Some(&[root]), 1)
            .await?
            .into_iter()
            .next(),
    )
}

pub async fn auto_follow(
    executor: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    root_message_id: MessageId,
    account_id: AccountId,
    read_through: Option<MessageId>,
) -> Result<bool, sqlx::Error> {
    let inserted = sqlx::query(
        "INSERT INTO thread_subscriptions \
         (campus_id, room_id, root_message_id, account_id, state, read_through) \
         VALUES ($1, $2, $3, $4, $5, $6) \
         ON CONFLICT (campus_id, root_message_id, account_id) DO NOTHING",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .bind(root_message_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(SubscriptionState::Following.as_str())
    .bind(read_through.map(|id| id.as_uuid()))
    .execute(executor)
    .await?;
    Ok(inserted.rows_affected() > 0)
}

pub(crate) async fn advance_read(
    executor: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    root_message_id: MessageId,
    account_id: AccountId,
    read_through: MessageId,
) -> Result<bool, sqlx::Error> {
    let updated = sqlx::query(
        "UPDATE thread_subscriptions \
         SET read_through = $5, revision = revision + 1, updated_at = now() \
         WHERE campus_id = $1 AND room_id = $2 AND root_message_id = $3 AND account_id = $4 \
         AND (read_through IS NULL OR read_through < $5)",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .bind(root_message_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(read_through.as_uuid())
    .execute(executor)
    .await?;
    Ok(updated.rows_affected() > 0)
}

async fn require_root(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_id: RoomId,
    root_message_id: MessageId,
) -> Result<(), ThreadFailure> {
    if is_top_level(connection, campus_id, room_id, root_message_id).await? {
        Ok(())
    } else {
        Err(ThreadFailure::Rejected(ThreadRejection::UnknownThread))
    }
}

pub async fn set_topic_title(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    request: SetTopicTitle,
) -> Result<RoomMessage, ThreadFailure> {
    let mut transaction = db.begin().await?;
    if !lock_room(&mut transaction, campus_id, request.room_id).await? {
        return Err(ThreadFailure::Rejected(ThreadRejection::UnknownThread));
    }
    let permitted = campus_role(&mut *transaction, campus_id, account_id)
        .await?
        .is_some_and(|role| role_permits(role, Permission::PostMessage));
    if !permitted {
        return Err(ThreadFailure::Rejected(ThreadRejection::Forbidden));
    }
    let title = normalize_topic_title(&request.topic_title)
        .map_err(|_| ThreadFailure::Rejected(ThreadRejection::InvalidTopicTitle))?;
    require_root(
        &mut transaction,
        campus_id,
        request.room_id,
        request.root_message_id,
    )
    .await?;
    sqlx::query(
        "INSERT INTO threads (campus_id, room_id, root_message_id, topic_title) \
         VALUES ($1, $2, $3, $4) \
         ON CONFLICT (campus_id, room_id, root_message_id) \
         DO UPDATE SET topic_title = EXCLUDED.topic_title",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(request.root_message_id.as_uuid())
    .bind(title)
    .execute(&mut *transaction)
    .await?;
    stamp_activity(
        &mut transaction,
        campus_id,
        request.room_id,
        request.root_message_id,
    )
    .await?;
    let message = fetch_message(&mut transaction, campus_id, request.root_message_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    transaction.commit().await?;
    Ok(message)
}

pub async fn set_subscription(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    request: SetThreadSubscription,
) -> Result<Option<ThreadSummary>, ThreadFailure> {
    if request.origin == SubscriptionOrigin::Viewed && request.state != SubscriptionState::Following
    {
        return Err(ThreadFailure::Rejected(ThreadRejection::Forbidden));
    }
    let mut connection = db.acquire().await?;
    require_root(
        &mut connection,
        campus_id,
        request.room_id,
        request.root_message_id,
    )
    .await?;
    let latest = latest_thread_message(
        &mut connection,
        campus_id,
        request.room_id,
        request.root_message_id,
    )
    .await?;
    let changed = match request.origin {
        SubscriptionOrigin::Viewed => {
            auto_follow(
                &mut connection,
                campus_id,
                request.room_id,
                request.root_message_id,
                account_id,
                Some(latest),
            )
            .await?
        }
        SubscriptionOrigin::Explicit => sqlx::query_scalar::<_, i64>(
            "INSERT INTO thread_subscriptions \
             (campus_id, room_id, root_message_id, account_id, state, read_through) \
             VALUES ($1, $2, $3, $4, $5, $6) \
             ON CONFLICT (campus_id, root_message_id, account_id) DO UPDATE \
             SET state = EXCLUDED.state, revision = thread_subscriptions.revision + 1, \
             updated_at = now() \
             WHERE thread_subscriptions.state <> EXCLUDED.state \
             RETURNING revision",
        )
        .bind(campus_id.as_uuid())
        .bind(request.room_id.as_uuid())
        .bind(request.root_message_id.as_uuid())
        .bind(account_id.as_uuid())
        .bind(request.state.as_str())
        .bind(latest.as_uuid())
        .fetch_optional(&mut *connection)
        .await?
        .is_some(),
    };
    if !changed {
        return Ok(None);
    }
    Ok(summary_for(
        &mut connection,
        campus_id,
        account_id,
        request.root_message_id,
    )
    .await?)
}

pub async fn mark_read(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    request: MarkThreadRead,
) -> Result<Option<ThreadSummary>, ThreadFailure> {
    let mut connection = db.acquire().await?;
    require_root(
        &mut connection,
        campus_id,
        request.room_id,
        request.root_message_id,
    )
    .await?;
    let belongs = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM room_messages \
         WHERE campus_id = $1 AND room_id = $2 AND message_id = $4 \
         AND (message_id = $3 OR thread_root_id = $3)",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(request.root_message_id.as_uuid())
    .bind(request.read_through.as_uuid())
    .fetch_optional(&mut *connection)
    .await?;
    if belongs.is_none() {
        return Err(ThreadFailure::Rejected(ThreadRejection::UnknownThread));
    }
    let advanced = advance_read(
        &mut connection,
        campus_id,
        request.room_id,
        request.root_message_id,
        account_id,
        request.read_through,
    )
    .await?;
    if !advanced {
        return Ok(None);
    }
    Ok(summary_for(
        &mut connection,
        campus_id,
        account_id,
        request.root_message_id,
    )
    .await?)
}
