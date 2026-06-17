
use serde::{Serialize, Deserialize};

use validator::Validate;
use crate::dto::common::MenuItem;

#[derive(Debug, Serialize)]
pub struct AdminListItem {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CreateAdminPayload {
    #[validate(length(min = 3, max= 16))]
    pub username: String,
    #[validate(email)]
    pub email: Option<String>,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize, validator::Validate)]
pub struct UpdateAdminPayload {
    // 允许局部更新：传了就改，没传就保持原样
    #[validate(length(min = 3, max = 16))]
    pub username: Option<String>,
    
    #[validate(email)]
    pub email: Option<String>,
    
    // 如果传了新密码，需要支持局部修改
    pub password: Option<String>,
    
    pub role: Option<String>,
}

#[derive(Serialize)]
pub struct AdminInfo {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Serialize)]
pub struct AdminProfileResponse  {
    pub admin: AdminInfo,
    pub menus: Vec<MenuItem>,
}

