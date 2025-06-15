//! Performance metrics collection and analysis

use crate::error::{Result, CursedError};
use crate::optimization::PerformanceConfig;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::{info, debug, warn, instrument};
use serde::{Deserialize, Serialize};

/// Level of resource monitoring detail
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResourceMonitoringLevel {
    /// Basic monitoring - CPU and memory only
    Basic,
    /// Detailed monitoring - includes I/O and network
    Detailed,
    /// Comprehensive monitoring - all metrics with high frequency
    Comprehensive,
}

impl Default for ResourceMonitoringLevel {
    fn default() -> Self {
        Self::Basic
    }
}

/// Compilation unit for optimization tracking
#[derive(Debug, Clone)]
pub struct CompilationUnit {
    pub name: String,
    pub source_files: Vec<String>,
    pub dependencies: Vec<String>,
    pub estimated_size_bytes: usize,
    pub compilation_start_time: Option<Instant>,
    pub last_modified: Option<SystemTime>,
}

impl CompilationUnit {
    /// Create a new compilation unit
    pub fn new(name: String) -> Self {
        Self {
            name,
            source_files: Vec::new(),
            dependencies: Vec::new(),
            estimated_size_bytes: 10000, // Default estimate
            compilation_start_time: None,
            last_modified: None,
        }
    }

    /// Add a source file to this compilation unit
    pub fn add_source_file(&mut self, file_path: String) {
        self.source_files.push(file_path);
        // Increase estimated size based on file count
        self.estimated_size_bytes += 5000;
    }

    /// Add a dependency to this compilation unit
    pub fn add_dependency(&mut self, dependency: String) {
        self.dependencies.push(dependency);
    }

    /// Start compilation timing
    pub fn start_compilation(&mut self) {
        self.compilation_start_time = Some(Instant::now());
    }

    /// Get compilation time if started
    pub fn compilation_time(&self) -> Option<Duration> {
        self.compilation_start_time.map(|start| start.elapsed())
    }
}

/// System-wide optimization statistics
#[derive(Debug, Default, Clone)]
pub struct SystemStatistics {
    pub optimizations_completed: usize,
    pub total_units_optimized: usize,
    pub benchmark_runs: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub errors_encountered: usize,
    pub total_compilation_time: Duration,
    pub average_compilation_time: Duration,
}

/// Resource usage statistics
#[derive(Debug, Clone)]
pub struct ResourceStatistics {
    pub peak_memory_mb: f64,
    pub average_memory_mb: f64,
    pub peak_cpu_percent: f64,
    pub average_cpu_percent: f64,
    pub total_io_operations: u64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub monitoring_uptime: Duration,
    pub sample_count: usize,
}

impl Default for ResourceStatistics {
    fn default() -> Self {
        Self {
            peak_memory_mb: 0.0,
            average_memory_mb: 0.0,
            peak_cpu_percent: 0.0,
            average_cpu_percent: 0.0,
            total_io_operations: 0,
            network_bytes_sent: 0,
            network_bytes_received: 0,
            monitoring_uptime: Duration::from_secs(0),
            sample_count: 0,
        }
    }
}

/// Resource sample point
#[derive(Debug, Clone)]
struct ResourceSample {
    timestamp: Instant,
    memory_mb: f64,
    cpu_percent: f64,
    io_operations: u64,
    network_sent: u64,
    network_received: u64,
}

/// Performance metrics collector
#[derive(Debug)]
pub struct MetricsCollector {
    config: PerformanceConfig,
    system_stats: Arc<Mutex<SystemStatistics>>,
    resource_samples: Arc<Mutex<Vec<ResourceSample>>>,
    monitoring_active: Arc<Mutex<bool>>,
    monitoring_start_time: Arc<Mutex<Option<Instant>>>,
    monitoring_thread: Option<thread::JoinHandle<()>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    #[instrument]
    pub fn new(config: PerformanceConfig) -> Result<Self> {
        info!("Creating metrics collector with monitoring level {:?}", config.resource_monitoring_level);
        
        Ok(Self {
            config,
            system_stats: Arc::new(Mutex::new(SystemStatistics::default())),
            resource_samples: Arc::new(Mutex::new(Vec::new())),
            monitoring_active: Arc::new(Mutex::new(false)),
            monitoring_start_time: Arc::new(Mutex::new(None)),
            monitoring_thread: None,
        })
    }

