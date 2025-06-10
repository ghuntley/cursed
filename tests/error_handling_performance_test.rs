//! Performance and Stress Tests for CURSED Error Handling System
//!
//! This module provides comprehensive performance testing for error handling:
//! - Benchmark error handling overhead
//! - Test memory usage during error scenarios
//! - Test panic recovery performance
//! - Stress test error propagation chains
//! - Multi-threaded performance testing
//! - Memory pressure scenarios

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::error_handling::{
    ErrorRuntime, ErrorContext, ErrorPropagationConfig, ErrorHandlingStatistics,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime
};
use cursed::runtime::panic::{PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory};
use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[path = "common.rs"]
mod common;

/// Initialize tracing for all tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .with_test_writer()
            .try_init();
    };
}

/// Benchmark configuration for performance tests
#[derive(Debug, Clone)]
struct BenchmarkConfig {
    /// Number of iterations for each benchmark
    iterations: usize,
    /// Number of threads for concurrent tests
    thread_count: usize,
    /// Size of error messages for memory tests
    message_size: usize,
    /// Maximum acceptable operation time
    max_operation_time: Duration,
    /// Warmup iterations before measurement
    warmup_iterations: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        BenchmarkConfig {
            iterations: 10000,
            thread_count: 8,
            message_size: 1000,
            max_operation_time: Duration::from_micros(100),
            warmup_iterations: 1000,
        }
    }
}

/// Performance measurement results
#[derive(Debug, Clone)]
struct PerformanceResults {
    /// Total time for all operations
    total_time: Duration,
    /// Average time per operation
    average_time: Duration,
    /// Minimum operation time
    min_time: Duration,
    /// Maximum operation time
    max_time: Duration,
    /// Operations per second
    operations_per_second: f64,
    /// Memory usage peak (approximate)
    peak_memory_usage: usize,
}

impl PerformanceResults {
    fn new(times: &[Duration], total_time: Duration) -> Self {
        let average_time = total_time / times.len() as u32;
        let min_time = *times.iter().min().unwrap_or(&Duration::ZERO);
        let max_time = *times.iter().max().unwrap_or(&Duration::ZERO);
        let operations_per_second = times.len() as f64 / total_time.as_secs_f64();
        
        PerformanceResults {
            total_time,
            average_time,
            min_time,
            max_time,
            operations_per_second,
            peak_memory_usage: 0, // Would need actual memory measurement
        }
    }
}

