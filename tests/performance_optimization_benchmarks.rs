/// Performance Optimization System Benchmarks
/// 
/// This module provides comprehensive benchmarks for measuring and comparing
/// performance improvements from the CURSED optimization system.

use cursed::optimization::comprehensive_performance_system::*;
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use cursed::optimization::pgo::{PgoConfig, InstrumentationMode, OptimizationStrategy};
use cursed::error::Result;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio;
use tracing_test::traced_test;

/// Benchmark suite for optimization system performance
pub struct OptimizationBenchmarkSuite {
    context: inkwell::context::Context,
    baseline_results: Option<BenchmarkResults>,
    optimization_results: HashMap<OptimizationLevel, BenchmarkResults>,
    pgo_results: Option<BenchmarkResults>,
}

impl OptimizationBenchmarkSuite {
    pub fn new() -> Self {
        Self {
            context: inkwell::context::Context::create(),
            baseline_results: None,
            optimization_results: HashMap::new(),
            pgo_results: None,
        }
    }
    
    /// Run comprehensive optimization benchmarks
    pub async fn run_comprehensive_benchmarks(&mut self) -> Result<BenchmarkReport> {
        println!("Starting comprehensive optimization benchmarks...");
        
        // 1. Baseline benchmark (O0)
        self.baseline_results = Some(self.run_baseline_benchmark().await?);
        
        // 2. Optimization level benchmarks
        for level in [
            OptimizationLevel::Less,
            OptimizationLevel::Default, 
            OptimizationLevel::Aggressive,
            OptimizationLevel::Size,
            OptimizationLevel::SizeAggressive
        ] {
            let results = self.run_optimization_level_benchmark(level.clone()).await?;
            self.optimization_results.insert(level, results);
        }
        
        // 3. Profile-guided optimization benchmark
        self.pgo_results = Some(self.run_pgo_benchmark().await?);
        
        // 4. Generate comprehensive report
        Ok(self.generate_benchmark_report())
    }
    
    async fn run_baseline_benchmark(&self) -> Result<BenchmarkResults> {
        println!("Running baseline benchmark (O0)...");
        
        let mut config = PerformanceConfig::default();
        config.optimization_level = OptimizationLevel::None;
        config.enable_benchmarking = true;
        config.benchmark_iterations = 10;
        
        let mut performance_system = ComprehensivePerformanceSystem::new(&self.context, config)?;
        
        // Create test workload
        let workload = self.create_benchmark_workload("baseline");
        let start_time = Instant::now();
        
        let mut total_compilation_time = Duration::default();
        let mut total_optimization_time = Duration::default();
        let mut memory_usage_samples = Vec::new();
        
        for (module, source_files) in workload {
            let results = performance_system.optimize_module(&module, &source_files).await?;
            total_compilation_time += results.compilation_time;
            total_optimization_time += results.total_optimization_time;
            
            if let Some(metrics) = &results.performance_metrics {
                memory_usage_samples.push(metrics.memory_usage);
            }
        }
        
        let benchmark_results = performance_system.run_benchmarks("baseline").await?;
        
        Ok(BenchmarkResults {
            benchmark_name: "Baseline (O0)".to_string(),
            optimization_level: OptimizationLevel::None,
            total_duration: start_time.elapsed(),
            compilation_time: total_compilation_time,
            optimization_time: total_optimization_time,
            memory_usage: memory_usage_samples.iter().sum::<u64>() / memory_usage_samples.len() as u64,
            performance_improvement: 0.0, // Baseline
            code_size_reduction: 0.0, // Baseline
            passes_applied: 0,
            detailed_results: benchmark_results,
        })
    }
    
