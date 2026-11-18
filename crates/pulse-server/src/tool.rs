use pulse_protocol::{
    AccountId, CallTool, CampusId, HistoryRead, ListRoomsArguments, MessagePosted,
    MessageRejection, Permission, PostMessageArguments, ReadRoomHistoryArguments,
    ReadThreadArguments, Room, RoomId, RoomsListed, SendMessage, StewardTool, SyncThread,
    TOOL_ARGUMENTS_MAX_BYTES, TOOL_HISTORY_LIMIT_MAX, ToolContent, ToolContentKind, ToolErrorCode,
    ToolResult, role_permits,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::campus::campus_role;
use crate::message::{PostFailure, PostOutcome, post_message, room_history, sync_thread};
use crate::state::AppState;
use crate::steward::steward_room_assigned;
use crate::ws::publish_created;

const DEFAULT_HISTORY_LIMIT: u32 = 20;

fn failure(request: &CallTool, code: ToolErrorCode) -> ToolResult {
    ToolResult {
        call_id: request.call_id,
        is_error: true,
        content: vec![ToolContent {
            kind: ToolContentKind::Text,
            text: code.as_str().to_owned(),
        }],
        structured_content: None,
    }
}

fn success<T: Serialize>(request: &CallTool, payload: &T) -> ToolResult {
    match serde_json::to_string(payload) {
        Ok(text) => ToolResult {
            call_id: request.call_id,
            is_error: false,
            content: vec![ToolContent {
                kind: ToolContentKind::Text,
                text: text.clone(),
            }],
            structured_content: Some(text),
        },
        Err(_) => failure(request, ToolErrorCode::Rejected),
    }
}

fn arguments<T: DeserializeOwned>(request: &CallTool) -> Result<T, ToolErrorCode> {
    serde_json::from_str(&request.arguments).map_err(|_| ToolErrorCode::InvalidArguments)
}

async fn require_room(
    state: &AppState,
    campus_id: CampusId,
    steward_id: AccountId,
    room_id: RoomId,
) -> Result<(), ToolErrorCode> {
    match steward_room_assigned(&state.db, campus_id, steward_id, room_id).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(ToolErrorCode::UnknownRoom),
        Err(error) => {
            tracing::warn!(%error, "tool room check failed");
            Err(ToolErrorCode::Rejected)
        }
    }
}

fn database_failure(error: sqlx::Error) -> ToolErrorCode {
    tracing::warn!(%error, "tool query failed");
    ToolErrorCode::Rejected
}

async fn list_rooms(
    state: &AppState,
    campus_id: CampusId,
    steward_id: AccountId,
    request: &CallTool,
) -> Result<ToolResult, ToolErrorCode> {
    arguments::<ListRoomsArguments>(request)?;
    let rows = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT r.room_id, r.slug FROM rooms r JOIN steward_rooms a \
         ON a.campus_id = r.campus_id AND a.room_id = r.room_id \
         WHERE r.campus_id = $1 AND a.steward_id = $2 ORDER BY r.slug",
    )
    .bind(campus_id.as_uuid())
    .bind(steward_id.as_uuid())
    .fetch_all(&state.db)
    .await
    .map_err(database_failure)?;
    let rooms = rows
        .into_iter()
        .map(|(room_id, slug)| Room {
            room_id: RoomId(room_id),
            campus_id,
            slug,
        })
        .collect();
    Ok(success(request, &RoomsListed { rooms }))
}

async fn read_history(
    state: &AppState,
    campus_id: CampusId,
    steward_id: AccountId,
    request: &CallTool,
) -> Result<ToolResult, ToolErrorCode> {
    let input = arguments::<ReadRoomHistoryArguments>(request)?;
    let limit = input.limit.unwrap_or(DEFAULT_HISTORY_LIMIT);
    if limit == 0 {
        return Err(ToolErrorCode::InvalidArguments);
    }
    require_room(state, campus_id, steward_id, input.room_id).await?;
    let limit = usize::try_from(limit.min(TOOL_HISTORY_LIMIT_MAX)).unwrap_or(1);
    let mut connection = state.db.acquire().await.map_err(database_failure)?;
    let (messages, has_more) = room_history(
        &mut connection,
        campus_id,
        input.room_id,
        input.before,
        limit,
    )
    .await
    .map_err(database_failure)?;
    Ok(success(request, &HistoryRead { messages, has_more }))
}

