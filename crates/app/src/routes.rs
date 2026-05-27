
use axum::{routing::{get, post }, Router};
use crate::state::AppState;
use crate::handlers::{create_user_handler, handle_get_user};

async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user_handler))
        .route("/user/{id}",get(handle_get_user))
        .with_state(state) // Axum 0.8 标准的状态注入方式
}

