//! Enhanced performance monitoring and metrics collection for optimization
//! 
//! This module provides comprehensive performance monitoring capabilities
//! for tracking optimization effectiveness and compilation performance.

use crate::error::{CursedError, Result};
use crate::optimization::config::OptimizationConfig;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;

/// Enhanced performance monitor for optimization processes
pub struct EnhancedPerformanceMonitor {
    config: OptimizationConfig,
    timing_data: Arc<Mutex<TimingData>>,
    enabled: bool,
    benchmarking_enabled: bool,
    baseline_metrics: Option<BaselineMetrics>,
    performance_thresholds: PerformanceThresholds,
}

impl EnhancedPerformanceMonitor {
    /// Create a new enhanced performance monitor
    pub fn new(config: OptimizationConfig) -> Result<Self> {
        Ok(Self {
            config,
            timing_data: Arc::new(Mutex::new(TimingData::new())),
            enabled: true,
            benchmarking_enabled: false,
            baseline_metrics: None,
            performance_thresholds: PerformanceThresholds::default(),
        })
    }
    
    /// Enable or disable performance monitoring
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Enable or disable benchmarking
    pub fn set_benchmarking_enabled(&mut self, enabled: bool) {
        self.benchmarking_enabled = enabled;
    }
    
    /// Set baseline metrics for comparison
    pub fn set_baseline_metrics(&mut self, metrics: BaselineMetrics) {
        self.baseline_metrics = Some(metrics);
    }
    
    /// Set performance thresholds
    pub fn set_performance_thresholds(&mut self, thresholds: PerformanceThresholds) {
        self.performance_thresholds = thresholds;
    }
    
    /// Start timing an operation
    pub fn start_timing(&self, operation: &str) -> TimingToken {
        if !self.enabled {
            return TimingToken::disabled();
        }
        
        TimingToken::new(operation.to_string(), self.timing_data.clone())
    }
    
    /// Record optimization metrics
    pub fn record_optimization(&mut self, metrics: &OptimizationResult) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let mut timing_data = self.timing_data.lock().unwrap();
        timing_data.record_optimization_result(metrics);
        
        // Check performance thresholds
        if let Err(warning) = self.check_performance_thresholds(metrics) {
            eprintln!("Performance warning: {}", warning);
        }
        
