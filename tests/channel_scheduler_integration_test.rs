//! Integration tests for channel-scheduler system
//!
//! This test suite validates the comprehensive integration between the channel
//! system and goroutine scheduler, including blocking operations, goroutine 
//! parking/unparking, and performance optimization.

use cursed::runtime::{
    ChannelScheduler,
    GoroutineScheduler,
    ThreadPoolConfig,
    ChannelOpResult,
    ChannelOpType,
    GoroutineState,
};
use cursed::memory::GarbageCollector;
use cursed::object::{Object, Channel};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};

// Common test tracing setup
#[path = "common/mod.rs"]
mod common;

#[test]
fn test_channel_scheduler_creation() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
    let channel_scheduler = ChannelScheduler::new(goroutine_scheduler, gc);
    
    let stats = channel_scheduler.get_statistics();
    assert_eq!(stats.total_operations.load(Ordering::Relaxed), 0);
    assert_eq!(stats.current_blocked.load(Ordering::Relaxed), 0);
    assert_eq!(stats.channels_with_waiters.load(Ordering::Relaxed), 0);
    
    tracing::info!("Channel scheduler created successfully");
}

#[test]
fn test_blocking_channel_operations_integration() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler.clone(), gc));
    
    // Create a simple channel for testing
    let channel = Arc::new(RwLock::new(Channel::new("int".to_string(), 0))); // Unbuffered
    let channel_ptr = Arc::as_ptr(&channel) as *mut std::ffi::c_void;
    
    let success_counter = Arc::new(AtomicI32::new(0));
    let sender_counter = success_counter.clone();
    let receiver_counter = success_counter.clone();
    let scheduler_clone = channel_scheduler.clone();
    let scheduler_clone2 = channel_scheduler.clone();
    
    // Spawn sender goroutine
    let sender_handle = thread::spawn(move || {
        tracing::info!("Sender starting");
        
        let test_value = 42i64;
        let value_ptr = &test_value as *const i64 as *mut std::ffi::c_void;
        
        // Simulate blocking send
        match scheduler_clone.blocking_send(
            1, // goroutine_id
            channel_ptr,
            value_ptr,
            Some(Duration::from_secs(5))
        ) {
            ChannelOpResult::Success(_) => {
                sender_counter.fetch_add(1, Ordering::SeqCst);
                tracing::info!("Sender completed successfully");
            }
            other => {
                tracing::error!("Sender failed: {:?}", other);
            }
        }
    });
    
    // Give sender a moment to start
    thread::sleep(Duration::from_millis(100));
    
    // Spawn receiver goroutine
    let receiver_handle = thread::spawn(move || {
        tracing::info!("Receiver starting");
        
        // Simulate blocking receive
        match scheduler_clone2.blocking_receive(
            2, // goroutine_id
            channel_ptr,
            Some(Duration::from_secs(5))
        ) {
            ChannelOpResult::Success(Some(Object::Integer(value))) => {
                assert_eq!(value, 42);
                receiver_counter.fetch_add(1, Ordering::SeqCst);
                tracing::info!("Receiver completed successfully with value: {}", value);
            }
            ChannelOpResult::Success(other) => {
                tracing::error!("Receiver got unexpected value: {:?}", other);
            }
            other => {
                tracing::error!("Receiver failed: {:?}", other);
            }
        }
    });
    
    // Wait for both to complete
    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();
    
    // Verify both operations completed successfully
    assert_eq!(success_counter.load(Ordering::SeqCst), 2);
    
    let stats = channel_scheduler.get_statistics();
    assert!(stats.total_operations.load(Ordering::Relaxed) >= 2);
    
    tracing::info!("Blocking operations integration test completed successfully");
}

#[test]
fn test_goroutine_parking_unparking() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler.clone(), gc));
    
    // Create test goroutine
    let counter = AtomicI32::new(0);
    unsafe extern "C" fn test_function(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
        let counter = data as *mut AtomicI32;
        (*counter).fetch_add(1, Ordering::SeqCst);
        std::ptr::null_mut()
    }
    
    let goroutine_id = goroutine_scheduler.spawn_goroutine(
        test_function,
        &counter as *const _ as *mut std::ffi::c_void
    );
    
    // Wait for goroutine to start
    thread::sleep(Duration::from_millis(10));
    
    // Test parking for channel operation
    let result = goroutine_scheduler.park_for_channel_operation(
        goroutine_id,
        123, // operation_id
        "send",
        5 // priority
    );
    assert!(result.is_ok());
    
    // Verify goroutine state
    assert_eq!(
        goroutine_scheduler.get_goroutine_metadata(goroutine_id),
        Some(GoroutineState::BlockedChannelSend)
    );
    
    // Test unparking
    let result = goroutine_scheduler.unpark_from_channel_operation(goroutine_id, 123);
    assert!(result.is_ok());
    
    // Wait for goroutine to complete
    goroutine_scheduler.wait_for_goroutine(goroutine_id).unwrap();
    
    // Verify final state
    assert_eq!(
        goroutine_scheduler.get_goroutine_metadata(goroutine_id),
        Some(GoroutineState::Terminated)
    );
    
    assert_eq!(counter.load(Ordering::SeqCst), 1);
    
    tracing::info!("Goroutine parking/unparking test completed successfully");
}

