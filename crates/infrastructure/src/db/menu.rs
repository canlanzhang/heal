use sqlx::PgPool;
use crate::dto::menu::*;
use crate::entity::Menu;
use crate::errors::DbError;

pub async fn list_menus(pool: &PgPool) -> Result<Vec<MenuItem>, DbError> {
    let menus = sqlx::query_as!(
        MenuItem,
        r#"
        SELECT id,name, path, title, icon
        FROM heal_menus
        ORDER BY sort ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(menus)
}

pub async fn get_menus_by_role(
    pool: &PgPool,
    role: &str,
) -> Result<Vec<MenuItem>, DbError> {
    let menus = sqlx::query_as!(
        MenuItem,
        r#"
        SELECT id,name, path, title, icon
        FROM heal_menus
        WHERE role = $1
        ORDER BY sort ASC
        "#,
        role
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(menus)
}

pub async fn get_menu_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<Menu, DbError> {
    let menu = sqlx::query_as!(
        Menu,
        r#"
        SELECT id, name, path, title, icon, role, parent_id, sort
        FROM heal_menus
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(menu)
}

pub async fn create_menu(
    pool: &PgPool,
    payload: CreateMenuPayload,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        INSERT INTO heal_menus
        (name, path, title, icon, role, parent_id, sort)
        VALUES ($1,$2,$3,$4,$5,$6,$7)
        "#,
        payload.name,
        payload.path,
        payload.title,
        payload.icon,
        payload.role.unwrap_or("admin".to_string()),
        payload.parent_id,
        payload.sort.unwrap_or(0)
    )
    .execute(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(())
}

pub async fn update_menu(
    pool: &PgPool,
    id: i32,
    payload: UpdateMenuPayload,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE heal_menus
        SET
            name = COALESCE($1, name),
            path = COALESCE($2, path),
            title = COALESCE($3, title),
            icon = COALESCE($4, icon),
            role = COALESCE($5, role),
            parent_id = COALESCE($6, parent_id),
            sort = COALESCE($7, sort),
            updated_at = NOW()
        WHERE id = $8
        "#,
        payload.name,
        payload.path,
        payload.title,
        payload.icon,
        payload.role,
        payload.parent_id,
        payload.sort,
        id
    )
    .execute(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(())
}

pub async fn delete_menu(pool: &PgPool, id: i32) -> Result<(), DbError> {
    sqlx::query!(
        "DELETE FROM heal_menus WHERE id = $1",
        id
    )
    .execute(pool)
    .await
    .map_err(DbError::Sql)?;

    Ok(())
}