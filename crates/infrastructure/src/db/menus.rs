use sqlx::PgPool;
use crate::entity::Menu;
use crate::dto::menus::{
    MenuItem,
    CreateMenuPayload,
    UpdateMenuPayload
};
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
    .await?;

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
    .await?;

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
    .fetch_optional(pool)
    .await?;

    menu.ok_or(DbError::NotFound) //返回 404
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
    .await?;

    Ok(())
}

pub async fn update_menu(
    pool: &PgPool,
    id: i32,
    payload: UpdateMenuPayload,
) -> Result<(), DbError> {
    // 先检查菜单是否存在
    let existing = sqlx::query!("SELECT id FROM heal_menus WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;

    if existing.is_none() {
        return Err(DbError::NotFound);
    }

    //使用 CASE WHEN 实现真正的部分更新（允许将字段设为 NULL）
    sqlx::query!(
        r#"
        UPDATE heal_menus
        SET
            name = CASE WHEN $1::TEXT IS NOT NULL THEN $1 ELSE name END,
            path = CASE WHEN $2::TEXT IS NOT NULL THEN $2 ELSE path END,
            title = CASE WHEN $3::TEXT IS NOT NULL THEN $3 ELSE title END,
            icon = CASE WHEN $4::TEXT IS NOT NULL THEN $4 ELSE icon END,
            role = CASE WHEN $5::TEXT IS NOT NULL THEN $5 ELSE role END,
            parent_id = CASE WHEN $6::INT IS NOT NULL THEN $6 ELSE parent_id END,
            sort = CASE WHEN $7::INT IS NOT NULL THEN $7 ELSE sort END
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
    .await?;

    Ok(())
}

//删除必须检查影响行数
pub async fn delete_menu(pool: &PgPool, id: i32) -> Result<(), DbError> {
    let res = sqlx::query!("DELETE FROM heal_menus WHERE id = $1",id)
    .execute(pool)
    .await?;
    //如果没有删除任何行，说明 ID 不存在
    if res.rows_affected() == 0 {
        return Err(DbError::NotFound)
    }

    Ok(())
}