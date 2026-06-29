use sqlx::PgPool;

use crate::db;
use crate::dto::users::{UserListItem,UserProfileResponse, UserInfo};

use crate::errors::DbError;
use crate::service::get_menus_by_role;




use bcrypt::{hash, DEFAULT_COST};

use crate::dto::users::CreateUserPayload;
use crate::dto::users::UpdateUserPayload;
use crate::entity::User;



pub async fn get_user_profile(
    pool: &PgPool,
    user_id: i32,
) -> Result<UserProfileResponse, DbError> {

    let user = db::users::get_user_by_id(pool, user_id).await?;

    let menus = get_menus_by_role(pool, &user.role).await?;

    let info = UserInfo {
        id: user.id,
        username: user.username,
        email: user.email,
        role: user.role,
    };
    
    Ok(UserProfileResponse {
        user: info,
        menus,
    })
}

pub async fn get_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<UserProfileResponse, DbError> {

    let user = db::users::get_user_by_id(pool, user_id).await?;

    let info = UserInfo {
        id: user.id,
        username: user.username,
        email: user.email,
        role: user.role,
    };

    let menus = vec![];

    Ok(UserProfileResponse {
        user: info,
        menus,
    })
}


pub async fn list_users(
    pool: &PgPool,
) -> Result<Vec<UserListItem>, DbError> {

    let users = db::users::list_users(pool).await?;

    let res = users
        .into_iter()
        .map(|a| UserListItem {
            id: a.id,
            username: a.username,
            email: a.email,
            role: a.role,
        }).collect();

    Ok(res)
}

pub async fn create_user(
    pool: &PgPool,
    payload: CreateUserPayload,
) -> Result<User, DbError> {

    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| DbError::Internal("Password hashing failed".into()))?;

    db::users::create_user(pool, &payload, &password_hash).await
}


pub async fn update_user(
    pool: &PgPool,
    user_id: i32,
    payload: UpdateUserPayload,
) -> Result<User, DbError> {
    // 💡 如果传入了新密码，则进行哈希
    let password_hash = if let Some(pwd) = &payload.password {
        Some(
            hash(pwd, DEFAULT_COST)
                .map_err(|_| DbError::Internal("Password hashing failed".into()))?,
        )
    } else {
        None
    };

    db::users::update_user(pool, user_id, &payload, password_hash.as_deref()).await
}



pub async fn delete_user(
    pool: &PgPool,
    user_id: i32,
    current_user_id:i32,
) -> Result<(), DbError> {
    // 防止管理员误删自己
    if user_id == current_user_id {
        return Err(DbError::BadRequest("不能删除当前登录的账号".into()));
    }
    db::users::delete_user(pool, user_id).await
}


