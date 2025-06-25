use crate::error::CursedError;
/// fr fr Rate limiting metrics - comprehensive monitoring and statistics
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use super::{RateLimitDecision, ErrorCategory, current_timestamp};

/// fr fr Rate limit metrics - comprehensive statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitMetrics {
    /// fr fr Total requests processed - overall volume
    
    /// fr fr Allowed requests - successful requests
    
    /// fr fr Denied requests - rate limited requests
    
    /// fr fr CursedError count - processing errors
    
    /// fr fr Per-client metrics - individual client statistics
    
    /// fr fr Algorithm performance - algorithm-specific stats
    
    /// fr fr Store performance - storage backend stats
    
    /// fr fr Time-based metrics - temporal patterns
    
    /// fr fr CursedError breakdown - error categorization
    
    /// fr fr Performance statistics - timing information
    
    /// fr fr Configuration snapshot - settings at time of metrics
impl RateLimitMetrics {
    /// fr fr Create new metrics instance
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Record request decision - update statistics
    pub fn record_decision(&mut self, client_id: &str, decision: &RateLimitDecision) {
        self.total_requests += 1;
        
        match decision {
            RateLimitDecision::Allow { .. } => {
                self.allowed_requests += 1;
            }
            RateLimitDecision::Deny { .. } => {
                self.denied_requests += 1;
            }
        }
        
        // Update client metrics
        let client_metrics = self.client_metrics.entry(client_id.to_string()).or_insert_with(ClientMetrics::new);
        client_metrics.record_decision(decision);
        
        // Update time metrics
        self.time_metrics.record_request();
    /// fr fr Record error - track error statistics
    pub fn record_error(&mut self, client_id: Option<&str>, category: ErrorCategory) {
        self.error_count += 1;
        
        // Update error breakdown
        *self.error_breakdown.entry(category).or_insert(0) += 1;
        
        // Update client metrics if client known
        if let Some(client_id) = client_id {
            let client_metrics = self.client_metrics.entry(client_id.to_string()).or_insert_with(ClientMetrics::new);
            client_metrics.record_error(category);
        }
    }

    /// fr fr Record algorithm performance
    pub fn record_algorithm_performance(&mut self, algorithm_name: &str, duration: Duration, success: bool) {
        let algo_metrics = self.algorithm_metrics.entry(algorithm_name.to_string()).or_insert_with(AlgorithmMetrics::new);
        algo_metrics.record_execution(duration, success);
        
        self.performance_stats.record_algorithm_execution(duration);
    /// fr fr Record store operation
    pub fn record_store_operation(&mut self, operation: &str, duration: Duration, success: bool) {
        self.store_metrics.record_operation(operation, duration, success);
        self.performance_stats.record_store_operation(duration);
    /// fr fr Get success rate - percentage of allowed requests
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.allowed_requests as f64 / self.total_requests as f64) * 100.0
        }
    }

