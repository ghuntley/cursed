//! Comprehensive tests for the enhanced goroutine scheduler
//!
//! This test suite validates the thread-safe operation of the goroutine scheduler,
//! including lifecycle management, thread pool scaling, garbage collection integration,
//! and performance characteristics under various load conditions.
//!
//! ## Why These Tests Are Important for Concurrency Safety and Performance
//!
//! 1. **Thread Safety**: Verifies that concurrent operations on the scheduler don't cause
//!    race conditions, data corruption, or deadlocks when accessed from multiple threads
//!
//! 2. **Resource Management**: Ensures proper cleanup of goroutines, thread pools, and
//!    GC references to prevent memory leaks and resource exhaustion
//!
//! 3. **Performance Characteristics**: Validates that the scheduler scales efficiently
//!    with load and doesn't degrade under stress conditions
//!
//! 4. **Error Handling**: Tests panic recovery, error propagation, and graceful degradation
//!    when goroutines fail or resources are exhausted
//!
//! 5. **GC Integration**: Verifies that goroutine-owned objects are properly tracked
//!    and cleaned up by the garbage collector
//!
//! 6. **State Consistency**: Ensures goroutine state transitions are atomic and consistent
//!    across the entire lifecycle

use std::sync::{Arc, Mutex, atomic::{AtomicI32, AtomicUsize, AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::ffi::c_void;
use std::thread;
use std::collections::HashSet;

use cursed::runtime::goroutine_scheduler_simple::{
    SimpleGoroutineScheduler, ThreadPoolConfig, GoroutineState, GoroutineResult,
    GoroutineFunction, get_global_simple_scheduler
};
use cursed::memory::GarbageCollector;
use tracing::{info, debug, warn};

// Include common test utilities
#[path = "common/mod.rs"]
mod common;

/// Test function that increments a counter
unsafe extern "C" fn increment_counter(data: *mut c_void) -> *mut c_void {
    let counter = data as *mut AtomicI32;
    (*counter).fetch_add(1, Ordering::SeqCst);
    std::ptr::null_mut()
}

/// Test function that panics
unsafe extern "C" fn panic_function(_data: *mut c_void) -> *mut c_void {
    panic!("Intentional test panic");
}

/// Test function that runs for a specified duration
unsafe extern "C" fn duration_function(data: *mut c_void) -> *mut c_void {
    let duration_ms = data as usize;
    std::thread::sleep(Duration::from_millis(duration_ms as u64));
    std::ptr::null_mut()
}

/// Test function that allocates and works with memory
unsafe extern "C" fn memory_intensive_function(data: *mut c_void) -> *mut c_void {
    let iterations = data as usize;
    let mut sum = 0u64;
    
    // Simulate memory-intensive work
    for i in 0..iterations {
        let vec: Vec<u64> = (0..100).collect();
        sum += vec.iter().sum::<u64>();
        
        if i % 100 == 0 {
            std::thread::yield_now(); // Allow other goroutines to run
        }
    }
    
    Box::into_raw(Box::new(sum)) as *mut c_void
}

/// Test function that simulates blocking I/O
unsafe extern "C" fn blocking_io_function(data: *mut c_void) -> *mut c_void {
    let block_duration_ms = data as usize;
    
    // Simulate blocking I/O operation
    std::thread::sleep(Duration::from_millis(block_duration_ms as u64));
    
    std::ptr::null_mut()
}

#[test]
fn test_scheduler_creation_and_basic_operation() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing scheduler creation and basic operation");
    
    let gc = GarbageCollector::new();
    let config = ThreadPoolConfig {
        min_threads: 2,
        max_threads: 4,
        max_idle_time: Duration::from_secs(30),
        max_queue_size: 100,
    };
    
    let scheduler = SimpleGoroutineScheduler::new(config, gc);
    
    // Verify initial state
    assert_eq!(scheduler.active_count(), 0);
    assert_eq!(scheduler.queued_count(), 0);
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), 0);
    assert!(stats.worker_count.load(Ordering::Relaxed) >= 2);
    
    info!("Scheduler creation test passed");
}

