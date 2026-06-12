
use axum::{routing::{post, delete, patch, get}, Router};
use crate::state::AppState;
use crate::handlers::{
    handler_create_admin,
    login_handler,
    handler_create_user, handler_delete_user, handler_patch_user, handle_get_user};
use tower_http::cors::CorsLayer;
async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/api/admins", post(handler_create_admin))
        .route("/api/login", post(login_handler))
       
        
        .route("/api/health", get(health_check))
        .route("/api/users", post(handler_create_user))
        .route("/api/users/{id}", delete(handler_delete_user))
        .route("/api/users/{id}", patch(handler_patch_user))
        .route("/api/users/{id}",get(handle_get_user))
        .layer(CorsLayer::permissive())
        .with_state(state) // Axum 0.8 标准的状态注入方式
}