        Ok(())
    }
    
    /// Check performance against thresholds
    fn check_performance_thresholds(&self, metrics: &OptimizationResult) -> Result<()> {
        if metrics.duration > self.performance_thresholds.max_optimization_time {
            return Err(CursedError::runtime_error(&format!(
                "Optimization {} took {:?}, exceeding threshold of {:?}",
                metrics.optimization_type,
                metrics.duration,
                self.performance_thresholds.max_optimization_time
            )));
        }
        
        if let Some(memory_usage) = metrics.metrics.get("memory_usage") {
            if *memory_usage > self.performance_thresholds.max_memory_usage as f64 {
                return Err(CursedError::runtime_error(&format!(
                    "Optimization {} used {} MB memory, exceeding threshold of {} MB",
                    metrics.optimization_type,
                    memory_usage,
                    self.performance_thresholds.max_memory_usage
                )));
            }
        }
        
        Ok(())
    }
    
    /// Get performance summary
    pub fn get_performance_summary(&self) -> PerformanceSummary {
        let timing_data = self.timing_data.lock().unwrap();
        
        PerformanceSummary {
            total_optimizations: timing_data.operation_counts.values().sum(),
            total_time: timing_data.total_time,
            average_time: timing_data.average_time(),
            operation_breakdown: timing_data.operation_times.clone(),
            cache_hit_rate: timing_data.cache_hit_rate,
            successful_optimizations: timing_data.successful_optimizations,
            failed_optimizations: timing_data.failed_optimizations,
            peak_memory_usage: timing_data.peak_memory_usage,
            regression_count: timing_data.regression_count,
        }
    }
    
    /// Generate performance report
    pub fn generate_report(&self) -> Result<String> {
        let summary = self.get_performance_summary();
        
        let mut report = String::new();
        report.push_str("=== CURSED Enhanced Optimization Performance Report ===\n\n");
        
        report.push_str(&format!("Total Optimizations: {}\n", summary.total_optimizations));
        report.push_str(&format!("Successful: {}\n", summary.successful_optimizations));
        report.push_str(&format!("Failed: {}\n", summary.failed_optimizations));
        report.push_str(&format!("Success Rate: {:.2}%\n", 
            if summary.total_optimizations > 0 {
                (summary.successful_optimizations as f64 / summary.total_optimizations as f64) * 100.0
            } else {
                0.0
            }
        ));
        
        report.push_str(&format!("Total Time: {:?}\n", summary.total_time));
        report.push_str(&format!("Average Time: {:?}\n", summary.average_time));
        report.push_str(&format!("Cache Hit Rate: {:.2}%\n", summary.cache_hit_rate * 100.0));
        report.push_str(&format!("Peak Memory Usage: {} MB\n", summary.peak_memory_usage));
        report.push_str(&format!("Regression Count: {}\n", summary.regression_count));
        
        report.push_str("\nOperation Breakdown:\n");
        let mut operations: Vec<_> = summary.operation_breakdown.iter().collect();
        operations.sort_by(|a, b| b.1.cmp(a.1));
        
        for (op, time) in operations {
            report.push_str(&format!("  {}: {:?}\n", op, time));
        }
        
        // Add baseline comparison if available
        if let Some(baseline) = &self.baseline_metrics {
            report.push_str("\nBaseline Comparison:\n");
            report.push_str(&format!("  Compile Time Change: {:.2}%\n", 
                ((summary.total_time.as_secs_f64() - baseline.compile_time.as_secs_f64()) / baseline.compile_time.as_secs_f64()) * 100.0
            ));
            report.push_str(&format!("  Memory Usage Change: {:.2}%\n", 
                ((summary.peak_memory_usage as f64 - baseline.memory_usage as f64) / baseline.memory_usage as f64) * 100.0
            ));
        }
        
        // Add performance recommendations
        report.push_str("\nRecommendations:\n");
        report.push_str(&self.generate_recommendations(&summary));
        
        Ok(report)
    }
    
    /// Generate performance recommendations
    fn generate_recommendations(&self, summary: &PerformanceSummary) -> String {
        let mut recommendations = String::new();
        
        if summary.cache_hit_rate < 0.5 {
            recommendations.push_str("  • Consider increasing cache size to improve hit rate\n");
        }
        
        if summary.failed_optimizations > summary.successful_optimizations / 10 {
            recommendations.push_str("  • High failure rate - review optimization configurations\n");
        }
        
        if summary.peak_memory_usage > 1000 {
            recommendations.push_str("  • High memory usage - consider memory optimization strategies\n");
        }
        
        if summary.regression_count > 0 {
            recommendations.push_str("  • Performance regressions detected - review recent changes\n");
        }
        
        let avg_time_ms = summary.average_time.as_millis();
        if avg_time_ms > 1000 {
            recommendations.push_str("  • Consider parallel optimization to reduce compilation time\n");
        }
        
        if recommendations.is_empty() {
            recommendations.push_str("  • Performance looks good - no immediate recommendations\n");
        }
        
        recommendations
    }
    
    /// Export performance data to file
    pub fn export_to_file(&self, filename: &str) -> Result<()> {
        let report = self.generate_report()?;
        let mut file = File::create(filename)?;
        file.write_all(report.as_bytes())?;
        
        Ok(())
    }
    
    /// Run performance benchmark
    pub fn run_benchmark(&mut self, benchmark_config: BenchmarkConfig) -> Result<BenchmarkResult> {
        if !self.benchmarking_enabled {
            return Err(CursedError::runtime_error("Benchmarking is not enabled"));
        }
        
        let start_time = Instant::now();
        let mut results = Vec::new();
        
        for iteration in 0..benchmark_config.iterations {
            let iteration_start = Instant::now();
            
            // Run benchmark iteration
            let iteration_result = self.run_benchmark_iteration(&benchmark_config, iteration)?;
            results.push(iteration_result);
            
            // Warmup check
            if iteration < benchmark_config.warmup_iterations {
                continue;
            }
            
            // Timeout check
            if start_time.elapsed() > benchmark_config.timeout {
                break;
            }
        }
        
        // Calculate statistics
        let valid_results: Vec<_> = results.into_iter().skip(benchmark_config.warmup_iterations).collect();
        let total_time = start_time.elapsed();
        
        if valid_results.is_empty() {
            return Err(CursedError::runtime_error("No valid benchmark results"));
        }
        
        let avg_time = valid_results.iter().map(|r| r.duration).sum::<Duration>() / valid_results.len() as u32;
        let min_time = valid_results.iter().map(|r| r.duration).min().unwrap();
        let max_time = valid_results.iter().map(|r| r.duration).max().unwrap();
        
        let avg_memory = valid_results.iter().map(|r| r.memory_usage).sum::<usize>() / valid_results.len();
        let peak_memory = valid_results.iter().map(|r| r.memory_usage).max().unwrap();
        
        Ok(BenchmarkResult {
            config: benchmark_config,
            total_time,
            avg_time,
            min_time,
            max_time,
            avg_memory,
            peak_memory,
            iterations_run: valid_results.len(),
            success_rate: valid_results.iter().filter(|r| r.success).count() as f64 / valid_results.len() as f64,
        })
    }
    
    /// Run single benchmark iteration
    fn run_benchmark_iteration(&self, _config: &BenchmarkConfig, _iteration: usize) -> Result<BenchmarkIterationResult> {
        let start_time = Instant::now();
        
        // Simulate optimization work
        std::thread::sleep(Duration::from_millis(10));
        
        Ok(BenchmarkIterationResult {
            duration: start_time.elapsed(),
            memory_usage: 100, // Simplified
            success: true,
        })
    }
    
    /// Compare with baseline
    pub fn compare_with_baseline(&self, current_metrics: &OptimizationResult) -> Option<BaselineComparison> {
        if let Some(baseline) = &self.baseline_metrics {
            let compile_time_change = (current_metrics.duration.as_secs_f64() - baseline.compile_time.as_secs_f64()) / baseline.compile_time.as_secs_f64();
            
            let memory_change = if let Some(current_memory) = current_metrics.metrics.get("memory_usage") {
                (*current_memory - baseline.memory_usage as f64) / baseline.memory_usage as f64
            } else {
                0.0
            };
            
            Some(BaselineComparison {
                compile_time_change,
                memory_change,
                is_regression: compile_time_change > 0.05 || memory_change > 0.1,
            })
        } else {
            None
        }
    }
    
    /// Record pass dependency timing
    pub fn record_pass_dependency(&mut self, pass_name: &str, dependencies: &[String], resolution_time: Duration) {
        let mut timing_data = self.timing_data.lock().unwrap();
        timing_data.record_pass_dependency(pass_name, dependencies, resolution_time);
    }
    
    /// Get pass dependency statistics
    pub fn get_pass_dependency_stats(&self) -> HashMap<String, PassDependencyStats> {
        let timing_data = self.timing_data.lock().unwrap();
        timing_data.pass_dependency_stats.clone()
    }
}

