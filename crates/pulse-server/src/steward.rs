use pulse_protocol::{
    AccountId, AuthToken, CampusId, ManageSteward, MessageId, Permission, Room, RoomId,
    RoomMessage, STEWARDS_PER_CAMPUS_MAX, StewardAction, StewardProfile, StewardRejection,
    StewardWorkItem, WorkSynced, normalize_display_name, normalize_room_ids, role_permits,
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::auth::token_digest;
pub use crate::auth::{StewardCredential, mint_steward_token, resolve_steward_token};
use crate::campus::campus_role;
use crate::message::fetch_messages;
use crate::note::NOT_A_NOTE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StewardOutcome {
    Issued(StewardProfile, AuthToken),
    Changed(StewardProfile),
}

#[derive(Debug)]
pub enum StewardFailure {
    Rejected(StewardRejection),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for StewardFailure {
    fn from(error: sqlx::Error) -> Self {
        StewardFailure::Database(error)
    }
}

fn rejected(reason: StewardRejection) -> StewardFailure {
    StewardFailure::Rejected(reason)
}

type ProfileRow = (Uuid, String, bool, i64, Vec<Uuid>);

fn profile_from_row(row: ProfileRow) -> StewardProfile {
    let (steward_id, display_name, revoked, revision, room_ids) = row;
    StewardProfile {
        steward_id: AccountId(steward_id),
        display_name,
        room_ids: room_ids.into_iter().map(RoomId).collect(),
        revoked,
        credential_revision: u64::try_from(revision).unwrap_or(0),
    }
}

async fn profiles(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    only: Option<AccountId>,
) -> Result<Vec<StewardProfile>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ProfileRow>(
        "SELECT s.steward_id, a.display_name, s.revoked_at IS NOT NULL, s.credential_revision, \
         COALESCE(array_agg(r.room_id ORDER BY r.room_id) FILTER (WHERE r.room_id IS NOT NULL), \
         '{}'::uuid[]) \
         FROM stewards s JOIN accounts a ON a.account_id = s.steward_id \
         LEFT JOIN steward_rooms r ON r.campus_id = s.campus_id AND r.steward_id = s.steward_id \
         WHERE s.campus_id = $1 AND ($2::uuid IS NULL OR s.steward_id = $2) \
         GROUP BY s.steward_id, a.display_name, s.revoked_at, s.credential_revision \
         ORDER BY s.steward_id",
    )
    .bind(campus_id.as_uuid())
    .bind(only.map(|steward_id| steward_id.as_uuid()))
    .fetch_all(executor)
    .await?;
    Ok(rows.into_iter().map(profile_from_row).collect())
}

pub async fn steward_profiles(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
) -> Result<Vec<StewardProfile>, sqlx::Error> {
    profiles(executor, campus_id, None).await
}

async fn profile_of(
    connection: &mut PgConnection,
    campus_id: CampusId,
    steward_id: AccountId,
) -> Result<Option<StewardProfile>, sqlx::Error> {
    Ok(profiles(&mut *connection, campus_id, Some(steward_id))
        .await?
        .into_iter()
        .next())
}

pub async fn steward_room_assigned(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    steward_id: AccountId,
    room_id: RoomId,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT 1 FROM steward_rooms r JOIN stewards s \
         ON s.campus_id = r.campus_id AND s.steward_id = r.steward_id \
         WHERE r.campus_id = $1 AND r.steward_id = $2 AND r.room_id = $3 \
         AND s.revoked_at IS NULL)",
    )
    .bind(campus_id.as_uuid())
    .bind(steward_id.as_uuid())
    .bind(room_id.as_uuid())
    .fetch_one(executor)
    .await
}

pub async fn assigned_rooms(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    steward_id: AccountId,
) -> Result<Vec<Room>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT rm.room_id, rm.slug FROM steward_rooms r JOIN stewards s \
         ON s.campus_id = r.campus_id AND s.steward_id = r.steward_id \
         JOIN rooms rm ON rm.campus_id = r.campus_id AND rm.room_id = r.room_id \
         WHERE r.campus_id = $1 AND r.steward_id = $2 AND s.revoked_at IS NULL \
         ORDER BY rm.slug",
    )
    .bind(campus_id.as_uuid())
    .bind(steward_id.as_uuid())
    .fetch_all(executor)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(room_id, slug)| Room {
            room_id: RoomId(room_id),
            campus_id,
            slug,
        })
        .collect())
}

