//! Debug tools for FFI operations
//!
//! This module provides comprehensive debugging capabilities for FFI operations
//! including call tracing, memory debugging, performance profiling, and
//! interactive debugging tools.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::error::CursedError;
use super::{FfiValue, FfiType, FunctionSignature};

/// Debug tools for FFI operations
pub struct DebugTools {
    /// Call tracer
    tracer: Arc<Mutex<CallTracer>>,
    
    /// Memory debugger
    memory_debugger: Arc<Mutex<MemoryDebugger>>,
    
    /// Performance profiler
    profiler: Arc<Mutex<PerformanceProfiler>>,
    
    /// Interactive debugger
    interactive_debugger: Arc<Mutex<InteractiveDebugger>>,
    
    /// Debug configuration
    config: DebugConfig,
}

/// Call tracer for FFI function calls
struct CallTracer {
    /// Traced calls
    calls: Vec<TracedCall>,
    
    /// Output writer
    output: Option<BufWriter<File>>,
    
    /// Tracer configuration
    config: TracerConfig,
}

/// Traced function call
#[derive(Debug, Clone)]
pub struct TracedCall {
    /// Function name
    pub function_name: String,
    
    /// Arguments
    pub arguments: Vec<FfiValue>,
    
    /// Return value
    pub return_value: Option<FfiValue>,
    
    /// Call timestamp
    pub timestamp: Instant,
    
    /// Call duration
    pub duration: Duration,
    
    /// Call result
    pub result: CallResult,
    
    /// Stack trace
    pub stack_trace: Option<Vec<String>>,
}

/// Call result
#[derive(Debug, Clone)]
pub enum CallResult {
    Success,
    Error(String),
    Timeout,
    Crash,
}

/// Tracer configuration
struct TracerConfig {
    /// Enable call tracing
    enabled: bool,
    
    /// Maximum number of calls to trace
    max_calls: usize,
    
    /// Enable stack traces
    enable_stack_traces: bool,
    
    /// Trace arguments
    trace_arguments: bool,
    
    /// Trace return values
    trace_return_values: bool,
    
    /// Output file path
    output_file: Option<String>,
}

/// Memory debugger for FFI operations
struct MemoryDebugger {
    /// Memory operations
    operations: Vec<MemoryOperation>,
    
    /// Memory leak detector
    leak_detector: LeakDetector,
    
    /// Buffer overflow detector
    overflow_detector: OverflowDetector,
    
    /// Memory usage tracker
    usage_tracker: MemoryUsageTracker,
}

/// Memory operation
#[derive(Debug, Clone)]
pub struct MemoryOperation {
    /// Operation type
    pub operation_type: MemoryOperationType,
    
    /// Memory address
    pub address: usize,
    
    /// Size
    pub size: usize,
    
    /// Timestamp
    pub timestamp: Instant,
    
    /// Stack trace
    pub stack_trace: Option<Vec<String>>,
}

/// Memory operation type
#[derive(Debug, Clone)]
pub enum MemoryOperationType {
    Allocate,
    Deallocate,
    Reallocate,
    Read,
    Write,
}

/// Memory leak detector
struct LeakDetector {
    /// Tracked allocations
    allocations: HashMap<usize, AllocationInfo>,
    
    /// Leak threshold
    leak_threshold: Duration,
}

/// Allocation information
#[derive(Debug, Clone)]
struct AllocationInfo {
    size: usize,
    allocated_at: Instant,
    stack_trace: Option<Vec<String>>,
}

/// Buffer overflow detector
struct OverflowDetector {
    /// Protected regions
    protected_regions: HashMap<usize, ProtectedRegion>,
    
    /// Guard pages
    guard_pages: HashMap<usize, usize>,
}

/// Protected memory region
#[derive(Debug, Clone)]
struct ProtectedRegion {
    start: usize,
    size: usize,
    guard_before: usize,
    guard_after: usize,
}

/// Memory usage tracker
struct MemoryUsageTracker {
    /// Current usage
    current_usage: usize,
    
