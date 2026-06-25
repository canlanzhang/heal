
use serde::{Serialize, Deserialize};

use validator::Validate;
use crate::dto::menu::MenuItem;
use crate::dto::utils::empty_string_as_none;

#[derive(Debug, Serialize)]
pub struct UserListItem {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateUserPayload {
    #[validate(length(min = 3, max= 16))]
    pub username: String,
    #[validate(email)]
    pub email: Option<String>,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateUserPayload {

    #[serde(default, deserialize_with = "empty_string_as_none")]
    #[validate(length(min = 3, max = 16))]
    pub username: Option<String>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub email: Option<String>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub password: Option<String>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub role: Option<String>,
}




#[derive(Serialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
}

#[derive(Serialize)]
pub struct UserProfileResponse  {
    pub user: UserInfo,
    pub menus: Vec<MenuItem>,
}