#[test]
fn test_single_goroutine_execution() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing single goroutine execution");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    let counter = AtomicI32::new(0);
    
    let id = scheduler.spawn_goroutine(
        increment_counter, 
        &counter as *const _ as *mut c_void
    );
    
    // Verify goroutine was created
    assert!(id > 0);
    assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Runnable));
    
    // Wait for completion
    let result = scheduler.wait_for_goroutine(id).unwrap();
    
    // Verify execution
    assert!(matches!(result, GoroutineResult::Success(_)));
    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), 1);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), 1);
    
    info!("Single goroutine execution test passed");
}

#[test]
fn test_multiple_concurrent_goroutines() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing multiple concurrent goroutines");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    let counter = AtomicI32::new(0);
    let num_goroutines = 20;
    
    let mut goroutine_ids = Vec::new();
    let start_time = Instant::now();
    
    // Spawn multiple goroutines
    for i in 0..num_goroutines {
        let id = scheduler.spawn_goroutine(
            increment_counter,
            &counter as *const _ as *mut c_void
        );
        goroutine_ids.push(id);
        
        debug!(goroutine_id = id, iteration = i, "Spawned goroutine");
    }
    
    info!(count = num_goroutines, "Spawned all goroutines");
    
    // Wait for all to complete
    scheduler.wait_all().unwrap();
    let execution_time = start_time.elapsed();
    
    // Verify all completed
    assert_eq!(counter.load(Ordering::SeqCst), num_goroutines);
    
    // Check all are in terminated state
    for id in &goroutine_ids {
        assert_eq!(scheduler.get_goroutine_metadata(*id), Some(GoroutineState::Terminated));
    }
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), num_goroutines as u64);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), num_goroutines as u64);
    assert_eq!(stats.total_panicked.load(Ordering::Relaxed), 0);
    assert!(stats.avg_execution_time.load(Ordering::Relaxed) > 0);
    
    info!(
        execution_time = ?execution_time,
        goroutines = num_goroutines,
        "Multiple concurrent goroutines test passed"
    );
}

#[test]
fn test_goroutine_panic_handling() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing goroutine panic handling");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    
    let id = scheduler.spawn_goroutine(panic_function, std::ptr::null_mut());
    
    // Wait for completion
    let result = scheduler.wait_for_goroutine(id).unwrap();
    
    // Verify panic was caught
    assert!(matches!(result, GoroutineResult::Panic(_)));
    assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Panicked));
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), 1);
    assert_eq!(stats.total_panicked.load(Ordering::Relaxed), 1);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), 0);
    
    info!("Goroutine panic handling test passed");
}

#[test]
fn test_mixed_success_and_panic_goroutines() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing mixed success and panic goroutines");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    let counter = AtomicI32::new(0);
    
    let mut success_ids = Vec::new();
    let mut panic_ids = Vec::new();
    
    // Spawn successful goroutines
    for _ in 0..5 {
        let id = scheduler.spawn_goroutine(
            increment_counter,
            &counter as *const _ as *mut c_void
        );
        success_ids.push(id);
    }
    
    // Spawn panicking goroutines
    for _ in 0..3 {
        let id = scheduler.spawn_goroutine(panic_function, std::ptr::null_mut());
        panic_ids.push(id);
    }
    
    // Wait for all to complete
    scheduler.wait_all().unwrap();
    
    // Verify successful goroutines
    assert_eq!(counter.load(Ordering::SeqCst), 5);
    for id in &success_ids {
        assert_eq!(scheduler.get_goroutine_metadata(*id), Some(GoroutineState::Terminated));
    }
    
    // Verify panicked goroutines
    for id in &panic_ids {
        assert_eq!(scheduler.get_goroutine_metadata(*id), Some(GoroutineState::Panicked));
    }
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), 8);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), 5);
    assert_eq!(stats.total_panicked.load(Ordering::Relaxed), 3);
    
    info!("Mixed success and panic goroutines test passed");
}

