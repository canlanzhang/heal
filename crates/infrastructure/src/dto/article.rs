
use serde::{Serialize, Deserialize};

#[derive(serde::Deserialize)]
pub struct UpdateArticlePayload {
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
}