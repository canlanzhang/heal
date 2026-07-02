use axum::{routing::{post, delete, patch, get}, Router};
use std::sync::Arc;
use crate::state::AppState;
use crate::handlers::{
    list_menus,
    create_menu,
    get_menu,
    update_menu,
    delete_menu,    
    };

pub fn menus_router() -> Router<Arc<AppState>> {
    Router::new()
        // ================= MENU =================
        .route("/", get(list_menus))          
        .route("/", post(create_menu))        
        .route("/{id}", get(get_menu))
        .route("/{id}", patch(update_menu))  
        .route("/{id}", delete(delete_menu)) 
        

}

