
use axum::{routing::{post, delete, patch, get}, Router};
use crate::state::AppState;
use crate::handlers::{
    handler_create_admin,handler_patch_admin,handler_delete_admin,
    login_handler,
    handler_admin_info,
    handler_create_user, handler_delete_user, handler_patch_user, handle_get_user};
use tower_http::cors::CorsLayer;
async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()

        // ================= AUTH =================
        .route("/api/system/auth/login", post(login_handler))
        .route("/api/system/auth/me", get(handler_admin_info))

        // ================= ADMIN =================
        .route("/api/system/admins", post(handler_create_admin))
        .route("/api/system/admins/{id}", patch(handler_patch_admin))
        .route("/api/system/admins/{id}", delete(handler_delete_admin))

        // ================= USER (C端 / 小程序) =================
        .route("/api/app/users", post(handler_create_user))
        .route("/api/app/users/{id}", get(handle_get_user))
        .route("/api/app/users/{id}", patch(handler_patch_user))
        .route("/api/app/users/{id}", delete(handler_delete_user))

        // ================= HEALTH =================
        .route("/api/health", get(health_check))

        .layer(CorsLayer::permissive())
        .with_state(state)
}

