use crate::AppState;
use crate::Arc;
use crate::handlers::health_check_handler;
use axum::{Router, routing::get};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/health_check", get(health_check_handler::health_check))
}
