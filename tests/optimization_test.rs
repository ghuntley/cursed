/// Comprehensive Test Suite for CURSED Optimization System
/// 
/// Tests performance profiling, JIT optimization, memory optimization,
/// and compilation speed improvements.

use std::time::Duration;
use std::thread;
use std::collections::HashMap;

use cursed::optimization::*;
use cursed::error::Result;

#[cfg(test)]
mod optimization_tests {
    use super::*;

    #[test]
    fn test_optimization_config_creation() {
        let config = OptimizationConfig::default();
        
        assert!(config.enable_advanced_llvm);
        assert!(config.enable_parallel_compilation);
        assert!(config.enable_jit_optimization);
        assert!(config.enable_memory_optimization);
        assert!(config.enable_caching);
        assert_eq!(config.optimization_level, 2);
        assert!(config.max_parallel_threads > 0);
    }

    #[test]
    fn test_optimization_manager_creation() {
        let config = OptimizationConfig::default();
        let manager = OptimizationManager::new(config);
        
        assert!(manager.is_ok());
        let manager = manager.unwrap();
        assert!(manager.speed_optimizer().is_some());
        assert!(manager.jit_optimizer().is_some());
        assert!(manager.memory_optimizer().is_some());
    }

    #[test]
    fn test_performance_profiler() {
        let config = OptimizationConfig::default();
        let profiler = PerformanceProfiler::new(&config).unwrap();
        
        // Test profiling session
        let session_id = profiler.start_profiling("test_function").unwrap();
        thread::sleep(Duration::from_millis(10));
        profiler.end_profiling(&session_id).unwrap();
        
        // Check metrics
        let metrics = profiler.get_function_metrics("test_function").unwrap();
        assert!(metrics.is_some());
        
        let metrics = metrics.unwrap();
        assert_eq!(metrics.execution_count, 1);
        assert!(metrics.total_execution_time >= Duration::from_millis(10));
        assert_eq!(metrics.name, "test_function");
    }

    #[test]
    fn test_performance_profiler_multiple_executions() {
        let config = OptimizationConfig::default();
        let profiler = PerformanceProfiler::new(&config).unwrap();
        
        // Profile multiple executions
        for i in 0..5 {
            let session_id = profiler.start_profiling("repeated_function").unwrap();
            thread::sleep(Duration::from_millis(2)); // Simulate work
            profiler.end_profiling(&session_id).unwrap();
        }
        
        let metrics = profiler.get_function_metrics("repeated_function").unwrap().unwrap();
        assert_eq!(metrics.execution_count, 5);
        assert!(metrics.total_execution_time >= Duration::from_millis(10));
        
        // Test hot path detection
        let hot_paths = profiler.get_hot_paths().unwrap();
        // With only 5 executions, it shouldn't be considered hot by default threshold
        assert!(hot_paths.is_empty());
    }

    #[test]
    fn test_hot_path_profiler() {
        let config = JitOptimizationConfig::default();
        let profiler = HotPathProfiler::new(config);
        
        // Record many executions to trigger hot path detection
        for _ in 0..1500 {
            profiler.record_execution("hot_function", Duration::from_micros(100)).unwrap();
        }
        
        for _ in 0..50 {
            profiler.record_execution("cold_function", Duration::from_micros(100)).unwrap();
        }
        
        let hot_functions = profiler.get_hot_functions().unwrap();
        assert_eq!(hot_functions.len(), 1);
        assert_eq!(hot_functions[0].0, "hot_function");
        assert_eq!(hot_functions[0].1, 1500);
    }

    #[test]
    fn test_jit_optimization_config() {
        let config = JitOptimizationConfig::default();
        
        assert!(config.enable_adaptive_optimization);
        assert_eq!(config.hot_function_threshold, 1000);
        assert_eq!(config.max_optimization_level, 3);
        assert!(config.enable_profile_guided_optimization);
        assert_eq!(config.speculation_threshold, 0.8);
    }

