use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use pulse_protocol::{
    AccountId, CampusId, MessageId, Permission, SHELF_ITEMS_MAX, ServerFrame, ShelfAction,
    ShelfChanged, ShelfItem, ShelfItemId, ShelfRejection, UpdateShelf, Validate, role_permits,
    validate_remind_at,
};
use sqlx::{Connection, PgConnection, PgPool};
use tokio::time::MissedTickBehavior;
use uuid::Uuid;

use crate::campus::campus_role;
use crate::fanout::FanoutEnvelope;
use crate::message::fetch_messages;
use crate::state::AppState;

const DUE_BATCH_PER_CAMPUS: usize = 200;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShelfOutcome {
    Changed(ShelfItem),
    Removed(ShelfItem),
    Unchanged,
}

#[derive(Debug)]
pub enum ShelfFailure {
    Rejected(ShelfRejection),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for ShelfFailure {
    fn from(error: sqlx::Error) -> Self {
        ShelfFailure::Database(error)
    }
}

fn rejected(reason: ShelfRejection) -> ShelfFailure {
    ShelfFailure::Rejected(reason)
}

type ShelfRow = (Uuid, Uuid, Option<i64>, Option<i64>, Option<i64>, i64, i64);

const SHELF_COLUMNS: &str = "shelf_item_id, message_id, \
     (EXTRACT(EPOCH FROM remind_at) * 1000)::bigint, \
     (EXTRACT(EPOCH FROM reminded_at) * 1000)::bigint, \
     (EXTRACT(EPOCH FROM done_at) * 1000)::bigint, \
     (EXTRACT(EPOCH FROM created_at) * 1000)::bigint, revision";

fn millis(value: Option<i64>) -> Option<u64> {
    value.map(|value| u64::try_from(value).unwrap_or(0))
}

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
        })
}

pub async fn shelf_items(
    executor: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
    ids: Option<&[ShelfItemId]>,
    limit: usize,
) -> Result<Vec<ShelfItem>, sqlx::Error> {
    let mut transaction = executor
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ")
        .await?;
    let items = snapshot_items(&mut transaction, campus_id, account_id, ids, limit).await?;
    transaction.commit().await?;
    Ok(items)
}

async fn snapshot_items(
    executor: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
    ids: Option<&[ShelfItemId]>,
    limit: usize,
) -> Result<Vec<ShelfItem>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ShelfRow>(select_shelf(
        "WHERE campus_id = $1 AND account_id = $2 \
         AND ($3::uuid[] IS NULL OR shelf_item_id = ANY($3)) \
         ORDER BY created_at DESC, shelf_item_id DESC LIMIT $4",
    ))
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(ids.map(|ids| ids.iter().map(|id| id.as_uuid()).collect::<Vec<Uuid>>()))
    .bind(i64::try_from(limit).unwrap_or(i64::MAX))
    .fetch_all(&mut *executor)
    .await?;
    let message_ids = rows.iter().map(|row| MessageId(row.1)).collect::<Vec<_>>();
    let messages = fetch_messages(executor, campus_id, &message_ids)
        .await?
        .into_iter()
        .map(|message| (message.message_id, message))
        .collect::<HashMap<_, _>>();
    Ok(rows
        .into_iter()
        .filter_map(
            |(shelf_item_id, message_id, remind_at, reminded_at, done_at, saved_at, revision)| {
                messages
                    .get(&MessageId(message_id))
                    .map(|message| ShelfItem {
                        shelf_item_id: ShelfItemId(shelf_item_id),
                        message: message.clone(),
                        remind_at_ms: millis(remind_at),
                        reminded_at_ms: millis(reminded_at),
                        done_at_ms: millis(done_at),
                        saved_at_ms: u64::try_from(saved_at).unwrap_or(0),
                        revision: u64::try_from(revision).unwrap_or(0),
                    })
            },
        )
        .collect())
}

fn select_shelf(tail: &str) -> sqlx::AssertSqlSafe<String> {
    sqlx::AssertSqlSafe(format!("SELECT {SHELF_COLUMNS} FROM shelf_items {tail}"))
}

async fn shelf_item_for_message(
    connection: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
    message_id: MessageId,
) -> Result<Option<ShelfItem>, sqlx::Error> {
    let shelf_item_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT shelf_item_id FROM shelf_items \
         WHERE campus_id = $1 AND account_id = $2 AND message_id = $3",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(message_id.as_uuid())
    .fetch_optional(&mut *connection)
    .await?;
    let Some(shelf_item_id) = shelf_item_id else {
        return Ok(None);
    };
    Ok(snapshot_items(
        connection,
        campus_id,
        account_id,
        Some(&[ShelfItemId(shelf_item_id)]),
        1,
    )
    .await?
    .into_iter()
    .next())
}

