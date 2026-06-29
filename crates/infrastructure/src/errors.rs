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
    //API错误
    #[error("epi error: {0} ")]
    Api(#[from] ApiError),

    // 启动时的致命错误
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
        match self {
            // 如果是 API 错误，直接委托给 ApiError 的 IntoResponse 去处理
            AppError::Api(api_err) => api_err.into_response(),

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
        let (status,message) = match self {

            ApiError::Db(e) => match e {
                DbError::NotFound => (StatusCode::NOT_FOUND, e.to_string()),

                DbError::Sql(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                ),

                DbError::Unauthorized => (StatusCode::UNAUTHORIZED, e.to_string()),

                DbError::TokenError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Token error".to_string(),
                ),

                DbError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),

                DbError::DuplicateEntry => (StatusCode::CONFLICT, e.to_string()),

                DbError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),

                DbError::Internal(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    msg,
                ),

            },
            
            ApiError::Auth(e) => match e {
                AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, e.to_string()),
                AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, e.to_string()),
                AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
                AuthError::InvalidToken => (StatusCode:: UNAUTHORIZED, e.to_string()),
                AuthError::VerifyInternalError(msg) => {
                    // 1. 把真实的底层报错打印到【后端日志】中，方便程序员排查
                    tracing::error!("密码校验底层异常: {}", msg); 
                    // 2. 返回给【前端】的依然是安全的模糊提示
                   (StatusCode::INTERNAL_SERVER_ERROR,"服务器内部异常".to_string())
                }
            },
            
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR,msg),
        };

        (status,Json(ErrorResponse::new(message, status))).into_response()
        
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

    // 🛠️ 新增：专门处理数据校验失败的错误类型
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("用户名或邮箱已存在")]
    DuplicateEntry,

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
            
            // 🛠️ 新增映射：参数校验失败返回 400 状态码
            DbError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            DbError::DuplicateEntry => (StatusCode::CONFLICT, self.to_string()),
            
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

    #[error("Password verification internal error: {0}")]
    VerifyInternalError(String), 
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, self.to_string()),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::VerifyInternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR,"服务器内部异常".to_string()),
        };

        (status, Json(ErrorResponse::new(message, status))).into_response()
    }
}


