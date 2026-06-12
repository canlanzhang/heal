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

    }
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

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>, // 👈 自动反序列化请求体的 JSON
) -> Result<(StatusCode, Json<User>), impl IntoResponse> {
    
    // 调用我们刚才写的 db 函数
    match db::create_user(&state.db_pool, payload).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))), // 返回 201 Created
        Err(db::DbError::NotFound) => {
            // 理论上插入不会 NotFound，但为了健壮性可以保留匹配
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error".to_string()).into_response())
        },
        Err(e) => {
            // 捕获其他数据库错误（如唯一索引冲突等），返回 500 或 409
            eprintln!("Database error: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user".to_string()).into_response())
        }
    }
}

pub async fn delete_user_handler(
    Path(user_id): Path<i32>, // 👈 从 URL 路径中自动提取 user_id (例如 /users/5)
    State(state): State<AppState>,
) -> Result<StatusCode, impl IntoResponse> {
    
    // 调用我们刚才写的 db 函数
    match db::delete_user(&state.db_pool, user_id).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT), // 返回 204 No Content，表示删除成功且无需返回数据
        
        Err(db::DbError::NotFound) => {
            // 如果数据库没找到这个 ID，说明资源不存在
            Err((StatusCode::NOT_FOUND, "User not found".to_string()).into_response())
        },
        
        Err(e) => {
            // 捕获其他数据库错误（如连接失败等）
            eprintln!("Database error during deletion: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user".to_string()).into_response())
        }
    }
}

pub async fn update_user_handler(
    Path(user_id): Path<i32>, // 👈 从 URL 路径中提取 user_id (例如 /users/5)
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserPayload>, // 👈 自动反序列化请求体的 JSON (如 {"username": "new_name"})
) -> Result<(StatusCode, Json<User>), impl IntoResponse> {
    
    // 调用我们之前写的 db 函数
    match db::update_user(&state.db_pool, user_id, payload).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))), // 返回 200 OK 及更新后的完整用户信息
        
        Err(db::DbError::NotFound) => {
            // 如果数据库没找到这个 ID，说明资源不存在
            Err((StatusCode::NOT_FOUND, "User not found".to_string()).into_response())
        },
        
        Err(e) => {
            // 捕获其他数据库错误（如连接失败等）
            eprintln!("Database error during update: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user".to_string()).into_response())
        }
    }
}

pub async fn handle_get_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
) ->  (StatusCode, Json<ApiResponse<User>>) {
    match db::get_user_by_id(&state.db_pool, user_id).await {
        Ok(user) => {
            // 查询成功，返回 JSON 数据
            tracing::debug!("找到用户: {:?}", user);
            (StatusCode::OK, Json(ApiResponse::success(user)))

        }
        Err(db::DbError::NotFound) => {
            (StatusCode::NOT_FOUND,Json(ApiResponse::error(404, "用户不存在".to_string())))

        }
        
        Err(e) => {
            // 查询失败，处理错误            
            tracing::debug!("数据库查询失败: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR,Json(ApiResponse::error(500, format!("系统错误: {}", e))))

        }
    }
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
        Err(db::DbError::NotFound) => {
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