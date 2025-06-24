use crate::error::Error;
/// SMTP client implementation for email sending

use std::time::Duration;
use crate::stdlib::net::error::{NetError, NetResult, protocol_error};
use crate::stdlib::net::socket::TcpSocket;
use crate::stdlib::net::protocols::{ProtocolError, ProtocolResult};

/// SMTP client configuration
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub use_tls: bool,
    pub timeout: Duration,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            server: "localhost".to_string(),
            port: 25,
            username: None,
            password: None,
            use_tls: false,
            timeout: Duration::from_secs(30),
        }
    }
}

/// Email message
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub html_body: Option<String>,
    pub headers: std::collections::HashMap<String, String>,
}

impl EmailMessage {
    pub fn new(from: String, to: Vec<String>, subject: String, body: String) -> Self {
        Self {
            from,
            to,
            cc: Vec::new(),
            bcc: Vec::new(),
            subject,
            body,
            html_body: None,
            headers: std::collections::HashMap::new(),
        }
    }
    
    pub fn cc(mut self, addresses: Vec<String>) -> Self {
        self.cc = addresses;
        self
    }
    
    pub fn bcc(mut self, addresses: Vec<String>) -> Self {
        self.bcc = addresses;
        self
    }
    
    pub fn html_body(mut self, html: String) -> Self {
        self.html_body = Some(html);
        self
    }
    
    pub fn header(mut self, name: String, value: String) -> Self {
        self.headers.insert(name, value);
        self
    }
}

/// SMTP client
#[derive(Debug)]
pub struct SmtpClient {
    config: SmtpConfig,
    socket: Option<TcpSocket>,
}

impl SmtpClient {
    /// Create new SMTP client
    pub fn new(config: SmtpConfig) -> Self {
        Self {
            config,
            socket: None,
        }
    }
    
    /// Connect to SMTP server
    pub fn connect(&mut self) -> ProtocolResult<()> {
        let addr = format!("{}:{}", self.config.server, self.config.port);
        let socket = TcpSocket::connect_timeout(&addr, self.config.timeout)
            .map_err(|e| ProtocolError::Connection(e.to_string()))?;
        
        self.socket = Some(socket);
        
        // Read greeting
        let greeting = self.read_response()?;
        if !greeting.starts_with("220") {
            return Err(ProtocolError::Protocol(format!("Invalid greeting: {}", greeting)));
        }
        
        // Send EHLO
        self.send_command("EHLO localhost")?;
        let response = self.read_response()?;
        if !response.starts_with("250") {
            return Err(ProtocolError::Protocol(format!("EHLO failed: {}", response)));
        }
        
        // Authenticate if credentials provided
        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            self.authenticate(username, password)?;
        }
        