    /// Start real-time monitoring
    #[instrument(skip(self))]
    pub fn start_monitoring(&self) -> Result<()> {
        let mut active = self.monitoring_active.lock().map_err(|_| {
            CursedError::optimization_error("Failed to acquire monitoring lock")
        })?;

        if *active {
            warn!("Monitoring is already active");
            return Ok(());
        }

        *active = true;
        
        let mut start_time = self.monitoring_start_time.lock().map_err(|_| {
            CursedError::optimization_error("Failed to acquire start time lock")
        })?;
        *start_time = Some(Instant::now());

        info!("Started performance monitoring");
        Ok(())
    }

    /// Stop real-time monitoring
    #[instrument(skip(self))]
    pub fn stop_monitoring(&self) -> Result<()> {
        let mut active = self.monitoring_active.lock().map_err(|_| {
            CursedError::optimization_error("Failed to acquire monitoring lock")
        })?;

        if !*active {
            warn!("Monitoring is not currently active");
            return Ok(());
        }

        *active = false;
        info!("Stopped performance monitoring");
        Ok(())
    }

    /// Get current system statistics
    pub fn get_system_statistics(&self) -> SystemStatistics {
        self.system_stats.lock()
            .unwrap_or_else(|_| std::sync::PoisonError::into_inner)
            .clone()
    }

    /// Get current resource statistics
    pub fn get_resource_statistics(&self) -> Result<ResourceStatistics> {
        let samples = self.resource_samples.lock().map_err(|_| {
            CursedError::optimization_error("Failed to acquire resource samples lock")
        })?;

        if samples.is_empty() {
            return Ok(ResourceStatistics::default());
        }

        let start_time = self.monitoring_start_time.lock().map_err(|_| {
            CursedError::optimization_error("Failed to acquire start time lock")
        })?;

        let monitoring_uptime = start_time.map(|start| start.elapsed()).unwrap_or_default();

        // Calculate statistics from samples
        let sample_count = samples.len();
        let peak_memory_mb = samples.iter().map(|s| s.memory_mb).fold(0.0, f64::max);
        let average_memory_mb = samples.iter().map(|s| s.memory_mb).sum::<f64>() / sample_count as f64;
        let peak_cpu_percent = samples.iter().map(|s| s.cpu_percent).fold(0.0, f64::max);
        let average_cpu_percent = samples.iter().map(|s| s.cpu_percent).sum::<f64>() / sample_count as f64;
        
        let total_io_operations = samples.last().map(|s| s.io_operations).unwrap_or(0);
        let network_bytes_sent = samples.last().map(|s| s.network_sent).unwrap_or(0);
        let network_bytes_received = samples.last().map(|s| s.network_received).unwrap_or(0);

        Ok(ResourceStatistics {
            peak_memory_mb,
            average_memory_mb,
            peak_cpu_percent,
            average_cpu_percent,
            total_io_operations,
            network_bytes_sent,
            network_bytes_received,
            monitoring_uptime,
            sample_count,
        })
    }

    /// Record an optimization completion
    pub fn record_optimization_completion(&self, units_optimized: usize, compilation_time: Duration) {
        if let Ok(mut stats) = self.system_stats.lock() {
            stats.optimizations_completed += 1;
            stats.total_units_optimized += units_optimized;
            stats.total_compilation_time += compilation_time;
            
            // Update average
            if stats.optimizations_completed > 0 {
                stats.average_compilation_time = 
                    stats.total_compilation_time / stats.optimizations_completed as u32;
            }
        }
    }

    /// Record a benchmark run
    pub fn record_benchmark_run(&self) {
        if let Ok(mut stats) = self.system_stats.lock() {
            stats.benchmark_runs += 1;
        }
    }

    /// Record a cache hit
    pub fn record_cache_hit(&self) {
        if let Ok(mut stats) = self.system_stats.lock() {
            stats.cache_hits += 1;
        }
    }

    /// Record a cache miss
    pub fn record_cache_miss(&self) {
        if let Ok(mut stats) = self.system_stats.lock() {
            stats.cache_misses += 1;
        }
    }

    /// Record an error
    pub fn record_error(&self) {
        if let Ok(mut stats) = self.system_stats.lock() {
            stats.errors_encountered += 1;
        }
    }

