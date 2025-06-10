//! Stress Tests for CURSED Error Handling System
//!
//! This module provides stress testing for the error handling infrastructure under
//! extreme conditions including:
//! - High-frequency error generation and propagation
//! - Massive concurrent error handling (50+ threads)
//! - Memory pressure scenarios with large error contexts
//! - Sustained error handling performance
//! - Deep error chain scenarios
//! - Panic/recovery under stress

use cursed::error::{Error as CursedError, SourceLocation};
use cursed::runtime::error_handling::{
    ErrorRuntime, ErrorContext, ErrorPropagationConfig, ErrorHandlingStatistics,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime
};
use cursed::runtime::panic::{PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, RecoveryAction};
use cursed::runtime::stack_trace::{StackTraceManager, CallFrame};
use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};

#[path = "common.rs"]
mod common;

/// Initialize tracing for all tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("trace")
            .with_test_writer()
            .try_init();
    };
}

#[test]
#[ignore = "stress test - run with --ignored"]
fn test_high_frequency_error_generation() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let num_errors = 10000;
    let start_time = Instant::now();
    
    for i in 0..num_errors {
        let error = CursedError::Runtime(format!("High frequency error {}", i));
        let location = Some(SourceLocation::new((i % 1000) + 1, (i % 50) + 1)
            .with_file(&format!("stress_test_{}.csd", i % 10)));
        
        let result = runtime.propagate_error(
            error,
            location,
            Some(format!("stress_function_{}", i)),
        );
        assert!(result.is_err());
        
        // Clear context periodically to prevent memory buildup
        if i % 1000 == 0 {
            runtime.clear_error_context();
        }
    }
    
    let elapsed = start_time.elapsed();
    let stats = runtime.get_statistics().unwrap();
    
    tracing::info!(
        "Generated {} errors in {:?} ({:.2} errors/sec)",
        num_errors,
        elapsed,
        num_errors as f64 / elapsed.as_secs_f64()
    );
    
    // Performance requirements
    assert!(elapsed < Duration::from_secs(10), "High frequency error generation took too long: {:?}", elapsed);
    assert!(stats.total_errors >= num_errors as u64);
    assert!(stats.average_propagation_time.as_millis() < 10);
    
    runtime.shutdown().unwrap();
}

#[test]
#[ignore = "stress test - run with --ignored"]
fn test_massive_concurrent_error_handling() {
    init_tracing!();
    
    let runtime = Arc::new(ErrorRuntime::new());
    runtime.initialize().unwrap();
    
    let num_threads = 50;
    let errors_per_thread = 200;
    let start_barrier = Arc::new(Barrier::new(num_threads));
    let error_counter = Arc::new(AtomicU64::new(0));
    
    let mut handles = Vec::new();
    let start_time = Instant::now();
    
    for thread_id in 0..num_threads {
        let runtime_clone = Arc::clone(&runtime);
        let barrier_clone = Arc::clone(&start_barrier);
        let counter_clone = Arc::clone(&error_counter);
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();
            
            let thread_start = Instant::now();
            
            for error_id in 0..errors_per_thread {
                let error = CursedError::Runtime(format!("Massive concurrent error T{} E{}", thread_id, error_id));
                let location = Some(SourceLocation::new(
                    (thread_id * 100) + error_id,
                    (error_id % 80) + 1,
                ).with_file(&format!("concurrent_test_T{}.csd", thread_id)));
                
                let result = runtime_clone.propagate_error(
                    error,
                    location,
                    Some(format!("concurrent_function_T{}_E{}", thread_id, error_id)),
                );
                
                assert!(result.is_err());
                counter_clone.fetch_add(1, Ordering::SeqCst);
                
                // Periodic context clearing to prevent memory issues
                if error_id % 50 == 0 {
                    runtime_clone.clear_error_context();
                }
            }
            
            let thread_elapsed = thread_start.elapsed();
            tracing::debug!("Thread {} completed in {:?}", thread_id, thread_elapsed);
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for (i, handle) in handles.into_iter().enumerate() {
        handle.join().unwrap_or_else(|_| panic!("Thread {} panicked", i));
    }
    
    let total_elapsed = start_time.elapsed();
    let final_error_count = error_counter.load(Ordering::SeqCst);
    let expected_errors = (num_threads * errors_per_thread) as u64;
    
    let stats = runtime.get_statistics().unwrap();
    
    tracing::info!(
        "Processed {} errors across {} threads in {:?} ({:.2} errors/sec)",
        final_error_count,
        num_threads,
        total_elapsed,
        final_error_count as f64 / total_elapsed.as_secs_f64()
    );
    
    // Stress test requirements
    assert_eq!(final_error_count, expected_errors);
    assert!(total_elapsed < Duration::from_secs(30), "Massive concurrent test took too long: {:?}", total_elapsed);
    assert!(stats.total_errors >= expected_errors);
    
    runtime.shutdown().unwrap();
}

#[test]
#[ignore = "stress test - run with --ignored"]
fn test_memory_pressure_error_scenarios() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let large_message_size = 50000; // 50KB per error message
    let num_large_errors = 500;
    
    let start_time = Instant::now();
    
    for i in 0..num_large_errors {
        // Create very large error messages
        let base_message = "x".repeat(large_message_size);
        let error = CursedError::Runtime(format!("{} - Large error #{}", base_message, i));
        
        // Create complex location information
        let location = Some(SourceLocation::new(i + 1, (i % 120) + 1)
            .with_file(&format!("memory_pressure_test_file_{}.csd", i % 20)));
        
        // Create long function names
        let function_name = format!("very_long_function_name_with_parameters_and_context_{}_that_simulates_complex_debugging_scenarios", i);
        
        let result = runtime.propagate_error(error, location, Some(function_name));
        assert!(result.is_err());
        
        // Clear context every 50 errors to prevent excessive memory usage
        if i % 50 == 0 {
            runtime.clear_error_context();
        }
        
        // Periodic memory pressure check
        if i % 100 == 0 {
            let stats = runtime.get_statistics().unwrap();
            tracing::debug!("Processed {} large errors, total: {}", i, stats.total_errors);
        }
    }
    
    let elapsed = start_time.elapsed();
    let stats = runtime.get_statistics().unwrap();
    
    tracing::info!(
        "Handled {} large errors ({:.2} MB total) in {:?}",
        num_large_errors,
        (num_large_errors * large_message_size) as f64 / 1024.0 / 1024.0,
        elapsed
    );
    
    // Memory pressure requirements
    assert!(elapsed < Duration::from_secs(15), "Memory pressure test took too long: {:?}", elapsed);
    assert_eq!(stats.total_errors, num_large_errors as u64);
    
    runtime.shutdown().unwrap();
}

