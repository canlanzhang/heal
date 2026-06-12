
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc}; 
use jsonwebtoken::{encode, EncodingKey, Header};
use validator::Validate;
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

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
