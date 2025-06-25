/// WebSocket configuration

use std::time::Duration;

/// WebSocket configuration
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            max_message_size: 64 * 1024 * 1024, // 64MB
            max_frame_size: 16 * 1024 * 1024,   // 16MB
        }
    }
/// WebSocket compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
        }
    }
impl WebSocketConfig {
    pub fn new() -> Self {
        Self::default()
    pub fn max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    pub fn max_frame_size(mut self, size: usize) -> Self {
        self.max_frame_size = size;
        self
    pub fn ping_interval(mut self, interval: Option<Duration>) -> Self {
        self.ping_interval = interval;
        self
    pub fn pong_timeout(mut self, timeout: Duration) -> Self {
        self.pong_timeout = timeout;
        self
    pub fn auto_pong(mut self, enabled: bool) -> Self {
        self.auto_pong = enabled;
        self
    pub fn compression(mut self, config: CompressionConfig) -> Self {
        self.compression = config;
        self
    }
}
