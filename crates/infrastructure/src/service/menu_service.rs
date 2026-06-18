
use sqlx::PgPool;
use crate::errors::DbError;
use crate::dto::common::MenuItem;

pub async fn get_menus_by_role(
    pool: &PgPool,
    role: &str,
) -> Result<Vec<MenuItem>, DbError> {

    let menus = sqlx::query_as!(
        MenuItem,
        r#"
        SELECT name, path, title, icon
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

/* 
pub fn build_default_menus() -> Vec<MenuItem> {
    vec![
        MenuItem {
            name: "home".into(),
            path: "/home".into(),
            title: "首页".into(),
            icon: "Home".into(),
        },
        MenuItem {
            name: "admin".into(),
            path: "/admin".into(),
            title: "管理员管理".into(),
            icon: "Admin".into(),
        },
        MenuItem {
            name: "article".into(),
            path: "/article".into(),
            title: "内容管理".into(),
            icon: "Article".into(),
        },
        MenuItem {
            name: "user".into(),
            path: "/user".into(),
            title: "用户管理".into(),
            icon: "User".into(),
        },
    ]
}

    */