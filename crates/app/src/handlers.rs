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
        Admin,
        Article,
        User,

    }, 
    errors::*,

}; // 引入底层的基础设施和连接池
use infrastructure::dto::*;
use serde::{Serialize, Deserialize};
use bcrypt::{hash,DEFAULT_COST,verify, BcryptError};
//use infrastructure::service::login;
use infrastructure::service;
use infrastructure::service::{admin_service,auth_service};

pub async fn handler_login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>,DbError> {

    let res = service::login(&state.db_pool, payload).await?;

    Ok(Json(ApiResponse::success(res)))
    
}

pub async fn handler_profile(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AdminProfileResponse>>, DbError> {

    let data = admin_service::get_admin_profile(
        &state.db_pool,
        claims.sub,
    ).await?;

    Ok(Json(ApiResponse::success(data)))
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
    ValidatedJson(payload): ValidatedJson<CreateAdminPayload>,
) -> Result<Json<ApiResponse<Admin>>, DbError> {

    let admin = service::create_admin(
        &state.db_pool, 
        payload
    ).await?;

    Ok(Json(ApiResponse::success(admin)))
}


pub async fn handler_get_admin(
    claims: Claims,
    Path(admin_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AdminProfileResponse>>, DbError> {

    let data = admin_service::get_admin(
        &state.db_pool,
        admin_id,
    ).await?;

    Ok(Json(ApiResponse::success(data)))
}


// PATCH /api/admins/:id
pub async fn handler_update_admin(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能修改
    Path(admin_id): Path<i32>,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<UpdateAdminPayload>, // 🛠️ 防弹衣：自动触发格式校验
) -> Result<Json<ApiResponse<Admin>>, DbError> {

    let admin = service::update_admin(
            &state.db_pool, 
            admin_id, payload
        ).await?;

    Ok(Json(ApiResponse::success(admin)))
}

// DELETE /api/admins/:id
pub async fn handler_delete_admin(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能删除
    Path(admin_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, DbError> {

    service::delete_admin(&state.db_pool, admin_id).await?;

    Ok(Json(ApiResponse::success(())))
}


pub async fn handler_list_menus(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<MenuItem>>>, DbError> {

    let menus = service::list_menus(&state.db_pool).await?;

    Ok(Json(ApiResponse::success(menus)))
}

pub async fn handler_create_menu(
    State(state): State<AppState>,
    Json(payload): Json<CreateMenuPayload>,
) -> Result<Json<ApiResponse<()>>, DbError> {
    service::create_menu(&state.db_pool, payload).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn handler_get_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Menu>>, DbError> {
    let data = service::get_menu(&state.db_pool, id).await?;
    Ok(Json(ApiResponse::success(data)))
}


pub async fn handler_update_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMenuPayload>,
) -> Result<Json<ApiResponse<()>>, DbError> {
    service::update_menu(&state.db_pool, id, payload).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn handler_delete_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, DbError> {
    service::delete_menu(&state.db_pool, id).await?;
    Ok(Json(ApiResponse::success(())))
}


pub async fn handler_list_articles(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能删除
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Article>>>, DbError> {
    
    let articles = service::list_articles(&state.db_pool).await?;

    Ok(Json(ApiResponse::success(articles)))
}

pub async fn handler_create_article(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<CreateArticlePayload>,
) -> Result<Json<ApiResponse<Article>>, DbError> {

    let data = service::create_article(
        &state.db_pool,
        payload,
        claims.sub,
    ).await?;

    Ok(Json(ApiResponse::success(data)))
}


pub async fn handler_update_article(
    _claims: Claims,
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateArticlePayload>,
) -> Result<Json<ApiResponse<Article>>, DbError> {

    let data = service::update_article(&state.db_pool, id, payload).await?;

    Ok(Json(ApiResponse::success(data)))
}

pub async fn handler_delete_article(
    _claims: Claims,
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, DbError> {

    service::delete_article(&state.db_pool, id).await?;
    Ok(Json(ApiResponse::success(())))
}


pub async fn handler_get_article(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Article>>, DbError> {

    let data = service::get_article_by_id(&state.db_pool, id).await?;
    Ok(Json(ApiResponse::success(data)))
}


// GET /users/:id
pub async fn handler_get_user(
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