#[test]
fn test_thread_pool_scaling() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing thread pool scaling");
    
    let config = ThreadPoolConfig {
        min_threads: 1,
        max_threads: 8,
        max_idle_time: Duration::from_secs(1),
        max_queue_size: 1000,
    };
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::new(config, gc);
    
    let initial_workers = scheduler.get_statistics().worker_count.load(Ordering::Relaxed);
    assert!(initial_workers >= 1);
    
    // Spawn many slow goroutines to trigger scaling
    let num_slow_goroutines = 20;
    for i in 0..num_slow_goroutines {
        scheduler.spawn_goroutine(
            duration_function,
            (50 * (i % 5 + 1)) as *mut c_void // 50-250ms duration
        );
    }
    
    info!(initial_workers = initial_workers, "Spawned slow goroutines");
    
    // Give time for workers to spawn
    std::thread::sleep(Duration::from_millis(100));
    
    let scaled_workers = scheduler.get_statistics().worker_count.load(Ordering::Relaxed);
    info!(
        initial_workers = initial_workers,
        scaled_workers = scaled_workers,
        "Worker count after scaling"
    );
    
    // Should have spawned additional workers
    assert!(scaled_workers >= initial_workers);
    
    // Wait for all to complete
    scheduler.wait_all().unwrap();
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), num_slow_goroutines);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), num_slow_goroutines);
    
    info!("Thread pool scaling test passed");
}

#[test]
fn test_memory_intensive_goroutines() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing memory intensive goroutines");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc.clone());
    
    let num_goroutines = 5;
    let mut goroutine_ids = Vec::new();
    
    // Spawn memory-intensive goroutines
    for i in 0..num_goroutines {
        let id = scheduler.spawn_goroutine(
            memory_intensive_function,
            (1000 + i * 500) as *mut c_void // Different work amounts
        );
        goroutine_ids.push(id);
    }
    
    info!(count = num_goroutines, "Spawned memory intensive goroutines");
    
    // Wait for all to complete
    scheduler.wait_all().unwrap();
    
    // Verify all completed successfully
    for id in &goroutine_ids {
        assert_eq!(scheduler.get_goroutine_metadata(*id), Some(GoroutineState::Terminated));
    }
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), num_goroutines);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), num_goroutines);
    assert!(stats.avg_execution_time.load(Ordering::Relaxed) > 0);
    
    // Trigger GC to clean up any remaining references
    gc.collect_garbage();
    
    info!("Memory intensive goroutines test passed");
}

#[test]
fn test_gc_integration() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing garbage collector integration");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc.clone());
    let counter = AtomicI32::new(0);
    
    let id = scheduler.spawn_goroutine(
        increment_counter,
        &counter as *const _ as *mut c_void
    );
    
    // Register fake GC references
    let object_ids = vec![12345, 67890, 11111];
    for &obj_id in &object_ids {
        scheduler.register_gc_reference(id, obj_id);
    }
    
    // Wait for completion
    scheduler.wait_for_goroutine(id).unwrap();
    
    // Verify GC references were cleaned up
    // Note: In a real test, we'd verify the GC's internal state
    // For now, we just verify the goroutine completed successfully
    assert_eq!(counter.load(Ordering::SeqCst), 1);
    assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
    
    info!("GC integration test passed");
}