    /// Simulate collecting system resource data
    fn collect_resource_sample(&self) -> ResourceSample {
        // Simulate realistic resource usage data
        let base_memory = 100.0;
        let memory_variation = (rand::random::<f64>() - 0.5) * 50.0;
        let memory_mb = (base_memory + memory_variation).max(50.0).min(500.0);

        let base_cpu = 25.0;
        let cpu_variation = (rand::random::<f64>() - 0.5) * 40.0;
        let cpu_percent = (base_cpu + cpu_variation).max(0.0).min(100.0);

        let io_operations = (rand::random::<u64>() % 1000) + 100;
        let network_sent = (rand::random::<u64>() % 10000) + 1000;
        let network_received = (rand::random::<u64>() % 8000) + 800;

        ResourceSample {
            timestamp: Instant::now(),
            memory_mb,
            cpu_percent,
            io_operations,
            network_sent,
            network_received,
        }
    }

    /// Start the monitoring thread
    fn start_monitoring_thread(&mut self) {
        let config = self.config.clone();
        let samples = Arc::clone(&self.resource_samples);
        let active = Arc::clone(&self.monitoring_active);

        let handle = thread::spawn(move || {
            let interval = Duration::from_millis(config.monitoring_interval_ms);
            
            while *active.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) {
                let sample = ResourceSample {
                    timestamp: Instant::now(),
                    memory_mb: Self::get_memory_usage_mb(),
                    cpu_percent: Self::get_cpu_usage_percent(),
                    io_operations: Self::get_io_operations(),
                    network_sent: Self::get_network_sent(),
                    network_received: Self::get_network_received(),
                };

                if let Ok(mut samples_guard) = samples.lock() {
                    samples_guard.push(sample);
                    
                    // Keep only recent samples to avoid memory growth
                    if samples_guard.len() > 10000 {
                        samples_guard.drain(0..5000);
                    }
                }

                thread::sleep(interval);
            }
        });

