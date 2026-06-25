use sqlx::PgPool;
use bcrypt::verify;

use crate::db;
use crate::dto::auth::*;
use crate::errors::DbError;

pub async fn login(
    pool: &PgPool,
    payload: LoginRequest,
) -> Result<LoginResponse, DbError> {

    let user = db::query_user_for_login(pool, &payload.username).await?;

    let ok = verify(&payload.password, &user.password_hash)
        .map_err(|_| DbError::Internal("bcrypt error".into()))?;

    if !ok {
        return Err(DbError::Unauthorized);
    }

    let token = Claims::generate_token(user.id)
        .map_err(|_| DbError::TokenError)?;

    Ok(LoginResponse {
        token,
        username: user.username,
    })
}



