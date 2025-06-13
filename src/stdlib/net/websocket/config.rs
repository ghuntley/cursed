/// WebSocket configuration

use std::time::Duration;

/// WebSocket configuration
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub max_message_size: usize,
    pub max_frame_size: usize,
    pub ping_interval: Option<Duration>,
    pub pong_timeout: Duration,
    pub compression: CompressionConfig,
    pub auto_pong: bool,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            max_message_size: 64 * 1024 * 1024, // 64MB
            max_frame_size: 16 * 1024 * 1024,   // 16MB
            ping_interval: Some(Duration::from_secs(30)),
            pong_timeout: Duration::from_secs(10),
            compression: CompressionConfig::default(),
            auto_pong: true,
        }
    }
}

/// WebSocket compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    pub enable_per_message_deflate: bool,
    pub deflate_no_context_takeover: bool,
    pub deflate_max_window_bits: u8,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enable_per_message_deflate: true,
            deflate_no_context_takeover: false,
            deflate_max_window_bits: 15,
        }
    }
}

impl WebSocketConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }
    
    pub fn max_frame_size(mut self, size: usize) -> Self {
        self.max_frame_size = size;
        self
    }
    
    pub fn ping_interval(mut self, interval: Option<Duration>) -> Self {
        self.ping_interval = interval;
        self
    }
    
    pub fn pong_timeout(mut self, timeout: Duration) -> Self {
        self.pong_timeout = timeout;
        self
    }
    
    pub fn auto_pong(mut self, enabled: bool) -> Self {
        self.auto_pong = enabled;
        self
    }
    
    pub fn compression(mut self, config: CompressionConfig) -> Self {
        self.compression = config;
        self
    }
}
