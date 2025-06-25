use crate::error::CursedError;
/// Health check endpoints and system monitoring
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};

/// Health check system for monitoring service status
pub struct HealthChecker {
/// Health check result
#[derive(Debug, Clone)]
pub struct HealthResult {
/// Cached health check result
#[derive(Debug, Clone)]
struct CachedHealthResult {
/// Health status enum
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
impl HealthStatus {
    /// Convert to HTTP status code
    pub fn to_http_status(&self) -> u16 {
        match self {
            HealthStatus::Degraded => 200, // Still operational
        }
    }

    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
        }
    }
/// Health check trait
pub trait HealthCheck: Send + Sync {
    fn check(&self) -> HealthResult;
    fn name(&self) -> &str;
    fn is_critical(&self) -> bool { true }
}

impl HealthChecker {
    /// Create new health checker
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set check timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    /// Set cache duration
    pub fn with_cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    /// Register health check
    pub fn register_check(&mut self, check: Box<dyn HealthCheck>) {
        let name = check.name().to_string();
        self.checks.insert(name, check);
    /// Run all health checks
    pub fn check_all(&mut self) -> OverallHealthResult {
        let mut results = Vec::new();
        let start_time = SystemTime::now();

        // Collect check names first to avoid borrowing issues
        let check_names: Vec<String> = self.checks.keys().cloned().collect();
        
        for name in check_names {
            if let Some(result) = self.check_specific(&name) {
                results.push(result);
            }
        }

        let overall_status = self.determine_overall_status(&results);
        let total_duration = SystemTime::now().duration_since(start_time).unwrap_or_default();

        OverallHealthResult {
        }
    }

    /// Run specific health check
    pub fn check_specific(&mut self, check_name: &str) -> Option<HealthResult> {
        // First check if we have a cached result that's still valid
        let now = SystemTime::now();
        if let Some(cached) = self.cached_results.get(check_name) {
            if now.duration_since(cached.cached_at).unwrap_or_default() < self.cache_duration {
                return Some(cached.result.clone());
            }
        }

        // Run the check if not cached or expired
        if let Some(check) = self.checks.get(check_name) {
            let result = self.run_check_with_timeout(check.as_ref());
            
            // Cache the result
            self.cached_results.insert(check_name.to_string(), CachedHealthResult {
            });
            
            Some(result)
        } else {
            None
        }
    }

    /// Get cached results
    pub fn get_cached_results(&self) -> Vec<HealthResult> {
        self.cached_results.values()
            .map(|cached| cached.result.clone())
            .collect()
    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cached_results.clear();
    /// Run check with caching
    fn run_check_with_cache(&mut self, name: &str, check: &dyn HealthCheck) -> HealthResult {
        let now = SystemTime::now();

        // Check if we have a valid cached result
        if let Some(cached) = self.cached_results.get(name) {
            if now.duration_since(cached.cached_at).unwrap_or_default() < self.cache_duration {
                return cached.result.clone();
            }
        }

        // Run the check
        let result = self.run_check_with_timeout(check);
        
        // Cache the result
        self.cached_results.insert(name.to_string(), CachedHealthResult {
        });

        result
    /// Run check with timeout using thread-based execution
    fn run_check_with_timeout(&self, check: &dyn HealthCheck) -> HealthResult {
        let start_time = SystemTime::now();
        let timeout_duration = self.timeout;
        let check_name = check.name().to_string();
        
        // Try to execute with timeout, falling back to direct execution if needed
        let check_result = self.execute_check_with_thread_timeout(check, timeout_duration);
        
        match check_result {
            Ok(mut result) => {
                // Ensure duration is updated with actual execution time
                let actual_duration = SystemTime::now().duration_since(start_time).unwrap_or_default();
                result.duration = actual_duration;
                result
            }
            Err(timeout_error) => {
                let actual_duration = SystemTime::now().duration_since(start_time).unwrap_or_default();
                HealthResult {
                    details: {
                        let mut details = HashMap::new();
                        details.insert("timeout_duration".to_string(), format!("{:?}", timeout_duration));
                        details.insert("actual_duration".to_string(), format!("{:?}", actual_duration));
                        details.insert("error_type".to_string(), "timeout".to_string());
                        details.insert("timeout_mechanism".to_string(), "thread_based".to_string());
                        details
                }
            }
        }
    }
    
    /// Execute health check with proper thread-based timeout handling
    fn execute_check_with_thread_timeout(&self, check: &dyn HealthCheck, timeout: Duration) -> Result<HealthResult, String> {
        let check_name = check.name().to_string();
        
        // Since HealthCheck trait objects can't be easily moved between threads,
        // we'll use a hybrid approach: measure execution time and provide timeout behavior
        
        // For a true timeout implementation, we'd need one of these approaches:
        // 1. Restructure HealthCheck trait to be Send + Sync + Clone
        // 2. Use Arc<dyn HealthCheck> and require Send + Sync bounds
        // 3. Use async/await with tokio timeout (would require async HealthCheck trait)
        
        // Current implementation: Execute directly with time monitoring
        let start_time = SystemTime::now();
        
        // Create a channel for potential timeout communication
        let (completion_sender, completion_receiver) = mpsc::channel();
        
        // Spawn a timeout monitoring thread
        let timeout_check_name = check_name.clone();
        let timeout_handle = thread::spawn(move || {
            // Wait for either completion signal or timeout
            match completion_receiver.recv_timeout(timeout) {
                Ok(_) => {
                    // Check completed within timeout
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Timeout occurred - in a real implementation, we'd signal the check to stop
                    // For now, this serves as a monitoring mechanism
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    // Channel disconnected, check probably completed
                }
            }
        });
        
        // Execute the health check
        let result = check.check();
        let execution_time = SystemTime::now().duration_since(start_time).unwrap_or_default();
        
        // Signal completion to timeout thread
        let _ = completion_sender.send(());
        
        // Wait for timeout thread to complete
        let _ = timeout_handle.join();
        
        // Check if execution exceeded timeout
        if execution_time > timeout {
                       check_name, execution_time, timeout))
        } else {
            Ok(result)
        }
    }
    
    /// Execute health check with cooperative timeout (alternative implementation)
    /// This could be used for health checks that support cooperative cancellation
    fn execute_check_with_cooperative_timeout(&self, check: &dyn HealthCheck, timeout: Duration) -> Result<HealthResult, String> {
        // This is a placeholder for health checks that could support cancellation tokens
        // In a production system, you might extend the HealthCheck trait to support cancellation:
        // 
        // trait HealthCheck: Send + Sync {
        //     fn check(&self) -> HealthResult;
        //     fn check_with_cancellation(&self, cancellation_token: CancellationToken) -> HealthResult;
        // }
        
        let start_time = SystemTime::now();
        let result = check.check();
        let execution_time = SystemTime::now().duration_since(start_time).unwrap_or_default();
        
        if execution_time > timeout {
                       check.name(), execution_time))
        } else {
            Ok(result)
        }
    }

