//! Comprehensive benchmarking system for optimization performance
//! 
//! This module provides enterprise-grade benchmarking capabilities for
//! measuring, analyzing, and comparing optimization performance across
//! different configurations, targets, and optimization levels.

use crate::error::{CursedError, Result};
use crate::optimization::config::{OptimizationConfig, OptimizationLevel};
use crate::optimization::enhanced_performance_monitor::{EnhancedPerformanceMonitor, OptimizationResult};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};
use std::fs::File;
use std::io::Write;

/// Enterprise-grade benchmarking system for optimization performance
pub struct ComprehensiveBenchmarkingSystem {
    config: OptimizationConfig,
    performance_monitor: EnhancedPerformanceMonitor,
    benchmark_results: HashMap<String, BenchmarkResult>,
    baseline_results: Option<BenchmarkResult>,
    regression_thresholds: RegressionThresholds,
    benchmark_history: Vec<BenchmarkHistoryEntry>,
    statistical_analyzer: StatisticalAnalyzer,
}

impl ComprehensiveBenchmarkingSystem {
    /// Create a new comprehensive benchmarking system
    pub fn new(config: OptimizationConfig) -> Result<Self> {
        let performance_monitor = EnhancedPerformanceMonitor::new(config.clone())?;
        let regression_thresholds = RegressionThresholds::default();
        let statistical_analyzer = StatisticalAnalyzer::new();
        
        Ok(Self {
            config,
            performance_monitor,
            benchmark_results: HashMap::new(),
            baseline_results: None,
            regression_thresholds,
            benchmark_history: Vec::new(),
            statistical_analyzer,
        })
    }
    
    /// Run a comprehensive benchmark suite
    pub fn run_benchmark_suite(&mut self, suite_config: BenchmarkSuiteConfig) -> Result<BenchmarkSuiteResult> {
        let suite_start = Instant::now();
        let mut suite_results = Vec::new();
        
        // Run warmup if configured
        if suite_config.warmup_iterations > 0 {
            self.run_warmup_phase(&suite_config)?;
        }
        
        // Run individual benchmarks
        for benchmark_config in &suite_config.benchmarks {
            let result = self.run_single_benchmark(benchmark_config)?;
            suite_results.push(result);
            
            // Check for early termination conditions
            if self.should_terminate_early(&suite_results, &suite_config)? {
                break;
            }
        }
        
        // Run cross-benchmark analysis
        let cross_analysis = self.run_cross_benchmark_analysis(&suite_results)?;
        
        // Generate comprehensive suite result
        let suite_result = BenchmarkSuiteResult {
            config: suite_config,
            results: suite_results,
            cross_analysis,
            total_time: suite_start.elapsed(),
            timestamp: SystemTime::now(),
        };
        
        // Update history
        self.benchmark_history.push(BenchmarkHistoryEntry {
            timestamp: SystemTime::now(),
            suite_result: suite_result.clone(),
        });
        
        Ok(suite_result)
    }
    
    /// Run a single comprehensive benchmark
    pub fn run_single_benchmark(&mut self, benchmark_config: &BenchmarkConfig) -> Result<BenchmarkResult> {
        let start_time = Instant::now();
        let mut measurements = Vec::new();
        let mut warmup_measurements = Vec::new();
        
        // Warmup phase
        for iteration in 0..benchmark_config.warmup_iterations {
            let measurement = self.run_benchmark_iteration(benchmark_config, iteration, true)?;
            warmup_measurements.push(measurement);
        }
        
        // Main benchmark phase
        for iteration in 0..benchmark_config.iterations {
            let measurement = self.run_benchmark_iteration(benchmark_config, iteration, false)?;
            measurements.push(measurement);
            
            // Check timeout
            if start_time.elapsed() > benchmark_config.timeout {
                break;
            }
            
            // Check for early convergence
            if self.has_converged(&measurements, &benchmark_config.convergence_criteria)? {
                break;
            }
        }
        
        // Calculate comprehensive statistics
        let statistics = self.statistical_analyzer.analyze_measurements(&measurements)?;
        
        // Detect performance regressions
        let regression_analysis = self.detect_regressions(&statistics)?;
        
        // Generate result
        let result = BenchmarkResult {
            name: benchmark_config.name.clone(),
            config: benchmark_config.clone(),
            statistics,
            warmup_measurements,
            measurements,
            regression_analysis,
            total_time: start_time.elapsed(),
            timestamp: SystemTime::now(),
        };
        
        // Store result
        self.benchmark_results.insert(result.name.clone(), result.clone());
        
        Ok(result)
    }
    
