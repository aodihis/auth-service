use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;
use validator::Validate;
use crate::error::RegisterError;
use crate::models::request::RegisterUser;

pub async fn register_user(
    State(auth_service): State<Arc<crate::services::authentication::Authentication>>,
    Json(payload): Json<RegisterUser>,
) -> impl IntoResponse {

    if let Err(err) = payload.validate() {
        let mut errors_map = HashMap::new();

        for (field, errors) in err.field_errors() {
            let messages: Vec<String> = errors.iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect();

            errors_map.insert(field.to_string(), messages);
        }

        let error_json = json!({ "fields": errors_map }).to_string();

        return RegisterError::InvalidInput(error_json).into_response();
    }


    match auth_service.register(payload).await {
        Ok(_) => {
            Json(json!({"success": true})).into_response()
        }
        Err(err) => {
            err.into_response()
        }
    }

}