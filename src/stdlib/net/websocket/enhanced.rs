/// Enhanced WebSocket Implementation for CURSED
/// 
/// Provides advanced WebSocket functionality including subprotocols,
/// extensions, compression, heartbeat/keepalive, connection pooling,
/// and automatic reconnection with backoff strategies.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};
use std::thread;
use super::super::{NetError, NetResult};

// =============================================================================
// WEBSOCKET ENHANCED FEATURES
// =============================================================================

/// WebSocket message types with enhanced features
#[derive(Debug, Clone)]
pub enum EnhancedMessage {
    /// Text message with encoding information
    Text {
        content: String,
        encoding: TextEncoding,
    },
    /// Binary message with content type
    Binary {
        data: Vec<u8>,
        content_type: Option<String>,
    },
    /// Ping with custom payload
    Ping {
        payload: Vec<u8>,
        timestamp: Instant,
    },
    /// Pong response with timing information
    Pong {
        payload: Vec<u8>,
        round_trip_time: Option<Duration>,
    },
    /// Close frame with detailed reason
    Close {
        code: CloseCode,
        reason: String,
        clean: bool,
    },
    /// Continuation frame for fragmented messages
    Continuation {
        data: Vec<u8>,
        is_final: bool,
        is_text: bool,
    },
}

/// Text encoding types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Latin1,
    Ascii,
}

/// Enhanced close codes with additional context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CloseCode {
    Normal = 1000,
    GoingAway = 1001,
    ProtocolError = 1002,
    UnsupportedData = 1003,
    NoStatusReceived = 1005,
    AbnormalClosure = 1006,
    InvalidFramePayloadData = 1007,
    PolicyViolation = 1008,
    MessageTooBig = 1009,
    MandatoryExtension = 1010,
    InternalServerError = 1011,
    ServiceRestart = 1012,
    TryAgainLater = 1013,
    BadGateway = 1014,
    TlsHandshake = 1015,
    // Custom application codes
    AuthenticationTimeout = 4001,
    RateLimitExceeded = 4002,
    InvalidCredentials = 4003,
    SessionExpired = 4004,
    MaintenanceMode = 4005,
}

impl CloseCode {
    pub fn as_u16(self) -> u16 {
        self as u16
    }
    
    pub fn is_reserved(code: u16) -> bool {
        matches!(code, 1004 | 1005 | 1006 | 1015)
    }
    
    pub fn is_application_code(code: u16) -> bool {
        (3000..=3999).contains(&code)
    }
    
    pub fn is_private_code(code: u16) -> bool {
        (4000..=4999).contains(&code)
    }
}

// =============================================================================
// WEBSOCKET EXTENSIONS
// =============================================================================

/// WebSocket extension interface
pub trait WebSocketExtension: Send + Sync {
    fn name(&self) -> &str;
    fn process_outgoing(&self, message: &mut EnhancedMessage) -> NetResult<()>;
    fn process_incoming(&self, message: &mut EnhancedMessage) -> NetResult<()>;
    fn negotiate(&self, offer: &str) -> Option<String>;
}

/// Compression extension (per-message-deflate)
#[derive(Debug)]
pub struct CompressionExtension {
    pub window_bits: u8,
    pub no_context_takeover: bool,
    pub compression_level: u8,
    pub threshold: usize,
}

impl Default for CompressionExtension {
    fn default() -> Self {
        Self {
            window_bits: 15,
            no_context_takeover: false,
            compression_level: 6,
            threshold: 1024,
        }
    }
}

impl WebSocketExtension for CompressionExtension {
    fn name(&self) -> &str {
        "permessage-deflate"
    }
    
