use crate::config::Config;
use crate::error::email::EmailError;
use crate::error::email::EmailError::Other;
use crate::services::traits::EmailServiceBase;
use anyhow::anyhow;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::{Message, SmtpTransport, Transport};
use std::pin::Pin;
use std::sync::Arc;
use tracing::{error, info};

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
        let from = match format!(
            "{}<{}>",
            self.config.smtp.from_name, self.config.smtp.from_email
        )
        .parse()
        {
            Ok(from) => from,
            Err(err) => {
                error!("Invalid from configuration: {}", err);
                return Box::pin(async { Err(EmailError::InternalServerError) });
            }
        };
        let mut builder = Message::builder()
            .from(from)
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

        let email = builder.body(content).unwrap();

        // SMTP credentials
        let creds = Credentials::new(
            self.config.smtp.username.to_string(),
            self.config.smtp.password.to_string(),
        );

        // Create the SMTP transport
        let mut mailer_builder = SmtpTransport::relay(&self.config.smtp.host)
            .unwrap()
            .port(self.config.smtp.port) // e.g., "smtp.gmail.com"
            .credentials(creds);

        if self.config.smtp.tls == false {
            mailer_builder = mailer_builder.tls(Tls::None);
        }

        let mailer = mailer_builder.build();

        match mailer.send(&email) {
            Ok(_) => Box::pin(async move {
                info!("Successfully sent email to {}", to);
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
        Self { config }
    }
}
