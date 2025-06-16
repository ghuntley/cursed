/// fr fr Rate limiting algorithms - different limiting strategies
use std::time::Duration;

use super::{ClientState, RateLimitDecision, RateLimitConfig, RateLimitResult, current_timestamp};

/// fr fr Rate limiting algorithm trait - strategy interface
pub trait RateLimitAlgorithm: Send + Sync {
    /// fr fr Check if request should be allowed - algorithm-specific logic
    fn check_limit(
        &self,
        state: &mut ClientState,
        now: u64,
        config: &RateLimitConfig,
    ) -> RateLimitResult<RateLimitDecision>;
    
    /// fr fr Get algorithm name - identification
    fn name(&self) -> &'static str;
    
    /// fr fr Get algorithm description - documentation
    fn description(&self) -> &'static str;
}

/// fr fr Fixed window algorithm - traditional rate limiting
pub struct FixedWindow {
    name: &'static str,
}

impl FixedWindow {
    /// fr fr Create new fixed window algorithm
    pub fn new() -> Self {
        Self {
            name: "FixedWindow",
        }
    }
}

impl Default for FixedWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for FixedWindow {
    fn check_limit(
        &self,
        state: &mut ClientState,
        now: u64,
        config: &RateLimitConfig,
    ) -> RateLimitResult<RateLimitDecision> {
        let window_duration = match &config.window_config {
            crate::stdlib::packages::web_vibez::ratelimit::WindowConfig::Fixed { duration } => duration.as_secs(),
            crate::stdlib::packages::web_vibez::ratelimit::WindowConfig::Sliding { duration } => duration.as_secs(),
        };

        // Check if we need to start a new window
        let window_elapsed = now.saturating_sub(state.window_start);
        if window_elapsed >= window_duration {
            // Start new window
            state.reset_window(now);
        }

        // Update last request time
        state.last_request = now;

        // Check if we're within the limit
        if state.count < config.max_requests {
            // Allow the request
            state.count += 1;
            let remaining = config.max_requests - state.count;
            let reset_time = state.window_start + window_duration;
            
            Ok(RateLimitDecision::Allow {
                remaining,
                reset_time,
                retry_after: None,
            })
        } else {
            // Deny the request
            let reset_time = state.window_start + window_duration;
            let retry_after = reset_time.saturating_sub(now);
            
            Ok(RateLimitDecision::Deny {
                retry_after,
                reset_time,
            })
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        "Fixed time window rate limiting with hard resets"
    }
}

/// fr fr Sliding window algorithm - smooth rate limiting
pub struct SlidingWindow {
    name: &'static str,
}

impl SlidingWindow {
    /// fr fr Create new sliding window algorithm
    pub fn new() -> Self {
        Self {
            name: "SlidingWindow",
        }
    }
}

impl Default for SlidingWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for SlidingWindow {
    fn check_limit(
        &self,
        state: &mut ClientState,
        now: u64,
        config: &RateLimitConfig,
    ) -> RateLimitResult<RateLimitDecision> {
        let window_duration = match &config.window_config {
            crate::stdlib::packages::web_vibez::ratelimit::WindowConfig::Fixed { duration } => duration.as_secs(),
            crate::stdlib::packages::web_vibez::ratelimit::WindowConfig::Sliding { duration } => duration.as_secs(),
        };

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
                remaining,
                reset_time,
                retry_after: None,
            })
        } else {
            // Deny the request
            // Find the oldest request to determine when we can make the next request
            let oldest_request = state.requests.iter().min().copied().unwrap_or(now);
            let retry_after = (oldest_request + window_duration).saturating_sub(now);
            
            Ok(RateLimitDecision::Deny {
                retry_after,
                reset_time: oldest_request + window_duration,
            })
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        "Sliding time window rate limiting with smooth request distribution"
    }
}

/// fr fr Token bucket algorithm - burst-friendly rate limiting
pub struct TokenBucket {
    name: &'static str,
}

impl TokenBucket {
    /// fr fr Create new token bucket algorithm
    pub fn new() -> Self {
        Self {
            name: "TokenBucket",
        }
    }

