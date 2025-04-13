use crate::handlers::health::health_handler;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new().route("/check", get(health_handler))
}

