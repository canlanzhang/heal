use crate::state::AppState; 
use std::sync::Arc;
use axum::{
    Json, 
    extract::{Path, State}, 
};
use infrastructure::{ 
    dto::{
        auth::Claims,
        menus::{MenuItem, CreateMenuPayload,UpdateMenuPayload },
        ApiResponse,
    },
    entity::Menu, 
    errors::{AppError,ApiError}, 
    service::{menus_service},
}; 


pub async fn list_menus(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<MenuItem>>>, AppError> {

    let menus = menus_service::list_menus(
        &state.db_pool
    ).await
    .map_err(ApiError::from)?;
    
    Ok(Json(ApiResponse::success(menus)))
}

pub async fn create_menu(
    _claims: Claims,
    State(state): State<Arc<AppState>>,
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
    _claims: Claims,
    State(state): State<Arc<AppState>>,
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
    _claims: Claims,
    State(state): State<Arc<AppState>>,
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
    _claims: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    menus_service::delete_menu(
        &state.db_pool, 
        id
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(())))
}


