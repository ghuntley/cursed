//! SMTP client implementation

use crate::error::CursedError;

/// SMTP client
#[derive(Debug)]
pub struct SmtpClient {
    config: SmtpConfig,
    connected: bool,
}

impl SmtpClient {
    pub fn new(config: SmtpConfig) -> Self {
        Self {
            config,
            connected: false,
        }
    }
    
    pub fn connect(&mut self) -> Result<(), CursedError> {
        // Stub implementation
        self.connected = true;
        Ok(())
    }
    
    pub fn disconnect(&mut self) -> Result<(), CursedError> {
        // Stub implementation
        self.connected = false;
        Ok(())
    }
    
    pub fn send_email(&self, email: &EmailMessage) -> Result<(), CursedError> {
        // Stub implementation
        if !self.connected {
            return Err(CursedError::runtime_error("SMTP client not connected"));
        }
        println!("Sending email to: {}", email.to);
        Ok(())
    }
}

/// SMTP configuration
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
}

impl SmtpConfig {
    pub fn new(server: &str, port: u16, username: &str, password: &str) -> Self {
        Self {
            server: server.to_string(),
            port,
            username: username.to_string(),
            password: password.to_string(),
            use_tls: true,
        }
    }
}

/// Email message structure
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub html_body: Option<String>,
    pub attachments: Vec<EmailAttachment>,
}

impl EmailMessage {
    pub fn new(from: &str, to: &str, subject: &str, body: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
            html_body: None,
            attachments: Vec::new(),
        }
    }
    
    pub fn html_body(mut self, html: &str) -> Self {
        self.html_body = Some(html.to_string());
        self
    }
    
    pub fn attach_file(mut self, attachment: EmailAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }
}

/// Email attachment
#[derive(Debug, Clone)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

impl EmailAttachment {
    pub fn new(filename: &str, content_type: &str, data: Vec<u8>) -> Self {
        Self {
            filename: filename.to_string(),
            content_type: content_type.to_string(),
            data,
        }
    }
}
