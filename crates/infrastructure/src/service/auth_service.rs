use sqlx::PgPool;
use bcrypt::verify;

use crate::db;
use crate::dto::auth::*;
use crate::errors::DbError;

pub async fn login(
    pool: &PgPool,
    payload: LoginRequest,
) -> Result<LoginResponse, DbError> {

    let admin = db::query_admin_for_login(pool, &payload.username).await?;

    let ok = verify(&payload.password, &admin.password_hash)
        .map_err(|_| DbError::Internal("bcrypt error".into()))?;

    if !ok {
        return Err(DbError::Unauthorized);
    }

    let token = Claims::generate_token(admin.id).map_err(|_| DbError::TokenError)?;

    Ok(LoginResponse {
        token,
        username: admin.username,
    })
}



use crate::dto::admin::{AdminListItem,AdminProfileResponse, AdminInfo};
use crate::dto::common::MenuItem;


pub async fn get_admin_profile(
    pool: &PgPool,
    admin_id: i32,
) -> Result<AdminProfileResponse, DbError> {

    let admin = db::get_admin_by_id(pool, admin_id).await?;

    let info = AdminInfo {
        id: admin.id,
        username: admin.username,
        email: admin.email,
        role: admin.role,
    };

    let menus = vec![
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
    ];

    Ok(AdminProfileResponse {
        admin: info,
        menus,
    })
}