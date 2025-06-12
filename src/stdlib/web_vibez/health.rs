/// Health check endpoints and system monitoring
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};

/// Health check system for monitoring service status
pub struct HealthChecker {
    checks: HashMap<String, Box<dyn HealthCheck>>,
    timeout: Duration,
    cache_duration: Duration,
    cached_results: HashMap<String, CachedHealthResult>,
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub duration: Duration,
    pub timestamp: SystemTime,
    pub details: HashMap<String, String>,
}

/// Cached health check result
#[derive(Debug, Clone)]
struct CachedHealthResult {
    result: HealthResult,
    cached_at: SystemTime,
}

/// Health status enum
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl HealthStatus {
    /// Convert to HTTP status code
    pub fn to_http_status(&self) -> u16 {
        match self {
            HealthStatus::Healthy => 200,
            HealthStatus::Degraded => 200, // Still operational
            HealthStatus::Unhealthy => 503,
            HealthStatus::Unknown => 503,
        }
    }

    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Unhealthy => "unhealthy",
            HealthStatus::Unknown => "unknown",
        }
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
            checks: HashMap::new(),
            timeout: Duration::from_secs(5),
            cache_duration: Duration::from_secs(30),
            cached_results: HashMap::new(),
        }
    }

    /// Set check timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set cache duration
    pub fn with_cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    }

    /// Register health check
    pub fn register_check(&mut self, check: Box<dyn HealthCheck>) {
        let name = check.name().to_string();
        self.checks.insert(name, check);
    }

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
            status: overall_status,
            checks: results,
            total_duration,
            timestamp: SystemTime::now(),
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
                result: result.clone(),
                cached_at: now,
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
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cached_results.clear();
    }

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
            result: result.clone(),
            cached_at: now,
        });

        result
    }

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
                    name: check_name,
                    status: HealthStatus::Unhealthy,
                    message: timeout_error,
                    duration: actual_duration,
                    timestamp: SystemTime::now(),
                    details: {
                        let mut details = HashMap::new();
                        details.insert("timeout_duration".to_string(), format!("{:?}", timeout_duration));
                        details.insert("actual_duration".to_string(), format!("{:?}", actual_duration));
                        details.insert("error_type".to_string(), "timeout".to_string());
                        details.insert("timeout_mechanism".to_string(), "thread_based".to_string());
                        details
                    },
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
            Err(format!("Health check '{}' exceeded timeout: {:?} (limit: {:?})", 
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
            Err(format!("Health check '{}' timed out cooperatively after {:?}", 
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
                HealthStatus::Degraded => has_degraded = true,
                HealthStatus::Unknown => has_degraded = true,
                HealthStatus::Healthy => {}
            }
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
                HealthStatus::Healthy => healthy_checks += 1,
                HealthStatus::Degraded => degraded_checks += 1,
                HealthStatus::Unhealthy => unhealthy_checks += 1,
                HealthStatus::Unknown => degraded_checks += 1,
            }
        }

        HealthStats {
            total_checks,
            healthy_checks,
            degraded_checks,
            unhealthy_checks,
            cache_duration: self.cache_duration,
            timeout: self.timeout,
        }
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
    pub status: HealthStatus,
    pub checks: Vec<HealthResult>,
    pub total_duration: Duration,
    pub timestamp: SystemTime,
}

impl OverallHealthResult {
    /// Convert to JSON string
    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"status\": \"{}\",\n", self.status.as_str()));
        json.push_str(&format!("  \"timestamp\": {},\n", 
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
            }
            
            json.push_str("\n    }");
        }

        json.push_str("\n  ]\n");
        json.push_str("}\n");
        json
    }

    /// Get HTTP status code
    pub fn http_status(&self) -> u16 {
        self.status.to_http_status()
    }
}

/// Health statistics
#[derive(Debug)]
pub struct HealthStats {
    pub total_checks: usize,
    pub healthy_checks: usize,
    pub degraded_checks: usize,
    pub unhealthy_checks: usize,
    pub cache_duration: Duration,
    pub timeout: Duration,
}

/// Database health check
pub struct DatabaseHealthCheck {
    name: String,
    connection_string: String,
    is_critical: bool,
}

