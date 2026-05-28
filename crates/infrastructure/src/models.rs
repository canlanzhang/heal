
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc}; 


#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
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