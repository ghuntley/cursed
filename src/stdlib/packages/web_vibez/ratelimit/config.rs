use crate::web::StatusCode;
use crate::error::Error;
/// fr fr Rate limiting configuration - comprehensive settings
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// fr fr Rate limit configuration - main settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// fr fr Maximum requests allowed - core limit
    pub max_requests: u64,
    
    /// fr fr Window configuration - time-based settings
    pub window_config: WindowConfig,
    
    /// fr fr Bucket configuration - token/leaky bucket settings
    pub bucket_config: Option<BucketConfig>,
    
    /// fr fr Client identification - how to identify clients
    pub client_identification: ClientIdentification,
    
    /// fr fr Error configuration - how to handle violations
    pub error_config: ErrorConfig,
    
    /// fr fr Cleanup configuration - maintenance settings
    pub cleanup_config: CleanupConfig,
}

impl RateLimitConfig {
    /// fr fr Create new rate limit config - basic setup
    pub fn new(max_requests: u64, duration: Duration) -> Self {
        Self {
            max_requests,
            window_config: WindowConfig::Fixed { duration },
            bucket_config: None,
            client_identification: ClientIdentification::default(),
            error_config: ErrorConfig::default(),
            cleanup_config: CleanupConfig::default(),
        }
    }

    /// fr fr Create per-minute configuration - common setup
    pub fn per_minute(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(60))
    }

    /// fr fr Create per-hour configuration - lenient setup
    pub fn per_hour(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(3600))
    }

    /// fr fr Create per-second configuration - strict setup
    pub fn per_second(max_requests: u64) -> Self {
        Self::new(max_requests, Duration::from_secs(1))
    }

    /// fr fr Set sliding window - smooth limiting
    pub fn with_sliding_window(mut self, duration: Duration) -> Self {
        self.window_config = WindowConfig::Sliding { duration };
        self
    }

    /// fr fr Set token bucket - burst-friendly limiting
    pub fn with_token_bucket(mut self, capacity: f64, refill_rate: f64) -> Self {
        self.bucket_config = Some(BucketConfig {
            capacity,
            refill_rate,
        });
        self
    }

    /// fr fr Set client identification strategy
    pub fn with_client_identification(mut self, identification: ClientIdentification) -> Self {
        self.client_identification = identification;
        self
    }

    /// fr fr Set error configuration
    pub fn with_error_config(mut self, error_config: ErrorConfig) -> Self {
        self.error_config = error_config;
        self
    }

    /// fr fr Set cleanup configuration
    pub fn with_cleanup_config(mut self, cleanup_config: CleanupConfig) -> Self {
        self.cleanup_config = cleanup_config;
        self
    }

    /// fr fr Validate configuration - ensure settings are valid
    pub fn validate(&self) -> Result<(), Error> {
        if self.max_requests == 0 {
            return Err(ConfigError::InvalidMaxRequests("max_requests must be greater than 0".to_string()));
        }

        match &self.window_config {
            WindowConfig::Fixed { duration } | WindowConfig::Sliding { duration } => {
                if duration.as_secs() == 0 {
                    return Err(ConfigError::InvalidDuration("duration must be greater than 0".to_string()));
                }
            }
        }

        if let Some(bucket_config) = &self.bucket_config {
            bucket_config.validate()?;
        }

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
    Fixed { duration: Duration },
    
    /// fr fr Sliding time window - smooth limiting
    Sliding { duration: Duration },
}

/// fr fr Token/Leaky bucket configuration - burst control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketConfig {
    /// fr fr Bucket capacity - maximum tokens/requests
    pub capacity: f64,
    
    /// fr fr Refill/leak rate - tokens per second
    pub refill_rate: f64,
}

impl BucketConfig {
    /// fr fr Create new bucket configuration
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            capacity,
            refill_rate,
        }
    }

    /// fr fr Validate bucket configuration
    pub fn validate(&self) -> Result<(), Error> {
        if self.capacity <= 0.0 {
            return Err(ConfigError::InvalidCapacity("capacity must be greater than 0".to_string()));
        }

        if self.refill_rate <= 0.0 {
            return Err(ConfigError::InvalidRefillRate("refill_rate must be greater than 0".to_string()));
        }

        Ok(())
    }
}

/// fr fr Client identification strategy - how to identify clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientIdentification {
    /// fr fr Identify by IP address - basic approach
    IpAddress,
    
    /// fr fr Identify by custom header - API key or user ID
    Header { name: String },
    
    /// fr fr Identify by multiple factors - composite approach
    Composite { factors: Vec<IdentificationFactor> },
    
    /// fr fr Custom identification function - advanced approach
    Custom { identifier: String },
}

impl Default for ClientIdentification {
    fn default() -> Self {
        Self::IpAddress
    }
}

