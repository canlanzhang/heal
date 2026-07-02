use axum::{
    http::{Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::trace::TraceId;

pub async fn trace_middleware(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    // 1. 生成 trace_id
    let trace_id = Uuid::new_v4().to_string();

    // 2. 注入 request extensions
    req.extensions_mut()
        .insert(TraceId(trace_id.clone()));

    // 3. 执行 handler
    let mut res = next.run(req).await;

    // 4. 返回 header
    res.headers_mut().insert(
        "x-trace-id",
        trace_id.parse().unwrap(),
    );

    res
}