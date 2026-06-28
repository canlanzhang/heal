
use axum::{routing::{post, delete, patch, get}, Router};
use tower_http::cors::CorsLayer;

use crate::state::AppState;
use crate::handlers::{
    login, profile,

    list_users,create_user,get_user,update_user,delete_user,

    list_menus,create_menu,get_menu,update_menu,delete_menu,
    
    list_articles,create_article,update_article,delete_article,get_article,
    };

async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()

        // ================= AUTH =================
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/profile", get(profile))

        // ================= ADMIN =================
        .route("/api/v1/users", get(list_users))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/{id}", get(get_user))
        .route("/api/v1/users/{id}", patch(update_user))
        .route("/api/v1/users/{id}", delete(delete_user))

        // ================= MENU =================
        .route("/api/v1/menus", get(list_menus))          // 菜单列表（后台）
        .route("/api/v1/menus", post(create_menu))        // 创建菜单
        .route("/api/v1/menus/{id}", get(get_menu))
        .route("/api/v1/menus/{id}", patch(update_menu))  // 更新菜单
        .route("/api/v1/menus/{id}", delete(delete_menu)) // 删除菜单

        // ================= Article =================
        .route("/api/v1/articles", get(list_articles))
        .route("/api/v1/articles", post(create_article))
        .route("/api/v1/articles/{id}", get(get_article))        
        .route("/api/v1/articles/{id}", patch(update_article))
        .route("/api/v1/articles/{id}", delete(delete_article))

        // ================= USER (C端 / 小程序) =================
        //.route("/api/app/users", post(handler_create_user))
        //.route("/api/app/users/{id}", get(handler_get_user))
        //.route("/api/app/users/{id}", patch(handler_patch_user))
        //.route("/api/app/users/{id}", delete(handler_delete_user))

        // ================= HEALTH =================
        .route("/api/v1/health", get(health_check))

        .layer(CorsLayer::permissive())
        .with_state(state)
}

