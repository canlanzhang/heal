

use crate::entity::{Article};
use crate::dto::article::{
    CreateArticlePayload,
    UpdateArticlePayload,
};

use crate::errors::DbError;
use sqlx::postgres::{PgPool};

pub async fn list_articles(pool: &PgPool) -> Result<Vec<Article>, DbError> {
    let articles = sqlx::query_as!(
        Article,
        r#"
        SELECT id, title, content, status, author_id
        FROM heal_articles
        ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(articles)
}

pub async fn create_article(
    pool: &PgPool,
    payload: &CreateArticlePayload,
    author_id: i32,
) -> Result<Article, DbError> {

    let res = sqlx::query_as!(
        Article,
        r#"
        INSERT INTO heal_articles (title, content, status, author_id)
        VALUES ($1, $2, 'draft', $3)
        RETURNING id, title, content, status, author_id
        "#,
        payload.title,
        payload.content,
        author_id
    )
    .fetch_one(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(res)
}

pub async fn update_article(
    pool: &PgPool,
    article_id: i32,
    payload: &UpdateArticlePayload,
) -> Result<Article, DbError> {

    let res = sqlx::query_as!(
        Article,
        r#"
        UPDATE heal_articles
        SET
            title = COALESCE($1, title),
            content = COALESCE($2, content),
            status = COALESCE($3, status),
            updated_at = NOW()
        WHERE id = $4
        RETURNING id, title, content, status, author_id
        "#,
        payload.title,
        payload.content,
        payload.status,
        article_id
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

    res.ok_or(DbError::NotFound)
}


pub async fn delete_article(pool: &PgPool, id: i32) -> Result<(), DbError> {
    let result = sqlx::query!(
        r#"DELETE FROM heal_articles WHERE id = $1"#,
        id
    )
    .execute(pool)
    .await
    .map_err(DbError::Sql)?;

    if result.rows_affected() == 0 {
        return Err(DbError::NotFound);
    }

    Ok(())
}

pub async fn get_article_by_id(pool: &PgPool, id: i32) -> Result<Article, DbError> {
    let res = sqlx::query_as!(
        Article,
        r#"
        SELECT id, title, content, status, author_id
        FROM heal_articles
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(DbError::Sql)?;

    res.ok_or(DbError::NotFound)
}