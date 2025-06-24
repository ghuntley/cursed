use crate::error::Error;
/// Core optimization types for the CURSED compiler
/// 
/// This module provides type definitions for optimization-related functionality
/// that are commonly referenced but may be missing from specific modules.

use std::collections::HashMap;
use std::time::Duration;

/// Comprehensive optimization result containing all optimization metrics
#[derive(Debug, Clone)]
pub struct ComprehensiveOptimizationResult {
    /// Overall optimization success status
    pub success: bool,
    /// Individual optimization pass results
    pub pass_results: Vec<OptimizationPassResult>,
    /// Performance improvements achieved
    pub performance_improvement: f64,
    /// Compilation time impact
    pub compilation_time_impact: Duration,
    /// Memory usage impact
    pub memory_impact: i64,
    /// Code size impact
    pub code_size_impact: i64,
    /// Optimization recommendations
    pub recommendations: Vec<String>,
    /// Detailed metrics
    pub metrics: OptimizationMetrics,
}

impl ComprehensiveOptimizationResult {
    /// Create a new comprehensive optimization result
    pub fn new() -> Self {
        Self {
            success: false,
            pass_results: Vec::new(),
            performance_improvement: 0.0,
            compilation_time_impact: Duration::from_secs(0),
            memory_impact: 0,
            code_size_impact: 0,
            recommendations: Vec::new(),
            metrics: OptimizationMetrics::new(),
        }
    }

    /// Create a successful result
    pub fn success() -> Self {
        Self {
            success: true,
            ..Self::new()
        }
    }

    /// Create a failed result with error message
    pub fn failure(error: String) -> Self {
        let mut result = Self::new();
        result.recommendations.push(format!("Error: {}", error));
        result
    }

    /// Add a pass result
    pub fn add_pass_result(&mut self, pass_result: OptimizationPassResult) {
        self.success = self.success && pass_result.success;
        self.pass_results.push(pass_result);
    }

    /// Set performance improvement
    pub fn set_performance_improvement(&mut self, improvement: f64) {
        self.performance_improvement = improvement;
    }

    /// Add recommendation
    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }
}

impl Default for ComprehensiveOptimizationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of an individual optimization pass
#[derive(Debug, Clone)]
pub struct OptimizationPassResult {
    /// Name of the optimization pass
    pub pass_name: String,
    /// Whether the pass succeeded
    pub success: bool,
    /// Time taken by the pass
    pub execution_time: Duration,
    /// Number of optimizations applied
    pub optimizations_applied: usize,
    /// Performance improvement from this pass
    pub performance_delta: f64,
    /// Error message if failed
    pub error_message: Option<String>,
}

impl OptimizationPassResult {
    /// Create a successful pass result
    pub fn success(pass_name: String, optimizations_applied: usize) -> Self {
        Self {
            pass_name,
            success: true,
            execution_time: Duration::from_millis(0),
            optimizations_applied,
            performance_delta: 0.0,
            error_message: None,
        }
    }

    /// Create a failed pass result
    pub fn failure(pass_name: String, error: String) -> Self {
        Self {
            pass_name,
            success: false,
            execution_time: Duration::from_millis(0),
            optimizations_applied: 0,
            performance_delta: 0.0,
            error_message: Some(error),
        }
    }
}

/// Detailed optimization metrics
#[derive(Debug, Clone)]
pub struct OptimizationMetrics {
    /// Instructions eliminated
    pub instructions_eliminated: usize,
    /// Functions inlined
    pub functions_inlined: usize,
    /// Dead code blocks removed
    pub dead_code_blocks: usize,
    /// Constants folded
    pub constants_folded: usize,
    /// Loops optimized
    pub loops_optimized: usize,
    /// Memory allocations optimized
    pub allocations_optimized: usize,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

impl OptimizationMetrics {
    pub fn new() -> Self {
        Self {
            instructions_eliminated: 0,
            functions_inlined: 0,
            dead_code_blocks: 0,
            constants_folded: 0,
            loops_optimized: 0,
            allocations_optimized: 0,
            custom_metrics: HashMap::new(),
        }
    }

    /// Add a custom metric
    pub fn add_custom_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
    }

    /// Get total optimizations count
    pub fn total_optimizations(&self) -> usize {
        self.instructions_eliminated + 
        self.functions_inlined + 
        self.dead_code_blocks + 
        self.constants_folded + 
        self.loops_optimized + 
        self.allocations_optimized
    }
}

impl Default for OptimizationMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Benchmark suite for performance testing
#[derive(Debug, Clone)]
pub struct BenchmarkSuite {
    /// Name of the benchmark suite
    pub name: String,
    /// Individual benchmarks
    pub benchmarks: Vec<Benchmark>,
    /// Suite configuration
    pub config: BenchmarkConfig,
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(name: String) -> Self {
        Self {
            name,
            benchmarks: Vec::new(),
            config: BenchmarkConfig::default(),
        }
    }

