use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::handlers::health::health_handler;
use crate::services::authentication::Authentication as AuthenticationService;

pub fn router(auth_service: Arc<AuthenticationService>) -> Router {

    Router::new()
        .route("/register", get(health_handler))
        .with_state(auth_service)
}