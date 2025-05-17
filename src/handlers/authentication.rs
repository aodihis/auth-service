use crate::app_state::AppState;
use crate::error::api::ApiError;
use crate::extractors::payload_json::PayloadJson;
use crate::models::request::{Login, RegisterUser, ResendToken, Token};
use crate::models::response::SuccessResponse;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;
use validator::Validate;

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<RegisterUser>,
) -> Result<SuccessResponse<()>, ApiError> {
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

        return Err(ApiError::ValidationError {
            message: "Invalid input".to_string(),
            field_errors: errors_map,
        });
    }
    let user = state.services.user_service.create_user(payload).await?;

    let _ = state
        .services
        .auth_service
        .send_activation_token(&state.services.email_service, user.id)
        .await?;
    Ok(SuccessResponse{
        data: None,
        message: "User created".to_string(),
    })
}

pub async fn verify_user(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<Token>,
) ->  Result<SuccessResponse<()>, ApiError> {
    let token = payload.token;

    state.services.auth_service.verify_user(token).await?;

    Ok(SuccessResponse {
        data: None,
        message: "User verified".to_string(),
    })
}

pub async fn resend_token(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<ResendToken>,
) -> Result<SuccessResponse<()>, ApiError> {
    let user_id = payload.user_id;

    state
        .services
        .auth_service
        .resend_activation_token(&state.services.email_service, &user_id)
        .await?;
    Ok(SuccessResponse {
        data: None,
        message: "Token resent".to_string(),
    })

}

pub async fn login(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<Login>,
) -> impl IntoResponse {
    Json(json!({"success": true})).into_response()
}