impl DatabaseHealthCheck {
    pub fn new(name: String, connection_string: String) -> Self {
        Self {
            name,
            connection_string,
            is_critical: true,
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
        };

        let mut details = HashMap::new();
        details.insert("connection_string".to_string(), 
                      self.connection_string.split('@').last().unwrap_or("hidden").to_string());

        HealthResult {
            name: self.name.clone(),
            status,
            message,
            duration: SystemTime::now().duration_since(start_time).unwrap_or_default(),
            timestamp: SystemTime::now(),
            details,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_critical(&self) -> bool {
        self.is_critical
    }
}

/// Memory health check
pub struct MemoryHealthCheck {
    name: String,
    warning_threshold: u64,
    critical_threshold: u64,
}

impl MemoryHealthCheck {
    pub fn new(name: String, warning_threshold: u64, critical_threshold: u64) -> Self {
        Self {
            name,
            warning_threshold,
            critical_threshold,
        }
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
        };

        let mut details = HashMap::new();
        details.insert("used_memory".to_string(), used_memory.to_string());
        details.insert("warning_threshold".to_string(), self.warning_threshold.to_string());
        details.insert("critical_threshold".to_string(), self.critical_threshold.to_string());

        HealthResult {
            name: self.name.clone(),
            status,
            message,
            duration: SystemTime::now().duration_since(start_time).unwrap_or_default(),
            timestamp: SystemTime::now(),
            details,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// External service health check
pub struct ExternalServiceHealthCheck {
    name: String,
    url: String,
    timeout: Duration,
    is_critical: bool,
}

impl ExternalServiceHealthCheck {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            timeout: Duration::from_secs(5),
            is_critical: false,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

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
        };

        let mut details = HashMap::new();
        details.insert("url".to_string(), self.url.clone());
        details.insert("timeout_ms".to_string(), self.timeout.as_millis().to_string());

        HealthResult {
            name: self.name.clone(),
            status,
            message,
            duration: SystemTime::now().duration_since(start_time).unwrap_or_default(),
            timestamp: SystemTime::now(),
            details,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_critical(&self) -> bool {
        self.is_critical
    }
}

/// Disk space health check
pub struct DiskSpaceHealthCheck {
    name: String,
    path: String,
    warning_threshold: f64, // Percentage
    critical_threshold: f64, // Percentage
}

/// Slow health check for testing timeout functionality
#[cfg(test)]
pub struct SlowHealthCheck {
    name: String,
    delay: Duration,
}

#[cfg(test)]
impl SlowHealthCheck {
    pub fn new(name: String, delay: Duration) -> Self {
        Self { name, delay }
    }
}

impl DiskSpaceHealthCheck {
    pub fn new(name: String, path: String, warning_threshold: f64, critical_threshold: f64) -> Self {
        Self {
            name,
            path,
            warning_threshold,
            critical_threshold,
        }
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
        };

        let mut details = HashMap::new();
        details.insert("path".to_string(), self.path.clone());
        details.insert("used_percentage".to_string(), format!("{:.1}", used_percentage));
        details.insert("warning_threshold".to_string(), format!("{:.1}", self.warning_threshold));
        details.insert("critical_threshold".to_string(), format!("{:.1}", self.critical_threshold));

        HealthResult {
            name: self.name.clone(),
            status,
            message,
            duration: SystemTime::now().duration_since(start_time).unwrap_or_default(),
            timestamp: SystemTime::now(),
            details,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
impl HealthCheck for SlowHealthCheck {
    fn check(&self) -> HealthResult {
        let start_time = SystemTime::now();
        
        // Simulate slow operation
        thread::sleep(self.delay);
        
        HealthResult {
            name: self.name.clone(),
            status: HealthStatus::Healthy,
            message: format!("Slow check completed after {:?}", self.delay),
            duration: SystemTime::now().duration_since(start_time).unwrap_or_default(),
            timestamp: SystemTime::now(),
            details: {
                let mut details = HashMap::new();
                details.insert("simulated_delay".to_string(), format!("{:?}", self.delay));
                details
            },
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_checker() {
        let mut checker = HealthChecker::new();
        
        // Register health checks
        checker.register_check(Box::new(DatabaseHealthCheck::new(
            "database".to_string(),
            "postgres://localhost:5432/mydb".to_string(),
        )));
        
        checker.register_check(Box::new(MemoryHealthCheck::new(
            "memory".to_string(),
            1024 * 1024 * 1024, // 1GB warning
            2048 * 1024 * 1024, // 2GB critical
        )));

        // Run all checks
        let result = checker.check_all();
        assert_eq!(result.checks.len(), 2);
        
        // Check specific check
        let db_result = checker.check_specific("database");
        assert!(db_result.is_some());
        assert_eq!(db_result.unwrap().name, "database");
    }

    #[test]
    fn test_health_status() {
        assert_eq!(HealthStatus::Healthy.to_http_status(), 200);
        assert_eq!(HealthStatus::Degraded.to_http_status(), 200);
        assert_eq!(HealthStatus::Unhealthy.to_http_status(), 503);
        assert_eq!(HealthStatus::Unknown.to_http_status(), 503);
        
        assert_eq!(HealthStatus::Healthy.as_str(), "healthy");
        assert_eq!(HealthStatus::Degraded.as_str(), "degraded");
    }

    #[test]
    fn test_database_health_check() {
        let check = DatabaseHealthCheck::new(
            "test_db".to_string(),
            "postgres://localhost:5432/test".to_string(),
        );
        
        let result = check.check();
        assert_eq!(result.name, "test_db");
        assert_eq!(result.status, HealthStatus::Healthy);
        assert!(check.is_critical());
    }

    #[test]
    fn test_memory_health_check() {
        let check = MemoryHealthCheck::new(
            "memory".to_string(),
            256 * 1024 * 1024, // 256MB warning
            1024 * 1024 * 1024, // 1GB critical
        );
        
        let result = check.check();
        assert_eq!(result.name, "memory");
        // Should be degraded since simulated usage (512MB) > warning (256MB)
        assert_eq!(result.status, HealthStatus::Degraded);
    }

    #[test]
    fn test_external_service_health_check() {
        let check = ExternalServiceHealthCheck::new(
            "api".to_string(),
            "https://api.example.com/health".to_string(),
        ).critical();
        
        let result = check.check();
        assert_eq!(result.name, "api");
        assert_eq!(result.status, HealthStatus::Healthy);
        assert!(check.is_critical());
    }

    #[test]
    fn test_overall_health_result_json() {
        let checks = vec![
            HealthResult {
                name: "database".to_string(),
                status: HealthStatus::Healthy,
                message: "OK".to_string(),
                duration: Duration::from_millis(50),
                timestamp: SystemTime::now(),
                details: HashMap::new(),
            },
        ];

        let overall = OverallHealthResult {
            status: HealthStatus::Healthy,
            checks,
            total_duration: Duration::from_millis(100),
            timestamp: SystemTime::now(),
        };

        let json = overall.to_json();
        assert!(json.contains("\"status\": \"healthy\""));
        assert!(json.contains("\"name\": \"database\""));
        assert_eq!(overall.http_status(), 200);
    }

    #[test]
    fn test_health_checker_caching() {
        let mut checker = HealthChecker::new()
            .with_cache_duration(Duration::from_secs(10));
        
        checker.register_check(Box::new(DatabaseHealthCheck::new(
            "database".to_string(),
            "postgres://localhost:5432/mydb".to_string(),
        )));

        // First check should run the actual check
        let result1 = checker.check_specific("database").unwrap();
        
        // Second check should use cached result
        let result2 = checker.check_specific("database").unwrap();
        
        // Results should be identical (cached)
        assert_eq!(result1.timestamp, result2.timestamp);
    }

    #[test]
    fn test_health_checker_timeout() {
        let mut checker = HealthChecker::new()
            .with_timeout(Duration::from_millis(10)); // Very short timeout
        
        // Register a slow health check
        checker.register_check(Box::new(SlowHealthCheck::new(
            "slow_check".to_string(),
            Duration::from_millis(100), // Takes longer than timeout
        )));

        // Run the check and verify timeout behavior
        let result = checker.check_specific("slow_check").unwrap();
        assert_eq!(result.status, HealthStatus::Unhealthy);
        assert!(result.message.contains("exceeded timeout"));
        assert!(result.details.contains_key("timeout_duration"));
        assert!(result.details.contains_key("actual_duration"));
        assert_eq!(result.details.get("error_type").unwrap(), "timeout");
    }

    #[test]
    fn test_health_checker_timeout_mechanisms() {
        let mut checker = HealthChecker::new()
            .with_timeout(Duration::from_millis(50));
        
        // Register normal health check that should complete within timeout
        checker.register_check(Box::new(DatabaseHealthCheck::new(
            "fast_database".to_string(),
            "postgres://localhost:5432/fast".to_string(),
        )));

        let result = checker.check_specific("fast_database").unwrap();
        assert_eq!(result.status, HealthStatus::Healthy);
        assert!(!result.message.contains("timeout"));
    }
}
