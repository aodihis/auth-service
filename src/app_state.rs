use crate::services::authentication::Authentication;
use crate::services::email::EmailService;
use crate::services::users::Users;

pub struct AppState {
    pub(crate) services: Services
}

pub struct Services {
    pub(crate) auth_service: Authentication,
    pub(crate) email_service: EmailService,
    pub(crate) user_service: Users
}