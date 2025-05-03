use crate::error::email::EmailError;

use std::future::Future;
use std::pin::Pin;

pub trait EmailServiceBase: Send + Sync {
    fn send_email(
        &self,
        to: String,
        cc: Vec<String>,
        bcc: Vec<String>,
        subject: String,
        content: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailError>> + Send>>;
}
