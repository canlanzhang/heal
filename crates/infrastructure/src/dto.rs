
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc}; 
use jsonwebtoken::{encode, EncodingKey, Header};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    //company: String,
    pub exp: usize,
}

impl Claims {
    pub fn generate_token(user_id: &str) -> Result<String,jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration as usize,
        };
        encode(&Header::default(), &claims, &EncodingKey::from_secret("my_super_secret_key".as_ref()))
    }
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

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
