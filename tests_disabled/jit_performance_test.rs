/// JIT Performance Tests for CURSED Language
/// 
/// Comprehensive performance testing for JIT compilation infrastructure:
/// - Compilation speed benchmarks
/// - Execution performance benchmarks  
/// - Memory usage benchmarks
/// - Hot path optimization benefits
/// - Scalability tests

use cursed::error::Error;
use cursed::config::JitConfig;
use cursed::codegen::llvm::{LlvmCodeGenerator, jit_engine::CursedJitEngine, jit_compilation::JitCompilationInterface};
use cursed::runtime::{Runtime, jit_runtime::JitRuntime};

use std::sync::Arc;
use std::time::{Duration, Instant};
use inkwell::context::Context;

/// Performance benchmark for JIT engine compilation speed
#[test]
#[ignore] // Ignore for regular test runs - run with: cargo test --release --ignored
fn test_jit_compilation_speed_benchmark() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    let function_count = 100;
    let start_time = Instant::now();
    
    // Compile multiple functions and measure time
    for i in 0..function_count {
        let function_name = format!("benchmark_function_{}", i);
        let result = engine.compile_function(&function_name, "");
        assert!(result.is_ok(), "Compilation should succeed for function {}", i);
    }
    
    let compilation_time = start_time.elapsed();
    let avg_compilation_time = compilation_time / function_count;
    
    println!("=== JIT Compilation Speed Benchmark ===");
    println!("Functions compiled: {}", function_count);
    println!("Total compilation time: {:?}", compilation_time);
    println!("Average compilation time per function: {:?}", avg_compilation_time);
    println!("Compilations per second: {:.2}", function_count as f64 / compilation_time.as_secs_f64());
    
    // Performance assertions
    assert!(avg_compilation_time < Duration::from_millis(100), 
           "Average compilation time should be under 100ms, got {:?}", avg_compilation_time);
    assert!(compilation_time < Duration::from_secs(10), 
           "Total compilation time should be under 10 seconds, got {:?}", compilation_time);
    
    // Verify statistics
    let stats = engine.get_stats();
    assert_eq!(stats.functions_compiled, function_count as u64);
    assert!(stats.compilation_time_ms > 0);
}

/// Performance benchmark for JIT function execution speed
#[test]
#[ignore] // Ignore for regular test runs
fn test_jit_execution_speed_benchmark() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    // Compile function once
    engine.compile_function("execution_benchmark", "").unwrap();
    
    let execution_count = 10000;
    let start_time = Instant::now();
    
    // Execute function many times
    for _ in 0..execution_count {
        let result = engine.execute_function("execution_benchmark");
        assert!(result.is_ok(), "Execution should succeed");
    }
    
    let execution_time = start_time.elapsed();
    let avg_execution_time = execution_time / execution_count;
    
    println!("=== JIT Execution Speed Benchmark ===");
    println!("Function executions: {}", execution_count);
    println!("Total execution time: {:?}", execution_time);
    println!("Average execution time per call: {:?}", avg_execution_time);
    println!("Executions per second: {:.0}", execution_count as f64 / execution_time.as_secs_f64());
    
    // Performance assertions
    assert!(avg_execution_time < Duration::from_micros(100), 
           "Average execution time should be under 100μs, got {:?}", avg_execution_time);
    assert!(execution_time < Duration::from_secs(1), 
           "Total execution time should be under 1 second, got {:?}", execution_time);
    
    // Verify statistics
    let stats = engine.get_stats();
    assert_eq!(stats.functions_executed, execution_count as u64);
}