    /// fr fr Refill tokens based on elapsed time
    fn refill_tokens(state: &mut ClientState, now: u64, config: &RateLimitConfig) {
        let bucket_config = match &config.bucket_config {
            Some(bucket) => bucket,
            None => return, // No bucket config, can't refill
        };

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
        &self,
        state: &mut ClientState,
        now: u64,
        config: &RateLimitConfig,
    ) -> RateLimitResult<RateLimitDecision> {
        let bucket_config = match &config.bucket_config {
            Some(bucket) => bucket,
            None => {
                // Fallback to fixed window if no bucket config
                let fixed_window = FixedWindow::new();
                return fixed_window.check_limit(state, now, config);
            }
        };

        // Initialize tokens if this is the first request
        if state.tokens == 0.0 && state.last_request == 0 {
            state.tokens = bucket_config.capacity;
            state.last_request = now;
        }

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
                remaining,
                reset_time,
                retry_after: None,
            })
        } else {
            // Not enough tokens, deny the request
            let tokens_needed = 1.0 - state.tokens;
            let retry_after = (tokens_needed / bucket_config.refill_rate).ceil() as u64;
            let reset_time = now + retry_after;
            
            Ok(RateLimitDecision::Deny {
                retry_after,
                reset_time,
            })
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        "Token bucket rate limiting allowing controlled bursts"
    }
}

/// fr fr Leaky bucket algorithm - steady rate limiting
pub struct LeakyBucket {
    name: &'static str,
}

impl LeakyBucket {
    /// fr fr Create new leaky bucket algorithm
    pub fn new() -> Self {
        Self {
            name: "LeakyBucket",
        }
    }

    /// fr fr Process leaks based on elapsed time
    fn process_leaks(state: &mut ClientState, now: u64, config: &RateLimitConfig) {
        let bucket_config = match &config.bucket_config {
            Some(bucket) => bucket,
            None => return,
        };

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
        &self,
        state: &mut ClientState,
        now: u64,
        config: &RateLimitConfig,
    ) -> RateLimitResult<RateLimitDecision> {
        let bucket_config = match &config.bucket_config {
            Some(bucket) => bucket,
            None => {
                // Fallback to fixed window if no bucket config
                let fixed_window = FixedWindow::new();
                return fixed_window.check_limit(state, now, config);
            }
        };

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
                remaining,
                reset_time,
                retry_after: None,
            })
        } else {
            // Bucket is full, deny the request
            let retry_after = (1.0 / bucket_config.refill_rate).ceil() as u64;
            let reset_time = now + retry_after;
            
            Ok(RateLimitDecision::Deny {
                retry_after,
                reset_time,
            })
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        "Leaky bucket rate limiting with steady request processing"
    }
}

/// fr fr Adaptive algorithm - dynamic rate limiting
pub struct AdaptiveAlgorithm {
    name: &'static str,
    fixed_window: FixedWindow,
    sliding_window: SlidingWindow,
    token_bucket: TokenBucket,
}

