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

// ==================== 🛠️ 新增：AppError (系统启动、证书、环境错误) ====================

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("❌ TLS证书加载失败，请检查物理路径。错误原因: {0}")]
    TlsConfig(String),

    #[error("❌ 环境变量端口解析失败: {0}")]
    PortParse(String),

    #[error("❌ 数据库初始化连接失败: {0}")]
    DbInit(String),
}
// 系统启动层面的错误通常会让程序退出(panic/return), 
// 但为了架构统一，我们也为其实现 IntoResponse (以备后续在路由中使用)
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        (status, Json(ErrorResponse::new(self.to_string(), status))).into_response()
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
    // 新增：处理业务逻辑错误
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            DbError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            DbError::Sql(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            DbError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            DbError::TokenError => (StatusCode::INTERNAL_SERVER_ERROR, "Token generation failed".to_string()),
            DbError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            DbError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
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


