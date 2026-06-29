use sqlx::PgPool;
use bcrypt::verify;

use crate::{AuthError, DbError, db};
use crate::dto::auth::*;
use crate::errors::ApiError;
pub async fn login(
    pool: &PgPool,
    payload: LoginRequest,
) -> Result<LoginResponse, AuthError> {

    // 1️ 查用户（DbError → AuthError）
    let user = db::query_user_for_login(pool, &payload.username)
        .await
        .map_err(|e| match e {
            DbError::NotFound => AuthError::WrongCredentials,
            _ => AuthError::InternalDbError(e)
        })?;

    // 2️ 校验密码
    let is_valid  = verify(&payload.password, &user.password_hash)
        .map_err(|e| AuthError::VerifyInternalError(e.to_string()))?;

    // 3️ 密码错误
    if !is_valid {//Unauthorized
        return Err(AuthError::WrongCredentials)?;
    }

    // 4️ 生成 Token
    let token = Claims::generate_token(user.id)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(LoginResponse {
        token,
        username: user.username,
    })
}