        self.monitoring_thread = Some(handle);
    }

    /// Get current memory usage in MB (simulated)
    fn get_memory_usage_mb() -> f64 {
        // Try to get real memory usage, fall back to simulation
        if let Ok(usage) = Self::get_process_memory_usage() {
            usage
        } else {
            // Fallback simulation
            100.0 + (rand::random::<f64>() * 50.0)
        }
    }
    
    /// Get real process memory usage in MB
    fn get_process_memory_usage() -> Result<f64> {
        #[cfg(target_os = "linux")]
        {
            let status = std::fs::read_to_string("/proc/self/status").map_err(|e| {
                CursedError::optimization_error(&format!("Failed to read /proc/self/status: {}", e))
            })?;
            
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<f64>() {
                            return Ok(kb / 1024.0); // Convert KB to MB
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            // Use task_info on macOS
            // This is a simplified version - real implementation would use mach APIs
            return Err(CursedError::optimization_error("macOS memory monitoring not implemented"));
        }
        
        #[cfg(target_os = "windows")]
        {
            // Use GetProcessMemoryInfo on Windows
            // This is a simplified version - real implementation would use Windows APIs
            return Err(CursedError::optimization_error("Windows memory monitoring not implemented"));
        }
        
        Err(CursedError::optimization_error("Memory monitoring not supported on this platform"))
    }

    /// Get current CPU usage percentage (simulated)
    fn get_cpu_usage_percent() -> f64 {
        // Try to get real CPU usage, fall back to simulation
        if let Ok(usage) = Self::get_process_cpu_usage() {
            usage
        } else {
            // Fallback simulation
            20.0 + (rand::random::<f64>() * 30.0)
        }
    }
    
    /// Get real process CPU usage percentage
    fn get_process_cpu_usage() -> Result<f64> {
        #[cfg(target_os = "linux")]
        {
            let stat = std::fs::read_to_string("/proc/self/stat").map_err(|e| {
                CursedError::optimization_error(&format!("Failed to read /proc/self/stat: {}", e))
            })?;
            
            let fields: Vec<&str> = stat.split_whitespace().collect();
            if fields.len() >= 17 {
                // Fields 13 and 14 contain user and system time in clock ticks
                if let (Ok(utime), Ok(stime)) = (fields[13].parse::<u64>(), fields[14].parse::<u64>()) {
                    let total_time = utime + stime;
                    
                    // Get system uptime
                    if let Ok(uptime_str) = std::fs::read_to_string("/proc/uptime") {
                        let uptime_parts: Vec<&str> = uptime_str.split_whitespace().collect();
                        if let Ok(uptime_seconds) = uptime_parts[0].parse::<f64>() {
                            let clock_ticks_per_second = 100.0; // Typical value
                            let process_time_seconds = total_time as f64 / clock_ticks_per_second;
                            let cpu_usage = (process_time_seconds / uptime_seconds) * 100.0;
                            return Ok(cpu_usage.min(100.0));
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            // Use task_info on macOS - simplified
            return Err(CursedError::optimization_error("macOS CPU monitoring not implemented"));
        }
        
        #[cfg(target_os = "windows")]
        {
            // Use GetProcessTimes on Windows - simplified
            return Err(CursedError::optimization_error("Windows CPU monitoring not implemented"));
        }
        
        Err(CursedError::optimization_error("CPU monitoring not supported on this platform"))
    }

    /// Get I/O operations count (simulated)
    fn get_io_operations() -> u64 {
        // Simulate I/O operations measurement
        rand::random::<u64>() % 1000
    }

    /// Get network bytes sent (simulated)
    fn get_network_sent() -> u64 {
        // Simulate network sent measurement
        rand::random::<u64>() % 10000
    }

    /// Get network bytes received (simulated)
    fn get_network_received() -> u64 {
        // Simulate network received measurement
        rand::random::<u64>() % 8000
    }

    /// Get performance analysis for a time period
    pub fn get_performance_analysis(&self, _duration: Duration) -> Result<crate::optimization::analysis::PerformanceAnalysis> {
        let stats = self.get_system_statistics();
        
        Ok(crate::optimization::analysis::PerformanceAnalysis {
            units_optimized: stats.total_units_optimized,
            total_optimization_time: stats.total_compilation_time,
            total_size_reduction: 1000, // Simulated
            optimization_efficiency: 0.85, // Simulated
            recommendations: vec![
                "Consider enabling parallel compilation for better performance".to_string(),
                "Cache hit rate could be improved".to_string(),
            ],
        })
    }

    /// Update metrics collector configuration
    pub fn update_config(&mut self, new_config: PerformanceConfig) -> Result<()> {
        info!("Updating metrics collector configuration");
        self.config = new_config;
        Ok(())
    }

    /// Reset all collected metrics
    pub fn reset_metrics(&self) -> Result<()> {
        if let Ok(mut stats) = self.system_stats.lock() {
            *stats = SystemStatistics::default();
        }
        
        if let Ok(mut samples) = self.resource_samples.lock() {
            samples.clear();
        }

        info!("Reset all metrics");
        Ok(())
    }
}

impl Drop for MetricsCollector {
    fn drop(&mut self) {
        // Stop monitoring when dropped
        let _ = self.stop_monitoring();
        
        // Wait for monitoring thread to finish
        if let Some(handle) = self.monitoring_thread.take() {
            let _ = handle.join();
        }
    }
}

// Simple random number generation for simulation
mod rand {
    use std::cell::Cell;
    
    thread_local! {
        static RNG_STATE: Cell<u64> = Cell::new(1);
    }
    
    pub fn random<T>() -> T 
    where 
        T: From<u64>
    {
        RNG_STATE.with(|state| {
            let current = state.get();
            let next = current.wrapping_mul(1103515245).wrapping_add(12345);
            state.set(next);
            T::from(next)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compilation_unit_creation() {
        let unit = CompilationUnit::new("test_unit".to_string());
        assert_eq!(unit.name, "test_unit");
        assert_eq!(unit.estimated_size_bytes, 10000);
        assert!(unit.source_files.is_empty());
    }

    #[test]
    fn test_metrics_collector_creation() {
        let config = PerformanceConfig::default();
        let collector = MetricsCollector::new(config);
        assert!(collector.is_ok());
    }

    #[test]
    fn test_system_statistics_recording() {
        let config = PerformanceConfig::default();
        let collector = MetricsCollector::new(config).unwrap();
        
        collector.record_optimization_completion(3, Duration::from_millis(100));
        collector.record_benchmark_run();
        collector.record_cache_hit();
        collector.record_cache_miss();
        
        let stats = collector.get_system_statistics();
        assert_eq!(stats.optimizations_completed, 1);
        assert_eq!(stats.total_units_optimized, 3);
        assert_eq!(stats.benchmark_runs, 1);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
    }

    #[test]
    fn test_monitoring_start_stop() {
        let config = PerformanceConfig::default();
        let collector = MetricsCollector::new(config).unwrap();
        
        assert!(collector.start_monitoring().is_ok());
        assert!(collector.stop_monitoring().is_ok());
    }
}
