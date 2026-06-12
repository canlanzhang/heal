
mod state;
use crate::state::AppState;

mod handlers;

mod routes;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use std::path::PathBuf;
use axum::{
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    BoxError,
};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //初始化 tracing 日志系统
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                //.unwrap_or_else(|_| EnvFilter::new("info")) 
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    tracing::info!(".env加载完成");

    let ports = Ports {
        http: 8080,
        https: 8443,
    };



    

    let pool = infrastructure::db::create_pool().await?;
    tracing::info!("数据库连接成功");

    let app_state = AppState {db_pool: pool};
    tracing::info!("state获取成功");

    let tls_config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("key.pem"),
    )
    .await?;
    tracing::info!("证书加载成功");

    let app =routes::get_router(app_state);
    tracing::info!("路由加载成功");


    tracing::info!("🚀 服务器启动中...");

    tokio::spawn(redirect_http_to_https(ports));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
    tracing::debug!("🔀 HTTP 监听地址 (重定向): https://{}",addr);
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await?;
    
    println!("Hello, world!");
    Ok(())
}


#[allow(dead_code)]
async fn redirect_http_to_https(ports: Ports) {
    fn make_https(uri: Uri, https_port: u16) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);
        parts.authority = Some(format!("localhost:{https_port}").parse()?);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |uri: Uri| async move {
        match make_https(uri, ports.https) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], ports.http));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("🔒 HTTPS 监听地址: http://{}",addr);
    let _ = axum::serve(listener, redirect.into_make_service()).await;
}