    async fn run_optimization_level_benchmark(&self, level: OptimizationLevel) -> Result<BenchmarkResults> {
        println!("Running optimization benchmark for level: {:?}", level);
        
        let mut config = PerformanceConfig::default();
        config.optimization_level = level.clone();
        config.enable_benchmarking = true;
        config.benchmark_iterations = 10;
        
        // Enable all optimization features for aggressive levels
        match level {
            OptimizationLevel::Aggressive => {
                config.enable_function_inlining = true;
                config.inline_threshold = 500;
                config.enable_dead_code_elimination = true;
                config.enable_constant_propagation = true;
                config.enable_loop_optimization = true;
                config.enable_vectorization = true;
                config.enable_instruction_scheduling = true;
            }
            OptimizationLevel::Default => {
                config.enable_function_inlining = true;
                config.inline_threshold = 225;
                config.enable_dead_code_elimination = true;
                config.enable_constant_propagation = true;
                config.enable_loop_optimization = true;
            }
            _ => {
                // Use default settings
            }
        }
        
        let mut performance_system = ComprehensivePerformanceSystem::new(&self.context, config)?;
        
        // Create test workload
        let workload = self.create_benchmark_workload(&format!("{:?}", level));
        let start_time = Instant::now();
        
        let mut total_compilation_time = Duration::default();
        let mut total_optimization_time = Duration::default();
        let mut memory_usage_samples = Vec::new();
        let mut total_passes_applied = 0;
        let mut performance_improvements = Vec::new();
        let mut code_size_reductions = Vec::new();
        
        for (module, source_files) in workload {
            let results = performance_system.optimize_module(&module, &source_files).await?;
            total_compilation_time += results.compilation_time;
            total_optimization_time += results.total_optimization_time;
            
            if let Some(metrics) = &results.performance_metrics {
                memory_usage_samples.push(metrics.memory_usage);
            }
            
            if let Some(llvm_results) = &results.llvm_optimization_results {
                total_passes_applied += llvm_results.passes_applied;
                performance_improvements.push(llvm_results.estimated_performance_improvement);
                code_size_reductions.push(llvm_results.code_size_reduction);
            }
        }
        
        let benchmark_results = performance_system.run_benchmarks(&format!("{:?}", level)).await?;
        
        let avg_performance_improvement = if !performance_improvements.is_empty() {
            performance_improvements.iter().sum::<f64>() / performance_improvements.len() as f64
        } else {
            0.0
        };
        
        let avg_code_size_reduction = if !code_size_reductions.is_empty() {
            code_size_reductions.iter().sum::<f64>() / code_size_reductions.len() as f64
        } else {
            0.0
        };
        
        Ok(BenchmarkResults {
            benchmark_name: format!("Optimization Level {:?}", level),
            optimization_level: level,
            total_duration: start_time.elapsed(),
            compilation_time: total_compilation_time,
            optimization_time: total_optimization_time,
            memory_usage: if !memory_usage_samples.is_empty() {
                memory_usage_samples.iter().sum::<u64>() / memory_usage_samples.len() as u64
            } else {
                0
            },
            performance_improvement: avg_performance_improvement,
            code_size_reduction: avg_code_size_reduction,
            passes_applied: total_passes_applied,
            detailed_results: benchmark_results,
        })
    }
    