pub async fn current_item(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    message_id: MessageId,
) -> Result<Option<ShelfItem>, sqlx::Error> {
    let mut transaction = db
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ")
        .await?;
    let item = shelf_item_for_message(&mut transaction, campus_id, account_id, message_id).await?;
    transaction.commit().await?;
    Ok(item)
}

async fn upsert_item(
    transaction: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
    request: &UpdateShelf,
) -> Result<(), ShelfFailure> {
    let exists = sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM shelf_items WHERE campus_id = $1 AND account_id = $2 AND message_id = $3",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(request.message_id.as_uuid())
    .fetch_optional(&mut *transaction)
    .await?
    .is_some();
    let remind_at = request
        .remind_at_ms
        .map(|remind| i64::try_from(remind).unwrap_or(i64::MAX));
    if exists {
        sqlx::query(
            "UPDATE shelf_items SET remind_at = to_timestamp($4::bigint / 1000.0), \
             reminded_at = NULL, done_at = NULL, revision = revision + 1, updated_at = now() \
             WHERE campus_id = $1 AND account_id = $2 AND message_id = $3",
        )
        .bind(campus_id.as_uuid())
        .bind(account_id.as_uuid())
        .bind(request.message_id.as_uuid())
        .bind(remind_at)
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }
    let total = sqlx::query_scalar::<_, i64>(
        "SELECT count(*) FROM shelf_items WHERE campus_id = $1 AND account_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .fetch_one(&mut *transaction)
    .await?;
    if usize::try_from(total).unwrap_or(usize::MAX) >= SHELF_ITEMS_MAX {
        return Err(rejected(ShelfRejection::ShelfFull));
    }
    sqlx::query(
        "INSERT INTO shelf_items \
         (campus_id, account_id, shelf_item_id, room_id, message_id, remind_at) \
         VALUES ($1, $2, $3, $4, $5, to_timestamp($6::bigint / 1000.0))",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(ShelfItemId::generate().as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(request.message_id.as_uuid())
    .bind(remind_at)
    .execute(&mut *transaction)
    .await?;
    Ok(())
}

async fn transition(
    transaction: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
    request: &UpdateShelf,
    statement: &'static str,
) -> Result<bool, ShelfFailure> {
    let affected = sqlx::query(statement)
        .bind(campus_id.as_uuid())
        .bind(account_id.as_uuid())
        .bind(request.message_id.as_uuid())
        .execute(&mut *transaction)
        .await?
        .rows_affected();
    if affected > 0 {
        return Ok(true);
    }
    let exists = shelf_item_for_message(transaction, campus_id, account_id, request.message_id)
        .await?
        .is_some();
    if exists {
        Ok(false)
    } else {
        Err(rejected(ShelfRejection::UnknownItem))
    }
}

pub async fn update_shelf(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    request: UpdateShelf,
    now_ms: u64,
) -> Result<ShelfOutcome, ShelfFailure> {
    request
        .validate()
        .map_err(|_| rejected(ShelfRejection::InvalidRemindAt))?;
    if let Some(remind_at_ms) = request.remind_at_ms {
        validate_remind_at(remind_at_ms, now_ms)
            .map_err(|_| rejected(ShelfRejection::InvalidRemindAt))?;
    }
    let mut transaction = db.begin().await?;
    let permitted = campus_role(&mut *transaction, campus_id, account_id)
        .await?
        .is_some_and(|role| role_permits(role, Permission::ViewRooms));
    if !permitted {
        return Err(rejected(ShelfRejection::Forbidden));
    }
    sqlx::query_scalar::<_, i32>(
        "SELECT 1 FROM campus_members WHERE campus_id = $1 AND account_id = $2 \
         FOR NO KEY UPDATE",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .fetch_one(&mut *transaction)
    .await?;
    let outcome = match request.action {
        ShelfAction::Save | ShelfAction::Reschedule => {
            let message_exists = sqlx::query_scalar::<_, i32>(
                "SELECT 1 FROM room_messages \
                 WHERE campus_id = $1 AND room_id = $2 AND message_id = $3",
            )
            .bind(campus_id.as_uuid())
            .bind(request.room_id.as_uuid())
            .bind(request.message_id.as_uuid())
            .fetch_optional(&mut *transaction)
            .await?
            .is_some();
            if !message_exists {
                return Err(rejected(ShelfRejection::UnknownMessage));
            }
            upsert_item(&mut transaction, campus_id, account_id, &request).await?;
            true
        }
        ShelfAction::Complete => {
            transition(
                &mut transaction,
                campus_id,
                account_id,
                &request,
                "UPDATE shelf_items SET done_at = now(), revision = revision + 1, \
                 updated_at = now() \
                 WHERE campus_id = $1 AND account_id = $2 AND message_id = $3 \
                 AND done_at IS NULL",
            )
            .await?
        }
        ShelfAction::Reopen => {
            transition(
                &mut transaction,
                campus_id,
                account_id,
                &request,
                "UPDATE shelf_items SET done_at = NULL, revision = revision + 1, \
                 updated_at = now() \
                 WHERE campus_id = $1 AND account_id = $2 AND message_id = $3 \
                 AND done_at IS NOT NULL",
            )
            .await?
        }
        ShelfAction::Remove => {
            let existing =
                shelf_item_for_message(&mut transaction, campus_id, account_id, request.message_id)
                    .await?
                    .ok_or_else(|| rejected(ShelfRejection::UnknownItem))?;
            sqlx::query(
                "DELETE FROM shelf_items \
                 WHERE campus_id = $1 AND account_id = $2 AND message_id = $3",
            )
            .bind(campus_id.as_uuid())
            .bind(account_id.as_uuid())
            .bind(request.message_id.as_uuid())
            .execute(&mut *transaction)
            .await?;
            transaction.commit().await?;
            return Ok(ShelfOutcome::Removed(existing));
        }
    };
    if !outcome {
        transaction.commit().await?;
        return Ok(ShelfOutcome::Unchanged);
    }
    let item = shelf_item_for_message(&mut transaction, campus_id, account_id, request.message_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    transaction.commit().await?;
    Ok(ShelfOutcome::Changed(item))
}

pub async fn claim_due_reminders(
    db: &PgPool,
    campus_id: CampusId,
    limit: usize,
) -> Result<Vec<(AccountId, ShelfItem)>, sqlx::Error> {
    let claimed = sqlx::query_as::<_, (Uuid, Uuid)>(
        "UPDATE shelf_items SET reminded_at = now(), revision = revision + 1, updated_at = now() \
         WHERE campus_id = $1 AND shelf_item_id IN ( \
             SELECT shelf_item_id FROM shelf_items \
             WHERE campus_id = $1 AND reminded_at IS NULL AND done_at IS NULL \
             AND remind_at IS NOT NULL AND remind_at <= now() \
             ORDER BY remind_at LIMIT $2 FOR UPDATE SKIP LOCKED) \
         RETURNING account_id, shelf_item_id",
    )
    .bind(campus_id.as_uuid())
    .bind(i64::try_from(limit).unwrap_or(i64::MAX))
    .fetch_all(db)
    .await?;
    let mut by_account: HashMap<Uuid, Vec<ShelfItemId>> = HashMap::new();
    for (account_id, shelf_item_id) in claimed {
        by_account
            .entry(account_id)
            .or_default()
            .push(ShelfItemId(shelf_item_id));
    }
    let mut connection = db.acquire().await?;
    let mut delivered = Vec::new();
    for (account_id, ids) in by_account {
        let account_id = AccountId(account_id);
        for item in shelf_items(
            &mut connection,
            campus_id,
            account_id,
            Some(&ids),
            ids.len(),
        )
        .await?
        {
            delivered.push((account_id, item));
        }
    }
    Ok(delivered)
}

pub async fn deliver_due_reminders(
    state: &AppState,
    limit_per_campus: usize,
) -> Result<usize, sqlx::Error> {
    let campuses = sqlx::query_scalar::<_, Uuid>("SELECT campus_id FROM campuses")
        .fetch_all(&state.db)
        .await?;
    let mut delivered = 0;
    for campus_id in campuses {
        let campus_id = CampusId(campus_id);
        for (account_id, item) in
            claim_due_reminders(&state.db, campus_id, limit_per_campus).await?
        {
            delivered += 1;
            let envelope = FanoutEnvelope {
                campus_id,
                account_id: Some(account_id),
                frame: ServerFrame::ShelfChanged(ShelfChanged {
                    item,
                    removed: false,
                }),
            };
            if let Err(error) = state.fanout.publish(&envelope).await {
                tracing::warn!(%error, "reminder fanout failed");
            }
        }
    }
    Ok(delivered)
}

pub fn spawn_reminder_loop(state: AppState, every: Duration) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(every);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
        loop {
            ticker.tick().await;
            if let Err(error) = deliver_due_reminders(&state, DUE_BATCH_PER_CAMPUS).await {
                tracing::warn!(%error, "reminder scan failed");
            }
        }
    })
}