    /// Peak usage
    peak_usage: usize,
    
    /// Usage history
    usage_history: Vec<MemoryUsageSnapshot>,
}

/// Memory usage snapshot
#[derive(Debug, Clone)]
struct MemoryUsageSnapshot {
    timestamp: Instant,
    usage: usize,
}

/// Performance profiler for FFI operations
struct PerformanceProfiler {
    /// Function profiles
    function_profiles: HashMap<String, FunctionProfile>,
    
    /// Call stack samples
    call_stack_samples: Vec<CallStackSample>,
    
    /// Profiler configuration
    config: ProfilerConfig,
}

/// Function performance profile
#[derive(Debug, Clone)]
pub struct FunctionProfile {
    /// Function name
    pub name: String,
    
    /// Total calls
    pub total_calls: u64,
    
    /// Total time
    pub total_time: Duration,
    
    /// Average time
    pub average_time: Duration,
    
    /// Min time
    pub min_time: Duration,
    
    /// Max time
    pub max_time: Duration,
    
    /// Time distribution
    pub time_distribution: Vec<Duration>,
}

/// Call stack sample
#[derive(Debug, Clone)]
struct CallStackSample {
    timestamp: Instant,
    stack_trace: Vec<String>,
}

/// Profiler configuration
struct ProfilerConfig {
    /// Enable profiling
    enabled: bool,
    
    /// Sampling rate
    sampling_rate: Duration,
    
    /// Maximum samples
    max_samples: usize,
    
    /// Enable call stack sampling
    enable_call_stack_sampling: bool,
}

/// Interactive debugger for FFI operations
struct InteractiveDebugger {
    /// Breakpoints
    breakpoints: HashMap<String, Breakpoint>,
    
    /// Watch expressions
    watches: Vec<WatchExpression>,
    
    /// Debugger state
    state: DebuggerState,
}

/// Breakpoint
#[derive(Debug, Clone)]
struct Breakpoint {
    function_name: String,
    condition: Option<String>,
    enabled: bool,
    hit_count: u32,
}

/// Watch expression
#[derive(Debug, Clone)]
struct WatchExpression {
    expression: String,
    value: Option<FfiValue>,
    last_updated: Instant,
}

/// Debugger state
#[derive(Debug, Clone)]
enum DebuggerState {
    Running,
    Paused,
    Stepping,
    Terminated,
}

/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Enable debug mode
    pub enable_debug: bool,
    
    /// Enable call tracing
    pub enable_tracing: bool,
    
    /// Enable memory debugging
    pub enable_memory_debug: bool,
    
    /// Enable performance profiling
    pub enable_profiling: bool,
    
    /// Enable interactive debugging
    pub enable_interactive: bool,
    
    /// Debug output directory
    pub output_directory: String,
    
    /// Maximum debug data size
    pub max_debug_data_size: usize,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            enable_debug: false,
            enable_tracing: false,
            enable_memory_debug: false,
            enable_profiling: false,
            enable_interactive: false,
            output_directory: "debug_output".to_string(),
            max_debug_data_size: 100 * 1024 * 1024, // 100MB
        }
    }
}

impl DebugTools {
    /// Create new debug tools
    pub fn new() -> Self {
        Self {
            tracer: Arc::new(Mutex::new(CallTracer::new())),
            memory_debugger: Arc::new(Mutex::new(MemoryDebugger::new())),
            profiler: Arc::new(Mutex::new(PerformanceProfiler::new())),
            interactive_debugger: Arc::new(Mutex::new(InteractiveDebugger::new())),
            config: DebugConfig::default(),
        }
    }
    
    /// Create debug tools with custom configuration
    pub fn with_config(config: DebugConfig) -> Self {
        let mut tools = Self::new();
        tools.config = config;
        tools
    }
    
