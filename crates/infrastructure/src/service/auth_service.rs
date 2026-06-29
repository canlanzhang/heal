use sqlx::PgPool;
use bcrypt::verify;

use crate::{AuthError, DbError, db};
use crate::dto::auth::*;
use crate::errors::ApiError;
pub async fn login(
    pool: &PgPool,
    payload: LoginRequest,
) -> Result<LoginResponse, ApiError> {

    // 1️⃣ 查用户（DbError → AuthError）
    let user = db::query_user_for_login(pool, &payload.username)
        .await
        .map_err(|e| match e {
            DbError::NotFound => ApiError::Auth(AuthError::WrongCredentials),
            _ => ApiError::Db(e)
        })?;

    // 2️⃣ 校验密码
    let is_valid  = verify(&payload.password, &user.password_hash)
        .map_err(|e| ApiError::Auth(AuthError::VerifyInternalError(e.to_string())))?;

    // 3️⃣ 密码错误
    if !is_valid {//Unauthorized
        return Err(ApiError::Auth(AuthError::WrongCredentials))?;
    }

    // 4️⃣ 生成 Token
    let token = Claims::generate_token(user.id)
        .map_err(|_| ApiError::Auth(AuthError::TokenCreation))?;

    Ok(LoginResponse {
        token,
        username: user.username,
    })
}



