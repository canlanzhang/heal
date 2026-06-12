
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc}; 
use jsonwebtoken::{encode, decode, 
    EncodingKey, DecodingKey, Header, Validation, Algorithm};
use validator::Validate;
use crate::errors::AuthError;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    //company: String,
    pub exp: usize,
}

impl Claims {
    // 内部辅助函数：安全动态获取密钥
    fn get_secret() -> String {
        
        std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| {
                if std::env::var("APP_ENV").unwrap_or_default() == "production" {
                    panic!("FATAL: JWT_SECRET environment variable is missing in production!");
                }
                "fallback_development_secret_key_only_for_dev".to_string()
            })
    }
    pub fn generate_token(user_id: &str) -> Result<String,jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };

        // 🛠️ 2. 动态读取密钥
        let secret = Self::get_secret();
        // println!("JWT_SECRET: {}",secret);
        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
    }

    pub fn decode_token(token: &str) -> Result<Self,jsonwebtoken::errors::Error> {
        let validation = Validation::new(Algorithm::HS256);

        // 🛠️ 3. 动态读取密钥
        let secret = Self::get_secret();
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?;
        Ok(token_data.claims)

    }
}



// 3. 🛠️ 新增：为 Claims 实现 Axum 提取器守护路由

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError; // 统一抛出你的 AuthError

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从请求头提取 Authorization
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::MissingCredentials)?; // 找不到 Header 返回 400 错误

        // 验证 Bearer 前缀
        if !auth_header.starts_with("Bearer ") {
            return Err(AuthError::InvalidToken);
        }
        let token = &auth_header[7..];

        // 验证 Token 合法性与过期时间
        let claims = Claims::decode_token(token).map_err(|_| AuthError::InvalidToken)?;

        Ok(claims)
    }
}








#[derive(Serialize)]
pub struct ApiResponse<T> {
    code: u16,        // 状态码，如 200, 404, 500
    message: String,  // 提示信息
    data: Option<T>,          // 泛型字段，存放真正的数据（比如 User, Vec<Post> 等）
}

impl<T> ApiResponse<T> 
where
    T: Serialize, // 约束：T 必须能被序列化
{
    // 快捷构造函数：成功
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "Success".to_string(),
            data:Some(data),
        }
    }

    // 快捷构造函数：失败
    pub fn error(code: u16, message: String) -> Self {
        // 注意：这里 data 可能需要是 Option<T> 或者默认值，视具体设计而定
        // 简化版示例假设 data 依然需要传，或者结构体改为 data: Option<T>
        Self {
            code,
            message,
            data: None, // 失败时没有数据，传 None 即可
        }
    }
}


#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
}


#[derive(Deserialize, Validate)]
pub struct CreateAdminPayload {
    #[validate(length(min = 3, max= 16))]
    pub username: String,
    #[validate(email)]
    pub email: Option<String>,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminPayload {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    // 使用 Option<String> 来实现部分更新：传了就用新值，没传就是 None
    pub username: Option<String>, 
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

