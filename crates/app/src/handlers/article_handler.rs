use axum::{
    Json, 

    extract::{Path, State}, 

};



use crate::state::AppState; 


use infrastructure::dto::auth::{
    Claims,
};

use infrastructure::{
    entity::{
        Article,

    }, 
    errors::*,

}; // 引入底层的基础设施和连接池
use infrastructure::dto::*;
//use infrastructure::service::login;
use infrastructure::service;





pub async fn handler_list_articles(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能删除
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ArticleListItem>>>, DbError> {
    
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

