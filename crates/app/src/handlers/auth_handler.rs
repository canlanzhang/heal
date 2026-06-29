use axum::{
    extract::{State}, 
    Json,     
};
use crate::state::AppState; 
use infrastructure::{
    service::{auth_service, users_service },
    dto::{
        auth::{LoginRequest, LoginResponse, Claims },
        users::UserProfileResponse,
        ApiResponse,
    },
    errors::{AppError, ApiError},
}; 

// 登录接口
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>,AppError> {
    // 自动触发 #[from] 转换，代码极其干净
    let token = auth_service::login(
        &state.db_pool, 
        payload
    ).await?;
    Ok(Json(ApiResponse::success(token)))
}

// 获取用户资料接口
pub async fn profile(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, AppError> {
    let user = users_service::get_user_profile(
        &state.db_pool,
        claims.sub,
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(user)))
}