/// Performance benchmark for hot path optimization benefits
#[test]
#[ignore] // Ignore for regular test runs
fn test_jit_hot_path_optimization_benchmark() {
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    
    let mut config = cursed::codegen::llvm::jit_compilation::JitCompilationConfig::default();
    config.hot_path_threshold = 50; // Moderate threshold
    
    let mut interface = JitCompilationInterface::new(&context, jit_engine, codegen, config);
    
    // Compile function
    interface.compile_function("hot_path_benchmark", "").unwrap();
    
    println!("=== Hot Path Optimization Benchmark ===");
    
    // Phase 1: Execute without optimization
    let warmup_executions = 100;
    let benchmark_executions = 1000;
    
    // Warmup executions
    for _ in 0..warmup_executions {
        interface.execute_function("hot_path_benchmark").unwrap();
    }
    
    // Measure execution time before optimization
    let start_time = Instant::now();
    for _ in 0..benchmark_executions {
        interface.execute_function("hot_path_benchmark").unwrap();
    }
    let pre_optimization_time = start_time.elapsed();
    
    println!("Pre-optimization executions: {}", benchmark_executions);
    println!("Pre-optimization time: {:?}", pre_optimization_time);
    println!("Pre-optimization avg time: {:?}", pre_optimization_time / benchmark_executions);
    
    // Phase 2: Trigger optimization
    let optimization_start = Instant::now();
    let optimized_count = interface.optimize_hot_paths().unwrap();
    let optimization_time = optimization_start.elapsed();
    
    println!("Functions optimized: {}", optimized_count);
    println!("Optimization time: {:?}", optimization_time);
    
    // Phase 3: Execute with optimization
    let start_time = Instant::now();
    for _ in 0..benchmark_executions {
        interface.execute_function("hot_path_benchmark").unwrap();
    }
    let post_optimization_time = start_time.elapsed();
    
    println!("Post-optimization executions: {}", benchmark_executions);
    println!("Post-optimization time: {:?}", post_optimization_time);
    println!("Post-optimization avg time: {:?}", post_optimization_time / benchmark_executions);
    
    // Calculate improvement
    let improvement_ratio = pre_optimization_time.as_nanos() as f64 / post_optimization_time.as_nanos() as f64;
    let improvement_percent = (improvement_ratio - 1.0) * 100.0;
    
    println!("Performance improvement: {:.2}% ({}x faster)", improvement_percent, improvement_ratio);
    
    // Verify statistics
    let stats = interface.get_stats();
    assert!(stats.hot_path_optimizations > 0, "Should have performed hot path optimizations");
}

/// Memory usage benchmark for JIT compilation
#[test]
#[ignore] // Ignore for regular test runs
fn test_jit_memory_usage_benchmark() {
    use cursed::runtime::jit_runtime::JitMemoryManager;
    
    let memory_manager = JitMemoryManager::new(100 * 1024 * 1024, true); // 100MB limit
    
    println!("=== JIT Memory Usage Benchmark ===");
    
    let function_count = 1000;
    let function_size = 10 * 1024; // 10KB per function
    
    let start_time = Instant::now();
    
    // Allocate memory for many functions
    for i in 0..function_count {
        let function_name = format!("memory_benchmark_{}", i);
        let result = memory_manager.allocate(&function_name, function_size);
        assert!(result.is_ok(), "Memory allocation should succeed for function {}", i);
    }
    
    let allocation_time = start_time.elapsed();
    
    println!("Functions allocated: {}", function_count);
    println!("Total memory allocated: {} MB", memory_manager.get_total_allocated() / (1024 * 1024));
    println!("Allocation time: {:?}", allocation_time);
    println!("Average allocation time: {:?}", allocation_time / function_count);
    
    // Test memory deallocation performance
    let start_time = Instant::now();
    
    for i in 0..function_count {
        let function_name = format!("memory_benchmark_{}", i);
        memory_manager.deallocate(&function_name).unwrap();
    }
    
    let deallocation_time = start_time.elapsed();
    
    println!("Deallocation time: {:?}", deallocation_time);
    println!("Average deallocation time: {:?}", deallocation_time / function_count);
    println!("Final memory usage: {} bytes", memory_manager.get_total_allocated());
    
    // Performance assertions
    assert!(allocation_time < Duration::from_secs(5), "Allocation should be fast");
    assert!(deallocation_time < Duration::from_secs(1), "Deallocation should be fast");
    assert_eq!(memory_manager.get_total_allocated(), 0, "All memory should be deallocated");
}

