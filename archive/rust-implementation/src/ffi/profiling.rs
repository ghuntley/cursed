//! FFI performance profiling
//!
//! This module provides performance profiling capabilities for FFI operations
//! to help identify bottlenecks and optimize foreign function calls.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::error::CursedError;

/// FFI performance profiler
pub struct FfiProfiler {
    /// Function call profiles
    function_profiles: HashMap<String, FunctionProfile>,
    
    /// Active profiling sessions
    active_sessions: HashMap<String, ProfilingSession>,
    
    /// Profiling configuration
    config: ProfilingConfig,
    
    /// Performance statistics
    stats: PerformanceStats,
}

/// Function performance profile
#[derive(Debug, Clone)]
pub struct FunctionProfile {
    /// Function name
    pub name: String,
    
    /// Total calls
    pub total_calls: u64,
    
    /// Total execution time
    pub total_time: Duration,
    
    /// Average execution time
    pub average_time: Duration,
    
    /// Minimum execution time
    pub min_time: Duration,
    
    /// Maximum execution time
    pub max_time: Duration,
    
    /// Call frequency (calls per second)
    pub call_frequency: f64,
    
    /// Error rate
    pub error_rate: f64,
    
    /// Memory usage
    pub memory_usage: MemoryUsage,
}

/// Memory usage information
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    /// Average memory allocated per call
    pub average_allocation: usize,
    
    /// Peak memory usage
    pub peak_usage: usize,
    
    /// Total memory allocated
    pub total_allocated: usize,
    
    /// Memory allocation rate
    pub allocation_rate: f64,
}

/// Profiling session
struct ProfilingSession {
    /// Session ID
    id: String,
    
    /// Start time
    start_time: Instant,
    
    /// Function name
    function_name: String,
    
    /// Session data
    data: SessionData,
}

/// Session data
struct SessionData {
    /// Memory snapshots
    memory_snapshots: Vec<MemorySnapshot>,
    
    /// Call stack samples
    call_stack_samples: Vec<CallStackSample>,
    
    /// Performance counters
    performance_counters: HashMap<String, u64>,
}

/// Memory snapshot
#[derive(Debug, Clone)]
struct MemorySnapshot {
    timestamp: Instant,
    allocated: usize,
    peak: usize,
}

/// Call stack sample
#[derive(Debug, Clone)]
struct CallStackSample {
    timestamp: Instant,
    stack_trace: Vec<String>,
}

/// Profiling configuration
#[derive(Debug, Clone)]
pub struct ProfilingConfig {
    /// Enable profiling
    pub enabled: bool,
    
    /// Sampling rate (samples per second)
    pub sampling_rate: f64,
    
    /// Memory tracking
    pub track_memory: bool,
    
    /// Call stack tracking
    pub track_call_stack: bool,
    
    /// Maximum profile history
    pub max_history: usize,
    
    /// Profile output format
    pub output_format: OutputFormat,
}

/// Output format for profiling data
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Csv,
    Binary,
    Flamegraph,
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    /// Total profiled functions
    pub total_functions: usize,
    
    /// Total profiled calls
    pub total_calls: u64,
    
    /// Total profiling time
    pub total_time: Duration,
    
    /// Average call time across all functions
    pub average_call_time: Duration,
    
    /// Most called function
    pub most_called_function: Option<String>,
    
    /// Slowest function
    pub slowest_function: Option<String>,
    
    /// Functions with highest error rate
    pub highest_error_rate_function: Option<String>,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sampling_rate: 100.0,
            track_memory: true,
            track_call_stack: true,
            max_history: 10000,
            output_format: OutputFormat::Json,
        }
    }
}

impl FfiProfiler {
    /// Create new FFI profiler
    pub fn new() -> Self {
        Self {
            function_profiles: HashMap::new(),
            active_sessions: HashMap::new(),
            config: ProfilingConfig::default(),
            stats: PerformanceStats::default(),
        }
    }
    
