
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc}; 


#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    // 使用 Option<String> 来实现部分更新：传了就用新值，没传就是 None
    pub username: Option<String>, 
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



#[derive(Debug, Deserialize)]
pub struct AdminPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Admin {
    pub id: i32,
    pub username: String,
    pub email:  Option<String>,
    pub password_hash: String,
    pub role:  Option<String>,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}