/// WebSocket client implementation

use std::sync::{Arc, Mutex};
use crate::stdlib::net::error::{NetError, NetResult, websocket_error};
use crate::stdlib::net::socket::TcpSocket;
use crate::stdlib::net::websocket::{WebSocketFrame, WebSocketMessage, WebSocketConfig, ConnectionState, CloseCode};

/// WebSocket client
#[derive(Debug)]
pub struct WebSocketClient {
    socket: Arc<Mutex<Option<TcpSocket>>>,
    state: Arc<Mutex<ConnectionState>>,
    config: WebSocketConfig,
    url: String,
}

impl WebSocketClient {
    /// Connect to a WebSocket server
    pub fn connect(url: &str) -> NetResult<Self> {
        Self::builder().connect(url)
    }
    
    /// Create a builder for configuring the client
    pub fn builder() -> WebSocketClientBuilder {
        WebSocketClientBuilder::new()
    }
    
    /// Send a text message
    pub fn send_text(&self, text: &str) -> NetResult<()> {
        let message = WebSocketMessage::text(text.to_string());
        self.send_message(&message)
    }
    
    /// Send a binary message
    pub fn send_binary(&self, data: Vec<u8>) -> NetResult<()> {
        let message = WebSocketMessage::binary(data);
        self.send_message(&message)
    }
    
    /// Send a WebSocket message
    pub fn send_message(&self, message: &WebSocketMessage) -> NetResult<()> {
        let state = *self.state.lock().unwrap();
        if state != ConnectionState::Open {
            return Err(websocket_error("WebSocket connection not open", None, Some(&self.url)));
        }
        
        let frame = message.to_frame()?;
        self.send_frame(&frame)
    }
    
    /// Receive a message
    pub fn receive(&self) -> NetResult<Option<WebSocketMessage>> {
        let state = *self.state.lock().unwrap();
        if state != ConnectionState::Open {
            return Ok(None);
        }
        
        let frame = self.receive_frame()?;
        Ok(Some(WebSocketMessage::from_frame(frame)?))
    }
    
    /// Close the WebSocket connection
    pub fn close(&self) -> NetResult<()> {
        self.close_with_code(CloseCode::NORMAL, "")
    }
    
    /// Close with specific code and reason
    pub fn close_with_code(&self, code: CloseCode, reason: &str) -> NetResult<()> {
        *self.state.lock().unwrap() = ConnectionState::Closing;
        
        // Send close frame
        let close_frame = WebSocketFrame::close(code, reason);
        self.send_frame(&close_frame)?;
        
        *self.state.lock().unwrap() = ConnectionState::Closed;
        
        // Close underlying socket
        let mut socket_guard = self.socket.lock().unwrap();
        if let Some(socket) = socket_guard.take() {
            socket.close()?;
        }
        
        Ok(())
    }
    
    /// Get current connection state
    pub fn state(&self) -> ConnectionState {
        *self.state.lock().unwrap()
    }
    
    /// Check if connection is open
    pub fn is_open(&self) -> bool {
        self.state() == ConnectionState::Open
    }
    
    /// Send a ping frame
    pub fn ping(&self, data: &[u8]) -> NetResult<()> {
        let ping_frame = WebSocketFrame::ping(data.to_vec());
        self.send_frame(&ping_frame)
    }
    
    /// Send a pong frame
    pub fn pong(&self, data: &[u8]) -> NetResult<()> {
        let pong_frame = WebSocketFrame::pong(data.to_vec());
        self.send_frame(&pong_frame)
    }
    
    fn send_frame(&self, frame: &WebSocketFrame) -> NetResult<()> {
        let socket_guard = self.socket.lock().unwrap();
        if let Some(ref socket) = *socket_guard {
            let frame_bytes = frame.to_bytes()?;
            socket.write_all(&frame_bytes)?;
            Ok(())
        } else {
            Err(websocket_error("No socket connection", None, Some(&self.url)))
        }
    }
    
    fn receive_frame(&self) -> NetResult<WebSocketFrame> {
        let socket_guard = self.socket.lock().unwrap();
        if let Some(ref socket) = *socket_guard {
            WebSocketFrame::from_socket(socket)
        } else {
            Err(websocket_error("No socket connection", None, Some(&self.url)))
        }
    }
    
    fn perform_handshake(&self, socket: &TcpSocket, url: &str) -> NetResult<()> {
        // Parse URL
        let (host, port, path) = self.parse_websocket_url(url)?;
        
        // Generate WebSocket key
        let key = self.generate_websocket_key();
        
        // Send HTTP upgrade request
        let request = format!(
            "GET {} HTTP/1.1\r\n\
             Host: {}:{}\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Key: {}\r\n\
             Sec-WebSocket-Version: 13\r\n\
             \r\n",
            path, host, port, key
        );
        
        socket.write_string(&request)?;
        
        // Read and validate response
        self.validate_handshake_response(socket, &key)?;
        
        Ok(())
    }
    
