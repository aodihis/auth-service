use crate::handlers::authentication::register_user;
use crate::services::authentication::Authentication as AuthenticationService;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn router(auth_service: Arc<AuthenticationService>) -> Router {

    Router::new()
        .route("/register", get(register_user))
        .with_state(auth_service)
}