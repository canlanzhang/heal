use axum::{
    extract::{FromRequest},
    http::Request,
    body::Body,
};
use serde::de::DeserializeOwned;
use validator::Validate;
use crate::errors::{ApiError};

// 1. 定义一个泛型结构体，包裹 Axum 原生的 Json
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

// 2. 为其实现 FromRequest（让它成为 Axum 的参数提取器）

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate, // 核心约束：T 必须可以反序列化，且实现了 validator::Validate
    S: Send + Sync,
{
    type Rejection = ApiError; // 如果校验失败，直接抛出我们第一步写好的错误

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        // 先利用 Axum 原生的 Json 提取器解析出 T
        let axum::Json(value) = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|e| crate::errors::DbError::BadRequest(e.to_string()))?;

        // 🛠️ 核心防御：触发 validator 的校验逻辑
        value.validate().map_err(|e| {
            // 将详细的字段错误原因转为字符串丢给自定义错误
            crate::errors::DbError::ValidationError(format!("参数格式校验失败: {}", e))
        })?;

        Ok(ValidatedJson(value))
    }
}
