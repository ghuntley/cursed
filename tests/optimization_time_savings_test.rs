//! Tests for the time savings calculation system

use cursed::optimization::{
    TimeSavingsCalculator, TimeSavingsConfig, CompilationTimingContext,
    CacheTimings, ParallelMetrics, UnitTiming,
};
use cursed::optimization::metrics::CompilationUnit;
use cursed::error::Result;
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[test]
fn test_time_savings_calculator_creation() {
    let config = TimeSavingsConfig::default();
    let _calculator = TimeSavingsCalculator::new(config);
}

#[test]
fn test_start_measurement() {
    let config = TimeSavingsConfig::default();
    let mut calculator = TimeSavingsCalculator::new(config);
    
    let context = calculator.start_measurement();
    assert!(context.unit_timings.is_empty());
    assert!(context.optimization_timings.is_empty());
}

#[test]
fn test_cache_savings_calculation() -> Result<()> {
    let config = TimeSavingsConfig {
        baseline_compile_time_per_unit: Duration::from_secs(3),
        cache_lookup_time: Duration::from_millis(50),
        ..Default::default()
    };
    let mut calculator = TimeSavingsCalculator::new(config);
    
    let mut context = create_sample_timing_context();
    context.cache_timings = CacheTimings {
        total_lookup_time: Duration::from_millis(250), // 5 lookups * 50ms avg
        total_store_time: Duration::from_millis(100),
        cache_hits: 5,
        cache_misses: 1,
    };
    
    let analysis = calculator.calculate_time_savings(
        &context,
        10, // units compiled
        5,  // units from cache
        0,  // units from incremental
        1.0, // parallel efficiency (no parallelism)
    )?;
    
    // Should save approximately (3 seconds - 50ms) * 5 units = ~14.75 seconds from cache
    assert!(analysis.cache_savings > Duration::from_secs(14));
    assert!(analysis.cache_savings < Duration::from_secs(16));
    
    // Check breakdown
    let cache_breakdown = analysis.savings_breakdown.get("cache").unwrap();
    assert_eq!(cache_breakdown.units_affected, 5);
    assert!(cache_breakdown.confidence_level > 0.0);
    
    Ok(())
}

#[test]
fn test_incremental_savings_calculation() -> Result<()> {
    let config = TimeSavingsConfig {
        baseline_compile_time_per_unit: Duration::from_secs(3),
        incremental_analysis_time: Duration::from_millis(100),
        ..Default::default()
    };
    let mut calculator = TimeSavingsCalculator::new(config);
    
    let context = create_sample_timing_context();
    
    let analysis = calculator.calculate_time_savings(
        &context,
        10, // units compiled
        0,  // units from cache
        3,  // units from incremental
        1.0, // parallel efficiency
    )?;
    
    // Should save approximately (3 seconds - 100ms) * 3 units = ~8.7 seconds from incremental
    assert!(analysis.incremental_savings > Duration::from_secs(8));
    assert!(analysis.incremental_savings < Duration::from_secs(9));
    
    // Check breakdown
    let incremental_breakdown = analysis.savings_breakdown.get("incremental").unwrap();
    assert_eq!(incremental_breakdown.units_affected, 3);
    
    Ok(())
}

#[test]
fn test_parallel_savings_calculation() -> Result<()> {
    let config = TimeSavingsConfig {
        baseline_compile_time_per_unit: Duration::from_secs(3),
        parallel_scheduling_overhead: Duration::from_millis(200),
        ..Default::default()
    };
    let mut calculator = TimeSavingsCalculator::new(config);
    
    let mut context = create_sample_timing_context();
    context.parallel_metrics = ParallelMetrics {
        worker_threads: 4,
        thread_utilizations: vec![0.8, 0.9, 0.7, 0.8],
        work_stealing_events: 10,
        synchronization_overhead: Duration::from_millis(100),
    };
    
    let analysis = calculator.calculate_time_savings(
        &context,
        8,   // units compiled
        0,   // units from cache
        0,   // units from incremental
        2.5, // good parallel efficiency (2.5x speedup)
    )?;
    
    // With 2.5x parallel efficiency, should have significant savings
    assert!(analysis.parallel_savings > Duration::from_secs(10));
    
    // Check parallel breakdown
    let parallel_breakdown = analysis.savings_breakdown.get("parallel").unwrap();
    assert_eq!(parallel_breakdown.units_affected, 8);
    assert!(parallel_breakdown.confidence_level > 0.8); // High confidence with good efficiency
    
    Ok(())
}

