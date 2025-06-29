//! WebSocket configuration

/// WebSocket configuration
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub max_frame_size: usize,
    pub max_message_size: usize,
    pub ping_interval_ms: u64,
    pub pong_timeout_ms: u64,
    pub compression: Option<CompressionConfig>,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            max_frame_size: 16 * 1024 * 1024, // 16MB
            max_message_size: 64 * 1024 * 1024, // 64MB
            ping_interval_ms: 30000, // 30 seconds
            pong_timeout_ms: 10000, // 10 seconds
            compression: None,
        }
    }
}

/// WebSocket compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    pub method: CompressionMethod,
    pub window_bits: u8,
    pub no_context_takeover: bool,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            method: CompressionMethod::PerMessageDeflate,
            window_bits: 15,
            no_context_takeover: false,
        }
    }
}

/// WebSocket compression methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionMethod {
    PerMessageDeflate,
}

impl std::fmt::Display for CompressionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionMethod::PerMessageDeflate => write!(f, "per-message-deflate"),
        }
    }
}
