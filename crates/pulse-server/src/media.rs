use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use pulse_protocol::{
    AccountId, ApiError, ApiErrorCode, Attachment, AttachmentId, AttachmentKind, CampusId,
    CreateUploadRequest, DOWNLOAD_TICKETS_MAX, DownloadTicket, DownloadTickets,
    DownloadTicketsRequest, InputField, MessageId, MessageRejection, Permission, RoomId,
    UploadTicket, Validate, normalize_content_type, normalize_file_name, validate_byte_size,
};
use uuid::Uuid;

use crate::auth::Authenticated;
use crate::campus::require_permission;
use crate::error::{ApiFailure, ApiJson};
use crate::message::PostFailure;
use crate::note::room_accessible;
use crate::state::AppState;
use crate::storage::ObjectLocation;

fn not_found() -> ApiFailure {
    ApiFailure(ApiError::new(ApiErrorCode::NotFound))
}

fn unavailable() -> ApiFailure {
    ApiFailure(ApiError::new(ApiErrorCode::Unavailable))
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct AttachmentRow {
    pub(crate) message_id: Uuid,
    pub(crate) room_id: Uuid,
    pub(crate) attachment_id: Uuid,
    pub(crate) file_name: String,
    pub(crate) content_type: String,
    pub(crate) kind: String,
    pub(crate) byte_size: i64,
}

macro_rules! select_attachments {
    ($tail:literal) => {
        concat!(
            "SELECT message_id, room_id, attachment_id, file_name, content_type, kind, byte_size \
             FROM attachments ",
            $tail
        )
    };
}

pub(crate) use select_attachments;

pub(crate) fn attachment_from_row(
    row: AttachmentRow,
) -> Result<(MessageId, RoomId, Attachment), sqlx::Error> {
    let kind: AttachmentKind = row
        .kind
        .parse()
        .map_err(|error| sqlx::Error::Decode(Box::new(error)))?;
    Ok((
        MessageId(row.message_id),
        RoomId(row.room_id),
        Attachment {
            attachment_id: AttachmentId(row.attachment_id),
            file_name: row.file_name,
            content_type: row.content_type,
            kind,
            byte_size: u64::try_from(row.byte_size).unwrap_or(0),
        },
    ))
}

pub(crate) async fn room_exists(
    executor: impl sqlx::PgExecutor<'_>,
    campus_id: CampusId,
    room_id: RoomId,
) -> Result<bool, sqlx::Error> {
    let found =
        sqlx::query_scalar::<_, i32>("SELECT 1 FROM rooms WHERE campus_id = $1 AND room_id = $2")
            .bind(campus_id.as_uuid())
            .bind(room_id.as_uuid())
            .fetch_optional(executor)
            .await?;
    Ok(found.is_some())
}

pub async fn create_upload(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path((raw_campus_id, raw_room_id)): Path<(String, String)>,
    ApiJson(request): ApiJson<CreateUploadRequest>,
) -> Result<(StatusCode, Json<UploadTicket>), ApiFailure> {
    let campus_id: CampusId = raw_campus_id.parse().map_err(|_| not_found())?;
    let room_id: RoomId = raw_room_id.parse().map_err(|_| not_found())?;
    require_permission(
        &state.db,
        campus_id,
        authenticated.account_id,
        Permission::UploadAttachment,
    )
    .await?;
    request.validate()?;
    let file_name = normalize_file_name(&request.file_name).map_err(ApiError::invalid)?;
    let content_type = normalize_content_type(&request.content_type).map_err(ApiError::invalid)?;
    let byte_size = validate_byte_size(request.byte_size).map_err(ApiError::invalid)?;
    if !room_exists(&state.db, campus_id, room_id).await?
        || !room_accessible(&state.db, campus_id, room_id, authenticated.account_id).await?
    {
        return Err(not_found());
    }
    let attachment_id = AttachmentId::generate();
    let location = ObjectLocation {
        campus_id,
        room_id,
        attachment_id,
    };
    let (put_url, expires_at_ms) = state
        .storage
        .presign_put(&location, &content_type, byte_size)
        .map_err(|_| unavailable())?;
    sqlx::query(
        "INSERT INTO attachments \
         (campus_id, room_id, attachment_id, uploader_id, object_key, file_name, content_type, \
         kind, byte_size) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(campus_id.as_uuid())
    .bind(room_id.as_uuid())
    .bind(attachment_id.as_uuid())
    .bind(authenticated.account_id.as_uuid())
    .bind(location.key())
    .bind(&file_name)
    .bind(&content_type)
    .bind(AttachmentKind::from_content_type(&content_type).as_str())
    .bind(i64::try_from(byte_size).unwrap_or(i64::MAX))
    .execute(&state.db)
    .await?;
    Ok((
        StatusCode::CREATED,
        Json(UploadTicket {
            attachment_id,
            put_url,
            content_type,
            expires_at_ms,
        }),
    ))
}

pub async fn download_tickets(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path(raw_campus_id): Path<String>,
    ApiJson(request): ApiJson<DownloadTicketsRequest>,
) -> Result<Json<DownloadTickets>, ApiFailure> {
    let campus_id: CampusId = raw_campus_id.parse().map_err(|_| not_found())?;
    require_permission(
        &state.db,
        campus_id,
        authenticated.account_id,
        Permission::ViewRooms,
    )
    .await?;
    if request.attachment_ids.len() > DOWNLOAD_TICKETS_MAX {
        return Err(ApiFailure(ApiError::invalid(InputField::Attachments)));
    }
    let ids: Vec<Uuid> = request
        .attachment_ids
        .iter()
        .map(|id| id.as_uuid())
        .collect();
    let rows = sqlx::query_as::<_, AttachmentRow>(select_attachments!(
        "WHERE campus_id = $1 AND attachment_id = ANY($2) AND message_id IS NOT NULL"
    ))
    .bind(campus_id.as_uuid())
    .bind(&ids)
    .fetch_all(&state.db)
    .await?;
    let mut found: HashMap<AttachmentId, (RoomId, Attachment)> = HashMap::new();
    let mut access: HashMap<RoomId, bool> = HashMap::new();
    for row in rows {
        let (_, room_id, attachment) = attachment_from_row(row)?;
        let open = match access.get(&room_id) {
            Some(open) => *open,
            None => {
                let open = room_accessible(&state.db, campus_id, room_id, authenticated.account_id)
                    .await?;
                access.insert(room_id, open);
                open
            }
        };
        if open {
            found.insert(attachment.attachment_id, (room_id, attachment));
        }
    }
    let mut tickets = Vec::new();
    for attachment_id in request.attachment_ids {
        let Some((room_id, attachment)) = found.remove(&attachment_id) else {
            continue;
        };
        let location = ObjectLocation {
            campus_id,
            room_id,
            attachment_id,
        };
        let (url, expires_at_ms) = state
            .storage
            .presign_get(&location, &attachment.file_name, attachment.kind)
            .map_err(|_| unavailable())?;
        tickets.push(DownloadTicket {
            attachment_id,
            url,
            expires_at_ms,
        });
    }
    Ok(Json(DownloadTickets { tickets }))
}

fn invalid_attachment() -> PostFailure {
    PostFailure::Rejected(MessageRejection::InvalidAttachment)
}

pub async fn verify_uploads(
    state: &AppState,
    campus_id: CampusId,
    room_id: RoomId,
    uploader_id: AccountId,
    ids: &[AttachmentId],
) -> Result<(), PostFailure> {
    for attachment_id in ids {
        let declared = sqlx::query_scalar::<_, i64>(
            "SELECT byte_size FROM attachments \
             WHERE campus_id = $1 AND room_id = $2 AND attachment_id = $3 \
             AND uploader_id = $4 AND message_id IS NULL",
        )
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(attachment_id.as_uuid())
        .bind(uploader_id.as_uuid())
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(invalid_attachment)?;
        let location = ObjectLocation {
            campus_id,
            room_id,
            attachment_id: *attachment_id,
        };
        let stored = state
            .storage
            .head_size(&location)
            .await
            .map_err(|error| {
                tracing::warn!(%error, "attachment verification failed");
                invalid_attachment()
            })?
            .ok_or_else(invalid_attachment)?;
        if i64::try_from(stored).ok() != Some(declared) {
            if let Err(error) = state.storage.delete(&location).await {
                tracing::warn!(%error, "mismatched attachment cleanup failed");
            }
            return Err(invalid_attachment());
        }
    }
    Ok(())
}