    /// Enable debug mode
    pub fn enable_debug_mode(&self) -> Result<(), CursedError> {
        // Create debug output directory
        std::fs::create_dir_all(&self.config.output_directory)
            .map_err(|e| CursedError::General(format!("Failed to create debug directory: {}", e)))?;
        
        // Enable components based on configuration
        if self.config.enable_tracing {
            let mut tracer = self.tracer.lock().unwrap();
            tracer.enable(Some(format!("{}/trace.log", self.config.output_directory)))?;
        }
        
        if self.config.enable_memory_debug {
            let mut memory_debugger = self.memory_debugger.lock().unwrap();
            memory_debugger.enable()?;
        }
        
        if self.config.enable_profiling {
            let mut profiler = self.profiler.lock().unwrap();
            profiler.enable()?;
        }
        
        if self.config.enable_interactive {
            let mut interactive_debugger = self.interactive_debugger.lock().unwrap();
            interactive_debugger.enable()?;
        }
        
        Ok(())
    }
    
    /// Trace a function call
    pub fn trace_call(
        &self,
        function_name: &str,
        args: &[FfiValue],
        call_fn: Box<dyn FnOnce() -> Result<FfiValue, CursedError>>,
    ) -> Result<FfiValue, CursedError> {
        if !self.config.enable_tracing {
            return call_fn();
        }
        
        let start_time = Instant::now();
        
        // Execute the function
        let result = call_fn();
        
        let duration = start_time.elapsed();
        
        // Create traced call
        let traced_call = TracedCall {
            function_name: function_name.to_string(),
            arguments: args.to_vec(),
            return_value: result.as_ref().ok().cloned(),
            timestamp: start_time,
            duration,
            result: match &result {
                Ok(_) => CallResult::Success,
                Err(e) => CallResult::Error(e.to_string()),
            },
            stack_trace: self.capture_stack_trace(),
        };
        
        // Add to tracer
        {
            let mut tracer = self.tracer.lock().unwrap();
            tracer.add_call(traced_call)?;
        }
        
        result
    }
    
    /// Debug memory operation
    pub fn debug_memory_operation(
        &self,
        operation_type: MemoryOperationType,
        address: usize,
        size: usize,
    ) -> Result<(), CursedError> {
        if !self.config.enable_memory_debug {
            return Ok(());
        }
        
        let operation = MemoryOperation {
            operation_type,
            address,
            size,
            timestamp: Instant::now(),
            stack_trace: self.capture_stack_trace(),
        };
        
        let mut memory_debugger = self.memory_debugger.lock().unwrap();
        memory_debugger.add_operation(operation)?;
        
        Ok(())
    }
    
    /// Profile function performance
    pub fn profile_function<F, R>(
        &self,
        function_name: &str,
        function: F,
    ) -> Result<R, CursedError>
    where
        F: FnOnce() -> Result<R, CursedError>,
    {
        if !self.config.enable_profiling {
            return function();
        }
        
        let start_time = Instant::now();
        let result = function();
        let duration = start_time.elapsed();
        
        // Update function profile
        {
            let mut profiler = self.profiler.lock().unwrap();
            profiler.update_function_profile(function_name, duration)?;
        }
        
        result
    }
    
    /// Set breakpoint
    pub fn set_breakpoint(&self, function_name: &str, condition: Option<String>) -> Result<(), CursedError> {
        if !self.config.enable_interactive {
            return Ok(());
        }
        
        let breakpoint = Breakpoint {
            function_name: function_name.to_string(),
            condition,
            enabled: true,
            hit_count: 0,
        };
        
        let mut debugger = self.interactive_debugger.lock().unwrap();
        debugger.breakpoints.insert(function_name.to_string(), breakpoint);
        
        Ok(())
    }
    
    /// Add watch expression
    pub fn add_watch(&self, expression: &str) -> Result<(), CursedError> {
        if !self.config.enable_interactive {
            return Ok(());
        }
        
        let watch = WatchExpression {
            expression: expression.to_string(),
            value: None,
            last_updated: Instant::now(),
        };
        
        let mut debugger = self.interactive_debugger.lock().unwrap();
        debugger.watches.push(watch);
        
        Ok(())
    }
    
