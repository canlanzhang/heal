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
    errors::{ApiError, AuthError},

}; // 引入底层的基础设施和连接池


pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>,ApiError> {

    let res = auth_service::login(&state.db_pool, payload)
    .await?;
    //.map_err(ApiError::Auth)?;

    Ok(Json(ApiResponse::success(res)))
    
}

pub async fn profile(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, ApiError> {

    let data = users_service::get_user_profile(
        &state.db_pool,
        claims.sub,
    ).await?;
    //.map_err(ApiError::Db)?;

    Ok(Json(ApiResponse::success(data)))
}



