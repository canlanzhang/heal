use sqlx::PgPool;

use crate::db;

use crate::dto::ArticleListItem;
use crate::errors::DbError;


use crate::{
    dto::article::{
        CreateArticlePayload,
        UpdateArticlePayload,
    },
    entity::Article,
};

pub async fn list_articles(
    pool: &PgPool
) -> Result<Vec<ArticleListItem>, DbError> {
    let articles = db::article::list_articles(pool).await?;

    let res = articles.into_iter().map(|a|ArticleListItem{
        id: a.id,
        title: a.title,
        status: a.status,
        author_id: a.author_id,
    }).collect();

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