/// Scalability test for concurrent JIT operations
#[test]
#[ignore] // Ignore for regular test runs
fn test_jit_concurrent_scalability_benchmark() {
    use std::thread;
    use std::sync::atomic::{AtomicU64, Ordering};
    
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let jit_runtime = Arc::new(JitRuntime::new_with_default_config(jit_interface, runtime));
    
    // Compile function for concurrent execution
    jit_runtime.compile_function("concurrent_benchmark", "").unwrap();
    
    println!("=== JIT Concurrent Scalability Benchmark ===");
    
    let thread_counts = vec![1, 2, 4, 8];
    let executions_per_thread = 1000;
    
    for thread_count in thread_counts {
        let total_executions = Arc::new(AtomicU64::new(0));
        let start_time = Instant::now();
        
        let handles: Vec<_> = (0..thread_count).map(|thread_id| {
            let jit_runtime_clone = jit_runtime.clone();
            let total_executions_clone = total_executions.clone();
            
            thread::spawn(move || {
                for _ in 0..executions_per_thread {
                    let result = jit_runtime_clone.execute_function("concurrent_benchmark");
                    if result.is_ok() {
                        total_executions_clone.fetch_add(1, Ordering::Relaxed);
                    }
                }
            })
        }).collect();
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        let elapsed_time = start_time.elapsed();
        let successful_executions = total_executions.load(Ordering::Relaxed);
        let throughput = successful_executions as f64 / elapsed_time.as_secs_f64();
        
        println!("Threads: {}, Executions: {}, Time: {:?}, Throughput: {:.0} ops/sec", 
                thread_count, successful_executions, elapsed_time, throughput);
        
        assert_eq!(successful_executions, (thread_count * executions_per_thread) as u64, 
                  "All executions should succeed");
    }
}

/// Compilation cache performance benchmark
#[test]
#[ignore] // Ignore for regular test runs
fn test_jit_cache_performance_benchmark() {
    let context = Context::create();
    let mut engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    
    println!("=== JIT Cache Performance Benchmark ===");
    
    let function_count = 100;
    let cache_test_iterations = 10;
    
    // Phase 1: Initial compilation (cache misses)
    let start_time = Instant::now();
    for i in 0..function_count {
        let function_name = format!("cache_benchmark_{}", i);
        engine.compile_function(&function_name, "").unwrap();
    }
    let initial_compilation_time = start_time.elapsed();
    
    let stats_after_initial = engine.get_stats();
    println!("Initial compilation: {} functions in {:?}", function_count, initial_compilation_time);
    println!("Cache misses: {}", stats_after_initial.cache_misses);
    
    // Phase 2: Re-compilation attempts (should be cache hits)
    let start_time = Instant::now();
    for _ in 0..cache_test_iterations {
        for i in 0..function_count {
            let function_name = format!("cache_benchmark_{}", i);
            engine.compile_function(&function_name, "").unwrap();
        }
    }
    let cache_hit_time = start_time.elapsed();
    
    let final_stats = engine.get_stats();
    let cache_hits = final_stats.cache_hits - stats_after_initial.cache_hits;
    
    println!("Cache hit tests: {} attempts in {:?}", 
            cache_test_iterations * function_count, cache_hit_time);
    println!("Cache hits: {}", cache_hits);
    
    let avg_cache_hit_time = cache_hit_time / (cache_test_iterations * function_count);
    let avg_initial_compilation_time = initial_compilation_time / function_count;
    
    println!("Average initial compilation time: {:?}", avg_initial_compilation_time);
    println!("Average cache hit time: {:?}", avg_cache_hit_time);
    
    let cache_speedup = avg_initial_compilation_time.as_nanos() as f64 / avg_cache_hit_time.as_nanos() as f64;
    println!("Cache speedup: {:.2}x", cache_speedup);
    
    // Performance assertions
    assert!(cache_hits > 0, "Should have cache hits");
    assert!(cache_speedup > 1.0, "Cache should provide speedup");
    assert!(avg_cache_hit_time < avg_initial_compilation_time, "Cache hits should be faster");
}

