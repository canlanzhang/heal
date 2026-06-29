use crate::state::AppState; 
use axum::{
    Json, 
    extract::{Path, State}, 
};
use infrastructure::{
    entity::Article, 
    service::articles_service,
    dto::{
        auth::Claims,
        articles::{ArticleListItem, CreateArticlePayload,UpdateArticlePayload },
        ApiResponse,
    },
    errors::{AppError,ApiError},
}; // 引入底层的基础设施和连接池

pub async fn list_articles(
    _claims: Claims, // 🛠️ 鉴权守卫：必须登录才能删除
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ArticleListItem>>>, AppError> {
    
    let articles = articles_service::list_articles(
        &state.db_pool
    ).await
    .map_err(ApiError::from)?;

    Ok(Json(ApiResponse::success(articles)))
}

pub async fn create_article(
    claims: Claims,
    State(state): State<AppState>,
    Json(payload): Json<CreateArticlePayload>,
) -> Result<Json<ApiResponse<Article>>, AppError> {

    let data = articles_service::create_article(
        &state.db_pool,
        payload,
        claims.sub,
    ).await
    .map_err(ApiError::from)?;

    Ok(Json(ApiResponse::success(data)))
}


pub async fn update_article(
    _claims: Claims,
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateArticlePayload>,
) -> Result<Json<ApiResponse<Article>>, AppError> {

    let data = articles_service::update_article(
        &state.db_pool, 
        id, 
        payload
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(data)))
}

pub async fn delete_article(
    _claims: Claims,
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, AppError> {

    articles_service::delete_article(
        &state.db_pool, 
        id
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(())))
}


pub async fn get_article(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Article>>, AppError> {

    let data = articles_service::get_article_by_id(
        &state.db_pool, 
        id
    ).await
    .map_err(ApiError::from)?;
    Ok(Json(ApiResponse::success(data)))
}