async fn read_thread(
    state: &AppState,
    campus_id: CampusId,
    steward_id: AccountId,
    request: &CallTool,
) -> Result<ToolResult, ToolErrorCode> {
    let input = arguments::<ReadThreadArguments>(request)?;
    require_room(state, campus_id, steward_id, input.room_id).await?;
    let synced = sync_thread(
        &state.db,
        campus_id,
        &SyncThread {
            room_id: input.room_id,
            root_message_id: input.root_message_id,
            after: None,
        },
    )
    .await
    .map_err(database_failure)?
    .ok_or(ToolErrorCode::UnknownThread)?;
    Ok(success(
        request,
        &HistoryRead {
            messages: synced.messages,
            has_more: synced.has_more,
        },
    ))
}

async fn post(
    state: &AppState,
    campus_id: CampusId,
    steward_id: AccountId,
    request: &CallTool,
) -> Result<ToolResult, ToolErrorCode> {
    let input = arguments::<PostMessageArguments>(request)?;
    require_room(state, campus_id, steward_id, input.room_id).await?;
    let outcome = post_message(
        &state.db,
        campus_id,
        steward_id,
        SendMessage {
            room_id: input.room_id,
            client_nonce: input.client_nonce,
            body: input.body,
            thread_root_id: input.thread_root_id,
            attachment_ids: Vec::new(),
        },
    )
    .await;
    match outcome {
        Ok(PostOutcome::Created(message, affected)) => {
            publish_created(state, campus_id, message.clone(), &affected).await;
            Ok(success(request, &MessagePosted { message }))
        }
        Ok(PostOutcome::Replayed(message)) => Ok(success(request, &MessagePosted { message })),
        Err(PostFailure::Rejected(MessageRejection::UnknownThread)) => {
            Err(ToolErrorCode::UnknownThread)
        }
        Err(PostFailure::Rejected(MessageRejection::UnknownRoom)) => {
            Err(ToolErrorCode::UnknownRoom)
        }
        Err(PostFailure::Rejected(MessageRejection::Forbidden)) => Err(ToolErrorCode::Forbidden),
        Err(PostFailure::Rejected(_)) => Err(ToolErrorCode::InvalidArguments),
        Err(PostFailure::Database(error)) => Err(database_failure(error)),
    }
}

async fn dispatch(
    state: &AppState,
    campus_id: CampusId,
    steward_id: AccountId,
    request: &CallTool,
) -> Result<ToolResult, ToolErrorCode> {
    if request.arguments.len() > TOOL_ARGUMENTS_MAX_BYTES {
        return Err(ToolErrorCode::InvalidArguments);
    }
    let permitted = campus_role(&state.db, campus_id, steward_id)
        .await
        .map_err(database_failure)?
        .is_some_and(|role| role_permits(role, Permission::UseStewardTools));
    if !permitted {
        return Err(ToolErrorCode::Forbidden);
    }
    match StewardTool::from_name(&request.name).ok_or(ToolErrorCode::UnknownTool)? {
        StewardTool::ListRooms => list_rooms(state, campus_id, steward_id, request).await,
        StewardTool::ReadRoomHistory => read_history(state, campus_id, steward_id, request).await,
        StewardTool::ReadThread => read_thread(state, campus_id, steward_id, request).await,
        StewardTool::PostMessage => post(state, campus_id, steward_id, request).await,
    }
}

pub async fn call_tool(
    state: &AppState,
    campus_id: CampusId,
    steward_id: AccountId,
    request: CallTool,
) -> ToolResult {
    match dispatch(state, campus_id, steward_id, &request).await {
        Ok(result) => result,
        Err(code) => failure(&request, code),
    }
}
