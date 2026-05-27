
use axum::{routing::{get, post }, Router};
use crate::state::AppState;
use crate::handlers::{create_user_handler, handle_get_user};
use tower_http::cors::CorsLayer;
async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/users", post(create_user_handler))
        .route("/api/user/{id}",get(handle_get_user))
        .layer(CorsLayer::permissive())
        .with_state(state) // Axum 0.8 标准的状态注入方式
}