    async fn run_pgo_benchmark(&self) -> Result<BenchmarkResults> {
        println!("Running profile-guided optimization benchmark...");
        
        let mut config = PerformanceConfig::default();
        config.optimization_level = OptimizationLevel::Aggressive;
        config.enable_pgo = true;
        config.pgo_config = PgoConfig {
            enabled: true,
            instrumentation_mode: InstrumentationMode::Frontend,
            optimization_strategy: OptimizationStrategy::Speed,
            hot_function_threshold: 0.05,
            enable_indirect_call_promotion: true,
            enable_value_profiling: true,
            ..PgoConfig::default()
        };
        config.enable_benchmarking = true;
        config.benchmark_iterations = 5; // Fewer iterations due to PGO overhead
        
        let mut performance_system = ComprehensivePerformanceSystem::new(&self.context, config)?;
        
        // Create test workload
        let workload = self.create_benchmark_workload("pgo");
        let start_time = Instant::now();
        
        let mut total_compilation_time = Duration::default();
        let mut total_optimization_time = Duration::default();
        let mut memory_usage_samples = Vec::new();
        let mut total_passes_applied = 0;
        let mut performance_improvements = Vec::new();
        let mut pgo_overheads = Vec::new();
        
        for (module, source_files) in workload {
            let results = performance_system.optimize_module(&module, &source_files).await?;
            total_compilation_time += results.compilation_time;
            total_optimization_time += results.total_optimization_time;
            
            if let Some(metrics) = &results.performance_metrics {
                memory_usage_samples.push(metrics.memory_usage);
            }
            
            if let Some(llvm_results) = &results.llvm_optimization_results {
                total_passes_applied += llvm_results.passes_applied;
                performance_improvements.push(llvm_results.estimated_performance_improvement);
            }
            
            if let Some(pgo_results) = &results.pgo_results {
                pgo_overheads.push(pgo_results.instrumentation_overhead);
            }
        }
        
        let benchmark_results = performance_system.run_benchmarks("pgo").await?;
        
        let avg_performance_improvement = if !performance_improvements.is_empty() {
            performance_improvements.iter().sum::<f64>() / performance_improvements.len() as f64
        } else {
            0.0
        };
        
        let avg_pgo_overhead = if !pgo_overheads.is_empty() {
            pgo_overheads.iter().sum::<f64>() / pgo_overheads.len() as f64
        } else {
            0.0
        };
        
        println!("PGO average overhead: {:.2}%", avg_pgo_overhead);
        
        Ok(BenchmarkResults {
            benchmark_name: "Profile-Guided Optimization".to_string(),
            optimization_level: OptimizationLevel::Aggressive,
            total_duration: start_time.elapsed(),
            compilation_time: total_compilation_time,
            optimization_time: total_optimization_time,
            memory_usage: if !memory_usage_samples.is_empty() {
                memory_usage_samples.iter().sum::<u64>() / memory_usage_samples.len() as u64
            } else {
                0
            },
            performance_improvement: avg_performance_improvement,
            code_size_reduction: 0.0, // Not directly measured for PGO
            passes_applied: total_passes_applied,
            detailed_results: benchmark_results,
        })
    }
    
    fn create_benchmark_workload(&self, workload_name: &str) -> Vec<(inkwell::module::Module, Vec<PathBuf>)> {
        let mut workload = Vec::new();
        
        // Create various types of modules to test different optimization scenarios
        for i in 0..5 {
            let module = self.context.create_module(&format!("{}_module_{}", workload_name, i));
            let source_files = vec![
                PathBuf::from(format!("{}_file_{}a.csd", workload_name, i)),
                PathBuf::from(format!("{}_file_{}b.csd", workload_name, i)),
            ];
            
            // Add some functions to the module to make optimization meaningful
            self.add_test_functions_to_module(&module);
            
            workload.push((module, source_files));
        }
        
        workload
    }
    
    fn add_test_functions_to_module(&self, module: &inkwell::module::Module) {
        // Add various function types to test different optimization scenarios
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        
        // Simple function for inlining tests
        let simple_fn = module.add_function("simple_add", fn_type, None);
        
        // More complex function for dead code elimination tests
        let complex_fn = module.add_function("complex_computation", fn_type, None);
        
        // Loop function for loop optimization tests
        let loop_fn = module.add_function("loop_computation", fn_type, None);
        
        // Functions are just declarations for benchmark purposes
        // In a real scenario, these would have bodies that could be optimized
    }
    
    fn generate_benchmark_report(&self) -> BenchmarkReport {
        let mut report = BenchmarkReport {
            timestamp: chrono::Utc::now(),
            baseline: self.baseline_results.clone(),
            optimization_levels: self.optimization_results.clone(),
            pgo_results: self.pgo_results.clone(),
            comparisons: Vec::new(),
            summary: BenchmarkSummary::default(),
        };
        
        // Generate comparisons
        if let Some(ref baseline) = report.baseline {
            for (level, results) in &report.optimization_levels {
                let comparison = self.compare_results(baseline, results);
                report.comparisons.push(comparison);
            }
            
            if let Some(ref pgo_results) = report.pgo_results {
                let pgo_comparison = self.compare_results(baseline, pgo_results);
                report.comparisons.push(pgo_comparison);
            }
        }
        
        // Generate summary
        report.summary = self.generate_summary(&report);
        
        report
    }
    