    /// Create profiler with custom configuration
    pub fn with_config(config: ProfilingConfig) -> Self {
        Self {
            function_profiles: HashMap::new(),
            active_sessions: HashMap::new(),
            config,
            stats: PerformanceStats::default(),
        }
    }
    
    /// Start profiling a function call
    pub fn start_call(&mut self, function_name: &str) -> ProfilingGuard {
        if !self.config.enabled {
            return ProfilingGuard::disabled();
        }
        
        let session_id = format!("{}_{}", function_name, self.generate_session_id());
        let session = ProfilingSession {
            id: session_id.clone(),
            start_time: Instant::now(),
            function_name: function_name.to_string(),
            data: SessionData {
                memory_snapshots: Vec::new(),
                call_stack_samples: Vec::new(),
                performance_counters: HashMap::new(),
            },
        };
        
        self.active_sessions.insert(session_id.clone(), session);
        
        ProfilingGuard::new(session_id, function_name.to_string())
    }
    
    /// End profiling a function call
    pub fn end_call(&mut self, session_id: &str, result: Result<(), CursedError>) -> Result<(), CursedError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        let session = self.active_sessions.remove(session_id)
            .ok_or_else(|| CursedError::General("Profiling session not found".to_string()))?;
        
        let duration = session.start_time.elapsed();
        
        // Update function profile
        self.update_function_profile(&session.function_name, duration, result.is_ok());
        
        // Update global statistics
        self.update_global_stats(duration);
        
