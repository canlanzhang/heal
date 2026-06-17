
use crate::entity::{Admin};
use crate::dto::admin::{
    CreateAdminPayload,
    UpdateAdminPayload,
};

use crate::errors::DbError;
use sqlx::postgres::{PgPool};

pub async fn list_admins(pool: &PgPool) -> Result<Vec<Admin>, DbError> {
    let admins = sqlx::query_as!(
        Admin,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role
        FROM heal_admin
        ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(admins)
}

pub async fn create_admin(
    pool: &PgPool,
    payload: &CreateAdminPayload,
) -> Result<Admin, DbError> {
    let admin = sqlx::query_as!(
        Admin,
        r#"
        INSERT INTO heal_admin (
            username,
            email,
            password_hash,
            role
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            username,
            email,
            password_hash,
            role
        "#,
        payload.username,
        payload.email,
        payload.password,
        payload.role
    )
    .fetch_one(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(admin)
}



pub async fn update_admin(
    pool: &PgPool,
    admin_id: i32,
    payload: &UpdateAdminPayload,
) -> Result<Admin, DbError> {
    let admin = sqlx::query_as!(
        Admin,
        r#"
        UPDATE heal_admin
        SET
            username = COALESCE($1, username),
            email = COALESCE($2, email),
            password_hash = COALESCE($3, password_hash),
            role = COALESCE($4, role)
        WHERE id = $5
        RETURNING
            id,
            username,
            email,
            password_hash,
            role
        "#,
        payload.username,
        payload.email,
        payload.password,
        payload.role,
        admin_id
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

    admin.ok_or(DbError::NotFound)
}



pub async fn delete_admin(pool: &PgPool, admin_id: i32) -> Result<(), DbError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM heal_admin
        WHERE id = $1
        "#,
        admin_id
    )
    .execute(pool)
    .await
    .map_err(DbError::Sql)?;

    if result.rows_affected() == 0 {
        return Err(DbError::NotFound);
    }

    Ok(())
}




pub async fn get_admin_by_id(pool: &PgPool, admin_id: i32) -> Result<Admin, DbError> {
    let admin = sqlx::query_as!(
        Admin,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role
        FROM heal_admin
        WHERE id = $1
        "#,
        admin_id
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

    admin.ok_or(DbError::NotFound)
}



pub async fn get_admin_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Admin, DbError> {
    let admin = sqlx::query_as!(
        Admin,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role
        FROM heal_admin
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

    admin.ok_or(DbError::NotFound)
}


pub async fn query_admin_for_login(
    pool: &PgPool,
    username: &str,
) -> Result<Admin, DbError> {
    let admin = sqlx::query_as!(
        Admin,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role
        FROM heal_admin
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

    admin.ok_or(DbError::NotFound)
}