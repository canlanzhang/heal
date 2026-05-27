
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
}


#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
}