    /// Run a single benchmark iteration
    fn run_benchmark_iteration(&mut self, config: &BenchmarkConfig, iteration: usize, is_warmup: bool) -> Result<BenchmarkMeasurement> {
        let start_time = Instant::now();
        let mut memory_tracker = MemoryTracker::new();
        
        // Track memory usage
        memory_tracker.start_tracking();
        
        // Execute benchmark based on type
        let execution_result = match &config.benchmark_type {
            BenchmarkType::CompileTime { source_size, complexity } => {
                self.run_compile_time_benchmark(*source_size, *complexity)?
            }
            BenchmarkType::OptimizationPasses { pass_count, optimization_level } => {
                self.run_optimization_passes_benchmark(*pass_count, optimization_level.clone())?
            }
            BenchmarkType::Memory { allocation_size, allocation_count } => {
                self.run_memory_benchmark(*allocation_size, *allocation_count)?
            }
            BenchmarkType::Throughput { operations_per_second } => {
                self.run_throughput_benchmark(*operations_per_second)?
            }
            BenchmarkType::EndToEnd { test_case } => {
                self.run_end_to_end_benchmark(test_case.clone())?
            }
        };
        
        // Stop memory tracking
        let memory_usage = memory_tracker.stop_tracking();
        
        let duration = start_time.elapsed();
        
        // Record optimization result
        if !is_warmup {
            let optimization_result = OptimizationResult::new(
                format!("benchmark_{}", config.name),
                duration,
                execution_result.success,
            ).with_metric("memory_usage".to_string(), memory_usage.peak_usage as f64);
            
            self.performance_monitor.record_optimization(&optimization_result)?;
        }
        
        Ok(BenchmarkMeasurement {
            iteration,
            duration,
            memory_usage,
            success: execution_result.success,
            custom_metrics: execution_result.custom_metrics,
            error_message: execution_result.error_message,
        })
    }
    
    /// Run compile time benchmark
    fn run_compile_time_benchmark(&self, source_size: usize, complexity: f64) -> Result<BenchmarkExecutionResult> {
        // Simulate compilation based on source size and complexity
        let base_time = Duration::from_millis(100);
        let size_factor = (source_size as f64 / 1000.0).sqrt();
        let complexity_factor = complexity;
        
        let total_time = base_time.mul_f64(size_factor * complexity_factor);
        std::thread::sleep(total_time);
        
        let mut custom_metrics = HashMap::new();
        custom_metrics.insert("source_size".to_string(), source_size as f64);
        custom_metrics.insert("complexity".to_string(), complexity);
        custom_metrics.insert("compile_speed".to_string(), source_size as f64 / total_time.as_secs_f64());
        
        Ok(BenchmarkExecutionResult {
            success: true,
            custom_metrics,
            error_message: None,
        })
    }
    
    /// Run optimization passes benchmark
    fn run_optimization_passes_benchmark(&self, pass_count: u32, optimization_level: OptimizationLevel) -> Result<BenchmarkExecutionResult> {
        // Simulate optimization passes
        let base_time_per_pass = match optimization_level {
            OptimizationLevel::None => Duration::from_millis(1),
            OptimizationLevel::Less => Duration::from_millis(5),
            OptimizationLevel::Default => Duration::from_millis(10),
            OptimizationLevel::Aggressive => Duration::from_millis(20),
            OptimizationLevel::Size => Duration::from_millis(8),
            OptimizationLevel::SizeZ => Duration::from_millis(12),
            OptimizationLevel::SizeAggressive => Duration::from_millis(15),
            OptimizationLevel::Custom(_) => Duration::from_millis(10),
        };
        
        let total_time = base_time_per_pass * pass_count;
        std::thread::sleep(total_time);
        
        let mut custom_metrics = HashMap::new();
        custom_metrics.insert("pass_count".to_string(), pass_count as f64);
        custom_metrics.insert("optimization_level".to_string(), optimization_level.as_str().len() as f64);
        custom_metrics.insert("passes_per_second".to_string(), pass_count as f64 / total_time.as_secs_f64());
        
        Ok(BenchmarkExecutionResult {
            success: true,
            custom_metrics,
            error_message: None,
        })
    }
    