    fn parse_websocket_url(&self, url: &str) -> NetResult<(String, u16, String)> {
        if !url.starts_with("ws://") && !url.starts_with("wss://") {
            return Err(websocket_error(&format!("Invalid WebSocket URL: {}", url), None, Some(url)));
        }
        
        let is_secure = url.starts_with("wss://");
        let without_scheme = if is_secure {
            &url[6..] // Remove "wss://"
        } else {
            &url[5..] // Remove "ws://"
        };
        
        let (host_port, path) = if let Some(slash_pos) = without_scheme.find('/') {
            (&without_scheme[..slash_pos], &without_scheme[slash_pos..])
        } else {
            (without_scheme, "/")
        };
        
        let (host, port) = if let Some(colon_pos) = host_port.rfind(':') {
            let host = &host_port[..colon_pos];
            let port_str = &host_port[colon_pos + 1..];
            let port = port_str.parse::<u16>()
                .map_err(|_| websocket_error(&format!("Invalid port: {}", port_str), None, Some(url)))?;
            (host.to_string(), port)
        } else {
            (host_port.to_string(), if is_secure { 443 } else { 80 })
        };
        
        Ok((host, port, path.to_string()))
    }
    
    fn generate_websocket_key(&self) -> String {
        // Generate a random 16-byte key and base64 encode it
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let key_data = format!("cursed-ws-{}", timestamp);
        base64_encode(key_data.as_bytes())
    }
    
    fn validate_handshake_response(&self, socket: &TcpSocket, key: &str) -> NetResult<()> {
        // Read status line
        let status_line = socket.read_line()?;
        if !status_line.contains("101") {
            return Err(websocket_error("WebSocket handshake failed", None, Some(&self.url)));
        }
        
        // Read headers
        let mut upgrade_found = false;
        let mut connection_found = false;
        let mut accept_valid = false;
        
        loop {
            let header_line = socket.read_line()?;
            if header_line.trim().is_empty() {
                break;
            }
            
            if let Some(colon_pos) = header_line.find(':') {
                let name = header_line[..colon_pos].trim().to_lowercase();
                let value = header_line[colon_pos + 1..].trim();
                
                match name.as_str() {
                    "upgrade" => {
                        if value.to_lowercase() == "websocket" {
                            upgrade_found = true;
                        }
                    },
                    "connection" => {
                        if value.to_lowercase().contains("upgrade") {
                            connection_found = true;
                        }
                    },
                    "sec-websocket-accept" => {
                        let expected_accept = self.calculate_websocket_accept(key);
                        if value == expected_accept {
                            accept_valid = true;
                        }
                    },
                    _ => {}
                }
            }
        }
        
        if !upgrade_found || !connection_found || !accept_valid {
            return Err(websocket_error("Invalid WebSocket handshake response", None, Some(&self.url)));
        }
        
        Ok(())
    }
    
    fn calculate_websocket_accept(&self, key: &str) -> String {
        // SHA-1 hash of key + magic string, then base64 encode
        let magic = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
        let combined = format!("{}{}", key, magic);
        let hash = sha1_hash(combined.as_bytes());
        base64_encode(&hash)
    }
}

/// WebSocket client builder
#[derive(Debug)]
pub struct WebSocketClientBuilder {
    config: WebSocketConfig,
}

impl WebSocketClientBuilder {
    pub fn new() -> Self {
        Self {
            config: WebSocketConfig::default(),
        }
    }
    
    pub fn config(mut self, config: WebSocketConfig) -> Self {
        self.config = config;
        self
    }
    
    pub fn connect(self, url: &str) -> NetResult<WebSocketClient> {
        let mut client = WebSocketClient {
            socket: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(ConnectionState::Connecting)),
            config: self.config,
            url: url.to_string(),
        };
        
        // Parse URL and connect
        let (host, port, _path) = client.parse_websocket_url(url)?;
        let socket = TcpSocket::connect(&format!("{}:{}", host, port))?;
        
        // Perform WebSocket handshake
        client.perform_handshake(&socket, url)?;
        
        // Store socket and update state
        *client.socket.lock().unwrap() = Some(socket);
        *client.state.lock().unwrap() = ConnectionState::Open;
        
        Ok(client)
    }
}

impl Default for WebSocketClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions (simplified implementations)
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

fn sha1_hash(_input: &[u8]) -> [u8; 20] {
    // Simplified SHA-1 implementation (placeholder)
    [0u8; 20]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_client_builder() {
        let builder = WebSocketClientBuilder::new();
        assert!(builder.config.max_message_size > 0);
    }

    #[test]
    fn test_url_parsing() {
        let client = WebSocketClient {
            socket: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(ConnectionState::Closed)),
            config: WebSocketConfig::default(),
            url: String::new(),
        };
        
        let (host, port, path) = client.parse_websocket_url("ws://example.com:8080/socket").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 8080);
        assert_eq!(path, "/socket");
        
        let (host, port, path) = client.parse_websocket_url("wss://example.com/ws").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 443);
        assert_eq!(path, "/ws");
    }

    #[test]
    fn test_websocket_key_generation() {
        let client = WebSocketClient {
            socket: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(ConnectionState::Closed)),
            config: WebSocketConfig::default(),
            url: String::new(),
        };
        
        let key1 = client.generate_websocket_key();
        let key2 = client.generate_websocket_key();
        
        assert!(!key1.is_empty());
        assert!(!key2.is_empty());
        // Keys should be different (probabilistically)
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_base64_encoding() {
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
    }
}
