/// fr fr Rate limiting metrics - comprehensive monitoring and statistics
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use super::{RateLimitDecision, ErrorCategory, current_timestamp};

/// fr fr Rate limit metrics - comprehensive statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitMetrics {
    /// fr fr Total requests processed - overall volume
    pub total_requests: u64,
    
    /// fr fr Allowed requests - successful requests
    pub allowed_requests: u64,
    
    /// fr fr Denied requests - rate limited requests
    pub denied_requests: u64,
    
    /// fr fr Error count - processing errors
    pub error_count: u64,
    
    /// fr fr Per-client metrics - individual client statistics
    pub client_metrics: HashMap<String, ClientMetrics>,
    
    /// fr fr Algorithm performance - algorithm-specific stats
    pub algorithm_metrics: HashMap<String, AlgorithmMetrics>,
    
    /// fr fr Store performance - storage backend stats
    pub store_metrics: StoreMetrics,
    
    /// fr fr Time-based metrics - temporal patterns
    pub time_metrics: TimeMetrics,
    
    /// fr fr Error breakdown - error categorization
    pub error_breakdown: HashMap<ErrorCategory, u64>,
    
    /// fr fr Performance statistics - timing information
    pub performance_stats: PerformanceStats,
    
    /// fr fr Configuration snapshot - settings at time of metrics
    pub config_snapshot: ConfigSnapshot,
}

impl RateLimitMetrics {
    /// fr fr Create new metrics instance
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            allowed_requests: 0,
            denied_requests: 0,
            error_count: 0,
            client_metrics: HashMap::new(),
            algorithm_metrics: HashMap::new(),
            store_metrics: StoreMetrics::new(),
            time_metrics: TimeMetrics::new(),
            error_breakdown: HashMap::new(),
            performance_stats: PerformanceStats::new(),
            config_snapshot: ConfigSnapshot::new(),
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
    }

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
    }

    /// fr fr Record store operation
    pub fn record_store_operation(&mut self, operation: &str, duration: Duration, success: bool) {
        self.store_metrics.record_operation(operation, duration, success);
        self.performance_stats.record_store_operation(duration);
    }

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
    }

    /// fr fr Get clients with highest denial rates
    pub fn highest_denial_rate_clients(&self, limit: usize) -> Vec<(&String, f64)> {
        let mut clients: Vec<_> = self.client_metrics
            .iter()
            .map(|(id, metrics)| (id, metrics.denial_rate()))
            .filter(|(_, rate)| *rate > 0.0)
            .collect();
        
        clients.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        clients.into_iter().take(limit).collect()
    }

    /// fr fr Reset metrics - clear all statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// fr fr Get metrics summary - key statistics
    pub fn summary(&self) -> MetricsSummary {
        MetricsSummary {
            total_requests: self.total_requests,
            success_rate: self.success_rate(),
            denial_rate: self.denial_rate(),
            error_rate: self.error_rate(),
            unique_clients: self.client_metrics.len(),
            avg_response_time: self.performance_stats.avg_algorithm_time(),
            requests_per_second: self.time_metrics.requests_per_second(),
        }
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
    pub total_requests: u64,
    
    /// fr fr Allowed requests from client
    pub allowed_requests: u64,
    
    /// fr fr Denied requests from client
    pub denied_requests: u64,
    
    /// fr fr Error count for client
    pub error_count: u64,
    
    /// fr fr First request timestamp
    pub first_request: u64,
    
    /// fr fr Last request timestamp
    pub last_request: u64,
    
    /// fr fr Request timestamps - recent activity
    pub recent_requests: Vec<u64>,
    
    /// fr fr Error breakdown by category
    pub error_breakdown: HashMap<ErrorCategory, u64>,
}

impl ClientMetrics {
    /// fr fr Create new client metrics
    pub fn new() -> Self {
        let now = current_timestamp();
        Self {
            total_requests: 0,
            allowed_requests: 0,
            denied_requests: 0,
            error_count: 0,
            first_request: now,
            last_request: now,
            recent_requests: Vec::new(),
            error_breakdown: HashMap::new(),
        }
    }

    /// fr fr Record request decision
    pub fn record_decision(&mut self, decision: &RateLimitDecision) {
        let now = current_timestamp();
        
        self.total_requests += 1;
        self.last_request = now;
        
        if self.first_request == 0 || now < self.first_request {
            self.first_request = now;
        }
        
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
    }

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
        }
        
        let time_span = self.last_request - self.first_request;
        if time_span == 0 {
            return 0.0;
        }
        
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
    pub executions: u64,
    
    /// fr fr Successful executions
    pub successes: u64,
    
    /// fr fr Failed executions
    pub failures: u64,
    
    /// fr fr Total execution time
    pub total_time: Duration,
    
    /// fr fr Minimum execution time
    pub min_time: Duration,
    
    /// fr fr Maximum execution time
    pub max_time: Duration,
}

