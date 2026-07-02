use axum::{routing::{post, delete, patch, get}, Router};
use std::sync::Arc;
use crate::state::AppState;
use crate::handlers::{
    list_users,
    create_user,
    get_user,
    update_user,
    delete_user,
    };

pub fn users_router() -> Router<Arc<AppState>> {
    Router::new()
        // ================= ADMIN =================
        .route("/", get(list_users))
        .route("/", post(create_user))
        .route("/{id}", get(get_user))
        .route("/{id}", patch(update_user))
        .route("/{id}", delete(delete_user))
       
}