    /// Get function debug information
    pub fn get_function_debug_info(&self, function_name: &str) -> Result<super::DebugInfo, CursedError> {
        let traced_calls = {
            let tracer = self.tracer.lock().unwrap();
            tracer.get_calls_for_function(function_name)
        };
        
        let function_profile = {
            let profiler = self.profiler.lock().unwrap();
            profiler.get_function_profile(function_name)
        };
        
        let call_count = traced_calls.len() as u64;
        let last_call_time = traced_calls.last()
            .map(|call| call.duration.as_secs_f64())
            .unwrap_or(0.0);
        
        let error_count = traced_calls.iter()
            .filter(|call| matches!(call.result, CallResult::Error(_)))
            .count() as u64;
        
        Ok(super::DebugInfo {
            function_name: function_name.to_string(),
            call_count,
            last_call_time,
            error_count,
            type_conversions: Vec::new(), // Would be populated from actual conversion data
        })
    }
    
    /// Get memory debug report
    pub fn get_memory_debug_report(&self) -> Result<MemoryDebugReport, CursedError> {
        let memory_debugger = self.memory_debugger.lock().unwrap();
        
        Ok(MemoryDebugReport {
            operations: memory_debugger.operations.clone(),
            leaks: memory_debugger.leak_detector.detect_leaks(),
            overflow_detections: memory_debugger.overflow_detector.get_detections(),
            usage_stats: memory_debugger.usage_tracker.get_stats(),
        })
    }
    
    /// Get performance profile report
    pub fn get_performance_report(&self) -> Result<PerformanceReport, CursedError> {
        let profiler = self.profiler.lock().unwrap();
        
        Ok(PerformanceReport {
            function_profiles: profiler.function_profiles.clone(),
            call_stack_samples: profiler.call_stack_samples.clone(),
            total_samples: profiler.call_stack_samples.len(),
        })
    }
    
    /// Export debug data
    pub fn export_debug_data(&self, export_path: &str) -> Result<(), CursedError> {
        // Export trace data
        if self.config.enable_tracing {
            let tracer = self.tracer.lock().unwrap();
            tracer.export_to_file(&format!("{}/trace_export.json", export_path))?;
        }
        
        // Export memory debug data
        if self.config.enable_memory_debug {
            let memory_debugger = self.memory_debugger.lock().unwrap();
            memory_debugger.export_to_file(&format!("{}/memory_export.json", export_path))?;
        }
        
        // Export performance data
        if self.config.enable_profiling {
            let profiler = self.profiler.lock().unwrap();
            profiler.export_to_file(&format!("{}/performance_export.json", export_path))?;
        }
        
        Ok(())
    }
    
    /// Clear debug data
    pub fn clear_debug_data(&self) -> Result<(), CursedError> {
        {
            let mut tracer = self.tracer.lock().unwrap();
            tracer.clear();
        }
        
        {
            let mut memory_debugger = self.memory_debugger.lock().unwrap();
            memory_debugger.clear();
        }
        
        {
            let mut profiler = self.profiler.lock().unwrap();
            profiler.clear();
        }
        
        {
            let mut interactive_debugger = self.interactive_debugger.lock().unwrap();
            interactive_debugger.clear();
        }
        
        Ok(())
    }
    
