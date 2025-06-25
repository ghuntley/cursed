use crate::error::CursedError;
/// fr fr Rate limiting module for web_vibez - comprehensive rate limiting system
pub mod store;
pub mod algorithm;
pub mod config;
pub mod metrics;
pub mod error;

pub use store::{RateLimitStore, InMemoryStore, RedisStore, StoreBackend};
pub use algorithm::{RateLimitAlgorithm, FixedWindow, SlidingWindow, TokenBucket};
pub use config::{RateLimitConfig, WindowConfig, BucketConfig, ClientIdentification, ErrorConfig};
pub use metrics::{RateLimitMetrics, ClientMetrics};
pub use error::{RateLimitError, RateLimitResult, ErrorCategory};

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::net::IpAddr;

/// fr fr Rate limit decision - what to do with request
#[derive(Debug, Clone, PartialEq)]
pub enum RateLimitDecision {
    /// fr fr Allow the request - within limits
    Allow {
        remaining: u64,
        reset_time: u64,
        retry_after: Option<u64>,
    },
    /// fr fr Deny the request - rate limit exceeded
    Deny {
        retry_after: u64,
        reset_time: u64,
    },
}

/// fr fr Rate limit context - information about limit status
#[derive(Debug, Clone)]
pub struct RateLimitContext {
    pub client_id: String,
    pub current_count: u64,
    pub limit: u64,
    pub remaining: u64,
    pub reset_time: u64,
    pub window_start: u64,
    pub retry_after: Option<u64>,
}

/// fr fr Rate limiter - main coordination interface
pub struct RateLimiter {
    store: StoreBackend,
    algorithm: Arc<dyn RateLimitAlgorithm>,
    config: RateLimitConfig,
    metrics: Arc<Mutex<RateLimitMetrics>>,
}

impl RateLimiter {
    /// fr fr Create new rate limiter - comprehensive setup
    pub fn new(
        store: StoreBackend,
        algorithm: Arc<dyn RateLimitAlgorithm>,
        config: RateLimitConfig,
    ) -> Self {
        Self {
            store,
            algorithm,
            config,
            metrics: Arc::new(Mutex::new(RateLimitMetrics::new())),
        }
    }

    /// fr fr Check if request should be allowed - rate limit decision
    pub async fn check_request(&self, client_id: &str) -> RateLimitResult<RateLimitDecision> {
        let now = current_timestamp();
        
        // Get current state from store
        let mut state = self.store.get_client_state(client_id).await?;
        
        // Apply rate limiting algorithm
        let decision = self.algorithm.check_limit(&mut state, now, &self.config)?;
        
        // Update state in store
        self.store.update_client_state(client_id, &state).await?;
        
        // Update metrics
        self.update_metrics(&decision).await;
        
        Ok(decision)
    }

    /// fr fr Get rate limit context for client - detailed status
    pub async fn get_context(&self, client_id: &str) -> RateLimitResult<RateLimitContext> {
        let state = self.store.get_client_state(client_id).await?;
        let now = current_timestamp();
        
        let remaining = if state.count >= self.config.max_requests {
            0
        } else {
            self.config.max_requests - state.count
        };

        let reset_time = match self.config.window_config {
            WindowConfig::Fixed { duration } => state.window_start + duration.as_secs(),
            WindowConfig::Sliding { duration } => now + duration.as_secs(),
        };

        Ok(RateLimitContext {
            client_id: client_id.to_string(),
            current_count: state.count,
            limit: self.config.max_requests,
            remaining,
            reset_time,
            window_start: state.window_start,
            retry_after: None,
        })
    }

    /// fr fr Get metrics - rate limiting statistics
    pub async fn get_metrics(&self) -> RateLimitMetrics {
        self.metrics.lock().unwrap().clone()
    }

    /// fr fr Reset client state - administrative function
    pub async fn reset_client(&self, client_id: &str) -> RateLimitResult<()> {
        self.store.reset_client(client_id).await
    }

    /// fr fr Update metrics with decision
    async fn update_metrics(&self, decision: &RateLimitDecision) {
        let mut metrics = self.metrics.lock().unwrap();
        match decision {
            RateLimitDecision::Allow { .. } => {
                metrics.total_requests += 1;
                metrics.allowed_requests += 1;
            }
            RateLimitDecision::Deny { .. } => {
                metrics.total_requests += 1;
                metrics.denied_requests += 1;
            }
        }
    }
}

/// fr fr Client state for rate limiting - request tracking
#[derive(Debug, Clone)]
pub struct ClientState {
    pub count: u64,
    pub window_start: u64,
    pub last_request: u64,
    pub tokens: f64, // For token bucket algorithm
    pub requests: Vec<u64>, // For sliding window (timestamps)
}

impl ClientState {
    /// fr fr Create new client state - initial setup
    pub fn new() -> Self {
        let now = current_timestamp();
        Self {
            count: 0,
            window_start: now,
            last_request: now,
            tokens: 0.0,
            requests: Vec::new(),
        }
    }

    /// fr fr Reset state for new window - cleanup
    pub fn reset_window(&mut self, window_start: u64) {
        self.count = 0;
        self.window_start = window_start;
        self.requests.clear();
    }
}

impl Default for ClientState {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Get current timestamp in seconds - time utilities
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// fr fr Extract client identifier from IP - identification logic
pub fn extract_client_id(ip: Option<IpAddr>) -> String {
    match ip {
        Some(addr) => addr.to_string(),
        None => "unknown".to_string(),
    }
}