pub async fn credential_current(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    steward_id: AccountId,
    credential_revision: u64,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT 1 FROM stewards WHERE campus_id = $1 AND steward_id = $2 \
         AND revoked_at IS NULL AND credential_revision = $3)",
    )
    .bind(campus_id.as_uuid())
    .bind(steward_id.as_uuid())
    .bind(i64::try_from(credential_revision).unwrap_or(i64::MAX))
    .fetch_one(executor)
    .await
}

pub async fn display_name_of(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    account_id: AccountId,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>(
        "SELECT a.display_name FROM accounts a JOIN campus_members m \
         ON m.account_id = a.account_id \
         WHERE m.campus_id = $1 AND a.account_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .fetch_optional(executor)
    .await
}

pub async fn mentioned_stewards(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    message: &RoomMessage,
) -> Result<Vec<AccountId>, sqlx::Error> {
    let mentioned: Vec<Uuid> = message
        .mentions
        .iter()
        .map(|mention| mention.account_id.as_uuid())
        .collect();
    if mentioned.is_empty() {
        return Ok(Vec::new());
    }
    let rows = sqlx::query_scalar::<_, Uuid>(
        "SELECT s.steward_id FROM stewards s JOIN steward_rooms r \
         ON r.campus_id = s.campus_id AND r.steward_id = s.steward_id \
         WHERE s.campus_id = $1 AND r.room_id = $2 AND s.steward_id = ANY($3) \
         AND s.revoked_at IS NULL \
         AND EXISTS (SELECT 1 FROM accounts author WHERE author.account_id = $4 \
         AND author.kind = 'person') \
         ORDER BY s.steward_id",
    )
    .bind(campus_id.as_uuid())
    .bind(message.room_id.as_uuid())
    .bind(&mentioned)
    .bind(message.author_id.as_uuid())
    .fetch_all(executor)
    .await?;
    Ok(rows.into_iter().map(AccountId).collect())
}

pub async fn work_items(
    db: &PgPool,
    campus_id: CampusId,
    steward_id: AccountId,
    limit: usize,
) -> Result<WorkSynced, sqlx::Error> {
    let mut transaction = db
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ")
        .await?;
    let cursor = sqlx::query_scalar::<_, Option<Uuid>>(
        "SELECT work_cursor FROM stewards \
         WHERE campus_id = $1 AND steward_id = $2 AND revoked_at IS NULL",
    )
    .bind(campus_id.as_uuid())
    .bind(steward_id.as_uuid())
    .fetch_optional(&mut *transaction)
    .await?
    .flatten();
    let page = i64::try_from(limit).unwrap_or(i64::MAX);
    let mut ids = sqlx::query_scalar::<_, Uuid>(
        "SELECT mm.message_id FROM message_mentions mm \
         JOIN steward_rooms r ON r.campus_id = mm.campus_id AND r.room_id = mm.room_id \
         AND r.steward_id = $2 \
         JOIN stewards s ON s.campus_id = r.campus_id AND s.steward_id = r.steward_id \
         AND s.revoked_at IS NULL \
         JOIN room_messages m ON m.campus_id = mm.campus_id AND m.room_id = mm.room_id \
         AND m.message_id = mm.message_id \
         JOIN accounts a ON a.account_id = m.author_id \
         WHERE mm.campus_id = $1 AND mm.account_id = $2 AND a.kind = 'person' \
         AND ($3::uuid IS NULL OR mm.message_id > $3) \
         ORDER BY mm.message_id LIMIT $4",
    )
    .bind(campus_id.as_uuid())
    .bind(steward_id.as_uuid())
    .bind(cursor)
    .bind(page.saturating_add(1))
    .fetch_all(&mut *transaction)
    .await?;
    let has_more = ids.len() > limit;
    ids.truncate(limit);
    let message_ids: Vec<MessageId> = ids.into_iter().map(MessageId).collect();
    let items = fetch_messages(&mut transaction, campus_id, &message_ids)
        .await?
        .into_iter()
        .map(|message| StewardWorkItem { message })
        .collect();
    transaction.commit().await?;
    Ok(WorkSynced { items, has_more })
}

pub async fn ack_work(
    db: &PgPool,
    campus_id: CampusId,
    steward_id: AccountId,
    through: MessageId,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE stewards SET work_cursor = $3 \
         WHERE campus_id = $1 AND steward_id = $2 \
         AND (work_cursor IS NULL OR $3 > work_cursor)",
    )
    .bind(campus_id.as_uuid())
    .bind(steward_id.as_uuid())
    .bind(through.as_uuid())
    .execute(db)
    .await?;
    Ok(())
}