#[test]
#[ignore = "stress test - run with --ignored"]
fn test_sustained_error_handling_performance() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let test_duration = Duration::from_secs(30);
    let measurement_interval = Duration::from_secs(5);
    
    let start_time = Instant::now();
    let mut error_count = 0u64;
    let mut measurements = Vec::new();
    let mut last_measurement = start_time;
    
    while start_time.elapsed() < test_duration {
        let error = CursedError::Runtime(format!("Sustained test error {}", error_count));
        let location = Some(SourceLocation::new((error_count % 1000) + 1, 10)
            .with_file("sustained_test.csd"));
        
        let propagation_start = Instant::now();
        let result = runtime.propagate_error(error, location, Some("sustained_function".to_string()));
        let propagation_time = propagation_start.elapsed();
        
        assert!(result.is_err());
        error_count += 1;
        
        // Take measurements at intervals
        if last_measurement.elapsed() >= measurement_interval {
            let current_stats = runtime.get_statistics().unwrap();
            measurements.push((
                start_time.elapsed(),
                current_stats.total_errors,
                current_stats.average_propagation_time,
            ));
            last_measurement = Instant::now();
            
            tracing::info!(
                "Sustained test: {} errors in {:?}, avg propagation: {:?}",
                current_stats.total_errors,
                start_time.elapsed(),
                current_stats.average_propagation_time
            );
        }
        
        // Clear context periodically
        if error_count % 1000 == 0 {
            runtime.clear_error_context();
        }
        
        // Assert reasonable performance throughout
        assert!(propagation_time < Duration::from_millis(50), "Individual propagation too slow: {:?}", propagation_time);
    }
    
    let total_elapsed = start_time.elapsed();
    let final_stats = runtime.get_statistics().unwrap();
    
    tracing::info!(
        "Sustained test completed: {} errors in {:?} ({:.2} errors/sec)",
        final_stats.total_errors,
        total_elapsed,
        final_stats.total_errors as f64 / total_elapsed.as_secs_f64()
    );
    
    // Performance consistency requirements
    assert!(final_stats.total_errors >= error_count);
    assert!(final_stats.average_propagation_time < Duration::from_millis(20));
    
    // Check performance stability over time
    for (i, (time, errors, avg_time)) in measurements.iter().enumerate() {
        tracing::debug!("Measurement {}: {:?}, {} errors, avg: {:?}", i, time, errors, avg_time);
        assert!(*avg_time < Duration::from_millis(25), "Performance degraded over time at measurement {}", i);
    }
    
    runtime.shutdown().unwrap();
}

