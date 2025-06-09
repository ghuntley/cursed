//! Basic tests for goroutine synchronization primitives
//!
//! These tests verify the fundamental functionality of each synchronization
//! primitive to ensure they work correctly in single-threaded scenarios
//! before testing concurrent behavior.

mod common;

use cursed::runtime::{WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker};
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use tracing::{debug, info};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_waitgroup_single_thread() {
    init_tracing!();
    info!("Testing WaitGroup in single thread");

    let wg = WaitGroup::new();
    
    // Test initial state
    assert_eq!(wg.count(), 0);
    
    // Test adding and removing
    wg.add_one().unwrap();
    assert_eq!(wg.count(), 1);
    
    wg.add(2).unwrap();
    assert_eq!(wg.count(), 3);
    
    wg.done().unwrap();
    assert_eq!(wg.count(), 2);
    
    wg.add(-2).unwrap();
    assert_eq!(wg.count(), 0);
    
    // Test that wait returns immediately when count is 0
    wg.wait().unwrap();
    
    debug!("WaitGroup single thread test completed");
}

#[test]
fn test_waitgroup_error_handling() {
    init_tracing!();
    info!("Testing WaitGroup error handling");

    let wg = WaitGroup::new();
    
    // Test negative counter error
    let result = wg.add(-1);
    assert!(result.is_err());
    
    // Test closed waitgroup
    wg.close();
    let result = wg.add_one();
    assert!(result.is_err());
    
    let result = wg.wait();
    assert!(result.is_err());
    
    debug!("WaitGroup error handling test completed");
}

#[test]
fn test_mutex_basic_operations() {
    init_tracing!();
    info!("Testing GoroutineMutex basic operations");

    let mutex = GoroutineMutex::new(42);
    
    // Test lock and access
    {
        let guard = mutex.lock().unwrap();
        assert_eq!(*guard, 42);
        assert!(mutex.is_owned_by_current_thread());
        debug!(value = *guard, "Mutex locked and value accessed");
    }
    
    // Test that mutex is unlocked
    assert!(!mutex.is_owned_by_current_thread());
    assert_eq!(mutex.owner(), 0);
    
    // Test try_lock
    {
        let guard = mutex.try_lock().unwrap();
        assert_eq!(*guard, 42);
        assert!(mutex.is_owned_by_current_thread());
        debug!("Mutex try_lock successful");
    }
    
    debug!("Mutex basic operations test completed");
}

#[test]
fn test_mutex_mutation() {
    init_tracing!();
    info!("Testing GoroutineMutex mutation");

    let mutex = GoroutineMutex::new(0);
    
    // Test mutation
    {
        let mut guard = mutex.lock().unwrap();
        *guard = 100;
        debug!(new_value = *guard, "Mutex value mutated");
    }
    
    // Verify mutation persisted
    {
        let guard = mutex.lock().unwrap();
        assert_eq!(*guard, 100);
        debug!(value = *guard, "Mutex mutation verified");
    }
    
    debug!("Mutex mutation test completed");
}

#[test]
fn test_atomic_counter_operations() {
    init_tracing!();
    info!("Testing AtomicCounter operations");

    let counter = AtomicCounter::new(10);
    
    // Test get and set
    assert_eq!(counter.get(), 10);
    counter.set(20);
    assert_eq!(counter.get(), 20);
    debug!(value = counter.get(), "Counter get/set operations");
    
    // Test add
    let new_value = counter.add(5);
    assert_eq!(new_value, 25);
    assert_eq!(counter.get(), 25);
    debug!(new_value = new_value, "Counter add operation");
    
    // Test compare and swap - success case
    let (old_value, success) = counter.compare_and_swap(25, 30);
    assert_eq!(old_value, 25);
    assert!(success);
    assert_eq!(counter.get(), 30);
    debug!(old_value = old_value, success = success, "Counter CAS success");
    
    // Test compare and swap - failure case
    let (old_value, success) = counter.compare_and_swap(25, 35);
    assert_eq!(old_value, 30); // Current value, not expected value
    assert!(!success);
    assert_eq!(counter.get(), 30); // Unchanged
    debug!(old_value = old_value, success = success, "Counter CAS failure");
    
    // Test operation counting
    assert!(counter.operation_count() > 0);
    debug!(operations = counter.operation_count(), "Counter operation count");
    
    debug!("Atomic counter operations test completed");
}

#[test]
fn test_condition_variable_basic() {
    init_tracing!();
    info!("Testing GoroutineCondvar basic operations");

    let condvar = GoroutineCondvar::new();
    
    // Test initial state
    assert_eq!(condvar.waiter_count(), 0);
    assert_eq!(condvar.notification_count(), 0);
    
    // Test notifications (no waiters)
    condvar.notify_one();
    condvar.notify_all();
    
    // Count should be 0 since no waiters
    assert_eq!(condvar.notification_count(), 0);
    
    debug!("Condition variable basic test completed");
}

#[test]
fn test_goroutine_parker_basic() {
    init_tracing!();
    info!("Testing GoroutineParker basic operations");

    let parker = GoroutineParker::new();
    
    // Test initial state
    assert_eq!(parker.parked_count(), 0);
    let (park_count, unpark_count, parked_count) = parker.stats();
    assert_eq!(park_count, 0);
    assert_eq!(unpark_count, 0);
    assert_eq!(parked_count, 0);
    
    // Test unpark_all with no parked goroutines
    let unparked = parker.unpark_all().unwrap();
    assert_eq!(unparked, 0);
    
    debug!("Goroutine parker basic test completed");
}

#[test]
fn test_waitgroup_timeout() {
    init_tracing!();
    info!("Testing WaitGroup timeout functionality");

    let wg = WaitGroup::new();
    wg.add_one().unwrap();
    
    // Test timeout when counter is not zero
    let start = std::time::Instant::now();
    let result = wg.wait_timeout(Duration::from_millis(50));
    let elapsed = start.elapsed();
    
    assert!(result.is_err());
    assert!(elapsed >= Duration::from_millis(40)); // Allow some tolerance
    debug!(elapsed = ?elapsed, "WaitGroup timeout test");
    
    // Complete the wait group and test successful timeout
    wg.done().unwrap();
    let result = wg.wait_timeout(Duration::from_millis(50));
    assert!(result.is_ok());
    
    debug!("WaitGroup timeout test completed");
}

#[test]
fn test_parker_timeout() {
    init_tracing!();
    info!("Testing GoroutineParker timeout functionality");

    let parker = GoroutineParker::new();
    
    // Test park timeout
    let start = std::time::Instant::now();
    let was_unparked = parker.park_timeout(Duration::from_millis(50)).unwrap();
    let elapsed = start.elapsed();
    
    assert!(!was_unparked); // Should have timed out
    assert!(elapsed >= Duration::from_millis(40)); // Allow some tolerance
    debug!(elapsed = ?elapsed, was_unparked = was_unparked, "Parker timeout test");
    
    debug!("Parker timeout test completed");
}

#[test]
fn test_error_types() {
    init_tracing!();
    info!("Testing synchronization error types");

    use cursed::runtime::SyncError;
    
    // Test error display
    let errors = vec![
        SyncError::Timeout,
        SyncError::Cancelled,
        SyncError::LockFailed("test".to_string()),
        SyncError::InvalidState("test".to_string()),
        SyncError::Closed,
        SyncError::Deadlock,
    ];
    
    for error in errors {
        let error_string = format!("{}", error);
        assert!(!error_string.is_empty());
        debug!(error = %error, "Error type test");
    }
    
    debug!("Error types test completed");
}