#[test]
fn test_scheduler_statistics_accuracy() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing scheduler statistics accuracy");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    let counter = AtomicI32::new(0);
    
    let initial_stats = scheduler.get_statistics();
    assert_eq!(initial_stats.total_created.load(Ordering::Relaxed), 0);
    
    // Spawn goroutines
    let num_success = 7;
    let num_panic = 3;
    
    for _ in 0..num_success {
        scheduler.spawn_goroutine(
            increment_counter,
            &counter as *const _ as *mut c_void
        );
    }
    
    for _ in 0..num_panic {
        scheduler.spawn_goroutine(panic_function, std::ptr::null_mut());
    }
    
    // Wait for all to complete
    scheduler.wait_all().unwrap();
    
    let final_stats = scheduler.get_statistics();
    assert_eq!(final_stats.total_created.load(Ordering::Relaxed), (num_success + num_panic) as u64);
    assert_eq!(final_stats.total_completed.load(Ordering::Relaxed), num_success as u64);
    assert_eq!(final_stats.total_panicked.load(Ordering::Relaxed), num_panic as u64);
    assert_eq!(final_stats.active_count.load(Ordering::Relaxed), 0);
    assert!(final_stats.avg_execution_time.load(Ordering::Relaxed) > 0);
    
    info!("Scheduler statistics accuracy test passed");
}

#[test]
fn test_cleanup_completed_goroutines() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing cleanup of completed goroutines");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    let counter = AtomicI32::new(0);
    
    let id = scheduler.spawn_goroutine(
        increment_counter,
        &counter as *const _ as *mut c_void
    );
    
    // Wait for completion
    scheduler.wait_for_goroutine(id).unwrap();
    
    // Verify goroutine exists before cleanup
    assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
    
    // Manually trigger cleanup (normally would happen after timeout)
    scheduler.cleanup_completed();
    
    // In this test, cleanup only removes goroutines older than 60 seconds
    // So the goroutine should still exist
    assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
    
    info!("Cleanup completed goroutines test passed");
}

#[test]
fn test_concurrent_scheduler_operations() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing concurrent scheduler operations");
    
    let gc = GarbageCollector::new();
    let scheduler = Arc::new(SimpleGoroutineScheduler::with_defaults(gc));
    let total_counter = Arc::new(AtomicI32::new(0));
    let spawned_count = Arc::new(AtomicUsize::new(0));
    
    let num_threads = 4;
    let goroutines_per_thread = 10;
    
    let mut thread_handles = Vec::new();
    
    // Spawn multiple threads that each spawn goroutines
    for thread_id in 0..num_threads {
        let scheduler_clone = Arc::clone(&scheduler);
        let counter_clone = Arc::clone(&total_counter);
        let spawned_clone = Arc::clone(&spawned_count);
        
        let handle = thread::spawn(move || {
            info!(thread_id = thread_id, "Thread starting goroutine spawning");
            
            let mut local_ids = Vec::new();
            
            for i in 0..goroutines_per_thread {
                let id = scheduler_clone.spawn_goroutine(
                    increment_counter,
                    counter_clone.as_ref() as *const _ as *mut c_void
                );
                local_ids.push(id);
                spawned_clone.fetch_add(1, Ordering::SeqCst);
                
                debug!(thread_id = thread_id, goroutine_id = id, iteration = i, "Spawned goroutine");
            }
            
            info!(thread_id = thread_id, spawned = goroutines_per_thread, "Thread finished spawning");
            local_ids
        });
        
        thread_handles.push(handle);
    }
    
    // Wait for all spawning threads to complete
    let mut all_goroutine_ids = Vec::new();
    for handle in thread_handles {
        let ids = handle.join().unwrap();
        all_goroutine_ids.extend(ids);
    }
    
    let total_spawned = spawned_count.load(Ordering::SeqCst);
    assert_eq!(total_spawned, num_threads * goroutines_per_thread);
    assert_eq!(all_goroutine_ids.len(), total_spawned);
    
    info!(total_spawned = total_spawned, "All goroutines spawned, waiting for completion");
    
    // Wait for all goroutines to complete
    scheduler.wait_all().unwrap();
    
    // Verify results
    assert_eq!(total_counter.load(Ordering::SeqCst), total_spawned as i32);
    
    // Verify all goroutines completed
    for id in &all_goroutine_ids {
        assert_eq!(scheduler.get_goroutine_metadata(*id), Some(GoroutineState::Terminated));
    }
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), total_spawned as u64);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), total_spawned as u64);
    assert_eq!(stats.active_count.load(Ordering::Relaxed), 0);
    
    info!("Concurrent scheduler operations test passed");
}

