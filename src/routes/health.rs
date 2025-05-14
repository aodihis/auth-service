use crate::handlers::health::health_handler;
use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new().route("/check", get(health_handler))
}
