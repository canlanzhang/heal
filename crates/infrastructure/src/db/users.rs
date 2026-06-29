use sqlx::PgPool;
use crate::entity::{User};
use crate::dto::users::{
    CreateUserPayload,
    UpdateUserPayload,
};
use crate::errors::DbError;

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, DbError> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role
        FROM heal_users
        ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn create_user(
    pool: &PgPool,
    payload: &CreateUserPayload,
    password_hash: &str,
) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO heal_users (
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
        password_hash,
        payload.role
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) =&e {
            if db_err.code().map(|c| c == "23505").unwrap_or(false) {
                return DbError::DuplicateEntry;
            }
        }
        DbError::Sql(e)
    })?;

    Ok(user)
}



pub async fn update_user(
    pool: &PgPool,
    user_id: i32,
    payload: &UpdateUserPayload,
    password_hash: Option<&str>,
) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE heal_users
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
        password_hash,
        payload.role,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    user.ok_or(DbError::NotFound)
}



pub async fn delete_user(pool: &PgPool, user_id: i32) -> Result<(), DbError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM heal_users
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(DbError::NotFound);
    }

    Ok(())
}




pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role
        FROM heal_users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    user.ok_or(DbError::NotFound)
}



pub async fn get_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role
        FROM heal_users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    user.ok_or(DbError::NotFound)
}


pub async fn query_user_for_login(
    pool: &PgPool,
    username: &str,
) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role
        FROM heal_users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    user.ok_or(DbError::NotFound)
}