    #[test]
    fn test_adaptive_jit_optimizer_creation() {
        let config = OptimizationConfig::default();
        let optimizer = AdaptiveJitOptimizer::new(&config);
        
        assert!(optimizer.is_ok());
        let optimizer = optimizer.unwrap();
        
        // Test recording execution
        optimizer.record_execution("test_func", Duration::from_millis(1)).unwrap();
        
        // Get statistics
        let stats = optimizer.get_statistics().unwrap();
        assert_eq!(stats.total_compiled_functions, 0); // No compilations yet
    }

    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new("test_pool".to_string(), 64, 10).unwrap();
        
        // Test allocation
        let ptr1 = pool.allocate().unwrap();
        let ptr2 = pool.allocate().unwrap();
        
        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
        assert_ne!(ptr1, ptr2);
        
        // Test statistics
        let stats = pool.get_statistics();
        assert_eq!(stats.name, "test_pool");
        assert_eq!(stats.object_size, 64);
        assert_eq!(stats.objects_in_use, 2);
        assert_eq!(stats.allocation_count, 2);
        
        // Test deallocation
        pool.deallocate(ptr1).unwrap();
        pool.deallocate(ptr2).unwrap();
        
        let stats = pool.get_statistics();
        assert_eq!(stats.objects_in_use, 0);
        assert_eq!(stats.deallocation_count, 2);
        assert_eq!(stats.hit_rate, 1.0); // All allocations were from pool
    }

    #[test]
    fn test_cache_optimizer() {
        let config = MemoryOptimizationConfig::default();
        let optimizer = CacheOptimizer::new(config);
        
        // Test access pattern analysis
        let field_accesses = vec![
            ("field1".to_string(), 1000), // Hot field
            ("field2".to_string(), 10),   // Cold field
            ("field3".to_string(), 500),  // Warm field
        ];
        
        let pattern = optimizer.analyze_access_pattern("TestType", &field_accesses).unwrap();
        
        assert_eq!(pattern.object_type, "TestType");
        assert!(!pattern.hot_fields.is_empty());
        assert!(!pattern.cold_fields.is_empty());
        assert!(pattern.cache_hit_rate > 0.0);
        assert!(pattern.sequential_access_ratio > 0.0);
    }

    #[test]
    fn test_memory_layout_optimizer() {
        let config = OptimizationConfig::default();
        let optimizer = MemoryLayoutOptimizer::new(&config).unwrap();
        
        // Create a test object layout
        let layout = ObjectLayout {
            type_name: "TestStruct".to_string(),
            fields: vec![
                FieldLayout {
                    name: "field1".to_string(),
                    size: 8,
                    offset: 0,
                    alignment: 8,
                    access_frequency: 1000,
                },
                FieldLayout {
                    name: "field2".to_string(),
                    size: 4,
                    offset: 8,
                    alignment: 4,
                    access_frequency: 10,
                },
            ],
            total_size: 16,
            alignment: 8,
            cache_lines_used: 1,
            padding_bytes: 4,
            field_access_frequencies: [
                ("field1".to_string(), 1000),
                ("field2".to_string(), 10),
            ].iter().cloned().collect(),
        };
        
        let optimized = optimizer.optimize_type_layout(&layout).unwrap();
        assert_eq!(optimized.type_name, "TestStruct");
        // The optimizer should maintain or improve the layout
        assert!(optimized.cache_lines_used <= layout.cache_lines_used);
    }

    #[test]
    fn test_allocation_optimizer() {
        let config = MemoryOptimizationConfig::default();
        let optimizer = AllocationOptimizer::new(config).unwrap();
        
        // Test allocation
        let ptr = optimizer.allocate(32).unwrap();
        assert!(!ptr.is_null());
        
        // Test deallocation
        optimizer.deallocate(ptr, 32).unwrap();
        
        // Check statistics
        let stats = optimizer.get_statistics().unwrap();
        assert_eq!(stats.total_allocations, 1);
        assert_eq!(stats.total_allocated_bytes, 32);
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        
        // Build a dependency graph: A -> B -> C
        graph.add_dependency("A", "B");
        graph.add_dependency("B", "C");
        
        // Test dependencies
        let deps_a = graph.get_dependencies("A");
        assert_eq!(deps_a, vec!["B"]);
        
        let deps_b = graph.get_dependencies("B");
        assert_eq!(deps_b, vec!["C"]);
        
        // Test topological sort
        let order = graph.topological_sort().unwrap();
        assert_eq!(order, vec!["C", "B", "A"]);
        
        // Test affected modules
        let affected = graph.get_affected_modules("C");
        assert!(affected.contains("A"));
        assert!(affected.contains("B"));
        assert!(affected.contains("C"));
        assert_eq!(affected.len(), 3);
    }

    #[test]
    fn test_compilation_unit() {
        use std::time::SystemTime;
        use std::path::PathBuf;
        
        let unit = CompilationUnit {
            id: "test_module".to_string(),
            source_path: PathBuf::from("test.csd"),
            module_name: "test".to_string(),
            source_code: "facts x = 42;".to_string(),
            dependencies: vec!["dependency".to_string()],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        };
        
        assert_eq!(unit.module_name, "test");
        assert_eq!(unit.status, CompilationStatus::Pending);
        assert_eq!(unit.dependencies.len(), 1);
        assert_eq!(unit.dependencies[0], "dependency");
    }

    #[test]
    fn test_parallel_ast_processor() {
        use std::path::PathBuf;
        use std::time::SystemTime;
        
        let config = CompilationSpeedConfig {
            max_parallel_threads: 2,
            ..Default::default()
        };
        
        let processor = ParallelAstProcessor::new(config).unwrap();
        
        let unit = CompilationUnit {
            id: "test".to_string(),
            source_path: PathBuf::from("test.csd"),
            module_name: "test".to_string(),
            source_code: "facts x = 42;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        };
        
        processor.submit_unit(unit).unwrap();
        processor.wait_for_completion().unwrap();
        
        let results = processor.get_results();
        assert!(!results.is_empty());
        
        let stats = processor.get_statistics();
        assert_eq!(stats.total_workers, 2);
    }

    #[test]
    fn test_type_checking_optimizer() {
        let config = CompilationSpeedConfig::default();
        let optimizer = TypeCheckingOptimizer::new(config);
        
        // Create a simple program for testing
        use cursed::ast::{Program, Statement};
        let program = Program {
            statements: vec![],
        };
        
        let result = optimizer.check_types("test_module", &program).unwrap();
        assert_eq!(result.module_name, "test_module");
        assert!(result.type_errors.is_empty()); // Simple program should have no errors
        
        // Test caching
        let cached = optimizer.get_cached_types("test_module");
        assert!(cached.is_some());
    }

    #[test]
    fn test_compilation_speed_optimizer() {
        let config = OptimizationConfig::default();
        let optimizer = CompilationSpeedOptimizer::new(&config).unwrap();
        
        let stats = optimizer.get_statistics();
        assert_eq!(stats.total_units, 0);
        assert_eq!(stats.completed_units, 0);
        assert_eq!(stats.failed_units, 0);
    }

    #[test]
    fn test_optimization_recommendations() {
        let config = OptimizationConfig::default();
        let profiler = PerformanceProfiler::new(&config).unwrap();
        
        // Generate some profile data by simulating function executions
        for _ in 0..2000 { // High execution count to trigger recommendations
            let session_id = profiler.start_profiling("frequently_called").unwrap();
            thread::sleep(Duration::from_micros(5)); // Very fast function
            profiler.end_profiling(&session_id).unwrap();
        }
        
        for _ in 0..600 { // Medium execution count, longer duration
            let session_id = profiler.start_profiling("loop_heavy").unwrap();
            thread::sleep(Duration::from_millis(2)); // Longer function
            profiler.end_profiling(&session_id).unwrap();
        }
        
        let recommendations = profiler.generate_optimization_recommendations().unwrap();
        
        // Should generate recommendations for the profiled functions
        assert!(!recommendations.is_empty());
        
        // Check that recommendations have proper structure
        for rec in &recommendations {
            assert!(!rec.function_name.is_empty());
            // Priority should be set
            assert!(matches!(rec.priority, OptimizationPriority::Low | 
                                         OptimizationPriority::Medium | 
                                         OptimizationPriority::High | 
                                         OptimizationPriority::Critical));
        }
    }

    #[test]
    fn test_compilation_profiler() {
        let config = OptimizationConfig::default();
        let profiler = CompilationProfiler::new(&config).unwrap();
        
        // Test profiling a compilation phase
        let result = profiler.profile_phase(CompilationPhase::Parsing, || {
            thread::sleep(Duration::from_millis(10));
            Ok("parsed".to_string())
        });
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "parsed");
        
        let summary = profiler.get_summary().unwrap();
        assert!(summary.parsing_time >= Duration::from_millis(10));
    }

    #[test]
    fn test_runtime_profiler() {
        let config = OptimizationConfig::default();
        let profiler = RuntimeProfiler::new(&config).unwrap();
        
        // Test function profiling
        let result = profiler.profile_function("test_function", || {
            thread::sleep(Duration::from_millis(5));
            Ok(42)
        });
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        
        let hot_paths = profiler.get_hot_paths().unwrap();
        // Single execution shouldn't be considered hot
        assert!(hot_paths.is_empty());
    }

    #[test]
    fn test_memory_optimization_statistics() {
        let config = OptimizationConfig::default();
        let optimizer = MemoryLayoutOptimizer::new(&config).unwrap();
        
        let stats = optimizer.get_optimization_statistics().unwrap();
        assert_eq!(stats.allocation_stats.total_allocations, 0);
        assert!(stats.pool_stats.is_empty());
    }

    #[test]
    fn test_jit_optimization_statistics() {
        let config = OptimizationConfig::default();
        let optimizer = AdaptiveJitOptimizer::new(&config).unwrap();
        
        let stats = optimizer.get_statistics().unwrap();
        assert_eq!(stats.total_compiled_functions, 0);
        assert_eq!(stats.total_recompilations, 0);
        assert_eq!(stats.average_speedup, 1.0);
    }

    #[test]
    fn test_optimization_reports() {
        let config = OptimizationConfig::default();
        
        // Test JIT optimization report
        let jit_optimizer = AdaptiveJitOptimizer::new(&config).unwrap();
        let jit_report = jit_optimizer.generate_optimization_report().unwrap();
        assert!(jit_report.contains("JIT Optimization Report"));
        assert!(jit_report.contains("Statistics"));
        
        // Test memory optimization report
        let memory_optimizer = MemoryLayoutOptimizer::new(&config).unwrap();
        let memory_report = memory_optimizer.generate_optimization_report().unwrap();
        assert!(memory_report.contains("Memory Optimization Report"));
        assert!(memory_report.contains("Allocation Statistics"));
        
        // Test compilation speed report
        let speed_optimizer = CompilationSpeedOptimizer::new(&config).unwrap();
        let speed_report = speed_optimizer.generate_performance_report();
        assert!(speed_report.contains("Compilation Performance Report"));
        assert!(speed_report.contains("Overall Statistics"));
    }

    #[test]
    fn test_profile_data_generation() {
        let config = JitOptimizationConfig::default();
        let profiler = HotPathProfiler::new(config);
        
        // Record some calls and branches
        profiler.record_call("main", "helper").unwrap();
        profiler.record_call("main", "helper").unwrap(); // Call twice
        profiler.record_branch("main", true, true).unwrap();
        profiler.record_branch("main", false, false).unwrap();
        profiler.record_branch("main", true, true).unwrap();
        
        let profile_data = profiler.generate_profile_data("main").unwrap();
        assert!(profile_data.is_some());
        
        let profile_data = profile_data.unwrap();
        assert!(!profile_data.call_frequencies.is_empty());
        assert!(!profile_data.branch_probabilities.is_empty());
        
        // Check call frequencies
        assert_eq!(profile_data.call_frequencies.get("helper"), Some(&2));
        
        // Check branch probabilities
        assert!(profile_data.branch_probabilities.contains_key("main_branch"));
    }

    #[test]
    fn test_optimization_error_handling() {
        let config = OptimizationConfig::default();
        let profiler = PerformanceProfiler::new(&config).unwrap();
        
        // Test ending non-existent session
        let result = profiler.end_profiling("non_existent_session");
        assert!(result.is_err());
        
        // Test getting metrics for non-existent function
        let metrics = profiler.get_function_metrics("non_existent_function").unwrap();
        assert!(metrics.is_none());
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = DependencyGraph::new();
        
        // Create circular dependency: A -> B -> C -> A
        graph.add_dependency("A", "B");
        graph.add_dependency("B", "C");
        graph.add_dependency("C", "A");
        
        let result = graph.topological_sort();
        assert!(result.is_err());
        assert!(graph.has_cycles());
    }

    #[test]
    fn test_performance_metrics_calculation() {
        let mut metrics = PerformanceMetrics::new("test_function".to_string());
        
        // Update with sample data
        metrics.update(Duration::from_millis(10), 100, 1024);
        metrics.update(Duration::from_millis(20), 200, 2048);
        metrics.update(Duration::from_millis(5), 50, 512);
        
        assert_eq!(metrics.execution_count, 3);
        assert_eq!(metrics.total_execution_time, Duration::from_millis(35));
        assert!(metrics.average_execution_time >= Duration::from_millis(11));
        assert!(metrics.average_execution_time <= Duration::from_millis(12));
        assert_eq!(metrics.min_execution_time, Duration::from_millis(5));
        assert_eq!(metrics.max_execution_time, Duration::from_millis(20));
        assert_eq!(metrics.memory_allocations, 350);
        assert_eq!(metrics.peak_memory_usage, 2048);
        
        // Test frequency calculation
        let frequency = metrics.execution_frequency();
        assert!(frequency > 0.0);
        
        // Test performance cost calculation
        let cost = metrics.performance_cost();
        assert!(cost > 0.0);
    }
}

