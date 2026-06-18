
use crate::dto::common::MenuItem;

pub fn build_default_menus() -> Vec<MenuItem> {
    vec![
        MenuItem {
            name: "home".into(),
            path: "/home".into(),
            title: "首页".into(),
            icon: "Home".into(),
        },
        MenuItem {
            name: "admin".into(),
            path: "/admin".into(),
            title: "管理员管理".into(),
            icon: "Admin".into(),
        },
        MenuItem {
            name: "article".into(),
            path: "/article".into(),
            title: "内容管理".into(),
            icon: "Article".into(),
        },
        MenuItem {
            name: "user".into(),
            path: "/user".into(),
            title: "用户管理".into(),
            icon: "User".into(),
        },
    ]
}