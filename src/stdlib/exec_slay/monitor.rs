use crate::error::CursedError;
// Process monitoring and resource management utilities

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use super::{SlayResult, ProcessStats, SlayProcess};

/// Process monitor for tracking resource usage and performance
#[derive(Debug)]
pub struct ProcessMonitor {
    /// Process being monitored
    /// Monitoring configuration
    /// Monitoring state
    /// Background monitoring thread handle
/// Configuration for process monitoring
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    /// Monitoring interval
    /// CPU usage threshold for alerts
    /// Memory usage threshold in bytes
    /// Maximum monitoring duration
    /// Enable detailed monitoring
    /// Log statistics to console
impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Internal monitoring state
#[derive(Debug)]
struct MonitorState {
    /// Whether monitoring is active
    /// Start time of monitoring
    /// Statistics history
    /// Alert count
    /// Last alert time
impl MonitorState {
    fn new() -> Self {
        Self {
        }
    }
impl ProcessMonitor {
    /// Create a new process monitor
    pub fn new(process: SlayProcess, config: MonitorConfig) -> Self {
        Self {
        }
    }

    /// Start monitoring the process
    pub fn start<F>(&mut self, callback: F) -> SlayResult<()>
    where
    {
        {
            let mut state = self.state.lock().unwrap();
            if state.active {
                return Err(CursedError::RuntimeError(
                    "Monitoring is already active".to_string()
                ));
            }
            state.active = true;
            state.start_time = Instant::now();
        let process_clone = SlayProcess::new(self.process.state.clone());
        let config = self.config.clone();
        let state_clone = self.state.clone();

        let handle = thread::spawn(move || {
            Self::monitoring_loop(process_clone, config, state_clone, callback);
        });

        self.thread_handle = Some(handle);
        Ok(())
    /// Stop monitoring
    pub fn stop(&mut self) -> SlayResult<()> {
        {
            let mut state = self.state.lock().unwrap();
            state.active = false;
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        Ok(())
    /// Check if monitoring is active
    pub fn is_active(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.active
    /// Get monitoring statistics
    pub fn get_stats(&self) -> MonitoringStats {
        let state = self.state.lock().unwrap();
        
        let duration = state.start_time.elapsed();
        let samples = state.stats_history.len();
        
        let (avg_cpu, avg_memory, peak_cpu, peak_memory) = if samples > 0 {
            let avg_cpu = state.stats_history.iter().map(|s| s.cpu).sum::<f64>() / samples as f64;
            let avg_memory = state.stats_history.iter().map(|s| s.memory).sum::<u64>() / samples as u64;
            let peak_cpu = state.stats_history.iter().map(|s| s.cpu).fold(0.0, f64::max);
            let peak_memory = state.stats_history.iter().map(|s| s.memory).max().unwrap_or(0);
            (avg_cpu, avg_memory, peak_cpu, peak_memory)
        } else {
            (0.0, 0, 0.0, 0)

        MonitoringStats {
        }
    }

    /// Get the statistics history
    pub fn get_history(&self) -> Vec<ProcessStats> {
        let state = self.state.lock().unwrap();
        state.stats_history.clone()
    /// Clear statistics history
    pub fn clear_history(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.stats_history.clear();
        state.alert_count = 0;
        state.last_alert = None;
    /// The main monitoring loop
    fn monitoring_loop<F>(
    )
    where
    {
        let start_time = Instant::now();
        
        loop {
            // Check if we should continue monitoring
            {
                let state_guard = state.lock().unwrap();
                if !state_guard.active {
                    break;
                // Check max duration
                if let Some(max_duration) = config.max_duration {
                    if start_time.elapsed() >= max_duration {
                        break;
                    }
                }
            // Check if process is still running
            if !process.is_running() {
                let mut state_guard = state.lock().unwrap();
                state_guard.active = false;
                break;
            // Get process statistics
            match process.stats() {
                Ok(stats) => {
                    // Check thresholds and generate alerts
                    let mut alert_triggered = false;
                    
                    if let Some(cpu_threshold) = config.cpu_threshold {
                        if stats.cpu > cpu_threshold {
                            alert_triggered = true;
                        }
                    }
                    
                    if let Some(memory_threshold) = config.memory_threshold {
                        if stats.memory > memory_threshold {
                            alert_triggered = true;
                        }
                    }

                    // Update state
                    {
                        let mut state_guard = state.lock().unwrap();
                        state_guard.stats_history.push(stats.clone());
                        
                        // Limit history size
                        if state_guard.stats_history.len() > 1000 {
                            state_guard.stats_history.remove(0);
                        if alert_triggered {
                            state_guard.alert_count += 1;
                            state_guard.last_alert = Some(Instant::now());
                        }
                    }

                    // Log statistics if enabled
                    if config.log_stats {
                               stats.cpu, stats.memory / 1024 / 1024, stats.thread_count);
                    // Call the callback
                    callback(&stats);
                }
                Err(_) => {
                    // Failed to get stats, process might have terminated
                    let mut state_guard = state.lock().unwrap();
                    state_guard.active = false;
                    break;
                }
            }

            thread::sleep(config.interval);
        }
    }
/// Statistics about the monitoring session
#[derive(Debug, Clone)]
pub struct MonitoringStats {
    /// Total monitoring duration
    /// Number of samples collected
    /// Average CPU usage
    /// Average memory usage in bytes
    /// Peak CPU usage
    /// Peak memory usage in bytes
    /// Number of alerts triggered
    /// Last collected statistics
impl std::fmt::Display for MonitoringStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               self.avg_memory / 1024 / 1024,
               self.peak_memory / 1024 / 1024,
               self.alert_count
        )
    }
}

/// Resource limiter for processes
#[derive(Debug)]
pub struct ResourceLimiter {
    /// CPU limit as percentage
    /// Memory limit in bytes
    /// Enable enforcement
impl ResourceLimiter {
    /// Create a new resource limiter
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set CPU limit as percentage
    pub fn with_cpu_limit(mut self, cpu_percent: f64) -> SlayResult<Self> {
        if cpu_percent <= 0.0 || cpu_percent > 100.0 {
            return Err(CursedError::RuntimeError(
                "CPU limit must be between 0 and 100".to_string()
            ));
        }
        self.cpu_limit = Some(cpu_percent);
        Ok(self)
    /// Set memory limit in megabytes
    pub fn with_memory_limit(mut self, memory_mb: u64) -> Self {
        self.memory_limit = Some(memory_mb * 1024 * 1024);
        self
    /// Enable or disable limit enforcement
    pub fn enforce(mut self, enforce: bool) -> Self {
        self.enforce_limits = enforce;
        self
    /// Apply limits to a process
    pub fn apply_to_process(&self, process: &SlayProcess) -> SlayResult<()> {
        if !self.enforce_limits {
            return Ok(());
        // This would need platform-specific implementation
        // For now, just validate the process exists
        if !process.is_running() {
            return Err(CursedError::RuntimeError(
                "Cannot apply limits to non-running process".to_string()
            ));
        // Platform-specific implementation would go here
        // On Unix: ulimit, cgroups, etc.
        // On Windows: Job objects, SetProcessWorkingSetSize, etc.
        
        Ok(())
    /// Check if a process exceeds limits
    pub fn check_limits(&self, stats: &ProcessStats) -> Vec<LimitViolation> {
        let mut violations = Vec::new();

        if let Some(cpu_limit) = self.cpu_limit {
            if stats.cpu > cpu_limit {
                violations.push(LimitViolation {
                });
            }
        }

        if let Some(memory_limit) = self.memory_limit {
            if stats.memory > memory_limit {
                violations.push(LimitViolation {
                });
            }
        }

        violations
    }
}

impl Default for ResourceLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of resource limits
#[derive(Debug, Clone, PartialEq)]
pub enum LimitType {
    /// CPU usage percentage
    /// Memory usage in bytes
/// Represents a resource limit violation
#[derive(Debug, Clone)]
pub struct LimitViolation {
    /// Type of limit violated
    /// Current value
    /// Limit value
impl std::fmt::Display for LimitViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.limit_type {
                                      self.current_value / 1024.0 / 1024.0,
                                      self.limit_value / 1024.0 / 1024.0),
        }
    }
