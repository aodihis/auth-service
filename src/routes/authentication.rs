use crate::handlers::authentication::{
    check_status, login, register_user, resend_token, verify_user,
};
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/verify", post(verify_user))
        .route("/resend-token", post(resend_token))
        .route("/login", post(login))
        .route("/check_status", get(check_status))
        .with_state(state)
}