        Ok(())
    }
    
    /// Send email message
    pub fn send_message(&mut self, message: &EmailMessage) -> ProtocolResult<()> {
        if self.socket.is_none() {
            self.connect()?;
        }
        
        // MAIL FROM
        self.send_command(&format!("MAIL FROM:<{}>", message.from))?;
        let response = self.read_response()?;
        if !response.starts_with("250") {
            return Err(ProtocolError::Protocol(format!("MAIL FROM failed: {}", response)));
        }
        
        // RCPT TO for all recipients
        let all_recipients: Vec<String> = message.to.iter()
            .chain(message.cc.iter())
            .chain(message.bcc.iter())
            .cloned()
            .collect();
        
        for recipient in &all_recipients {
            self.send_command(&format!("RCPT TO:<{}>", recipient))?;
            let response = self.read_response()?;
            if !response.starts_with("250") {
                return Err(ProtocolError::Protocol(format!("RCPT TO failed for {}: {}", recipient, response)));
            }
        }
        
        // DATA
        self.send_command("DATA")?;
        let response = self.read_response()?;
        if !response.starts_with("354") {
            return Err(ProtocolError::Protocol(format!("DATA failed: {}", response)));
        }
        
        // Send message data
        let message_data = self.format_message(message);
        self.send_data(&message_data)?;
        self.send_command(".")?; // End of data
        
        let response = self.read_response()?;
        if !response.starts_with("250") {
            return Err(ProtocolError::Protocol(format!("Message send failed: {}", response)));
        }
        
        Ok(())
    }
    
    /// Disconnect from server
    pub fn quit(&mut self) -> ProtocolResult<()> {
        if let Some(_) = &self.socket {
            self.send_command("QUIT")?;
            let _ = self.read_response(); // Don't care about response
            self.socket = None;
        }
        Ok(())
    }
    
    fn authenticate(&mut self, username: &str, password: &str) -> ProtocolResult<()> {
        // Simple AUTH LOGIN implementation
        self.send_command("AUTH LOGIN")?;
        let response = self.read_response()?;
        if !response.starts_with("334") {
            return Err(ProtocolError::Authentication("AUTH LOGIN not supported".to_string()));
        }
        
        // Send base64 encoded username
        let username_b64 = base64_encode(username.as_bytes());
        self.send_command(&username_b64)?;
        let response = self.read_response()?;
        if !response.starts_with("334") {
            return Err(ProtocolError::Authentication("Username rejected".to_string()));
        }
        
        // Send base64 encoded password
        let password_b64 = base64_encode(password.as_bytes());
        self.send_command(&password_b64)?;
        let response = self.read_response()?;
        if !response.starts_with("235") {
            return Err(ProtocolError::Authentication("Authentication failed".to_string()));
        }
        
        Ok(())
    }
    
    fn send_command(&mut self, command: &str) -> ProtocolResult<()> {
        if let Some(ref socket) = self.socket {
            let command_line = format!("{}\r\n", command);
            socket.write_string(&command_line)
                .map_err(|e| ProtocolError::Connection(e.to_string()))?;
            Ok(())
        } else {
            Err(ProtocolError::Connection("Not connected".to_string()))
        }
    }
    
    fn send_data(&mut self, data: &str) -> ProtocolResult<()> {
        if let Some(ref socket) = self.socket {
            socket.write_string(data)
                .map_err(|e| ProtocolError::Connection(e.to_string()))?;
            Ok(())
        } else {
            Err(ProtocolError::Connection("Not connected".to_string()))
        }
    }
    
    fn read_response(&mut self) -> ProtocolResult<String> {
        if let Some(ref socket) = self.socket {
            socket.read_line()
                .map_err(|e| ProtocolError::Connection(e.to_string()))
        } else {
            Err(ProtocolError::Connection("Not connected".to_string()))
        }
    }
    
    fn format_message(&self, message: &EmailMessage) -> String {
        let mut msg = String::new();
        
        // Headers
        msg.push_str(&format!("From: {}\r\n", message.from));
        msg.push_str(&format!("To: {}\r\n", message.to.join(", ")));
        
        if !message.cc.is_empty() {
            msg.push_str(&format!("Cc: {}\r\n", message.cc.join(", ")));
        }
        
        msg.push_str(&format!("Subject: {}\r\n", message.subject));
        msg.push_str("MIME-Version: 1.0\r\n");
        
        // Custom headers
        for (name, value) in &message.headers {
            msg.push_str(&format!("{}: {}\r\n", name, value));
        }
        
        // Content type and body
        if message.html_body.is_some() {
            msg.push_str("Content-Type: multipart/alternative; boundary=\"boundary123\"\r\n");
            msg.push_str("\r\n");
            
            // Plain text part
            msg.push_str("--boundary123\r\n");
            msg.push_str("Content-Type: text/plain; charset=utf-8\r\n");
            msg.push_str("\r\n");
            msg.push_str(&message.body);
            msg.push_str("\r\n\r\n");
            
            // HTML part
            if let Some(ref html) = message.html_body {
                msg.push_str("--boundary123\r\n");
                msg.push_str("Content-Type: text/html; charset=utf-8\r\n");
                msg.push_str("\r\n");
                msg.push_str(html);
                msg.push_str("\r\n\r\n");
            }
            
            msg.push_str("--boundary123--\r\n");
        } else {
            msg.push_str("Content-Type: text/plain; charset=utf-8\r\n");
            msg.push_str("\r\n");
            msg.push_str(&message.body);
        }
        
        msg.push_str("\r\n");
        msg
    }
}

impl Drop for SmtpClient {
    fn drop(&mut self) {
        let _ = self.quit();
    }
}

// Helper function for base64 encoding
fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in input.chunks(3) {
        let b1 = chunk[0];
        let b2 = chunk.get(1).copied().unwrap_or(0);
        let b3 = chunk.get(2).copied().unwrap_or(0);
        
        result.push(CHARS[(b1 >> 2) as usize] as char);
        result.push(CHARS[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[(((b2 & 0x0f) << 2) | (b3 >> 6)) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(b3 & 0x3f) as usize] as char } else { '=' });
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smtp_config_default() {
        let config = SmtpConfig::default();
        assert_eq!(config.server, "localhost");
        assert_eq!(config.port, 25);
        assert!(!config.use_tls);
    }

    #[test]
    fn test_email_message_creation() {
        let message = EmailMessage::new(
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test Subject".to_string(),
            "Test Body".to_string(),
        );
        
        assert_eq!(message.from, "sender@example.com");
        assert_eq!(message.to.len(), 1);
        assert_eq!(message.subject, "Test Subject");
        assert_eq!(message.body, "Test Body");
    }

    #[test]
    fn test_email_message_builder() {
        let message = EmailMessage::new(
            "sender@example.com".to_string(),
            vec!["recipient@example.com".to_string()],
            "Test".to_string(),
            "Body".to_string(),
        )
        .cc(vec!["cc@example.com".to_string()])
        .bcc(vec!["bcc@example.com".to_string()])
        .html_body("<h1>HTML Body</h1>".to_string())
        .header("X-Custom".to_string(), "Custom Value".to_string());
        
        assert_eq!(message.cc.len(), 1);
        assert_eq!(message.bcc.len(), 1);
        assert!(message.html_body.is_some());
        assert!(message.headers.contains_key("X-Custom"));
    }

    #[test]
    fn test_base64_encoding() {
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
    }

    #[test]
    fn test_smtp_client_creation() {
        let config = SmtpConfig {
            server: "smtp.example.com".to_string(),
            port: 587,
            use_tls: true,
            ..Default::default()
        };
        
        let client = SmtpClient::new(config);
        assert_eq!(client.config.server, "smtp.example.com");
        assert_eq!(client.config.port, 587);
        assert!(client.config.use_tls);
    }
}
