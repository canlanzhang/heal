use axum::{
    Json, 
    http::StatusCode,
    extract::{Path, State}, 
    response::IntoResponse
};

use crate::state::AppState; 
use infrastructure::{
    db, 
    dto::{
        Claims,
        ApiResponse,
        LoginResponse,AdminPayload,
        CreateAdminPayload, CreateUserPayload, UpdateUserPayload

    }, 
    entity::{
        Admin,User,

    }, 
    errors::*,

}; // 引入底层的基础设施和连接池

use serde::{Serialize, Deserialize};
use bcrypt::{hash,DEFAULT_COST,verify, BcryptError};



// POST /admins
pub async fn handler_create_admin(
    _claims: Claims,
    State(state): State<AppState>,
    Json(mut payload): Json<CreateAdminPayload>,
) -> Result<Json<ApiResponse<Admin>>, DbError> {
     // 1. 密码加密 (关键步骤)
    payload.password  = hash(&payload.password, DEFAULT_COST)
        .map_err(|_|DbError::Internal("Password hashing failed".to_string()))?;
    let admin = db::create_admin(&state.db_pool, &payload).await?;

    //let admin = db::create_admin(&state.db_pool, new_admin).await?;
    Ok(Json(ApiResponse::success(admin)))
}




pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<AdminPayload>,
) -> Result<Json<ApiResponse<LoginResponse>>,DbError> {

    // 1. 查用户
    let admin = db::find_user_for_login(&state.db_pool, &payload.username).await?;
    // 2. 验证密码
    let is_valid = bcrypt::verify(&payload.password, &admin.password_hash)
        .map_err(|_| DbError::Internal("Password verification failed".to_string()))?;
    

    if !is_valid {
        //tracing::debug!("login_handler: {}",is_valid);
        return Err(DbError::Unauthorized);
    }

    // 3. 生成 token
    let token = Claims::generate_token(&admin.id.to_string())
        .map_err(|_| DbError::TokenError)?;

    // 4. 返回成功响应
    let response = LoginResponse {
        token,
        username: admin.username.clone(),
    };

     Ok(Json(ApiResponse::success(response)))


    
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
    
    let user = db::create_user(&state.db_pool, &payload).await?;
    Ok(Json(ApiResponse::success(user)))
}

// PATCH  /users/:id
pub async fn handler_patch_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<ApiResponse<User>>, DbError> {
    let user = db::update_user(&state.db_pool, user_id, &payload).await?;
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


