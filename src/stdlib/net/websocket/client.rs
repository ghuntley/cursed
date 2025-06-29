//! WebSocket client functionality

use crate::error::CursedError;

/// WebSocket client
#[derive(Debug)]
pub struct WebSocketClient {
    url: String,
    state: ConnectionState,
}

impl WebSocketClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            state: ConnectionState::Disconnected,
        }
    }
    
    pub fn connect(&mut self) -> Result<(), CursedError> {
        // Stub implementation
        self.state = ConnectionState::Connected;
        Ok(())
    }
    
    pub fn disconnect(&mut self) -> Result<(), CursedError> {
        // Stub implementation
        self.state = ConnectionState::Disconnected;
        Ok(())
    }
    
    pub fn send_text(&self, _text: &str) -> Result<(), CursedError> {
        // Stub implementation
        Ok(())
    }
    
    pub fn send_binary(&self, _data: &[u8]) -> Result<(), CursedError> {
        // Stub implementation
        Ok(())
    }
}

/// WebSocket client builder
#[derive(Debug, Default)]
pub struct WebSocketClientBuilder {
    url: Option<String>,
    headers: Vec<(String, String)>,
}

impl WebSocketClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
    
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }
    
    pub fn build(self) -> Result<WebSocketClient, CursedError> {
        let url = self.url.ok_or_else(|| CursedError::runtime_error("URL is required"))?;
        Ok(WebSocketClient::new(&url))
    }
}

/// WebSocket connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
}
