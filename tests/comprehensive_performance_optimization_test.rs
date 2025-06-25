/// Comprehensive Performance Optimization System Tests
/// 
/// This test suite validates the complete performance optimization system including:
/// - Advanced LLVM optimization passes effectiveness
/// - Profile-guided optimization workflow
/// - Build performance optimization features
/// - Runtime performance monitoring accuracy
/// - Regression detection and benchmarking

use cursed::optimization::comprehensive_performance_system::*;
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use cursed::optimization::pgo::{PgoConfig, InstrumentationMode, CollectionMode, OptimizationStrategy};
use cursed::error::Result;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn test_comprehensive_performance_system_initialization() -> Result<()> {
    let context = inkwell::context::Context::create();
    let config = PerformanceConfig::default();
    
    let performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Verify system components are properly initialized
    let statistics = performance_system.get_performance_statistics();
    assert_eq!(statistics.total_compilations, 0);
    assert_eq!(statistics.cache_hit_rate, 0.0);
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_llvm_optimization_passes_effectiveness() -> Result<()> {
    let context = inkwell::context::Context::create();
    
    // Test different optimization levels
    for opt_level in [OptimizationLevel::None, OptimizationLevel::Less, OptimizationLevel::Default, OptimizationLevel::Aggressive] {
        let mut config = PerformanceConfig::default();
        config.optimization_level = opt_level.clone();
        config.enable_function_inlining = true;
        config.enable_dead_code_elimination = true;
        config.enable_constant_propagation = true;
        config.enable_loop_optimization = true;
        
        let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
        
        // Create a simple test module
        let module = context.create_module("test_module");
        let source_files = vec![PathBuf::from("test.csd")];
        
        // Optimize the module
        let results = performance_system.optimize_module(&module, &source_files).await?;
        
        // Verify optimization results
        assert!(results.compilation_time > Duration::from_millis(0));
        assert!(results.total_optimization_time >= results.compilation_time);
        
        if let Some(llvm_results) = &results.llvm_optimization_results {
            // Higher optimization levels should apply more passes
            match opt_level {
                OptimizationLevel::None => {
                    assert!(llvm_results.passes_applied >= 1);
                }
                OptimizationLevel::Less => {
                    assert!(llvm_results.passes_applied >= 3);
                }
                OptimizationLevel::Default => {
                    assert!(llvm_results.passes_applied >= 5);
                }
                OptimizationLevel::Aggressive => {
                    assert!(llvm_results.passes_applied >= 7);
                }
                _ => {}
            }
            
            // Verify optimization metrics are reasonable
            assert!(llvm_results.estimated_performance_improvement >= 0.0);
            assert!(llvm_results.estimated_performance_improvement <= 100.0);
            assert!(llvm_results.code_size_reduction >= -10.0); // Allow for code growth in some cases
            assert!(llvm_results.code_size_reduction <= 50.0);  // Reasonable upper bound
        }
        
        println!("Optimization level {:?}: {:?}", opt_level, results.llvm_optimization_results);
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_profile_guided_optimization_workflow() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    
    // Enable PGO with different configurations
    config.enable_pgo = true;
    config.pgo_config = PgoConfig {
        enabled: true,
        profile_data_dir: PathBuf::from("test_pgo_profiles"),
        instrumentation_mode: InstrumentationMode::Frontend,
        collection_mode: CollectionMode::CountersAndSampling,
        optimization_strategy: OptimizationStrategy::Speed,
        hot_function_threshold: 0.1,
        cold_function_threshold: 0.01,
        min_execution_count: 100,
        enable_indirect_call_promotion: true,
        enable_value_profiling: true,
        enable_control_flow_profiling: true,
        ..PgoConfig::default()
    };
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Create test module
    let module = context.create_module("pgo_test_module");
    let source_files = vec![PathBuf::from("pgo_test.csd")];
    
    // First compilation with PGO instrumentation
    let results = performance_system.optimize_module(&module, &source_files).await?;
    
    // Verify PGO results
    if let Some(pgo_results) = &results.pgo_results {
        assert!(!pgo_results.session_id.is_empty());
        assert!(pgo_results.instrumentation_overhead >= 0.0);
        assert!(pgo_results.instrumentation_overhead <= 50.0); // Reasonable overhead limit
        
        // Verify optimization recommendations
        assert!(!pgo_results.recommendations.hot_functions.is_empty() || 
                !pgo_results.recommendations.cold_functions.is_empty() ||
                !pgo_results.recommendations.optimization_opportunities.is_empty());
        
        println!("PGO session: {}", pgo_results.session_id);
        println!("Instrumentation overhead: {:.2}%", pgo_results.instrumentation_overhead);
        println!("Optimization opportunities: {}", pgo_results.recommendations.optimization_opportunities.len());
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_build_performance_optimization() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    
    // Enable build performance features
    config.enable_incremental_compilation = true;
    config.enable_parallel_compilation = true;
    config.max_parallel_jobs = 4;
    config.enable_compilation_caching = true;
    config.cache_directory = PathBuf::from("test_cache");
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Create test modules
    let module = context.create_module("build_test_module");
    let source_files = vec![
        PathBuf::from("file1.csd"),
        PathBuf::from("file2.csd"),
        PathBuf::from("file3.csd"),
    ];
    
    // First compilation (cache miss)
    let start_time = Instant::now();
    let results1 = performance_system.optimize_module(&module, &source_files).await?;
    let first_compilation_time = start_time.elapsed();
    
    assert!(!results1.cache_hit);
    assert!(results1.dependency_analysis.is_some());
    
    if let Some(dep_analysis) = &results1.dependency_analysis {
        assert_eq!(dep_analysis.files_analyzed, source_files.len() as u32);
        assert!(dep_analysis.incremental_compilation_possible);
        assert!(dep_analysis.dependencies_found > 0);
    }
    
    // Second compilation (should hit cache)
    let start_time = Instant::now();
    let results2 = performance_system.optimize_module(&module, &source_files).await?;
    let second_compilation_time = start_time.elapsed();
    
    // Cache hit should be much faster
    if results2.cache_hit {
        assert!(second_compilation_time < first_compilation_time / 2);
        println!("Cache hit! Second compilation: {:?} vs First: {:?}", 
                second_compilation_time, first_compilation_time);
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_runtime_performance_monitoring() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    
    // Enable performance monitoring
    config.enable_performance_monitoring = true;
    config.collect_memory_usage = true;
    config.collect_compilation_time = true;
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Perform several compilations to generate metrics
    let module = context.create_module("monitoring_test_module");
    
    for i in 0..5 {
        let source_files = vec![PathBuf::from(format!("test_file_{}.csd", i))];
        let results = performance_system.optimize_module(&module, &source_files).await?;
        
        // Verify performance metrics are collected
        if let Some(metrics) = &results.performance_metrics {
            assert!(metrics.memory_usage > 0);
            assert!(metrics.cpu_usage_percentage >= 0.0);
            assert!(metrics.cpu_usage_percentage <= 100.0);
            assert!(metrics.compilation_time > Duration::from_millis(0));
            assert!(metrics.cache_operations >= 0);
            assert!(metrics.parallel_jobs_used > 0);
        }
    }
    
    // Get performance statistics
    let statistics = performance_system.get_performance_statistics();
    assert!(statistics.total_compilations >= 5);
    assert!(statistics.average_compilation_time > Duration::from_millis(0));
    assert!(statistics.total_optimization_time > Duration::from_millis(0));
    
    // Test optimization effectiveness
    assert!(statistics.optimization_effectiveness >= 0.0);
    assert!(statistics.optimization_effectiveness <= 1.0);
    
    println!("Total compilations: {}", statistics.total_compilations);
    println!("Average compilation time: {:?}", statistics.average_compilation_time);
    println!("Cache hit rate: {:.2}%", statistics.cache_hit_rate * 100.0);
    println!("Optimization effectiveness: {:.2}%", statistics.optimization_effectiveness * 100.0);
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_regression_detection() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    
    // Enable regression detection
    config.enable_regression_detection = true;
    config.regression_threshold_percentage = 10.0; // 10% threshold
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Create test module
    let module = context.create_module("regression_test_module");
    let source_files = vec![PathBuf::from("regression_test.csd")];
    
    // Perform compilation
    let results = performance_system.optimize_module(&module, &source_files).await?;
    
    // Test regression analysis
    if let Some(regression_analysis) = &results.regression_analysis {
        // Should not have regressions on first run (no baseline)
        assert!(!regression_analysis.has_regressions);
        assert!(regression_analysis.regressions.is_empty());
        
        // Overall performance change should be reasonable
        assert!(regression_analysis.overall_performance_change >= -10.0);
        assert!(regression_analysis.overall_performance_change <= 10.0);
        
        println!("Regression analysis: {} regressions, {} improvements", 
                regression_analysis.regressions.len(), 
                regression_analysis.improvements.len());
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_comprehensive_benchmarking() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    
    // Enable benchmarking
    config.enable_benchmarking = true;
    config.benchmark_iterations = 5;
    config.benchmark_warmup_iterations = 2;
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Run comprehensive benchmarks
    let benchmark_results = performance_system.run_benchmarks("comprehensive_test").await?;
    
    // Verify benchmark results
    assert_eq!(benchmark_results.benchmark_name, "comprehensive_test");
    assert!(benchmark_results.total_duration > Duration::from_millis(0));
    
    // Check compilation benchmarks
    let comp_benchmarks = &benchmark_results.compilation_benchmarks;
    assert!(comp_benchmarks.total_time > Duration::from_millis(0));
    assert!(comp_benchmarks.files_compiled > 0);
    assert!(comp_benchmarks.cache_utilization >= 0.0);
    assert!(comp_benchmarks.cache_utilization <= 1.0);
    
    // Check runtime benchmarks
    let runtime_benchmarks = &benchmark_results.runtime_benchmarks;
    assert!(runtime_benchmarks.execution_time > Duration::from_millis(0));
    assert!(runtime_benchmarks.memory_usage > 0);
    assert!(runtime_benchmarks.cpu_usage >= 0.0);
    assert!(runtime_benchmarks.cpu_usage <= 100.0);
    assert!(runtime_benchmarks.throughput > 0.0);
    
    // Check optimization benchmarks
    let opt_benchmarks = &benchmark_results.optimization_benchmarks;
    assert!(opt_benchmarks.passes_executed > 0);
    assert!(opt_benchmarks.optimization_effectiveness >= 0.0);
    assert!(opt_benchmarks.optimization_effectiveness <= 1.0);
    
    // Check system metrics
    let system_metrics = &benchmark_results.system_metrics;
    assert!(system_metrics.total_memory > 0);
    assert!(system_metrics.cpu_cores > 0);
    assert!(system_metrics.cpu_usage >= 0.0);
    assert!(system_metrics.cpu_usage <= 100.0);
    
    println!("Benchmark completed in {:?}", benchmark_results.total_duration);
    println!("Compilation: {} files in {:?}", 
            comp_benchmarks.files_compiled, comp_benchmarks.total_time);
    println!("Runtime throughput: {:.2} ops/sec", runtime_benchmarks.throughput);
    println!("Optimization effectiveness: {:.2}%", 
            opt_benchmarks.optimization_effectiveness * 100.0);
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_optimization_recommendations() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    
    // Configure system to generate recommendations
    config.enable_performance_monitoring = true;
    config.enable_pgo = false; // To trigger PGO recommendation
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Perform some compilations to generate data
    let module = context.create_module("recommendation_test_module");
    for i in 0..3 {
        let source_files = vec![PathBuf::from(format!("rec_test_{}.csd", i))];
        let _results = performance_system.optimize_module(&module, &source_files).await?;
    }
    
    // Generate optimization recommendations
    let recommendations = performance_system.generate_optimization_recommendations();
    
    // Verify recommendations are generated
    assert!(!recommendations.is_empty());
    
    for recommendation in &recommendations {
        // Verify recommendation structure
        assert!(!recommendation.description.is_empty());
        assert!(recommendation.expected_improvement >= 0.0);
        assert!(recommendation.expected_improvement <= 100.0);
        
        // Verify category and priority are set
        match recommendation.category {
            OptimizationCategory::BuildPerformance |
            OptimizationCategory::RuntimePerformance |
            OptimizationCategory::MemoryUsage |
            OptimizationCategory::CodeSize => {
                // Valid categories
            }
        }
        
        match recommendation.priority {
            RecommendationPriority::Low |
            RecommendationPriority::Medium |
            RecommendationPriority::High |
            RecommendationPriority::Critical => {
                // Valid priorities
            }
        }
        
        println!("Recommendation: {:?} - {} ({}% improvement expected)", 
                recommendation.priority, 
                recommendation.description,
                recommendation.expected_improvement);
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_performance_data_export() -> Result<()> {
    let context = inkwell::context::Context::create();
    let config = PerformanceConfig::default();
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Perform some compilations to generate data
    let module = context.create_module("export_test_module");
    for i in 0..2 {
        let source_files = vec![PathBuf::from(format!("export_test_{}.csd", i))];
        let _results = performance_system.optimize_module(&module, &source_files).await?;
    }
    
    // Export performance data
    let export_path = PathBuf::from("test_performance_export.json");
    performance_system.export_performance_data(&export_path)?;
    
    // Verify export file was created
    assert!(export_path.exists());
    
    // Verify export file contains valid JSON
    let export_content = std::fs::read_to_string(&export_path)?;
    let _export_data: serde_json::Value = serde_json::from_str(&export_content)?;
    
    // Clean up
    std::fs::remove_file(&export_path)?;
    
    println!("Performance data successfully exported and validated");
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_parallel_compilation_performance() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    
    // Test with different parallel job counts
    for job_count in [1, 2, 4, 8] {
        config.enable_parallel_compilation = job_count > 1;
        config.max_parallel_jobs = job_count;
        
        let mut performance_system = ComprehensivePerformanceSystem::new(&context, config.clone())?;
        
        // Create multiple source files to benefit from parallelization
        let module = context.create_module("parallel_test_module");
        let source_files: Vec<PathBuf> = (0..10)
            .map(|i| PathBuf::from(format!("parallel_test_{}.csd", i)))
            .collect();
        
        let start_time = Instant::now();
        let results = performance_system.optimize_module(&module, &source_files).await?;
        let compilation_time = start_time.elapsed();
        
        // Verify performance metrics include parallel job usage
        if let Some(metrics) = &results.performance_metrics {
            if job_count > 1 {
                assert!(metrics.parallel_jobs_used > 1);
                assert!(metrics.parallel_jobs_used <= job_count as u32);
            } else {
                assert_eq!(metrics.parallel_jobs_used, 1);
            }
        }
        
        println!("Parallel jobs: {}, Compilation time: {:?}", 
                job_count, compilation_time);
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_optimization_level_comparison() -> Result<()> {
    let context = inkwell::context::Context::create();
    
    let optimization_levels = [
        OptimizationLevel::None,
        OptimizationLevel::Less, 
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Size,
        OptimizationLevel::SizeAggressive,
    ];
    
    let mut level_results = Vec::new();
    
    for opt_level in optimization_levels {
        let mut config = PerformanceConfig::default();
        config.optimization_level = opt_level.clone();
        
        let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
        
        let module = context.create_module("level_comparison_module");
        let source_files = vec![PathBuf::from("level_test.csd")];
        
        let start_time = Instant::now();
        let results = performance_system.optimize_module(&module, &source_files).await?;
        let compilation_time = start_time.elapsed();
        
        level_results.push((opt_level, compilation_time, results));
    }
    
    // Analyze results across optimization levels
    for (i, (level, time, results)) in level_results.iter().enumerate() {
        println!("Level {:?}: Compilation time: {:?}", level, time);
        
        if let Some(llvm_results) = &results.llvm_optimization_results {
            println!("  Passes applied: {}", llvm_results.passes_applied);
            println!("  Performance improvement: {:.2}%", llvm_results.estimated_performance_improvement);
            println!("  Code size reduction: {:.2}%", llvm_results.code_size_reduction);
        }
        
        // Higher optimization levels should generally apply more passes
        if i > 0 {
            let (prev_level, _prev_time, prev_results) = &level_results[i - 1];
            
            if let (Some(curr_llvm), Some(prev_llvm)) = 
                (&results.llvm_optimization_results, &prev_results.llvm_optimization_results) {
                
                // More aggressive levels should generally apply more passes
                // (with some exceptions for size-optimized builds)
                match (prev_level, level) {
                    (OptimizationLevel::None, OptimizationLevel::Less) |
                    (OptimizationLevel::Less, OptimizationLevel::Default) |
                    (OptimizationLevel::Default, OptimizationLevel::Aggressive) => {
                        assert!(curr_llvm.passes_applied >= prev_llvm.passes_applied);
                    }
                    _ => {
                        // Size optimizations might use different pass counts
                    }
                }
            }
        }
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_memory_usage_monitoring() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    config.collect_memory_usage = true;
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Perform multiple compilations to collect memory usage data
    let module = context.create_module("memory_test_module");
    
    for i in 0..10 {
        let source_files = vec![PathBuf::from(format!("memory_test_{}.csd", i))];
        let results = performance_system.optimize_module(&module, &source_files).await?;
        
        if let Some(metrics) = &results.performance_metrics {
            assert!(metrics.memory_usage > 0);
            // Memory usage should be reasonable (less than 2GB for test)
            assert!(metrics.memory_usage < 2_000_000_000);
        }
    }
    
    // Check aggregated memory statistics
    let statistics = performance_system.get_performance_statistics();
    if let Some(memory_stats) = &statistics.memory_usage_stats {
        assert!(memory_stats.samples_count >= 10);
        assert!(memory_stats.average_memory_usage > 0);
        assert!(memory_stats.peak_memory_usage >= memory_stats.average_memory_usage);
        assert!(memory_stats.minimum_memory_usage <= memory_stats.average_memory_usage);
        
        println!("Memory usage stats:");
        println!("  Average: {} bytes", memory_stats.average_memory_usage);
        println!("  Peak: {} bytes", memory_stats.peak_memory_usage);
        println!("  Minimum: {} bytes", memory_stats.minimum_memory_usage);
        println!("  Samples: {}", memory_stats.samples_count);
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_cache_effectiveness() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    config.enable_compilation_caching = true;
    config.cache_directory = PathBuf::from("test_cache_effectiveness");
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    let module = context.create_module("cache_test_module");
    let source_files = vec![PathBuf::from("cache_test.csd")];
    
    // First compilation - should be cache miss
    let results1 = performance_system.optimize_module(&module, &source_files).await?;
    assert!(!results1.cache_hit);
    
    // Second compilation with same inputs - should be cache hit
    let results2 = performance_system.optimize_module(&module, &source_files).await?;
    
    // Third compilation with same inputs - should also be cache hit
    let results3 = performance_system.optimize_module(&module, &source_files).await?;
    
    // Check cache effectiveness
    let statistics = performance_system.get_performance_statistics();
    
    // At least one cache hit should have occurred
    if results2.cache_hit || results3.cache_hit {
        assert!(statistics.cache_hit_rate > 0.0);
        println!("Cache hit rate: {:.2}%", statistics.cache_hit_rate * 100.0);
    }
    
    // Cache hit compilations should be much faster
    if results2.cache_hit {
        assert!(results2.compilation_time < results1.compilation_time / 2);
        println!("Cache hit speedup: {:?} -> {:?}", 
                results1.compilation_time, results2.compilation_time);
    }
    
    Ok(())
}

/// Test optimization pass ordering and dependencies
#[tokio::test]
#[traced_test]
async fn test_optimization_pass_ordering() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    config.optimization_level = OptimizationLevel::Aggressive;
    config.enable_function_inlining = true;
    config.enable_dead_code_elimination = true;
    config.enable_constant_propagation = true;
    config.enable_loop_optimization = true;
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    let module = context.create_module("pass_ordering_test_module");
    let source_files = vec![PathBuf::from("pass_ordering_test.csd")];
    
    let results = performance_system.optimize_module(&module, &source_files).await?;
    
    if let Some(llvm_results) = &results.llvm_optimization_results {
        // Verify that multiple optimization types were applied
        assert!(llvm_results.passes_applied >= 4);
        
        // Should have some measurable optimization effects
        if llvm_results.functions_inlined > 0 {
            println!("Functions inlined: {}", llvm_results.functions_inlined);
        }
        if llvm_results.dead_code_eliminated > 0 {
            println!("Dead code eliminated: {}", llvm_results.dead_code_eliminated);
        }
        if llvm_results.constants_propagated > 0 {
            println!("Constants propagated: {}", llvm_results.constants_propagated);
        }
        if llvm_results.loops_optimized > 0 {
            println!("Loops optimized: {}", llvm_results.loops_optimized);
        }
    }
    
    Ok(())
}

/// Test performance monitoring under load
#[tokio::test]
#[traced_test]
async fn test_performance_monitoring_under_load() -> Result<()> {
    let context = inkwell::context::Context::create();
    let mut config = PerformanceConfig::default();
    config.enable_performance_monitoring = true;
    config.max_parallel_jobs = 4;
    
    let mut performance_system = ComprehensivePerformanceSystem::new(&context, config)?;
    
    // Simulate load by compiling many modules
    let start_time = Instant::now();
    let num_modules = 20;
    
    for i in 0..num_modules {
        let module = context.create_module(&format!("load_test_module_{}", i));
        let source_files = vec![PathBuf::from(format!("load_test_{}.csd", i))];
        
        let results = performance_system.optimize_module(&module, &source_files).await?;
        
        // Verify metrics are still being collected under load
        if let Some(metrics) = &results.performance_metrics {
            assert!(metrics.memory_usage > 0);
            assert!(metrics.compilation_time > Duration::from_millis(0));
        }
    }
    
    let total_time = start_time.elapsed();
    
    // Get final statistics
    let statistics = performance_system.get_performance_statistics();
    assert_eq!(statistics.total_compilations, num_modules);
    
    println!("Compiled {} modules in {:?}", num_modules, total_time);
    println!("Average time per module: {:?}", statistics.average_compilation_time);
    
    // Performance should remain reasonable under load
    assert!(statistics.average_compilation_time < Duration::from_secs(5));
    
    Ok(())
}
