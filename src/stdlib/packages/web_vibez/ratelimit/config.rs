use std::time::Duration;

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window_config: WindowConfig,
    pub client_identification: ClientIdentification,
    pub error_config: ErrorConfig,
}

/// Window configuration
#[derive(Debug, Clone)]
pub enum WindowConfig {
    Fixed { duration: u64 },
    Sliding { duration: u64 },
}

/// Bucket configuration for token bucket algorithm
#[derive(Debug, Clone)]
pub struct BucketConfig {
    pub capacity: u32,
    pub refill_rate: f64,
    pub refill_period: Duration,
}

/// Client identification method
#[derive(Debug, Clone)]
pub enum ClientIdentification {
    IpAddress,
    ApiKey,
    UserId,
    Custom(String),
}

/// Error handling configuration
#[derive(Debug, Clone)]
pub struct ErrorConfig {
    pub status_code: u16,
    pub message: String,
    pub include_retry_after: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_config: WindowConfig::Fixed { duration: 3600 },
            client_identification: ClientIdentification::IpAddress,
            error_config: ErrorConfig {
                status_code: 429,
                message: "Rate limit exceeded".to_string(),
                include_retry_after: true,
            },
        }
    }
}
