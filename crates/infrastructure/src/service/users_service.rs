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

    let user = db::get_user_by_id(pool, user_id).await?;

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


pub async fn list_users(
    pool: &PgPool,
) -> Result<Vec<UserListItem>, DbError> {

    let users = db::user::list_users(pool).await?;

    let result = users.into_iter().map(|a| UserListItem {
        id: a.id,
        username: a.username,
        email: a.email,
        role: a.role,
    }).collect();

    Ok(result)
}

pub async fn create_user(
    pool: &PgPool,
    mut payload: CreateUserPayload,
) -> Result<User, DbError> {

    payload.password = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| DbError::Internal("Password hashing failed".into()))?;

    db::create_user(pool, &payload).await
}


pub async fn update_user(
    pool: &PgPool,
    user_id: i32,
    mut payload: UpdateUserPayload,
) -> Result<User, DbError> {

    if let Some(pwd) = payload.password {
        payload.password = Some(
            bcrypt::hash(&pwd, DEFAULT_COST)
                .map_err(|_| DbError::Internal("hash failed".into()))?
        );
    }

    db::update_user(pool, user_id, &payload).await
}



pub async fn delete_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<(), DbError> {

    db::delete_user(pool, user_id).await
}


pub async fn get_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<UserProfileResponse, DbError> {

    let user = db::get_user_by_id(pool, user_id).await?;

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