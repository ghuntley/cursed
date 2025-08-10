//! HTTP authentication functionality

/// HTTP authentication types
#[derive(Debug, Clone)]
pub enum HttpAuth {
    Basic(BasicAuth),
    Bearer(BearerAuth),
    OAuth2(OAuth2Auth),
}

/// Basic authentication
#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
    
    pub fn encode(&self) -> String {
        use std::collections::HashMap;
        // Stub implementation - would use base64 encoding
        format!("{}:{}", self.username, self.password)
    }
}

/// Bearer token authentication
#[derive(Debug, Clone)]
pub struct BearerAuth {
    pub token: String,
}

impl BearerAuth {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}

/// OAuth2 authentication
#[derive(Debug, Clone)]
pub struct OAuth2Auth {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
}

impl OAuth2Auth {
    pub fn new(access_token: &str, token_type: &str) -> Self {
        Self {
            access_token: access_token.to_string(),
            token_type: token_type.to_string(),
            expires_in: None,
        }
    }
    
    pub fn with_expiry(mut self, expires_in: u64) -> Self {
        self.expires_in = Some(expires_in);
        self
    }
}