#[test]
fn test_basic_error_propagation_performance() {
    init_tracing!();
    
    let config = BenchmarkConfig::default();
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Warmup
    for _ in 0..config.warmup_iterations {
        let error = CursedError::Runtime("warmup error".to_string());
        let _ = runtime.propagate_error(error, None, None);
    }
    runtime.clear_error_context();
    
    // Measure error propagation performance
    let mut operation_times = Vec::new();
    let start_time = Instant::now();
    
    for i in 0..config.iterations {
        let op_start = Instant::now();
        
        let error = CursedError::Runtime(format!("performance test error {}", i));
        let location = Some(SourceLocation::new(i % 1000, 5));
        let result = runtime.propagate_error(error, location, None);
        
        let op_time = op_start.elapsed();
        operation_times.push(op_time);
        
        assert!(result.is_err());
        
        // Clear context periodically to prevent buildup
        if i % 100 == 0 {
            runtime.clear_error_context();
        }
    }
    
    let total_time = start_time.elapsed();
    let results = PerformanceResults::new(&operation_times, total_time);
    
    // Performance assertions
    assert!(results.average_time < config.max_operation_time, 
            "Average operation time {} exceeds maximum {}", 
            results.average_time.as_micros(), 
            config.max_operation_time.as_micros());
    
    assert!(results.operations_per_second > 10000.0, 
            "Operations per second {} is too low", 
            results.operations_per_second);
    
    tracing::info!("Basic error propagation performance:");
    tracing::info!("  Total time: {:?}", results.total_time);
    tracing::info!("  Average time: {:?}", results.average_time);
    tracing::info!("  Min/Max time: {:?} / {:?}", results.min_time, results.max_time);
    tracing::info!("  Operations/sec: {:.2}", results.operations_per_second);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_concurrent_error_handling_performance() {
    init_tracing!();
    
    let config = BenchmarkConfig::default();
    let runtime = Arc::new(ErrorRuntime::new());
    runtime.initialize().unwrap();
    
    let barrier = Arc::new(Barrier::new(config.thread_count));
    let total_operations = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();
    
    let start_time = Instant::now();
    
    for thread_id in 0..config.thread_count {
        let runtime_clone = Arc::clone(&runtime);
        let barrier_clone = Arc::clone(&barrier);
        let total_ops = Arc::clone(&total_operations);
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();
            
            let mut thread_operations = 0;
            let iterations_per_thread = config.iterations / config.thread_count;
            
            for i in 0..iterations_per_thread {
                let error = CursedError::Runtime(
                    format!("thread {} concurrent error {}", thread_id, i)
                );
                let location = Some(SourceLocation::new(
                    thread_id * 1000 + i,
                    thread_id + 1,
                ));
                
                let result = runtime_clone.propagate_error(error, location, None);
                assert!(result.is_err());
                
                thread_operations += 1;
                
                // Clear context periodically
                if i % 50 == 0 {
                    runtime_clone.clear_error_context();
                }
            }
            
            total_ops.fetch_add(thread_operations, Ordering::SeqCst);
            thread_operations
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut total_thread_ops = 0;
    for handle in handles {
        total_thread_ops += handle.join().unwrap();
    }
    
    let total_time = start_time.elapsed();
    let total_ops = total_operations.load(Ordering::SeqCst);
    
    assert_eq!(total_ops, total_thread_ops as u64);
    
    let ops_per_second = total_ops as f64 / total_time.as_secs_f64();
    let avg_time_per_op = total_time / total_ops as u32;
    
    // Performance assertions for concurrent scenario
    assert!(ops_per_second > 50000.0, 
            "Concurrent operations per second {} is too low", 
            ops_per_second);
    
    assert!(avg_time_per_op < Duration::from_micros(200), 
            "Average concurrent operation time {} is too high", 
            avg_time_per_op.as_micros());
    
    tracing::info!("Concurrent error handling performance:");
    tracing::info!("  Threads: {}", config.thread_count);
    tracing::info!("  Total operations: {}", total_ops);
    tracing::info!("  Total time: {:?}", total_time);
    tracing::info!("  Average time per op: {:?}", avg_time_per_op);
    tracing::info!("  Operations/sec: {:.2}", ops_per_second);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_large_error_message_performance() {
    init_tracing!();
    
    let config = BenchmarkConfig {
        iterations: 1000, // Fewer iterations for large messages
        message_size: 10000, // 10KB messages
        ..Default::default()
    };
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Create large error messages
    let large_message = "x".repeat(config.message_size);
    let start_time = Instant::now();
    
    for i in 0..config.iterations {
        let error = CursedError::Runtime(format!("{} - error {}", large_message, i));
        let location = Some(SourceLocation::new(i, 10).with_file("large_message_test.csd"));
        
        let result = runtime.propagate_error(error, location, None);
        assert!(result.is_err());
        
        // Clear context more frequently for large messages
        if i % 10 == 0 {
            runtime.clear_error_context();
        }
    }
    
    let total_time = start_time.elapsed();
    let avg_time = total_time / config.iterations as u32;
    let ops_per_second = config.iterations as f64 / total_time.as_secs_f64();
    
    // Performance should still be reasonable with large messages
    assert!(avg_time < Duration::from_millis(5), 
            "Large message handling too slow: {:?}", avg_time);
    
    assert!(ops_per_second > 200.0, 
            "Large message operations per second too low: {:.2}", 
            ops_per_second);
    
    tracing::info!("Large error message performance:");
    tracing::info!("  Message size: {} bytes", config.message_size);
    tracing::info!("  Total time: {:?}", total_time);
    tracing::info!("  Average time: {:?}", avg_time);
    tracing::info!("  Operations/sec: {:.2}", ops_per_second);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_error_chain_depth_performance() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let chain_depths = [1, 10, 50, 100, 200];
    
    for &depth in &chain_depths {
        let start_time = Instant::now();
        
        // Create error chain of specified depth
        for i in 0..depth {
            let error = CursedError::Runtime(format!("Chain error level {}", i));
            let location = Some(SourceLocation::new(i + 1, 5));
            let result = runtime.propagate_error(
                error, 
                location, 
                Some(format!("chain_function_{}", i))
            );
            assert!(result.is_err());
        }
        
        let chain_time = start_time.elapsed();
        let time_per_level = chain_time / depth as u32;
        
        // Verify context exists and has expected depth
        if let Some(context) = runtime.get_current_error_context() {
            assert!(!context.error_chain.is_empty());
        }
        
        tracing::info!("Error chain depth {} performance:", depth);
        tracing::info!("  Total time: {:?}", chain_time);
        tracing::info!("  Time per level: {:?}", time_per_level);
        
        // Performance should scale reasonably with depth
        assert!(time_per_level < Duration::from_micros(500), 
                "Error chain level time too high for depth {}: {:?}", 
                depth, time_per_level);
        
        runtime.clear_error_context();
    }
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_memory_pressure_performance() {
    init_tracing!();
    
    let config = BenchmarkConfig {
        iterations: 5000,
        ..Default::default()
    };
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    // Test performance under memory pressure
    let large_context_data = "context_data".repeat(1000); // Large context
    let start_time = Instant::now();
    
    for i in 0..config.iterations {
        // Create error with large context
        let mut context = ErrorContext::new()
            .with_location(SourceLocation::new(i, 10).with_file("memory_pressure.csd"))
            .with_metadata("large_data".to_string(), large_context_data.clone())
            .with_metadata("iteration".to_string(), i.to_string());
        
        // Add multiple chain entries
        for j in 0..5 {
            context.add_to_chain(
                format!("Memory pressure error {} level {}", i, j),
                Some(SourceLocation::new(i + j, 15)),
                Some(format!("pressure_function_{}_{}", i, j)),
            );
        }
        
        let error = CursedError::Runtime(format!("Memory pressure test {}", i));
        let result = runtime.propagate_error(error, None, None);
        assert!(result.is_err());
        
        // Force cleanup every 100 iterations to test memory management
        if i % 100 == 0 {
            runtime.clear_error_context();
        }
    }
    
    let total_time = start_time.elapsed();
    let avg_time = total_time / config.iterations as u32;
    let ops_per_second = config.iterations as f64 / total_time.as_secs_f64();
    
    // Performance should remain reasonable under memory pressure
    assert!(avg_time < Duration::from_millis(1), 
            "Memory pressure handling too slow: {:?}", avg_time);
    
    assert!(ops_per_second > 1000.0, 
            "Memory pressure operations per second too low: {:.2}", 
            ops_per_second);
    
    tracing::info!("Memory pressure performance:");
    tracing::info!("  Total time: {:?}", total_time);
    tracing::info!("  Average time: {:?}", avg_time);
    tracing::info!("  Operations/sec: {:.2}", ops_per_second);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_error_statistics_performance() {
    init_tracing!();
    
    let config = BenchmarkConfig {
        iterations: 10000,
        ..Default::default()
    };
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let start_time = Instant::now();
    
    for i in 0..config.iterations {
        let error = CursedError::Runtime(format!("statistics test error {}", i));
        let _ = runtime.propagate_error(error, None, None);
        
        // Periodically check statistics (this should be fast)
        if i % 100 == 0 {
            let stats_start = Instant::now();
            let stats = runtime.get_statistics().unwrap();
            let stats_time = stats_start.elapsed();
            
            assert!(stats_time < Duration::from_micros(100), 
                    "Statistics retrieval too slow: {:?}", stats_time);
            
            assert_eq!(stats.total_errors, (i + 1) as u64);
        }
        
        runtime.clear_error_context();
    }
    
    let total_time = start_time.elapsed();
    let final_stats = runtime.get_statistics().unwrap();
    
    assert_eq!(final_stats.total_errors, config.iterations as u64);
    assert!(final_stats.average_propagation_time.as_micros() > 0);
    
    tracing::info!("Error statistics performance:");
    tracing::info!("  Total time: {:?}", total_time);
    tracing::info!("  Final stats: {:?}", final_stats);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_global_runtime_performance() {
    init_tracing!();
    
    // Initialize global runtime
    if get_error_runtime().is_none() {
        let _ = initialize_error_runtime();
    }
    
    let config = BenchmarkConfig {
        iterations: 5000,
        ..Default::default()
    };
    
    let start_time = Instant::now();
    
    for i in 0..config.iterations {
        if let Some(global_runtime) = get_error_runtime() {
            let error = CursedError::Runtime(format!("global runtime test {}", i));
            let location = Some(SourceLocation::new(i, 5));
            
            let result = global_runtime.propagate_error(error, location, None);
            assert!(result.is_err());
            
            if i % 50 == 0 {
                global_runtime.clear_error_context();
            }
        }
    }
    
    let total_time = start_time.elapsed();
    let avg_time = total_time / config.iterations as u32;
    let ops_per_second = config.iterations as f64 / total_time.as_secs_f64();
    
    // Global runtime should perform similarly to local runtime
    assert!(avg_time < Duration::from_micros(200), 
            "Global runtime too slow: {:?}", avg_time);
    
    assert!(ops_per_second > 5000.0, 
            "Global runtime operations per second too low: {:.2}", 
            ops_per_second);
    
    tracing::info!("Global runtime performance:");
    tracing::info!("  Total time: {:?}", total_time);
    tracing::info!("  Average time: {:?}", avg_time);
    tracing::info!("  Operations/sec: {:.2}", ops_per_second);
    
    let _ = shutdown_error_runtime();
}

#[test]
fn test_configuration_change_performance() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let config_changes = 100;
    let operations_per_config = 100;
    
    let start_time = Instant::now();
    
    for config_id in 0..config_changes {
        // Change configuration
        let config_start = Instant::now();
        let result = runtime.update_config(|config| {
            config.max_chain_depth = 50 + (config_id % 50);
            config.capture_stack_traces = config_id % 2 == 0;
            config.log_propagation = config_id % 3 == 0;
        });
        let config_time = config_start.elapsed();
        
        assert!(result.is_ok());
        assert!(config_time < Duration::from_micros(100), 
                "Configuration change too slow: {:?}", config_time);
        
        // Test operations with new configuration
        for op_id in 0..operations_per_config {
            let error = CursedError::Runtime(
                format!("config {} operation {}", config_id, op_id)
            );
            let result = runtime.propagate_error(error, None, None);
            assert!(result.is_err());
        }
        
        runtime.clear_error_context();
    }
    
    let total_time = start_time.elapsed();
    let total_operations = config_changes * operations_per_config;
    let ops_per_second = total_operations as f64 / total_time.as_secs_f64();
    
    tracing::info!("Configuration change performance:");
    tracing::info!("  Total time: {:?}", total_time);
    tracing::info!("  Config changes: {}", config_changes);
    tracing::info!("  Total operations: {}", total_operations);
    tracing::info!("  Operations/sec: {:.2}", ops_per_second);
    
    runtime.shutdown().unwrap();
}

#[test]
fn test_stress_error_recovery_performance() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let stress_iterations = 1000;
    let recovery_cycles = 10;
    
    let start_time = Instant::now();
    
    for cycle in 0..recovery_cycles {
        // Generate many errors
        for i in 0..stress_iterations {
            let error = CursedError::Runtime(
                format!("stress recovery cycle {} error {}", cycle, i)
            );
            let result = runtime.propagate_error(error, None, None);
            assert!(result.is_err());
        }
        
        // Measure recovery time
        let recovery_start = Instant::now();
        runtime.clear_error_context();
        let recovery_time = recovery_start.elapsed();
        
        assert!(recovery_time < Duration::from_millis(10), 
                "Error context clearing too slow: {:?}", recovery_time);
        
        tracing::debug!("Recovery cycle {} completed in {:?}", cycle, recovery_time);
    }
    
    let total_time = start_time.elapsed();
    let total_operations = stress_iterations * recovery_cycles;
    let ops_per_second = total_operations as f64 / total_time.as_secs_f64();
    
    assert!(ops_per_second > 10000.0, 
            "Stress recovery operations per second too low: {:.2}", 
            ops_per_second);
    
    tracing::info!("Stress error recovery performance:");
    tracing::info!("  Total time: {:?}", total_time);
    tracing::info!("  Recovery cycles: {}", recovery_cycles);
    tracing::info!("  Total operations: {}", total_operations);
    tracing::info!("  Operations/sec: {:.2}", ops_per_second);
    
    runtime.shutdown().unwrap();
}

/// Documentation: Performance Testing for Error Handling
/// 
/// This comprehensive performance test suite validates the efficiency and
/// scalability of CURSED's error handling system under various conditions:
/// 
/// 1. **Basic Performance**: Tests fundamental error propagation speed
/// 2. **Concurrent Performance**: Validates thread-safe operations under load
/// 3. **Large Message Handling**: Tests performance with large error messages
/// 4. **Chain Depth Scaling**: Verifies performance scales with error chain depth
/// 5. **Memory Pressure**: Tests performance under high memory usage
/// 6. **Statistics Overhead**: Validates statistics tracking doesn't impact performance
/// 7. **Global Runtime**: Tests global runtime performance characteristics
/// 8. **Configuration Changes**: Tests dynamic configuration update performance
/// 9. **Stress Recovery**: Tests performance under repeated error/recovery cycles
/// 
/// Performance Targets:
/// - Basic error propagation: <100μs per operation
/// - Concurrent operations: >50,000 ops/sec across threads
/// - Large messages (10KB): >200 ops/sec
/// - Chain depth scaling: <500μs per level
/// - Memory pressure: >1,000 ops/sec with large contexts
/// - Statistics retrieval: <100μs
/// - Configuration updates: <100μs
/// - Recovery operations: <10ms
/// 
/// These tests ensure that error handling remains efficient even under
/// stress conditions and doesn't become a performance bottleneck in
/// production CURSED applications.

#[cfg(test)]
mod performance_test_utilities {
    use super::*;
    
    /// Helper to create performance test configuration
    pub fn create_perf_config(iterations: usize, thread_count: usize) -> BenchmarkConfig {
        BenchmarkConfig {
            iterations,
            thread_count,
            ..Default::default()
        }
    }
    
    /// Helper to measure operation performance
    pub fn measure_operation<T, F>(operation: F) -> (T, Duration)
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = operation();
        let elapsed = start.elapsed();
        (result, elapsed)
    }
    
    /// Helper to run performance benchmark
    pub fn run_benchmark<F>(
        name: &str,
        iterations: usize,
        operation: F,
    ) -> PerformanceResults
    where
        F: Fn(usize) -> Duration,
    {
        let mut times = Vec::new();
        let start_time = Instant::now();
        
        for i in 0..iterations {
            let op_time = operation(i);
            times.push(op_time);
        }
        
        let total_time = start_time.elapsed();
        let results = PerformanceResults::new(&times, total_time);
        
        tracing::info!("{} benchmark results:", name);
        tracing::info!("  Operations: {}", iterations);
        tracing::info!("  Total time: {:?}", results.total_time);
        tracing::info!("  Average time: {:?}", results.average_time);
        tracing::info!("  Operations/sec: {:.2}", results.operations_per_second);
        
        results
    }
    
    /// Helper to assert performance requirements
    pub fn assert_performance_requirements(
        results: &PerformanceResults,
        max_avg_time: Duration,
        min_ops_per_sec: f64,
    ) {
        assert!(results.average_time <= max_avg_time,
                "Average time {} exceeds maximum {}",
                results.average_time.as_micros(),
                max_avg_time.as_micros());
        
        assert!(results.operations_per_second >= min_ops_per_sec,
                "Operations per second {:.2} below minimum {:.2}",
                results.operations_per_second,
                min_ops_per_sec);
    }
}
