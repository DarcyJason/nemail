use std::sync::Arc;

use axum::http::{HeaderValue, Method};
use router::routes;
use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod handlers;
mod models;
mod router;

// 定义全局共享状态
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct AppState {
    pool: Pool<MySql>,
    config: config::Config,
}

#[tokio::main]
async fn main() {
    // 启动日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // 加载环境变量
    dotenvy::dotenv().ok();
    // 创建配置
    let config = config::Config::new();
    // 创建数据库连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create pool");
    // 配置跨域资源共享
    let cors = CorsLayer::new()
        .allow_origin(config.frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT]);
    // 绑定服务器IP地址
    let listener = tokio::net::TcpListener::bind(&config.backend_url)
        .await
        .expect("Failed to bind address");
    // 创建全局共享状态
    let shared_state = Arc::new(AppState { pool, config });
    // 创建路由
    let app = routes().with_state(shared_state.clone()).layer(cors);
    tracing::info!(
        "Server is running on http://{}",
        &shared_state.config.backend_url
    );
    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}
