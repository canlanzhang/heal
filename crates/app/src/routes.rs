
use axum::{routing::{post, delete, patch, get}, Router};
use crate::state::AppState;
use crate::handlers::{
    handler_list_admins,handler_create_admin,handler_get_admin,handler_update_admin,handler_delete_admin,
    handler_login, handler_profile,
    handler_list_articles,handler_create_article,handler_update_article,handler_delete_article,handler_get_article,
    handler_create_user, handler_delete_user, handler_patch_user, handler_get_user};
use tower_http::cors::CorsLayer;
async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()

        // ================= AUTH =================
        .route("/api/v1/auth/login", post(handler_login))
        .route("/api/v1/auth/profile", get(handler_profile))

        // ================= ADMIN =================
        .route("/api/v1/admins", get(handler_list_admins))
        .route("/api/v1/admins", post(handler_create_admin))
        .route("/api/v1/admins/{id}", get(handler_get_admin))
        .route("/api/v1/admins/{id}", patch(handler_update_admin))
        .route("/api/v1/admins/{id}", delete(handler_delete_admin))

        // ================= Article =================
        .route("/api/v1/articles", get(handler_list_articles))
        .route("/api/v1/articles", post(handler_create_article))
        .route("/api/v1/articles/{id}", get(handler_get_article))        
        .route("/api/v1/articles/{id}", patch(handler_update_article))
        .route("/api/v1/articles/{id}", delete(handler_delete_article))

        // ================= USER (C端 / 小程序) =================
        .route("/api/app/users", post(handler_create_user))
        .route("/api/app/users/{id}", get(handler_get_user))
        .route("/api/app/users/{id}", patch(handler_patch_user))
        .route("/api/app/users/{id}", delete(handler_delete_user))

        // ================= HEALTH =================
        .route("/api/health", get(health_check))

        .layer(CorsLayer::permissive())
        .with_state(state)
}

