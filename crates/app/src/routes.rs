
use axum::{routing::{post, delete, patch, get}, Router};
use crate::state::AppState;
use crate::handlers::{
    handler_login, handler_profile,

    handler_list_users,handler_create_users,handler_get_users,handler_update_users,handler_delete_users,

    handler_list_menus,handler_create_menu,handler_get_menu,handler_update_menu,handler_delete_menu,
    
    handler_list_articles,handler_create_article,handler_update_article,handler_delete_article,handler_get_article,
    };
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
        .route("/api/v1/users", get(handler_list_users))
        .route("/api/v1/users", post(handler_create_users))
        .route("/api/v1/users/{id}", get(handler_get_users))
        .route("/api/v1/users/{id}", patch(handler_update_users))
        .route("/api/v1/users/{id}", delete(handler_delete_users))

        // ================= MENU =================
        .route("/api/v1/menus", get(handler_list_menus))          // 菜单列表（后台）
        .route("/api/v1/menus", post(handler_create_menu))        // 创建菜单
        .route("/api/v1/menus/{id}", get(handler_get_menu))
        .route("/api/v1/menus/{id}", patch(handler_update_menu))  // 更新菜单
        .route("/api/v1/menus/{id}", delete(handler_delete_menu)) // 删除菜单

        // ================= Article =================
        .route("/api/v1/articles", get(handler_list_articles))
        .route("/api/v1/articles", post(handler_create_article))
        .route("/api/v1/articles/{id}", get(handler_get_article))        
        .route("/api/v1/articles/{id}", patch(handler_update_article))
        .route("/api/v1/articles/{id}", delete(handler_delete_article))

        // ================= USER (C端 / 小程序) =================
        //.route("/api/app/users", post(handler_create_user))
        //.route("/api/app/users/{id}", get(handler_get_user))
        //.route("/api/app/users/{id}", patch(handler_patch_user))
        //.route("/api/app/users/{id}", delete(handler_delete_user))

        // ================= HEALTH =================
        .route("/api/health", get(health_check))

        .layer(CorsLayer::permissive())
        .with_state(state)
}

