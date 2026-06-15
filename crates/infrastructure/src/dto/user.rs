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


#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    // 使用 Option<String> 来实现部分更新：传了就用新值，没传就是 None
    pub username: Option<String>, 
}
