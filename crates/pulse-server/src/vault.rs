use std::collections::HashMap;

use pulse_protocol::{
    Attachment, AttachmentId, AttachmentKind, CampusId, MessageId, RoomMessage, SyncVault,
    VAULT_PAGE_LIMIT, VaultItem, VaultSynced, VaultTab,
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::message::fetch_messages;

type AttachmentRow = (Uuid, Uuid, String, String, String, i64);

fn tab_kinds(tab: VaultTab) -> Vec<&'static str> {
    match tab {
        VaultTab::Media => vec![
            AttachmentKind::Image.as_str(),
            AttachmentKind::Video.as_str(),
            AttachmentKind::Audio.as_str(),
        ],
        VaultTab::Files => vec![AttachmentKind::File.as_str()],
        VaultTab::Links | VaultTab::Pinned => Vec::new(),
    }
}

fn page_bound(request: &SyncVault) -> Option<Uuid> {
    request.before.map(|before| before.as_uuid())
}

fn paged_ids(rows: Vec<Uuid>) -> (Vec<Uuid>, bool) {
    let has_more = rows.len() > VAULT_PAGE_LIMIT;
    let mut ids = rows;
    ids.truncate(VAULT_PAGE_LIMIT);
    (ids, has_more)
}

fn page_limit() -> i64 {
    i64::try_from(VAULT_PAGE_LIMIT.saturating_add(1)).unwrap_or(i64::MAX)
}

fn attachment_of(row: AttachmentRow) -> Result<(Uuid, Attachment), sqlx::Error> {
    let (message_id, attachment_id, file_name, content_type, kind, byte_size) = row;
    let kind = kind
        .parse()
        .map_err(|error| sqlx::Error::Decode(Box::new(error)))?;
    Ok((
        message_id,
        Attachment {
            attachment_id: AttachmentId(attachment_id),
            file_name,
            content_type,
            kind,
            byte_size: u64::try_from(byte_size).unwrap_or(0),
        },
    ))
}

async fn attachment_items(
    connection: &mut PgConnection,
    campus_id: CampusId,
    request: &SyncVault,
) -> Result<(Vec<VaultItem>, Option<MessageId>), sqlx::Error> {
    let kinds = tab_kinds(request.tab);
    let candidates = sqlx::query_scalar::<_, Uuid>(
        "SELECT DISTINCT message_id FROM attachments \
         WHERE campus_id = $1 AND room_id = $2 AND kind = ANY($3) AND message_id IS NOT NULL \
         AND ($4::uuid IS NULL OR message_id < $4) \
         ORDER BY message_id DESC LIMIT $5",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(&kinds)
    .bind(page_bound(request))
    .bind(page_limit())
    .fetch_all(&mut *connection)
    .await?;
    let (ids, has_more) = paged_ids(candidates);
    let rows = sqlx::query_as::<_, AttachmentRow>(
        "SELECT message_id, attachment_id, file_name, content_type, kind, byte_size \
         FROM attachments \
         WHERE campus_id = $1 AND room_id = $2 AND message_id = ANY($3) AND kind = ANY($4) \
         ORDER BY message_id DESC, position",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(&ids)
    .bind(&kinds)
    .fetch_all(&mut *connection)
    .await?;
    let typed = ids.iter().copied().map(MessageId).collect::<Vec<_>>();
    let messages = load_messages(connection, campus_id, &typed).await?;
    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let (message_id, attachment) = attachment_of(row)?;
        if let Some(message) = messages.get(&MessageId(message_id)) {
            items.push(VaultItem {
                message: message.clone(),
                attachment: Some(attachment),
                link: None,
            });
        }
    }
    let next_before = has_more.then(|| typed.last().copied()).flatten();
    Ok((items, next_before))
}

async fn link_items(
    connection: &mut PgConnection,
    campus_id: CampusId,
    request: &SyncVault,
) -> Result<(Vec<VaultItem>, Option<MessageId>), sqlx::Error> {
    let candidates = sqlx::query_scalar::<_, Uuid>(
        "SELECT DISTINCT message_id FROM message_links \
         WHERE campus_id = $1 AND room_id = $2 \
         AND ($3::uuid IS NULL OR message_id < $3) \
         ORDER BY message_id DESC LIMIT $4",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(page_bound(request))
    .bind(page_limit())
    .fetch_all(&mut *connection)
    .await?;
    let (ids, has_more) = paged_ids(candidates);
    let rows = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT message_id, url FROM message_links \
         WHERE campus_id = $1 AND room_id = $2 AND message_id = ANY($3) \
         ORDER BY message_id DESC, position",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .bind(&ids)
    .fetch_all(&mut *connection)
    .await?;
    let typed = ids.iter().copied().map(MessageId).collect::<Vec<_>>();
    let messages = load_messages(connection, campus_id, &typed).await?;
    let items = rows
        .into_iter()
        .filter_map(|(message_id, url)| {
            messages
                .get(&MessageId(message_id))
                .map(|message| VaultItem {
                    message: message.clone(),
                    attachment: None,
                    link: Some(url),
                })
        })
        .collect();
    let next_before = has_more.then(|| typed.last().copied()).flatten();
    Ok((items, next_before))
}

async fn pinned_items(
    connection: &mut PgConnection,
    campus_id: CampusId,
    request: &SyncVault,
) -> Result<Vec<VaultItem>, sqlx::Error> {
    let ids = sqlx::query_scalar::<_, Uuid>(
        "SELECT message_id FROM message_pins \
         WHERE campus_id = $1 AND room_id = $2 \
         ORDER BY pinned_at DESC, message_id DESC",
    )
    .bind(campus_id.as_uuid())
    .bind(request.room_id.as_uuid())
    .fetch_all(&mut *connection)
    .await?;
    let typed = ids.into_iter().map(MessageId).collect::<Vec<_>>();
    let messages = load_messages(connection, campus_id, &typed).await?;
    Ok(typed
        .iter()
        .filter_map(|message_id| messages.get(message_id))
        .map(|message| VaultItem {
            message: message.clone(),
            attachment: None,
            link: None,
        })
        .collect())
}

async fn load_messages(
    connection: &mut PgConnection,
    campus_id: CampusId,
    ids: &[MessageId],
) -> Result<HashMap<MessageId, RoomMessage>, sqlx::Error> {
    Ok(fetch_messages(connection, campus_id, ids)
        .await?
        .into_iter()
        .map(|message| (message.message_id, message))
        .collect())
}

pub async fn sync_vault(
    db: &PgPool,
    campus_id: CampusId,
    request: &SyncVault,
) -> Result<Option<VaultSynced>, sqlx::Error> {
    let mut transaction = db
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ")
        .await?;
    let room =
        sqlx::query_scalar::<_, i32>("SELECT 1 FROM rooms WHERE campus_id = $1 AND room_id = $2")
            .bind(campus_id.as_uuid())
            .bind(request.room_id.as_uuid())
            .fetch_optional(&mut *transaction)
            .await?;
    if room.is_none() {
        return Ok(None);
    }
    let (items, next_before) = match request.tab {
        VaultTab::Media | VaultTab::Files => {
            attachment_items(&mut transaction, campus_id, request).await?
        }
        VaultTab::Links => link_items(&mut transaction, campus_id, request).await?,
        VaultTab::Pinned => (
            pinned_items(&mut transaction, campus_id, request).await?,
            None,
        ),
    };
    transaction.commit().await?;
    Ok(Some(VaultSynced {
        room_id: request.room_id,
        tab: request.tab,
        items,
        next_before,
    }))
}