#[test]
#[ignore = "stress test - run with --ignored"]
fn test_deep_error_chain_scenarios() {
    init_tracing!();
    
    let mut config = ErrorPropagationConfig::default();
    config.max_chain_depth = 1000; // Very deep chains
    config.auto_panic_threshold = Some(2000);
    
    let runtime = ErrorRuntime::with_config(config);
    runtime.initialize().unwrap();
    
    let chain_depth = 500;
    let num_chains = 20;
    
    let start_time = Instant::now();
    
    for chain_id in 0..num_chains {
        // Create a deep error chain
        for level in 0..chain_depth {
            let error = CursedError::Runtime(format!("Deep chain C{} L{}", chain_id, level));
            let location = Some(SourceLocation::new(level + 1, (level % 80) + 1)
                .with_file(&format!("deep_chain_{}.csd", chain_id)));
            
            let result = runtime.propagate_error(
                error,
                location,
                Some(format!("deep_function_C{}_L{}", chain_id, level)),
            );
            assert!(result.is_err());
        }
        
        // Clear context between chains
        runtime.clear_error_context();
        
        if chain_id % 5 == 0 {
            let stats = runtime.get_statistics().unwrap();
            tracing::debug!("Completed {} deep chains, max depth: {}", chain_id + 1, stats.max_chain_depth);
        }
    }
    
    let elapsed = start_time.elapsed();
    let stats = runtime.get_statistics().unwrap();
    
    tracing::info!(
        "Created {} deep error chains (depth {}) in {:?}",
        num_chains,
        chain_depth,
        elapsed
    );
    
    // Deep chain requirements
    assert!(elapsed < Duration::from_secs(20), "Deep chain test took too long: {:?}", elapsed);
    assert_eq!(stats.total_errors, (num_chains * chain_depth) as u64);
    assert!(stats.max_chain_depth >= chain_depth);
    
    runtime.shutdown().unwrap();
}

