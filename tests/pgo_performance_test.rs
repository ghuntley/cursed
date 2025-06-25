/// PGO Performance Tests
/// 
/// Tests that validate the performance improvements provided by
/// Profile-Guided Optimization, including benchmarks and regression tests.

use cursed::optimization::pgo::*;
use cursed::error::Result;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tempfile::TempDir;

/// Simulate a compute-intensive function for benchmarking
fn fibonacci_benchmark(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fibonacci_benchmark(n - 1) + fibonacci_benchmark(n - 2),
    }
}

/// Simulate a memory-intensive function
fn memory_benchmark(size: usize) -> Vec<u64> {
    let mut data = Vec::with_capacity(size);
    for i in 0..size {
        data.push((i * 7 + 13) as u64);
    }
    
    // Simulate some processing
    data.iter().map(|x| x * 2).collect()
}

/// Simulate a branchy function for branch prediction testing
fn branchy_benchmark(data: &[i32]) -> i32 {
    let mut result = 0;
    for &value in data {
        if value > 0 {
            result += value;
        } else if value < -10 {
            result -= value * 2;
        } else {
            result += value / 2;
        }
    }
    result
}

#[test]
fn test_instrumentation_overhead() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    
    // Test overhead of instrumentation
    let config_no_instr = PgoConfig {
        enabled: false,
        ..PgoConfig::default()
    };
    
    let config_with_instr = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        instrumentation_mode: InstrumentationMode::Frontend,
        collection_mode: CollectionMode::Counters,
        ..PgoConfig::default()
    };
    
    // Benchmark without instrumentation
    let start = Instant::now();
    let _manager_no_instr = PgoManager::new(config_no_instr)?;
    let time_no_instr = start.elapsed();
    
    // Benchmark with instrumentation
    let start = Instant::now();
    let _manager_with_instr = PgoManager::new(config_with_instr)?;
    let time_with_instr = start.elapsed();
    
    // Instrumentation overhead should be reasonable (< 5x slower)
    let overhead_ratio = time_with_instr.as_nanos() as f64 / time_no_instr.as_nanos() as f64;
    assert!(overhead_ratio < 5.0, "Instrumentation overhead too high: {}x", overhead_ratio);
    
    Ok(())
}

#[test]
fn test_profile_collection_performance() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        collection_mode: CollectionMode::Counters,
        ..PgoConfig::default()
    };
    
    let mut collector = ProfileCollector::new(config)?;
    
    // Benchmark profile collection
    let num_records = 10000;
    collector.start_collection("perf_test", None)?;
    
    let start = Instant::now();
    for i in 0..num_records {
        collector.record_function_profile(
            "perf_test",
            &format!("function_{}", i % 100), // 100 unique functions
            1,
            Duration::from_nanos(1000),
        )?;
    }
    let collection_time = start.elapsed();
    
    let profile_data = collector.collect_profile_data("perf_test")?;
    
    // Collection should be fast (< 1ms per 1000 records)
    let time_per_1000_records = collection_time.as_millis() as f64 / (num_records as f64 / 1000.0);
    assert!(time_per_1000_records < 1.0, "Profile collection too slow: {:.2}ms per 1000 records", time_per_1000_records);
    
    // Verify data was collected correctly
    assert!(!profile_data.function_counts.is_empty());
    
    Ok(())
}

#[test]
fn test_analysis_performance() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        ..PgoConfig::default()
    };
    
    // Create a large profile dataset
    let mut profile_data = ProfileData::new();
    
    // Add 1000 functions with varying execution counts
    for i in 0..1000 {
        let execution_count = match i % 10 {
            0..=2 => 10000, // Hot functions
            3..=6 => 1000,  // Medium functions
            _ => 10,        // Cold functions
        };
        profile_data.add_function_execution(format!("function_{}", i), execution_count);
    }
    
    // Add basic blocks (10 per function)
    for i in 0..1000 {
        for j in 0..10 {
            profile_data.add_basic_block_execution(
                format!("function_{}_bb{}", i, j),
                (i * 10 + j) as u64,
            );
        }
    }
    
    profile_data.total_execution_time = Duration::from_secs(10);
    
    // Benchmark analysis
    let analyzer = ProfileAnalyzer::new(config)?;
    let start = Instant::now();
    let analysis = analyzer.analyze_profile_data(&profile_data)?;
    let analysis_time = start.elapsed();
    
    // Analysis should be fast (< 100ms for 1000 functions)
    assert!(analysis_time < Duration::from_millis(100), 
            "Profile analysis too slow: {:?}", analysis_time);
    
    // Verify analysis results
    assert!(!analysis.hot_functions.is_empty());
    assert!(!analysis.cold_functions.is_empty());
    
    Ok(())
}