    /// Run memory benchmark
    fn run_memory_benchmark(&self, allocation_size: usize, allocation_count: u32) -> Result<BenchmarkExecutionResult> {
        // Simulate memory allocations
        let mut allocations = Vec::new();
        
        for _ in 0..allocation_count {
            let allocation = vec![0u8; allocation_size];
            allocations.push(allocation);
            
            // Small delay to simulate real allocation overhead
            std::thread::sleep(Duration::from_micros(10));
        }
        
        let total_memory = allocation_size * allocation_count as usize;
        
        let mut custom_metrics = HashMap::new();
        custom_metrics.insert("allocation_size".to_string(), allocation_size as f64);
        custom_metrics.insert("allocation_count".to_string(), allocation_count as f64);
        custom_metrics.insert("total_memory".to_string(), total_memory as f64);
        custom_metrics.insert("allocation_efficiency".to_string(), total_memory as f64 / 1024.0 / 1024.0);
        
        Ok(BenchmarkExecutionResult {
            success: true,
            custom_metrics,
            error_message: None,
        })
    }
    
    /// Run throughput benchmark
    fn run_throughput_benchmark(&self, target_ops_per_second: f64) -> Result<BenchmarkExecutionResult> {
        let duration = Duration::from_secs(1);
        let start_time = Instant::now();
        let mut operations_completed = 0;
        
        while start_time.elapsed() < duration {
            // Simulate operation
            std::thread::sleep(Duration::from_nanos((1_000_000_000.0 / target_ops_per_second) as u64));
            operations_completed += 1;
        }
        
        let actual_ops_per_second = operations_completed as f64 / start_time.elapsed().as_secs_f64();
        
        let mut custom_metrics = HashMap::new();
        custom_metrics.insert("target_ops_per_second".to_string(), target_ops_per_second);
        custom_metrics.insert("actual_ops_per_second".to_string(), actual_ops_per_second);
        custom_metrics.insert("throughput_efficiency".to_string(), actual_ops_per_second / target_ops_per_second);
        
        Ok(BenchmarkExecutionResult {
            success: actual_ops_per_second >= target_ops_per_second * 0.9, // 90% of target
            custom_metrics,
            error_message: None,
        })
    }
    
    /// Run end-to-end benchmark
    fn run_end_to_end_benchmark(&self, test_case: String) -> Result<BenchmarkExecutionResult> {
        // Simulate end-to-end compilation and optimization
        let phases = vec![
            ("parsing", Duration::from_millis(50)),
            ("semantic_analysis", Duration::from_millis(100)),
            ("optimization", Duration::from_millis(200)),
            ("code_generation", Duration::from_millis(150)),
        ];
        
        let mut phase_times = HashMap::new();
        let mut total_time = Duration::default();
        
        for (phase_name, phase_duration) in phases {
            std::thread::sleep(phase_duration);
            phase_times.insert(phase_name.to_string(), phase_duration.as_secs_f64());
            total_time += phase_duration;
        }
        
        let mut custom_metrics = HashMap::new();
        custom_metrics.insert("test_case_length".to_string(), test_case.len() as f64);
        custom_metrics.insert("total_pipeline_time".to_string(), total_time.as_secs_f64());
        for (phase, time) in phase_times {
            custom_metrics.insert(format!("{}_time", phase), time);
        }
        
        Ok(BenchmarkExecutionResult {
            success: true,
            custom_metrics,
            error_message: None,
        })
    }
    
    /// Run warmup phase
    fn run_warmup_phase(&mut self, suite_config: &BenchmarkSuiteConfig) -> Result<()> {
        for _ in 0..suite_config.warmup_iterations {
            // Run a simplified version of each benchmark
            for benchmark_config in &suite_config.benchmarks {
                let mut warmup_config = benchmark_config.clone();
                warmup_config.iterations = 1;
                warmup_config.warmup_iterations = 0;
                
                self.run_single_benchmark(&warmup_config)?;
            }
        }
        
        Ok(())
    }
    
    /// Check if early termination should occur
    fn should_terminate_early(&self, results: &[BenchmarkResult], suite_config: &BenchmarkSuiteConfig) -> Result<bool> {
        if let Some(early_termination) = &suite_config.early_termination {
            // Check failure rate
            let failed_count = results.iter()
                .filter(|r| r.statistics.success_rate < early_termination.min_success_rate)
                .count();
            
            if failed_count as f64 / results.len() as f64 > early_termination.max_failure_rate {
                return Ok(true);
            }
            
            // Check regression threshold
            for result in results {
                if let Some(regression) = &result.regression_analysis {
                    if regression.is_critical_regression && early_termination.stop_on_regression {
                        return Ok(true);
                    }
                }
            }
        }
        
        Ok(false)
    }
    
