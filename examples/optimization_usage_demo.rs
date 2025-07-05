//! Demonstration of the complete optimization system

use cursed::optimization::{
    OptimizationManager, BaselineComparator, BaselineComparisonConfig, BaselineMetadata,
    EnvironmentInfo, TimeSavingsConfig, BenchmarkConfig, BenchmarkResult,
};
use cursed::codegen::llvm::optimization::OptimizationConfig;
use cursed::error::Result;
use std::time::Duration;
use tempfile::tempdir;

fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 CURSED Optimization System Demo");
    println!("==================================");
    
    // 1. Create optimization manager with baseline comparison
    let temp_dir = tempdir().unwrap();
    let baseline_config = BaselineComparisonConfig {
        tolerance: 0.05,
        min_samples: 10,
        confidence_level: 0.95,
        regression_threshold_percent: 5.0,
        improvement_threshold_percent: 5.0,
        min_confidence_level: 0.8,
        max_baseline_age_days: 30,
        use_statistical_testing: true,
    };
    
    let time_savings_config = TimeSavingsConfig {
        baseline_compile_time_per_unit: Duration::from_secs(3),
        cache_lookup_time: Duration::from_millis(50),
        incremental_analysis_time: Duration::from_millis(100),
        parallel_scheduling_overhead: Duration::from_millis(200),
        include_confidence_intervals: true,
        ..Default::default()
    };
    
    let mut optimization_manager = OptimizationManager::new()
        .with_config(OptimizationConfig::release_config())
        .with_baseline_comparison(temp_dir.path(), baseline_config)
        .with_time_savings_config(time_savings_config);
    
    println!("✅ Optimization manager created with baseline comparison enabled");
    
    // 2. Create a baseline from simulated benchmark results
    println!("\n📊 Creating Performance Baseline");
    println!("--------------------------------");
    
    let baseline_result = optimization_manager.create_baseline(
        "1.0.0".to_string(),
        Some("abc123def456".to_string()),
        Some("Initial baseline for optimization demo".to_string()),
    )?;
    
    if let Some(baseline) = baseline_result {
        println!("✅ Baseline created with version: {}", baseline.version);
        println!("   - Benchmarks: {}", baseline.benchmark_results.len());
        println!("   - Environment: {} on {}", baseline.metadata.environment.os, baseline.metadata.environment.arch);
        println!("   - CPU cores: {}", baseline.metadata.environment.cpu_cores);
        println!("   - Memory: {} MB", baseline.metadata.environment.memory_mb);
    } else {
        println!("⚠️  Baseline creation skipped (benchmarking not configured)");
    }
    
    // 3. Demonstrate time savings calculation
    println!("\n⏱️  Time Savings Analysis");
    println!("------------------------");
    
    let mut timing_context = optimization_manager.start_timing_measurement();
    
    // Simulate optimization pass timings
    optimization_manager.time_savings_calculator.record_optimization_timing(
        &mut timing_context,
        "dead_code_elimination",
        Duration::from_millis(300)
    );
    optimization_manager.time_savings_calculator.record_optimization_timing(
        &mut timing_context,
        "constant_folding",
        Duration::from_millis(200)
    );
    optimization_manager.time_savings_calculator.record_optimization_timing(
        &mut timing_context,
        "loop_optimization",
        Duration::from_millis(500)
    );
    
    // Simulate cache performance
    timing_context.cache_timings.total_lookup_time = Duration::from_millis(250);
    timing_context.cache_timings.cache_hits = 15;
    timing_context.cache_timings.cache_misses = 3;
    
    // Simulate parallel execution
    timing_context.parallel_metrics.worker_threads = 8;
    timing_context.parallel_metrics.thread_utilizations = vec![0.85, 0.92, 0.78, 0.88, 0.90, 0.82, 0.95, 0.87];
    timing_context.parallel_metrics.work_stealing_events = 12;
    timing_context.parallel_metrics.synchronization_overhead = Duration::from_millis(150);
    
    // Calculate actual time savings
    let time_savings = optimization_manager.calculate_time_savings(
        &timing_context,
        25,  // units compiled
        15,  // units from cache
        5,   // units from incremental
        2.8, // parallel efficiency
    )?;
    
    println!("📈 Time Savings Analysis Results:");
    println!("   - Total time saved: {:.2} seconds", time_savings.total_time_saved.as_secs_f64());
    println!("   - Cache savings: {:.2} seconds", time_savings.cache_savings.as_secs_f64());
    println!("   - Incremental savings: {:.2} seconds", time_savings.incremental_savings.as_secs_f64());
    println!("   - Parallel savings: {:.2} seconds", time_savings.parallel_savings.as_secs_f64());
    println!("   - LLVM optimization savings: {:.2} seconds", time_savings.llvm_optimization_savings.as_secs_f64());
    println!("   - Efficiency improvement: {:.1}%", time_savings.efficiency_improvement_percent);
    println!("   - Throughput improvement: {:.2} units/sec", time_savings.throughput_improvement);
    
    // 4. Demonstrate optimization breakdown analysis
    println!("\n🔍 Optimization Breakdown");
    println!("-------------------------");
    
    for (optimization_type, savings) in &time_savings.savings_breakdown {
        println!("🔧 {} Optimization:", savings.optimization_name);
        println!("   - Time saved: {:.2} seconds", savings.time_saved.as_secs_f64());
        println!("   - Units affected: {}", savings.units_affected);
        println!("   - Average savings per unit: {:.2} seconds", savings.avg_savings_per_unit.as_secs_f64());
        println!("   - Confidence level: {:.1}%", savings.confidence_level * 100.0);
        
        if !savings.metadata.is_empty() {
            println!("   - Metadata:");
            for (key, value) in &savings.metadata {
                println!("     • {}: {}", key, value);
            }
        }
        println!();
    }
    
    // 5. Demonstrate trend analysis
    println!("📊 Performance Trend Analysis");
    println!("-----------------------------");
    
    if let Some(trend) = optimization_manager.get_time_savings_analysis() {
        println!("📈 Recent Performance Trends:");
        println!("   - Average efficiency ratio: {:.3}", trend.average_efficiency_ratio);
        println!("   - Average parallel efficiency: {:.2}x", trend.average_parallel_efficiency);
        println!("   - Measurements analyzed: {}", trend.measurement_count);
        println!("   - Trend direction: {:?}", trend.trend_direction);
    } else {
        println!("📊 Insufficient data for trend analysis (need more compilation history)");
    }
    
    // 6. Generate optimization recommendations
    println!("\n💡 Optimization Recommendations");
    println!("-------------------------------");
    
    let sample_code = r#"
    fn fibonacci(n: usize) -> usize {
        lowkey (n <= 1) {
            facts n
        } bestie {
            facts fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
    "#;
    
    let recommendations = optimization_manager.generate_recommendations(sample_code);
    
    for (i, recommendation) in recommendations.iter().enumerate() {
        println!("🎯 Recommendation {}:", i + 1);
        println!("   - Category: {:?}", recommendation.category);
        println!("   - Priority: {:?}", recommendation.priority);
        println!("   - Description: {}", recommendation.description);
        println!("   - Suggested config: {:?}", recommendation.suggested_config);
        println!();
    }
    
    // 7. Performance validation demonstration
    println!("🔍 Performance Validation");
    println!("-------------------------");
    
    let validation_result = optimization_manager.validate_performance(None)?;
    if validation_result {
        println!("✅ Performance validation passed - no regressions detected");
    } else {
        println!("⚠️  Performance validation failed - regressions detected");
    }
    
    // 8. Summary
    println!("\n🎉 Optimization System Demo Complete!");
    println!("=====================================");
    println!("The CURSED optimization system provides:");
    println!("• 📊 Real-time performance monitoring and analysis");
    println!("• 🎯 Intelligent optimization recommendations");
    println!("• ⏱️  Accurate time savings calculations");
    println!("• 📈 Performance trend analysis and regression detection");
    println!("• 🔧 Comprehensive optimization breakdown and insights");
    println!("• 🛡️  Production-ready performance validation");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_demo() {
        // Test that the demo can run without panicking
        // In a real scenario, this would be more comprehensive
        let result = main();
        assert!(result.is_ok());
    }
}
