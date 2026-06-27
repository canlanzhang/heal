use sqlx::PgPool;
use crate::db;
use crate::dto::menu::*;
use crate::entity::Menu;
use crate::errors::DbError;

pub async fn list_menus(pool: &PgPool) -> Result<Vec<MenuItem>, DbError> {
    db::list_menus(pool).await
}

pub async fn get_menus_by_role(
    pool: &PgPool,
    role: &str,
) -> Result<Vec<MenuItem>, DbError> {

    let role = if role.trim().is_empty() {
        "super_admin"
    } else {
        role
    };

    db::get_menus_by_role(pool, role).await
}

pub async fn create_menu(
    pool: &PgPool,
    payload: CreateMenuPayload,
) -> Result<(), DbError> {
    db::create_menu(pool, payload).await
}


pub async fn get_menu(pool: &PgPool, id: i32) -> Result<Menu, DbError> {
    db::menu::get_menu_by_id(pool, id).await
}

pub async fn update_menu(
    pool: &PgPool,
    id: i32,
    payload: UpdateMenuPayload,
) -> Result<(), DbError> {
    db::menu::update_menu(pool, id, payload).await
}

pub async fn delete_menu(pool: &PgPool, id: i32) -> Result<(), DbError> {
    db::menu::delete_menu(pool, id).await
}