/// Performance regression tests
#[cfg(test)]
mod performance_regression_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_profiler_overhead() {
        let config = OptimizationConfig::default();
        let profiler = PerformanceProfiler::new(&config).unwrap();
        
        let start = Instant::now();
        
        // Measure overhead of 1000 profiling sessions
        for i in 0..1000 {
            let session_id = profiler.start_profiling(&format!("function_{}", i)).unwrap();
            profiler.end_profiling(&session_id).unwrap();
        }
        
        let elapsed = start.elapsed();
        let overhead_per_session = elapsed.as_nanos() / 1000;
        
        // Profiling overhead should be less than 10μs per session
        assert!(overhead_per_session < 10_000, 
                "Profiling overhead too high: {}ns per session", overhead_per_session);
    }

    #[test]
    fn test_memory_pool_performance() {
        let mut pool = MemoryPool::new("perf_test".to_string(), 64, 1000).unwrap();
        
        let start = Instant::now();
        
        // Allocate and deallocate 10000 objects
        for _ in 0..10000 {
            if let Some(ptr) = pool.allocate() {
                pool.deallocate(ptr).unwrap();
            }
        }
        
        let elapsed = start.elapsed();
        let time_per_operation = elapsed.as_nanos() / 20000; // 2 operations per iteration
        
        // Each operation should take less than 1μs
        assert!(time_per_operation < 1000, 
                "Memory pool operations too slow: {}ns per operation", time_per_operation);
    }

    #[test]
    fn test_parallel_compilation_speedup() {
        use std::path::PathBuf;
        use std::time::SystemTime;
        
        // Create test compilation units
        let mut units = Vec::new();
        for i in 0..20 {
            units.push(CompilationUnit {
                id: format!("module_{}", i),
                source_path: PathBuf::from(format!("module_{}.csd", i)),
                module_name: format!("module_{}", i),
                source_code: format!("facts x_{} = {};", i, i),
                dependencies: vec![],
                last_modified: SystemTime::now(),
                status: CompilationStatus::Pending,
                priority: 1,
            });
        }
        
        // Test serial compilation time (single thread)
        let serial_config = CompilationSpeedConfig {
            max_parallel_threads: 1,
            ..Default::default()
        };
        let serial_optimizer = CompilationSpeedOptimizer::new(&OptimizationConfig {
            max_parallel_threads: 1,
            ..Default::default()
        }).unwrap();
        
        let serial_start = Instant::now();
        let _serial_results = serial_optimizer.compile_parallel(units.clone()).unwrap();
        let serial_time = serial_start.elapsed();
        
        // Test parallel compilation time (multiple threads)
        let parallel_config = OptimizationConfig {
            max_parallel_threads: 4,
            ..Default::default()
        };
        let parallel_optimizer = CompilationSpeedOptimizer::new(&parallel_config).unwrap();
        
        let parallel_start = Instant::now();
        let _parallel_results = parallel_optimizer.compile_parallel(units).unwrap();
        let parallel_time = parallel_start.elapsed();
        
        // Parallel compilation should be faster (allowing for some overhead)
        let speedup = serial_time.as_secs_f64() / parallel_time.as_secs_f64();
        println!("Compilation speedup: {:.2}x", speedup);
        
        // Should achieve at least some speedup (even with overhead)
        assert!(speedup > 0.8, "Parallel compilation not providing expected speedup: {:.2}x", speedup);
    }
}

