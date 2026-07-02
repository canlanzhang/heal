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



//pub use auth::*;
//pub use users::*;
//pub use menus::*;
//pub use articles::*;

pub fn get_router(state: AppState) -> Router<AppState> {
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
        .with_state(state)
}