    /// Check if breakpoint is hit
    pub fn check_breakpoint(&self, function_name: &str) -> Result<bool, CursedError> {
        if !self.config.enable_interactive {
            return Ok(false);
        }
        
        let mut debugger = self.interactive_debugger.lock().unwrap();
        
        if let Some(breakpoint) = debugger.breakpoints.get_mut(function_name) {
            if breakpoint.enabled {
                breakpoint.hit_count += 1;
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    // Private helper methods
    
    fn capture_stack_trace(&self) -> Option<Vec<String>> {
        // This would implement actual stack trace capture
        // For now, return a placeholder
        Some(vec!["stack_trace_placeholder".to_string()])
    }
}

/// Memory debug report
#[derive(Debug, Clone)]
pub struct MemoryDebugReport {
    pub operations: Vec<MemoryOperation>,
    pub leaks: Vec<MemoryLeak>,
    pub overflow_detections: Vec<OverflowDetection>,
    pub usage_stats: MemoryUsageStats,
}

/// Memory leak information
#[derive(Debug, Clone)]
pub struct MemoryLeak {
    pub address: usize,
    pub size: usize,
    pub age: Duration,
    pub stack_trace: Option<Vec<String>>,
}

/// Buffer overflow detection
#[derive(Debug, Clone)]
pub struct OverflowDetection {
    pub address: usize,
    pub size: usize,
    pub detected_at: Instant,
    pub stack_trace: Option<Vec<String>>,
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryUsageStats {
    pub current_usage: usize,
    pub peak_usage: usize,
    pub total_allocations: u64,
    pub total_deallocations: u64,
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub function_profiles: HashMap<String, FunctionProfile>,
    pub call_stack_samples: Vec<CallStackSample>,
    pub total_samples: usize,
}

// Implementation of helper structs
impl CallTracer {
    fn new() -> Self {
        Self {
            calls: Vec::new(),
            output: None,
            config: TracerConfig {
                enabled: false,
                max_calls: 10000,
                enable_stack_traces: true,
                trace_arguments: true,
                trace_return_values: true,
                output_file: None,
            },
        }
    }
    
    fn enable(&mut self, output_file: Option<String>) -> Result<(), CursedError> {
        self.config.enabled = true;
        self.config.output_file = output_file.clone();
        
        if let Some(file_path) = output_file {
            let file = File::create(file_path)
                .map_err(|e| CursedError::General(format!("Failed to create trace file: {}", e)))?;
            self.output = Some(BufWriter::new(file));
        }
        
        Ok(())
    }
    
    fn add_call(&mut self, call: TracedCall) -> Result<(), CursedError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Write to output file if configured
        if let Some(ref mut output) = self.output {
            writeln!(output, "{:?}", call)
                .map_err(|e| CursedError::General(format!("Failed to write trace: {}", e)))?;
            output.flush()
                .map_err(|e| CursedError::General(format!("Failed to flush trace: {}", e)))?;
        }
        
        self.calls.push(call);
        
        // Limit the number of stored calls
        if self.calls.len() > self.config.max_calls {
            self.calls.remove(0);
        }
        
        Ok(())
    }
    
    fn get_calls_for_function(&self, function_name: &str) -> Vec<TracedCall> {
        self.calls.iter()
            .filter(|call| call.function_name == function_name)
            .cloned()
            .collect()
    }
    
    fn export_to_file(&self, file_path: &str) -> Result<(), CursedError> {
        let json = serde_json::to_string_pretty(&self.calls)
            .map_err(|e| CursedError::General(format!("Failed to serialize trace data: {}", e)))?;
        
        std::fs::write(file_path, json)
            .map_err(|e| CursedError::General(format!("Failed to write trace export: {}", e)))?;
        
        Ok(())
    }
    
    fn clear(&mut self) {
        self.calls.clear();
    }
}

impl MemoryDebugger {
    fn new() -> Self {
        Self {
            operations: Vec::new(),
            leak_detector: LeakDetector::new(),
            overflow_detector: OverflowDetector::new(),
            usage_tracker: MemoryUsageTracker::new(),
        }
    }
    
    fn enable(&mut self) -> Result<(), CursedError> {
        // Enable memory debugging
        Ok(())
    }
    
    fn add_operation(&mut self, operation: MemoryOperation) -> Result<(), CursedError> {
        self.operations.push(operation.clone());
        
        // Update leak detector
        match operation.operation_type {
            MemoryOperationType::Allocate => {
                self.leak_detector.track_allocation(operation.address, operation.size);
            }
            MemoryOperationType::Deallocate => {
                self.leak_detector.track_deallocation(operation.address);
            }
            _ => {}
        }
        
        // Update usage tracker
        self.usage_tracker.update(&operation);
        
        Ok(())
    }
    
    fn export_to_file(&self, file_path: &str) -> Result<(), CursedError> {
        let json = serde_json::to_string_pretty(&self.operations)
            .map_err(|e| CursedError::General(format!("Failed to serialize memory data: {}", e)))?;
        
        std::fs::write(file_path, json)
            .map_err(|e| CursedError::General(format!("Failed to write memory export: {}", e)))?;
        
        Ok(())
    }
    
    fn clear(&mut self) {
        self.operations.clear();
        self.leak_detector.clear();
        self.overflow_detector.clear();
        self.usage_tracker.clear();
    }
}

impl LeakDetector {
    fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            leak_threshold: Duration::from_secs(300),
        }
    }
    
    fn track_allocation(&mut self, address: usize, size: usize) {
        let info = AllocationInfo {
            size,
            allocated_at: Instant::now(),
            stack_trace: None,
        };
        self.allocations.insert(address, info);
    }
    
    fn track_deallocation(&mut self, address: usize) {
        self.allocations.remove(&address);
    }
    
    fn detect_leaks(&self) -> Vec<MemoryLeak> {
        let now = Instant::now();
        self.allocations.iter()
            .filter(|(_, info)| now.duration_since(info.allocated_at) > self.leak_threshold)
            .map(|(&address, info)| MemoryLeak {
                address,
                size: info.size,
                age: now.duration_since(info.allocated_at),
                stack_trace: info.stack_trace.clone(),
            })
            .collect()
    }
    
    fn clear(&mut self) {
        self.allocations.clear();
    }
}

impl OverflowDetector {
    fn new() -> Self {
        Self {
            protected_regions: HashMap::new(),
            guard_pages: HashMap::new(),
        }
    }
    
