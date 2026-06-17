pub mod admin;
pub mod article;
pub mod user;
pub mod pool;

// ===== 对外导出 =====

pub use pool::create_pool;

pub use admin::{
    create_admin,
    update_admin,
    delete_admin,
    get_admin_by_id,
    get_admin_by_username,
    query_admin_for_login,

};
pub use article::*;
pub use user::{
    create_user,
    update_user,
    delete_user,
    get_user_by_id,
    get_user_by_username,
};