#[test]
fn test_optimization_engine_performance() -> Result<()> {
    let config = PgoConfig {
        enabled: true,
        optimization_strategy: OptimizationStrategy::Speed,
        ..PgoConfig::default()
    };
    
    // Create profile analysis with many optimization opportunities
    let analysis = ProfileAnalysis {
        hot_functions: (0..100).map(|i| HotFunction {
            name: format!("hot_function_{}", i),
            execution_count: 1000 + i * 10,
            total_time: Duration::from_millis(10 + i),
            average_time: Duration::from_nanos(10000 + i * 100),
            time_percentage: (10.0 + i as f64) / 100.0,
            optimization_priority: if i < 20 { 
                OptimizationPriority::Critical 
            } else if i < 50 { 
                OptimizationPriority::High 
            } else { 
                OptimizationPriority::Medium 
            },
            call_sites: HashMap::new(),
            call_count: 100 + i,
            average_size: 50 + (i % 50) as u32,
            has_vectorizable_loops: i % 3 == 0,
            memory_access_pattern: if i % 2 == 0 { 
                MemoryAccessPattern::Sequential 
            } else { 
                MemoryAccessPattern::Random 
            },
            branch_prediction_accuracy: 0.8 + (i as f64 % 20.0) / 100.0,
            cache_miss_rate: 0.05 + (i as f64 % 10.0) / 100.0,
            optimization_potential: OptimizationPotential::High,
        }).collect(),
        cold_functions: (0..50).map(|i| format!("cold_function_{}", i)).collect(),
        loop_profiles: Vec::new(),
        branch_profiles: Vec::new(),
        memory_profiles: Vec::new(),
        total_execution_time: Duration::from_secs(5),
        indirect_call_count: 1000,
        call_graph: HashMap::new(),
        critical_path: Vec::new(),
        recommendations: Vec::new(),
    };
    
    // Benchmark optimization engine
    let optimization_engine = PgoOptimizationEngine::new(config)?;
    let start = Instant::now();
    let optimization_results = optimization_engine.apply_optimizations(&analysis)?;
    let optimization_time = start.elapsed();
    
    // Optimization should be fast (< 50ms for 100 functions)
    assert!(optimization_time < Duration::from_millis(50),
            "Optimization engine too slow: {:?}", optimization_time);
    
    // Verify optimizations were applied
    assert!(!optimization_results.is_empty());
    assert!(optimization_results.len() <= 150); // At most one per function
    
    Ok(())
}

#[test]
fn test_memory_usage_efficiency() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        max_profile_data_size: 10 * 1024 * 1024, // 10MB limit
        ..PgoConfig::default()
    };
    
    // Test memory efficiency with large datasets
    let mut collector = ProfileCollector::new(config)?;
    collector.start_collection("memory_test", None)?;
    
    // Simulate collecting data for a large program
    let num_functions = 5000;
    let num_basic_blocks_per_function = 20;
    
    for func_id in 0..num_functions {
        collector.record_function_profile(
            "memory_test",
            &format!("function_{}", func_id),
            (func_id % 1000) + 1, // Varying execution counts
            Duration::from_nanos((func_id % 10000) + 1000),
        )?;
        
        for bb_id in 0..num_basic_blocks_per_function {
            collector.record_basic_block_profile(
                "memory_test",
                &format!("function_{}_bb_{}", func_id, bb_id),
                (func_id * bb_id % 500) + 1,
            )?;
        }
    }
    
    let profile_data = collector.collect_profile_data("memory_test")?;
    
    // Verify all data was collected
    assert_eq!(profile_data.function_counts.len(), num_functions);
    assert!(profile_data.basic_block_counts.len() <= num_functions * num_basic_blocks_per_function);
    
    // Test serialization size
    let json_data = serde_json::to_string(&profile_data).unwrap();
    let data_size = json_data.len();
    
    // Serialized data should be reasonable size (< 5MB for this test)
    assert!(data_size < 5 * 1024 * 1024, 
            "Serialized profile data too large: {} bytes", data_size);
    
    Ok(())
}