/// fr fr Identification factor - component of composite identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentificationFactor {
    /// fr fr IP address factor
    IpAddress,
    
    /// fr fr HTTP header factor
    Header { name: String },
    
    /// fr fr User agent factor
    UserAgent,
    
    /// fr fr Custom factor
    Custom { name: String, extractor: String },
}

/// fr fr Error configuration - how to handle rate limit violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorConfig {
    /// fr fr HTTP status code to return - standard response
    pub status_code: u16,
    
    /// fr fr Error message - user-facing message
    pub message: String,
    
    /// fr fr Include rate limit headers - transparency
    pub include_headers: bool,
    
    /// fr fr Include retry-after header - guidance
    pub include_retry_after: bool,
    
    /// fr fr Custom error response - advanced customization
    pub custom_response: Option<String>,
}

impl ErrorConfig {
    /// fr fr Create new error configuration
    pub fn new() -> Self {
        Self {
            status_code: 429,
            message: "Too Many Requests".to_string(),
            include_headers: true,
            include_retry_after: true,
            custom_response: None,
        }
    }

    /// fr fr Set custom status code
    pub fn with_status_code(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    }

    /// fr fr Set custom message
    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    /// fr fr Set custom response body
    pub fn with_custom_response(mut self, response: String) -> Self {
        self.custom_response = Some(response);
        self
    }