    /// Add a benchmark to the suite
    pub fn add_benchmark(&mut self, benchmark: Benchmark) {
        self.benchmarks.push(benchmark);
    }

    /// Run all benchmarks in the suite
    pub fn run(&self) -> BenchmarkSuiteResults {
        let mut results = BenchmarkSuiteResults::new(self.name.clone());
        
        for benchmark in &self.benchmarks {
            let result = benchmark.run(&self.config);
            results.add_result(result);
        }
        
        results
    }
}

/// Individual benchmark
#[derive(Debug, Clone)]
pub struct Benchmark {
    /// Name of the benchmark
    pub name: String,
    /// Description of what this benchmark tests
    pub description: String,
    /// Function to execute for benchmarking
    pub test_function: String,
    /// Expected performance baseline
    pub baseline: Option<Duration>,
}

impl Benchmark {
    /// Create a new benchmark
    pub fn new(name: String, test_function: String) -> Self {
        Self {
            name,
            description: String::new(),
            test_function,
            baseline: None,
        }
    }

    /// Run this benchmark
    pub fn run(&self, _config: &BenchmarkConfig) -> BenchmarkResult {
        // Mock implementation - in real usage this would execute the test function
        BenchmarkResult {
            benchmark_name: self.name.clone(),
            execution_time: Duration::from_millis(100),
            success: true,
            error_message: None,
            memory_usage: 1024,
            cpu_usage: 0.5,
        }
    }
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of iterations to run each benchmark
    pub iterations: usize,
    /// Timeout for each benchmark
    pub timeout: Duration,
    /// Whether to collect memory statistics
    pub collect_memory_stats: bool,
    /// Whether to collect CPU statistics
    pub collect_cpu_stats: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 10,
            timeout: Duration::from_secs(30),
            collect_memory_stats: true,
            collect_cpu_stats: true,
        }
    }
}

/// Results from running a benchmark suite
#[derive(Debug, Clone)]
pub struct BenchmarkSuiteResults {
    /// Name of the benchmark suite
    pub suite_name: String,
    /// Individual benchmark results
    pub results: Vec<BenchmarkResult>,
    /// Overall suite success
    pub success: bool,
    /// Total execution time
    pub total_time: Duration,
}

impl BenchmarkSuiteResults {
    /// Create new benchmark suite results
    pub fn new(suite_name: String) -> Self {
        Self {
            suite_name,
            results: Vec::new(),
            success: true,
            total_time: Duration::from_secs(0),
        }
    }

    /// Add a benchmark result
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.success = self.success && result.success;
        self.total_time += result.execution_time;
        self.results.push(result);
    }

    /// Get average execution time
    pub fn average_execution_time(&self) -> Duration {
        if self.results.is_empty() {
            Duration::from_secs(0)
        } else {
            self.total_time / self.results.len() as u32
        }
    }
}

/// Result from running a single benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name of the benchmark
    pub benchmark_name: String,
    /// Time taken to execute
    pub execution_time: Duration,
    /// Whether the benchmark succeeded
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Memory usage during execution
    pub memory_usage: usize,
    /// CPU usage during execution
    pub cpu_usage: f64,
}

/// Performance profiler for optimization analysis
#[derive(Debug, Clone)]
pub struct PerformanceProfiler {
    /// Profile data collected
    pub profiles: Vec<ProfileData>,
    /// Profiling configuration
    pub config: ProfilerConfig,
    /// Current profiling session
    pub current_session: Option<ProfilingSession>,
}

impl PerformanceProfiler {
    /// Create a new performance profiler
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            config: ProfilerConfig::default(),
            current_session: None,
        }
    }

    /// Start a profiling session
    pub fn start_session(&mut self, name: String) {
        self.current_session = Some(ProfilingSession::new(name));
    }

    /// End the current profiling session
    pub fn end_session(&mut self) -> Option<ProfileData> {
        if let Some(session) = self.current_session.take() {
            let profile_data = session.finish();
            self.profiles.push(profile_data.clone());
            Some(profile_data)
        } else {
            None
        }
    }

    /// Get profiling results
    pub fn get_results(&self) -> &[ProfileData] {
        &self.profiles
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Profiler configuration
#[derive(Debug, Clone)]
pub struct ProfilerConfig {
    /// Whether to collect CPU profiling data
    pub collect_cpu_profile: bool,
    /// Whether to collect memory profiling data
    pub collect_memory_profile: bool,
    /// Sampling frequency for profiling
    pub sampling_frequency: Duration,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            collect_cpu_profile: true,
            collect_memory_profile: true,
            sampling_frequency: Duration::from_millis(10),
        }
    }
}

