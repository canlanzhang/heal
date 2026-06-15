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


pub async fn handler_admin_info(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AdminProfileResponse>>, DbError> {

    // 1. 从 JWT 拿 admin_id
    let admin_id = claims.sub;

    // 2. 查 admin
    let admin = db::get_admin_by_id(&state.db_pool, admin_id).await?;

    // 3. 组装 admin info
    let admin_info = AdminInfo {
        id: admin.id,
        username: admin.username.clone(),
        email: admin.email.clone(),
        role: admin.role.clone(),
    };

    // 4. 菜单（先写死，后面可改 RBAC）
    let menus = vec![
        MenuItem {
            name: "home".to_string(),
            path: "/home".to_string(),
            title: "首页".to_string(),
            icon: "Home".to_string(),
        },
        MenuItem {
            name: "user".to_string(),
            path: "/user".to_string(),
            title: "用户管理".to_string(),
            icon: "User".to_string(),
        },
    ];

    // 5. 返回结构
    let data = AdminProfileResponse {
        admin: admin_info,
        menus,
    };

    Ok(Json(ApiResponse::success(data)))
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

    // 1. 查用户
    let admin = db::query_admin_for_login(&state.db_pool, &payload.username).await?;
    // 2. 验证密码
    let is_valid = bcrypt::verify(&payload.password, &admin.password_hash)
        .map_err(|_| DbError::Internal("Password verification failed".to_string()))?;
    

    if !is_valid {
        //tracing::debug!("login_handler: {}",is_valid);
        return Err(DbError::Unauthorized);
    }

    // 3. 生成 token
    let token = Claims::generate_token(admin.id)
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


