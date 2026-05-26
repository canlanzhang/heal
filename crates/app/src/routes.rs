use axum::{routing::get, Router};
use crate::state::AppState;

async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state) // Axum 0.8 标准的状态注入方式
}