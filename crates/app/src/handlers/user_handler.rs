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
    dto::*,
    service::users_service,
    entity::{
        User,

    }, 
    errors::*,

}; // 引入底层的基础设施和连接池





// GET /users
pub async fn list_users(
    _claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserListItem>>>, AppError> {

    let list = users_service::list_users(&state.db_pool)
    .await
    .map_err(ApiError::from)?;

    Ok(Json(ApiResponse::success(list)))
}


// POST /users
pub async fn create_user(
    _claims: Claims,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserPayload>,
) -> Result<Json<ApiResponse<User>>, DbError> {

    let user = users_service::create_user(
        &state.db_pool, 
        payload
    ).await?;

    Ok(Json(ApiResponse::success(user)))
}


pub async fn get_user(
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
pub async fn update_user(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能修改
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<UpdateUserPayload>, // 🛠️ 防弹衣：自动触发格式校验
) -> Result<Json<ApiResponse<User>>, DbError> {

    let user = users_service::update_user(
            &state.db_pool, 
            user_id, payload
        ).await?;

    Ok(Json(ApiResponse::success(user)))
}

// DELETE /api/users/:id
pub async fn delete_user(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能删除
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, DbError> {

    users_service::delete_user(&state.db_pool, user_id).await?;

    Ok(Json(ApiResponse::success(())))
}