/// Timing token for measuring operation duration
pub struct TimingToken {
    operation: String,
    start_time: Instant,
    timing_data: Arc<Mutex<TimingData>>,
    enabled: bool,
}

impl TimingToken {
    fn new(operation: String, timing_data: Arc<Mutex<TimingData>>) -> Self {
        Self {
            operation,
            start_time: Instant::now(),
            timing_data,
            enabled: true,
        }
    }
    
    fn disabled() -> Self {
        Self {
            operation: String::new(),
            start_time: Instant::now(),
            timing_data: Arc::new(Mutex::new(TimingData::new())),
            enabled: false,
        }
    }
}

impl Drop for TimingToken {
    fn drop(&mut self) {
        if !self.enabled {
            return;
        }
        
        let duration = self.start_time.elapsed();
        
        if let Ok(mut timing_data) = self.timing_data.lock() {
            timing_data.record_operation(&self.operation, duration);
        }
    }
}

/// Internal timing data
#[derive(Debug)]
struct TimingData {
    operation_times: HashMap<String, Duration>,
    operation_counts: HashMap<String, u32>,
    total_time: Duration,
    cache_hit_rate: f64,
    successful_optimizations: u32,
    failed_optimizations: u32,
    peak_memory_usage: usize,
    regression_count: u32,
    pass_dependency_stats: HashMap<String, PassDependencyStats>,
}

impl TimingData {
    fn new() -> Self {
        Self {
            operation_times: HashMap::new(),
            operation_counts: HashMap::new(),
            total_time: Duration::default(),
            cache_hit_rate: 0.0,
            successful_optimizations: 0,
            failed_optimizations: 0,
            peak_memory_usage: 0,
            regression_count: 0,
            pass_dependency_stats: HashMap::new(),
        }
    }
    
