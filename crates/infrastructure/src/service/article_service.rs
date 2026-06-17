use sqlx::PgPool;

use crate::db;

use crate::dto::common::MenuItem;
use crate::errors::DbError;


use crate::{
    dto::article::{
        ArticleListItem,
        CreateArticlePayload,
        UpdateArticlePayload,
    },
    entity::Article,
};

pub async fn list_articles(pool: &PgPool) -> Result<Vec<Article>, DbError> {
    let res = sqlx::query_as!(
        Article,
        r#"
        SELECT id, title, content, status, author_id
        FROM heal_article
        ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(res)
}


pub async fn create_article(
    pool: &PgPool,
    payload: CreateArticlePayload,
    author_id: i32,
) -> Result<Article, DbError> {
    db::article::create_article(pool, &payload, author_id).await
}

pub async fn update_article(
    pool: &PgPool,
    id: i32,
    payload: UpdateArticlePayload,
) -> Result<Article, DbError> {
    db::article::update_article(pool, id, &payload).await
}

pub async fn delete_article(pool: &PgPool, id: i32) -> Result<(), DbError> {
    db::article::delete_article(pool, id).await
}

pub async fn get_article_by_id(pool: &PgPool, id: i32) -> Result<Article, DbError> {
    db::article::get_article_by_id(pool, id).await
}