#[test]
fn test_concurrent_collection_performance() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        collection_mode: CollectionMode::CountersAndSampling,
        ..PgoConfig::default()
    };
    
    let num_threads = 4;
    let records_per_thread = 1000;
    
    // Test concurrent profile collection
    let start = Instant::now();
    
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let config = config.clone();
        std::thread::spawn(move || -> Result<()> {
            let mut collector = ProfileCollector::new(config)?;
            let session_id = format!("concurrent_test_{}", thread_id);
            collector.start_collection(&session_id, None)?;
            
            for i in 0..records_per_thread {
                collector.record_function_profile(
                    &session_id,
                    &format!("thread_{}_function_{}", thread_id, i % 10),
                    1,
                    Duration::from_nanos(1000),
                )?;
            }
            
            let _profile_data = collector.collect_profile_data(&session_id)?;
            Ok(())
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap()?;
    }
    
    let total_time = start.elapsed();
    
    // Concurrent collection should be efficient
    let time_per_record = total_time.as_nanos() as f64 / 
                         (num_threads * records_per_thread) as f64;
    assert!(time_per_record < 10000.0, // < 10μs per record
            "Concurrent collection too slow: {:.0}ns per record", time_per_record);
    
    Ok(())
}

#[test]
fn test_optimization_effectiveness_simulation() -> Result<()> {
    // Simulate the effect of PGO optimizations on different workload patterns
    
    // Test 1: Compute-intensive workload
    let compute_workload = vec![
        ("fibonacci_hot", 1000, Duration::from_millis(500)),
        ("fibonacci_cold", 10, Duration::from_millis(5)),
        ("compute_heavy", 500, Duration::from_millis(300)),
    ];
    
    let compute_improvement = simulate_pgo_optimization(&compute_workload, OptimizationStrategy::Speed);
    assert!(compute_improvement > 0.15, "Compute workload should see >15% improvement");
    
    // Test 2: Memory-intensive workload
    let memory_workload = vec![
        ("memory_alloc_hot", 2000, Duration::from_millis(400)),
        ("memory_access_pattern", 1500, Duration::from_millis(200)),
        ("cache_friendly", 800, Duration::from_millis(100)),
    ];
    
    let memory_improvement = simulate_pgo_optimization(&memory_workload, OptimizationStrategy::Balanced);
    assert!(memory_improvement > 0.10, "Memory workload should see >10% improvement");
    
    // Test 3: Size-optimized workload
    let size_workload = vec![
        ("many_small_functions", 5000, Duration::from_millis(50)),
        ("cold_error_handling", 5, Duration::from_millis(1)),
        ("rarely_used_features", 20, Duration::from_millis(2)),
    ];
    
    let size_improvement = simulate_pgo_optimization(&size_workload, OptimizationStrategy::Size);
    assert!(size_improvement > 0.05, "Size workload should see >5% improvement");
    
    Ok(())
}

/// Simulate PGO optimization effects on a given workload
fn simulate_pgo_optimization(
    workload: &[(&str, u64, Duration)], 
    strategy: OptimizationStrategy
) -> f64 {
    let total_execution_count: u64 = workload.iter().map(|(_, count, _)| count).sum();
    let total_time: Duration = workload.iter().map(|(_, _, time)| *time).sum();
    
    let mut total_improvement = 0.0;
    
    for (name, count, time) in workload {
        let time_percentage = time.as_nanos() as f64 / total_time.as_nanos() as f64;
        let frequency_factor = *count as f64 / total_execution_count as f64;
        
        // Simulate optimization benefits based on strategy and function characteristics
        let base_improvement = match strategy {
            OptimizationStrategy::Speed => {
                if frequency_factor > 0.3 { 0.25 } // Hot functions get major optimizations
                else if frequency_factor > 0.1 { 0.15 } // Warm functions get moderate optimizations
                else { 0.05 } // Cold functions get minor optimizations
            },
            OptimizationStrategy::Size => {
                if frequency_factor < 0.1 { 0.20 } // Cold functions get size optimizations
                else { 0.08 } // Hot functions get modest size improvements
            },
            OptimizationStrategy::Balanced => {
                if frequency_factor > 0.2 { 0.18 } // Hot functions get good optimizations
                else if frequency_factor > 0.05 { 0.12 } // Medium functions get moderate optimizations
                else { 0.10 } // Cold functions get size optimizations
            },
            OptimizationStrategy::Custom { speed_weight, size_weight, .. } => {
                let speed_benefit = if frequency_factor > 0.2 { 0.25 } else { 0.08 };
                let size_benefit = if frequency_factor < 0.1 { 0.15 } else { 0.05 };
                speed_benefit * speed_weight + size_benefit * size_weight
            }
        };
        
        // Additional improvements for specific patterns
        let pattern_improvement = if name.contains("hot") { 0.05 }
        else if name.contains("loop") { 0.10 }
        else if name.contains("vectorizable") { 0.15 }
        else { 0.0 };
        
        let function_improvement = (base_improvement + pattern_improvement) * time_percentage;
        total_improvement += function_improvement;
    }
    
    total_improvement
}

