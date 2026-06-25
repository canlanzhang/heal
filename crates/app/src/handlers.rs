use axum::{
    Json, 
    http::StatusCode,
    extract::{Path, State}, 
    response::IntoResponse
};



use crate::state::AppState; 
use infrastructure::dto::users::{
    UserProfileResponse,
    UserInfo,
};

use infrastructure::dto::auth::{
    LoginRequest,
    LoginResponse,
    Claims,
};

use infrastructure::{
    db, 
    entity::{
        User,
        Menu,
        Article,

    }, 
    errors::*,

}; // 引入底层的基础设施和连接池
use infrastructure::dto::*;
use serde::{Serialize, Deserialize};
use bcrypt::{hash,DEFAULT_COST,verify, BcryptError};
//use infrastructure::service::login;
use infrastructure::service;
use infrastructure::service::{users_service,auth_service};

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
) -> Result<Json<ApiResponse<UserProfileResponse>>, DbError> {

    let data = users_service::get_user_profile(
        &state.db_pool,
        claims.sub,
    ).await?;

    Ok(Json(ApiResponse::success(data)))
}



// GET /users
pub async fn handler_list_users(
    _claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserListItem>>>, DbError> {

    let list = users_service::list_users(&state.db_pool).await?;

    Ok(Json(ApiResponse::success(list)))
}


// POST /users
pub async fn handler_create_users(
    _claims: Claims,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserPayload>,
) -> Result<Json<ApiResponse<User>>, DbError> {

    let user = service::create_user(
        &state.db_pool, 
        payload
    ).await?;

    Ok(Json(ApiResponse::success(user)))
}


pub async fn handler_get_users(
    claims: Claims,
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, DbError> {

    let data = users_service::get_user(
        &state.db_pool,
        user_id,
    ).await?;

    Ok(Json(ApiResponse::success(data)))
}


// PATCH /api/users/:id
pub async fn handler_update_users(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能修改
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<UpdateUserPayload>, // 🛠️ 防弹衣：自动触发格式校验
) -> Result<Json<ApiResponse<User>>, DbError> {

    let user = service::update_user(
            &state.db_pool, 
            user_id, payload
        ).await?;

    Ok(Json(ApiResponse::success(user)))
}

// DELETE /api/users/:id
pub async fn handler_delete_users(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能删除
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, DbError> {

    service::delete_user(&state.db_pool, user_id).await?;

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