/// Profiling session
#[derive(Debug, Clone)]
pub struct ProfilingSession {
    /// Name of the session
    pub name: String,
    /// Start time
    pub start_time: std::time::Instant,
    /// Collected samples
    pub samples: Vec<ProfileSample>,
}

impl ProfilingSession {
    /// Create a new profiling session
    pub fn new(name: String) -> Self {
        Self {
            name,
            start_time: std::time::Instant::now(),
            samples: Vec::new(),
        }
    }

    /// Finish the profiling session
    pub fn finish(self) -> ProfileData {
        ProfileData {
            session_name: self.name,
            duration: self.start_time.elapsed(),
            samples: self.samples,
            cpu_usage: 0.0, // Mock data
            memory_usage: 0, // Mock data
        }
    }
}

/// Profile data collected from a session
#[derive(Debug, Clone)]
pub struct ProfileData {
    /// Name of the profiling session
    pub session_name: String,
    /// Total duration of the session
    pub duration: Duration,
    /// Profile samples collected
    pub samples: Vec<ProfileSample>,
    /// Average CPU usage
    pub cpu_usage: f64,
    /// Peak memory usage
    pub memory_usage: usize,
}

/// Individual profile sample
#[derive(Debug, Clone)]
pub struct ProfileSample {
    /// Timestamp of the sample
    pub timestamp: Duration,
    /// CPU usage at this sample
    pub cpu_usage: f64,
    /// Memory usage at this sample
    pub memory_usage: usize,
    /// Function being executed
    pub function_name: Option<String>,
}

/// Incremental compilation result
#[derive(Debug, Clone)]
pub struct IncrementalCompilationResult {
    /// Whether incremental compilation was successful
    pub success: bool,
    /// Files that were recompiled
    pub recompiled_files: Vec<String>,
    /// Files that were skipped due to being up-to-date
    pub skipped_files: Vec<String>,
    /// Time saved by incremental compilation
    pub time_saved: Duration,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Error message if failed
    pub error_message: Option<String>,
}

impl IncrementalCompilationResult {
    /// Create a successful incremental compilation result
    pub fn success(
        recompiled_files: Vec<String>,
        skipped_files: Vec<String>,
        time_saved: Duration,
        cache_hit_rate: f64,
    ) -> Self {
        Self {
            success: true,
            recompiled_files,
            skipped_files,
            time_saved,
            cache_hit_rate,
            error_message: None,
        }
    }

    /// Create a failed incremental compilation result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            recompiled_files: Vec::new(),
            skipped_files: Vec::new(),
            time_saved: Duration::from_secs(0),
            cache_hit_rate: 0.0,
            error_message: Some(error),
        }
    }
}

/// Adaptation result for dynamic optimization
#[derive(Debug, Clone)]
pub struct AdaptationResult {
    /// Whether adaptation was successful
    pub success: bool,
    /// New optimization strategy chosen
    pub new_strategy: String,
    /// Reason for adaptation
    pub adaptation_reason: String,
    /// Expected performance improvement
    pub expected_improvement: f64,
    /// Confidence in the adaptation decision
    pub confidence: f64,
}

impl AdaptationResult {
    /// Create a successful adaptation result
    pub fn success(
        new_strategy: String,
        reason: String,
        expected_improvement: f64,
        confidence: f64,
    ) -> Self {
        Self {
            success: true,
            new_strategy,
            adaptation_reason: reason,
            expected_improvement,
            confidence,
        }
    }

    /// Create a failed adaptation result
    pub fn failure(reason: String) -> Self {
        Self {
            success: false,
            new_strategy: String::new(),
            adaptation_reason: reason,
            expected_improvement: 0.0,
            confidence: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_optimization_result() {
        let mut result = ComprehensiveOptimizationResult::new();
        assert!(!result.success);
        
        result.add_pass_result(OptimizationPassResult::success("test".to_string(), 5));
        result.set_performance_improvement(0.15);
        
        assert!(result.success);
        assert_eq!(result.performance_improvement, 0.15);
        assert_eq!(result.pass_results.len(), 1);
    }

    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new("test_suite".to_string());
        let benchmark = Benchmark::new("test_bench".to_string(), "test_func".to_string());
        suite.add_benchmark(benchmark);
        
        let results = suite.run();
        assert_eq!(results.results.len(), 1);
        assert!(results.success);
    }

    #[test]
    fn test_performance_profiler() {
        let mut profiler = PerformanceProfiler::new();
        profiler.start_session("test_session".to_string());
        
        let profile_data = profiler.end_session();
        assert!(profile_data.is_some());
        assert_eq!(profiler.get_results().len(), 1);
    }
}