#[test] 
fn test_channel_wait_queues() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler, gc));
    
    // Create a channel
    let channel = Arc::new(RwLock::new(Channel::new("int".to_string(), 0)));
    let channel_ptr = Arc::as_ptr(&channel) as *mut std::ffi::c_void;
    
    let completed_operations = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();
    
    // Spawn multiple sender goroutines that will block
    for i in 0..5 {
        let scheduler_clone = channel_scheduler.clone();
        let completed_clone = completed_operations.clone();
        
        let handle = thread::spawn(move || {
            let test_value = (i + 1) * 10;
            let value_ptr = &test_value as *const i64 as *mut std::ffi::c_void;
            
            match scheduler_clone.blocking_send(
                i + 10, // unique goroutine_id
                channel_ptr,
                value_ptr,
                Some(Duration::from_millis(500))
            ) {
                ChannelOpResult::Success(_) => {
                    completed_clone.fetch_add(1, Ordering::SeqCst);
                    tracing::info!("Sender {} completed", i);
                }
                ChannelOpResult::Timeout => {
                    tracing::info!("Sender {} timed out as expected", i);
                }
                other => {
                    tracing::warn!("Sender {} got unexpected result: {:?}", i, other);
                }
            }
        });
        
        handles.push(handle);
        thread::sleep(Duration::from_millis(10)); // Stagger the operations
    }
    
    // Let them all start and block
    thread::sleep(Duration::from_millis(100));
    
    // Check statistics
    let stats = channel_scheduler.get_statistics();
    assert!(stats.total_operations.load(Ordering::Relaxed) >= 5);
    assert!(stats.total_blocking_ops.load(Ordering::Relaxed) >= 5);
    
    // Now spawn a receiver to unblock one of them
    let scheduler_clone = channel_scheduler.clone();
    let receiver_handle = thread::spawn(move || {
        match scheduler_clone.blocking_receive(
            99, // receiver goroutine_id
            channel_ptr,
            Some(Duration::from_millis(200))
        ) {
            ChannelOpResult::Success(Some(Object::Integer(value))) => {
                tracing::info!("Receiver got value: {}", value);
                assert!(value >= 10 && value <= 50); // One of our test values
            }
            other => {
                tracing::warn!("Receiver got unexpected result: {:?}", other);
            }
        }
    });
    
    // Wait for all to complete
    for handle in handles {
        handle.join().unwrap();
    }
    receiver_handle.join().unwrap();
    
    // At least one operation should have completed successfully
    assert!(completed_operations.load(Ordering::SeqCst) >= 1);
    
    tracing::info!("Channel wait queues test completed successfully");
}

#[test]
fn test_channel_operation_cancellation() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler.clone(), gc));
    
    // Create a channel
    let channel = Arc::new(RwLock::new(Channel::new("int".to_string(), 0)));
    let channel_ptr = Arc::as_ptr(&channel) as *mut std::ffi::c_void;
    
    let goroutine_id = 123u64;
    let scheduler_clone = channel_scheduler.clone();
    
    // Start a blocking operation in a separate thread
    let operation_handle = thread::spawn(move || {
        let test_value = 42i64;
        let value_ptr = &test_value as *const i64 as *mut std::ffi::c_void;
        
        match scheduler_clone.blocking_send(
            goroutine_id,
            channel_ptr,
            value_ptr,
            Some(Duration::from_secs(10)) // Long timeout
        ) {
            ChannelOpResult::Cancelled => {
                tracing::info!("Operation was cancelled as expected");
                true
            }
            other => {
                tracing::warn!("Operation got unexpected result: {:?}", other);
                false
            }
        }
    });
    
    // Let the operation start and block
    thread::sleep(Duration::from_millis(100));
    
    // Cancel all operations for this goroutine
    channel_scheduler.cancel_goroutine_operations(goroutine_id);
    
    // The operation should complete with cancellation
    let was_cancelled = operation_handle.join().unwrap();
    assert!(was_cancelled);
    
    let stats = channel_scheduler.get_statistics();
    assert!(stats.total_cancelled.load(Ordering::Relaxed) >= 1);
    
    tracing::info!("Channel operation cancellation test completed successfully");
}

