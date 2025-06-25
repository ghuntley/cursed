use crate::error::CursedError;
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
    /// Binary message with content type
    Binary {
    /// Ping with custom payload
    Ping {
    /// Pong response with timing information
    Pong {
    /// Close frame with detailed reason
    Close {
    /// Continuation frame for fragmented messages
    Continuation {
/// Text encoding types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
/// Enhanced close codes with additional context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum CloseCode {
    // Custom application codes
impl CloseCode {
    pub fn as_u16(self) -> u16 {
        self as u16
    pub fn is_reserved(code: u16) -> bool {
        matches!(code, 1004 | 1005 | 1006 | 1015)
    pub fn is_application_code(code: u16) -> bool {
        (3000..=3999).contains(&code)
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
/// Compression extension (per-message-deflate)
#[derive(Debug)]
pub struct CompressionExtension {
impl Default for CompressionExtension {
    fn default() -> Self {
        Self {
        }
    }
impl WebSocketExtension for CompressionExtension {
    fn name(&self) -> &str {
        "permessage-deflate"
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
    fn negotiate(&self, offer: &str) -> Option<String> {
        if offer.contains("permessage-deflate") {
            Some(format!(
                if self.no_context_takeover { "; server_no_context_takeover" } else { "" }
            ))
        } else {
            None
        }
    }
// =============================================================================
// CONNECTION MANAGEMENT
// =============================================================================

/// Enhanced WebSocket connection configuration
#[derive(Debug, Clone)]
pub struct EnhancedWebSocketConfig {
    /// Supported subprotocols in order of preference
    
    /// Extensions to negotiate
    
    /// Maximum message size (bytes)
    
    /// Maximum frame size (bytes)
    
    /// Ping interval for keepalive
    
    /// Timeout for ping response
    
    /// Connection timeout
    
    /// Auto-reconnect configuration
    
    /// Compression settings
    
    /// Rate limiting
impl Default for EnhancedWebSocketConfig {
    fn default() -> Self {
        Self {
            max_message_size: 64 * 1024 * 1024, // 64MB
            max_frame_size: 16 * 1024 * 1024,   // 16MB
        }
    }
/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
/// Enhanced WebSocket connection with advanced features
#[derive(Debug)]
pub struct EnhancedWebSocketConnection {
    
    // Message handling
    
    // Connection statistics
    
    // Event channels
    
    // Background workers
    
    // Extensions
    
    // Rate limiting
/// Connection statistics
#[derive(Debug, Default)]
pub struct ConnectionStats {
/// WebSocket events
#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    Connected {
    Disconnected {
    PingReceived {
    PongReceived {
    Reconnecting {
    RateLimitExceeded {
/// Rate limiter for connection throttling
#[derive(Debug)]
pub struct RateLimiter {
impl RateLimiter {
    pub fn new(message_limit: Option<u32>, byte_limit: Option<u64>) -> Self {
        Self {
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
        if let Some(limit) = self.byte_limit {
            self.byte_tokens = (self.byte_tokens + elapsed * limit).min(limit);
        self.last_refill = now;
    }
}

impl EnhancedWebSocketConnection {
    /// Creates a new enhanced WebSocket connection
    pub fn new(config: EnhancedWebSocketConfig) -> Self {
        let (event_sender, event_receiver) = mpsc::channel();
        
        let rate_limiter = RateLimiter::new(
        );
        
        let mut connection = Self {
        
        // Add default extensions
        if config.compression_enabled {
            connection.extensions.push(Box::new(CompressionExtension::default()));
        connection
    /// Connects to WebSocket server with enhanced features
    pub fn connect(&mut self, url: &str) -> NetResult<()> {
        *self.state.lock().unwrap() = ConnectionState::Connecting;
        
        // Negotiate subprotocols and extensions
        let mut headers = HashMap::new();
        
        if !self.config.subprotocols.is_empty() {
            headers.insert(
            );
        if !self.config.extensions.is_empty() {
            headers.insert(
            );
        // Simulate connection (in real implementation, this would use actual WebSocket handshake)
        *self.state.lock().unwrap() = ConnectionState::Connected;
        
        // Set connection start time
        self.stats.lock().unwrap().connection_start = Some(Instant::now());
        
        // Start background workers
        self.start_ping_worker();
        self.start_message_processor();
        
        // Send connection event
        let _ = self.event_sender.send(WebSocketEvent::Connected {
        });
        
        Ok(())
    /// Sends a message with enhanced features
    pub fn send_message(&self, mut message: EnhancedMessage) -> NetResult<()> {
        let state = *self.state.lock().unwrap();
        if state != ConnectionState::Connected {
            return Err(NetError::ConnectionError {
            });
        // Check rate limits
        let message_size = self.get_message_size(&message);
        if !self.rate_limiter.lock().unwrap().can_send_message(message_size) {
            let _ = self.event_sender.send(WebSocketEvent::RateLimitExceeded {
                current_rate: 0.0, // Would calculate actual rate
            });
            return Err(NetError::ProtocolError {
            });
        // Process message through extensions
        for extension in &self.extensions {
            extension.process_outgoing(&mut message)?;
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
    /// Receives a message (non-blocking)
    pub fn try_receive_message(&self) -> Option<EnhancedMessage> {
        self.incoming_queue.lock().unwrap().pop_front()
    /// Receives a message (blocking with timeout)
    pub fn receive_message_timeout(&self, timeout: Duration) -> NetResult<Option<EnhancedMessage>> {
        let start = Instant::now();
        
        while start.elapsed() < timeout {
            if let Some(message) = self.try_receive_message() {
                return Ok(Some(message));
            }
            thread::sleep(Duration::from_millis(10));
        Ok(None)
    /// Closes the connection with enhanced close handling
    pub fn close(&mut self, code: CloseCode, reason: &str) -> NetResult<()> {
        *self.state.lock().unwrap() = ConnectionState::Disconnecting;
        
        // Send close frame
        let close_message = EnhancedMessage::Close {
        
        self.outgoing_queue.lock().unwrap().push_back(close_message);
        
        // Wait for close acknowledgment or timeout
        thread::sleep(Duration::from_millis(1000));
        
        *self.state.lock().unwrap() = ConnectionState::Disconnected;
        
        // Send disconnection event
        let _ = self.event_sender.send(WebSocketEvent::Disconnected {
        });
        
        Ok(())
    /// Gets connection statistics
    pub fn get_stats(&self) -> ConnectionStats {
        self.stats.lock().unwrap().clone()
    /// Gets connection state
    pub fn get_state(&self) -> ConnectionState {
        *self.state.lock().unwrap()
    /// Receives events from the connection
    pub fn try_receive_event(&self) -> Option<WebSocketEvent> {
        self.event_receiver.lock().unwrap().try_recv().ok()
    /// Starts automatic reconnection
    pub fn reconnect(&mut self) -> NetResult<()> {
        if !self.config.auto_reconnect {
            return Err(NetError::ConnectionError {
            });
        *self.state.lock().unwrap() = ConnectionState::Reconnecting;
        
        let mut attempt = 0;
        let mut delay = self.config.reconnect_base_delay;
        
        while attempt < self.config.reconnect_max_attempts {
            attempt += 1;
            
            // Send reconnecting event
            let _ = self.event_sender.send(WebSocketEvent::Reconnecting {
            });
            
            // Wait before attempting reconnection
            thread::sleep(delay);
            
            // Attempt to reconnect (simplified)
            if self.connect("").is_ok() {
                self.stats.lock().unwrap().reconnect_count += 1;
                return Ok(());
            // Exponential backoff with jitter
            delay = std::cmp::min(delay * 2, self.config.reconnect_max_delay);
        *self.state.lock().unwrap() = ConnectionState::Failed;
        Err(NetError::ConnectionError {
        })
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
                if last_ping.elapsed() >= ping_interval {
                    let ping_payload = Instant::now().elapsed().as_nanos().to_be_bytes().to_vec();
                    let ping_message = EnhancedMessage::Ping {
                    
                    outgoing_queue.lock().unwrap().push_back(ping_message);
                    stats.lock().unwrap().ping_count += 1;
                    last_ping = Instant::now();
                }
            }
        });
        
        self.worker_handles.push(handle);
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
                // Process outgoing messages
                if let Some(message) = outgoing_queue.lock().unwrap().pop_front() {
                    // Simulate sending message
                    let _ = event_sender.send(WebSocketEvent::MessageSent(message));
                // Simulate receiving messages (in real implementation, this would read from socket)
                // For now, just sleep
                thread::sleep(Duration::from_millis(10));
            }
        });
        
        self.worker_handles.push(handle);
    /// Gets message size for rate limiting
    fn get_message_size(&self, message: &EnhancedMessage) -> usize {
        match message {
            EnhancedMessage::Close { reason, .. } => reason.len() + 2, // 2 bytes for close code
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
// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

/// Creates a simple enhanced WebSocket connection
pub fn connect_enhanced(url: &str) -> NetResult<EnhancedWebSocketConnection> {
    let mut connection = EnhancedWebSocketConnection::new(EnhancedWebSocketConfig::default());
    connection.connect(url)?;
    Ok(connection)
/// Creates an enhanced WebSocket connection with custom configuration
pub fn connect_enhanced_with_config(
) -> NetResult<EnhancedWebSocketConnection> {
    let mut connection = EnhancedWebSocketConnection::new(config);
    connection.connect(url)?;
    Ok(connection)
/// Creates a WebSocket connection with compression enabled
pub fn connect_with_compression(url: &str) -> NetResult<EnhancedWebSocketConnection> {
    let mut config = EnhancedWebSocketConfig::default();
    config.compression_enabled = true;
    config.compression_threshold = 512;
    
    connect_enhanced_with_config(url, config)
/// Creates a WebSocket connection with custom subprotocol
pub fn connect_with_subprotocol(
) -> NetResult<EnhancedWebSocketConnection> {
    let mut config = EnhancedWebSocketConfig::default();
    config.subprotocols = subprotocols.iter().map(|s| s.to_string()).collect();
    
    connect_enhanced_with_config(url, config)
