use axum::{
    Json, 
    extract::{State}, 
};

use crate::state::AppState; 

use infrastructure::dto::users::{
    UserProfileResponse,

};

use infrastructure::dto::auth::{
    LoginRequest,
    LoginResponse,
    Claims,
};

use infrastructure::{
    service,
    service::users_service,
dto::*,
    errors::*,

}; // 引入底层的基础设施和连接池


pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>,DbError> {

    let res = service::login(&state.db_pool, payload).await?;

    Ok(Json(ApiResponse::success(res)))
    
}

pub async fn profile(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, DbError> {

    let data = users_service::get_user_profile(
        &state.db_pool,
        claims.sub,
    ).await?;

    Ok(Json(ApiResponse::success(data)))
}



