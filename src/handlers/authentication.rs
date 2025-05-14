use crate::app_state::AppState;
use crate::error::api::ApiError;
use crate::error::authentication::AuthenticationError;
use crate::extractors::payload_json::PayloadJson;
use crate::models::request::{Login, RegisterUser, ResendToken, Token};
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde_json::json;
use std::sync::Arc;
use validator::Validate;

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<RegisterUser>,
) -> impl IntoResponse {
    if let Err(err) = payload.validate() {
        let mut errors_map = vec![];

        for (field, errors) in err.field_errors() {
            let messages: Vec<String> = errors
                .iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect();

            errors_map.push((field.to_string(), messages.join(", ")));
        }

        return ApiError::ValidationError {
            message: "Invalid input".to_string(),
            field_errors: errors_map,
        }
        .into_response();
    }
    let user = state.services.user_service.create_user(payload).await?;

    match state
        .services
        .auth_service
        .send_activation_token(&state.services.email_service, user.id)
        .await
    {
        Ok(_) => Json(json!({"success": true})).into_response(),
        Err(err) => convert_error(err).into_response(),
    }
}

pub async fn verify_user(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<Token>,
) -> impl IntoResponse {
    let token = payload.token;

    match state.services.auth_service.verify_user(token).await {
        Ok(_) => Json(json!({"success": true})).into_response(),
        Err(err) => convert_error(err).into_response(),
    }
}

pub async fn resend_token(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<ResendToken>,
) -> impl IntoResponse {
    let user_id = payload.user_id;

    match state
        .services
        .auth_service
        .resend_activation_token(&state.services.email_service, &user_id)
        .await
    {
        Ok(_) => Json(json!({"success": true})).into_response(),
        Err(err) => convert_error(err).into_response(),
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<Login>,
) -> impl IntoResponse {
    Json(json!({"success": true})).into_response()
}

fn convert_error(err: AuthenticationError) -> ApiError {
    match err {
        AuthenticationError::AccountAlreadyExists => {
            ApiError::Conflict("Account already exists".to_string())
        }
        AuthenticationError::InvalidInput(msg) => ApiError::ValidationError {
            message: msg,
            field_errors: vec![],
        },
        AuthenticationError::InternalServerError => {
            ApiError::InternalServerError("Internal server error".to_string())
        }
        AuthenticationError::InvalidToken => ApiError::BadRequest("Invalid token".to_string()),
    }
}
