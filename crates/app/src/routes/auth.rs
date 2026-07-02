use axum::{routing::{post,get}, Router};
use crate::state::AppState;
use crate::handlers::{
    login, 
    profile,
    };

pub fn auth_router() -> Router<AppState> {
    Router::new()
        // ================= AUTH =================
        .route("/login", post(login))
        .route("/profile", get(profile))
}