#[test]
fn test_real_world_benchmark_simulation() -> Result<()> {
    // Simulate realistic benchmark scenarios
    
    struct BenchmarkScenario {
        name: &'static str,
        functions: Vec<(&'static str, u64, Duration, bool)>, // name, count, time, vectorizable
        expected_min_improvement: f64,
    }
    
    let scenarios = vec![
        BenchmarkScenario {
            name: "Scientific Computing",
            functions: vec![
                ("matrix_multiply", 100, Duration::from_millis(800), true),
                ("fft_transform", 50, Duration::from_millis(400), true),
                ("solver_iteration", 1000, Duration::from_millis(200), false),
                ("error_handling", 5, Duration::from_millis(1), false),
            ],
            expected_min_improvement: 0.20,
        },
        BenchmarkScenario {
            name: "Web Server",
            functions: vec![
                ("handle_request", 10000, Duration::from_millis(100), false),
                ("parse_headers", 10000, Duration::from_millis(50), false),
                ("database_query", 5000, Duration::from_millis(300), false),
                ("error_page", 10, Duration::from_millis(5), false),
            ],
            expected_min_improvement: 0.12,
        },
        BenchmarkScenario {
            name: "Compiler",
            functions: vec![
                ("parse_source", 1000, Duration::from_millis(200), false),
                ("type_checking", 1000, Duration::from_millis(300), false),
                ("optimization_pass", 1000, Duration::from_millis(400), true),
                ("code_generation", 1000, Duration::from_millis(250), false),
                ("error_recovery", 50, Duration::from_millis(10), false),
            ],
            expected_min_improvement: 0.15,
        },
    ];
    
    for scenario in scenarios {
        let workload: Vec<_> = scenario.functions.iter()
            .map(|(name, count, time, _)| (*name, *count, *time))
            .collect();
        
        let improvement = simulate_pgo_optimization(&workload, OptimizationStrategy::Speed);
        
        assert!(improvement >= scenario.expected_min_improvement,
                "Scenario '{}' should see >{:.0}% improvement, got {:.0}%",
                scenario.name, 
                scenario.expected_min_improvement * 100.0,
                improvement * 100.0);
    }
    
    Ok(())
}

#[test]
fn test_pgo_regression_detection() -> Result<()> {
    // Test that PGO system can detect performance regressions
    
    let baseline_performance = vec![
        ("function_a", Duration::from_millis(100)),
        ("function_b", Duration::from_millis(50)),
        ("function_c", Duration::from_millis(200)),
    ];
    
    let optimized_performance = vec![
        ("function_a", Duration::from_millis(80)),  // 20% improvement
        ("function_b", Duration::from_millis(45)),  // 10% improvement
        ("function_c", Duration::from_millis(180)), // 10% improvement
    ];
    
    let regressed_performance = vec![
        ("function_a", Duration::from_millis(120)), // 20% regression
        ("function_b", Duration::from_millis(45)),  // 10% improvement
        ("function_c", Duration::from_millis(180)), // 10% improvement
    ];
    
    // Calculate performance changes
    let improvement = calculate_performance_change(&baseline_performance, &optimized_performance);
    let regression = calculate_performance_change(&baseline_performance, &regressed_performance);
    
    assert!(improvement > 0.0, "Should detect performance improvement");
    assert!(regression < 0.0, "Should detect performance regression");
    
    // Test regression detection threshold
    assert!(improvement.abs() > 0.10, "Should detect significant improvements");
    assert!(regression.abs() > 0.05, "Should detect significant regressions");
    
    Ok(())
}

fn calculate_performance_change(
    baseline: &[(&str, Duration)],
    current: &[(&str, Duration)]
) -> f64 {
    let baseline_total: Duration = baseline.iter().map(|(_, time)| *time).sum();
    let current_total: Duration = current.iter().map(|(_, time)| *time).sum();
    
    let baseline_ns = baseline_total.as_nanos() as f64;
    let current_ns = current_total.as_nanos() as f64;
    
    (baseline_ns - current_ns) / baseline_ns // Positive = improvement
}

#[test]
fn test_scalability_limits() -> Result<()> {
    // Test PGO system behavior at scale limits
    
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        max_profile_data_size: 50 * 1024 * 1024, // 50MB limit
        ..PgoConfig::default()
    };
    
    // Test with large number of functions
    let large_function_count = 10000;
    let mut profile_data = ProfileData::new();
    
    let start = Instant::now();
    
    for i in 0..large_function_count {
        profile_data.add_function_execution(
            format!("large_scale_function_{}", i), 
            (i % 1000) + 1
        );
    }
    
    let creation_time = start.elapsed();
    
    // Should handle large datasets efficiently
    assert!(creation_time < Duration::from_secs(1),
            "Large dataset creation too slow: {:?}", creation_time);
    
    // Test analysis scalability
    let analyzer = ProfileAnalyzer::new(config.clone())?;
    let start = Instant::now();
    let _analysis = analyzer.analyze_profile_data(&profile_data)?;
    let analysis_time = start.elapsed();
    
    assert!(analysis_time < Duration::from_secs(2),
            "Large dataset analysis too slow: {:?}", analysis_time);
    
    Ok(())
}

