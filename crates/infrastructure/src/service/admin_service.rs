use sqlx::PgPool;

use crate::db;
use crate::dto::admin::{AdminListItem,AdminProfileResponse, AdminInfo};

use crate::errors::DbError;
use crate::service::get_menus_by_role;




use bcrypt::{hash, DEFAULT_COST};

use crate::dto::admin::CreateAdminPayload;
use crate::dto::admin::UpdateAdminPayload;
use crate::entity::Admin;



pub async fn get_admin_profile(
    pool: &PgPool,
    admin_id: i32,
) -> Result<AdminProfileResponse, DbError> {

    let admin = db::get_admin_by_id(pool, admin_id).await?;

    let menus = get_menus_by_role(pool, &admin.role).await?;

    let info = AdminInfo {
        id: admin.id,
        username: admin.username,
        email: admin.email,
        role: admin.role,
    };
    
    Ok(AdminProfileResponse {
        admin: info,
        menus,
    })
}


pub async fn list_admins(
    pool: &PgPool,
) -> Result<Vec<AdminListItem>, DbError> {

    let admins = db::admin::list_admins(pool).await?;

    let result = admins.into_iter().map(|a| AdminListItem {
        id: a.id,
        username: a.username,
        email: a.email,
        role: a.role,
    }).collect();

    Ok(result)
}

pub async fn create_admin(
    pool: &PgPool,
    mut payload: CreateAdminPayload,
) -> Result<Admin, DbError> {

    payload.password = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| DbError::Internal("Password hashing failed".into()))?;

    db::create_admin(pool, &payload).await
}


pub async fn update_admin(
    pool: &PgPool,
    admin_id: i32,
    mut payload: UpdateAdminPayload,
) -> Result<Admin, DbError> {

    if let Some(pwd) = payload.password {
        payload.password = Some(
            bcrypt::hash(&pwd, DEFAULT_COST)
                .map_err(|_| DbError::Internal("hash failed".into()))?
        );
    }

    db::update_admin(pool, admin_id, &payload).await
}



pub async fn delete_admin(
    pool: &PgPool,
    admin_id: i32,
) -> Result<(), DbError> {

    db::delete_admin(pool, admin_id).await
}


pub async fn get_admin(
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

    let menus = vec![];

    Ok(AdminProfileResponse {
        admin: info,
        menus,
    })
}