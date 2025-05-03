use std::pin::Pin;
use crate::config::Config;
use crate::error::email::EmailError;
use crate::error::email::EmailError::Other;
use crate::services::traits::EmailServiceBase;
use lettre::message::header::{ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::sync::Arc;
use anyhow::anyhow;
use tracing::error;

pub struct EmailService {
    pub config: Arc<Config>,
}

impl EmailServiceBase for EmailService {
    fn send_email(
        &self,
        to: String,
        cc: Vec<String>,
        bcc: Vec<String>,
        subject: String,
        content: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailError>> + Send>> {
        let mut builder = Message::builder()
            .from(self.config.smtp.from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML);

        // Add CC recipients
        for cc_email in &cc {
            builder = builder.cc(cc_email.parse().unwrap());
        }

        // Add BCC recipients
        for bcc_email in &bcc {
            builder = builder.bcc(bcc_email.parse().unwrap());
        }

        let email = builder
            .body(content)
            .unwrap();

        // SMTP credentials
        let creds = Credentials::new(self.config.smtp.username.to_string(), self.config.smtp.password.to_string());

        // Create the SMTP transport
        let mailer = SmtpTransport::relay(&self.config.smtp.host).unwrap() // e.g., "smtp.gmail.com"
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => Box::pin(async move {
                Ok(())
            }),
            Err(e) => Box::pin(async move {
                error!("Failed to send email: {:?}", e);
                Err(Other(anyhow!(e.to_string())))
            }),
        }

    }
}

impl EmailService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
        }
    }

}