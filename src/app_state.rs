use crate::config::Config;
use crate::services::authentications::Authentication;
use crate::services::email::EmailService;
use crate::services::users::Users;
use std::sync::Arc;

pub struct AppState {
    pub services: Services,
    pub config: Arc<Config>,
}

pub struct Services {
    pub(crate) auth_service: Authentication,
    pub(crate) email_service: EmailService,
    pub(crate) user_service: Users,
}