    fn get_detections(&self) -> Vec<OverflowDetection> {
        // This would implement actual overflow detection
        Vec::new()
    }
    
    fn clear(&mut self) {
        self.protected_regions.clear();
        self.guard_pages.clear();
    }
}

impl MemoryUsageTracker {
    fn new() -> Self {
        Self {
            current_usage: 0,
            peak_usage: 0,
            usage_history: Vec::new(),
        }
    }
    
    fn update(&mut self, operation: &MemoryOperation) {
        match operation.operation_type {
            MemoryOperationType::Allocate => {
                self.current_usage += operation.size;
                if self.current_usage > self.peak_usage {
                    self.peak_usage = self.current_usage;
                }
            }
            MemoryOperationType::Deallocate => {
                self.current_usage = self.current_usage.saturating_sub(operation.size);
            }
            _ => {}
        }
        
        self.usage_history.push(MemoryUsageSnapshot {
            timestamp: operation.timestamp,
            usage: self.current_usage,
        });
    }
    
    fn get_stats(&self) -> MemoryUsageStats {
        MemoryUsageStats {
            current_usage: self.current_usage,
            peak_usage: self.peak_usage,
            total_allocations: self.usage_history.iter()
                .filter(|snapshot| snapshot.usage > 0)
                .count() as u64,
            total_deallocations: self.usage_history.iter()
                .filter(|snapshot| snapshot.usage == 0)
                .count() as u64,
        }
    }
    
    fn clear(&mut self) {
        self.current_usage = 0;
        self.peak_usage = 0;
        self.usage_history.clear();
    }
}

impl PerformanceProfiler {
    fn new() -> Self {
        Self {
            function_profiles: HashMap::new(),
            call_stack_samples: Vec::new(),
            config: ProfilerConfig {
                enabled: false,
                sampling_rate: Duration::from_millis(10),
                max_samples: 10000,
                enable_call_stack_sampling: true,
            },
        }
    }
    
    fn enable(&mut self) -> Result<(), CursedError> {
        self.config.enabled = true;
        Ok(())
    }
    
