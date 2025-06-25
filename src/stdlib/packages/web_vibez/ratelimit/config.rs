use crate::web::StatusCode;
use crate::error::CursedError;
/// fr fr Rate limiting configuration - comprehensive settings
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// fr fr Rate limit configuration - main settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// fr fr Maximum requests allowed - core limit
    
    /// fr fr Window configuration - time-based settings
    
    /// fr fr Bucket configuration - token/leaky bucket settings
    
    /// fr fr Client identification - how to identify clients
    
    /// fr fr CursedError configuration - how to handle violations
    
    /// fr fr Cleanup configuration - maintenance settings
impl RateLimitConfig {
    /// fr fr Create new rate limit config - basic setup
    pub fn new(max_requests: u64, duration: Duration) -> Self {
        Self {
        }
    }

    /// fr fr Create per-minute configuration - common setup
    pub fn per_minute(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(60))
    /// fr fr Create per-hour configuration - lenient setup
    pub fn per_hour(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(3600))
    /// fr fr Create per-second configuration - strict setup
    pub fn per_second(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(1))
    /// fr fr Set sliding window - smooth limiting
    pub fn with_sliding_window(mut self, duration: Duration) -> Self {
        self.window_config = WindowConfig::Sliding { duration };
        self
    /// fr fr Set token bucket - burst-friendly limiting
    pub fn with_token_bucket(mut self, capacity: f64, refill_rate: f64) -> Self {
        self.bucket_config = Some(BucketConfig {
        });
        self
    /// fr fr Set client identification strategy
    pub fn with_client_identification(mut self, identification: ClientIdentification) -> Self {
        self.client_identification = identification;
        self
    /// fr fr Set error configuration
    pub fn with_error_config(mut self, error_config: ErrorConfig) -> Self {
        self.error_config = error_config;
        self
    /// fr fr Set cleanup configuration
    pub fn with_cleanup_config(mut self, cleanup_config: CleanupConfig) -> Self {
        self.cleanup_config = cleanup_config;
        self
    /// fr fr Validate configuration - ensure settings are valid
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.max_requests == 0 {
            return Err(ConfigError::InvalidMaxRequests("max_requests must be greater than 0".to_string()));
        match &self.window_config {
            WindowConfig::Fixed { duration } | WindowConfig::Sliding { duration } => {
                if duration.as_secs() == 0 {
                    return Err(ConfigError::InvalidDuration("duration must be greater than 0".to_string()));
                }
            }
        if let Some(bucket_config) = &self.bucket_config {
            bucket_config.validate()?;
        self.error_config.validate()?;
        self.cleanup_config.validate()?;

        Ok(())
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self::per_minute(60) // 60 requests per minute
    }
}

/// fr fr Time window configuration - different window strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowConfig {
    /// fr fr Fixed time window - traditional approach
    
    /// fr fr Sliding time window - smooth limiting
/// fr fr Token/Leaky bucket configuration - burst control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketConfig {
    /// fr fr Bucket capacity - maximum tokens/requests
    
    /// fr fr Refill/leak rate - tokens per second
impl BucketConfig {
    /// fr fr Create new bucket configuration
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
        }
    }

    /// fr fr Validate bucket configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.capacity <= 0.0 {
            return Err(ConfigError::InvalidCapacity("capacity must be greater than 0".to_string()));
        if self.refill_rate <= 0.0 {
            return Err(ConfigError::InvalidRefillRate("refill_rate must be greater than 0".to_string()));
        Ok(())
    }
}

/// fr fr Client identification strategy - how to identify clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientIdentification {
    /// fr fr Identify by IP address - basic approach
    
    /// fr fr Identify by custom header - API key or user ID
    
    /// fr fr Identify by multiple factors - composite approach
    
    /// fr fr Custom identification function - advanced approach
impl Default for ClientIdentification {
    fn default() -> Self {
        Self::IpAddress
    }
}

/// fr fr Identification factor - component of composite identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentificationFactor {
    /// fr fr IP address factor
    
    /// fr fr HTTP header factor
    
    /// fr fr User agent factor
    
    /// fr fr Custom factor
/// fr fr CursedError configuration - how to handle rate limit violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorConfig {
    /// fr fr HTTP status code to return - standard response
    
    /// fr fr CursedError message - user-facing message
    
    /// fr fr Include rate limit headers - transparency
    
    /// fr fr Include retry-after header - guidance
    
    /// fr fr Custom error response - advanced customization
impl ErrorConfig {
    /// fr fr Create new error configuration
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Set custom status code
    pub fn with_status_code(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    /// fr fr Set custom message
    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    /// fr fr Set custom response body
    pub fn with_custom_response(mut self, response: String) -> Self {
        self.custom_response = Some(response);
        self
    /// fr fr Validate error configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.status_code < 400 || self.status_code >= 600 {
            return Err(ConfigError::InvalidStatusCode("status_code must be a valid 4xx or 5xx status code".to_string()));
        if self.message.is_empty() {
            return Err(ConfigError::InvalidMessage("message cannot be empty".to_string()));
        Ok(())
    }
}

impl Default for ErrorConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Cleanup configuration - maintenance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupConfig {
    /// fr fr Enable automatic cleanup - maintenance
    
    /// fr fr Cleanup interval - how often to run
    
    /// fr fr TTL for client states - when to expire
    
    /// fr fr Maximum clients to keep - memory management
    
    /// fr fr Cleanup strategy - how to clean up
impl CleanupConfig {
    /// fr fr Create new cleanup configuration
    pub fn new() -> Self {
        Self {
            interval: Duration::from_secs(300), // 5 minutes
            client_ttl: Duration::from_secs(3600), // 1 hour
        }
    }

    /// fr fr Disable cleanup - manual management
    pub fn disabled() -> Self {
        Self {
        }
    }

    /// fr fr Set cleanup interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    /// fr fr Set client TTL
    pub fn with_client_ttl(mut self, ttl: Duration) -> Self {
        self.client_ttl = ttl;
        self
    /// fr fr Set maximum clients
    pub fn with_max_clients(mut self, max_clients: usize) -> Self {
        self.max_clients = Some(max_clients);
        self
    /// fr fr Set cleanup strategy
    pub fn with_strategy(mut self, strategy: CleanupStrategy) -> Self {
        self.strategy = strategy;
        self
    /// fr fr Validate cleanup configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.enabled {
            if self.interval.as_secs() == 0 {
                return Err(ConfigError::InvalidInterval("cleanup interval must be greater than 0".to_string()));
            if self.client_ttl.as_secs() == 0 {
                return Err(ConfigError::InvalidTtl("client TTL must be greater than 0".to_string()));
            }
        }

        Ok(())
    }
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Cleanup strategy - how to evict old clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CleanupStrategy {
    /// fr fr Least recently used - evict oldest accessed
    
    /// fr fr First in, first out - evict oldest created
    
    /// fr fr Random eviction - unpredictable but fair
    
    /// fr fr Least frequently used - evict least active
/// fr fr Configuration error - validation failures
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// fr fr Invalid maximum requests
    
    /// fr fr Invalid duration
    
    /// fr fr Invalid capacity
    
    /// fr fr Invalid refill rate
    
    /// fr fr Invalid status code
    
    /// fr fr Invalid message
    
    /// fr fr Invalid interval
    
    /// fr fr Invalid TTL
// impl std::fmt::Display for ConfigError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ConfigError::InvalidMaxRequests(msg) => write!(f, "Invalid max requests: {}", msg),
//             ConfigError::InvalidDuration(msg) => write!(f, "Invalid duration: {}", msg),
//             ConfigError::InvalidCapacity(msg) => write!(f, "Invalid capacity: {}", msg),
//             ConfigError::InvalidRefillRate(msg) => write!(f, "Invalid refill rate: {}", msg),
//             ConfigError::InvalidStatusCode(msg) => write!(f, "Invalid status code: {}", msg),
//             ConfigError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
//             ConfigError::InvalidInterval(msg) => write!(f, "Invalid interval: {}", msg),
//             ConfigError::InvalidTtl(msg) => write!(f, "Invalid TTL: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for ConfigError {}
// 
