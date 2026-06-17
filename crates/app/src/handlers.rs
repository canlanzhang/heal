use axum::{
    Json, 
    http::StatusCode,
    extract::{Path, State}, 
    response::IntoResponse
};



use crate::state::AppState; 
use infrastructure::dto::admin::{
    AdminProfileResponse,
    AdminInfo,
};
use infrastructure::dto::common::{
    MenuItem,
};
use infrastructure::dto::auth::{
    LoginRequest,
    LoginResponse,
    Claims,
};
use infrastructure::dto::user::{
    CreateUserPayload,
    UpdateUserPayload,
};
use infrastructure::{
    db, 
    entity::{
        Admin,User,

    }, 
    errors::*,

}; // 引入底层的基础设施和连接池
use infrastructure::dto::*;
use serde::{Serialize, Deserialize};
use bcrypt::{hash,DEFAULT_COST,verify, BcryptError};
//use infrastructure::service::login;
use infrastructure::service;
use infrastructure::service::admin_service;
pub async fn handler_admin_info(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AdminProfileResponse>>, DbError> {

    let data = admin_service::get_admin_profile(
        &state.db_pool,
        claims.sub,
    ).await?;

    Ok(Json(ApiResponse::success(data)))
}

fn empty_to_none(v: Option<String>) -> Option<String> {
    match v {
        Some(s) if s.trim().is_empty() => None,
        other => other,
    }
}

// GET /admins
pub async fn handler_list_admins(
    _claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AdminListItem>>>, DbError> {

    let list = admin_service::list_admins(&state.db_pool).await?;

    Ok(Json(ApiResponse::success(list)))
}


// POST /admins
pub async fn handler_create_admin(
    _claims: Claims,
    State(state): State<AppState>,
    ValidatedJson(mut payload): ValidatedJson<CreateAdminPayload>,
) -> Result<Json<ApiResponse<Admin>>, DbError> {
     // 1. 密码加密 (关键步骤)
    payload.password  = hash(&payload.password, DEFAULT_COST)
        .map_err(|_|DbError::Internal("Password hashing failed".to_string()))?;
    let admin = db::create_admin(&state.db_pool, &payload).await?;

    //let admin = db::create_admin(&state.db_pool, new_admin).await?;
    Ok(Json(ApiResponse::success(admin)))
}

// PATCH /api/admins/:id
pub async fn handler_patch_admin(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能修改
    Path(admin_id): Path<i32>,
    State(state): State<AppState>,
    ValidatedJson(mut payload): ValidatedJson<UpdateAdminPayload>, // 🛠️ 防弹衣：自动触发格式校验
) -> Result<Json<ApiResponse<Admin>>, DbError> {
    let mut payload = UpdateAdminPayload {
    username: empty_to_none(payload.username),
    email: empty_to_none(payload.email),
    password: empty_to_none(payload.password),
    role: empty_to_none(payload.role),
};
    
    // 如果前端传了新密码，必须先进行加密转换，否则不能直接入库！
    if let Some(plain_password) = payload.password {
        let hashed = hash(&plain_password, DEFAULT_COST)
            .map_err(|_| DbError::Internal("Password hashing failed".to_string()))?;
        payload.password = Some(hashed);
    }

    let admin = db::update_admin(&state.db_pool, admin_id, &payload).await?;
    Ok(Json(ApiResponse::success(admin)))
}

// DELETE /api/admins/:id
pub async fn handler_delete_admin(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能删除
    Path(admin_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, DbError> {
    db::delete_admin(&state.db_pool, admin_id).await?;
    Ok(Json(ApiResponse::success(())))
}



pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>,DbError> {

    let res = service::login(&state.db_pool, payload).await?;

    Ok(Json(ApiResponse::success(res)))
    
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