    /// fr fr Get denial rate - percentage of denied requests
    pub fn denial_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.denied_requests as f64 / self.total_requests as f64) * 100.0
        }
    }

    /// fr fr Get error rate - percentage of errors
    pub fn error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.error_count as f64 / self.total_requests as f64) * 100.0
        }
    }

    /// fr fr Get top clients by request count
    pub fn top_clients(&self, limit: usize) -> Vec<(&String, &ClientMetrics)> {
        let mut clients: Vec<_> = self.client_metrics.iter().collect();
        clients.sort_by(|a, b| b.1.total_requests.cmp(&a.1.total_requests));
        clients.into_iter().take(limit).collect()
    /// fr fr Get clients with highest denial rates
    pub fn highest_denial_rate_clients(&self, limit: usize) -> Vec<(&String, f64)> {
        let mut clients: Vec<_> = self.client_metrics
            .iter()
            .map(|(id, metrics)| (id, metrics.denial_rate()))
            .filter(|(_, rate)| *rate > 0.0)
            .collect();
        
        clients.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        clients.into_iter().take(limit).collect()
    /// fr fr Reset metrics - clear all statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    /// fr fr Get metrics summary - key statistics
    pub fn summary(&self) -> MetricsSummary {
        MetricsSummary {
        }
    }
impl Default for RateLimitMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Client-specific metrics - per-client statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMetrics {
    /// fr fr Total requests from client
    
    /// fr fr Allowed requests from client
    
    /// fr fr Denied requests from client
    
    /// fr fr CursedError count for client
    
    /// fr fr First request timestamp
    
    /// fr fr Last request timestamp
    
    /// fr fr Request timestamps - recent activity
    
    /// fr fr CursedError breakdown by category
impl ClientMetrics {
    /// fr fr Create new client metrics
    pub fn new() -> Self {
        let now = current_timestamp();
        Self {
        }
    }

    /// fr fr Record request decision
    pub fn record_decision(&mut self, decision: &RateLimitDecision) {
        let now = current_timestamp();
        
        self.total_requests += 1;
        self.last_request = now;
        
        if self.first_request == 0 || now < self.first_request {
            self.first_request = now;
        match decision {
            RateLimitDecision::Allow { .. } => {
                self.allowed_requests += 1;
            }
            RateLimitDecision::Deny { .. } => {
                self.denied_requests += 1;
            }
        }
        
        // Keep track of recent requests (last 100)
        self.recent_requests.push(now);
        if self.recent_requests.len() > 100 {
            self.recent_requests.remove(0);
        }
    }

    /// fr fr Record error
    pub fn record_error(&mut self, category: ErrorCategory) {
        self.error_count += 1;
        *self.error_breakdown.entry(category).or_insert(0) += 1;
    /// fr fr Get denial rate for client
    pub fn denial_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.denied_requests as f64 / self.total_requests as f64) * 100.0
        }
    }

    /// fr fr Get request frequency (requests per minute)
    pub fn request_frequency(&self) -> f64 {
        if self.recent_requests.len() < 2 {
            return 0.0;
        let time_span = self.last_request - self.first_request;
        if time_span == 0 {
            return 0.0;
        (self.total_requests as f64 / time_span as f64) * 60.0 // Per minute
    }
}

impl Default for ClientMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Algorithm-specific metrics - performance per algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmMetrics {
    /// fr fr Total executions
    
    /// fr fr Successful executions
    
    /// fr fr Failed executions
    
    /// fr fr Total execution time
    
    /// fr fr Minimum execution time
    
    /// fr fr Maximum execution time
impl AlgorithmMetrics {
    /// fr fr Create new algorithm metrics
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Record algorithm execution
    pub fn record_execution(&mut self, duration: Duration, success: bool) {
        self.executions += 1;
        self.total_time += duration;
        
        if duration < self.min_time {
            self.min_time = duration;
        if duration > self.max_time {
            self.max_time = duration;
        if success {
            self.successes += 1;
        } else {
            self.failures += 1;
        }
    }

    /// fr fr Get average execution time
    pub fn avg_time(&self) -> Duration {
        if self.executions == 0 {
            Duration::default()
        } else {
            self.total_time / self.executions as u32
        }
    }

    /// fr fr Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.executions == 0 {
            0.0
        } else {
            (self.successes as f64 / self.executions as f64) * 100.0
        }
    }
impl Default for AlgorithmMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Store performance metrics - storage backend statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreMetrics {
    /// fr fr Operation counts by type
    
    /// fr fr Operation times by type
    
    /// fr fr Success counts by operation
    
    /// fr fr Failure counts by operation
impl StoreMetrics {
    /// fr fr Create new store metrics
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Record store operation
    pub fn record_operation(&mut self, operation: &str, duration: Duration, success: bool) {
        *self.operations.entry(operation.to_string()).or_insert(0) += 1;
        
        let current_time = self.operation_times.entry(operation.to_string()).or_insert(Duration::default());
        *current_time += duration;
        
        if success {
            *self.successes.entry(operation.to_string()).or_insert(0) += 1;
        } else {
            *self.failures.entry(operation.to_string()).or_insert(0) += 1;
        }
    }

    /// fr fr Get average time for operation
    pub fn avg_time(&self, operation: &str) -> Duration {
        let count = self.operations.get(operation).copied().unwrap_or(0);
        let total_time = self.operation_times.get(operation).copied().unwrap_or_default();
        
        if count == 0 {
            Duration::default()
        } else {
            total_time / count as u32
        }
    }

    /// fr fr Get success rate for operation
    pub fn success_rate(&self, operation: &str) -> f64 {
        let successes = self.successes.get(operation).copied().unwrap_or(0);
        let failures = self.failures.get(operation).copied().unwrap_or(0);
        let total = successes + failures;
        
        if total == 0 {
            0.0
        } else {
            (successes as f64 / total as f64) * 100.0
        }
    }
impl Default for StoreMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Time-based metrics - temporal patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeMetrics {
    /// fr fr Requests in current minute
    
    /// fr fr Requests in current hour
    
    /// fr fr Requests in current day
    
    /// fr fr Start time of current minute window
    
    /// fr fr Start time of current hour window
    
    /// fr fr Start time of current day window
    
    /// fr fr Historical request counts
    
    /// fr fr Peak requests per minute
impl TimeMetrics {
    /// fr fr Create new time metrics
    pub fn new() -> Self {
        let now = current_timestamp();
        Self {
        }
    }

    /// fr fr Record request
    pub fn record_request(&mut self) {
        let now = current_timestamp();
        
        // Update minute window
        if now - self.minute_start >= 60 {
            if self.requests_this_minute > self.peak_requests_per_minute {
                self.peak_requests_per_minute = self.requests_this_minute;
            }
            self.requests_this_minute = 0;
            self.minute_start = now;
        // Update hour window
        if now - self.hour_start >= 3600 {
            self.hourly_history.push(self.requests_this_hour);
            if self.hourly_history.len() > 24 {
                self.hourly_history.remove(0);
            }
            self.requests_this_hour = 0;
            self.hour_start = now;
        // Update day window
        if now - self.day_start >= 86400 {
            self.requests_this_day = 0;
            self.day_start = now;
        self.requests_this_minute += 1;
        self.requests_this_hour += 1;
        self.requests_this_day += 1;
    /// fr fr Get requests per second (approximate)
    pub fn requests_per_second(&self) -> f64 {
        self.requests_this_minute as f64 / 60.0
    }
}

impl Default for TimeMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Performance statistics - timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    /// fr fr Algorithm execution times
    
    /// fr fr Store operation times
    
    /// fr fr Total operations
impl PerformanceStats {
    /// fr fr Create new performance stats
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Record algorithm execution time
    pub fn record_algorithm_execution(&mut self, duration: Duration) {
        self.algorithm_times.push(duration);
        if self.algorithm_times.len() > 1000 {
            self.algorithm_times.remove(0);
        }
        self.total_operations += 1;
    /// fr fr Record store operation time
    pub fn record_store_operation(&mut self, duration: Duration) {
        self.store_times.push(duration);
        if self.store_times.len() > 1000 {
            self.store_times.remove(0);
        }
    }

    /// fr fr Get average algorithm time
    pub fn avg_algorithm_time(&self) -> Duration {
        if self.algorithm_times.is_empty() {
            Duration::default()
        } else {
            let total: Duration = self.algorithm_times.iter().sum();
            total / self.algorithm_times.len() as u32
        }
    }

    /// fr fr Get average store time
    pub fn avg_store_time(&self) -> Duration {
        if self.store_times.is_empty() {
            Duration::default()
        } else {
            let total: Duration = self.store_times.iter().sum();
            total / self.store_times.len() as u32
        }
    }
impl Default for PerformanceStats {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Configuration snapshot - settings at time of metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSnapshot {
    /// fr fr Rate limit settings
    
    /// fr fr Window duration in seconds
    
    /// fr fr Algorithm name
    
    /// fr fr Store type
    
    /// fr fr Snapshot timestamp
impl ConfigSnapshot {
    /// fr fr Create new config snapshot
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Create from rate limit config
    pub fn from_config(max_requests: u64, window_duration: u64, algorithm: &str, store_type: &str) -> Self {
        Self {
        }
    }
impl Default for ConfigSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Metrics summary - key statistics overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSummary {
    /// fr fr Total requests processed
    
    /// fr fr Success rate percentage
    
    /// fr fr Denial rate percentage
    
    /// fr fr CursedError rate percentage
    
    /// fr fr Number of unique clients
    
    /// fr fr Average response time
    
    /// fr fr Requests per second
