use axum::{
    Json, 

    extract::{Path, State}, 

};



use crate::state::AppState; 
use infrastructure::dto::users::{
    UserProfileResponse,

};

use infrastructure::dto::auth::{

    Claims,
};

use infrastructure::{
    entity::{
        User,

    }, 
    errors::*,

}; // 引入底层的基础设施和连接池
use infrastructure::dto::*;
//use infrastructure::service::login;
use infrastructure::service;
use infrastructure::service::{users_service};




// GET /users
pub async fn handler_list_users(
    _claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserListItem>>>, DbError> {

    let list = users_service::list_users(&state.db_pool).await?;

    Ok(Json(ApiResponse::success(list)))
}


// POST /users
pub async fn handler_create_user(
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


pub async fn handler_get_user(
    _claims: Claims,
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
pub async fn handler_update_user(
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
pub async fn handler_delete_user(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能删除
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, DbError> {

    service::delete_user(&state.db_pool, user_id).await?;

    Ok(Json(ApiResponse::success(())))
}

