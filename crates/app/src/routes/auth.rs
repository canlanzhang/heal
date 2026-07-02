use axum::{routing::{post,get}, Router};
use std::sync::Arc;
use crate::state::AppState;
use crate::handlers::{
    login, 
    profile,
    };

pub fn auth_router() -> Router<Arc<AppState>> {
    Router::new()
        // ================= AUTH =================
        .route("/login", post(login))
        .route("/profile", get(profile))
}

