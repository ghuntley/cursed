/// CPU Profiler Implementation for CURSED vibecheck
/// 
/// Provides comprehensive CPU profiling with function call tracing, timing analysis,
/// hot path identification, call graph generation, and performance bottleneck detection.

use crate::error::CursedError;
use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::time::{SystemTime, Duration, Instant};
use std::thread;
use std::fmt;
use std::backtrace::{Backtrace, BacktraceStatus};

/// CPU profiler configuration
#[derive(Debug, Clone)]
pub struct CpuProfilerConfig {
    /// Sampling rate in Hz (samples per second)
    /// Enable function call tracing
    /// Maximum call stack depth to record
    /// Minimum function duration to record (microseconds)
    /// Maximum number of samples to keep
    /// Enable call graph generation
    /// Track thread-specific profiles
impl Default for CpuProfilerConfig {
    fn default() -> Self {
        Self {
            sample_rate: 100, // 100 Hz
            min_function_duration: 1, // 1 microsecond
        }
    }
/// Function call record
#[derive(Debug, Clone)]
pub struct FunctionCall {
    /// Function name
    /// Module or file name
    /// Thread ID
    /// Start time
    /// Duration (None if still executing)
    /// Call depth in stack
    /// Child function calls
/// CPU sample record
#[derive(Debug, Clone)]
pub struct CpuSample {
    /// Sample timestamp
    /// Thread ID
    /// Stack trace at sample time
    /// CPU usage percentage at sample time
/// Call graph node
#[derive(Debug, Clone)]
pub struct CallGraphNode {
    /// Function name
    /// Total time spent in this function (inclusive)
    /// Time spent only in this function (exclusive)
    /// Number of calls to this function
    /// Callers of this function
    /// Functions called by this function
/// Hot path information
#[derive(Debug, Clone)]
pub struct HotPath {
    /// Call stack representing the hot path
    /// Total time spent in this path
    /// Number of samples in this path
    /// Percentage of total execution time
/// Performance bottleneck information
#[derive(Debug, Clone)]
pub struct PerformanceBottleneck {
    /// Function or code location
    /// Type of bottleneck
    /// Impact on performance (percentage)
    /// Suggested optimization
#[derive(Debug, Clone)]
pub enum BottleneckType {
    /// High CPU usage
    /// Frequent function calls
    /// Long execution time
    /// Deep recursion
    /// Lock contention
/// CPU profiling results
#[derive(Debug)]
pub struct CpuProfile {
    /// Function call traces
    /// CPU samples
    /// Call graph
    /// Hot paths
    /// Performance bottlenecks
    /// Profiling duration
    /// Total samples collected
/// CPU profiler implementation
pub struct CpuProfiler {
impl CpuProfiler {
    /// Create a new CPU profiler with default configuration
    pub fn new() -> Self {
        Self::with_config(CpuProfilerConfig::default())
    /// Create a new CPU profiler with custom configuration
    pub fn with_config(config: CpuProfilerConfig) -> Self {
        Self {
        }
    }

    /// Start CPU profiling
    pub fn start(&self) -> crate::error::Result<()> {
        if self.is_profiling.load(Ordering::SeqCst) {
            return Err(CursedError::Runtime("CPU profiler already running".to_string()));
        self.is_profiling.store(true, Ordering::SeqCst);
        
        {
            let mut start_time = self.start_time.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock start time".to_string()))?;
            *start_time = Some(Instant::now());
        // Clear previous data
        self.clear_data()?;

        // Start sampling thread
        if self.config.sample_rate > 0 {
            self.start_sampling_thread()?;
        Ok(())
    /// Stop CPU profiling and return results
    pub fn stop(&self) -> crate::error::Result<()> {
        if !self.is_profiling.load(Ordering::SeqCst) {
            return Err(CursedError::Runtime("CPU profiler not running".to_string()));
        self.is_profiling.store(false, Ordering::SeqCst);

        // Wait for sampling threads to complete
        {
            let mut threads = self.profiling_threads.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiling threads".to_string()))?;
            
            for thread in threads.drain(..) {
                let _ = thread.join();
            }
        }

        let profiling_duration = {
            let start_time = self.start_time.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock start time".to_string()))?;
            
            start_time.map(|start| start.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0))

        let profile = self.generate_profile(profiling_duration)?;
        Ok(profile)
    /// Record function entry
    pub fn enter_function(&self, name: String, module: String) -> crate::error::Result<()> {
        if !self.is_profiling.load(Ordering::SeqCst) || !self.config.function_tracing {
            return Ok(());
        let thread_id = thread::current().id();
        let mut call_stack = self.call_stack.write()
            .map_err(|_| CursedError::Runtime("Failed to lock call stack".to_string()))?;

        let stack = call_stack.entry(thread_id).or_insert_with(Vec::new);
        
        if stack.len() >= self.config.max_stack_depth {
            return Ok(()); // Ignore deeply nested calls
        let function_call = FunctionCall {

        stack.push(function_call);
        Ok(())
    /// Record function exit
    pub fn exit_function(&self) -> crate::error::Result<()> {
        if !self.is_profiling.load(Ordering::SeqCst) || !self.config.function_tracing {
            return Ok(());
        let thread_id = thread::current().id();
        let mut call_stack = self.call_stack.write()
            .map_err(|_| CursedError::Runtime("Failed to lock call stack".to_string()))?;

        if let Some(stack) = call_stack.get_mut(&thread_id) {
            if let Some(mut function_call) = stack.pop() {
                let duration = function_call.start_time.elapsed();
                function_call.duration = Some(duration);

                // Only record if duration exceeds minimum threshold
                if duration.as_micros() >= self.config.min_function_duration as u128 {
                    // Add to parent's children or to main list
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(function_call);
                    } else {
                        let mut function_calls = self.function_calls.write()
                            .map_err(|_| CursedError::Runtime("Failed to lock function calls".to_string()))?;
                        
                        if function_calls.len() < self.config.max_samples {
                            function_calls.push(function_call);
                        }
                    }
                }
            }
        Ok(())
    /// Generate complete CPU profile
    fn generate_profile(&self, profiling_duration: Duration) -> crate::error::Result<()> {
        let function_calls = {
            let calls = self.function_calls.read()
                .map_err(|_| CursedError::Runtime("Failed to lock function calls".to_string()))?;
            calls.clone()

        let samples = {
            let samples = self.samples.read()
                .map_err(|_| CursedError::Runtime("Failed to lock samples".to_string()))?;
            samples.clone()

        let call_graph = self.build_call_graph(&function_calls)?;
        let hot_paths = self.identify_hot_paths(&samples)?;
        let bottlenecks = self.detect_bottlenecks(&function_calls, &call_graph)?;

        Ok(CpuProfile {
        })
    /// Build call graph from function calls
    fn build_call_graph(&self, function_calls: &[FunctionCall]) -> crate::error::Result<()> {
        if !self.config.call_graph {
            return Ok(HashMap::new());
        let mut call_graph = HashMap::new();

        fn process_call(call: &FunctionCall, parent: Option<&str>, graph: &mut HashMap<String, CallGraphNode>) {
            let node = graph.entry(call.name.clone()).or_insert_with(|| CallGraphNode {
            });

            node.call_count += 1;

            if let Some(duration) = call.duration {
                node.inclusive_time += duration;
                
                // Calculate exclusive time (duration minus children)
                let children_time: Duration = call.children.iter()
                    .filter_map(|child| child.duration)
                    .sum();
                node.exclusive_time += duration.saturating_sub(children_time);
            // Record caller relationship
            if let Some(parent_name) = parent {
                *node.callers.entry(parent_name.to_string()).or_insert(0) += 1;
                
                // Record callee relationship in parent
                if let Some(parent_node) = graph.get_mut(parent_name) {
                    *parent_node.callees.entry(call.name.clone()).or_insert(0) += 1;
                }
            }

            // Process children
            for child in &call.children {
                process_call(child, Some(&call.name), graph);
            }
        }

        for call in function_calls {
            process_call(call, None, &mut call_graph);
        Ok(call_graph)
    /// Identify hot paths from CPU samples
    fn identify_hot_paths(&self, samples: &[CpuSample]) -> crate::error::Result<()> {
        let mut path_counts = HashMap::new();
        let total_samples = samples.len() as u64;

        for sample in samples {
            let path_key = sample.stack_trace.join(" -> ");
            let entry = path_counts.entry(path_key.clone()).or_insert((0u64, Duration::from_nanos(0)));
            entry.0 += 1;
            // Estimate time based on sampling rate
            entry.1 += Duration::from_nanos(1_000_000_000 / self.config.sample_rate as u64);
        let mut hot_paths = Vec::new();
        for (path_key, (count, total_time)) in path_counts {
            let percentage = if total_samples > 0 {
                (count as f64 / total_samples as f64) * 100.0
            } else {
                0.0

            if percentage >= 1.0 { // Only include paths with >= 1% of samples
                hot_paths.push(HotPath {
                });
            }
        }

        // Sort by percentage (highest first)
        hot_paths.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap_or(std::cmp::Ordering::Equal));
        hot_paths.truncate(20); // Top 20 hot paths

        Ok(hot_paths)
    /// Detect performance bottlenecks
    fn detect_bottlenecks(&self, function_calls: &[FunctionCall], call_graph: &HashMap<String, CallGraphNode>) -> crate::error::Result<()> {
        let mut bottlenecks = Vec::new();

        // Find CPU-intensive functions
        let total_time: Duration = call_graph.values()
            .map(|node| node.exclusive_time)
            .sum();

        for (function_name, node) in call_graph {
            let time_percentage = if total_time.as_nanos() > 0 {
                (node.exclusive_time.as_nanos() as f64 / total_time.as_nanos() as f64) * 100.0
            } else {
                0.0

            // CPU-intensive functions (>10% of total time)
            if time_percentage > 10.0 {
                bottlenecks.push(PerformanceBottleneck {
                });
            // High call frequency (>1000 calls)
            if node.call_count > 1000 {
                bottlenecks.push(PerformanceBottleneck {
                    impact: node.call_count as f64 / 100.0, // Scale to percentage
                });
            // Long execution time (>100ms average)
            let avg_time = if node.call_count > 0 {
                node.inclusive_time.as_millis() / node.call_count as u128
            } else {
                0

            if avg_time > 100 {
                bottlenecks.push(PerformanceBottleneck {
                    impact: avg_time as f64 / 10.0, // Scale to percentage
                });
            }
        }

        // Detect deep recursion
        fn check_recursion_depth(call: &FunctionCall, current_depth: usize) -> usize {
            let max_child_depth = call.children.iter()
                .map(|child| check_recursion_depth(child, current_depth + 1))
                .max()
                .unwrap_or(current_depth);
            max_child_depth
        for call in function_calls {
            let max_depth = check_recursion_depth(call, 0);
            if max_depth > 50 {
                bottlenecks.push(PerformanceBottleneck {
                    impact: max_depth as f64 / 10.0,
                });
            }
        }

        // Sort by impact (highest first)
        bottlenecks.sort_by(|a, b| b.impact.partial_cmp(&a.impact).unwrap_or(std::cmp::Ordering::Equal));
        bottlenecks.truncate(10); // Top 10 bottlenecks

        Ok(bottlenecks)
    /// Start CPU sampling thread
    fn start_sampling_thread(&self) -> crate::error::Result<()> {
        let is_profiling = Arc::new(AtomicBool::new(true));
        let is_profiling_clone = is_profiling.clone();
        let samples_clone = self.samples.clone();
        let sample_count_clone = self.sample_count.clone();
        let config = self.config.clone();
        let profiler_is_profiling = &self.is_profiling;

        let sampling_thread = thread::spawn(move || {
            let sample_interval = Duration::from_nanos(1_000_000_000 / config.sample_rate as u64);
            
            while is_profiling_clone.load(Ordering::SeqCst) {
                // Capture stack trace
                let stack_trace = capture_stack_trace();
                
                let sample = CpuSample {

                // Store sample
                if let Ok(mut samples) = samples_clone.write() {
                    if samples.len() < config.max_samples {
                        samples.push(sample);
                        sample_count_clone.fetch_add(1, Ordering::SeqCst);
                    }
                }

                thread::sleep(sample_interval);
            }
        });

        {
            let mut threads = self.profiling_threads.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiling threads".to_string()))?;
            threads.push(sampling_thread);
        Ok(())
    /// Clear all profiling data
    fn clear_data(&self) -> crate::error::Result<()> {
        {
            let mut function_calls = self.function_calls.write()
                .map_err(|_| CursedError::Runtime("Failed to lock function calls".to_string()))?;
            function_calls.clear();
        {
            let mut samples = self.samples.write()
                .map_err(|_| CursedError::Runtime("Failed to lock samples".to_string()))?;
            samples.clear();
        {
            let mut call_stack = self.call_stack.write()
                .map_err(|_| CursedError::Runtime("Failed to lock call stack".to_string()))?;
            call_stack.clear();
        self.sample_count.store(0, Ordering::SeqCst);

        Ok(())
    }
}

/// Capture current stack trace
fn capture_stack_trace() -> Vec<String> {
    let backtrace = Backtrace::capture();
    match backtrace.status() {
        BacktraceStatus::Captured => {
            backtrace.to_string()
                .lines()
                .skip(2) // Skip capture_stack_trace and sampling function
                .take(10) // Limit to top 10 frames
                .map(|line| {
                    // Extract function name from backtrace line
                    line.trim()
                        .split_whitespace()
                        .last()
                        .unwrap_or("unknown")
                        .to_string()
                })
                .collect()
        }
        _ => vec!["stack_trace_unavailable".to_string()]
    }
}

/// Get current CPU usage (simplified implementation)
fn get_cpu_usage() -> f64 {
    // This would require platform-specific implementation
    // For now, return a mock value
    50.0
impl fmt::Display for CpuProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== CURSED CPU Profile ===")?;
        writeln!(f)?;
        writeln!(f, "Profiling Duration: {:.2}s", self.profiling_duration.as_secs_f64())?;
        writeln!(f, "Total Samples: {}", self.total_samples)?;
        writeln!(f, "Function Calls: {}", self.function_calls.len())?;
        writeln!(f)?;

        writeln!(f, "Hot Paths:")?;
        for (i, hot_path) in self.hot_paths.iter().take(10).enumerate() {
                hot_path.call_stack.join(" -> "))?;
        }
        writeln!(f)?;

        writeln!(f, "Performance Bottlenecks:")?;
        for (i, bottleneck) in self.bottlenecks.iter().take(5).enumerate() {
                i + 1, bottleneck.location, bottleneck.impact)?;
            writeln!(f, "     {}", bottleneck.suggestion)?;
        }
        writeln!(f)?;

        writeln!(f, "Top Functions by Exclusive Time:")?;
        let mut functions: Vec<_> = self.call_graph.values().collect();
        functions.sort_by(|a, b| b.exclusive_time.cmp(&a.exclusive_time));
        
        for (i, function) in functions.iter().take(10).enumerate() {
                function.call_count)?;
        Ok(())
    }
}

/// Global CPU profiler instance
static GLOBAL_CPU_PROFILER: std::sync::OnceLock<Arc<CpuProfiler>> = std::sync::OnceLock::new();

/// Get or create the global CPU profiler
pub fn get_cpu_profiler() -> Arc<CpuProfiler> {
    GLOBAL_CPU_PROFILER.get_or_init(|| {
        Arc::new(CpuProfiler::new())
    }).clone()
/// Configure CPU profiler
pub fn configure_cpu_profiler(config: CpuProfilerConfig) -> crate::error::Result<()> {
    let profiler = Arc::new(CpuProfiler::with_config(config));
    GLOBAL_CPU_PROFILER.set(profiler)
        .map_err(|_| CursedError::Runtime("CPU profiler already configured".to_string()))?;
    Ok(())
/// Start CPU profiling
pub fn start_cpu_profiling() -> crate::error::Result<()> {
    let profiler = get_cpu_profiler();
    profiler.start()
/// Stop CPU profiling and return results
pub fn stop_cpu_profiling() -> crate::error::Result<()> {
    let profiler = get_cpu_profiler();
    profiler.stop()
/// Record function entry (for manual instrumentation)
pub fn profile_function_enter(name: String, module: String) -> crate::error::Result<()> {
    let profiler = get_cpu_profiler();
    profiler.enter_function(name, module)
/// Record function exit (for manual instrumentation)
pub fn profile_function_exit() -> crate::error::Result<()> {
    let profiler = get_cpu_profiler();
    profiler.exit_function()
/// Function profiling guard for RAII-style profiling
pub struct FunctionProfileGuard {
impl FunctionProfileGuard {
    /// Create a new function profile guard
    pub fn new(name: String, module: String) -> crate::error::Result<()> {
        profile_function_enter(name, module)?;
        Ok(Self { _phantom: std::marker::PhantomData })
    }
}

impl Drop for FunctionProfileGuard {
    fn drop(&mut self) {
        let _ = profile_function_exit();
    }
}

/// Macro for easy function profiling
#[macro_export]
macro_rules! profile_function {
    () => {
//         let _guard = $crate::stdlib::vibecheck::cpu_profiler::FunctionProfileGuard::new(
            module_path!().to_string()
        )?;
    ($name:expr) => {
//         let _guard = $crate::stdlib::vibecheck::cpu_profiler::FunctionProfileGuard::new(
            module_path!().to_string()
        )?;
