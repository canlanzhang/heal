use axum::{
    Json, 
    extract::{Path, State}, 
};

use crate::state::AppState; 

use infrastructure::{ 
    dto::*, entity::Menu, errors::*, service::{menus_service},

}; // 引入底层的基础设施和连接池


pub async fn list_menus(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<MenuItem>>>, AppError> {

    let menus = menus_service::list_menus(
        &state.db_pool
    ).await
    .map_err(ApiError::from)?;
    

    Ok(Json(ApiResponse::success(menus)))
}

pub async fn create_menu(
    State(state): State<AppState>,
    Json(payload): Json<CreateMenuPayload>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    menus_service::create_menu(
        &state.db_pool, 
        payload
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn get_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Menu>>, AppError> {
    let data = menus_service::get_menu(
        &state.db_pool, 
        id
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(data)))
}


pub async fn update_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMenuPayload>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    menus_service::update_menu(
        &state.db_pool, 
        id, payload
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn delete_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    menus_service::delete_menu(
        &state.db_pool, 
        id
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(())))
}