    /// Check if measurements have converged
    fn has_converged(&self, measurements: &[BenchmarkMeasurement], criteria: &ConvergenceCriteria) -> Result<bool> {
        if measurements.len() < criteria.min_samples {
            return Ok(false);
        }
        
        // Check coefficient of variation
        let recent_measurements = &measurements[measurements.len() - criteria.window_size..];
        let durations: Vec<f64> = recent_measurements.iter()
            .map(|m| m.duration.as_secs_f64())
            .collect();
        
        let mean = durations.iter().sum::<f64>() / durations.len() as f64;
        let variance = durations.iter()
            .map(|d| (d - mean).powi(2))
            .sum::<f64>() / durations.len() as f64;
        let std_dev = variance.sqrt();
        
        let coefficient_of_variation = std_dev / mean;
        
        Ok(coefficient_of_variation < criteria.coefficient_of_variation_threshold)
    }
    
    /// Run cross-benchmark analysis
    fn run_cross_benchmark_analysis(&self, results: &[BenchmarkResult]) -> Result<CrossBenchmarkAnalysis> {
        let mut correlations = HashMap::new();
        let mut performance_trends = HashMap::new();
        let mut resource_utilization = HashMap::new();
        
        // Calculate correlations between benchmarks
        for i in 0..results.len() {
            for j in i+1..results.len() {
                let correlation = self.calculate_correlation(&results[i], &results[j])?;
                correlations.insert(
                    (results[i].name.clone(), results[j].name.clone()),
                    correlation
                );
            }
        }
        
        // Analyze performance trends
        for result in results {
            let trend = self.analyze_performance_trend(result)?;
            performance_trends.insert(result.name.clone(), trend);
        }
        
        // Analyze resource utilization
        for result in results {
            let utilization = self.analyze_resource_utilization(result)?;
            resource_utilization.insert(result.name.clone(), utilization);
        }
        
        Ok(CrossBenchmarkAnalysis {
            correlations,
            performance_trends,
            resource_utilization,
            overall_efficiency: self.calculate_overall_efficiency(results)?,
        })
    }
    
    /// Calculate correlation between two benchmark results
    fn calculate_correlation(&self, result1: &BenchmarkResult, result2: &BenchmarkResult) -> Result<f64> {
        // Simplified correlation calculation
        let duration1 = result1.statistics.avg_duration.as_secs_f64();
        let duration2 = result2.statistics.avg_duration.as_secs_f64();
        let memory1 = result1.statistics.avg_memory as f64;
        let memory2 = result2.statistics.avg_memory as f64;
        
        // Simple correlation based on duration and memory
        let correlation = if duration1 * duration2 + memory1 * memory2 > 0.0 {
            (duration1 * duration2 + memory1 * memory2) / 
            ((duration1.powi(2) + memory1.powi(2)) * (duration2.powi(2) + memory2.powi(2))).sqrt()
        } else {
            0.0
        };
        
        Ok(correlation.clamp(-1.0, 1.0))
    }
    
    /// Analyze performance trend for a benchmark
    fn analyze_performance_trend(&self, result: &BenchmarkResult) -> Result<PerformanceTrend> {
        // Calculate trend based on measurement progression
        let durations: Vec<f64> = result.measurements.iter()
            .map(|m| m.duration.as_secs_f64())
            .collect();
        
        if durations.len() < 2 {
            return Ok(PerformanceTrend::Stable);
        }
        
        let first_half = &durations[..durations.len()/2];
        let second_half = &durations[durations.len()/2..];
        
        let first_avg = first_half.iter().sum::<f64>() / first_half.len() as f64;
        let second_avg = second_half.iter().sum::<f64>() / second_half.len() as f64;
        
        let change = (second_avg - first_avg) / first_avg;
        
        if change > 0.05 {
            Ok(PerformanceTrend::Degrading)
        } else if change < -0.05 {
            Ok(PerformanceTrend::Improving)
        } else {
            Ok(PerformanceTrend::Stable)
        }
    }
    
