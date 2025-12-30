use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::sync::Arc;
use tokio::fs::read_to_string;

/// Configuration for SMTP email sending.
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub from_address: String,
    pub frontend_url: String,
}

/// Service responsible for sending emails.
#[derive(Clone)]
pub struct EmailService {
    config: Arc<EmailConfig>,
    verify_template_path: String,
    reset_password_path: String,
}

impl EmailService {
    /// Creates a new `EmailService` with shared config and template path.
    pub fn new(config: Arc<EmailConfig>, verify_template_path: String, reset_password_path: String) -> Self {
        log::info!("Initializing SMTP Service");
        Self { config, verify_template_path, reset_password_path }
    }

    /// Sends an account verification email with a verification code link.
    ///
    /// Reads the HTML template asynchronously, replaces the `{{link}}` placeholder,
    /// and sends the email via SMTP.
    pub async fn send_verification_code(
        &self,
        to_email: &str,
        verification_code: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let template = read_to_string(&self.verify_template_path).await?;
        let link = format!("{}verify/{}", self.config.frontend_url, verification_code);
        let html_body = template.replace("{{link}}", &link);

        let email = Message::builder()
            .from(self.config.from_address.parse()?)
            .to(to_email.parse()?)
            .subject("Account registration code")
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let creds = Credentials::new(self.config.smtp_user.clone(), self.config.smtp_pass.clone());

        let mailer = SmtpTransport::relay(&self.config.smtp_server)?
            .credentials(creds)
            .port(self.config.smtp_port)
            .build();

        mailer.send(&email)?;
        Ok(())
    }

    pub async fn send_password_reset_code(
        &self,
        to_email: &str,
        reset_code: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let template = read_to_string(&self.reset_password_path).await?;
        let html_body = template.replace("{{code}}", &reset_code);

        let email = Message::builder()
            .from(self.config.from_address.parse()?)
            .to(to_email.parse()?)
            .subject("Password Reset Request")
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let creds = Credentials::new(self.config.smtp_user.clone(), self.config.smtp_pass.clone());
        let mailer = SmtpTransport::relay(&self.config.smtp_server)?
            .credentials(creds)
            .port(self.config.smtp_port)
            .build();

        mailer.send(&email)?;
        Ok(())
    }
}
