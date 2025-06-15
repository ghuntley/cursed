//! Process monitoring and resource management utilities

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use crate::error::CursedError;
use super::{SlayResult, ProcessStats, SlayProcess};

/// Process monitor for tracking resource usage and performance
#[derive(Debug)]
pub struct ProcessMonitor {
    /// Process being monitored
    process: SlayProcess,
    /// Monitoring configuration
    config: MonitorConfig,
    /// Monitoring state
    state: Arc<Mutex<MonitorState>>,
    /// Background monitoring thread handle
    thread_handle: Option<thread::JoinHandle<()>>,
}

/// Configuration for process monitoring
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    /// Monitoring interval
    pub interval: Duration,
    /// CPU usage threshold for alerts
    pub cpu_threshold: Option<f64>,
    /// Memory usage threshold in bytes
    pub memory_threshold: Option<u64>,
    /// Maximum monitoring duration
    pub max_duration: Option<Duration>,
    /// Enable detailed monitoring
    pub detailed_monitoring: bool,
    /// Log statistics to console
    pub log_stats: bool,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            cpu_threshold: None,
            memory_threshold: None,
            max_duration: None,
            detailed_monitoring: false,
            log_stats: false,
        }
    }
}

/// Internal monitoring state
#[derive(Debug)]
struct MonitorState {
    /// Whether monitoring is active
    active: bool,
    /// Start time of monitoring
    start_time: Instant,
    /// Statistics history
    stats_history: Vec<ProcessStats>,
    /// Alert count
    alert_count: usize,
    /// Last alert time
    last_alert: Option<Instant>,
}

impl MonitorState {
    fn new() -> Self {
        Self {
            active: false,
            start_time: Instant::now(),
            stats_history: Vec::new(),
            alert_count: 0,
            last_alert: None,
        }
    }
}

impl ProcessMonitor {
    /// Create a new process monitor
    pub fn new(process: SlayProcess, config: MonitorConfig) -> Self {
        Self {
            process,
            config,
            state: Arc::new(Mutex::new(MonitorState::new())),
            thread_handle: None,
        }
    }

    /// Start monitoring the process
    pub fn start<F>(&mut self, callback: F) -> SlayResult<()>
    where
        F: Fn(&ProcessStats) + Send + 'static,
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
        }

        let process_clone = SlayProcess::new(self.process.state.clone());
        let config = self.config.clone();
        let state_clone = self.state.clone();

        let handle = thread::spawn(move || {
            Self::monitoring_loop(process_clone, config, state_clone, callback);
        });

        self.thread_handle = Some(handle);
        Ok(())
    }

    /// Stop monitoring
    pub fn stop(&mut self) -> SlayResult<()> {
        {
            let mut state = self.state.lock().unwrap();
            state.active = false;
        }

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }

        Ok(())
    }

    /// Check if monitoring is active
    pub fn is_active(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.active
    }

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
        };

        MonitoringStats {
            duration,
            samples,
            avg_cpu,
            avg_memory,
            peak_cpu,
            peak_memory,
            alert_count: state.alert_count,
            last_stats: state.stats_history.last().cloned(),
        }
    }

    /// Get the statistics history
    pub fn get_history(&self) -> Vec<ProcessStats> {
        let state = self.state.lock().unwrap();
        state.stats_history.clone()
    }

    /// Clear statistics history
    pub fn clear_history(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.stats_history.clear();
        state.alert_count = 0;
        state.last_alert = None;
    }

    /// The main monitoring loop
    fn monitoring_loop<F>(
        process: SlayProcess,
        config: MonitorConfig,
        state: Arc<Mutex<MonitorState>>,
        callback: F,
    )
    where
        F: Fn(&ProcessStats),
    {
        let start_time = Instant::now();
        
        loop {
            // Check if we should continue monitoring
            {
                let state_guard = state.lock().unwrap();
                if !state_guard.active {
                    break;
                }
                
                // Check max duration
                if let Some(max_duration) = config.max_duration {
                    if start_time.elapsed() >= max_duration {
                        break;
                    }
                }
            }

            // Check if process is still running
            if !process.is_running() {
                let mut state_guard = state.lock().unwrap();
                state_guard.active = false;
                break;
            }

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
                        }
                        
                        if alert_triggered {
                            state_guard.alert_count += 1;
                            state_guard.last_alert = Some(Instant::now());
                        }
                    }

                    // Log statistics if enabled
                    if config.log_stats {
                        println!("Process Stats - CPU: {:.1}%, Memory: {} MB, Threads: {}", 
                               stats.cpu, stats.memory / 1024 / 1024, stats.thread_count);
                    }

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
}

