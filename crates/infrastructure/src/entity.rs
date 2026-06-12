use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc}; 
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub role: Option<String>,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
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