    fn record_operation(&mut self, operation: &str, duration: Duration) {
        *self.operation_times.entry(operation.to_string()).or_insert(Duration::default()) += duration;
        *self.operation_counts.entry(operation.to_string()).or_insert(0) += 1;
        self.total_time += duration;
    }
    
    fn record_optimization_result(&mut self, result: &OptimizationResult) {
        self.record_operation(&result.optimization_type, result.duration);
        
        if result.success {
            self.successful_optimizations += 1;
        } else {
            self.failed_optimizations += 1;
        }
        
        if let Some(memory_usage) = result.metrics.get("memory_usage") {
            self.peak_memory_usage = self.peak_memory_usage.max(*memory_usage as usize);
        }
    }
    
    fn record_pass_dependency(&mut self, pass_name: &str, dependencies: &[String], resolution_time: Duration) {
        let stats = self.pass_dependency_stats.entry(pass_name.to_string()).or_insert_with(|| {
            PassDependencyStats {
                total_resolution_time: Duration::default(),
                resolution_count: 0,
                average_resolution_time: Duration::default(),
                dependency_count: dependencies.len(),
            }
        });
        
        stats.total_resolution_time += resolution_time;
        stats.resolution_count += 1;
        stats.average_resolution_time = stats.total_resolution_time / stats.resolution_count;
    }
    
    fn average_time(&self) -> Duration {
        let total_operations: u32 = self.operation_counts.values().sum();
        if total_operations > 0 {
            self.total_time / total_operations
        } else {
            Duration::default()
        }
    }
}

/// Performance summary
#[derive(Debug)]
pub struct PerformanceSummary {
    pub total_optimizations: u32,
    pub total_time: Duration,
    pub average_time: Duration,
    pub operation_breakdown: HashMap<String, Duration>,
    pub cache_hit_rate: f64,
    pub successful_optimizations: u32,
    pub failed_optimizations: u32,
    pub peak_memory_usage: usize,
    pub regression_count: u32,
}

/// Optimization result for recording
#[derive(Debug)]
pub struct OptimizationResult {
    pub optimization_type: String,
    pub duration: Duration,
    pub success: bool,
    pub metrics: HashMap<String, f64>,
}

impl OptimizationResult {
    pub fn new(optimization_type: String, duration: Duration, success: bool) -> Self {
        Self {
            optimization_type,
            duration,
            success,
            metrics: HashMap::new(),
        }
    }
    
    pub fn with_metric(mut self, name: String, value: f64) -> Self {
        self.metrics.insert(name, value);
        self
    }
}

/// Baseline metrics for comparison
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    pub compile_time: Duration,
    pub memory_usage: usize,
    pub binary_size: usize,
    pub performance_score: f64,
    pub timestamp: SystemTime,
}

/// Performance thresholds
#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub max_optimization_time: Duration,
    pub max_memory_usage: usize,
    pub max_regression_threshold: f64,
    pub min_cache_hit_rate: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_optimization_time: Duration::from_secs(300),
            max_memory_usage: 2048, // MB
            max_regression_threshold: 0.1, // 10%
            min_cache_hit_rate: 0.5, // 50%
        }
    }
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub timeout: Duration,
    pub operation_type: String,
}

/// Benchmark result
#[derive(Debug)]
pub struct BenchmarkResult {
    pub config: BenchmarkConfig,
    pub total_time: Duration,
    pub avg_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub avg_memory: usize,
    pub peak_memory: usize,
    pub iterations_run: usize,
    pub success_rate: f64,
}

/// Single benchmark iteration result
#[derive(Debug)]
struct BenchmarkIterationResult {
    pub duration: Duration,
    pub memory_usage: usize,
    pub success: bool,
}

/// Baseline comparison result
#[derive(Debug)]
pub struct BaselineComparison {
    pub compile_time_change: f64,
    pub memory_change: f64,
    pub is_regression: bool,
}

/// Pass dependency statistics
#[derive(Debug, Clone)]
pub struct PassDependencyStats {
    pub total_resolution_time: Duration,
    pub resolution_count: u32,
    pub average_resolution_time: Duration,
    pub dependency_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_enhanced_performance_monitor_creation() {
        let config = OptimizationConfig::debug();
        let monitor = EnhancedPerformanceMonitor::new(config);
        assert!(monitor.is_ok());
    }
    
    #[test]
    fn test_timing_token() {
        let config = OptimizationConfig::debug();
        let monitor = EnhancedPerformanceMonitor::new(config).unwrap();
        
        {
            let _token = monitor.start_timing("test_operation");
            thread::sleep(Duration::from_millis(10));
        }
        
        let summary = monitor.get_performance_summary();
        assert!(summary.operation_breakdown.contains_key("test_operation"));
    }
    