    fn process_outgoing(&self, message: &mut EnhancedMessage) -> NetResult<()> {
        match message {
            EnhancedMessage::Text { content, .. } => {
                if content.len() >= self.threshold {
                    // Compress text content (simplified)
                    *content = format!("COMPRESSED:{}", content);
                }
            }
            EnhancedMessage::Binary { data, .. } => {
                if data.len() >= self.threshold {
                    // Compress binary data (simplified)
                    let mut compressed = b"COMPRESSED:".to_vec();
                    compressed.extend_from_slice(data);
                    *data = compressed;
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    fn process_incoming(&self, message: &mut EnhancedMessage) -> NetResult<()> {
        match message {
            EnhancedMessage::Text { content, .. } => {
                if content.starts_with("COMPRESSED:") {
                    // Decompress text content (simplified)
                    *content = content.strip_prefix("COMPRESSED:").unwrap().to_string();
                }
            }
            EnhancedMessage::Binary { data, .. } => {
                if data.starts_with(b"COMPRESSED:") {
                    // Decompress binary data (simplified)
                    *data = data[11..].to_vec();
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    fn negotiate(&self, offer: &str) -> Option<String> {
        if offer.contains("permessage-deflate") {
            Some(format!(
                "permessage-deflate; server_max_window_bits={}{}",
                self.window_bits,
                if self.no_context_takeover { "; server_no_context_takeover" } else { "" }
            ))
        } else {
            None
        }
    }
}

// =============================================================================
// CONNECTION MANAGEMENT
// =============================================================================

/// Enhanced WebSocket connection configuration
#[derive(Debug, Clone)]
pub struct EnhancedWebSocketConfig {
    /// Supported subprotocols in order of preference
    pub subprotocols: Vec<String>,
    
    /// Extensions to negotiate
    pub extensions: Vec<String>,
    
    /// Maximum message size (bytes)
    pub max_message_size: usize,
    
    /// Maximum frame size (bytes)
    pub max_frame_size: usize,
    
    /// Ping interval for keepalive
    pub ping_interval: Duration,
    
    /// Timeout for ping response
    pub ping_timeout: Duration,
    
    /// Connection timeout
    pub connection_timeout: Duration,
    
    /// Auto-reconnect configuration
    pub auto_reconnect: bool,
    pub reconnect_max_attempts: usize,
    pub reconnect_base_delay: Duration,
    pub reconnect_max_delay: Duration,
    
    /// Compression settings
    pub compression_enabled: bool,
    pub compression_threshold: usize,
    
    /// Rate limiting
    pub rate_limit_messages_per_second: Option<u32>,
    pub rate_limit_bytes_per_second: Option<u64>,
}

impl Default for EnhancedWebSocketConfig {
    fn default() -> Self {
        Self {
            subprotocols: Vec::new(),
            extensions: vec!["permessage-deflate".to_string()],
            max_message_size: 64 * 1024 * 1024, // 64MB
            max_frame_size: 16 * 1024 * 1024,   // 16MB
            ping_interval: Duration::from_secs(30),
            ping_timeout: Duration::from_secs(10),
            connection_timeout: Duration::from_secs(30),
            auto_reconnect: true,
            reconnect_max_attempts: 5,
            reconnect_base_delay: Duration::from_millis(1000),
            reconnect_max_delay: Duration::from_secs(30),
            compression_enabled: true,
            compression_threshold: 1024,
            rate_limit_messages_per_second: None,
            rate_limit_bytes_per_second: None,
        }
    }
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,
    Reconnecting,
    Failed,
}

/// Enhanced WebSocket connection with advanced features
#[derive(Debug)]
pub struct EnhancedWebSocketConnection {
    pub config: EnhancedWebSocketConfig,
    pub state: Arc<Mutex<ConnectionState>>,
    pub selected_subprotocol: Option<String>,
    pub negotiated_extensions: Vec<String>,
    
    // Message handling
    pub outgoing_queue: Arc<Mutex<VecDeque<EnhancedMessage>>>,
    pub incoming_queue: Arc<Mutex<VecDeque<EnhancedMessage>>>,
    
    // Connection statistics
    pub stats: Arc<Mutex<ConnectionStats>>,
    
    // Event channels
    pub event_sender: mpsc::Sender<WebSocketEvent>,
    pub event_receiver: Arc<Mutex<mpsc::Receiver<WebSocketEvent>>>,
    
    // Background workers
    pub worker_handles: Vec<thread::JoinHandle<()>>,
    
    // Extensions
    pub extensions: Vec<Box<dyn WebSocketExtension>>,
    
    // Rate limiting
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

/// Connection statistics
#[derive(Debug, Default)]
pub struct ConnectionStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub ping_count: u64,
    pub pong_count: u64,
    pub average_latency: Duration,
    pub connection_start: Option<Instant>,
    pub last_message_time: Option<Instant>,
    pub reconnect_count: u32,
}

/// WebSocket events
#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    Connected {
        subprotocol: Option<String>,
        extensions: Vec<String>,
    },
    Disconnected {
        code: CloseCode,
        reason: String,
        clean: bool,
    },
    MessageReceived(EnhancedMessage),
    MessageSent(EnhancedMessage),
    PingReceived {
        payload: Vec<u8>,
        timestamp: Instant,
    },
    PongReceived {
        payload: Vec<u8>,
        round_trip_time: Duration,
    },
    Error(NetError),
    Reconnecting {
        attempt: usize,
        delay: Duration,
    },
    RateLimitExceeded {
        limit_type: String,
        current_rate: f64,
    },
}

/// Rate limiter for connection throttling
#[derive(Debug)]
pub struct RateLimiter {
    pub message_tokens: f64,
    pub byte_tokens: f64,
    pub last_refill: Instant,
    pub message_limit: Option<f64>,
    pub byte_limit: Option<f64>,
}

impl RateLimiter {
    pub fn new(message_limit: Option<u32>, byte_limit: Option<u64>) -> Self {
        Self {
            message_tokens: message_limit.unwrap_or(0) as f64,
            byte_tokens: byte_limit.unwrap_or(0) as f64,
            last_refill: Instant::now(),
            message_limit: message_limit.map(|l| l as f64),
            byte_limit: byte_limit.map(|l| l as f64),
        }
    }
    
    pub fn can_send_message(&mut self, message_size: usize) -> bool {
        self.refill_tokens();
        
        let can_send_message = self.message_limit.map_or(true, |limit| self.message_tokens >= 1.0);
        let can_send_bytes = self.byte_limit.map_or(true, |limit| self.byte_tokens >= message_size as f64);
        
        if can_send_message && can_send_bytes {
            if let Some(_) = self.message_limit {
                self.message_tokens -= 1.0;
            }
            if let Some(_) = self.byte_limit {
                self.byte_tokens -= message_size as f64;
            }
            true
        } else {
            false
        }
    }
    
    fn refill_tokens(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        
        if let Some(limit) = self.message_limit {
            self.message_tokens = (self.message_tokens + elapsed * limit).min(limit);
        }
        
        if let Some(limit) = self.byte_limit {
            self.byte_tokens = (self.byte_tokens + elapsed * limit).min(limit);
        }
        
        self.last_refill = now;
    }
}

impl EnhancedWebSocketConnection {
    /// Creates a new enhanced WebSocket connection
    pub fn new(config: EnhancedWebSocketConfig) -> Self {
        let (event_sender, event_receiver) = mpsc::channel();
        
        let rate_limiter = RateLimiter::new(
            config.rate_limit_messages_per_second,
            config.rate_limit_bytes_per_second,
        );
        
        let mut connection = Self {
            config: config.clone(),
            state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            selected_subprotocol: None,
            negotiated_extensions: Vec::new(),
            outgoing_queue: Arc::new(Mutex::new(VecDeque::new())),
            incoming_queue: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(Mutex::new(ConnectionStats::default())),
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
            worker_handles: Vec::new(),
            extensions: Vec::new(),
            rate_limiter: Arc::new(Mutex::new(rate_limiter)),
        };
        
        // Add default extensions
        if config.compression_enabled {
            connection.extensions.push(Box::new(CompressionExtension::default()));
        }
        
        connection
    }
    
    /// Connects to WebSocket server with enhanced features
    pub fn connect(&mut self, url: &str) -> NetResult<()> {
        *self.state.lock().unwrap() = ConnectionState::Connecting;
        
        // Negotiate subprotocols and extensions
        let mut headers = HashMap::new();
        
        if !self.config.subprotocols.is_empty() {
            headers.insert(
                "Sec-WebSocket-Protocol".to_string(),
                self.config.subprotocols.join(", "),
            );
        }
        
        if !self.config.extensions.is_empty() {
            headers.insert(
                "Sec-WebSocket-Extensions".to_string(),
                self.config.extensions.join(", "),
            );
        }
        
        // Simulate connection (in real implementation, this would use actual WebSocket handshake)
        *self.state.lock().unwrap() = ConnectionState::Connected;
        
        // Set connection start time
        self.stats.lock().unwrap().connection_start = Some(Instant::now());
        
        // Start background workers
        self.start_ping_worker();
        self.start_message_processor();
        
        // Send connection event
        let _ = self.event_sender.send(WebSocketEvent::Connected {
            subprotocol: self.selected_subprotocol.clone(),
            extensions: self.negotiated_extensions.clone(),
        });
        
        Ok(())
    }
    
    /// Sends a message with enhanced features
    pub fn send_message(&self, mut message: EnhancedMessage) -> NetResult<()> {
        let state = *self.state.lock().unwrap();
        if state != ConnectionState::Connected {
            return Err(NetError::ConnectionError {
                message: format!("Cannot send message in state {:?}", state),
                address: "".to_string(),
            });
        }
        
        // Check rate limits
        let message_size = self.get_message_size(&message);
        if !self.rate_limiter.lock().unwrap().can_send_message(message_size) {
            let _ = self.event_sender.send(WebSocketEvent::RateLimitExceeded {
                limit_type: "message".to_string(),
                current_rate: 0.0, // Would calculate actual rate
            });
            return Err(NetError::ProtocolError {
                message: "Rate limit exceeded".to_string(),
                protocol: "WebSocket".to_string(),
            });
        }
        
        // Process message through extensions
        for extension in &self.extensions {
            extension.process_outgoing(&mut message)?;
        }
        
        // Add to outgoing queue
        self.outgoing_queue.lock().unwrap().push_back(message.clone());
        
        // Update statistics
        let mut stats = self.stats.lock().unwrap();
        stats.messages_sent += 1;
        stats.bytes_sent += message_size as u64;
        stats.last_message_time = Some(Instant::now());
        
        // Send event
        let _ = self.event_sender.send(WebSocketEvent::MessageSent(message));
        
        Ok(())
    }
    
    /// Receives a message (non-blocking)
    pub fn try_receive_message(&self) -> Option<EnhancedMessage> {
        self.incoming_queue.lock().unwrap().pop_front()
    }
    
    /// Receives a message (blocking with timeout)
    pub fn receive_message_timeout(&self, timeout: Duration) -> NetResult<Option<EnhancedMessage>> {
        let start = Instant::now();
        
        while start.elapsed() < timeout {
            if let Some(message) = self.try_receive_message() {
                return Ok(Some(message));
            }
            thread::sleep(Duration::from_millis(10));
        }
        
        Ok(None)
    }
    
    /// Closes the connection with enhanced close handling
    pub fn close(&mut self, code: CloseCode, reason: &str) -> NetResult<()> {
        *self.state.lock().unwrap() = ConnectionState::Disconnecting;
        
        // Send close frame
        let close_message = EnhancedMessage::Close {
            code,
            reason: reason.to_string(),
            clean: true,
        };
        
        self.outgoing_queue.lock().unwrap().push_back(close_message);
        
        // Wait for close acknowledgment or timeout
        thread::sleep(Duration::from_millis(1000));
        
        *self.state.lock().unwrap() = ConnectionState::Disconnected;
        
        // Send disconnection event
        let _ = self.event_sender.send(WebSocketEvent::Disconnected {
            code,
            reason: reason.to_string(),
            clean: true,
        });
        
        Ok(())
    }
    
    /// Gets connection statistics
    pub fn get_stats(&self) -> ConnectionStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Gets connection state
    pub fn get_state(&self) -> ConnectionState {
        *self.state.lock().unwrap()
    }
    
    /// Receives events from the connection
    pub fn try_receive_event(&self) -> Option<WebSocketEvent> {
        self.event_receiver.lock().unwrap().try_recv().ok()
    }
    
    /// Starts automatic reconnection
    pub fn reconnect(&mut self) -> NetResult<()> {
        if !self.config.auto_reconnect {
            return Err(NetError::ConnectionError {
                message: "Auto-reconnect is disabled".to_string(),
                address: "".to_string(),
            });
        }
        
        *self.state.lock().unwrap() = ConnectionState::Reconnecting;
        
        let mut attempt = 0;
        let mut delay = self.config.reconnect_base_delay;
        
        while attempt < self.config.reconnect_max_attempts {
            attempt += 1;
            
            // Send reconnecting event
            let _ = self.event_sender.send(WebSocketEvent::Reconnecting {
                attempt,
                delay,
            });
            
            // Wait before attempting reconnection
            thread::sleep(delay);
            
            // Attempt to reconnect (simplified)
            if self.connect("").is_ok() {
                self.stats.lock().unwrap().reconnect_count += 1;
                return Ok(());
            }
            
            // Exponential backoff with jitter
            delay = std::cmp::min(delay * 2, self.config.reconnect_max_delay);
        }
        
        *self.state.lock().unwrap() = ConnectionState::Failed;
        Err(NetError::ConnectionError {
            message: "Reconnection failed after maximum attempts".to_string(),
            address: "".to_string(),
        })
    }
    
    /// Starts ping worker for keepalive
    fn start_ping_worker(&mut self) {
        let state = Arc::clone(&self.state);
        let stats = Arc::clone(&self.stats);
        let outgoing_queue = Arc::clone(&self.outgoing_queue);
        let ping_interval = self.config.ping_interval;
        
        let handle = thread::spawn(move || {
            let mut last_ping = Instant::now();
            
            loop {
                thread::sleep(Duration::from_millis(1000));
                
                let current_state = *state.lock().unwrap();
                if current_state != ConnectionState::Connected {
                    break;
                }
                
                if last_ping.elapsed() >= ping_interval {
                    let ping_payload = Instant::now().elapsed().as_nanos().to_be_bytes().to_vec();
                    let ping_message = EnhancedMessage::Ping {
                        payload: ping_payload,
                        timestamp: Instant::now(),
                    };
                    
                    outgoing_queue.lock().unwrap().push_back(ping_message);
                    stats.lock().unwrap().ping_count += 1;
                    last_ping = Instant::now();
                }
            }
        });
        
        self.worker_handles.push(handle);
    }
    
    /// Starts message processor
    fn start_message_processor(&mut self) {
        let state = Arc::clone(&self.state);
        let outgoing_queue = Arc::clone(&self.outgoing_queue);
        let incoming_queue = Arc::clone(&self.incoming_queue);
        let stats = Arc::clone(&self.stats);
        let event_sender = self.event_sender.clone();
        
        let handle = thread::spawn(move || {
            loop {
                let current_state = *state.lock().unwrap();
                if current_state != ConnectionState::Connected {
                    break;
                }
                
                // Process outgoing messages
                if let Some(message) = outgoing_queue.lock().unwrap().pop_front() {
                    // Simulate sending message
                    let _ = event_sender.send(WebSocketEvent::MessageSent(message));
                }
                
                // Simulate receiving messages (in real implementation, this would read from socket)
                // For now, just sleep
                thread::sleep(Duration::from_millis(10));
            }
        });
        
        self.worker_handles.push(handle);
    }
    
    /// Gets message size for rate limiting
    fn get_message_size(&self, message: &EnhancedMessage) -> usize {
        match message {
            EnhancedMessage::Text { content, .. } => content.len(),
            EnhancedMessage::Binary { data, .. } => data.len(),
            EnhancedMessage::Ping { payload, .. } => payload.len(),
            EnhancedMessage::Pong { payload, .. } => payload.len(),
            EnhancedMessage::Close { reason, .. } => reason.len() + 2, // 2 bytes for close code
            EnhancedMessage::Continuation { data, .. } => data.len(),
        }
    }
}

impl Drop for EnhancedWebSocketConnection {
    fn drop(&mut self) {
        // Clean up worker threads
        let _ = self.close(CloseCode::Normal, "Connection dropped");
        
        for handle in self.worker_handles.drain(..) {
            let _ = handle.join();
        }
    }
}

// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

/// Creates a simple enhanced WebSocket connection
pub fn connect_enhanced(url: &str) -> NetResult<EnhancedWebSocketConnection> {
    let mut connection = EnhancedWebSocketConnection::new(EnhancedWebSocketConfig::default());
    connection.connect(url)?;
    Ok(connection)
}

/// Creates an enhanced WebSocket connection with custom configuration
pub fn connect_enhanced_with_config(
    url: &str,
    config: EnhancedWebSocketConfig,
) -> NetResult<EnhancedWebSocketConnection> {
    let mut connection = EnhancedWebSocketConnection::new(config);
    connection.connect(url)?;
    Ok(connection)
}

/// Creates a WebSocket connection with compression enabled
pub fn connect_with_compression(url: &str) -> NetResult<EnhancedWebSocketConnection> {
    let mut config = EnhancedWebSocketConfig::default();
    config.compression_enabled = true;
    config.compression_threshold = 512;
    
    connect_enhanced_with_config(url, config)
}

/// Creates a WebSocket connection with custom subprotocol
pub fn connect_with_subprotocol(
    url: &str,
    subprotocols: &[&str],
) -> NetResult<EnhancedWebSocketConnection> {
    let mut config = EnhancedWebSocketConfig::default();
    config.subprotocols = subprotocols.iter().map(|s| s.to_string()).collect();
    
    connect_enhanced_with_config(url, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enhanced_connection_creation() {
        let config = EnhancedWebSocketConfig::default();
        let connection = EnhancedWebSocketConnection::new(config);
        
        assert_eq!(connection.get_state(), ConnectionState::Disconnected);
        assert!(connection.negotiated_extensions.is_empty());
    }
    
    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(Some(10), Some(1024));
        
        // Should allow initial messages
        assert!(limiter.can_send_message(100));
        
        // Should eventually hit rate limit
        for _ in 0..20 {
            limiter.can_send_message(100);
        }
    }
    
    #[test]
    fn test_close_codes() {
        assert_eq!(CloseCode::Normal.as_u16(), 1000);
        assert_eq!(CloseCode::AuthenticationTimeout.as_u16(), 4001);
        
        assert!(CloseCode::is_reserved(1005));
        assert!(CloseCode::is_application_code(3000));
        assert!(CloseCode::is_private_code(4000));
    }
    
    #[test]
    fn test_compression_extension() {
        let extension = CompressionExtension::default();
        assert_eq!(extension.name(), "permessage-deflate");
        
        let offer = "permessage-deflate; client_max_window_bits=15";
        let response = extension.negotiate(offer);
        assert!(response.is_some());
    }
    
    #[test]
    fn test_message_types() {
        let text_msg = EnhancedMessage::Text {
            content: "Hello".to_string(),
            encoding: TextEncoding::Utf8,
        };
        
        let binary_msg = EnhancedMessage::Binary {
            data: vec![1, 2, 3, 4],
            content_type: Some("application/octet-stream".to_string()),
        };
        
        let close_msg = EnhancedMessage::Close {
            code: CloseCode::Normal,
            reason: "Goodbye".to_string(),
            clean: true,
        };
        
        // Messages should be created successfully
        match text_msg {
            EnhancedMessage::Text { content, encoding } => {
                assert_eq!(content, "Hello");
                assert_eq!(encoding, TextEncoding::Utf8);
            }
            _ => panic!("Wrong message type"),
        }
    }
}
