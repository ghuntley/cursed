/// fr fr Rate limiting algorithms - different limiting strategies
use std::time::Duration;

use super::{ClientState, RateLimitDecision, RateLimitConfig, RateLimitResult, current_timestamp};

/// fr fr Rate limiting algorithm trait - strategy interface
pub trait RateLimitAlgorithm: Send + Sync {
    /// fr fr Check if request should be allowed - algorithm-specific logic
    fn check_limit(
    ) -> RateLimitResult<RateLimitDecision>;
    
    /// fr fr Get algorithm name - identification
    fn name(&self) -> &'static str;
    
    /// fr fr Get algorithm description - documentation
    fn description(&self) -> &'static str;
/// fr fr Fixed window algorithm - traditional rate limiting
pub struct FixedWindow {
impl FixedWindow {
    /// fr fr Create new fixed window algorithm
    pub fn new() -> Self {
        Self {
        }
    }
impl Default for FixedWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for FixedWindow {
    fn check_limit(
    ) -> RateLimitResult<RateLimitDecision> {
        let window_duration = match &config.window_config {
//             crate::stdlib::packages::web_vibez::ratelimit::WindowConfig::Fixed { duration } => duration.as_secs(),
//             crate::stdlib::packages::web_vibez::ratelimit::WindowConfig::Sliding { duration } => duration.as_secs(),

        // Check if we need to start a new window
        let window_elapsed = now.saturating_sub(state.window_start);
        if window_elapsed >= window_duration {
            // Start new window
            state.reset_window(now);
        // Update last request time
        state.last_request = now;

        // Check if we're within the limit
        if state.count < config.max_requests {
            // Allow the request
            state.count += 1;
            let remaining = config.max_requests - state.count;
            let reset_time = state.window_start + window_duration;
            
            Ok(RateLimitDecision::Allow {
            })
        } else {
            // Deny the request
            let reset_time = state.window_start + window_duration;
            let retry_after = reset_time.saturating_sub(now);
            
            Ok(RateLimitDecision::Deny {
            })
        }
    }

    fn name(&self) -> &'static str {
        self.name
    fn description(&self) -> &'static str {
        "Fixed time window rate limiting with hard resets"
    }
}

/// fr fr Sliding window algorithm - smooth rate limiting
pub struct SlidingWindow {
impl SlidingWindow {
    /// fr fr Create new sliding window algorithm
    pub fn new() -> Self {
        Self {
        }
    }
impl Default for SlidingWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for SlidingWindow {
    fn check_limit(
    ) -> RateLimitResult<RateLimitDecision> {
        let window_duration = match &config.window_config {
//             crate::stdlib::packages::web_vibez::ratelimit::WindowConfig::Fixed { duration } => duration.as_secs(),
//             crate::stdlib::packages::web_vibez::ratelimit::WindowConfig::Sliding { duration } => duration.as_secs(),

        // Remove expired requests from sliding window
        let cutoff_time = now.saturating_sub(window_duration);
        state.requests.retain(|&timestamp| timestamp > cutoff_time);

        // Update last request time
        state.last_request = now;

        // Check if we're within the limit
        if state.requests.len() < config.max_requests as usize {
            // Allow the request
            state.requests.push(now);
            state.count = state.requests.len() as u64;
            
            let remaining = config.max_requests - state.count;
            let reset_time = now + window_duration;
            
            Ok(RateLimitDecision::Allow {
            })
        } else {
            // Deny the request
            // Find the oldest request to determine when we can make the next request
            let oldest_request = state.requests.iter().min().copied().unwrap_or(now);
            let retry_after = (oldest_request + window_duration).saturating_sub(now);
            
            Ok(RateLimitDecision::Deny {
            })
        }
    }

    fn name(&self) -> &'static str {
        self.name
    fn description(&self) -> &'static str {
        "Sliding time window rate limiting with smooth request distribution"
    }
}

/// fr fr Token bucket algorithm - burst-friendly rate limiting
pub struct TokenBucket {
impl TokenBucket {
    /// fr fr Create new token bucket algorithm
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Refill tokens based on elapsed time
    fn refill_tokens(state: &mut ClientState, now: u64, config: &RateLimitConfig) {
        let bucket_config = match &config.bucket_config {
            None => return, // No bucket config, can't refill

        let elapsed = now.saturating_sub(state.last_request);
        let tokens_to_add = (elapsed as f64) * bucket_config.refill_rate;
        
        state.tokens = (state.tokens + tokens_to_add).min(bucket_config.capacity);
        state.last_request = now;
    }
}

impl Default for TokenBucket {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for TokenBucket {
    fn check_limit(
    ) -> RateLimitResult<RateLimitDecision> {
        let bucket_config = match &config.bucket_config {
            None => {
                // Fallback to fixed window if no bucket config
                let fixed_window = FixedWindow::new();
                return fixed_window.check_limit(state, now, config);
            }

        // Initialize tokens if this is the first request
        if state.tokens == 0.0 && state.last_request == 0 {
            state.tokens = bucket_config.capacity;
            state.last_request = now;
        // Refill tokens based on elapsed time
        Self::refill_tokens(state, now, config);

        // Check if we have enough tokens
        if state.tokens >= 1.0 {
            // Consume a token and allow the request
            state.tokens -= 1.0;
            state.count += 1;
            
            let remaining = state.tokens.floor() as u64;
            let reset_time = now + (bucket_config.capacity / bucket_config.refill_rate) as u64;
            
            Ok(RateLimitDecision::Allow {
            })
        } else {
            // Not enough tokens, deny the request
            let tokens_needed = 1.0 - state.tokens;
            let retry_after = (tokens_needed / bucket_config.refill_rate).ceil() as u64;
            let reset_time = now + retry_after;
            
            Ok(RateLimitDecision::Deny {
            })
        }
    }

    fn name(&self) -> &'static str {
        self.name
    fn description(&self) -> &'static str {
        "Token bucket rate limiting allowing controlled bursts"
    }
}

/// fr fr Leaky bucket algorithm - steady rate limiting
pub struct LeakyBucket {
impl LeakyBucket {
    /// fr fr Create new leaky bucket algorithm
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Process leaks based on elapsed time
    fn process_leaks(state: &mut ClientState, now: u64, config: &RateLimitConfig) {
        let bucket_config = match &config.bucket_config {

        let elapsed = now.saturating_sub(state.last_request);
        let requests_leaked = (elapsed as f64) * bucket_config.refill_rate;
        
        state.count = (state.count as f64 - requests_leaked).max(0.0) as u64;
        state.last_request = now;
    }
}

impl Default for LeakyBucket {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for LeakyBucket {
    fn check_limit(
    ) -> RateLimitResult<RateLimitDecision> {
        let bucket_config = match &config.bucket_config {
            None => {
                // Fallback to fixed window if no bucket config
                let fixed_window = FixedWindow::new();
                return fixed_window.check_limit(state, now, config);
            }

        // Process leaks to reduce count
        Self::process_leaks(state, now, config);

        // Check if bucket has capacity
        let capacity = bucket_config.capacity as u64;
        if state.count < capacity {
            // Add request to bucket
            state.count += 1;
            
            let remaining = capacity - state.count;
            let reset_time = now + ((state.count as f64) / bucket_config.refill_rate) as u64;
            
            Ok(RateLimitDecision::Allow {
            })
        } else {
            // Bucket is full, deny the request
            let retry_after = (1.0 / bucket_config.refill_rate).ceil() as u64;
            let reset_time = now + retry_after;
            
            Ok(RateLimitDecision::Deny {
            })
        }
    }

    fn name(&self) -> &'static str {
        self.name
    fn description(&self) -> &'static str {
        "Leaky bucket rate limiting with steady request processing"
    }
}

/// fr fr Adaptive algorithm - dynamic rate limiting
pub struct AdaptiveAlgorithm {
impl AdaptiveAlgorithm {
    /// fr fr Create new adaptive algorithm
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Choose best algorithm based on request patterns
    fn choose_algorithm(&self, state: &ClientState, config: &RateLimitConfig) -> &dyn RateLimitAlgorithm {
        // Simple heuristics for algorithm selection
        let request_frequency = if state.requests.len() > 1 {
            let time_span = state.requests.last().unwrap_or(&0) - state.requests.first().unwrap_or(&0);
            if time_span > 0 {
                state.requests.len() as f64 / time_span as f64
            } else {
                0.0
            }
        } else {
            0.0

        // High frequency bursts -> Token bucket
        if request_frequency > 0.1 && config.bucket_config.is_some() {
            &self.token_bucket
        }
        // Steady traffic -> Sliding window
        else if request_frequency > 0.01 {
            &self.sliding_window
        }
        // Low frequency -> Fixed window
        else {
            &self.fixed_window
        }
    }
impl Default for AdaptiveAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for AdaptiveAlgorithm {
    fn check_limit(
    ) -> RateLimitResult<RateLimitDecision> {
        let algorithm = self.choose_algorithm(state, config);
        algorithm.check_limit(state, now, config)
    fn name(&self) -> &'static str {
        self.name
    fn description(&self) -> &'static str {
        "Adaptive rate limiting that chooses the best algorithm based on traffic patterns"
    }
}