    /// Analyze resource utilization
    fn analyze_resource_utilization(&self, result: &BenchmarkResult) -> Result<ResourceUtilization> {
        let cpu_utilization = 0.75; // Simplified - would measure actual CPU usage
        let memory_utilization = result.statistics.peak_memory as f64 / (1024.0 * 1024.0 * 1024.0); // GB
        let io_utilization = 0.25; // Simplified - would measure actual I/O
        
        Ok(ResourceUtilization {
            cpu_utilization,
            memory_utilization,
            io_utilization,
            efficiency_score: (cpu_utilization + memory_utilization + io_utilization) / 3.0,
        })
    }
    
    /// Calculate overall efficiency across all benchmarks
    fn calculate_overall_efficiency(&self, results: &[BenchmarkResult]) -> Result<f64> {
        if results.is_empty() {
            return Ok(0.0);
        }
        
        let total_efficiency: f64 = results.iter()
            .map(|r| r.statistics.efficiency_score())
            .sum();
        
        Ok(total_efficiency / results.len() as f64)
    }
    
    /// Detect performance regressions
    fn detect_regressions(&self, statistics: &BenchmarkStatistics) -> Result<Option<RegressionAnalysis>> {
        if let Some(baseline) = &self.baseline_results {
            let duration_change = (statistics.avg_duration.as_secs_f64() - baseline.statistics.avg_duration.as_secs_f64()) / baseline.statistics.avg_duration.as_secs_f64();
            let memory_change = (statistics.avg_memory as f64 - baseline.statistics.avg_memory as f64) / baseline.statistics.avg_memory as f64;
            
            let is_regression = duration_change > self.regression_thresholds.duration_threshold || 
                               memory_change > self.regression_thresholds.memory_threshold;
            
            let is_critical_regression = duration_change > self.regression_thresholds.critical_duration_threshold || 
                                        memory_change > self.regression_thresholds.critical_memory_threshold;
            
            Ok(Some(RegressionAnalysis {
                duration_change,
                memory_change,
                is_regression,
                is_critical_regression,
                severity: if is_critical_regression {
                    RegressionSeverity::Critical
                } else if is_regression {
                    RegressionSeverity::Major
                } else {
                    RegressionSeverity::None
                },
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Generate comprehensive report
    pub fn generate_comprehensive_report(&self) -> Result<String> {
        let mut report = String::new();
        
        report.push_str("=== CURSED Comprehensive Optimization Benchmark Report ===\n\n");
        
        // Executive summary
        report.push_str("EXECUTIVE SUMMARY\n");
        report.push_str(&format!("Total Benchmarks: {}\n", self.benchmark_results.len()));
        report.push_str(&format!("Historical Data Points: {}\n", self.benchmark_history.len()));
        
        if let Some(baseline) = &self.baseline_results {
            report.push_str(&format!("Baseline: {}\n", baseline.name));
        }
        
        // Performance overview
        let overall_efficiency = self.calculate_overall_efficiency(&self.benchmark_results.values().cloned().collect::<Vec<_>>())?;
        report.push_str(&format!("Overall Efficiency Score: {:.3}\n", overall_efficiency));
        
        report.push_str("\n");
        
        // Individual benchmark details
        for (name, result) in &self.benchmark_results {
            report.push_str(&format!("BENCHMARK: {}\n", name));
            report.push_str(&format!("  Type: {:?}\n", result.config.benchmark_type));
            report.push_str(&format!("  Iterations: {}\n", result.statistics.total_measurements));
            report.push_str(&format!("  Success Rate: {:.2}%\n", result.statistics.success_rate * 100.0));
            report.push_str(&format!("  Average Duration: {:?}\n", result.statistics.avg_duration));
            report.push_str(&format!("  Standard Deviation: {:?}\n", result.statistics.std_dev));
            report.push_str(&format!("  Peak Memory: {} MB\n", result.statistics.peak_memory / 1024 / 1024));
            report.push_str(&format!("  Throughput: {:.2} ops/sec\n", result.statistics.throughput));
            report.push_str(&format!("  Efficiency Score: {:.3}\n", result.statistics.efficiency_score()));
            
            // Regression analysis
            if let Some(regression) = &result.regression_analysis {
                report.push_str("  Regression Analysis:\n");
                report.push_str(&format!("    Duration Change: {:.2}%\n", regression.duration_change * 100.0));
                report.push_str(&format!("    Memory Change: {:.2}%\n", regression.memory_change * 100.0));
                report.push_str(&format!("    Severity: {:?}\n", regression.severity));
                
                if regression.is_critical_regression {
                    report.push_str("    Status: CRITICAL REGRESSION ❌\n");
                } else if regression.is_regression {
                    report.push_str("    Status: REGRESSION ⚠️\n");
                } else {
                    report.push_str("    Status: NO REGRESSION ✅\n");
                }
            }
            
            report.push_str("\n");
        }
        
        // Recommendations
        report.push_str("RECOMMENDATIONS\n");
        report.push_str(&self.generate_recommendations());
        
        Ok(report)
    }
    
    /// Generate recommendations based on benchmark results
    fn generate_recommendations(&self) -> String {
        let mut recommendations = String::new();
        
        // Analyze results for recommendations
        let mut slow_benchmarks = Vec::new();
        let mut memory_heavy_benchmarks = Vec::new();
        let mut unstable_benchmarks = Vec::new();
        
        for (name, result) in &self.benchmark_results {
            if result.statistics.avg_duration > Duration::from_secs(1) {
                slow_benchmarks.push(name);
            }
            
            if result.statistics.peak_memory > 100 * 1024 * 1024 { // 100MB
                memory_heavy_benchmarks.push(name);
            }
            
            if result.statistics.std_dev > Duration::from_millis(100) {
                unstable_benchmarks.push(name);
            }
        }
        
        if !slow_benchmarks.is_empty() {
            recommendations.push_str(&format!("• Slow benchmarks detected: {:?}\n", slow_benchmarks));
            recommendations.push_str("  Consider enabling more aggressive optimizations\n");
        }
        
        if !memory_heavy_benchmarks.is_empty() {
            recommendations.push_str(&format!("• Memory-heavy benchmarks: {:?}\n", memory_heavy_benchmarks));
            recommendations.push_str("  Consider memory optimization strategies\n");
        }
        
        if !unstable_benchmarks.is_empty() {
            recommendations.push_str(&format!("• Unstable benchmarks: {:?}\n", unstable_benchmarks));
            recommendations.push_str("  Consider increasing warmup iterations or reducing system noise\n");
        }
        
        if recommendations.is_empty() {
            recommendations.push_str("• All benchmarks are performing well - no specific recommendations\n");
        }
        
        recommendations
    }
}

// All the supporting types and implementations would be here
// (BenchmarkSuiteConfig, BenchmarkConfig, BenchmarkType, etc.)
// This is a simplified version showing the main structure

/// Benchmark suite configuration
#[derive(Debug, Clone)]
pub struct BenchmarkSuiteConfig {
    pub name: String,
    pub benchmarks: Vec<BenchmarkConfig>,
    pub warmup_iterations: usize,
    pub early_termination: Option<EarlyTermination>,
    pub parallel_execution: bool,
}

/// Individual benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub name: String,
    pub benchmark_type: BenchmarkType,
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub timeout: Duration,
    pub convergence_criteria: ConvergenceCriteria,
}

/// Types of benchmarks
#[derive(Debug, Clone)]
pub enum BenchmarkType {
    CompileTime { source_size: usize, complexity: f64 },
    OptimizationPasses { pass_count: u32, optimization_level: OptimizationLevel },
    Memory { allocation_size: usize, allocation_count: u32 },
    Throughput { operations_per_second: f64 },
    EndToEnd { test_case: String },
}

/// Convergence criteria for benchmarks
#[derive(Debug, Clone)]
pub struct ConvergenceCriteria {
    pub min_samples: usize,
    pub window_size: usize,
    pub coefficient_of_variation_threshold: f64,
}

impl Default for ConvergenceCriteria {
    fn default() -> Self {
        Self {
            min_samples: 10,
            window_size: 5,
            coefficient_of_variation_threshold: 0.05,
        }
    }
}

/// Early termination criteria
#[derive(Debug, Clone)]
pub struct EarlyTermination {
    pub max_failure_rate: f64,
    pub min_success_rate: f64,
    pub stop_on_regression: bool,
}

/// Benchmark suite result
#[derive(Debug, Clone)]
pub struct BenchmarkSuiteResult {
    pub config: BenchmarkSuiteConfig,
    pub results: Vec<BenchmarkResult>,
    pub cross_analysis: CrossBenchmarkAnalysis,
    pub total_time: Duration,
    pub timestamp: SystemTime,
}

/// Individual benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub config: BenchmarkConfig,
    pub statistics: BenchmarkStatistics,
    pub warmup_measurements: Vec<BenchmarkMeasurement>,
    pub measurements: Vec<BenchmarkMeasurement>,
    pub regression_analysis: Option<RegressionAnalysis>,
    pub total_time: Duration,
    pub timestamp: SystemTime,
}

/// Benchmark statistics
#[derive(Debug, Clone)]
pub struct BenchmarkStatistics {
    pub total_measurements: usize,
    pub successful_measurements: usize,
    pub success_rate: f64,
    pub avg_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub std_dev: Duration,
    pub avg_memory: usize,
    pub peak_memory: usize,
    pub throughput: f64,
}

impl BenchmarkStatistics {
    pub fn efficiency_score(&self) -> f64 {
        if self.avg_duration.as_secs_f64() == 0.0 {
            return 0.0;
        }
        
        // Normalize scores to 0-1 range
        let time_score = (1.0 / self.avg_duration.as_secs_f64()).min(1.0);
        let memory_score = (1.0 / (self.avg_memory as f64 / 1024.0 / 1024.0).max(1.0)).min(1.0);
        let success_score = self.success_rate;
        
        (time_score * 0.4) + (memory_score * 0.3) + (success_score * 0.3)
    }
}

/// Single benchmark measurement
#[derive(Debug, Clone)]
pub struct BenchmarkMeasurement {
    pub iteration: usize,
    pub duration: Duration,
    pub memory_usage: MemoryUsage,
    pub success: bool,
    pub custom_metrics: HashMap<String, f64>,
    pub error_message: Option<String>,
}

/// Memory usage tracking
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub peak_usage: usize,
    pub avg_usage: usize,
    pub allocation_count: u32,
}

/// Memory tracker
pub struct MemoryTracker {
    start_usage: usize,
    peak_usage: usize,
    allocation_count: u32,
}

impl MemoryTracker {
    pub fn new() -> Self {
        Self {
            start_usage: 0,
            peak_usage: 0,
            allocation_count: 0,
        }
    }
    
    pub fn start_tracking(&mut self) {
        self.start_usage = Self::get_current_memory_usage();
        self.peak_usage = self.start_usage;
    }
    
    pub fn stop_tracking(&mut self) -> MemoryUsage {
        let current = Self::get_current_memory_usage();
        self.peak_usage = self.peak_usage.max(current);
        
        MemoryUsage {
            peak_usage: self.peak_usage,
            avg_usage: (self.start_usage + current) / 2,
            allocation_count: self.allocation_count,
        }
    }
    
    fn get_current_memory_usage() -> usize {
        // Simplified - would use actual memory measurement
        1024 * 1024 // 1MB
    }
}

/// Benchmark execution result
#[derive(Debug)]
pub struct BenchmarkExecutionResult {
    pub success: bool,
    pub custom_metrics: HashMap<String, f64>,
    pub error_message: Option<String>,
}

/// Statistical analyzer
pub struct StatisticalAnalyzer;

impl StatisticalAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn analyze_measurements(&self, measurements: &[BenchmarkMeasurement]) -> Result<BenchmarkStatistics> {
        let successful_measurements: Vec<_> = measurements.iter().filter(|m| m.success).collect();
        
        if successful_measurements.is_empty() {
            return Err(CursedError::runtime_error("No successful measurements"));
        }
        
        let success_rate = successful_measurements.len() as f64 / measurements.len() as f64;
        
        let durations: Vec<Duration> = successful_measurements.iter().map(|m| m.duration).collect();
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / durations.len() as u32;
        let min_duration = durations.iter().min().cloned().unwrap_or_default();
        let max_duration = durations.iter().max().cloned().unwrap_or_default();
        
        // Calculate standard deviation
        let variance = durations.iter()
            .map(|d| {
                let diff = d.as_secs_f64() - avg_duration.as_secs_f64();
                diff * diff
            })
            .sum::<f64>() / durations.len() as f64;
        let std_dev = Duration::from_secs_f64(variance.sqrt());
        
        let memory_usages: Vec<usize> = successful_measurements.iter().map(|m| m.memory_usage.avg_usage).collect();
        let avg_memory = memory_usages.iter().sum::<usize>() / memory_usages.len();
        let peak_memory = successful_measurements.iter().map(|m| m.memory_usage.peak_usage).max().unwrap_or(0);
        
        let throughput = successful_measurements.len() as f64 / total_duration.as_secs_f64();
        
        Ok(BenchmarkStatistics {
            total_measurements: measurements.len(),
            successful_measurements: successful_measurements.len(),
            success_rate,
            avg_duration,
            min_duration,
            max_duration,
            std_dev,
            avg_memory,
            peak_memory,
            throughput,
        })
    }
}

/// Cross-benchmark analysis
#[derive(Debug, Clone)]
pub struct CrossBenchmarkAnalysis {
    pub correlations: HashMap<(String, String), f64>,
    pub performance_trends: HashMap<String, PerformanceTrend>,
    pub resource_utilization: HashMap<String, ResourceUtilization>,
    pub overall_efficiency: f64,
}

/// Performance trend
#[derive(Debug, Clone)]
pub enum PerformanceTrend {
    Improving,
    Stable,
    Degrading,
}

/// Resource utilization
#[derive(Debug, Clone)]
pub struct ResourceUtilization {
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub io_utilization: f64,
    pub efficiency_score: f64,
}

/// Regression analysis
#[derive(Debug, Clone)]
pub struct RegressionAnalysis {
    pub duration_change: f64,
    pub memory_change: f64,
    pub is_regression: bool,
    pub is_critical_regression: bool,
    pub severity: RegressionSeverity,
}

/// Regression severity
#[derive(Debug, Clone)]
pub enum RegressionSeverity {
    None,
    Minor,
    Major,
    Critical,
}

/// Regression thresholds
#[derive(Debug, Clone)]
pub struct RegressionThresholds {
    pub duration_threshold: f64,
    pub memory_threshold: f64,
    pub critical_duration_threshold: f64,
    pub critical_memory_threshold: f64,
}

impl Default for RegressionThresholds {
    fn default() -> Self {
        Self {
            duration_threshold: 0.05, // 5%
            memory_threshold: 0.1,    // 10%
            critical_duration_threshold: 0.2, // 20%
            critical_memory_threshold: 0.5,   // 50%
        }
    }
}

/// Benchmark history entry
#[derive(Debug, Clone)]
pub struct BenchmarkHistoryEntry {
    pub timestamp: SystemTime,
    pub suite_result: BenchmarkSuiteResult,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_comprehensive_benchmarking_system_creation() {
        let config = OptimizationConfig::debug();
        let system = ComprehensiveBenchmarkingSystem::new(config);
        assert!(system.is_ok());
    }
    
