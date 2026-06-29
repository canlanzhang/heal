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

// ==================== 🛠️ AppError (API错误、系统启动、证书、环境错误) ====================
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("api error: {0} ")]
    Api(#[from] ApiError),

    #[error("❌ TLS证书加载失败，请检查物理路径。错误原因: {0}")]
    TlsConfig(String),

    #[error("❌ 环境变量端口解析失败: {0}")]
    PortParse(String),

    #[error("❌ 数据库初始化连接失败: {0}")]
    DbInit(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            // 如果是 API 错误，直接委托给 ApiError 的 IntoResponse 去处理
            AppError::Api(api_err) => api_err.into_response(),
            // 系统启动层面的致命错误，统一返回 500
            _ => {
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                (status, Json(ErrorResponse::new(self.to_string(), status))).into_response()
            }            
        }
        
    }
}

// ==================== 🛠️ ApiError (所有错误) ====================
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("database error: {0}")]
    Db(#[from] DbError),

    #[error("auth error: {0}")]
    Auth(#[from] AuthError), 

    #[error("internal error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            // 💡 1. DbError
            ApiError::Db(DbError::NotFound) => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            ApiError::Db(DbError::Sql(_)) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::Db(DbError::Unauthorized) => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ApiError::Db(DbError::TokenError) => (StatusCode::INTERNAL_SERVER_ERROR, "Token error".to_string()),
            ApiError::Db(DbError::ValidationError(msg)) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Db(DbError::DuplicateEntry) => (StatusCode::CONFLICT, "Resource already exists".to_string()),
            ApiError::Db(DbError::BadRequest(msg)) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Db(DbError::Internal(msg)) => (StatusCode::INTERNAL_SERVER_ERROR, msg),

            // 💡 2. AuthError
            ApiError::Auth(AuthError::WrongCredentials) => (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string()),
            ApiError::Auth(AuthError::MissingCredentials) => (StatusCode::BAD_REQUEST, "Missing credentials".to_string()),
            ApiError::Auth(AuthError::TokenCreation) => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed".to_string()),
            ApiError::Auth(AuthError::InvalidToken) => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            
            ApiError::Auth(AuthError::VerifyInternalError(msg)) => {
                tracing::error!("密码校验底层异常: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部异常".to_string())
            }
            
            // 💡 4. Auth 中包含的 DbError 统一返回 500
            ApiError::Auth(AuthError::InternalDbError(db_err)) => {
                tracing::error!("认证模块数据库异常: {:?}", db_err); // 💡 顺便也记一下日志
                (StatusCode::INTERNAL_SERVER_ERROR, "认证模块数据库异常".to_string())
            }

            // 💡 5. 顶层内部错误
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(ErrorResponse::new(message, status))).into_response()
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
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("用户名或邮箱已存在")]
    DuplicateEntry,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    Internal(String),
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
    #[error("Password verification internal error: {0}")]
    VerifyInternalError(String), 
    #[error("数据库内部错误: {0}")]
    InternalDbError(#[from] DbError), 
}
