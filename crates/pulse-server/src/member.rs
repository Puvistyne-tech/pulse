use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use pulse_protocol::{
    AccountId, AddMemberRequest, ApiError, ApiErrorCode, CampusId, CampusMember, InputField,
    MemberList, MembershipRevoked, Permission, Role, RoleParseError, ServerFrame,
    UpdateMemberRequest, Validate, member_manageable, normalize_email, role_assignable,
    role_permits,
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::auth::{Authenticated, account_by_email};
use crate::campus::{
    campus_role_locked, insert_membership, not_found, parse_campus_id, require_permission,
};
use crate::error::{ApiFailure, ApiJson};
use crate::publish::publish_frame;
use crate::state::AppState;

const MEMBER_COLUMNS: &str = "SELECT a.account_id, a.display_name, a.email, m.role \
     FROM campus_members m JOIN accounts a ON a.account_id = m.account_id \
     WHERE m.campus_id = $1 AND m.role <> $2";

type MemberRow = (Uuid, String, String, String);

fn forbidden() -> ApiFailure {
    ApiFailure(ApiError::new(ApiErrorCode::Forbidden))
}

fn member_of(campus_id: CampusId, row: MemberRow) -> Result<CampusMember, sqlx::Error> {
    let (account_id, display_name, email, role) = row;
    Ok(CampusMember {
        campus_id,
        account_id: AccountId(account_id),
        display_name,
        email,
        role: role
            .parse()
            .map_err(|error: RoleParseError| sqlx::Error::Decode(Box::new(error)))?,
    })
}

fn parse_account_id(raw: &str) -> Result<AccountId, ApiFailure> {
    raw.parse().map_err(|_| not_found())
}

async fn member_row(
    connection: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
) -> Result<Option<CampusMember>, sqlx::Error> {
    let query = format!("{MEMBER_COLUMNS} AND m.account_id = $3");
    sqlx::query_as::<_, MemberRow>(sqlx::AssertSqlSafe(query))
        .bind(campus_id.as_uuid())
        .bind(Role::Steward.as_str())
        .bind(account_id.as_uuid())
        .fetch_optional(connection)
        .await?
        .map(|row| member_of(campus_id, row))
        .transpose()
}

async fn actor_role(
    connection: &mut PgConnection,
    campus_id: CampusId,
    account_id: AccountId,
) -> Result<Role, ApiFailure> {
    let role = campus_role_locked(&mut *connection, campus_id, account_id)
        .await?
        .ok_or_else(not_found)?;
    if role_permits(role, Permission::ManageMembers) {
        Ok(role)
    } else {
        Err(forbidden())
    }
}

async fn manageable_target(
    connection: &mut PgConnection,
    campus_id: CampusId,
    actor: Role,
    account_id: AccountId,
) -> Result<Role, ApiFailure> {
    let target = campus_role_locked(&mut *connection, campus_id, account_id)
        .await?
        .ok_or_else(not_found)?;
    if member_manageable(actor, target) {
        Ok(target)
    } else {
        Err(forbidden())
    }
}

pub async fn list_members(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path(raw_campus_id): Path<String>,
) -> Result<Json<MemberList>, ApiFailure> {
    let campus_id = parse_campus_id(&raw_campus_id)?;
    require_permission(
        &state.db,
        campus_id,
        authenticated.account_id,
        Permission::ViewCampus,
    )
    .await?;
    let members = campus_members(&state.db, campus_id).await?;
    Ok(Json(MemberList { campus_id, members }))
}

async fn campus_members(
    db: &PgPool,
    campus_id: CampusId,
) -> Result<Vec<CampusMember>, sqlx::Error> {
    let query = format!("{MEMBER_COLUMNS} ORDER BY lower(a.display_name), a.account_id");
    sqlx::query_as::<_, MemberRow>(sqlx::AssertSqlSafe(query))
        .bind(campus_id.as_uuid())
        .bind(Role::Steward.as_str())
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|row| member_of(campus_id, row))
        .collect()
}

pub async fn add_member(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path(raw_campus_id): Path<String>,
    ApiJson(request): ApiJson<AddMemberRequest>,
) -> Result<(StatusCode, Json<CampusMember>), ApiFailure> {
    let campus_id = parse_campus_id(&raw_campus_id)?;
    let mut transaction = state.db.begin().await?;
    let actor = actor_role(&mut transaction, campus_id, authenticated.account_id).await?;
    request.validate()?;
    if !role_assignable(actor, request.role) {
        return Err(forbidden());
    }
    let email = normalize_email(&request.email).map_err(ApiError::invalid)?;
    let account_id = account_by_email(&mut *transaction, &email)
        .await?
        .ok_or(ApiFailure(ApiError {
            code: ApiErrorCode::NotFound,
            field: Some(InputField::Email),
        }))?;
    let inserted =
        insert_membership(&mut *transaction, campus_id, account_id, request.role).await?;
    let member = member_row(&mut transaction, campus_id, account_id)
        .await?
        .ok_or_else(forbidden)?;
    transaction.commit().await?;
    let status = if inserted {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };
    Ok((status, Json(member)))
}

pub async fn update_member(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path((raw_campus_id, raw_account_id)): Path<(String, String)>,
    ApiJson(request): ApiJson<UpdateMemberRequest>,
) -> Result<Json<CampusMember>, ApiFailure> {
    let campus_id = parse_campus_id(&raw_campus_id)?;
    let account_id = parse_account_id(&raw_account_id)?;
    let mut transaction = state.db.begin().await?;
    let actor = actor_role(&mut transaction, campus_id, authenticated.account_id).await?;
    request.validate()?;
    manageable_target(&mut transaction, campus_id, actor, account_id).await?;
    if account_id == authenticated.account_id || !role_assignable(actor, request.role) {
        return Err(forbidden());
    }
    sqlx::query("UPDATE campus_members SET role = $3 WHERE campus_id = $1 AND account_id = $2")
        .bind(campus_id.as_uuid())
        .bind(account_id.as_uuid())
        .bind(request.role.as_str())
        .execute(&mut *transaction)
        .await?;
    let member = member_row(&mut transaction, campus_id, account_id)
        .await?
        .ok_or_else(not_found)?;
    transaction.commit().await?;
    Ok(Json(member))
}

pub async fn remove_member(
    State(state): State<AppState>,
    authenticated: Authenticated,
    Path((raw_campus_id, raw_account_id)): Path<(String, String)>,
) -> Result<StatusCode, ApiFailure> {
    let campus_id = parse_campus_id(&raw_campus_id)?;
    let account_id = parse_account_id(&raw_account_id)?;
    let mut transaction = state.db.begin().await?;
    let actor = actor_role(&mut transaction, campus_id, authenticated.account_id).await?;
    manageable_target(&mut transaction, campus_id, actor, account_id).await?;
    if account_id == authenticated.account_id {
        return Err(forbidden());
    }
    sqlx::query("DELETE FROM campus_members WHERE campus_id = $1 AND account_id = $2")
        .bind(campus_id.as_uuid())
        .bind(account_id.as_uuid())
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    let frame = ServerFrame::MembershipRevoked(MembershipRevoked {
        campus_id,
        account_id,
    });
    publish_frame(&state, campus_id, Some(account_id), frame).await;
    Ok(StatusCode::NO_CONTENT)
}
