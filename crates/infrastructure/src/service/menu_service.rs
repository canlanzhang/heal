use sqlx::PgPool;
use crate::dto::menu::*;
use crate::errors::DbError;
use crate::db::menu as menu_db;

pub async fn list_menus(pool: &PgPool) -> Result<Vec<MenuItem>, DbError> {
    menu_db::list_menus(pool).await
}

pub async fn get_menus_by_role(
    pool: &PgPool,
    role: &str,
) -> Result<Vec<MenuItem>, DbError> {

    let role = if role.trim().is_empty() {
        "admin"
    } else {
        role
    };

    menu_db::get_menus_by_role(pool, role).await
}

pub async fn create_menu(
    pool: &PgPool,
    payload: CreateMenuPayload,
) -> Result<(), DbError> {
    menu_db::create_menu(pool, payload).await
}

pub async fn update_menu(
    pool: &PgPool,
    id: i32,
    payload: UpdateMenuPayload,
) -> Result<(), DbError> {
    menu_db::update_menu(pool, id, payload).await
}

pub async fn delete_menu(pool: &PgPool, id: i32) -> Result<(), DbError> {
    menu_db::delete_menu(pool, id).await
}