/// Statistics about the monitoring session
#[derive(Debug, Clone)]
pub struct MonitoringStats {
    /// Total monitoring duration
    pub duration: Duration,
    /// Number of samples collected
    pub samples: usize,
    /// Average CPU usage
    pub avg_cpu: f64,
    /// Average memory usage in bytes
    pub avg_memory: u64,
    /// Peak CPU usage
    pub peak_cpu: f64,
    /// Peak memory usage in bytes
    pub peak_memory: u64,
    /// Number of alerts triggered
    pub alert_count: usize,
    /// Last collected statistics
    pub last_stats: Option<ProcessStats>,
}

impl std::fmt::Display for MonitoringStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
               "MonitoringStats {{ duration: {:?}, samples: {}, avg_cpu: {:.1}%, avg_memory: {} MB, peak_cpu: {:.1}%, peak_memory: {} MB, alerts: {} }}",
               self.duration,
               self.samples,
               self.avg_cpu,
               self.avg_memory / 1024 / 1024,
               self.peak_cpu,
               self.peak_memory / 1024 / 1024,
               self.alert_count
        )
    }
}

/// Resource limiter for processes
#[derive(Debug)]
pub struct ResourceLimiter {
    /// CPU limit as percentage
    cpu_limit: Option<f64>,
    /// Memory limit in bytes
    memory_limit: Option<u64>,
    /// Enable enforcement
    enforce_limits: bool,
}

