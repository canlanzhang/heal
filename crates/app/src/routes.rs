
use axum::{routing::{post, delete, patch, get}, Router};
use crate::state::AppState;
use crate::handlers::{login_handler,
    create_user_handler, delete_user_handler, update_user_handler, handle_get_user};
use tower_http::cors::CorsLayer;
async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/api/login", post(login_handler))
        .route("/api/health", get(health_check))
        .route("/api/users", post(create_user_handler))
        .route("/api/users/{id}", delete(delete_user_handler))
        .route("/api/users/{id}", patch(update_user_handler))
        .route("/api/users/{id}",get(handle_get_user))
        .layer(CorsLayer::permissive())
        .with_state(state) // Axum 0.8 标准的状态注入方式
}