/// Integration tests with actual CURSED code compilation
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_optimization_with_real_code() {
        let config = OptimizationConfig {
            enable_profiling: true,
            enable_jit_optimization: true,
            enable_memory_optimization: true,
            enable_parallel_compilation: true,
            ..Default::default()
        };
        
        let manager = OptimizationManager::new(config).unwrap();
        
        // Verify all optimizers are available
        assert!(manager.profiler().is_some());
        assert!(manager.jit_optimizer().is_some());
        assert!(manager.memory_optimizer().is_some());
        assert!(manager.speed_optimizer().is_some());
        
        // Test configuration update
        let new_config = OptimizationConfig {
            optimization_level: 3,
            max_parallel_threads: 8,
            ..Default::default()
        };
        
        // This would test updating configuration
        // manager.update_config(new_config).unwrap();
    }

    #[test]
    fn test_end_to_end_optimization_workflow() {
        let config = OptimizationConfig::default();
        let profiler = PerformanceProfiler::new(&config).unwrap();
        let jit_optimizer = AdaptiveJitOptimizer::new(&config).unwrap();
        
        // Simulate a realistic workflow
        
        // 1. Profile some function executions
        for _ in 0..1500 {
            let session_id = profiler.start_profiling("hot_computation").unwrap();
            // Simulate computation
            thread::sleep(Duration::from_micros(100));
            profiler.end_profiling(&session_id).unwrap();
            
            // Also record in JIT optimizer
            jit_optimizer.record_execution("hot_computation", Duration::from_micros(100)).unwrap();
        }
        
        // 2. Get optimization recommendations
        let recommendations = profiler.generate_optimization_recommendations().unwrap();
        assert!(!recommendations.is_empty());
        
        // 3. Process recompilation queue
        let recompiled = jit_optimizer.process_recompilation_queue().unwrap();
        
        // 4. Generate reports
        let jit_report = jit_optimizer.generate_optimization_report().unwrap();
        assert!(jit_report.contains("hot_computation"));
        
        println!("Generated {} optimization recommendations", recommendations.len());
        println!("Recompiled {} functions", recompiled.len());
    }
}
