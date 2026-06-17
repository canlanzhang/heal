
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct ArticleListItem {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub author_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateArticlePayload {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateArticlePayload {
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct ArticleProfileResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub status: String,
    pub author_id: Option<i32>,
}