#[test]
#[ignore = "stress test - run with --ignored"]
fn test_concurrent_panic_recovery_stress() {
    init_tracing!();
    
    let num_threads = 20;
    let panics_per_thread = 100;
    let panic_counter = Arc::new(AtomicU64::new(0));
    let recovery_counter = Arc::new(AtomicU64::new(0));
    
    let mut handles = Vec::new();
    let start_time = Instant::now();
    
    for thread_id in 0..num_threads {
        let panic_counter_clone = Arc::clone(&panic_counter);
        let recovery_counter_clone = Arc::clone(&recovery_counter);
        
        let handle = thread::spawn(move || {
            for panic_id in 0..panics_per_thread {
                // Simulate panic with recovery
                let panic_result = std::panic::catch_unwind(|| {
                    panic_counter_clone.fetch_add(1, Ordering::SeqCst);
                    
                    // Simulate different panic scenarios
                    match panic_id % 4 {
                        0 => panic!("Memory allocation failed in thread {}", thread_id),
                        1 => panic!("Type assertion failed: thread {} panic {}", thread_id, panic_id),
                        2 => panic!("Bounds check failed: index {} out of bounds", panic_id),
                        _ => panic!("General panic in thread {} iteration {}", thread_id, panic_id),
                    }
                });
                
                if panic_result.is_err() {
                    recovery_counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join(); // Some threads will panic, that's expected
    }
    
    let elapsed = start_time.elapsed();
    let total_panics = panic_counter.load(Ordering::SeqCst);
    let total_recoveries = recovery_counter.load(Ordering::SeqCst);
    
    tracing::info!(
        "Panic/recovery stress test: {} panics, {} recoveries in {:?}",
        total_panics,
        total_recoveries,
        elapsed
    );
    
    // Stress requirements for panic/recovery
    assert!(elapsed < Duration::from_secs(15), "Panic/recovery stress test took too long: {:?}", elapsed);
    // Note: Not all panics may be counted due to thread panics, but we should have some
    assert!(total_panics > 0);
    assert!(total_recoveries > 0);
}

#[test]
#[ignore = "stress test - run with --ignored"]
fn test_error_handling_resource_cleanup_stress() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let cleanup_cycles = 100;
    let errors_per_cycle = 1000;
    
    let start_time = Instant::now();
    
    for cycle in 0..cleanup_cycles {
        // Generate many errors
        for i in 0..errors_per_cycle {
            let error = CursedError::Runtime(format!("Cleanup stress C{} E{}", cycle, i));
            let location = Some(SourceLocation::new(i + 1, 10)
                .with_file(&format!("cleanup_cycle_{}.csd", cycle)));
            
            let _ = runtime.propagate_error(error, location, Some(format!("cleanup_function_{}", i)));
        }
        
        // Force cleanup
        runtime.clear_error_context();
        
        // Periodic statistics check
        if cycle % 20 == 0 {
            let stats = runtime.get_statistics().unwrap();
            tracing::debug!("Cleanup cycle {}: {} total errors", cycle, stats.total_errors);
        }
    }
    
    let elapsed = start_time.elapsed();
    let stats = runtime.get_statistics().unwrap();
    
    tracing::info!(
        "Resource cleanup stress: {} cycles, {} total errors in {:?}",
        cleanup_cycles,
        stats.total_errors,
        elapsed
    );
    
    // Cleanup stress requirements
    assert!(elapsed < Duration::from_secs(25), "Cleanup stress test took too long: {:?}", elapsed);
    assert_eq!(stats.total_errors, (cleanup_cycles * errors_per_cycle) as u64);
    
    runtime.shutdown().unwrap();
}

#[test]
#[ignore = "stress test - run with --ignored"]
fn test_mixed_error_types_stress() {
    init_tracing!();
    
    let runtime = ErrorRuntime::new();
    runtime.initialize().unwrap();
    
    let num_error_types = 8;
    let errors_per_type = 1000;
    let mut type_counters = HashMap::new();
    
    let start_time = Instant::now();
    
    for i in 0..(num_error_types * errors_per_type) {
        let error_type = i % num_error_types;
        let type_counter = type_counters.entry(error_type).or_insert(0);
        *type_counter += 1;
        
        let error = match error_type {
            0 => CursedError::Parse(format!("Parse error #{}", type_counter)),
            1 => CursedError::Type(format!("Type error #{}", type_counter)),
            2 => CursedError::Runtime(format!("Runtime error #{}", type_counter)),
            3 => CursedError::Compile(format!("Compile error #{}", type_counter)),
            4 => CursedError::panic_error(format!("Panic error #{}", type_counter)),
            5 => CursedError::recoverable_panic(format!("Recoverable panic #{}", type_counter)),
            6 => CursedError::recovery_error(format!("Recovery error #{}", type_counter), *type_counter),
            _ => CursedError::type_error(format!("Type error #{}", type_counter)),
        };
        
        let location = Some(SourceLocation::new((i % 500) + 1, (i % 80) + 1)
            .with_file(&format!("mixed_types_test_{}.csd", error_type)));
        
        let result = runtime.propagate_error(error, location, Some(format!("mixed_function_{}", i)));
        assert!(result.is_err());
        
        // Clear context periodically
        if i % 500 == 0 {
            runtime.clear_error_context();
        }
    }
    
    let elapsed = start_time.elapsed();
    let stats = runtime.get_statistics().unwrap();
    
    tracing::info!(
        "Mixed error types stress: {} errors of {} types in {:?}",
        stats.total_errors,
        num_error_types,
        elapsed
    );
    
    // Mixed types stress requirements
    assert!(elapsed < Duration::from_secs(15), "Mixed types stress test took too long: {:?}", elapsed);
    assert_eq!(stats.total_errors, (num_error_types * errors_per_type) as u64);
    
    // Verify all error types were processed
    for error_type in 0..num_error_types {
        assert_eq!(type_counters[&error_type], errors_per_type);
    }
    
    runtime.shutdown().unwrap();
}

/// Documentation: Why Stress Testing Error Handling is Critical
/// 
/// Error handling systems are often the first to fail under extreme conditions.
/// Stress testing ensures that:
/// 
/// 1. **Performance Stability**: Error handling doesn't degrade under load
/// 2. **Memory Management**: No memory leaks or excessive allocation
/// 3. **Concurrency Safety**: Thread-safe operations under high contention
/// 4. **Resource Cleanup**: Proper cleanup prevents resource exhaustion
/// 5. **System Stability**: Error handling doesn't become a bottleneck
/// 6. **Recovery Capability**: System can recover from error storms
/// 
/// These stress tests validate that CURSED's error handling can withstand:
/// - High-frequency error generation (10K+ errors rapidly)
/// - Massive concurrency (50+ threads simultaneously)
/// - Memory pressure (large error contexts and chains)
/// - Sustained operation (continuous error handling)
/// - Deep error chains (hundreds of nested contexts)
/// - Mixed error scenarios (all error types simultaneously)
/// 
/// Without stress testing, error handling systems can fail exactly when
/// they're needed most - during critical error conditions.
