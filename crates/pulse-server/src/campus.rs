use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use pulse_protocol::{
    AccountId, ApiError, ApiErrorCode, CampusCreated, CampusId, CampusList, CampusSummary,
    CreateCampusRequest, CreateRoomRequest, DEFAULT_ROOM_SLUGS, InputField, Permission, Role,
    RoleParseError, Room, RoomId, RoomList, Validate, role_permits,
};
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

use crate::auth::Authenticated;
use crate::error::{ApiFailure, ApiJson, is_unique_violation};
use crate::state::AppState;

fn not_found() -> ApiFailure {
    ApiFailure(ApiError::new(ApiErrorCode::NotFound))
}

fn parse_campus_id(raw: &str) -> Result<CampusId, ApiFailure> {
    raw.parse().map_err(|_| not_found())
}

fn parse_role(text: &str) -> Result<Role, sqlx::Error> {
    text.parse()
        .map_err(|error: RoleParseError| sqlx::Error::Decode(Box::new(error)))
}

fn conflict_on_slug(error: sqlx::Error) -> ApiFailure {
    if is_unique_violation(&error) {
        ApiFailure(ApiError::conflict(InputField::Slug))
    } else {
        ApiFailure::from(error)
    }
}

pub async fn campus_role(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
) -> Result<Option<Role>, sqlx::Error> {
    let text = sqlx::query_scalar::<_, String>(
        "SELECT m.role FROM campus_members m \
         JOIN campuses c ON c.campus_id = m.campus_id \
         WHERE m.campus_id = $1 AND m.account_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .fetch_optional(db)
    .await?;
    text.as_deref().map(parse_role).transpose()
}

pub async fn require_permission(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    permission: Permission,
) -> Result<Role, ApiFailure> {
    let role = campus_role(db, campus_id, account_id)
        .await?
        .ok_or_else(not_found)?;
    if role_permits(role, permission) {
        Ok(role)
    } else {
        Err(ApiFailure(ApiError::new(ApiErrorCode::Forbidden)))
    }
}

async fn insert_room<'e, E>(
    executor: E,
    campus_id: CampusId,
    slug: &str,
) -> Result<Room, sqlx::Error>
where
    E: sqlx::Executor<'e, Database = Postgres>,
{
    let room_id = RoomId::generate();
    sqlx::query("INSERT INTO rooms (campus_id, room_id, slug) VALUES ($1, $2, $3)")
        .bind(campus_id.as_uuid())
        .bind(room_id.as_uuid())
        .bind(slug)
        .execute(executor)
        .await?;
    Ok(Room {
        room_id,
        campus_id,
        slug: slug.to_owned(),
    })
}

pub async fn list_campuses(
    State(state): State<AppState>,
    authenticated: Authenticated,
) -> Result<Json<CampusList>, ApiFailure> {
    let rows = sqlx::query_as::<_, (Uuid, String, String, String)>(
        "SELECT c.campus_id, c.slug, c.display_name, m.role FROM campus_members m \
         JOIN campuses c ON c.campus_id = m.campus_id \
         WHERE m.account_id = $1 \
         ORDER BY c.created_at, c.campus_id",
    )
    .bind(authenticated.account_id.as_uuid())
    .fetch_all(&state.db)
    .await?;
    let campuses = rows
        .into_iter()
        .map(|(campus_id, slug, display_name, role)| {
            Ok(CampusSummary {
                campus_id: CampusId(campus_id),
                slug,
                display_name,
                role: parse_role(&role)?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;
    Ok(Json(CampusList { campuses }))
}

pub async fn create_campus(
    State(state): State<AppState>,
    authenticated: Authenticated,
    ApiJson(request): ApiJson<CreateCampusRequest>,
) -> Result<(StatusCode, Json<CampusCreated>), ApiFailure> {
    request.validate()?;
    let campus_id = CampusId::generate();
    let mut transaction = state.db.begin().await?;
    sqlx::query("INSERT INTO campuses (campus_id, slug, display_name) VALUES ($1, $2, $3)")
        .bind(campus_id.as_uuid())
        .bind(&request.slug)
        .bind(&request.display_name)
        .execute(&mut *transaction)
        .await
        .map_err(conflict_on_slug)?;
    sqlx::query("INSERT INTO campus_members (campus_id, account_id, role) VALUES ($1, $2, $3)")
        .bind(campus_id.as_uuid())
        .bind(authenticated.account_id.as_uuid())
        .bind(Role::Owner.as_str())
        .execute(&mut *transaction)
        .await?;
    let mut rooms = Vec::with_capacity(DEFAULT_ROOM_SLUGS.len());
    for slug in DEFAULT_ROOM_SLUGS {
        rooms.push(insert_room(&mut *transaction, campus_id, slug).await?);
    }
    transaction.commit().await?;
    let campus = CampusSummary {
        campus_id,
        slug: request.slug,
        display_name: request.display_name,
        role: Role::Owner,
    };
    Ok((StatusCode::CREATED, Json(CampusCreated { campus, rooms })))
}

pub async fn list_rooms(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path(raw_campus_id): Path<String>,
) -> Result<Json<RoomList>, ApiFailure> {
    let campus_id = parse_campus_id(&raw_campus_id)?;
    require_permission(
        &state.db,
        campus_id,
        authenticated.account_id,
        Permission::ViewRooms,
    )
    .await?;
    let rows = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT room_id, slug FROM rooms WHERE campus_id = $1 ORDER BY slug",
    )
    .bind(campus_id.as_uuid())
    .fetch_all(&state.db)
    .await?;
    let rooms = rows
        .into_iter()
        .map(|(room_id, slug)| Room {
            room_id: RoomId(room_id),
            campus_id,
            slug,
        })
        .collect();
    Ok(Json(RoomList { campus_id, rooms }))
}

pub async fn create_room(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path(raw_campus_id): Path<String>,
    ApiJson(request): ApiJson<CreateRoomRequest>,
) -> Result<(StatusCode, Json<Room>), ApiFailure> {
    let campus_id = parse_campus_id(&raw_campus_id)?;
    require_permission(
        &state.db,
        campus_id,
        authenticated.account_id,
        Permission::CreateRoom,
    )
    .await?;
    request.validate()?;
    let room = insert_room(&state.db, campus_id, &request.slug)
        .await
        .map_err(conflict_on_slug)?;
    Ok((StatusCode::CREATED, Json(room)))
}
