use pulse_protocol::{
    AccountId, CampusId, Permission, PinRejection, ROOM_PINS_MAX, RoomMessage, SetPin, role_permits,
};
use sqlx::PgPool;

use crate::campus::campus_role;
use crate::message::{fetch_message, lock_room, message_root, stamp_activity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PinOutcome {
    Changed(RoomMessage),
    Unchanged(RoomMessage),
}

#[derive(Debug)]
pub enum PinFailure {
    Rejected(PinRejection),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for PinFailure {
    fn from(error: sqlx::Error) -> Self {
        PinFailure::Database(error)
    }
}

fn rejected(reason: PinRejection) -> PinFailure {
    PinFailure::Rejected(reason)
}

pub async fn set_pin(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    request: SetPin,
) -> Result<PinOutcome, PinFailure> {
    let mut transaction = db.begin().await?;
    if !lock_room(&mut transaction, campus_id, request.room_id).await? {
        return Err(rejected(PinRejection::UnknownMessage));
    }
    let permitted = campus_role(&mut *transaction, campus_id, account_id)
        .await?
        .is_some_and(|role| role_permits(role, Permission::PinMessage));
    if !permitted {
        return Err(rejected(PinRejection::Forbidden));
    }
    message_root(
        &mut transaction,
        campus_id,
        request.room_id,
        request.message_id,
    )
    .await?
    .ok_or_else(|| rejected(PinRejection::UnknownMessage))?;
    let changed = if request.pinned {
        let pinned = sqlx::query_scalar::<_, i32>(
            "SELECT 1 FROM message_pins \
             WHERE campus_id = $1 AND room_id = $2 AND message_id = $3",
        )
        .bind(campus_id.as_uuid())
        .bind(request.room_id.as_uuid())
        .bind(request.message_id.as_uuid())
        .fetch_optional(&mut *transaction)
        .await?
        .is_some();
        if pinned {
            false
        } else {
            let total = sqlx::query_scalar::<_, i64>(
                "SELECT count(*) FROM message_pins WHERE campus_id = $1 AND room_id = $2",
            )
            .bind(campus_id.as_uuid())
            .bind(request.room_id.as_uuid())
            .fetch_one(&mut *transaction)
            .await?;
            if usize::try_from(total).unwrap_or(usize::MAX) >= ROOM_PINS_MAX {
                return Err(rejected(PinRejection::LimitReached));
            }
            sqlx::query(
                "INSERT INTO message_pins (campus_id, room_id, message_id, pinned_by) \
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(campus_id.as_uuid())
            .bind(request.room_id.as_uuid())
            .bind(request.message_id.as_uuid())
            .bind(account_id.as_uuid())
            .execute(&mut *transaction)
            .await?;
            true
        }
    } else {
        sqlx::query(
            "DELETE FROM message_pins \
             WHERE campus_id = $1 AND room_id = $2 AND message_id = $3",
        )
        .bind(campus_id.as_uuid())
        .bind(request.room_id.as_uuid())
        .bind(request.message_id.as_uuid())
        .execute(&mut *transaction)
        .await?
        .rows_affected()
            > 0
    };
    if changed {
        stamp_activity(
            &mut transaction,
            campus_id,
            request.room_id,
            request.message_id,
        )
        .await?;
    }
    let message = fetch_message(&mut transaction, campus_id, request.message_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    transaction.commit().await?;
    Ok(if changed {
        PinOutcome::Changed(message)
    } else {
        PinOutcome::Unchanged(message)
    })
}