async fn rooms_exist(
    connection: &mut PgConnection,
    campus_id: CampusId,
    room_ids: &[RoomId],
) -> Result<bool, sqlx::Error> {
    let ids: Vec<Uuid> = room_ids.iter().map(|room_id| room_id.as_uuid()).collect();
    let found = sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(format!(
        "SELECT count(*) FROM rooms r WHERE r.campus_id = $1 AND r.room_id = ANY($2) \
         AND {NOT_A_NOTE}"
    )))
    .bind(campus_id.as_uuid())
    .bind(&ids)
    .fetch_one(connection)
    .await?;
    Ok(usize::try_from(found).is_ok_and(|found| found == room_ids.len()))
}

async fn replace_rooms(
    connection: &mut PgConnection,
    campus_id: CampusId,
    steward_id: AccountId,
    room_ids: &[RoomId],
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM steward_rooms WHERE campus_id = $1 AND steward_id = $2")
        .bind(campus_id.as_uuid())
        .bind(steward_id.as_uuid())
        .execute(&mut *connection)
        .await?;
    for room_id in room_ids {
        sqlx::query(
            "INSERT INTO steward_rooms (campus_id, steward_id, room_id) VALUES ($1, $2, $3)",
        )
        .bind(campus_id.as_uuid())
        .bind(steward_id.as_uuid())
        .bind(room_id.as_uuid())
        .execute(&mut *connection)
        .await?;
    }
    Ok(())
}

async fn create(
    connection: &mut PgConnection,
    campus_id: CampusId,
    request: &ManageSteward,
) -> Result<StewardOutcome, StewardFailure> {
    let display_name = request
        .display_name
        .as_deref()
        .map(normalize_display_name)
        .and_then(Result::ok)
        .ok_or_else(|| rejected(StewardRejection::InvalidDisplayName))?;
    let room_ids = normalize_room_ids(&request.room_ids)
        .map_err(|_| rejected(StewardRejection::InvalidRooms))?;
    sqlx::query_scalar::<_, i32>("SELECT 1 FROM campuses WHERE campus_id = $1 FOR NO KEY UPDATE")
        .bind(campus_id.as_uuid())
        .fetch_one(&mut *connection)
        .await?;
    if !rooms_exist(connection, campus_id, &room_ids).await? {
        return Err(rejected(StewardRejection::InvalidRooms));
    }
    let active = sqlx::query_scalar::<_, i64>(
        "SELECT count(*) FROM stewards WHERE campus_id = $1 AND revoked_at IS NULL",
    )
    .bind(campus_id.as_uuid())
    .fetch_one(&mut *connection)
    .await?;
    if usize::try_from(active).is_ok_and(|active| active >= STEWARDS_PER_CAMPUS_MAX) {
        return Err(rejected(StewardRejection::TooManyStewards));
    }
    let steward_id = AccountId::generate();
    let token = mint_steward_token();
    sqlx::query(
        "INSERT INTO accounts (account_id, email, display_name, password_hash, kind) \
         VALUES ($1, NULL, $2, NULL, 'steward')",
    )
    .bind(steward_id.as_uuid())
    .bind(&display_name)
    .execute(&mut *connection)
    .await?;
    sqlx::query("INSERT INTO campus_members (campus_id, account_id, role) VALUES ($1, $2, $3)")
        .bind(campus_id.as_uuid())
        .bind(steward_id.as_uuid())
        .bind(pulse_protocol::Role::Steward.as_str())
        .execute(&mut *connection)
        .await?;
    sqlx::query("INSERT INTO stewards (campus_id, steward_id, token_hash) VALUES ($1, $2, $3)")
        .bind(campus_id.as_uuid())
        .bind(steward_id.as_uuid())
        .bind(token_digest(&token))
        .execute(&mut *connection)
        .await?;
    replace_rooms(connection, campus_id, steward_id, &room_ids).await?;
    let profile = profile_of(connection, campus_id, steward_id)
        .await?
        .ok_or(StewardFailure::Database(sqlx::Error::RowNotFound))?;
    Ok(StewardOutcome::Issued(profile, token))
}