    fn compare_results(&self, baseline: &BenchmarkResults, optimized: &BenchmarkResults) -> BenchmarkComparison {
        let compilation_speedup = if optimized.compilation_time > Duration::default() && baseline.compilation_time > Duration::default() {
            baseline.compilation_time.as_secs_f64() / optimized.compilation_time.as_secs_f64()
        } else {
            1.0
        };
        
        let optimization_overhead = if baseline.optimization_time > Duration::default() {
            (optimized.optimization_time.as_secs_f64() / baseline.optimization_time.as_secs_f64() - 1.0) * 100.0
        } else {
            optimized.optimization_time.as_millis() as f64 / 10.0 // Rough percentage for very small times
        };
        
        let memory_efficiency = if baseline.memory_usage > 0 {
            (baseline.memory_usage as f64 / optimized.memory_usage as f64 - 1.0) * 100.0
        } else {
            0.0
        };
        
        BenchmarkComparison {
            baseline_name: baseline.benchmark_name.clone(),
            optimized_name: optimized.benchmark_name.clone(),
            compilation_speedup,
            optimization_overhead,
            performance_improvement: optimized.performance_improvement,
            code_size_reduction: optimized.code_size_reduction,
            memory_efficiency,
            passes_applied: optimized.passes_applied,
            overall_score: self.calculate_overall_score(compilation_speedup, optimized.performance_improvement, optimization_overhead),
        }
    }
    
    fn calculate_overall_score(&self, compilation_speedup: f64, performance_improvement: f64, optimization_overhead: f64) -> f64 {
        // Weighted score combining different factors
        let speedup_weight = 0.3;
        let performance_weight = 0.5;
        let overhead_penalty_weight = 0.2;
        
        let speedup_score = (compilation_speedup - 1.0) * 100.0; // Convert to percentage
        let performance_score = performance_improvement;
        let overhead_penalty = optimization_overhead * overhead_penalty_weight;
        
        (speedup_score * speedup_weight + performance_score * performance_weight - overhead_penalty).max(0.0)
    }
    
