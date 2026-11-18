use pulse_protocol::{
    AccountId, CampusId, Permission, REACTION_KINDS_MAX, React, ReactionAction, ReactionRejection,
    RoomMessage, normalize_reaction_emoji, role_permits,
};
use sqlx::PgPool;

use crate::campus::campus_role;
use crate::message::{fetch_message, lock_room, message_root, stamp_activity};
use crate::thread::auto_follow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReactionOutcome {
    Changed(RoomMessage, Vec<AccountId>),
    Unchanged(RoomMessage),
}

#[derive(Debug)]
pub enum ReactionFailure {
    Rejected(ReactionRejection),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for ReactionFailure {
    fn from(error: sqlx::Error) -> Self {
        ReactionFailure::Database(error)
    }
}

fn rejected(reason: ReactionRejection) -> ReactionFailure {
    ReactionFailure::Rejected(reason)
}

pub async fn apply_reaction(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    request: React,
) -> Result<ReactionOutcome, ReactionFailure> {
    let mut transaction = db.begin().await?;
    if !lock_room(&mut transaction, campus_id, request.room_id).await? {
        return Err(rejected(ReactionRejection::UnknownMessage));
    }
    let permitted = campus_role(&mut *transaction, campus_id, account_id)
        .await?
        .is_some_and(|role| role_permits(role, Permission::React));
    if !permitted {
        return Err(rejected(ReactionRejection::Forbidden));
    }
    let emoji = normalize_reaction_emoji(&request.emoji)
        .map_err(|_| rejected(ReactionRejection::InvalidEmoji))?;
    let thread_root = message_root(
        &mut transaction,
        campus_id,
        request.room_id,
        request.message_id,
    )
    .await?
    .ok_or_else(|| rejected(ReactionRejection::UnknownMessage))?;
    let changed = match request.action {
        ReactionAction::Add => {
            let kinds = sqlx::query_scalar::<_, String>(
                "SELECT DISTINCT emoji FROM message_reactions \
                 WHERE campus_id = $1 AND message_id = $2",
            )
            .bind(campus_id.as_uuid())
            .bind(request.message_id.as_uuid())
            .fetch_all(&mut *transaction)
            .await?;
            if !kinds.contains(&emoji) && kinds.len() >= REACTION_KINDS_MAX {
                return Err(rejected(ReactionRejection::TooManyKinds));
            }
            sqlx::query(
                "INSERT INTO message_reactions \
                 (campus_id, room_id, message_id, account_id, emoji) \
                 VALUES ($1, $2, $3, $4, $5) \
                 ON CONFLICT (campus_id, message_id, account_id, emoji) DO NOTHING",
            )
            .bind(campus_id.as_uuid())
            .bind(request.room_id.as_uuid())
            .bind(request.message_id.as_uuid())
            .bind(account_id.as_uuid())
            .bind(&emoji)
            .execute(&mut *transaction)
            .await?
            .rows_affected()
                > 0
        }
        ReactionAction::Remove => {
            sqlx::query(
                "DELETE FROM message_reactions \
                 WHERE campus_id = $1 AND message_id = $2 AND account_id = $3 AND emoji = $4",
            )
            .bind(campus_id.as_uuid())
            .bind(request.message_id.as_uuid())
            .bind(account_id.as_uuid())
            .bind(&emoji)
            .execute(&mut *transaction)
            .await?
            .rows_affected()
                > 0
        }
    };
    let mut affected = Vec::new();
    if changed {
        stamp_activity(
            &mut transaction,
            campus_id,
            request.room_id,
            request.message_id,
        )
        .await?;
        if request.action == ReactionAction::Add
            && auto_follow(
                &mut transaction,
                campus_id,
                request.room_id,
                thread_root.unwrap_or(request.message_id),
                account_id,
                None,
            )
            .await?
        {
            affected.push(account_id);
        }
    }
    let message = fetch_message(&mut transaction, campus_id, request.message_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    transaction.commit().await?;
    Ok(if changed {
        ReactionOutcome::Changed(message, affected)
    } else {
        ReactionOutcome::Unchanged(message)
    })
}