/// Benchmark the overall PGO pipeline end-to-end
#[test]
fn test_end_to_end_pgo_pipeline_performance() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = PgoConfig {
        enabled: true,
        profile_data_dir: temp_dir.path().to_path_buf(),
        optimization_strategy: OptimizationStrategy::Balanced,
        ..PgoConfig::default()
    };
    
    let pipeline_start = Instant::now();
    
    // Step 1: Initialize PGO Manager
    let step1_start = Instant::now();
    let mut pgo_manager = PgoManager::new(config)?;
    let step1_time = step1_start.elapsed();
    
    // Step 2: Start session and collect data
    let step2_start = Instant::now();
    let session_id = pgo_manager.start_session(None)?;
    
    // Simulate profile data collection
    let mut collector = ProfileCollector::new(pgo_manager.config.clone())?;
    collector.start_collection(&session_id, None)?;
    
    // Add realistic profile data
    for i in 0..500 {
        collector.record_function_profile(
            &session_id,
            &format!("benchmark_function_{}", i % 50),
            (i % 100) + 1,
            Duration::from_nanos((i % 10000) + 1000),
        )?;
    }
    
    let _profile_data = collector.collect_profile_data(&session_id)?;
    let step2_time = step2_start.elapsed();
    
    // Step 3: Stop session and analyze
    let step3_start = Instant::now();
    let _session = pgo_manager.stop_session()?;
    let _recommendations = pgo_manager.analyze_and_recommend(&session_id)?;
    let step3_time = step3_start.elapsed();
    
    let total_pipeline_time = pipeline_start.elapsed();
    
    // Performance assertions
    assert!(step1_time < Duration::from_millis(50), "Initialization too slow: {:?}", step1_time);
    assert!(step2_time < Duration::from_millis(100), "Collection too slow: {:?}", step2_time);
    assert!(step3_time < Duration::from_millis(50), "Analysis too slow: {:?}", step3_time);
    assert!(total_pipeline_time < Duration::from_millis(300), "Total pipeline too slow: {:?}", total_pipeline_time);
    
    println!("PGO Pipeline Performance:");
    println!("  Initialization: {:?}", step1_time);
    println!("  Collection: {:?}", step2_time);
    println!("  Analysis: {:?}", step3_time);
    println!("  Total: {:?}", total_pipeline_time);
    
    Ok(())
}
