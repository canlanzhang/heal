
use crate::entity::{User};
use crate::dto::user::{
    CreateUserPayload,
    UpdateUserPayload,
};
use crate::errors::DbError;
use sqlx::postgres::{PgPool};



pub async fn create_user(
    pool: &PgPool,
    payload: &CreateUserPayload,
) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username)
        VALUES ($1)
        RETURNING
            id,
            username,
            created_at as "created_at!",
            updated_at as "updated_at!"
        "#,
        payload.username
    )
    .fetch_one(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(user)
}



pub async fn update_user(
    pool: &PgPool,
    user_id: i32,
    payload: &UpdateUserPayload,
) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET
            username = COALESCE($1, username),
            updated_at = NOW()
        WHERE id = $2
        RETURNING
            id,
            username,
            created_at as "created_at!",
            updated_at as "updated_at!"
        "#,
        payload.username,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

    user.ok_or(DbError::NotFound)
}



pub async fn delete_user(pool: &PgPool, user_id: i32) -> Result<(), DbError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await
    .map_err(DbError::Sql)?;

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
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

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
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

    user.ok_or(DbError::NotFound)
}