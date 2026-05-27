
use axum::{routing::get, Router};
use crate::state::AppState;
use crate::handlers::{handle_get_user};

async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/user/{id}",get(handle_get_user))
        .with_state(state) // Axum 0.8 标准的状态注入方式
}