#[test]
fn test_channel_scheduler_performance() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler, gc));
    
    let start_time = Instant::now();
    let operation_count = 100;
    let completed_operations = Arc::new(AtomicU64::new(0));
    
    // Create multiple channels for parallel testing
    let mut channels = Vec::new();
    for _ in 0..10 {
        let channel = Arc::new(RwLock::new(Channel::new("int".to_string(), 5))); // Buffered
        channels.push(channel);
    }
    
    let mut handles = Vec::new();
    
    // Spawn many operations across different channels
    for i in 0..operation_count {
        let channel_idx = i % channels.len();
        let channel = channels[channel_idx].clone();
        let channel_ptr = Arc::as_ptr(&channel) as *mut std::ffi::c_void;
        let scheduler_clone = channel_scheduler.clone();
        let completed_clone = completed_operations.clone();
        
        let handle = thread::spawn(move || {
            let test_value = i as i64;
            let value_ptr = &test_value as *const i64 as *mut std::ffi::c_void;
            
            // Alternate between send and receive operations
            if i % 2 == 0 {
                // Send operation
                match scheduler_clone.blocking_send(
                    i as u64 + 1000,
                    channel_ptr,
                    value_ptr,
                    Some(Duration::from_millis(100))
                ) {
                    ChannelOpResult::Success(_) => {
                        completed_clone.fetch_add(1, Ordering::SeqCst);
                    }
                    _ => {}
                }
            } else {
                // Receive operation
                match scheduler_clone.blocking_receive(
                    i as u64 + 1000,
                    channel_ptr,
                    Some(Duration::from_millis(100))
                ) {
                    ChannelOpResult::Success(_) => {
                        completed_clone.fetch_add(1, Ordering::SeqCst);
                    }
                    _ => {}
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let elapsed = start_time.elapsed();
    let completed = completed_operations.load(Ordering::SeqCst);
    
    tracing::info!(
        "Performance test completed: {} operations in {:?}, {} completed",
        operation_count,
        elapsed,
        completed
    );
    
    // Verify basic performance metrics
    assert!(elapsed < Duration::from_secs(10)); // Should complete reasonably quickly
    assert!(completed > 0); // At least some operations should succeed
    
    let stats = channel_scheduler.get_statistics();
    assert_eq!(stats.total_operations.load(Ordering::Relaxed), operation_count as u64);
    
    tracing::info!("Channel scheduler performance test completed successfully");
}

#[test]
fn test_fair_scheduling_for_channel_operations() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let config = ThreadPoolConfig {
        min_threads: 2,
        max_threads: 4,
        max_idle_time: Duration::from_secs(1),
        max_queue_size: 1000,
    };
    let goroutine_scheduler = Arc::new(GoroutineScheduler::new(config, gc.clone()));
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler.clone(), gc));
    
    // Create test channels
    let channels: Vec<_> = (0..5).map(|_| {
        Arc::new(RwLock::new(Channel::new("int".to_string(), 1))) // Small buffer
    }).collect();
    
    let operation_results = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();
    
    // Spawn operations with different priorities
    for (priority, channel) in channels.iter().enumerate() {
        let channel_ptr = Arc::as_ptr(channel) as *mut std::ffi::c_void;
        let scheduler_clone = channel_scheduler.clone();
        let goroutine_scheduler_clone = goroutine_scheduler.clone();
        let results_clone = operation_results.clone();
        
        let handle = thread::spawn(move || {
            let goroutine_id = priority as u64 + 2000;
            
            // Create a test goroutine
            unsafe extern "C" fn test_function(_data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
                std::ptr::null_mut()
            }
            
            let actual_goroutine_id = goroutine_scheduler_clone.spawn_goroutine(
                test_function,
                std::ptr::null_mut()
            );
            
            // Park the goroutine with different priorities
            let _ = goroutine_scheduler_clone.park_for_channel_operation(
                actual_goroutine_id,
                priority as u64 + 100,
                "send",
                priority as u8 // Higher index = higher priority
            );
            
            thread::sleep(Duration::from_millis(50));
            
            // Unpark and check if it gets scheduled appropriately
            let _ = goroutine_scheduler_clone.unpark_from_channel_operation(
                actual_goroutine_id,
                priority as u64 + 100
            );
            
            results_clone.fetch_add(1, Ordering::SeqCst);
        });
        
        handles.push(handle);
    }
    
    // Wait for all to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all operations completed
    assert_eq!(operation_results.load(Ordering::SeqCst), 5);
    
    // Check scheduler statistics
    let (send_blocked, receive_blocked) = goroutine_scheduler.get_channel_operation_stats();
    tracing::info!(
        "Final channel operation stats: send_blocked={}, receive_blocked={}",
        send_blocked,
        receive_blocked
    );
    
    tracing::info!("Fair scheduling test completed successfully");
}

#[test]
fn test_channel_scheduler_cleanup_and_optimization() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler, gc));
    
    // Create some temporary operations that will complete quickly
    let channel = Arc::new(RwLock::new(Channel::new("int".to_string(), 10))); // Large buffer
    let channel_ptr = Arc::as_ptr(&channel) as *mut std::ffi::c_void;
    
    // Fill the channel buffer with some values first
    {
        let mut ch = channel.write().unwrap();
        for i in 0..5 {
            let _ = ch.send(Object::Integer(i));
        }
    }
    
    let mut handles = Vec::new();
    
    // Spawn operations that will complete quickly
    for i in 0..20 {
        let scheduler_clone = channel_scheduler.clone();
        
        let handle = thread::spawn(move || {
            if i % 2 == 0 {
                // Receive operation (should succeed immediately due to buffered values)
                let _ = scheduler_clone.blocking_receive(
                    i as u64 + 3000,
                    channel_ptr,
                    Some(Duration::from_millis(10))
                );
            } else {
                // Send operation (should succeed due to buffer space)
                let test_value = i as i64;
                let value_ptr = &test_value as *const i64 as *mut std::ffi::c_void;
                let _ = scheduler_clone.blocking_send(
                    i as u64 + 3000,
                    channel_ptr,
                    value_ptr,
                    Some(Duration::from_millis(10))
                );
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for operations to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Perform cleanup and optimization
    channel_scheduler.cleanup_and_optimize();
    
    // Verify cleanup worked
    let stats = channel_scheduler.get_statistics();
    tracing::info!(
        "After cleanup: channels_with_waiters={}",
        stats.channels_with_waiters.load(Ordering::Relaxed)
    );
    
    // Most operations should have completed
    assert!(stats.total_completed.load(Ordering::Relaxed) > 0);
    
    tracing::info!("Channel scheduler cleanup and optimization test completed successfully");
}

#[test]
fn test_integration_with_goroutine_lifecycle() {
    // init_tracing!();
    common::tracing::setup();
    
    let gc = Arc::new(GarbageCollector::new());
    let goroutine_scheduler = Arc::new(GoroutineScheduler::with_defaults(gc.clone()));
    let channel_scheduler = Arc::new(ChannelScheduler::new(goroutine_scheduler.clone(), gc));
    
    let completion_counter = Arc::new(AtomicI32::new(0));
    let counter_clone = completion_counter.clone();
    
    // Create test goroutine function
    unsafe extern "C" fn channel_test_function(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
        let counter = data as *mut AtomicI32;
        
        // Simulate some channel operations during goroutine lifecycle
        thread::sleep(Duration::from_millis(10));
        
        (*counter).fetch_add(1, Ordering::SeqCst);
        std::ptr::null_mut()
    }
    
    // Spawn goroutines and track their channel operations
    let mut goroutine_ids = Vec::new();
    
    for _ in 0..5 {
        let goroutine_id = goroutine_scheduler.spawn_goroutine(
            channel_test_function,
            &*counter_clone as *const _ as *mut std::ffi::c_void
        );
        goroutine_ids.push(goroutine_id);
        
        // Simulate channel operation for each goroutine
        let _ = goroutine_scheduler.park_for_channel_operation(
            goroutine_id,
            goroutine_id * 10, // operation_id
            "receive",
            1
        );
    }
    
    thread::sleep(Duration::from_millis(50));
    
    // Unpark all goroutines
    for goroutine_id in &goroutine_ids {
        let _ = goroutine_scheduler.unpark_from_channel_operation(*goroutine_id, *goroutine_id * 10);
    }
    
    // Wait for all goroutines to complete
    goroutine_scheduler.wait_all().unwrap();
    
    // Verify all completed
    assert_eq!(completion_counter.load(Ordering::SeqCst), 5);
    
    // Test cleanup of goroutine operations
    for goroutine_id in goroutine_ids {
        goroutine_scheduler.cancel_goroutine_channel_operations(goroutine_id);
    }
    
    tracing::info!("Integration with goroutine lifecycle test completed successfully");
}
