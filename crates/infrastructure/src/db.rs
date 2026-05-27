
use crate::models::{CreateUserPayload,User};

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("User not found")]
    NotFound,
    #[error("Database error: {0}")]
    Sql(#[from] sqlx::Error),
}


pub async fn create_pool() ->  Result<PgPool, sqlx::Error> {
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
        .expect("Failed to create configured PostgreSQL pool");

    Ok(pool)
}


pub async fn create_user(pool: &PgPool, payload: CreateUserPayload) -> Result<User, DbError> {
    let user = sqlx::query_as!(
        User, 
        "INSERT INTO users (username) VALUES ($1) RETURNING id, username",
        payload.username
    )
    .fetch_one(pool)
    .await?; // 如果发生冲突或错误，会自动通过 #[from] 转为 DbError::Sql

    Ok(user)
}

// 编写查询接口
pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, DbError> {
    let user = sqlx::query_as::<_, User>("SELECT id, username FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    match user {
        Some(u) => Ok(u),
        None => Err(DbError::NotFound),
    }
}