#[test]
fn test_combined_optimizations() -> Result<()> {
    let config = TimeSavingsConfig::default();
    let mut calculator = TimeSavingsCalculator::new(config);
    
    let mut context = create_sample_timing_context();
    context.cache_timings = CacheTimings {
        total_lookup_time: Duration::from_millis(200),
        total_store_time: Duration::from_millis(50),
        cache_hits: 4,
        cache_misses: 1,
    };
    context.parallel_metrics = ParallelMetrics {
        worker_threads: 4,
        thread_utilizations: vec![0.8, 0.9, 0.7, 0.8],
        work_stealing_events: 5,
        synchronization_overhead: Duration::from_millis(50),
    };
    
    let analysis = calculator.calculate_time_savings(
        &context,
        12, // units compiled
        4,  // units from cache
        3,  // units from incremental
        2.0, // parallel efficiency
    )?;
    
    // Total savings should be sum of all optimization types
    let expected_total = analysis.cache_savings + 
                        analysis.incremental_savings + 
                        analysis.parallel_savings + 
                        analysis.llvm_optimization_savings +
                        analysis.dependency_optimization_savings;
    
    assert_eq!(analysis.total_time_saved, expected_total);
    
    // Should have efficiency improvement
    assert!(analysis.efficiency_improvement_percent > 0.0);
    
    // Should have throughput improvement
    assert!(analysis.throughput_improvement > 0.0);
    
    Ok(())
}

#[test]
fn test_optimization_timing_recording() {
    let config = TimeSavingsConfig::default();
    let calculator = TimeSavingsCalculator::new(config);
    
    let mut context = create_sample_timing_context();
    
    // Record optimization pass timings
    calculator.record_optimization_timing(&mut context, "dead_code_elimination", Duration::from_millis(500));
    calculator.record_optimization_timing(&mut context, "constant_folding", Duration::from_millis(300));
    calculator.record_optimization_timing(&mut context, "loop_optimization", Duration::from_millis(800));
    
    assert_eq!(context.optimization_timings.len(), 3);
    assert_eq!(context.optimization_timings.get("dead_code_elimination"), Some(&Duration::from_millis(500)));
    assert_eq!(context.optimization_timings.get("constant_folding"), Some(&Duration::from_millis(300)));
    assert_eq!(context.optimization_timings.get("loop_optimization"), Some(&Duration::from_millis(800)));
}

#[test]
fn test_unit_timing_recording() {
    let config = TimeSavingsConfig::default();
    let calculator = TimeSavingsCalculator::new(config);
    
    let mut context = create_sample_timing_context();
    let unit = CompilationUnit::new("test_module".to_string());
    
    let timing = UnitTiming {
        name: "test_module".to_string(),
        start_time: Instant::now(),
        end_time: Some(Instant::now()),
        from_cache: false,
        from_incremental: true,
        optimization_passes: HashMap::new(),
    };
    
    calculator.record_unit_timing(&mut context, &unit, timing);
    
    assert!(context.unit_timings.contains_key("test_module"));
    let recorded_timing = context.unit_timings.get("test_module").unwrap();
    assert!(recorded_timing.from_incremental);
    assert!(!recorded_timing.from_cache);
}

#[test]
fn test_confidence_level_calculations() -> Result<()> {
    let config = TimeSavingsConfig::default();
    let mut calculator = TimeSavingsCalculator::new(config);
    
    // Test high confidence scenario (many cache hits)
    let mut context = create_sample_timing_context();
    context.cache_timings = CacheTimings {
        total_lookup_time: Duration::from_millis(1000),
        total_store_time: Duration::from_millis(200),
        cache_hits: 20,
        cache_misses: 2,
    };
    
    let analysis = calculator.calculate_time_savings(&context, 22, 20, 0, 1.0)?;
    
    let cache_breakdown = analysis.savings_breakdown.get("cache").unwrap();
    assert!(cache_breakdown.confidence_level > 0.8); // High confidence with many samples and good hit rate
    
    // Test low confidence scenario (few cache operations)
    context.cache_timings = CacheTimings {
        total_lookup_time: Duration::from_millis(100),
        total_store_time: Duration::from_millis(50),
        cache_hits: 1,
        cache_misses: 1,
    };
    
    let analysis = calculator.calculate_time_savings(&context, 2, 1, 0, 1.0)?;
    
    let cache_breakdown = analysis.savings_breakdown.get("cache").unwrap();
    assert!(cache_breakdown.confidence_level < 0.7); // Lower confidence with few samples
    
    Ok(())
}