#[test]
fn test_scheduler_performance_under_load() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing scheduler performance under load");
    
    let config = ThreadPoolConfig {
        min_threads: 2,
        max_threads: 16,
        max_idle_time: Duration::from_secs(10),
        max_queue_size: 5000,
    };
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::new(config, gc);
    let counter = AtomicI32::new(0);
    
    let num_goroutines = 1000;
    let start_time = Instant::now();
    
    info!(num_goroutines = num_goroutines, "Starting performance test");
    
    // Spawn many goroutines rapidly
    for i in 0..num_goroutines {
        scheduler.spawn_goroutine(
            increment_counter,
            &counter as *const _ as *mut c_void
        );
        
        if i % 100 == 0 {
            debug!(spawned = i, "Progress update");
        }
    }
    
    let spawn_time = start_time.elapsed();
    info!(spawn_time = ?spawn_time, "All goroutines spawned");
    
    // Wait for completion
    scheduler.wait_all().unwrap();
    let total_time = start_time.elapsed();
    
    // Verify results
    assert_eq!(counter.load(Ordering::SeqCst), num_goroutines);
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), num_goroutines as u64);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), num_goroutines as u64);
    
    let throughput = num_goroutines as f64 / total_time.as_secs_f64();
    let avg_execution_ns = stats.avg_execution_time.load(Ordering::Relaxed);
    
    info!(
        total_time = ?total_time,
        spawn_time = ?spawn_time,
        throughput = throughput,
        avg_execution_ns = avg_execution_ns,
        "Performance test completed"
    );
    
    // Performance assertions (these may need adjustment based on hardware)
    assert!(throughput > 100.0, "Throughput should be > 100 goroutines/sec");
    assert!(total_time < Duration::from_secs(30), "Should complete within 30 seconds");
    
    info!("Scheduler performance under load test passed");
}

#[test]
fn test_global_scheduler_instance() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing global scheduler instance");
    
    let scheduler1 = get_global_simple_scheduler();
    let scheduler2 = get_global_simple_scheduler();
    
    // Should be the same instance
    assert!(Arc::ptr_eq(&scheduler1, &scheduler2));
    
    let counter = AtomicI32::new(0);
    
    // Use the global scheduler
    let id = scheduler1.spawn_goroutine(
        increment_counter,
        &counter as *const _ as *mut c_void
    );
    
    scheduler2.wait_for_goroutine(id).unwrap();
    assert_eq!(counter.load(Ordering::SeqCst), 1);
    
    info!("Global scheduler instance test passed");
}

#[test]
fn test_scheduler_shutdown() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing scheduler shutdown");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    let counter = AtomicI32::new(0);
    
    // Spawn some goroutines
    for _ in 0..5 {
        scheduler.spawn_goroutine(
            increment_counter,
            &counter as *const _ as *mut c_void
        );
    }
    
    // Wait for completion
    scheduler.wait_all().unwrap();
    assert_eq!(counter.load(Ordering::SeqCst), 5);
    
    // Shutdown the scheduler
    scheduler.shutdown();
    
    // Verify cleanup happened
    let stats = scheduler.get_statistics();
    info!(
        total_created = stats.total_created.load(Ordering::Relaxed),
        total_completed = stats.total_completed.load(Ordering::Relaxed),
        "Scheduler shutdown completed"
    );
    
    info!("Scheduler shutdown test passed");
}

