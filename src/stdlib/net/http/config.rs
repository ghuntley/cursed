//! HTTP client configuration

/// HTTP client configuration
#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub timeout: TimeoutConfig,
    pub retry: RetryConfig,
    pub compression: CompressionConfig,
    pub user_agent: String,
    pub max_redirects: usize,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout: TimeoutConfig::default(),
            retry: RetryConfig::default(),
            compression: CompressionConfig::default(),
            user_agent: "CURSED-HTTP/1.0".to_string(),
            max_redirects: 5,
        }
    }
}

/// Timeout configuration
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub connection_timeout_ms: u64,
    pub request_timeout_ms: u64,
    pub read_timeout_ms: u64,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connection_timeout_ms: 30000,
            request_timeout_ms: 60000,
            read_timeout_ms: 30000,
        }
    }
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub retry_delay_ms: u64,
    pub exponential_backoff: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay_ms: 1000,
            exponential_backoff: true,
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