impl AdaptiveAlgorithm {
    /// fr fr Create new adaptive algorithm
    pub fn new() -> Self {
        Self {
            name: "Adaptive",
            fixed_window: FixedWindow::new(),
            sliding_window: SlidingWindow::new(),
            token_bucket: TokenBucket::new(),
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
        };

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
}

impl Default for AdaptiveAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for AdaptiveAlgorithm {
    fn check_limit(
        &self,
        state: &mut ClientState,
        now: u64,
        config: &RateLimitConfig,
    ) -> RateLimitResult<RateLimitDecision> {
        let algorithm = self.choose_algorithm(state, config);
        algorithm.check_limit(state, now, config)
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> &'static str {
        "Adaptive rate limiting that chooses the best algorithm based on traffic patterns"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::packages::web_vibez::ratelimit::{RateLimitConfig, WindowConfig, BucketConfig};
    use std::time::Duration;

    #[tokio::test]
    async fn test_fixed_window_algorithm() {
        let algorithm = FixedWindow::new();
        let config = RateLimitConfig::new(3, Duration::from_secs(60));
        let mut state = ClientState::new();
        let now = current_timestamp();

        // First 3 requests should be allowed
        for i in 0..3 {
            let decision = algorithm.check_limit(&mut state, now + i, &config).unwrap();
            assert!(matches!(decision, RateLimitDecision::Allow { .. }));
        }

        // 4th request should be denied
        let decision = algorithm.check_limit(&mut state, now + 3, &config).unwrap();
        assert!(matches!(decision, RateLimitDecision::Deny { .. }));
    }

    #[tokio::test]
    async fn test_sliding_window_algorithm() {
        let algorithm = SlidingWindow::new();
        let config = RateLimitConfig {
            max_requests: 2,
            window_config: WindowConfig::Sliding { duration: Duration::from_secs(10) },
            bucket_config: None,
        };
        let mut state = ClientState::new();
        let now = current_timestamp();

        // First 2 requests should be allowed
        let decision1 = algorithm.check_limit(&mut state, now, &config).unwrap();
        assert!(matches!(decision1, RateLimitDecision::Allow { .. }));
        
        let decision2 = algorithm.check_limit(&mut state, now + 1, &config).unwrap();
        assert!(matches!(decision2, RateLimitDecision::Allow { .. }));

        // 3rd request should be denied
        let decision3 = algorithm.check_limit(&mut state, now + 2, &config).unwrap();
        assert!(matches!(decision3, RateLimitDecision::Deny { .. }));

        // After window slides, should be allowed again
        let decision4 = algorithm.check_limit(&mut state, now + 12, &config).unwrap();
        assert!(matches!(decision4, RateLimitDecision::Allow { .. }));
    }

    #[tokio::test]
    async fn test_token_bucket_algorithm() {
        let algorithm = TokenBucket::new();
        let config = RateLimitConfig {
            max_requests: 5,
            window_config: WindowConfig::Fixed { duration: Duration::from_secs(60) },
            bucket_config: Some(BucketConfig {
                capacity: 3.0,
                refill_rate: 1.0, // 1 token per second
            }),
        };
        let mut state = ClientState::new();
        let now = current_timestamp();

        // Should be able to make 3 requests immediately (full bucket)
        for i in 0..3 {
            let decision = algorithm.check_limit(&mut state, now + i, &config).unwrap();
            assert!(matches!(decision, RateLimitDecision::Allow { .. }));
        }

        // 4th request should be denied (bucket empty)
        let decision = algorithm.check_limit(&mut state, now + 3, &config).unwrap();
        assert!(matches!(decision, RateLimitDecision::Deny { .. }));

        // After waiting for refill, should be allowed again
        let decision = algorithm.check_limit(&mut state, now + 5, &config).unwrap();
        assert!(matches!(decision, RateLimitDecision::Allow { .. }));
    }

    #[tokio::test]
    async fn test_leaky_bucket_algorithm() {
        let algorithm = LeakyBucket::new();
        let config = RateLimitConfig {
            max_requests: 5,
            window_config: WindowConfig::Fixed { duration: Duration::from_secs(60) },
            bucket_config: Some(BucketConfig {
                capacity: 3.0,
                refill_rate: 0.5, // 0.5 requests leak per second
            }),
        };
        let mut state = ClientState::new();
        let now = current_timestamp();

        // Fill bucket to capacity
        for i in 0..3 {
            let decision = algorithm.check_limit(&mut state, now + i, &config).unwrap();
            assert!(matches!(decision, RateLimitDecision::Allow { .. }));
        }

        // Should be denied when bucket is full
        let decision = algorithm.check_limit(&mut state, now + 3, &config).unwrap();
        assert!(matches!(decision, RateLimitDecision::Deny { .. }));

        // After leak time, should have capacity again
        let decision = algorithm.check_limit(&mut state, now + 5, &config).unwrap();
        assert!(matches!(decision, RateLimitDecision::Allow { .. }));
    }

    #[tokio::test]
    async fn test_adaptive_algorithm() {
        let algorithm = AdaptiveAlgorithm::new();
        let config = RateLimitConfig::new(5, Duration::from_secs(60));
        let mut state = ClientState::new();
        let now = current_timestamp();

        // Should work with adaptive selection
        let decision = algorithm.check_limit(&mut state, now, &config).unwrap();
        assert!(matches!(decision, RateLimitDecision::Allow { .. }));
        
        assert_eq!(algorithm.name(), "Adaptive");
        assert!(algorithm.description().contains("Adaptive"));
    }

    #[tokio::test]
    async fn test_algorithm_descriptions() {
        let fixed = FixedWindow::new();
        let sliding = SlidingWindow::new();
        let token = TokenBucket::new();
        let leaky = LeakyBucket::new();
        let adaptive = AdaptiveAlgorithm::new();

        assert_eq!(fixed.name(), "FixedWindow");
        assert_eq!(sliding.name(), "SlidingWindow");
        assert_eq!(token.name(), "TokenBucket");
        assert_eq!(leaky.name(), "LeakyBucket");
        assert_eq!(adaptive.name(), "Adaptive");

        assert!(fixed.description().contains("Fixed"));
        assert!(sliding.description().contains("Sliding"));
        assert!(token.description().contains("Token"));
        assert!(leaky.description().contains("Leaky"));
        assert!(adaptive.description().contains("Adaptive"));
    }
}
