use axum::{
    Json, 
    http::StatusCode,
    extract::{Path, State}, 
    response::IntoResponse
};

use crate::state::AppState; 
use infrastructure::{
    db,
    models::{User}
}; // 引入底层的基础设施和连接池

use serde::{Serialize, Deserialize};
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


pub async fn handle_get_user(
    Path(user_id): Path<i32>,
    State(state): State<AppState>,
) ->  (StatusCode, Json<ApiResponse<User>>) {
    match db::get_user_by_id(&state.db_pool, user_id).await {
        Ok(user) => {
            // 查询成功，返回 JSON 数据
            tracing::debug!("找到用户: {:?}", user);
            (StatusCode::OK, Json(ApiResponse::success(user)))

        }
        Err(e) => {
            // 查询失败，处理错误            
            tracing::debug!("数据库查询失败: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(ApiResponse::error(500, "服务器内部错误".to_string()))
            )

        }
    }
}