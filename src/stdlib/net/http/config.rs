/// HTTP client configuration

use std::time::Duration;

/// HTTP client configuration
#[derive(Debug, Clone)]
pub struct HttpConfig {
impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            user_agent: "CURSED-HTTP-Client/1.0".to_string(),
        }
    }
/// Timeout configuration
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
impl Default for RetryConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
        }
    }
}
