use axum::{routing::get, Router};
use crate::state::AppState;

async fn health_check() -> &'static str {
    "OK"
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/user/{id}",get(handle_get_user))
        .with_state(state) // Axum 0.8 标准的状态注入方式
}


use axum::extract::State; // 确保引入了 State
use axum::{extract::{Extension, Path}, Json, response::IntoResponse};
use infrastructure::{PgPool, db}; // 引入底层的基础设施和连接池

// 具体的路由处理函数
pub async fn handle_get_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>, // 假设你把连接池放到了全局状态/中间件中
) -> impl IntoResponse {
    
    // 直接调用 infrastructure 层写好的查询接口
    match db::get_user_by_id(&state.db_pool, user_id).await {
        Ok(user) => {
            // 查询成功，返回 JSON 数据
            println!("找到用户: {:?}", user);
            // 这里可以将其转为前端需要的 JSON 返回
        }
        Err(e) => {
            // 查询失败，处理错误
            eprintln!("数据库查询失败: {:?}", e);
        }
    }
}