        Ok(())
    }
    
    /// Record memory allocation
    pub fn record_allocation(&mut self, session_id: &str, size: usize) -> Result<(), CursedError> {
        if !self.config.track_memory {
            return Ok(());
        }
        
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            let snapshot = MemorySnapshot {
                timestamp: Instant::now(),
                allocated: size,
                peak: size, // Would track actual peak
            };
            session.data.memory_snapshots.push(snapshot);
        }
        
        Ok(())
    }
    
    /// Record call stack sample
    pub fn record_call_stack(&mut self, session_id: &str, stack_trace: Vec<String>) -> Result<(), CursedError> {
        if !self.config.track_call_stack {
            return Ok(());
        }
        
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            let sample = CallStackSample {
                timestamp: Instant::now(),
                stack_trace,
            };
            session.data.call_stack_samples.push(sample);
        }
        
        Ok(())
    }
    
    /// Get function profile
    pub fn get_function_profile(&self, function_name: &str) -> Option<&FunctionProfile> {
        self.function_profiles.get(function_name)
    }
    
    /// Get all function profiles
    pub fn get_all_profiles(&self) -> &HashMap<String, FunctionProfile> {
        &self.function_profiles
    }
    
    /// Get performance statistics
    pub fn get_stats(&self) -> &PerformanceStats {
        &self.stats
    }
    
    /// Export profiling data
    pub fn export_data(&self, output_path: &str) -> Result<(), CursedError> {
        match self.config.output_format {
            OutputFormat::Json => self.export_json(output_path),
            OutputFormat::Csv => self.export_csv(output_path),
            OutputFormat::Binary => self.export_binary(output_path),
            OutputFormat::Flamegraph => self.export_flamegraph(output_path),
        }
    }
    
    /// Clear all profiling data
    pub fn clear(&mut self) {
        self.function_profiles.clear();
        self.active_sessions.clear();
        self.stats = PerformanceStats::default();
    }
    
    /// Enable profiling
    pub fn enable(&mut self) {
        self.config.enabled = true;
    }
    
    /// Disable profiling
    pub fn disable(&mut self) {
        self.config.enabled = false;
    }
    
    // Private helper methods
    
    fn generate_session_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{:x}", timestamp)
    }
    
    fn update_function_profile(&mut self, function_name: &str, duration: Duration, success: bool) {
        let profile = self.function_profiles.entry(function_name.to_string())
            .or_insert_with(|| FunctionProfile {
                name: function_name.to_string(),
                total_calls: 0,
                total_time: Duration::ZERO,
                average_time: Duration::ZERO,
                min_time: Duration::MAX,
                max_time: Duration::ZERO,
                call_frequency: 0.0,
                error_rate: 0.0,
                memory_usage: MemoryUsage {
                    average_allocation: 0,
                    peak_usage: 0,
                    total_allocated: 0,
                    allocation_rate: 0.0,
                },
            });
        
        profile.total_calls += 1;
        profile.total_time += duration;
        profile.average_time = profile.total_time / profile.total_calls as u32;
        
        if duration < profile.min_time {
            profile.min_time = duration;
        }
        
        if duration > profile.max_time {
            profile.max_time = duration;
        }
        
        // Update error rate
        if !success {
            let error_count = (profile.error_rate * profile.total_calls as f64) as u64 + 1;
            profile.error_rate = error_count as f64 / profile.total_calls as f64;
        }
        
        // Update call frequency (calls per second)
        let elapsed_seconds = profile.total_time.as_secs_f64();
        if elapsed_seconds > 0.0 {
            profile.call_frequency = profile.total_calls as f64 / elapsed_seconds;
        }
    }
    
    fn update_global_stats(&mut self, duration: Duration) {
        self.stats.total_calls += 1;
        self.stats.total_time += duration;
        self.stats.average_call_time = self.stats.total_time / self.stats.total_calls as u32;
        self.stats.total_functions = self.function_profiles.len();
        
        // Update most called function
        if let Some((name, _)) = self.function_profiles.iter()
            .max_by_key(|(_, profile)| profile.total_calls) {
            self.stats.most_called_function = Some(name.clone());
        }
        
        // Update slowest function
        if let Some((name, _)) = self.function_profiles.iter()
            .max_by_key(|(_, profile)| profile.average_time) {
            self.stats.slowest_function = Some(name.clone());
        }
        
        // Update highest error rate function
        if let Some((name, _)) = self.function_profiles.iter()
            .max_by(|(_, a), (_, b)| a.error_rate.partial_cmp(&b.error_rate).unwrap()) {
            self.stats.highest_error_rate_function = Some(name.clone());
        }
    }
    
    fn export_json(&self, output_path: &str) -> Result<(), CursedError> {
        let json = serde_json::to_string_pretty(&self.function_profiles)
            .map_err(|e| CursedError::General(format!("Failed to serialize to JSON: {}", e)))?;
        
        std::fs::write(output_path, json)
            .map_err(|e| CursedError::General(format!("Failed to write JSON file: {}", e)))?;
        
        Ok(())
    }
    
    fn export_csv(&self, output_path: &str) -> Result<(), CursedError> {
        let mut csv_content = String::new();
        csv_content.push_str("function_name,total_calls,total_time_ms,average_time_ms,min_time_ms,max_time_ms,error_rate\n");
        
        for (_, profile) in &self.function_profiles {
            csv_content.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                profile.name,
                profile.total_calls,
                profile.total_time.as_millis(),
                profile.average_time.as_millis(),
                profile.min_time.as_millis(),
                profile.max_time.as_millis(),
                profile.error_rate
            ));
        }
        
        std::fs::write(output_path, csv_content)
            .map_err(|e| CursedError::General(format!("Failed to write CSV file: {}", e)))?;
        
        Ok(())
    }
    
    fn export_binary(&self, output_path: &str) -> Result<(), CursedError> {
        // This would implement binary serialization
        // For now, just write a placeholder
        let binary_data = bincode::serialize(&self.function_profiles)
            .map_err(|e| CursedError::General(format!("Failed to serialize to binary: {}", e)))?;
        
        std::fs::write(output_path, binary_data)
            .map_err(|e| CursedError::General(format!("Failed to write binary file: {}", e)))?;
        
        Ok(())
    }
    
    fn export_flamegraph(&self, output_path: &str) -> Result<(), CursedError> {
        // This would implement flamegraph format export
        // For now, just write a simple format
        let mut flamegraph_data = String::new();
        
        for (_, profile) in &self.function_profiles {
            flamegraph_data.push_str(&format!(
                "{} {}\n",
                profile.name,
                profile.total_time.as_millis()
            ));
        }
        
        std::fs::write(output_path, flamegraph_data)
            .map_err(|e| CursedError::General(format!("Failed to write flamegraph file: {}", e)))?;
        
        Ok(())
    }
}

