/// HTTP client configuration

use std::time::Duration;

/// HTTP client configuration
#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub timeout: TimeoutConfig,
    pub retry: RetryConfig,
    pub compression: CompressionConfig,
    pub follow_redirects: bool,
    pub max_redirects: usize,
    pub user_agent: String,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout: TimeoutConfig::default(),
            retry: RetryConfig::default(),
            compression: CompressionConfig::default(),
            follow_redirects: true,
            max_redirects: 10,
            user_agent: "CURSED-HTTP-Client/1.0".to_string(),
        }
    }
}

/// Timeout configuration
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub connect: Duration,
    pub read: Duration,
    pub write: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connect: Duration::from_secs(30),
            read: Duration::from_secs(30),
            write: Duration::from_secs(30),
        }
    }
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub retry_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
        }
    }
}

/// Compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    pub enable_gzip: bool,
    pub enable_deflate: bool,
    pub enable_brotli: bool,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enable_gzip: true,
            enable_deflate: true,
            enable_brotli: false,
        }
    }
}
