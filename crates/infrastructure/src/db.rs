
use crate::entity::{Admin,User};
use crate::dto::{CreateUserPayload,UpdateUserPayload};
use crate::errors::DbError;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn create_pool() ->  Result<PgPool, DbError> {
    let database_url  = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://dbuser:dbpass@localhost:5432/dioxusdb".to_string());
    // ⚡ 使用 PgPoolOptions 进行精细化配置
    let pool = PgPoolOptions::new()
        // 1. 设置最大连接数（根据业务并发量调整，默认是 10）
        .max_connections(20)
        // 2. 设置最小保持连接数（避免冷启动延迟）
        .min_connections(5) 
        // 3. 建立连接的超时时间（防止数据库挂掉时后端无限卡死）
        .acquire_timeout(Duration::from_secs(10))
        // 4. 空闲连接回收时间（释放长期不用的连接，节约省数据库资源）
        .idle_timeout(Duration::from_secs(600)) 
        .connect(&database_url )
        .await
        .map_err(DbError::Sql)?;

    Ok(pool)
}


pub async fn create_user(pool: &PgPool, payload: CreateUserPayload) -> Result<User, DbError> {
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
    .await?; // 如果发生冲突或错误，会自动通过 #[from] 转为 DbError::Sql

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: i32) -> Result<(), DbError> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await?;

    // 检查受影响的行数，如果为 0，说明该 ID 的用户根本不存在
    if result.rows_affected() == 0 {
        return Err(DbError::NotFound);
    }

    Ok(())
}

pub async fn update_user(
    pool: &PgPool,
    user_id: i32,
    payload: UpdateUserPayload,
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
            created_at AS "created_at!",
            updated_at AS "updated_at!"
        "#,
        payload.username,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    user.ok_or(DbError::NotFound) // ✅ 更简洁的写法
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
    .await?;

    match user {
        Some(u) => Ok(u),
        None => Err(DbError::NotFound),
    }
}



pub async fn find_user_for_login(pool: &PgPool, username: &str) -> Result<Admin, DbError> {
    let admin = sqlx::query_as!(
        Admin,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role,
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM heal_admin
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    match admin {
        Some(u) => Ok(u),
        None => Err(DbError::NotFound),
    }
}