    fn update_function_profile(&mut self, function_name: &str, duration: Duration) -> Result<(), CursedError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        let profile = self.function_profiles.entry(function_name.to_string())
            .or_insert_with(|| FunctionProfile {
                name: function_name.to_string(),
                total_calls: 0,
                total_time: Duration::ZERO,
                average_time: Duration::ZERO,
                min_time: Duration::MAX,
                max_time: Duration::ZERO,
                time_distribution: Vec::new(),
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
        
        profile.time_distribution.push(duration);
        
        Ok(())
    }
    
    fn get_function_profile(&self, function_name: &str) -> Option<FunctionProfile> {
        self.function_profiles.get(function_name).cloned()
    }
    
    fn export_to_file(&self, file_path: &str) -> Result<(), CursedError> {
        let json = serde_json::to_string_pretty(&self.function_profiles)
            .map_err(|e| CursedError::General(format!("Failed to serialize performance data: {}", e)))?;
        
        std::fs::write(file_path, json)
            .map_err(|e| CursedError::General(format!("Failed to write performance export: {}", e)))?;
        
        Ok(())
    }
    
    fn clear(&mut self) {
        self.function_profiles.clear();
        self.call_stack_samples.clear();
    }
}

impl InteractiveDebugger {
    fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
            watches: Vec::new(),
            state: DebuggerState::Running,
        }
    }
    
    fn enable(&mut self) -> Result<(), CursedError> {
        self.state = DebuggerState::Running;
        Ok(())
    }
    
    fn clear(&mut self) {
        self.breakpoints.clear();
        self.watches.clear();
        self.state = DebuggerState::Running;
    }
}

impl Default for DebugTools {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_debug_tools_creation() {
        let tools = DebugTools::new();
        assert!(!tools.config.enable_debug);
    }
    
    #[test]
    fn test_call_tracer() {
        let mut tracer = CallTracer::new();
        assert!(!tracer.config.enabled);
        
        tracer.enable(None).unwrap();
        assert!(tracer.config.enabled);
        
        let call = TracedCall {
            function_name: "test_function".to_string(),
            arguments: vec![FfiValue::SignedInteger(42)],
            return_value: Some(FfiValue::SignedInteger(84)),
            timestamp: Instant::now(),
            duration: Duration::from_millis(10),
            result: CallResult::Success,
            stack_trace: None,
        };
        
        tracer.add_call(call).unwrap();
        assert_eq!(tracer.calls.len(), 1);
        
        let calls = tracer.get_calls_for_function("test_function");
        assert_eq!(calls.len(), 1);
    }
    
    #[test]
    fn test_memory_debugger() {
        let mut debugger = MemoryDebugger::new();
        
        let operation = MemoryOperation {
            operation_type: MemoryOperationType::Allocate,
            address: 0x1000,
            size: 1024,
            timestamp: Instant::now(),
            stack_trace: None,
        };
        
        debugger.add_operation(operation).unwrap();
        assert_eq!(debugger.operations.len(), 1);
        assert_eq!(debugger.usage_tracker.current_usage, 1024);
    }
    
    #[test]
    fn test_performance_profiler() {
        let mut profiler = PerformanceProfiler::new();
        profiler.enable().unwrap();
        
        let duration = Duration::from_millis(100);
        profiler.update_function_profile("test_function", duration).unwrap();
        
        let profile = profiler.get_function_profile("test_function").unwrap();
        assert_eq!(profile.total_calls, 1);
        assert_eq!(profile.total_time, duration);
        assert_eq!(profile.average_time, duration);
    }
    
    #[test]
    fn test_interactive_debugger() {
        let mut debugger = InteractiveDebugger::new();
        debugger.enable().unwrap();
        
        assert!(matches!(debugger.state, DebuggerState::Running));
        
        let breakpoint = Breakpoint {
            function_name: "test_function".to_string(),
            condition: None,
            enabled: true,
            hit_count: 0,
        };
        
        debugger.breakpoints.insert("test_function".to_string(), breakpoint);
        assert!(debugger.breakpoints.contains_key("test_function"));
    }
}
