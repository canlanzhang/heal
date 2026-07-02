use crate::state::AppState;
use std::sync::Arc;
use axum::{routing::{get}, Router};
use tower_http::cors::CorsLayer;


async fn health_check() -> &'static str {
    "OK"
}

pub mod auth;
pub mod users;
pub mod menus;
pub mod articles;

use self::auth::auth_router;
use self::users::users_router;
use self::menus::menus_router;
use self::articles::articles_router;


pub fn create_router(state: AppState) -> Router  {
    // 用 Arc 包装状态
    let shared_state = Arc::new(state);

    Router::new()
        // ================= AUTH =================
        .nest("/api/v1/auth", auth_router())
        // ================= USER =================
        .nest("/api/v1/users", users_router())
        // ================= MENU =================
        .nest("/api/v1/menus", menus_router())
        // ================= Article =================
        .nest("/api/v1/articles", articles_router())

        // ================= USER (C端 / 小程序) =================
        //.route("/api/app/users", post(handler_create_user))
        //.route("/api/app/users/{id}", get(handler_get_user))
        //.route("/api/app/users/{id}", patch(handler_patch_user))
        //.route("/api/app/users/{id}", delete(handler_delete_user))


        .route("/api/v1/health", get(health_check))

        .layer(CorsLayer::permissive())
        .with_state(shared_state)
}

