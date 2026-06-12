use axum::{
    Json, 
    http::StatusCode,
    extract::{Path, State}, 
    response::IntoResponse
};

use crate::state::AppState; 
use infrastructure::{
    db,
    entity::{
        Admin,User,

    },
    dto::{
        CreateUserPayload,UpdateUserPayload,AdminPayload,Claims

    },
    errors::*,
}; // 引入底层的基础设施和连接池

use serde::{Serialize, Deserialize};
use bcrypt::{hash,DEFAULT_COST,verify, BcryptError};


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

// GET /users/:id
pub async fn handle_get_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<User>>, DbError> {
    let user = db::get_user_by_id(&state.db_pool, user_id).await?;
    Ok(Json(ApiResponse::success(user)))
}

// POST /users
pub async fn handler_create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>, // 👈 自动反序列化请求体的 JSON
) -> Result<Json<ApiResponse<User>>, DbError> {
    
    let user = db::create_user(&state.db_pool, payload).await?;
    Ok(Json(ApiResponse::success(user)))
}

// PUT /users/:id
pub async fn handler_update_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<ApiResponse<User>>, DbError> {
    let user = db::update_user(&state.db_pool, user_id, payload).await?;
    Ok(Json(ApiResponse::success(user)))
}

// DELETE /users/:id
pub async fn handler_delete_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, DbError> {
    db::delete_user(&state.db_pool, user_id).await?;
    Ok(Json(ApiResponse::success(())))
}



// 定义登录成功的响应结构
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<AdminPayload>,
) -> Result<(StatusCode, Json<LoginResponse>), impl IntoResponse> {

    //let hashed = hash("123456", DEFAULT_COST).unwrap();
    //println!("新哈希: {}", hashed);

    let admin_result = db::find_user_for_login(&state.db_pool, &payload.username).await;
    
    match admin_result {
        Ok(admin) => {
            let is_valid = match bcrypt::verify(&payload.password, &admin.password_hash) {
                Ok(valid) => valid,
                Err(e) => {
                    tracing::debug!("Bcrypt error: {:?}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    ).into_response());
                }
            };

            if !is_valid {
                tracing::debug!("密码验证失败");
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid username or password".to_string(),
                ).into_response());
            }

            let token = match Claims::generate_token(&admin.id.to_string()){
                Ok(token) => token,
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR, 
                        "Failed to generate token".to_string(),
                    ).into_response());
                }
            };
            
            let response = LoginResponse {
                token,
                username: admin.username.clone(),
                // id: admin.id,
            };

            // ✅ 登录成功返回 200 OK
            Ok((StatusCode::OK, Json(response)))
        }
        Err(DbError::NotFound) => {
            // ✅ 用户不存在也返回 401，和密码错误保持一致
            Err((
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            ).into_response())
        },
        Err(e) => {
            eprintln!("Database error during login: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to process login".to_string(),
            ).into_response())
        }
    }
}