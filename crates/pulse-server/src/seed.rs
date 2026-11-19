use pulse_protocol::{
    AccountId, CampusId, DEV_SEED_CAMPUS_NAME, DEV_SEED_CAMPUS_SLUG, DEV_SEED_PASSWORD,
    DEV_SEED_PEOPLE, Role,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::hash_password;
use crate::campus::furnish_campus;
use crate::error::ApiFailure;

async fn seeded_account(
    db: &PgPool,
    email: &str,
    display_name: &str,
    password_hash: &str,
) -> Result<AccountId, sqlx::Error> {
    sqlx::query(
        "INSERT INTO accounts (account_id, email, display_name, password_hash) \
         VALUES ($1, $2, $3, $4) ON CONFLICT (email) DO NOTHING",
    )
    .bind(AccountId::generate().as_uuid())
    .bind(email)
    .bind(display_name)
    .bind(password_hash)
    .execute(db)
    .await?;
    let account_id =
        sqlx::query_scalar::<_, Uuid>("SELECT account_id FROM accounts WHERE email = $1")
            .bind(email)
            .fetch_one(db)
            .await?;
    Ok(AccountId(account_id))
}

async fn seeded_campus(db: &PgPool, owner: AccountId) -> Result<CampusId, sqlx::Error> {
    let mut transaction = db.begin().await?;
    let campus_id = CampusId::generate();
    let inserted = sqlx::query(
        "INSERT INTO campuses (campus_id, slug, display_name) VALUES ($1, $2, $3) \
         ON CONFLICT (slug) DO NOTHING",
    )
    .bind(campus_id.as_uuid())
    .bind(DEV_SEED_CAMPUS_SLUG)
    .bind(DEV_SEED_CAMPUS_NAME)
    .execute(&mut *transaction)
    .await?
    .rows_affected()
        == 1;
    if inserted {
        furnish_campus(&mut transaction, campus_id, owner).await?;
        transaction.commit().await?;
        return Ok(campus_id);
    }
    transaction.rollback().await?;
    let existing = sqlx::query_scalar::<_, Uuid>("SELECT campus_id FROM campuses WHERE slug = $1")
        .bind(DEV_SEED_CAMPUS_SLUG)
        .fetch_one(db)
        .await?;
    Ok(CampusId(existing))
}

async fn seeded_membership(
    db: &PgPool,
    campus_id: CampusId,
    account_id: AccountId,
    role: Role,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO campus_members (campus_id, account_id, role) VALUES ($1, $2, $3) \
         ON CONFLICT (campus_id, account_id) DO UPDATE SET role = EXCLUDED.role",
    )
    .bind(campus_id.as_uuid())
    .bind(account_id.as_uuid())
    .bind(role.as_str())
    .execute(db)
    .await?;
    Ok(())
}

pub async fn seed_dev_campus(db: &PgPool) -> Result<CampusId, ApiFailure> {
    let password_hash = hash_password(DEV_SEED_PASSWORD.to_owned()).await?;
    let mut people = Vec::with_capacity(DEV_SEED_PEOPLE.len());
    for (email, display_name, role) in DEV_SEED_PEOPLE {
        let account_id = seeded_account(db, email, display_name, &password_hash).await?;
        people.push((account_id, role));
    }
    let owner = people
        .iter()
        .find(|(_, role)| *role == Role::Owner)
        .map_or_else(|| people[0].0, |(account_id, _)| *account_id);
    let campus_id = seeded_campus(db, owner).await?;
    for (account_id, role) in people {
        seeded_membership(db, campus_id, account_id, role).await?;
    }
    Ok(campus_id)
}