    /// fr fr Validate error configuration
    pub fn validate(&self) -> Result<(), Error> {
        if self.status_code < 400 || self.status_code >= 600 {
            return Err(ConfigError::InvalidStatusCode("status_code must be a valid 4xx or 5xx status code".to_string()));
        }

        if self.message.is_empty() {
            return Err(ConfigError::InvalidMessage("message cannot be empty".to_string()));
        }

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
    pub enabled: bool,
    
    /// fr fr Cleanup interval - how often to run
    pub interval: Duration,
    
    /// fr fr TTL for client states - when to expire
    pub client_ttl: Duration,
    
    /// fr fr Maximum clients to keep - memory management
    pub max_clients: Option<usize>,
    
    /// fr fr Cleanup strategy - how to clean up
    pub strategy: CleanupStrategy,
}

impl CleanupConfig {
    /// fr fr Create new cleanup configuration
    pub fn new() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(300), // 5 minutes
            client_ttl: Duration::from_secs(3600), // 1 hour
            max_clients: Some(10000),
            strategy: CleanupStrategy::LeastRecentlyUsed,
        }
    }

    /// fr fr Disable cleanup - manual management
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            interval: Duration::from_secs(300),
            client_ttl: Duration::from_secs(3600),
            max_clients: None,
            strategy: CleanupStrategy::LeastRecentlyUsed,
        }
    }

    /// fr fr Set cleanup interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// fr fr Set client TTL
    pub fn with_client_ttl(mut self, ttl: Duration) -> Self {
        self.client_ttl = ttl;
        self
    }

    /// fr fr Set maximum clients
    pub fn with_max_clients(mut self, max_clients: usize) -> Self {
        self.max_clients = Some(max_clients);
        self
    }

    /// fr fr Set cleanup strategy
    pub fn with_strategy(mut self, strategy: CleanupStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// fr fr Validate cleanup configuration
    pub fn validate(&self) -> Result<(), Error> {
        if self.enabled {
            if self.interval.as_secs() == 0 {
                return Err(ConfigError::InvalidInterval("cleanup interval must be greater than 0".to_string()));
            }

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
    LeastRecentlyUsed,
    
    /// fr fr First in, first out - evict oldest created
    FirstInFirstOut,
    
    /// fr fr Random eviction - unpredictable but fair
    Random,
    
    /// fr fr Least frequently used - evict least active
    LeastFrequentlyUsed,
}

/// fr fr Configuration error - validation failures
#[derive(Debug, Clone)]
pub enum ConfigError {
    /// fr fr Invalid maximum requests
    InvalidMaxRequests(String),
    
    /// fr fr Invalid duration
    InvalidDuration(String),
    
    /// fr fr Invalid capacity
    InvalidCapacity(String),
    
    /// fr fr Invalid refill rate
    InvalidRefillRate(String),
    
    /// fr fr Invalid status code
    InvalidStatusCode(String),
    
    /// fr fr Invalid message
    InvalidMessage(String),
    
    /// fr fr Invalid interval
    InvalidInterval(String),
    
    /// fr fr Invalid TTL
    InvalidTtl(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidMaxRequests(msg) => write!(f, "Invalid max requests: {}", msg),
            ConfigError::InvalidDuration(msg) => write!(f, "Invalid duration: {}", msg),
            ConfigError::InvalidCapacity(msg) => write!(f, "Invalid capacity: {}", msg),
            ConfigError::InvalidRefillRate(msg) => write!(f, "Invalid refill rate: {}", msg),
            ConfigError::InvalidStatusCode(msg) => write!(f, "Invalid status code: {}", msg),
            ConfigError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            ConfigError::InvalidInterval(msg) => write!(f, "Invalid interval: {}", msg),
            ConfigError::InvalidTtl(msg) => write!(f, "Invalid TTL: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_config_creation() {
        let config = RateLimitConfig::new(100, Duration::from_secs(60));
        assert_eq!(config.max_requests, 100);
        
        let per_minute = RateLimitConfig::per_minute(50);
        assert_eq!(per_minute.max_requests, 50);
        
        let per_hour = RateLimitConfig::per_hour(1000);
        assert_eq!(per_hour.max_requests, 1000);
    }

    #[test]
    fn test_config_validation() {
        // Valid config
        let valid_config = RateLimitConfig::per_minute(10);
        assert!(valid_config.validate().is_ok());
        
        // Invalid max_requests
        let mut invalid_config = RateLimitConfig::per_minute(10);
        invalid_config.max_requests = 0;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_bucket_config() {
        let bucket = BucketConfig::new(10.0, 1.0);
        assert_eq!(bucket.capacity, 10.0);
        assert_eq!(bucket.refill_rate, 1.0);
        assert!(bucket.validate().is_ok());
        
        // Invalid capacity
        let invalid_bucket = BucketConfig::new(0.0, 1.0);
        assert!(invalid_bucket.validate().is_err());
    }

    #[test]
    fn test_error_config() {
        let error_config = ErrorConfig::new()
            .with_status_code(429)
            .with_message("Rate limited".to_string());
        
        assert_eq!(error_config.status_code, 429);
        assert_eq!(error_config.message, "Rate limited");
        assert!(error_config.validate().is_ok());
        
        // Invalid status code
        let invalid_error_config = ErrorConfig::new().with_status_code(200);
        assert!(invalid_error_config.validate().is_err());
    }

    #[test]
    fn test_cleanup_config() {
        let cleanup = CleanupConfig::new()
            .with_interval(Duration::from_secs(60))
            .with_client_ttl(Duration::from_secs(300))
            .with_max_clients(1000);
        
        assert_eq!(cleanup.interval, Duration::from_secs(60));
        assert_eq!(cleanup.client_ttl, Duration::from_secs(300));
        assert_eq!(cleanup.max_clients, Some(1000));
        assert!(cleanup.validate().is_ok());
        
        let disabled_cleanup = CleanupConfig::disabled();
        assert!(!disabled_cleanup.enabled);
        assert!(disabled_cleanup.validate().is_ok());
    }

    #[test]
    fn test_client_identification() {
        let ip_identification = ClientIdentification::IpAddress;
        let header_identification = ClientIdentification::Header {
            name: "X-API-Key".to_string(),
        };
        let composite_identification = ClientIdentification::Composite {
            factors: vec![
                IdentificationFactor::IpAddress,
                IdentificationFactor::Header { name: "User-Agent".to_string() },
            ],
        };
        
        assert!(matches!(ip_identification, ClientIdentification::IpAddress));
        assert!(matches!(header_identification, ClientIdentification::Header { .. }));
        assert!(matches!(composite_identification, ClientIdentification::Composite { .. }));
    }

    #[test]
    fn test_window_config() {
        let fixed_window = WindowConfig::Fixed { duration: Duration::from_secs(60) };
        let sliding_window = WindowConfig::Sliding { duration: Duration::from_secs(60) };
        
        assert!(matches!(fixed_window, WindowConfig::Fixed { .. }));
        assert!(matches!(sliding_window, WindowConfig::Sliding { .. }));
    }

    #[test]
    fn test_cleanup_strategy() {
        let lru = CleanupStrategy::LeastRecentlyUsed;
        let fifo = CleanupStrategy::FirstInFirstOut;
        let random = CleanupStrategy::Random;
        let lfu = CleanupStrategy::LeastFrequentlyUsed;
        
        assert!(matches!(lru, CleanupStrategy::LeastRecentlyUsed));
        assert!(matches!(fifo, CleanupStrategy::FirstInFirstOut));
        assert!(matches!(random, CleanupStrategy::Random));
        assert!(matches!(lfu, CleanupStrategy::LeastFrequentlyUsed));
    }

    #[test]
    fn test_config_builder_pattern() {
        let config = RateLimitConfig::per_minute(100)
            .with_sliding_window(Duration::from_secs(60))
            .with_token_bucket(10.0, 1.0)
            .with_error_config(ErrorConfig::new().with_status_code(429))
            .with_cleanup_config(CleanupConfig::new().with_interval(Duration::from_secs(300)));
        
        assert_eq!(config.max_requests, 100);
        assert!(matches!(config.window_config, WindowConfig::Sliding { .. }));
        assert!(config.bucket_config.is_some());
        assert_eq!(config.error_config.status_code, 429);
        assert!(config.cleanup_config.enabled);
        
        assert!(config.validate().is_ok());
    }
}
