use crate::handlers::authentication::{login, register_user, resend_token, verify_user};
use crate::services::authentication::Authentication as AuthenticationService;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

pub fn router(auth_service: Arc<AuthenticationService>) -> Router {

    Router::new()
        .route("/register", post(register_user))
        .route("/verify", post(verify_user))
        .route("/resend-token", post(resend_token))
        .route("/login", post(login))
        .with_state(auth_service)
}