/// Performance test for different optimization levels
#[test]
#[ignore] // Ignore for regular test runs  
fn test_jit_optimization_level_performance_benchmark() {
    let context = Context::create();
    
    println!("=== JIT Optimization Level Performance Benchmark ===");
    
    let optimization_levels = ["none", "less", "default", "aggressive"];
    let function_count = 50;
    let execution_count = 1000;
    
    for opt_level in &optimization_levels {
        let mut engine = CursedJitEngine::new(&context, cursed::codegen::llvm::jit_engine::JitEngineConfig {
            optimization_level: cursed::config::parse_optimization_level(opt_level).unwrap(),
            enable_function_cache: true,
            enable_performance_monitoring: true,
            max_cached_functions: 1000,
            enable_debug_info: false,
            target_cpu: None,
            target_features: Vec::new(),
        }).unwrap();
        
        // Compilation benchmark
        let start_time = Instant::now();
        for i in 0..function_count {
            let function_name = format!("opt_benchmark_{}_{}", opt_level, i);
            engine.compile_function(&function_name, "").unwrap();
        }
        let compilation_time = start_time.elapsed();
        
        // Execution benchmark
        let function_name = format!("opt_benchmark_{}_0", opt_level);
        let start_time = Instant::now();
        for _ in 0..execution_count {
            engine.execute_function(&function_name).unwrap();
        }
        let execution_time = start_time.elapsed();
        
        println!("Optimization level: {}", opt_level);
        println!("  Compilation time: {:?} ({:?} avg)", 
                compilation_time, compilation_time / function_count);
        println!("  Execution time: {:?} ({:?} avg)", 
                execution_time, execution_time / execution_count);
        println!("  Compilations/sec: {:.1}", 
                function_count as f64 / compilation_time.as_secs_f64());
        println!("  Executions/sec: {:.0}", 
                execution_count as f64 / execution_time.as_secs_f64());
        println!();
    }
}

/// Memory pressure performance test
#[test]
#[ignore] // Ignore for regular test runs
fn test_jit_memory_pressure_performance_benchmark() {
    use cursed::runtime::jit_runtime::JitMemoryManager;
    
    println!("=== JIT Memory Pressure Performance Benchmark ===");
    
    // Test with different memory limits
    let memory_limits = vec![
        1 * 1024 * 1024,    // 1MB - high pressure
        10 * 1024 * 1024,   // 10MB - medium pressure  
        100 * 1024 * 1024,  // 100MB - low pressure
    ];
    
    let function_size = 50 * 1024; // 50KB per function
    
    for memory_limit in memory_limits {
        let memory_manager = JitMemoryManager::new(memory_limit, true);
        let max_functions = memory_limit / function_size;
        
        println!("Memory limit: {} MB", memory_limit / (1024 * 1024));
        println!("Expected max functions: {}", max_functions);
        
        let start_time = Instant::now();
        let mut allocated_functions = 0;
        
        // Allocate until memory limit is reached
        for i in 0..max_functions * 2 {
            let function_name = format!("pressure_test_{}", i);
            let result = memory_manager.allocate(&function_name, function_size);
            
            if result.is_ok() {
                allocated_functions += 1;
            } else {
                break; // Memory limit reached
            }
        }
        
        let allocation_time = start_time.elapsed();
        
        println!("Functions allocated: {}", allocated_functions);
        println!("Allocation time: {:?}", allocation_time);
        println!("Memory usage: {} MB", memory_manager.get_total_allocated() / (1024 * 1024));
        
        // Test GC trigger performance under pressure
        let gc_start = Instant::now();
        memory_manager.trigger_gc_if_needed().unwrap();
        let gc_time = gc_start.elapsed();
        
        println!("GC trigger time: {:?}", gc_time);
        
        // Performance assertions
        assert!(allocated_functions > 0, "Should allocate some functions");
        assert!(allocation_time < Duration::from_secs(10), "Allocation should be reasonably fast");
        assert!(gc_time < Duration::from_millis(100), "GC trigger should be fast");
        
        println!();
    }
}