    fn generate_summary(&self, report: &BenchmarkReport) -> BenchmarkSummary {
        let mut best_compilation_speedup = 1.0;
        let mut best_performance_improvement = 0.0;
        let mut best_code_size_reduction = 0.0;
        let mut best_overall_level = None;
        let mut best_overall_score = 0.0;
        
        for comparison in &report.comparisons {
            if comparison.compilation_speedup > best_compilation_speedup {
                best_compilation_speedup = comparison.compilation_speedup;
            }
            
            if comparison.performance_improvement > best_performance_improvement {
                best_performance_improvement = comparison.performance_improvement;
            }
            
            if comparison.code_size_reduction > best_code_size_reduction {
                best_code_size_reduction = comparison.code_size_reduction;
            }
            
            if comparison.overall_score > best_overall_score {
                best_overall_score = comparison.overall_score;
                best_overall_level = Some(comparison.optimized_name.clone());
            }
        }
        
        BenchmarkSummary {
            best_compilation_speedup,
            best_performance_improvement,
            best_code_size_reduction,
            best_overall_level,
            best_overall_score,
            total_optimizations_tested: report.optimization_levels.len() + if report.pgo_results.is_some() { 1 } else { 0 },
            average_optimization_overhead: report.comparisons.iter()
                .map(|c| c.optimization_overhead)
                .sum::<f64>() / report.comparisons.len() as f64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub benchmark_name: String,
    pub optimization_level: OptimizationLevel,
    pub total_duration: Duration,
    pub compilation_time: Duration,
    pub optimization_time: Duration,
    pub memory_usage: u64,
    pub performance_improvement: f64,
    pub code_size_reduction: f64,
    pub passes_applied: u32,
    pub detailed_results: cursed::optimization::comprehensive_performance_system::BenchmarkResults,
}

#[derive(Debug, Clone)]
pub struct BenchmarkComparison {
    pub baseline_name: String,
    pub optimized_name: String,
    pub compilation_speedup: f64,
    pub optimization_overhead: f64,
    pub performance_improvement: f64,
    pub code_size_reduction: f64,
    pub memory_efficiency: f64,
    pub passes_applied: u32,
    pub overall_score: f64,
}

#[derive(Debug, Clone)]
pub struct BenchmarkReport {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub baseline: Option<BenchmarkResults>,
    pub optimization_levels: HashMap<OptimizationLevel, BenchmarkResults>,
    pub pgo_results: Option<BenchmarkResults>,
    pub comparisons: Vec<BenchmarkComparison>,
    pub summary: BenchmarkSummary,
}

#[derive(Debug, Clone, Default)]
pub struct BenchmarkSummary {
    pub best_compilation_speedup: f64,
    pub best_performance_improvement: f64,
    pub best_code_size_reduction: f64,
    pub best_overall_level: Option<String>,
    pub best_overall_score: f64,
    pub total_optimizations_tested: usize,
    pub average_optimization_overhead: f64,
}

impl BenchmarkReport {
    /// Generate a human-readable report
    pub fn generate_report_text(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# CURSED Optimization System Benchmark Report\n\n");
        report.push_str(&format!("Generated: {}\n\n", self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
        
        // Summary section
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- Total optimizations tested: {}\n", self.summary.total_optimizations_tested));
        report.push_str(&format!("- Best compilation speedup: {:.2}x\n", self.summary.best_compilation_speedup));
        report.push_str(&format!("- Best performance improvement: {:.2}%\n", self.summary.best_performance_improvement));
        report.push_str(&format!("- Best code size reduction: {:.2}%\n", self.summary.best_code_size_reduction));
        if let Some(ref best_level) = self.summary.best_overall_level {
            report.push_str(&format!("- Best overall optimization: {} (score: {:.2})\n", best_level, self.summary.best_overall_score));
        }
        report.push_str(&format!("- Average optimization overhead: {:.2}%\n\n", self.summary.average_optimization_overhead));
        
        // Detailed results
        report.push_str("## Detailed Results\n\n");
        
        if let Some(ref baseline) = self.baseline {
            report.push_str(&format!("### Baseline ({})\n", baseline.benchmark_name));
            report.push_str(&format!("- Compilation time: {:?}\n", baseline.compilation_time));
            report.push_str(&format!("- Memory usage: {} bytes\n", baseline.memory_usage));
            report.push_str("\n");
        }
        
        for (level, results) in &self.optimization_levels {
            report.push_str(&format!("### {:?}\n", level));
            report.push_str(&format!("- Compilation time: {:?}\n", results.compilation_time));
            report.push_str(&format!("- Optimization time: {:?}\n", results.optimization_time));
            report.push_str(&format!("- Memory usage: {} bytes\n", results.memory_usage));
            report.push_str(&format!("- Performance improvement: {:.2}%\n", results.performance_improvement));
            report.push_str(&format!("- Code size reduction: {:.2}%\n", results.code_size_reduction));
            report.push_str(&format!("- Passes applied: {}\n", results.passes_applied));
            report.push_str("\n");
        }
        
        if let Some(ref pgo_results) = self.pgo_results {
            report.push_str("### Profile-Guided Optimization\n");
            report.push_str(&format!("- Compilation time: {:?}\n", pgo_results.compilation_time));
            report.push_str(&format!("- Optimization time: {:?}\n", pgo_results.optimization_time));
            report.push_str(&format!("- Memory usage: {} bytes\n", pgo_results.memory_usage));
            report.push_str(&format!("- Performance improvement: {:.2}%\n", pgo_results.performance_improvement));
            report.push_str(&format!("- Passes applied: {}\n", pgo_results.passes_applied));
            report.push_str("\n");
        }
        
        // Comparisons
        report.push_str("## Performance Comparisons\n\n");
        
        for comparison in &self.comparisons {
            report.push_str(&format!("### {} vs {}\n", comparison.baseline_name, comparison.optimized_name));
            report.push_str(&format!("- Compilation speedup: {:.2}x\n", comparison.compilation_speedup));
            report.push_str(&format!("- Optimization overhead: {:.2}%\n", comparison.optimization_overhead));
            report.push_str(&format!("- Performance improvement: {:.2}%\n", comparison.performance_improvement));
            report.push_str(&format!("- Code size reduction: {:.2}%\n", comparison.code_size_reduction));
            report.push_str(&format!("- Memory efficiency: {:.2}%\n", comparison.memory_efficiency));
            report.push_str(&format!("- Overall score: {:.2}\n", comparison.overall_score));
            report.push_str("\n");
        }
        
        report
    }
    
    /// Export report to file
    pub fn export_to_file(&self, path: &std::path::Path) -> Result<()> {
        let report_text = self.generate_report_text();
        std::fs::write(path, report_text)?;
        Ok(())
    }
}

// Test functions for the benchmark suite

#[tokio::test]
#[traced_test]
async fn test_optimization_benchmark_suite() -> Result<()> {
    let mut benchmark_suite = OptimizationBenchmarkSuite::new();
    let report = benchmark_suite.run_comprehensive_benchmarks().await?;
    
    // Verify report structure
    assert!(report.baseline.is_some());
    assert!(!report.optimization_levels.is_empty());
    assert!(report.pgo_results.is_some());
    assert!(!report.comparisons.is_empty());
    
    // Verify summary makes sense
    assert!(report.summary.best_compilation_speedup >= 1.0);
    assert!(report.summary.best_performance_improvement >= 0.0);
    assert!(report.summary.total_optimizations_tested > 0);
    
    // Print report
    println!("\n{}", report.generate_report_text());
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_optimization_level_performance_progression() -> Result<()> {
    let mut benchmark_suite = OptimizationBenchmarkSuite::new();
    
    // Run benchmarks for different optimization levels
    let baseline = benchmark_suite.run_baseline_benchmark().await?;
    let o1_results = benchmark_suite.run_optimization_level_benchmark(OptimizationLevel::Less).await?;
    let o2_results = benchmark_suite.run_optimization_level_benchmark(OptimizationLevel::Default).await?;
    let o3_results = benchmark_suite.run_optimization_level_benchmark(OptimizationLevel::Aggressive).await?;
    
    // Verify optimization progression
    // Higher levels should generally apply more passes
    assert!(o1_results.passes_applied >= baseline.passes_applied);
    assert!(o2_results.passes_applied >= o1_results.passes_applied);
    assert!(o3_results.passes_applied >= o2_results.passes_applied);
    
    // Performance improvement should generally increase with optimization level
    assert!(o1_results.performance_improvement >= baseline.performance_improvement);
    assert!(o2_results.performance_improvement >= o1_results.performance_improvement);
    assert!(o3_results.performance_improvement >= o2_results.performance_improvement);
    
    println!("Optimization level progression:");
    println!("Baseline: {} passes, {:.2}% improvement", baseline.passes_applied, baseline.performance_improvement);
    println!("O1: {} passes, {:.2}% improvement", o1_results.passes_applied, o1_results.performance_improvement);
    println!("O2: {} passes, {:.2}% improvement", o2_results.passes_applied, o2_results.performance_improvement);
    println!("O3: {} passes, {:.2}% improvement", o3_results.passes_applied, o3_results.performance_improvement);
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_pgo_effectiveness_benchmark() -> Result<()> {
    let mut benchmark_suite = OptimizationBenchmarkSuite::new();
    
    // Run baseline and PGO benchmarks
    let baseline = benchmark_suite.run_baseline_benchmark().await?;
    let pgo_results = benchmark_suite.run_pgo_benchmark().await?;
    
    // PGO should provide significant performance improvements
    assert!(pgo_results.performance_improvement > baseline.performance_improvement);
    assert!(pgo_results.performance_improvement >= 10.0); // At least 10% improvement expected
    
    // PGO compilation might take longer due to instrumentation
    // but should provide substantial runtime benefits
    let compilation_overhead = (pgo_results.compilation_time.as_secs_f64() / baseline.compilation_time.as_secs_f64() - 1.0) * 100.0;
    
    println!("PGO effectiveness:");
    println!("Performance improvement: {:.2}%", pgo_results.performance_improvement);
    println!("Compilation overhead: {:.2}%", compilation_overhead);
    println!("Passes applied: {}", pgo_results.passes_applied);
    
    // The performance benefit should outweigh the compilation cost
    assert!(pgo_results.performance_improvement > compilation_overhead);
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_memory_usage_optimization_benchmark() -> Result<()> {
    let mut benchmark_suite = OptimizationBenchmarkSuite::new();
    
    // Test different optimization strategies for memory usage
    let default_results = benchmark_suite.run_optimization_level_benchmark(OptimizationLevel::Default).await?;
    let size_results = benchmark_suite.run_optimization_level_benchmark(OptimizationLevel::Size).await?;
    let size_aggressive_results = benchmark_suite.run_optimization_level_benchmark(OptimizationLevel::SizeAggressive).await?;
    
    // Size optimizations should generally use less memory
    println!("Memory usage comparison:");
    println!("Default: {} bytes", default_results.memory_usage);
    println!("Size: {} bytes", size_results.memory_usage);
    println!("Size Aggressive: {} bytes", size_aggressive_results.memory_usage);
    
    // Code size reduction should be higher for size-optimized builds
    assert!(size_results.code_size_reduction >= default_results.code_size_reduction);
    assert!(size_aggressive_results.code_size_reduction >= size_results.code_size_reduction);
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_benchmark_report_generation() -> Result<()> {
    let mut benchmark_suite = OptimizationBenchmarkSuite::new();
    let report = benchmark_suite.run_comprehensive_benchmarks().await?;
    
    // Generate and verify report text
    let report_text = report.generate_report_text();
    assert!(report_text.contains("# CURSED Optimization System Benchmark Report"));
    assert!(report_text.contains("## Summary"));
    assert!(report_text.contains("## Detailed Results"));
    assert!(report_text.contains("## Performance Comparisons"));
    
    // Test export to file
    let report_path = std::path::PathBuf::from("test_benchmark_report.md");
    report.export_to_file(&report_path)?;
    
    // Verify file was created and contains expected content
    assert!(report_path.exists());
    let file_content = std::fs::read_to_string(&report_path)?;
    assert_eq!(file_content, report_text);
    
    // Clean up
    std::fs::remove_file(&report_path)?;
    
    println!("Report generation and export successful");
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_optimization_pass_effectiveness() -> Result<()> {
    let benchmark_suite = OptimizationBenchmarkSuite::new();
    
    // Test individual optimization pass effectiveness
    let context = &benchmark_suite.context;
    
    let test_cases = vec![
        ("function_inlining", OptimizationLevel::Default),
        ("dead_code_elimination", OptimizationLevel::Default),
        ("constant_propagation", OptimizationLevel::Default),
        ("loop_optimization", OptimizationLevel::Aggressive),
        ("vectorization", OptimizationLevel::Aggressive),
    ];
    
    for (pass_name, opt_level) in test_cases {
        let mut config = PerformanceConfig::default();
        config.optimization_level = opt_level.clone();
        
        // Enable specific pass
        match pass_name {
            "function_inlining" => {
                config.enable_function_inlining = true;
                config.inline_threshold = 500;
            }
            "dead_code_elimination" => {
                config.enable_dead_code_elimination = true;
            }
            "constant_propagation" => {
                config.enable_constant_propagation = true;
            }
            "loop_optimization" => {
                config.enable_loop_optimization = true;
            }
            "vectorization" => {
                config.enable_vectorization = true;
            }
            _ => {}
        }
        
        let mut performance_system = ComprehensivePerformanceSystem::new(context, config)?;
        
        let module = context.create_module(&format!("test_{}", pass_name));
        let source_files = vec![PathBuf::from(format!("{}_test.csd", pass_name))];
        
        let results = performance_system.optimize_module(&module, &source_files).await?;
        
        // Verify optimization was applied
        if let Some(llvm_results) = &results.llvm_optimization_results {
            assert!(llvm_results.passes_applied > 0);
            
            match pass_name {
                "function_inlining" => {
                    // Should have some function inlining activity
                    println!("{}: {} functions inlined", pass_name, llvm_results.functions_inlined);
                }
                "dead_code_elimination" => {
                    // Should eliminate some dead code
                    println!("{}: {} dead instructions eliminated", pass_name, llvm_results.dead_code_eliminated);
                }
                "constant_propagation" => {
                    // Should propagate some constants
                    println!("{}: {} constants propagated", pass_name, llvm_results.constants_propagated);
                }
                "loop_optimization" => {
                    // Should optimize some loops
                    println!("{}: {} loops optimized", pass_name, llvm_results.loops_optimized);
                }
                "vectorization" => {
                    // Should apply vectorization
                    println!("{}: {} vectorization applications", pass_name, llvm_results.vectorization_applied);
                }
                _ => {}
            }
            
            assert!(llvm_results.estimated_performance_improvement >= 0.0);
        }
    }
    
    Ok(())
}