    #[test]
    fn test_performance_summary() {
        let config = OptimizationConfig::debug();
        let monitor = EnhancedPerformanceMonitor::new(config).unwrap();
        
        {
            let _token = monitor.start_timing("optimization1");
            thread::sleep(Duration::from_millis(5));
        }
        
        {
            let _token = monitor.start_timing("optimization2");
            thread::sleep(Duration::from_millis(5));
        }
        
        let summary = monitor.get_performance_summary();
        assert_eq!(summary.total_optimizations, 2);
        assert!(summary.total_time > Duration::from_millis(8));
    }
    
    #[test]
    fn test_disabled_monitoring() {
        let config = OptimizationConfig::debug();
        let mut monitor = EnhancedPerformanceMonitor::new(config).unwrap();
        monitor.set_enabled(false);
        
        {
            let _token = monitor.start_timing("disabled_operation");
            thread::sleep(Duration::from_millis(10));
        }
        
        let summary = monitor.get_performance_summary();
        assert_eq!(summary.total_optimizations, 0);
    }
    
    #[test]
    fn test_report_generation() {
        let config = OptimizationConfig::debug();
        let monitor = EnhancedPerformanceMonitor::new(config).unwrap();
        
        {
            let _token = monitor.start_timing("report_test");
            thread::sleep(Duration::from_millis(1));
        }
        
        let report = monitor.generate_report();
        assert!(report.is_ok());
        
        let report_text = report.unwrap();
        assert!(report_text.contains("Performance Report"));
        assert!(report_text.contains("report_test"));
    }
    
    #[test]
    fn test_benchmark_config() {
        let config = BenchmarkConfig {
            iterations: 100,
            warmup_iterations: 10,
            timeout: Duration::from_secs(60),
            operation_type: "test".to_string(),
        };
        
        assert_eq!(config.iterations, 100);
        assert_eq!(config.warmup_iterations, 10);
    }
    
    #[test]
    fn test_baseline_comparison() {
        let baseline = BaselineMetrics {
            compile_time: Duration::from_secs(10),
            memory_usage: 512,
            binary_size: 1024,
            performance_score: 1.0,
            timestamp: SystemTime::now(),
        };
        
        let config = OptimizationConfig::debug();
        let mut monitor = EnhancedPerformanceMonitor::new(config).unwrap();
        monitor.set_baseline_metrics(baseline);
        
        let current = OptimizationResult::new(
            "test".to_string(),
            Duration::from_secs(12),
            true
        ).with_metric("memory_usage".to_string(), 600.0);
        
        let comparison = monitor.compare_with_baseline(&current);
        assert!(comparison.is_some());
        
        let comp = comparison.unwrap();
        assert!(comp.compile_time_change > 0.0);
        assert!(comp.memory_change > 0.0);
    }
    
    #[test]
    fn test_benchmarking() {
        let config = OptimizationConfig::debug();
        let mut monitor = EnhancedPerformanceMonitor::new(config).unwrap();
        monitor.set_benchmarking_enabled(true);
        
        let benchmark_config = BenchmarkConfig {
            iterations: 5,
            warmup_iterations: 1,
            timeout: Duration::from_secs(10),
            operation_type: "test_benchmark".to_string(),
        };
        
        let result = monitor.run_benchmark(benchmark_config);
        assert!(result.is_ok());
        
        let benchmark_result = result.unwrap();
        assert_eq!(benchmark_result.iterations_run, 4); // 5 total - 1 warmup
        assert!(benchmark_result.success_rate > 0.0);
    }
    
    #[test]
    fn test_pass_dependency_tracking() {
        let config = OptimizationConfig::debug();
        let mut monitor = EnhancedPerformanceMonitor::new(config).unwrap();
        
        let dependencies = vec!["mem2reg".to_string(), "simplifycfg".to_string()];
        monitor.record_pass_dependency("gvn", &dependencies, Duration::from_millis(50));
        
        let stats = monitor.get_pass_dependency_stats();
        assert!(stats.contains_key("gvn"));
        
        let gvn_stats = stats.get("gvn").unwrap();
        assert_eq!(gvn_stats.dependency_count, 2);
        assert_eq!(gvn_stats.resolution_count, 1);
    }
}
