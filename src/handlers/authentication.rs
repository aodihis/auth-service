use axum::Json;
use axum::response::IntoResponse;
use crate::models::request::RegisterUser;

pub async fn register_user(
    Json(payload): Json<RegisterUser>,
) -> impl IntoResponse {

}