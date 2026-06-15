use axum::{
    extract::{FromRequest,FromRequestParts},
    http::request::Parts,
    http::Request,
    body::Body,
};

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use chrono::{DateTime, Duration, Utc}; 
use jsonwebtoken::{encode, decode, 
    EncodingKey, DecodingKey, Header, Validation, Algorithm};
use validator::Validate;
use crate::errors::{DbError,AuthError};



#[derive(Serialize)]
pub struct ApiResponse<T> {
    code: u16,        // 状态码，如 200, 404, 500
    message: String,  // 提示信息
    data: Option<T>,          // 泛型字段，存放真正的数据（比如 User, Vec<Post> 等）
}

impl<T> ApiResponse<T> 
where
    T: Serialize, // 约束：T 必须能被序列化
{
    // 快捷构造函数：成功
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "Success".to_string(),
            data:Some(data),
        }
    }

    // 快捷构造函数：失败
    pub fn error(code: u16, message: String) -> Self {
        // 注意：这里 data 可能需要是 Option<T> 或者默认值，视具体设计而定
        // 简化版示例假设 data 依然需要传，或者结构体改为 data: Option<T>
        Self {
            code,
            message,
            data: None, // 失败时没有数据，传 None 即可
        }
    }
}



#[derive(Serialize)]
pub struct MenuItem {
    pub name: String,
    pub path: String,
    pub title: String,
    pub icon: String,
}