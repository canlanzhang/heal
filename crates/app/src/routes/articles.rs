use axum::{routing::{post, delete, patch, get}, Router};
use std::sync::Arc;
use crate::state::AppState;
use crate::handlers::{
    list_articles,
    create_article,
    update_article,
    delete_article,
    get_article,
    };


pub fn articles_router() -> Router<Arc<AppState>> {
    Router::new()
        // ================= Article =================
        .route("/", get(list_articles))
        .route("/", post(create_article))
        .route("/{id}", get(get_article))        
        .route("/{id}", patch(update_article))
        .route("/{id}", delete(delete_article))
}