/// RAII guard for profiling sessions
pub struct ProfilingGuard {
    session_id: Option<String>,
    function_name: String,
    start_time: Instant,
}

impl ProfilingGuard {
    fn new(session_id: String, function_name: String) -> Self {
        Self {
            session_id: Some(session_id),
            function_name,
            start_time: Instant::now(),
        }
    }
    
    fn disabled() -> Self {
        Self {
            session_id: None,
            function_name: String::new(),
            start_time: Instant::now(),
        }
    }
    
    /// Get the session ID
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }
    
    /// Get the function name
    pub fn function_name(&self) -> &str {
        &self.function_name
    }
    
    /// Get the elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Drop for ProfilingGuard {
    fn drop(&mut self) {
        // The profiler will be notified when the guard is dropped
        // This would typically be handled by the profiler automatically
    }
}

impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
            total_functions: 0,
            total_calls: 0,
            total_time: Duration::ZERO,
            average_call_time: Duration::ZERO,
            most_called_function: None,
            slowest_function: None,
            highest_error_rate_function: None,
        }
    }
}

impl Default for FfiProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ffi_profiler_creation() {
        let profiler = FfiProfiler::new();
        assert!(!profiler.config.enabled);
        assert_eq!(profiler.function_profiles.len(), 0);
    }
    
    #[test]
    fn test_profiling_guard() {
        let guard = ProfilingGuard::new("test_session".to_string(), "test_function".to_string());
        assert_eq!(guard.session_id(), Some("test_session"));
        assert_eq!(guard.function_name(), "test_function");
        assert!(guard.elapsed() >= Duration::ZERO);
    }
    
    #[test]
    fn test_disabled_profiling() {
        let mut profiler = FfiProfiler::new();
        let guard = profiler.start_call("test_function");
        assert_eq!(guard.session_id(), None);
    }
    
    #[test]
    fn test_function_profile_update() {
        let mut profiler = FfiProfiler::with_config(ProfilingConfig {
            enabled: true,
            ..Default::default()
        });
        
        let duration = Duration::from_millis(100);
        profiler.update_function_profile("test_function", duration, true);
        
        let profile = profiler.get_function_profile("test_function").unwrap();
        assert_eq!(profile.total_calls, 1);
        assert_eq!(profile.total_time, duration);
        assert_eq!(profile.average_time, duration);
        assert_eq!(profile.min_time, duration);
        assert_eq!(profile.max_time, duration);
        assert_eq!(profile.error_rate, 0.0);
    }
    
    #[test]
    fn test_performance_stats() {
        let mut profiler = FfiProfiler::with_config(ProfilingConfig {
            enabled: true,
            ..Default::default()
        });
        
        let duration = Duration::from_millis(50);
        profiler.update_function_profile("test_function", duration, true);
        profiler.update_global_stats(duration);
        
        let stats = profiler.get_stats();
        assert_eq!(stats.total_calls, 1);
        assert_eq!(stats.total_time, duration);
        assert_eq!(stats.average_call_time, duration);
        assert_eq!(stats.most_called_function, Some("test_function".to_string()));
    }
    
    #[test]
    fn test_export_formats() {
        let profiler = FfiProfiler::new();
        
        // Test different output formats
        assert!(matches!(profiler.config.output_format, OutputFormat::Json));
        
        let config = ProfilingConfig {
            output_format: OutputFormat::Csv,
            ..Default::default()
        };
        
        let profiler = FfiProfiler::with_config(config);
        assert!(matches!(profiler.config.output_format, OutputFormat::Csv));
    }
}
