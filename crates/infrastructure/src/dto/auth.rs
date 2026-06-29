use axum::{
    extract::{FromRequestParts},
    http::request::Parts,
};
use serde::{Serialize, Deserialize};
use chrono::{ Duration, Utc}; 
use jsonwebtoken::{encode, decode, 
    EncodingKey, DecodingKey, Header, Validation, Algorithm};
use crate::errors::{ApiError, AuthError}; 

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
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
    pub fn generate_token(user_id:i32) -> Result<String,jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id,
            exp: expiration as usize,
        };

        // 🛠️ 2. 动态读取密钥
        let secret = Self::get_secret();
        // println!("JWT_SECRET: {}",secret);
        encode(
            &Header::default(), 
            &claims, 
            &EncodingKey::from_secret(secret.as_ref())
        )
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
    type Rejection = ApiError; // 统一抛出你的 AuthError

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从请求头提取 Authorization
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(crate::errors::AuthError::MissingCredentials)?; // 找不到 Header 返回 400 错误
        
        // 验证 Bearer 前缀
        if !auth_header.starts_with("Bearer ") {
            return Err(ApiError::from(AuthError::InvalidToken));
        }
        let token = &auth_header[7..];

        // 验证 Token 合法性与过期时间
        let claims = Claims::decode_token(token).map_err(|_| crate::errors::AuthError::InvalidToken)?;

        Ok(claims)
    }
}




