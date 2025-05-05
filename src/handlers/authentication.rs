use crate::error::api::ApiError;
use crate::error::authentication::AuthenticationError;
use crate::models::request::RegisterUser;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;
use validator::Validate;
use crate::extractors::payload_json::PayloadJson;

pub async fn register_user(
    State(auth_service): State<Arc<crate::services::authentication::Authentication>>,
    PayloadJson(payload): PayloadJson<RegisterUser>,
) -> impl IntoResponse {

    if let Err(err) = payload.validate() {
        let mut errors_map = vec![];

        for (field, errors) in err.field_errors() {
            let messages: Vec<String> = errors.iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect();

            errors_map.push((field.to_string(), messages.join(", ")));
        }

        return ApiError::ValidationError {
            message: "Invalid input".to_string(),
            field_errors: errors_map,
        }.into_response();
    }


    match auth_service.register(payload).await {
        Ok(_) => {
            Json(json!({"success": true})).into_response()
        }
        Err(err) => {
            match err {
                AuthenticationError::AccountAlreadyExists => {ApiError::Conflict("Account already exists".to_string())}
                AuthenticationError::InvalidInput(msg) => { ApiError::ValidationError { message: msg, field_errors: vec![] }}
                AuthenticationError::InternalServerError => { ApiError::InternalServerError("Internal server error".to_string()) }
            }.into_response()
        }
    }

}