#[test]
fn test_trend_analysis() -> Result<()> {
    let config = TimeSavingsConfig::default();
    let mut calculator = TimeSavingsCalculator::new(config);
    
    // No trend analysis with insufficient data
    assert!(calculator.get_trend_analysis().is_none());
    
    // Generate several measurements
    for i in 0..5 {
        let context = create_sample_timing_context();
        let _analysis = calculator.calculate_time_savings(
            &context,
            10 + i,  // varying unit counts
            i,       // varying cache hits
            0,       // no incremental
            1.5,     // consistent parallel efficiency
        )?;
    }
    
    // Should now have trend analysis
    let trend = calculator.get_trend_analysis();
    assert!(trend.is_some());
    
    let trend = trend.unwrap();
    assert!(trend.measurement_count >= 3);
    assert!(trend.average_parallel_efficiency > 1.0);
    
    Ok(())
}

#[test]
fn test_metadata_generation() -> Result<()> {
    let config = TimeSavingsConfig::default();
    let mut calculator = TimeSavingsCalculator::new(config);
    
    let mut context = create_sample_timing_context();
    context.cache_timings = CacheTimings {
        total_lookup_time: Duration::from_millis(400),
        total_store_time: Duration::from_millis(100),
        cache_hits: 8,
        cache_misses: 2,
    };
    context.parallel_metrics = ParallelMetrics {
        worker_threads: 4,
        thread_utilizations: vec![0.75, 0.85, 0.80, 0.90],
        work_stealing_events: 15,
        synchronization_overhead: Duration::from_millis(75),
    };
    
    let analysis = calculator.calculate_time_savings(&context, 10, 8, 0, 2.2)?;
    
    // Check cache metadata
    let cache_breakdown = analysis.savings_breakdown.get("cache").unwrap();
    assert!(cache_breakdown.metadata.contains_key("cache_hits"));
    assert!(cache_breakdown.metadata.contains_key("hit_rate_percent"));
    assert_eq!(cache_breakdown.metadata.get("cache_hits").unwrap(), "8");
    
    // Check parallel metadata
    let parallel_breakdown = analysis.savings_breakdown.get("parallel").unwrap();
    assert!(parallel_breakdown.metadata.contains_key("worker_threads"));
    assert!(parallel_breakdown.metadata.contains_key("parallel_efficiency"));
    assert_eq!(parallel_breakdown.metadata.get("worker_threads").unwrap(), "4");
    
    Ok(())
}

#[test]
fn test_zero_optimizations() -> Result<()> {
    let config = TimeSavingsConfig::default();
    let mut calculator = TimeSavingsCalculator::new(config);
    
    let context = create_sample_timing_context();
    
    // No optimizations enabled
    let analysis = calculator.calculate_time_savings(&context, 5, 0, 0, 1.0)?;
    
    assert_eq!(analysis.cache_savings, Duration::from_secs(0));
    assert_eq!(analysis.incremental_savings, Duration::from_secs(0));
    assert_eq!(analysis.parallel_savings, Duration::from_secs(0));
    assert_eq!(analysis.total_time_saved, analysis.llvm_optimization_savings + analysis.dependency_optimization_savings);
    
    Ok(())
}

// Helper functions

fn create_sample_timing_context() -> CompilationTimingContext {
    CompilationTimingContext {
        start_time: Instant::now(),
        unit_timings: HashMap::new(),
        optimization_timings: HashMap::new(),
        cache_timings: CacheTimings::default(),
        parallel_metrics: ParallelMetrics::default(),
    }
}
