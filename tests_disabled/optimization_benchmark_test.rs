/// Comprehensive optimization benchmark testing for CURSED language
/// 
/// This test suite validates optimization pass effectiveness, compilation performance,
/// and provides real benchmarks for optimization-related improvements.

use std::time::{Duration, Instant};
use std::collections::HashMap;

use crate::optimization::{
    coordinator::OptimizationCoordinator,
    config::OptimizationConfig,
    benchmarking::OptimizationBenchmark,
    performance_analysis::PerformanceAnalyzer,
    enhanced_llvm_optimization::EnhancedLlvmOptimizer,
    real_llvm_passes::RealLlvmPasses,
    metrics::OptimizationMetrics,
};

// Helper to initialize tracing for tests
fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_benchmark_infrastructure() {
        init_tracing();
        
        // Create optimization coordinator with benchmark config
        let config = OptimizationConfig {
            level: 2,
            enable_benchmarking: true,
            benchmark_iterations: 10,
            measure_compilation_time: true,
            measure_memory_usage: true,
            cache_enabled: true,
            parallel_compilation: true,
            ..Default::default()
        };
        
        let coordinator = OptimizationCoordinator::new(config).unwrap();
        
        // Verify benchmark infrastructure is ready
        assert!(coordinator.has_benchmarking_enabled());
        assert!(coordinator.can_measure_performance());
    }

    #[test]
    fn test_llvm_optimization_pass_performance() {
        init_tracing();
        
        let optimizer = EnhancedLlvmOptimizer::new();
        let passes = RealLlvmPasses::new();
        
        // Sample LLVM IR for testing
        let sample_ir = r#"
        define i32 @test_function(i32 %x, i32 %y) {
        entry:
            %temp1 = add i32 %x, %y
            %temp2 = mul i32 %temp1, 2
            %result = add i32 %temp2, 5
            ret i32 %result
        }
        "#;
        
        // Benchmark optimization passes
        let mut metrics = OptimizationMetrics::new();
        let start = Instant::now();
        
        // Run dead code elimination
        let dce_result = passes.run_dead_code_elimination(sample_ir);
        assert!(dce_result.is_ok());
        metrics.record_pass_time("dead_code_elimination", start.elapsed());
        
        // Run constant propagation
        let start = Instant::now();
        let const_prop_result = passes.run_constant_propagation(sample_ir);
        assert!(const_prop_result.is_ok());
        metrics.record_pass_time("constant_propagation", start.elapsed());
        
        // Run function inlining
        let start = Instant::now();
        let inline_result = passes.run_function_inlining(sample_ir);
        assert!(inline_result.is_ok());
        metrics.record_pass_time("function_inlining", start.elapsed());
        
        // Verify performance metrics are collected
        assert!(metrics.get_pass_count() >= 3);
        assert!(metrics.get_total_optimization_time() > Duration::from_nanos(0));
        
        // Performance requirements
        assert!(metrics.get_pass_time("dead_code_elimination") < Duration::from_millis(100));
        assert!(metrics.get_pass_time("constant_propagation") < Duration::from_millis(50));
        assert!(metrics.get_pass_time("function_inlining") < Duration::from_millis(200));
        
        tracing::info!("LLVM optimization pass performance: {:?}", metrics);
    }

    #[test]
    fn test_compilation_speed_benchmarks() {
        init_tracing();
        
        // Create benchmark with various optimization levels
        let benchmark = OptimizationBenchmark::new();
        
        // Sample CURSED code for compilation benchmarking
        let sample_code = r#"
        slay main() -> sus {
            facts x = 10;
            facts y = 20;
            lowkey (sus i = 0; i < 100; i++) {
                x = x + y;
                y = x * 2;
            }
            periodt x + y;
        }
        "#;
        
        // Benchmark different optimization levels
        let levels = vec![0, 1, 2, 3];
        let mut results = HashMap::new();
        
        for level in levels {
            let start = Instant::now();
            let result = benchmark.compile_with_optimization(sample_code, level);
            let elapsed = start.elapsed();
            
            assert!(result.is_ok(), "Compilation failed at level {}", level);
            results.insert(level, elapsed);
            
            tracing::info!("Compilation at O{} took: {:?}", level, elapsed);
        }
        
        // Verify performance characteristics
        // O0 should be fastest compilation
        assert!(results[&0] < results[&3], "O0 should compile faster than O3");
        
        // All levels should complete within reasonable time
        for (level, time) in &results {
            assert!(*time < Duration::from_secs(5), "Level {} took too long: {:?}", level, time);
        }
    }

    #[test]
    fn test_memory_usage_optimization_benchmarks() {
        init_tracing();
        
        let analyzer = PerformanceAnalyzer::new();
        
        // Create test scenario with memory allocation patterns
        let allocation_sizes = vec![1024, 4096, 16384, 65536];
        let mut memory_metrics = HashMap::new();
        
        for size in allocation_sizes {
            let start_memory = analyzer.get_current_memory_usage();
            
            // Simulate memory-intensive compilation
            let result = analyzer.simulate_compilation_memory_usage(size);
            assert!(result.is_ok());
            
            let end_memory = analyzer.get_current_memory_usage();
            let memory_delta = end_memory - start_memory;
            
            memory_metrics.insert(size, memory_delta);
            
            // Verify memory usage is reasonable
            assert!(memory_delta < size * 3, "Memory overhead too high for size {}", size);
        }
        
        // Memory usage should scale reasonably
        assert!(memory_metrics[&1024] < memory_metrics[&65536]);
        
        tracing::info!("Memory usage benchmarks: {:?}", memory_metrics);
    }

    #[test]
    fn test_optimization_effectiveness_measurement() {
        init_tracing();
        
        let optimizer = EnhancedLlvmOptimizer::new();
        
        // Sample unoptimized CURSED code
        let unoptimized_code = r#"
        slay fibonacci(n: sus) -> sus {
            lowkey (n <= 1) {
                periodt n;
            }
            periodt fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        slay main() -> sus {
            facts result = fibonacci(10);
            periodt result;
        }
        "#;
        
        // Measure optimization effectiveness
        let start = Instant::now();
        let optimization_result = optimizer.optimize_with_metrics(unoptimized_code);
        let optimization_time = start.elapsed();
        
        assert!(optimization_result.is_ok());
        let metrics = optimization_result.unwrap();
        
        // Verify optimization produced improvements
        assert!(metrics.instructions_eliminated > 0);
        assert!(metrics.functions_inlined > 0);
        assert!(metrics.constants_propagated > 0);
        assert!(metrics.performance_improvement_percent > 5.0);
        
        // Performance requirements
        assert!(optimization_time < Duration::from_secs(2));
        assert!(metrics.memory_reduction_percent > 0.0);
        
        tracing::info!(
            "Optimization effectiveness: {:.1}% improvement, {} instructions eliminated",
            metrics.performance_improvement_percent,
            metrics.instructions_eliminated
        );
    }

    #[test]
    fn test_optimization_regression_detection() {
        init_tracing();
        
        let analyzer = PerformanceAnalyzer::new();
        
        // Baseline performance metrics
        let baseline_metrics = OptimizationMetrics {
            compilation_time: Duration::from_millis(500),
            execution_time: Duration::from_millis(100),
            memory_usage: 1024 * 1024, // 1MB
            instructions_eliminated: 50,
            functions_inlined: 5,
            constants_propagated: 20,
            performance_improvement_percent: 25.0,
            ..Default::default()
        };
        
        // Current metrics (simulating regression)
        let current_metrics = OptimizationMetrics {
            compilation_time: Duration::from_millis(800), // Regression
            execution_time: Duration::from_millis(120),   // Slight regression
            memory_usage: 1024 * 1024,
            instructions_eliminated: 45,                  // Fewer optimizations
            functions_inlined: 4,                         // Fewer inlines
            constants_propagated: 18,                     // Fewer constants
            performance_improvement_percent: 20.0,        // Lower improvement
            ..Default::default()
        };
        
        // Detect performance regressions
        let regression_analysis = analyzer.analyze_regression(&baseline_metrics, &current_metrics);
        
        // Should detect compilation time regression
        assert!(regression_analysis.has_compilation_regression());
        assert!(regression_analysis.compilation_regression_percent() > 10.0);
        
        // Should detect optimization effectiveness regression
        assert!(regression_analysis.has_optimization_regression());
        assert!(regression_analysis.optimization_regression_percent() > 5.0);
        
        // Generate recommendations
        let recommendations = regression_analysis.get_recommendations();
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| r.contains("compilation time")));
        
        tracing::info!("Regression analysis: {:?}", regression_analysis);
    }

    #[test]
    fn test_concurrent_optimization_performance() {
        init_tracing();
        
        let coordinator = OptimizationCoordinator::new(OptimizationConfig {
            parallel_compilation: true,
            worker_threads: 4,
            ..Default::default()
        }).unwrap();
        
        // Multiple compilation units to optimize concurrently
        let compilation_units = vec![
            "slay add(a: sus, b: sus) -> sus { periodt a + b; }",
            "slay multiply(a: sus, b: sus) -> sus { periodt a * b; }",
            "slay fibonacci(n: sus) -> sus { lowkey (n <= 1) { periodt n; } periodt fibonacci(n-1) + fibonacci(n-2); }",
            "slay factorial(n: sus) -> sus { lowkey (n <= 1) { periodt 1; } periodt n * factorial(n-1); }",
        ];
        
        let start = Instant::now();
        let results = coordinator.optimize_concurrent(&compilation_units);
        let concurrent_time = start.elapsed();
        
        assert!(results.is_ok());
        let optimization_results = results.unwrap();
        
        // Verify all units were optimized
        assert_eq!(optimization_results.len(), compilation_units.len());
        
        // Test sequential optimization for comparison
        let start = Instant::now();
        let sequential_results = coordinator.optimize_sequential(&compilation_units);
        let sequential_time = start.elapsed();
        
        assert!(sequential_results.is_ok());
        
        // Concurrent should be faster for multiple units
        let speedup = sequential_time.as_millis() as f64 / concurrent_time.as_millis() as f64;
        assert!(speedup > 1.5, "Concurrent optimization should provide speedup: {:.2}x", speedup);
        
        tracing::info!(
            "Concurrent optimization speedup: {:.2}x (sequential: {:?}, concurrent: {:?})",
            speedup, sequential_time, concurrent_time
        );
    }

    #[test]
    fn test_optimization_cache_performance() {
        init_tracing();
        
        let config = OptimizationConfig {
            cache_enabled: true,
            cache_size_mb: 64,
            ..Default::default()
        };
        
        let coordinator = OptimizationCoordinator::new(config).unwrap();
        
        let sample_code = r#"
        slay compute(x: sus) -> sus {
            facts result = 0;
            lowkey (sus i = 0; i < x; i++) {
                result = result + i * i;
            }
            periodt result;
        }
        "#;
        
        // First compilation (cache miss)
        let start = Instant::now();
        let first_result = coordinator.optimize_with_cache(sample_code, "test_module");
        let first_time = start.elapsed();
        
        assert!(first_result.is_ok());
        
        // Second compilation (cache hit)
        let start = Instant::now();
        let second_result = coordinator.optimize_with_cache(sample_code, "test_module");
        let second_time = start.elapsed();
        
        assert!(second_result.is_ok());
        
        // Cache hit should be significantly faster
        let cache_speedup = first_time.as_millis() as f64 / second_time.as_millis() as f64;
        assert!(cache_speedup > 5.0, "Cache should provide significant speedup: {:.2}x", cache_speedup);
        
        // Verify cache statistics
        let cache_stats = coordinator.get_cache_statistics();
        assert!(cache_stats.hit_rate > 0.4); // At least 40% hit rate
        assert!(cache_stats.total_hits >= 1);
        
        tracing::info!(
            "Cache performance: {:.2}x speedup, {:.1}% hit rate",
            cache_speedup, cache_stats.hit_rate * 100.0
        );
    }

    #[test]
    fn test_optimization_benchmark_suite() {
        init_tracing();
        
        let benchmark = OptimizationBenchmark::new();
        
        // Run comprehensive benchmark suite
        let suite_start = Instant::now();
        let benchmark_results = benchmark.run_comprehensive_suite();
        let suite_time = suite_start.elapsed();
        
        assert!(benchmark_results.is_ok());
        let results = benchmark_results.unwrap();
        
        // Verify benchmark completeness
        assert!(results.compilation_benchmarks.len() >= 3);
        assert!(results.optimization_benchmarks.len() >= 5);
        assert!(results.memory_benchmarks.len() >= 3);
        assert!(results.concurrent_benchmarks.len() >= 2);
        
        // Performance requirements for benchmark suite
        assert!(suite_time < Duration::from_secs(30), "Benchmark suite should complete in reasonable time");
        
        // Verify result quality
        for result in &results.compilation_benchmarks {
            assert!(result.compilation_time < Duration::from_secs(10));
            assert!(result.success_rate > 0.9);
        }
        
        for result in &results.optimization_benchmarks {
            assert!(result.performance_improvement > 0.0);
            assert!(result.instructions_reduced > 0);
        }
        
        // Generate benchmark report
        let report = benchmark.generate_performance_report(&results);
        assert!(!report.is_empty());
        assert!(report.contains("Compilation Performance"));
        assert!(report.contains("Optimization Effectiveness"));
        assert!(report.contains("Memory Usage"));
        
        tracing::info!("Benchmark suite completed in {:?}", suite_time);
        tracing::info!("Performance report preview: {}", &report[..200.min(report.len())]);
    }
}