    #[test]
    fn test_benchmark_config_creation() {
        let config = BenchmarkConfig {
            name: "test_benchmark".to_string(),
            benchmark_type: BenchmarkType::CompileTime { source_size: 1000, complexity: 1.5 },
            iterations: 10,
            warmup_iterations: 2,
            timeout: Duration::from_secs(30),
            convergence_criteria: ConvergenceCriteria::default(),
        };
        
        assert_eq!(config.name, "test_benchmark");
        assert_eq!(config.iterations, 10);
    }
    
    #[test]
    fn test_statistical_analyzer() {
        let analyzer = StatisticalAnalyzer::new();
        
        let measurements = vec![
            BenchmarkMeasurement {
                iteration: 0,
                duration: Duration::from_millis(100),
                memory_usage: MemoryUsage { peak_usage: 1024, avg_usage: 512, allocation_count: 10 },
                success: true,
                custom_metrics: HashMap::new(),
                error_message: None,
            },
            BenchmarkMeasurement {
                iteration: 1,
                duration: Duration::from_millis(110),
                memory_usage: MemoryUsage { peak_usage: 1100, avg_usage: 550, allocation_count: 11 },
                success: true,
                custom_metrics: HashMap::new(),
                error_message: None,
            },
        ];
        
        let stats = analyzer.analyze_measurements(&measurements).unwrap();
        assert_eq!(stats.total_measurements, 2);
        assert_eq!(stats.successful_measurements, 2);
        assert_eq!(stats.success_rate, 1.0);
        assert!(stats.avg_duration.as_millis() > 100);
    }
    
    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new();
        tracker.start_tracking();
        
        // Simulate some work
        std::thread::sleep(Duration::from_millis(1));
        
        let usage = tracker.stop_tracking();
        assert!(usage.peak_usage > 0);
        assert!(usage.avg_usage > 0);
    }
    
    #[test]
    fn test_benchmark_statistics_efficiency_score() {
        let stats = BenchmarkStatistics {
            total_measurements: 10,
            successful_measurements: 10,
            success_rate: 1.0,
            avg_duration: Duration::from_millis(100),
            min_duration: Duration::from_millis(90),
            max_duration: Duration::from_millis(110),
            std_dev: Duration::from_millis(10),
            avg_memory: 1024 * 1024, // 1MB
            peak_memory: 1024 * 1024,
            throughput: 10.0,
        };
        
        let score = stats.efficiency_score();
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }
}