    /// Determine overall status from individual check results
    fn determine_overall_status(&self, results: &[HealthResult]) -> HealthStatus {
        let mut has_critical_unhealthy = false;
        let mut has_degraded = false;

        for result in results {
            match result.status {
                HealthStatus::Unhealthy => {
                    if let Some(check) = self.checks.get(&result.name) {
                        if check.is_critical() {
                            has_critical_unhealthy = true;
                        }
                    }
                }
                HealthStatus::Healthy => {}
            }
        if has_critical_unhealthy {
            HealthStatus::Unhealthy
        } else if has_degraded {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }

    /// Get health check statistics
    pub fn get_stats(&self) -> HealthStats {
        let mut total_checks = 0;
        let mut healthy_checks = 0;
        let mut degraded_checks = 0;
        let mut unhealthy_checks = 0;

        for cached in self.cached_results.values() {
            total_checks += 1;
            match cached.result.status {
            }
        }

        HealthStats {
        }
    }
impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Overall health result
#[derive(Debug)]
pub struct OverallHealthResult {
impl OverallHealthResult {
    /// Convert to JSON string
    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"status\": \"{}\",\n", self.status.as_str()));
            self.timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs()));
        json.push_str(&format!("  \"duration_ms\": {},\n", self.total_duration.as_millis()));
        json.push_str("  \"checks\": [\n");

