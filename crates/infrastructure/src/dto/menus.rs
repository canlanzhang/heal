use serde::{ Serialize,Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateMenuPayload {
    pub name: String,
    pub path: String,
    pub title: String,
    pub icon: Option<String>,
    pub role: Option<String>,
    pub parent_id: Option<i32>,
    pub sort: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMenuPayload {
    pub name: Option<String>,
    pub path: Option<String>,
    pub title: Option<String>,
    pub icon: Option<String>,
    pub role: Option<String>,
    pub parent_id: Option<i32>,
    pub sort: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct MenuItem {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub title: String,
    pub icon: Option<String>,
}