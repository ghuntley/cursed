use super::{ClientState, RateLimitConfig, RateLimitDecision, RateLimitError, RateLimitResult, WindowConfig};
/// Rate limit algorithm trait
pub trait RateLimitAlgorithm: Send + Sync {
    fn check_limit(&self, state: &mut ClientState, now: u64, config: &RateLimitConfig) -> RateLimitResult<RateLimitDecision>;
}

/// Fixed window algorithm
pub struct FixedWindow;

impl FixedWindow {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FixedWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for FixedWindow {
    fn check_limit(&self, state: &mut ClientState, now: u64, config: &RateLimitConfig) -> RateLimitResult<RateLimitDecision> {
        // Check if we need to reset the window
        let window_duration = match config.window_config {
            WindowConfig::Fixed { duration } => duration,
            _ => return Err(RateLimitError::ConfigurationError("Invalid window config for fixed window".to_string())),
        };
        
        if now >= state.window_start + window_duration {
            state.reset_window(now);
        }
        
        if state.count >= config.max_requests {
            Ok(RateLimitDecision::Deny {
                retry_after: state.window_start + window_duration - now,
                reset_time: state.window_start + window_duration,
            })
        } else {
            state.count += 1;
            Ok(RateLimitDecision::Allow {
                remaining: config.max_requests - state.count,
                reset_time: state.window_start + window_duration,
            })
        }
    }
}

/// Sliding window algorithm
pub struct SlidingWindow;

impl SlidingWindow {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SlidingWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimitAlgorithm for SlidingWindow {
    fn check_limit(&self, state: &mut ClientState, now: u64, config: &RateLimitConfig) -> RateLimitResult<RateLimitDecision> {
        let window_duration = match config.window_config {
            WindowConfig::Sliding { duration } => duration,
            _ => return Err(RateLimitError::ConfigurationError("Invalid window config for sliding window".to_string())),
        };
        
        // Remove old requests outside the window
        state.requests.retain(|&timestamp| timestamp > now - window_duration);
        
        if state.requests.len() as u32 >= config.max_requests {
            let oldest_request = state.requests.iter().min().copied().unwrap_or(now);
            Ok(RateLimitDecision::Deny {
                retry_after: oldest_request + window_duration - now,
                reset_time: now + window_duration,
            })
        } else {
            state.requests.push(now);
            Ok(RateLimitDecision::Allow {
                remaining: config.max_requests - state.requests.len() as u32,
                reset_time: now + window_duration,
            })
        }
    }
}

/// Token bucket algorithm
pub struct TokenBucket {
    pub capacity: f64,
    pub refill_rate: f64,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self { capacity, refill_rate }
    }
}

impl Default for TokenBucket {
    fn default() -> Self {
        Self::new(100.0, 10.0)
    }
}

impl RateLimitAlgorithm for TokenBucket {
    fn check_limit(&self, state: &mut ClientState, now: u64, _config: &RateLimitConfig) -> RateLimitResult<RateLimitDecision> {
        // Calculate tokens to add based on time elapsed
        let time_elapsed = now - state.window_start;
        let tokens_to_add = time_elapsed as f64 * self.refill_rate / 3600.0; // per hour
        
        state.tokens = (state.tokens + tokens_to_add).min(self.capacity);
        state.window_start = now;
        
        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            Ok(RateLimitDecision::Allow {
                remaining: state.tokens as u32,
                reset_time: now + ((self.capacity - state.tokens) / self.refill_rate * 3600.0) as u64,
            })
        } else {
            let retry_after = ((1.0 - state.tokens) / self.refill_rate * 3600.0) as u64;
            Ok(RateLimitDecision::Deny {
                retry_after,
                reset_time: now + retry_after,
            })
        }
    }
}
