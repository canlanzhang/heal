use axum::{
    Json, 

    extract::{Path, State}, 

};



use crate::state::AppState; 




use infrastructure::{
    entity::{
        Menu,

    }, 
    errors::*,

}; // 引入底层的基础设施和连接池
use infrastructure::dto::*;
//use infrastructure::service::login;
use infrastructure::service;



pub async fn list_menus(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<MenuItem>>>, DbError> {

    let menus = service::list_menus(&state.db_pool).await?;

    Ok(Json(ApiResponse::success(menus)))
}

pub async fn create_menu(
    State(state): State<AppState>,
    Json(payload): Json<CreateMenuPayload>,
) -> Result<Json<ApiResponse<()>>, DbError> {
    service::create_menu(&state.db_pool, payload).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn get_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Menu>>, DbError> {
    let data = service::get_menu(&state.db_pool, id).await?;
    Ok(Json(ApiResponse::success(data)))
}


pub async fn update_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMenuPayload>,
) -> Result<Json<ApiResponse<()>>, DbError> {
    service::update_menu(&state.db_pool, id, payload).await?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn delete_menu(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, DbError> {
    service::delete_menu(&state.db_pool, id).await?;
    Ok(Json(ApiResponse::success(())))
}


