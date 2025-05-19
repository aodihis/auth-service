use crate::app_state::AppState;
use crate::error::api::ApiError;
use crate::extractors::payload_json::PayloadJson;
use crate::models::request::{Login, RegisterUser, ResendToken, Token};
use crate::models::response::SuccessResponse;
use axum::extract::State;
use std::sync::Arc;
use tower_cookies::cookie::time::Duration;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use validator::Validate;
use crate::error::authentication::AuthenticationError;
use crate::models::authenticate::LoginInfo;

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

    state
        .services
        .auth_service
        .send_activation_token(&state.services.email_service, user.id)
        .await?;
    Ok(SuccessResponse {
        data: None,
        message: "User created".to_string(),
    })
}

pub async fn verify_user(
    State(state): State<Arc<AppState>>,
    PayloadJson(payload): PayloadJson<Token>,
) -> Result<SuccessResponse<()>, ApiError> {
    let token = payload.token;

    state.services.auth_service.verify_email(token).await?;

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
    cookies: Cookies,
    PayloadJson(payload): PayloadJson<Login>,
) -> Result<SuccessResponse<()>, ApiError> {
    let identity = payload.identity;
    let password = payload.password;
    let user = state
        .services
        .user_service
        .get_user_by_email_or_username(&identity)
        .await?;

    let token = state.services.auth_service.login(user, password).await?;

    debug!("Create cookie for jwt_token");

    let secure = state.config.app.env != "dev".to_string();
    let cookie = Cookie::build(("jwt_token", token))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Strict)
        .max_age(Duration::seconds(state.config.jwt.expiration));

    debug!("Set cookie for jwt_token");
    cookies.add(cookie.into());
    Ok(SuccessResponse {
        message: "Login success".to_string(),
        data: None,
    })
}

pub async fn check_status(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
) -> Result<SuccessResponse<LoginInfo>, ApiError> {
    let token = match cookies.get("jwt_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return Ok(SuccessResponse {
                message: "".to_string(),
                data: Some(LoginInfo {
                    user: None,
                    logged: false
                })
            });
        }
    };

    let username = match state.services.auth_service.validate_token(token) {
        Ok(username) => username,
        Err(err) => {
            return match err {
                AuthenticationError::InvalidToken => {
                    Ok(SuccessResponse {
                        message: "".to_string(),
                        data: Some(LoginInfo {
                            user: None,
                            logged: false
                        })
                    })
                },
                _ => {
                    Err(err.into())
                }
            }
        }
    };
    let user = state.services.user_service.get_user_by_email_or_username(&username).await?;

    Ok(
        SuccessResponse {
            message: "".to_string(),
            data: Some(LoginInfo {
                user: Some(user),
                logged: true
            }),
        }
    )
}
