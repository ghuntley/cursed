// Rate limiting module for CURSED
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Identification factor for rate limiting
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdentificationFactor {
    /// Client IP address
    /// User ID
    /// API key
    /// Session token
    /// Custom identifier
impl IdentificationFactor {
    /// Create IP address identification
    pub fn ip<I: Into<std::net::IpAddr>>(ip: I) -> Self {
        Self::IpAddress(ip.into())
    /// Create user ID identification
    pub fn user_id<S: Into<String>>(id: S) -> Self {
        Self::UserId(id.into())
    /// Create API key identification
    pub fn api_key<S: Into<String>>(key: S) -> Self {
        Self::ApiKey(key.into())
    /// Create session token identification
    pub fn session_token<S: Into<String>>(token: S) -> Self {
        Self::SessionToken(token.into())
    /// Create custom identification
    pub fn custom<S: Into<String>>(id: S) -> Self {
        Self::Custom(id.into())
    }
}

impl std::fmt::Display for IdentificationFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Maximum requests per window
    /// Time window duration
    /// Burst allowance
    /// Reset behavior
/// Reset behavior for rate limiter
#[derive(Debug, Clone, PartialEq)]
pub enum ResetBehavior {
    /// Fixed window (resets at fixed intervals)
    /// Sliding window (continuous tracking)
    /// Token bucket (refills at constant rate)
impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Rate limiter state for a client
#[derive(Debug, Clone)]
struct RateLimitState {
impl RateLimitState {
    fn new(initial_tokens: u32) -> Self {
        Self {
        }
    }
/// Rate limiter
#[derive(Debug)]
pub struct RateLimiter {
impl RateLimiter {
    /// Create new rate limiter
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
        }
    }
    
    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(RateLimiterConfig::default())
    /// Check if request is allowed
    pub fn is_allowed(&mut self, id: &IdentificationFactor) -> bool {
        let now = Instant::now();
        let state = self.states.entry(id.clone()).or_insert_with(|| {
            RateLimitState::new(self.config.max_requests)
        });
        
        match self.config.reset_behavior {
        }
    }
    
    /// Get remaining requests for identifier
    pub fn remaining_requests(&mut self, id: &IdentificationFactor) -> u32 {
        let now = Instant::now();
        let state = self.states.entry(id.clone()).or_insert_with(|| {
            RateLimitState::new(self.config.max_requests)
        });
        
        match self.config.reset_behavior {
            ResetBehavior::TokenBucket => {
                self.refill_tokens(state, now);
                state.tokens
            ResetBehavior::SlidingWindow => {
                self.cleanup_old_requests(state, now);
                self.config.max_requests.saturating_sub(state.requests.len() as u32)
            ResetBehavior::FixedWindow => {
                if self.should_reset_window(state, now) {
                    self.config.max_requests
                } else {
                    self.config.max_requests.saturating_sub(state.requests.len() as u32)
                }
            }
        }
    }
    
    /// Get time until reset
    pub fn time_until_reset(&self, id: &IdentificationFactor) -> Option<Duration> {
        if let Some(state) = self.states.get(id) {
            let now = Instant::now();
            match self.config.reset_behavior {
                ResetBehavior::FixedWindow => {
                    if let Some(oldest) = state.requests.first() {
                        let window_end = *oldest + self.config.window_duration;
                        if window_end > now {
                            Some(window_end - now)
                        } else {
                            Some(Duration::ZERO)
                        }
                    } else {
                        Some(Duration::ZERO)
                    }
                ResetBehavior::SlidingWindow => {
                    if let Some(oldest) = state.requests.first() {
                        let window_end = *oldest + self.config.window_duration;
                        if window_end > now {
                            Some(window_end - now)
                        } else {
                            Some(Duration::ZERO)
                        }
                    } else {
                        Some(Duration::ZERO)
                    }
                ResetBehavior::TokenBucket => {
                    // Token bucket refills continuously
                    Some(Duration::from_secs(1))
                }
            }
        } else {
            Some(Duration::ZERO)
        }
    }
    
    /// Clear all rate limit states
    pub fn clear(&mut self) {
        self.states.clear();
    /// Clear state for specific identifier
    pub fn clear_identifier(&mut self, id: &IdentificationFactor) {
        self.states.remove(id);
    fn check_fixed_window(&mut self, state: &mut RateLimitState, now: Instant) -> bool {
        if self.should_reset_window(state, now) {
            state.requests.clear();
        if state.requests.len() < self.config.max_requests as usize {
            state.requests.push(now);
            true
        } else {
            false
        }
    }
    
    fn check_sliding_window(&mut self, state: &mut RateLimitState, now: Instant) -> bool {
        self.cleanup_old_requests(state, now);
        
        if state.requests.len() < self.config.max_requests as usize {
            state.requests.push(now);
            true
        } else {
            false
        }
    }
    
    fn check_token_bucket(&mut self, state: &mut RateLimitState, now: Instant) -> bool {
        self.refill_tokens(state, now);
        
        if state.tokens > 0 {
            state.tokens -= 1;
            true
        } else {
            false
        }
    }
    
    fn should_reset_window(&self, state: &RateLimitState, now: Instant) -> bool {
        if let Some(first_request) = state.requests.first() {
            now.duration_since(*first_request) >= self.config.window_duration
        } else {
            false
        }
    }
    
    fn cleanup_old_requests(&self, state: &mut RateLimitState, now: Instant) {
        let cutoff = now - self.config.window_duration;
        state.requests.retain(|&timestamp| timestamp > cutoff);
    fn refill_tokens(&self, state: &mut RateLimitState, now: Instant) {
        let time_passed = now.duration_since(state.last_refill);
        let refill_rate = self.config.max_requests as f64 / self.config.window_duration.as_secs_f64();
        let tokens_to_add = (time_passed.as_secs_f64() * refill_rate) as u32;
        
        if tokens_to_add > 0 {
            state.tokens = (state.tokens + tokens_to_add).min(self.config.max_requests);
            state.last_refill = now;
        }
    }
impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(RateLimiterConfig::default())
    }
}

/// Rate limit result
#[derive(Debug, Clone)]
pub struct RateLimitResult {
impl RateLimitResult {
    pub fn allowed(remaining: u32, reset_time: Option<Duration>) -> Self {
        Self {
        }
    }
    
    pub fn denied(retry_after: Duration) -> Self {
        Self {
        }
    }
}
