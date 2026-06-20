use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc}; 
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub role: String,
    //pub created_at: DateTime<chrono::Utc>,
    //pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Menu {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub title: String,
    pub icon: Option<String>,
    pub role: String,
    pub parent_id: Option<i32>,
    pub sort: Option<i32>,
}


#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub status: String,
    pub author_id: Option<i32>,
    //pub created_at: chrono::NaiveDateTime,
    //pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>, 
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub is_done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>, 
}