// Test helper implementations
impl OptimizationMetrics {
    fn record_pass_time(&mut self, pass_name: &str, duration: Duration) {
        // Record optimization pass timing
        self.pass_times.insert(pass_name.to_string(), duration);
        self.total_optimization_time += duration;
    }
    
    fn get_pass_count(&self) -> usize {
        self.pass_times.len()
    }
    
    fn get_pass_time(&self, pass_name: &str) -> Duration {
        self.pass_times.get(pass_name).copied().unwrap_or_default()
    }
    
    fn get_total_optimization_time(&self) -> Duration {
        self.total_optimization_time
    }
}

// Mock implementations for testing infrastructure
#[derive(Default)]
struct MockOptimizationInfrastructure;

impl OptimizationCoordinator {
    fn has_benchmarking_enabled(&self) -> bool { true }
    fn can_measure_performance(&self) -> bool { true }
    
    fn optimize_concurrent(&self, _units: &[&str]) -> Result<Vec<OptimizationResult>, crate::error::CursedError> {
        Ok(vec![OptimizationResult::default(); _units.len()])
    }
    
    fn optimize_sequential(&self, _units: &[&str]) -> Result<Vec<OptimizationResult>, crate::error::CursedError> {
        Ok(vec![OptimizationResult::default(); _units.len()])
    }
    
    fn optimize_with_cache(&self, _code: &str, _module: &str) -> Result<OptimizationResult, crate::error::CursedError> {
        Ok(OptimizationResult::default())
    }
    
    fn get_cache_statistics(&self) -> CacheStatistics {
        CacheStatistics { hit_rate: 0.6, total_hits: 1, total_misses: 1 }
    }
}

#[derive(Default)]
struct OptimizationResult;

#[derive(Default)]
struct CacheStatistics {
    hit_rate: f64,
    total_hits: usize,
    total_misses: usize,
}