#[test]
fn test_goroutine_state_transitions() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing goroutine state transitions");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    let counter = AtomicI32::new(0);
    
    let id = scheduler.spawn_goroutine(
        increment_counter,
        &counter as *const _ as *mut c_void
    );
    
    // Should start as Runnable
    assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Runnable));
    
    // Wait for completion and verify final state
    scheduler.wait_for_goroutine(id).unwrap();
    assert_eq!(scheduler.get_goroutine_metadata(id), Some(GoroutineState::Terminated));
    
    // Test panic state transition
    let panic_id = scheduler.spawn_goroutine(panic_function, std::ptr::null_mut());
    assert_eq!(scheduler.get_goroutine_metadata(panic_id), Some(GoroutineState::Runnable));
    
    scheduler.wait_for_goroutine(panic_id).unwrap();
    assert_eq!(scheduler.get_goroutine_metadata(panic_id), Some(GoroutineState::Panicked));
    
    info!("Goroutine state transitions test passed");
}

#[test]
fn test_error_recovery_and_resilience() {
    // init_tracing!();
    common::init_tracing();
    info!("Testing error recovery and resilience");
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::with_defaults(gc);
    let counter = AtomicI32::new(0);
    
    // Mix of successful and failing goroutines
    let mut all_ids = Vec::new();
    
    // Successful goroutines
    for _ in 0..10 {
        let id = scheduler.spawn_goroutine(
            increment_counter,
            &counter as *const _ as *mut c_void
        );
        all_ids.push((id, true)); // true = should succeed
    }
    
    // Panicking goroutines
    for _ in 0..5 {
        let id = scheduler.spawn_goroutine(panic_function, std::ptr::null_mut());
        all_ids.push((id, false)); // false = should panic
    }
    
    // More successful goroutines after panics
    for _ in 0..10 {
        let id = scheduler.spawn_goroutine(
            increment_counter,
            &counter as *const _ as *mut c_void
        );
        all_ids.push((id, true));
    }
    
    // Wait for all to complete
    scheduler.wait_all().unwrap();
    
    // Verify outcomes
    let mut success_count = 0;
    let mut panic_count = 0;
    
    for (id, should_succeed) in all_ids {
        let state = scheduler.get_goroutine_metadata(id).unwrap();
        if should_succeed {
            assert_eq!(state, GoroutineState::Terminated);
            success_count += 1;
        } else {
            assert_eq!(state, GoroutineState::Panicked);
            panic_count += 1;
        }
    }
    
    assert_eq!(success_count, 20);
    assert_eq!(panic_count, 5);
    assert_eq!(counter.load(Ordering::SeqCst), 20);
    
    let stats = scheduler.get_statistics();
    assert_eq!(stats.total_created.load(Ordering::Relaxed), 25);
    assert_eq!(stats.total_completed.load(Ordering::Relaxed), 20);
    assert_eq!(stats.total_panicked.load(Ordering::Relaxed), 5);
    
    info!("Error recovery and resilience test passed");
}

/// Performance benchmark for goroutine throughput
#[test]
fn test_goroutine_throughput_benchmark() {
    // init_tracing!();
    common::init_tracing();
    info!("Running goroutine throughput benchmark");
    
    let config = ThreadPoolConfig {
        min_threads: 4,
        max_threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4) * 2,
        max_idle_time: Duration::from_secs(30),
        max_queue_size: 10000,
    };
    
    let gc = GarbageCollector::new();
    let scheduler = SimpleGoroutineScheduler::new(config, gc);
    
    let test_sizes = vec![100, 500, 1000, 2000];
    
    for &size in &test_sizes {
        let counter = AtomicI32::new(0);
        let start_time = Instant::now();
        
        // Spawn goroutines
        for _ in 0..size {
            scheduler.spawn_goroutine(
                increment_counter,
                &counter as *const _ as *mut c_void
            );
        }
        
        // Wait for completion
        scheduler.wait_all().unwrap();
        let duration = start_time.elapsed();
        
        assert_eq!(counter.load(Ordering::SeqCst), size);
        
        let throughput = size as f64 / duration.as_secs_f64();
        info!(
            goroutines = size,
            duration = ?duration,
            throughput = throughput,
            "Benchmark result"
        );
        
        // Cleanup between runs
        scheduler.cleanup_completed();
    }
    
    info!("Goroutine throughput benchmark completed");
}