impl AlgorithmMetrics {
    /// fr fr Create new algorithm metrics
    pub fn new() -> Self {
        Self {
            executions: 0,
            successes: 0,
            failures: 0,
            total_time: Duration::default(),
            min_time: Duration::MAX,
            max_time: Duration::default(),
        }
    }

    /// fr fr Record algorithm execution
    pub fn record_execution(&mut self, duration: Duration, success: bool) {
        self.executions += 1;
        self.total_time += duration;
        
        if duration < self.min_time {
            self.min_time = duration;
        }
        
        if duration > self.max_time {
            self.max_time = duration;
        }
        
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
    pub operations: HashMap<String, u64>,
    
    /// fr fr Operation times by type
    pub operation_times: HashMap<String, Duration>,
    
    /// fr fr Success counts by operation
    pub successes: HashMap<String, u64>,
    
    /// fr fr Failure counts by operation
    pub failures: HashMap<String, u64>,
}

impl StoreMetrics {
    /// fr fr Create new store metrics
    pub fn new() -> Self {
        Self {
            operations: HashMap::new(),
            operation_times: HashMap::new(),
            successes: HashMap::new(),
            failures: HashMap::new(),
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
    pub requests_this_minute: u64,
    
    /// fr fr Requests in current hour
    pub requests_this_hour: u64,
    
    /// fr fr Requests in current day
    pub requests_this_day: u64,
    
    /// fr fr Start time of current minute window
    pub minute_start: u64,
    
    /// fr fr Start time of current hour window
    pub hour_start: u64,
    
    /// fr fr Start time of current day window
    pub day_start: u64,
    
    /// fr fr Historical request counts
    pub hourly_history: Vec<u64>,
    
    /// fr fr Peak requests per minute
    pub peak_requests_per_minute: u64,
}

impl TimeMetrics {
    /// fr fr Create new time metrics
    pub fn new() -> Self {
        let now = current_timestamp();
        Self {
            requests_this_minute: 0,
            requests_this_hour: 0,
            requests_this_day: 0,
            minute_start: now,
            hour_start: now,
            day_start: now,
            hourly_history: Vec::new(),
            peak_requests_per_minute: 0,
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
        }
        
        // Update hour window
        if now - self.hour_start >= 3600 {
            self.hourly_history.push(self.requests_this_hour);
            if self.hourly_history.len() > 24 {
                self.hourly_history.remove(0);
            }
            self.requests_this_hour = 0;
            self.hour_start = now;
        }
        
        // Update day window
        if now - self.day_start >= 86400 {
            self.requests_this_day = 0;
            self.day_start = now;
        }
        
        self.requests_this_minute += 1;
        self.requests_this_hour += 1;
        self.requests_this_day += 1;
    }

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
    pub algorithm_times: Vec<Duration>,
    
    /// fr fr Store operation times
    pub store_times: Vec<Duration>,
    
    /// fr fr Total operations
    pub total_operations: u64,
}

impl PerformanceStats {
    /// fr fr Create new performance stats
    pub fn new() -> Self {
        Self {
            algorithm_times: Vec::new(),
            store_times: Vec::new(),
            total_operations: 0,
        }
    }

    /// fr fr Record algorithm execution time
    pub fn record_algorithm_execution(&mut self, duration: Duration) {
        self.algorithm_times.push(duration);
        if self.algorithm_times.len() > 1000 {
            self.algorithm_times.remove(0);
        }
        self.total_operations += 1;
    }

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
    pub max_requests: u64,
    
    /// fr fr Window duration in seconds
    pub window_duration: u64,
    
    /// fr fr Algorithm name
    pub algorithm: String,
    
    /// fr fr Store type
    pub store_type: String,
    
    /// fr fr Snapshot timestamp
    pub timestamp: u64,
}

impl ConfigSnapshot {
    /// fr fr Create new config snapshot
    pub fn new() -> Self {
        Self {
            max_requests: 0,
            window_duration: 0,
            algorithm: "Unknown".to_string(),
            store_type: "Unknown".to_string(),
            timestamp: current_timestamp(),
        }
    }

    /// fr fr Create from rate limit config
    pub fn from_config(max_requests: u64, window_duration: u64, algorithm: &str, store_type: &str) -> Self {
        Self {
            max_requests,
            window_duration,
            algorithm: algorithm.to_string(),
            store_type: store_type.to_string(),
            timestamp: current_timestamp(),
        }
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
    pub total_requests: u64,
    
    /// fr fr Success rate percentage
    pub success_rate: f64,
    
    /// fr fr Denial rate percentage
    pub denial_rate: f64,
    
    /// fr fr Error rate percentage
    pub error_rate: f64,
    
    /// fr fr Number of unique clients
    pub unique_clients: usize,
    
    /// fr fr Average response time
    pub avg_response_time: Duration,
    
    /// fr fr Requests per second
    pub requests_per_second: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_metrics_creation() {
        let metrics = RateLimitMetrics::new();
        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.allowed_requests, 0);
        assert_eq!(metrics.denied_requests, 0);
        assert_eq!(metrics.success_rate(), 0.0);
    }

    #[test]
    fn test_metrics_record_decision() {
        let mut metrics = RateLimitMetrics::new();
        
        // Record allow decision
        let allow_decision = RateLimitDecision::Allow {
            remaining: 9,
            reset_time: current_timestamp() + 60,
            retry_after: None,
        };
        metrics.record_decision("client1", &allow_decision);
        
        assert_eq!(metrics.total_requests, 1);
        assert_eq!(metrics.allowed_requests, 1);
        assert_eq!(metrics.denied_requests, 0);
        assert_eq!(metrics.success_rate(), 100.0);
        
        // Record deny decision
        let deny_decision = RateLimitDecision::Deny {
            retry_after: 60,
            reset_time: current_timestamp() + 60,
        };
        metrics.record_decision("client2", &deny_decision);
        
        assert_eq!(metrics.total_requests, 2);
        assert_eq!(metrics.allowed_requests, 1);
        assert_eq!(metrics.denied_requests, 1);
        assert_eq!(metrics.success_rate(), 50.0);
        assert_eq!(metrics.denial_rate(), 50.0);
    }

    #[test]
    fn test_client_metrics() {
        let mut client_metrics = ClientMetrics::new();
        
        let allow_decision = RateLimitDecision::Allow {
            remaining: 9,
            reset_time: current_timestamp() + 60,
            retry_after: None,
        };
        client_metrics.record_decision(&allow_decision);
        
        assert_eq!(client_metrics.total_requests, 1);
        assert_eq!(client_metrics.allowed_requests, 1);
        assert_eq!(client_metrics.denial_rate(), 0.0);
        
        client_metrics.record_error(ErrorCategory::Storage);
        assert_eq!(client_metrics.error_count, 1);
    }

    #[test]
    fn test_algorithm_metrics() {
        let mut algo_metrics = AlgorithmMetrics::new();
        
        algo_metrics.record_execution(Duration::from_millis(10), true);
        algo_metrics.record_execution(Duration::from_millis(20), false);
        
        assert_eq!(algo_metrics.executions, 2);
        assert_eq!(algo_metrics.successes, 1);
        assert_eq!(algo_metrics.failures, 1);
        assert_eq!(algo_metrics.success_rate(), 50.0);
        assert_eq!(algo_metrics.avg_time(), Duration::from_millis(15));
    }

    #[test]
    fn test_store_metrics() {
        let mut store_metrics = StoreMetrics::new();
        
        store_metrics.record_operation("get", Duration::from_millis(5), true);
        store_metrics.record_operation("set", Duration::from_millis(10), true);
        store_metrics.record_operation("get", Duration::from_millis(3), false);
        
        assert_eq!(store_metrics.operations.get("get"), Some(&2));
        assert_eq!(store_metrics.operations.get("set"), Some(&1));
        assert_eq!(store_metrics.success_rate("get"), 50.0);
        assert_eq!(store_metrics.success_rate("set"), 100.0);
    }

    #[test]
    fn test_time_metrics() {
        let mut time_metrics = TimeMetrics::new();
        
        time_metrics.record_request();
        time_metrics.record_request();
        
        assert_eq!(time_metrics.requests_this_minute, 2);
        assert_eq!(time_metrics.requests_this_hour, 2);
        assert_eq!(time_metrics.requests_this_day, 2);
        
        let rps = time_metrics.requests_per_second();
        assert!(rps > 0.0);
    }

    #[test]
    fn test_metrics_summary() {
        let mut metrics = RateLimitMetrics::new();
        
        // Add some data
        let allow_decision = RateLimitDecision::Allow {
            remaining: 9,
            reset_time: current_timestamp() + 60,
            retry_after: None,
        };
        metrics.record_decision("client1", &allow_decision);
        metrics.record_decision("client2", &allow_decision);
        
        let summary = metrics.summary();
        assert_eq!(summary.total_requests, 2);
        assert_eq!(summary.success_rate, 100.0);
        assert_eq!(summary.unique_clients, 2);
    }

    #[test]
    fn test_top_clients() {
        let mut metrics = RateLimitMetrics::new();
        
        let decision = RateLimitDecision::Allow {
            remaining: 9,
            reset_time: current_timestamp() + 60,
            retry_after: None,
        };
        
        // Client1 makes 3 requests
        for _ in 0..3 {
            metrics.record_decision("client1", &decision);
        }
        
        // Client2 makes 1 request
        metrics.record_decision("client2", &decision);
        
        let top_clients = metrics.top_clients(2);
        assert_eq!(top_clients.len(), 2);
        assert_eq!(top_clients[0].0, "client1");
        assert_eq!(top_clients[0].1.total_requests, 3);
    }

    #[test]
    fn test_config_snapshot() {
        let snapshot = ConfigSnapshot::from_config(100, 60, "FixedWindow", "InMemory");
        
        assert_eq!(snapshot.max_requests, 100);
        assert_eq!(snapshot.window_duration, 60);
        assert_eq!(snapshot.algorithm, "FixedWindow");
        assert_eq!(snapshot.store_type, "InMemory");
        assert!(snapshot.timestamp > 0);
    }
}
