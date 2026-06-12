use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;

// 统一错误响应格式
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        Self {
            error: message.into(),
            code: status.as_u16(),
        }
    }
}

// ==================== DbError ====================

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("User not found")]
    NotFound,
    #[error("Database error: {0}")]
    Sql(#[from] sqlx::Error),
    #[error("Invalid credentials")]
    Unauthorized,
    #[error("Token generation failed")]
    TokenError,
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            DbError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            DbError::Sql(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            DbError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            DbError::TokenError => (StatusCode::INTERNAL_SERVER_ERROR, "Token generation failed".to_string()),
        };

        (status, Json(ErrorResponse::new(message, status))).into_response()
    }
}

// ==================== AuthError ====================

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Token creation failed")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, self.to_string()),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
        };

        (status, Json(ErrorResponse::new(message, status))).into_response()
    }
}