/// Comprehensive Integration Tests for Performance Optimization System
/// 
/// Tests the complete integration of all optimization features including
/// parallel compilation, incremental compilation, LLVM optimization, and adaptive optimization.

use cursed::optimization::{
    performance_integration::{
        PerformanceIntegrationSystem, PerformanceIntegrationConfig, PerformanceTargets,
        AdaptiveOptimizer, ProjectCharacteristics, OptimizationRecord,
        RecommendationCategory, ImplementationEffort,
    },
    config::{OptimizationConfig, OptimizationProfile, OptimizationLevel},
};
use cursed::error::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tracing_test::traced_test;

/// Test data for performance optimization integration tests
struct TestProjectData {
    temp_dir: TempDir,
    source_files: Vec<PathBuf>,
    expected_output: PathBuf,
}

impl TestProjectData {
    fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let mut source_files = Vec::new();
        
        // Create test source files with varying complexity
        let simple_file = temp_dir.path().join("simple.csd");
        std::fs::write(&simple_file, r#"
            facts main() {
                sus x = 42;
                sus y = 24;
                println(x + y);
            }
        "#)?;
        source_files.push(simple_file);
        
        let complex_file = temp_dir.path().join("complex.csd");
        std::fs::write(&complex_file, r#"
            squad ComplexCalculation {
                sus data: Vec<f64>;
                
                facts new(size: usize) -> ComplexCalculation {
                    facts result = ComplexCalculation {
                        data: Vec::with_capacity(size);
                    };
                    
                    lowkey (sus i = 0; i < size; i++) {
                        result.data.push(i as f64 * 3.14159);
                    }
                    
                    result
                }
                
                facts compute_intensive(&self) -> f64 {
                    sus sum = 0.0;
                    lowkey (sus value in &self.data) {
                        sum += value * value / (value + 1.0);
                    }
                    sum
                }
            }
            
            facts main() {
                sus calc = ComplexCalculation::new(10000);
                sus result = calc.compute_intensive();
                println("Result: {}", result);
            }
        "#)?;
        source_files.push(complex_file);
        
        let generics_file = temp_dir.path().join("generics.csd");
        std::fs::write(&generics_file, r#"
            squad Container<T> {
                sus items: Vec<T>;
                
                facts new() -> Container<T> {
                    Container { items: Vec::new() }
                }
                
                facts add(&mut self, item: T) {
                    self.items.push(item);
                }
                
                facts process<F>(&self, func: F) -> Vec<T>
                where F: Fn(&T) -> T {
                    self.items.iter().map(func).collect()
                }
            }
            
            facts main() {
                sus mut container = Container::<i32>::new();
                container.add(1);
                container.add(2);
                container.add(3);
                
                facts doubled = container.process(|x| x * 2);
                lowkey (sus item in doubled) {
                    println("{}", item);
                }
            }
        "#)?;
        source_files.push(generics_file);
        
        let expected_output = temp_dir.path().join("output.exe");
        
        Ok(Self {
            temp_dir,
            source_files,
            expected_output,
        })
    }
}

#[traced_test]
#[test]
fn test_performance_integration_system_creation() -> Result<()> {
    let config = PerformanceIntegrationConfig::default();
    let optimization_config = OptimizationConfig::default();
    
    let system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    // Verify system was created successfully
    // Note: We can't directly access private fields, so we test through public interface
    let stats = system.get_performance_statistics()?;
    assert!(stats.optimization_history.total_optimizations == 0); // New system should have no history
    
    Ok(())
}

#[traced_test]
#[test]
fn test_performance_integration_config_defaults() {
    let config = PerformanceIntegrationConfig::default();
    
    assert!(config.enable_adaptive_optimization);
    assert!(config.enable_performance_monitoring);
    assert!(!config.enable_automatic_reporting);
    assert_eq!(config.monitoring_interval_ms, 1000);
    assert_eq!(config.optimization_threshold_seconds, 30.0);
    assert_eq!(config.max_parallel_workers, 0); // Auto-detect
    assert!(config.enable_pgo);
    assert!(!config.enable_distributed);
    assert_eq!(config.cache_size_limit_mb, 2048);
    assert!(config.report_output_dir.is_none());
    assert_eq!(config.benchmark_configs.len(), 2); // quick and thorough
    
    // Check performance targets
    let targets = &config.target_improvements;
    assert_eq!(targets.compilation_time_reduction, 30.0);
    assert_eq!(targets.runtime_performance_improvement, 20.0);
    assert_eq!(targets.memory_usage_reduction, 15.0);
    assert_eq!(targets.binary_size_reduction, 10.0);
}

#[traced_test]
#[test]
fn test_optimization_profiles_integration() -> Result<()> {
    let profiles = vec![
        OptimizationProfile::Development,
        OptimizationProfile::Release,
        OptimizationProfile::Debug,
        OptimizationProfile::Size,
        OptimizationProfile::Performance,
    ];
    
    for profile in profiles {
        let config = PerformanceIntegrationConfig::default();
        let optimization_config = profile.to_config();
        
        let system = PerformanceIntegrationSystem::new(config, optimization_config)?;
        
        // Verify each profile creates a valid system
        let stats = system.get_performance_statistics()?;
        assert!(stats.optimization_history.total_optimizations == 0);
    }
    
    Ok(())
}

#[traced_test]
#[test]
fn test_project_optimization_workflow() -> Result<()> {
    let test_data = TestProjectData::new()?;
    let mut config = PerformanceIntegrationConfig::default();
    config.enable_automatic_reporting = true;
    config.report_output_dir = Some(test_data.temp_dir.path().join("reports"));
    
    let optimization_config = OptimizationConfig {
        optimization_level: OptimizationLevel::Default,
        enable_parallel: true,
        parallel_workers: 2,
        enable_incremental: true,
        ..Default::default()
    };
    
    let mut system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    // Run optimization on test project
    let results = system.optimize_project(&test_data.source_files, &test_data.expected_output)?;
    
    // Verify optimization results
    assert!(results.compilation_time > Duration::from_millis(0));
    assert!(results.parallel_efficiency >= 0.0 && results.parallel_efficiency <= 1.0);
    assert!(results.cache_hit_rate >= 0.0 && results.cache_hit_rate <= 1.0);
    assert!(!results.recommendations.is_empty() || results.parallel_efficiency > 0.8);
    assert!(!results.checkpoints.is_empty());
    
    // Verify performance improvements are reported
    let improvements = &results.performance_improvements;
    assert!(improvements.compilation_time_saved >= Duration::from_millis(0));
    assert!(improvements.binary_size_reduction >= 0.0);
    assert!(improvements.runtime_improvement_estimate >= 0.0);
    assert!(improvements.memory_usage_reduction >= 0.0);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_adaptive_optimization_profile_selection() -> Result<()> {
    let mut optimizer = AdaptiveOptimizer::new();
    
    // Test small project characteristics
    let small_project = ProjectCharacteristics {
        total_source_files: 5,
        total_lines_of_code: 500,
        average_file_size: 100,
        dependency_count: 2,
        has_heavy_computation: false,
        has_many_generics: false,
        typical_build_time_seconds: 2.0,
    };
    
    let profile = optimizer.select_optimal_profile(&small_project);
    assert_eq!(profile, OptimizationProfile::Development);
    
    // Test large project with heavy computation
    let large_project = ProjectCharacteristics {
        total_source_files: 200,
        total_lines_of_code: 150000,
        average_file_size: 750,
        dependency_count: 50,
        has_heavy_computation: true,
        has_many_generics: true,
        typical_build_time_seconds: 120.0,
    };
    
    let profile = optimizer.select_optimal_profile(&large_project);
    assert_eq!(profile, OptimizationProfile::Performance);
    
    // Test very large project
    let very_large_project = ProjectCharacteristics {
        total_source_files: 600,
        total_lines_of_code: 80000,
        average_file_size: 133,
        dependency_count: 30,
        has_heavy_computation: false,
        has_many_generics: false,
        typical_build_time_seconds: 60.0,
    };
    
    let profile = optimizer.select_optimal_profile(&very_large_project);
    assert_eq!(profile, OptimizationProfile::Release);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_optimization_history_tracking() -> Result<()> {
    let mut optimizer = AdaptiveOptimizer::new();
    
    // Record several optimization attempts
    let records = vec![
        OptimizationRecord {
            timestamp: Instant::now(),
            profile_used: OptimizationProfile::Development,
            compilation_time: Duration::from_secs(5),
            binary_size: 1024 * 1024, // 1MB
            performance_score: 75.0,
            success: true,
        },
        OptimizationRecord {
            timestamp: Instant::now(),
            profile_used: OptimizationProfile::Release,
            compilation_time: Duration::from_secs(15),
            binary_size: 800 * 1024, // 800KB
            performance_score: 85.0,
            success: true,
        },
        OptimizationRecord {
            timestamp: Instant::now(),
            profile_used: OptimizationProfile::Performance,
            compilation_time: Duration::from_secs(30),
            binary_size: 750 * 1024, // 750KB
            performance_score: 95.0,
            success: true,
        },
    ];
    
    for record in records {
        optimizer.record_optimization(record);
    }
    
    let history = optimizer.get_history_summary();
    assert_eq!(history.total_optimizations, 3);
    assert!(history.average_compilation_time > Duration::from_secs(0));
    assert_eq!(history.best_performance_score, 95.0);
    assert_eq!(history.most_effective_profile, OptimizationProfile::Performance);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_performance_benchmarking_integration() -> Result<()> {
    let config = PerformanceIntegrationConfig::default();
    let optimization_config = OptimizationConfig::default();
    let system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    // Test quick benchmark
    let quick_results = system.run_performance_benchmarks("quick")?;
    assert!(quick_results.compilation_times.len() >= 3); // At least 3 iterations
    assert!(quick_results.average_time > Duration::from_millis(0));
    
    // Test thorough benchmark
    let thorough_results = system.run_performance_benchmarks("thorough")?;
    assert!(thorough_results.compilation_times.len() >= 10); // At least 10 iterations
    assert!(thorough_results.average_time > Duration::from_millis(0));
    
    // Verify thorough benchmark takes longer on average (due to more complex test data)
    assert!(thorough_results.average_time >= quick_results.average_time);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_configuration_update_and_adaptation() -> Result<()> {
    let initial_config = PerformanceIntegrationConfig {
        cache_size_limit_mb: 1024, // 1GB
        monitoring_interval_ms: 500,
        enable_adaptive_optimization: false,
        ..Default::default()
    };
    
    let optimization_config = OptimizationConfig::default();
    let mut system = PerformanceIntegrationSystem::new(initial_config, optimization_config)?;
    
    // Update configuration
    let new_config = PerformanceIntegrationConfig {
        cache_size_limit_mb: 4096, // 4GB
        monitoring_interval_ms: 2000,
        enable_adaptive_optimization: true,
        enable_automatic_reporting: true,
        ..Default::default()
    };
    
    system.update_configuration(new_config)?;
    
    // Verify configuration was updated (we can't directly check private fields,
    // so we verify through behavior)
    let stats = system.get_performance_statistics()?;
    
    // The system should still function after configuration update
    assert!(stats.optimization_history.total_optimizations == 0);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_performance_statistics_collection() -> Result<()> {
    let config = PerformanceIntegrationConfig {
        enable_performance_monitoring: true,
        monitoring_interval_ms: 100,
        ..Default::default()
    };
    
    let optimization_config = OptimizationConfig::default();
    let system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    // Get initial statistics
    let stats = system.get_performance_statistics()?;
    
    // Verify statistics structure
    assert!(stats.system.cpu_cores > 0);
    assert!(stats.system.total_memory_gb > 0.0);
    assert!(stats.resources.cpu_usage_percent >= 0.0);
    assert!(stats.resources.cpu_usage_percent <= 100.0);
    assert!(stats.resources.memory_usage_mb >= 0.0);
    assert!(stats.cache.total_requests >= 0);
    assert!(stats.cache.cache_hits >= 0);
    assert!(stats.cache.cache_misses >= 0);
    assert_eq!(stats.optimization_history.total_optimizations, 0);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_optimization_recommendations_generation() -> Result<()> {
    let test_data = TestProjectData::new()?;
    let config = PerformanceIntegrationConfig::default();
    let optimization_config = OptimizationConfig::default();
    
    let mut system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    // Run optimization to generate recommendations
    let results = system.optimize_project(&test_data.source_files, &test_data.expected_output)?;
    
    // Analyze recommendations
    for recommendation in &results.recommendations {
        // Verify recommendation structure
        assert!(!recommendation.description.is_empty());
        assert!(recommendation.expected_improvement >= 0.0);
        assert!(recommendation.expected_improvement <= 100.0);
        
        // Verify recommendation categories are valid
        match recommendation.category {
            RecommendationCategory::CompilationSpeed |
            RecommendationCategory::RuntimePerformance |
            RecommendationCategory::MemoryUsage |
            RecommendationCategory::BinarySize |
            RecommendationCategory::CacheUtilization |
            RecommendationCategory::ParallelizationEfficiency => {
                // Valid categories
            }
        }
        
        // Verify implementation effort levels
        match recommendation.implementation_effort {
            ImplementationEffort::Low |
            ImplementationEffort::Medium |
            ImplementationEffort::High => {
                // Valid effort levels
            }
        }
    }
    
    Ok(())
}

#[traced_test]
#[test]
fn test_performance_monitoring_checkpoints() -> Result<()> {
    let test_data = TestProjectData::new()?;
    let config = PerformanceIntegrationConfig {
        enable_performance_monitoring: true,
        ..Default::default()
    };
    
    let optimization_config = OptimizationConfig::default();
    let mut system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    // Run optimization with monitoring
    let results = system.optimize_project(&test_data.source_files, &test_data.expected_output)?;
    
    // Verify checkpoints were created
    assert!(!results.checkpoints.is_empty());
    
    // Verify checkpoint progression
    let mut last_timestamp = None;
    for checkpoint in &results.checkpoints {
        assert!(!checkpoint.name.is_empty());
        assert!(checkpoint.memory_usage_mb >= 0.0);
        assert!(checkpoint.cpu_usage_percent >= 0.0);
        assert!(checkpoint.cpu_usage_percent <= 100.0);
        
        // Verify timestamps are in order
        if let Some(last) = last_timestamp {
            assert!(checkpoint.timestamp >= last);
        }
        last_timestamp = Some(checkpoint.timestamp);
    }
    
    // Verify expected checkpoints exist
    let checkpoint_names: Vec<&str> = results.checkpoints.iter()
        .map(|c| c.name.as_str())
        .collect();
    
    assert!(checkpoint_names.contains(&"configuration_complete"));
    assert!(checkpoint_names.contains(&"compilation_complete"));
    
    Ok(())
}

#[traced_test]
#[test]
fn test_performance_target_achievement_analysis() -> Result<()> {
    let targets = PerformanceTargets {
        compilation_time_reduction: 40.0,
        runtime_performance_improvement: 30.0,
        memory_usage_reduction: 20.0,
        binary_size_reduction: 15.0,
    };
    
    let config = PerformanceIntegrationConfig {
        target_improvements: targets,
        ..Default::default()
    };
    
    let optimization_config = OptimizationConfig {
        optimization_level: OptimizationLevel::Aggressive,
        profile_guided: true,
        ..Default::default()
    };
    
    let test_data = TestProjectData::new()?;
    let mut system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    let results = system.optimize_project(&test_data.source_files, &test_data.expected_output)?;
    
    // Analyze if performance targets influenced recommendations
    let performance_recommendations: Vec<_> = results.recommendations.iter()
        .filter(|r| matches!(r.category, RecommendationCategory::RuntimePerformance))
        .collect();
    
    // Should have performance recommendations if targets are high
    if results.performance_improvements.runtime_improvement_estimate < 30.0 {
        assert!(!performance_recommendations.is_empty());
    }
    
    Ok(())
}

#[traced_test]
#[test]
fn test_large_project_optimization_scalability() -> Result<()> {
    // Create a larger test project
    let temp_dir = TempDir::new()?;
    let mut source_files = Vec::new();
    
    // Generate multiple source files to simulate a larger project
    for i in 0..50 {
        let file_path = temp_dir.path().join(format!("module_{}.csd", i));
        std::fs::write(&file_path, format!(r#"
            squad Module{} {{
                sus value: i32;
                
                facts new(val: i32) -> Module{} {{
                    Module{} {{ value: val }}
                }}
                
                facts compute(&self) -> i32 {{
                    self.value * {} + {}
                }}
            }}
            
            facts process_module_{}() -> i32 {{
                sus module = Module{}::new({});
                module.compute()
            }}
        "#, i, i, i, i, i, i, i, i))?;
        
        source_files.push(file_path);
    }
    
    let config = PerformanceIntegrationConfig {
        enable_adaptive_optimization: true,
        enable_performance_monitoring: true,
        max_parallel_workers: 4,
        ..Default::default()
    };
    
    let optimization_config = OptimizationConfig {
        enable_parallel: true,
        parallel_workers: 4,
        optimization_level: OptimizationLevel::Default,
        ..Default::default()
    };
    
    let mut system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    let output_path = temp_dir.path().join("large_project_output.exe");
    
    let start_time = Instant::now();
    let results = system.optimize_project(&source_files, &output_path)?;
    let total_time = start_time.elapsed();
    
    // Verify scalability metrics
    assert!(results.parallel_efficiency > 0.0); // Should have some parallel efficiency
    assert!(total_time < Duration::from_secs(120)); // Should complete within reasonable time
    assert!(!results.checkpoints.is_empty());
    
    // For larger projects, should recommend parallel optimizations
    let has_parallelization_recommendations = results.recommendations.iter()
        .any(|r| matches!(r.category, RecommendationCategory::ParallelizationEfficiency));
    
    // Large projects should either have good parallel efficiency or recommendations to improve it
    assert!(results.parallel_efficiency > 0.6 || has_parallelization_recommendations);
    
    Ok(())
}

// Integration test helpers

#[allow(dead_code)]
fn create_benchmark_config(name: &str) -> cursed::optimization::benchmarking::BenchmarkConfig {
    cursed::optimization::benchmarking::BenchmarkConfig {
        test_type: cursed::optimization::benchmarking::BenchmarkType::CompilationSpeed,
        iterations: 5,
        warm_up_iterations: 1,
        timeout_seconds: 30,
        complexity_level: cursed::optimization::benchmarking::ComplexityLevel::Medium,
        test_data: cursed::optimization::benchmarking::BenchmarkTestData::Medium,
    }
}

#[allow(dead_code)]
fn verify_optimization_effectiveness(
    results: &cursed::optimization::performance_integration::IntegratedOptimizationResults,
    min_efficiency: f64,
) -> bool {
    results.parallel_efficiency >= min_efficiency &&
    results.cache_hit_rate >= 0.5 &&
    results.performance_improvements.compilation_time_saved > Duration::from_millis(100)
}