/// Hot path detection performance test  
#[test]
#[ignore] // Ignore for regular test runs
fn test_jit_hot_path_detection_performance_benchmark() {
    use cursed::runtime::jit_runtime::JitPerformanceMonitor;
    
    println!("=== JIT Hot Path Detection Performance Benchmark ===");
    
    let monitor = JitPerformanceMonitor::new(true);
    let function_count = 1000;
    let executions_per_function = 100;
    
    let start_time = Instant::now();
    
    // Record performance data for many functions
    for i in 0..function_count {
        let function_name = format!("detection_test_{}", i);
        
        for j in 0..executions_per_function {
            // Simulate varying execution times
            let execution_time = Duration::from_micros(10 + (i * j) % 100);
            monitor.record_execution(&function_name, execution_time);
        }
    }
    
    let recording_time = start_time.elapsed();
    
    // Test optimization opportunity detection
    let detection_start = Instant::now();
    let opportunities = monitor.get_optimization_opportunities();
    let detection_time = detection_start.elapsed();
    
    println!("Functions monitored: {}", function_count);
    println!("Total executions recorded: {}", function_count * executions_per_function);
    println!("Recording time: {:?}", recording_time);
    println!("Detection time: {:?}", detection_time);
    println!("Optimization opportunities found: {}", opportunities.len());
    
    let avg_recording_time = recording_time / (function_count * executions_per_function);
    println!("Average recording time per execution: {:?}", avg_recording_time);
    
    // Performance assertions
    assert!(avg_recording_time < Duration::from_micros(10), 
           "Recording should be very fast, got {:?}", avg_recording_time);
    assert!(detection_time < Duration::from_millis(100), 
           "Detection should be fast, got {:?}", detection_time);
}

/// End-to-end performance benchmark
#[test]
#[ignore] // Ignore for regular test runs
fn test_jit_end_to_end_performance_benchmark() {
    println!("=== JIT End-to-End Performance Benchmark ===");
    
    let context = Context::create();
    let jit_engine = CursedJitEngine::new_with_default_config(&context).unwrap();
    let codegen = LlvmCodeGenerator::new().unwrap();
    let jit_interface = JitCompilationInterface::new_with_default_config(&context, jit_engine, codegen);
    
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    jit_runtime.initialize().unwrap();
    
    let function_count = 100;
    let executions_per_function = 100;
    
    let start_time = Instant::now();
    
    // Phase 1: Compilation
    let compilation_start = Instant::now();
    for i in 0..function_count {
        let function_name = format!("e2e_benchmark_{}", i);
        jit_runtime.compile_function(&function_name, "").unwrap();
    }
    let compilation_time = compilation_start.elapsed();
    
    // Phase 2: Execution
    let execution_start = Instant::now();
    for i in 0..function_count {
        let function_name = format!("e2e_benchmark_{}", i);
        for _ in 0..executions_per_function {
            jit_runtime.execute_function(&function_name).unwrap();
        }
    }
    let execution_time = execution_start.elapsed();
    
    // Phase 3: Optimization
    let optimization_start = Instant::now();
    let optimized_count = jit_runtime.optimize_hot_paths().unwrap();
    let optimization_time = optimization_start.elapsed();
    
    let total_time = start_time.elapsed();
    
    println!("=== End-to-End Results ===");
    println!("Functions: {}", function_count);
    println!("Executions per function: {}", executions_per_function);
    println!("Total executions: {}", function_count * executions_per_function);
    println!();
    println!("Compilation time: {:?}", compilation_time);
    println!("Execution time: {:?}", execution_time);
    println!("Optimization time: {:?}", optimization_time);
    println!("Total time: {:?}", total_time);
    println!();
    println!("Functions optimized: {}", optimized_count);
    println!("Average compilation time: {:?}", compilation_time / function_count);
    println!("Average execution time: {:?}", execution_time / (function_count * executions_per_function));
    println!("Compilations per second: {:.1}", function_count as f64 / compilation_time.as_secs_f64());
    println!("Executions per second: {:.0}", (function_count * executions_per_function) as f64 / execution_time.as_secs_f64());
    
    // Verify final statistics
    let stats = jit_runtime.get_stats();
    assert_eq!(stats.total_jit_executions, (function_count * executions_per_function) as u64);
    
    let jit_stats = jit_runtime.get_jit_stats();
    assert_eq!(jit_stats.total_jit_compilations, function_count as u64);
    
    jit_runtime.shutdown().unwrap();
}