impl ResourceLimiter {
    /// Create a new resource limiter
    pub fn new() -> Self {
        Self {
            cpu_limit: None,
            memory_limit: None,
            enforce_limits: true,
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
    }

    /// Set memory limit in megabytes
    pub fn with_memory_limit(mut self, memory_mb: u64) -> Self {
        self.memory_limit = Some(memory_mb * 1024 * 1024);
        self
    }

    /// Enable or disable limit enforcement
    pub fn enforce(mut self, enforce: bool) -> Self {
        self.enforce_limits = enforce;
        self
    }

    /// Apply limits to a process
    pub fn apply_to_process(&self, process: &SlayProcess) -> SlayResult<()> {
        if !self.enforce_limits {
            return Ok(());
        }

        // This would need platform-specific implementation
        // For now, just validate the process exists
        if !process.is_running() {
            return Err(CursedError::RuntimeError(
                "Cannot apply limits to non-running process".to_string()
            ));
        }

        // Platform-specific implementation would go here
        // On Unix: ulimit, cgroups, etc.
        // On Windows: Job objects, SetProcessWorkingSetSize, etc.
        
        Ok(())
    }

    /// Check if a process exceeds limits
    pub fn check_limits(&self, stats: &ProcessStats) -> Vec<LimitViolation> {
        let mut violations = Vec::new();

        if let Some(cpu_limit) = self.cpu_limit {
            if stats.cpu > cpu_limit {
                violations.push(LimitViolation {
                    limit_type: LimitType::Cpu,
                    current_value: stats.cpu,
                    limit_value: cpu_limit,
                });
            }
        }

        if let Some(memory_limit) = self.memory_limit {
            if stats.memory > memory_limit {
                violations.push(LimitViolation {
                    limit_type: LimitType::Memory,
                    current_value: stats.memory as f64,
                    limit_value: memory_limit as f64,
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
    Cpu,
    /// Memory usage in bytes
    Memory,
}

/// Represents a resource limit violation
#[derive(Debug, Clone)]
pub struct LimitViolation {
    /// Type of limit violated
    pub limit_type: LimitType,
    /// Current value
    pub current_value: f64,
    /// Limit value
    pub limit_value: f64,
}

impl std::fmt::Display for LimitViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.limit_type {
            LimitType::Cpu => write!(f, "CPU limit violation: {:.1}% > {:.1}%", 
                                   self.current_value, self.limit_value),
            LimitType::Memory => write!(f, "Memory limit violation: {} MB > {} MB", 
                                      self.current_value / 1024.0 / 1024.0,
                                      self.limit_value / 1024.0 / 1024.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::exec_slay::{SlayCommand, SharedProcessState};
    use std::sync::{Arc, Mutex};

    fn create_test_process() -> SlayProcess {
        SlayProcess::new(Arc::new(Mutex::new(SharedProcessState::new())))
    }

    #[test]
    fn test_monitor_config_default() {
        let config = MonitorConfig::default();
        assert_eq!(config.interval, Duration::from_secs(1));
        assert!(config.cpu_threshold.is_none());
        assert!(config.memory_threshold.is_none());
        assert!(!config.detailed_monitoring);
    }

    #[test]
    fn test_process_monitor_creation() {
        let process = create_test_process();
        let config = MonitorConfig::default();
        let monitor = ProcessMonitor::new(process, config);
        
        assert!(!monitor.is_active());
    }

    #[test]
    fn test_monitoring_stats_display() {
        let stats = MonitoringStats {
            duration: Duration::from_secs(60),
            samples: 60,
            avg_cpu: 25.5,
            avg_memory: 100 * 1024 * 1024, // 100 MB
            peak_cpu: 45.2,
            peak_memory: 150 * 1024 * 1024, // 150 MB
            alert_count: 3,
            last_stats: None,
        };
        
        let display_str = stats.to_string();
        assert!(display_str.contains("60"));
        assert!(display_str.contains("25.1%"));
        assert!(display_str.contains("100 MB"));
    }

    #[test]
    fn test_resource_limiter_creation() {
        let limiter = ResourceLimiter::new();
        assert!(limiter.cpu_limit.is_none());
        assert!(limiter.memory_limit.is_none());
        assert!(limiter.enforce_limits);
    }

    #[test]
    fn test_resource_limiter_with_limits() {
        let limiter = ResourceLimiter::new()
            .with_cpu_limit(80.0).unwrap()
            .with_memory_limit(512); // 512 MB
        
        assert_eq!(limiter.cpu_limit, Some(80.0));
        assert_eq!(limiter.memory_limit, Some(512 * 1024 * 1024));
    }

    #[test]
    fn test_resource_limiter_invalid_cpu() {
        let result = ResourceLimiter::new().with_cpu_limit(150.0);
        assert!(result.is_err());
        
        let result = ResourceLimiter::new().with_cpu_limit(-10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_resource_limiter_check_limits() {
        let limiter = ResourceLimiter::new()
            .with_cpu_limit(50.0).unwrap()
            .with_memory_limit(100); // 100 MB
        
        let stats = ProcessStats {
            cpu: 75.0, // Exceeds limit
            memory: 200 * 1024 * 1024, // 200 MB, exceeds limit
            ..Default::default()
        };
        
        let violations = limiter.check_limits(&stats);
        assert_eq!(violations.len(), 2);
        
        assert_eq!(violations[0].limit_type, LimitType::Cpu);
        assert_eq!(violations[1].limit_type, LimitType::Memory);
    }

    #[test]
    fn test_limit_violation_display() {
        let cpu_violation = LimitViolation {
            limit_type: LimitType::Cpu,
            current_value: 75.5,
            limit_value: 50.0,
        };
        
        let display_str = cpu_violation.to_string();
        assert!(display_str.contains("CPU limit violation"));
        assert!(display_str.contains("75.5%"));
        assert!(display_str.contains("50.0%"));
        
        let memory_violation = LimitViolation {
            limit_type: LimitType::Memory,
            current_value: 200.0 * 1024.0 * 1024.0, // 200 MB
            limit_value: 100.0 * 1024.0 * 1024.0,   // 100 MB
        };
        
        let display_str = memory_violation.to_string();
        assert!(display_str.contains("Memory limit violation"));
        assert!(display_str.contains("200"));
        assert!(display_str.contains("100"));
    }

    #[test]
    fn test_monitor_state() {
        let state = MonitorState::new();
        assert!(!state.active);
        assert_eq!(state.stats_history.len(), 0);
        assert_eq!(state.alert_count, 0);
        assert!(state.last_alert.is_none());
    }
}