async fn lock_steward(
    connection: &mut PgConnection,
    campus_id: CampusId,
    steward_id: Option<AccountId>,
) -> Result<(AccountId, bool), StewardFailure> {
    let steward_id = steward_id.ok_or_else(|| rejected(StewardRejection::UnknownSteward))?;
    let revoked = sqlx::query_scalar::<_, bool>(
        "SELECT revoked_at IS NOT NULL FROM stewards \
         WHERE campus_id = $1 AND steward_id = $2 FOR UPDATE",
    )
    .bind(campus_id.as_uuid())
    .bind(steward_id.as_uuid())
    .fetch_optional(connection)
    .await?
    .ok_or_else(|| rejected(StewardRejection::UnknownSteward))?;
    Ok((steward_id, revoked))
}

async fn apply_existing(
    connection: &mut PgConnection,
    campus_id: CampusId,
    request: &ManageSteward,
) -> Result<StewardOutcome, StewardFailure> {
    let (steward_id, revoked) = lock_steward(connection, campus_id, request.steward_id).await?;
    let mut issued = None;
    match request.action {
        StewardAction::Revoke => {
            sqlx::query(
                "UPDATE stewards SET revoked_at = now(), \
                 credential_revision = credential_revision + 1 \
                 WHERE campus_id = $1 AND steward_id = $2 AND revoked_at IS NULL",
            )
            .bind(campus_id.as_uuid())
            .bind(steward_id.as_uuid())
            .execute(&mut *connection)
            .await?;
        }
        StewardAction::RotateToken => {
            if revoked {
                return Err(rejected(StewardRejection::Revoked));
            }
            let token = mint_steward_token();
            sqlx::query(
                "UPDATE stewards SET token_hash = $3, \
                 credential_revision = credential_revision + 1 \
                 WHERE campus_id = $1 AND steward_id = $2",
            )
            .bind(campus_id.as_uuid())
            .bind(steward_id.as_uuid())
            .bind(token_digest(&token))
            .execute(&mut *connection)
            .await?;
            issued = Some(token);
        }
        StewardAction::AssignRooms => {
            if revoked {
                return Err(rejected(StewardRejection::Revoked));
            }
            let room_ids = normalize_room_ids(&request.room_ids)
                .map_err(|_| rejected(StewardRejection::InvalidRooms))?;
            if !rooms_exist(connection, campus_id, &room_ids).await? {
                return Err(rejected(StewardRejection::InvalidRooms));
            }
            replace_rooms(connection, campus_id, steward_id, &room_ids).await?;
        }
        StewardAction::Create => return Err(rejected(StewardRejection::UnknownSteward)),
    }
    let profile = profile_of(connection, campus_id, steward_id)
        .await?
        .ok_or_else(|| rejected(StewardRejection::UnknownSteward))?;
    Ok(match issued {
        Some(token) => StewardOutcome::Issued(profile, token),
        None => StewardOutcome::Changed(profile),
    })
}

pub async fn manage_steward(
    db: &PgPool,
    campus_id: CampusId,
    actor: AccountId,
    request: ManageSteward,
) -> Result<StewardOutcome, StewardFailure> {
    let mut transaction = db.begin().await?;
    let permitted = campus_role(&mut *transaction, campus_id, actor)
        .await?
        .is_some_and(|role| role_permits(role, Permission::ManageStewards));
    if !permitted {
        return Err(rejected(StewardRejection::Forbidden));
    }
    let outcome = match request.action {
        StewardAction::Create => create(&mut transaction, campus_id, &request).await?,
        _ => apply_existing(&mut transaction, campus_id, &request).await?,
    };
    transaction.commit().await?;
    Ok(outcome)
}