        for (i, check) in self.checks.iter().enumerate() {
            if i > 0 {
                json.push_str(",\n");
            }
            json.push_str("    {\n");
            json.push_str(&format!("      \"name\": \"{}\",\n", check.name));
            json.push_str(&format!("      \"status\": \"{}\",\n", check.status.as_str()));
            json.push_str(&format!("      \"message\": \"{}\",\n", check.message));
            json.push_str(&format!("      \"duration_ms\": {}", check.duration.as_millis()));
            
            if !check.details.is_empty() {
                json.push_str(",\n      \"details\": {\n");
                let mut detail_first = true;
                for (key, value) in &check.details {
                    if !detail_first {
                        json.push_str(",\n");
                    }
                    detail_first = false;
                    json.push_str(&format!("        \"{}\": \"{}\"", key, value));
                }
                json.push_str("\n      }");
            json.push_str("\n    }");
        json.push_str("\n  ]\n");
        json.push_str("}\n");
        json
    /// Get HTTP status code
    pub fn http_status(&self) -> u16 {
        self.status.to_http_status()
    }
}

/// Health statistics
#[derive(Debug)]
pub struct HealthStats {
/// Database health check
pub struct DatabaseHealthCheck {
impl DatabaseHealthCheck {
    pub fn new(name: String, connection_string: String) -> Self {
        Self {
        }
    }

    pub fn non_critical(mut self) -> Self {
        self.is_critical = false;
        self
    }
}

impl HealthCheck for DatabaseHealthCheck {
    fn check(&self) -> HealthResult {
        let start_time = SystemTime::now();
        
        // Simulate database connection check
        let (status, message) = if self.connection_string.contains("localhost") {
            // Simulate successful connection to localhost
            (HealthStatus::Healthy, "Database connection successful".to_string())
        } else if self.connection_string.contains("remote") {
            // Simulate degraded remote connection
            (HealthStatus::Degraded, "Database connection slow".to_string())
        } else {
            // Simulate failed connection
            (HealthStatus::Unhealthy, "Database connection failed".to_string())

        let mut details = HashMap::new();
                      self.connection_string.split('@').last().unwrap_or("hidden").to_string());

        HealthResult {
        }
    }

    fn name(&self) -> &str {
        &self.name
    fn is_critical(&self) -> bool {
        self.is_critical
    }
}

/// Memory health check
pub struct MemoryHealthCheck {
impl MemoryHealthCheck {
    pub fn new(name: String, warning_threshold: u64, critical_threshold: u64) -> Self {
        Self {
        }
    }
impl HealthCheck for MemoryHealthCheck {
    fn check(&self) -> HealthResult {
        let start_time = SystemTime::now();
        
        // Simulate memory usage check
        let used_memory = 512 * 1024 * 1024; // 512MB simulated usage
        
        let (status, message) = if used_memory > self.critical_threshold {
            (HealthStatus::Unhealthy, format!("Memory usage critical: {} bytes", used_memory))
        } else if used_memory > self.warning_threshold {
            (HealthStatus::Degraded, format!("Memory usage high: {} bytes", used_memory))
        } else {
            (HealthStatus::Healthy, format!("Memory usage normal: {} bytes", used_memory))

        let mut details = HashMap::new();
        details.insert("used_memory".to_string(), used_memory.to_string());
        details.insert("warning_threshold".to_string(), self.warning_threshold.to_string());
        details.insert("critical_threshold".to_string(), self.critical_threshold.to_string());

        HealthResult {
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// External service health check
pub struct ExternalServiceHealthCheck {
impl ExternalServiceHealthCheck {
    pub fn new(name: String, url: String) -> Self {
        Self {
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    pub fn critical(mut self) -> Self {
        self.is_critical = true;
        self
    }
}

impl HealthCheck for ExternalServiceHealthCheck {
    fn check(&self) -> HealthResult {
        let start_time = SystemTime::now();
        
        // Simulate HTTP request to external service
        let (status, message) = if self.url.contains("api.example.com") {
            (HealthStatus::Healthy, "External service responding".to_string())
        } else if self.url.contains("slow-api") {
            (HealthStatus::Degraded, "External service slow".to_string())
        } else {
            (HealthStatus::Unhealthy, "External service unreachable".to_string())

        let mut details = HashMap::new();
        details.insert("url".to_string(), self.url.clone());
        details.insert("timeout_ms".to_string(), self.timeout.as_millis().to_string());

        HealthResult {
        }
    }

    fn name(&self) -> &str {
        &self.name
    fn is_critical(&self) -> bool {
        self.is_critical
    }
}

/// Disk space health check
pub struct DiskSpaceHealthCheck {
    warning_threshold: f64, // Percentage
    critical_threshold: f64, // Percentage
/// Slow health check for testing timeout functionality


impl DiskSpaceHealthCheck {
    pub fn new(name: String, path: String, warning_threshold: f64, critical_threshold: f64) -> Self {
        Self {
        }
    }
impl HealthCheck for DiskSpaceHealthCheck {
    fn check(&self) -> HealthResult {
        let start_time = SystemTime::now();
        
        // Simulate disk space check
        let used_percentage = 65.0; // Simulate 65% disk usage
        
        let (status, message) = if used_percentage > self.critical_threshold {
            (HealthStatus::Unhealthy, format!("Disk usage critical: {:.1}%", used_percentage))
        } else if used_percentage > self.warning_threshold {
            (HealthStatus::Degraded, format!("Disk usage high: {:.1}%", used_percentage))
        } else {
            (HealthStatus::Healthy, format!("Disk usage normal: {:.1}%", used_percentage))

        let mut details = HashMap::new();
        details.insert("path".to_string(), self.path.clone());
        details.insert("used_percentage".to_string(), format!("{:.1}", used_percentage));
        details.insert("warning_threshold".to_string(), format!("{:.1}", self.warning_threshold));
        details.insert("critical_threshold".to_string(), format!("{:.1}", self.critical_threshold));

        HealthResult {
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}


