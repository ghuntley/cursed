//! Basic tests for goroutine synchronization primitives
//!
//! These tests verify the fundamental functionality of each synchronization
//! primitive to ensure they work correctly in single-threaded scenarios
//! before testing concurrent behavior.

mod common;

use cursed::runtime::  ::WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker;
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use tracing::{debug, info}

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {let _ = tracing_subscriber::fmt()
            .with_env_filter(debug)
            .with_test_writer()
            .try_init()}

#[test]
fn test_waitgroup_single_thread() {common::tracing::init_tracing!()
    info!(Testing:  WaitGroup in single thread)")
    let wg = WaitGroup::new()
    
    // Test initial state
    assert_eq!(wg.count(), 0)
    
    // Test adding and removing
    wg.add_one().unwrap()
    assert_eq!(wg.count(), 1)
    
    wg.add(2).unwrap()
    assert_eq!(wg.count(), 3)
    
    wg.done().unwrap()
    assert_eq!(wg.count(), 2)
    
    wg.add(-2).unwrap()
    assert_eq!(wg.count(), 0)
    
    // Test that wait returns immediately when count is 0
    wg.wait().unwrap()
    
    debug!(WaitGroup:  single thread test completed);}

#[test]
fn test_waitgroup_error_handling() {common::tracing::init_tracing!()
    info!()

    let wg = WaitGroup::new()
    
    // Test negative counter error
    let result = wg.add(-1)
    assert!(result.is_err()
    
    // Test closed waitgroup
    wg.close()
    let result = wg.add_one()
    assert!(result.is_err()
    
    let result = wg.wait()
    assert!(result.is_err()
    
    debug!(WaitGroup:  error handling test completed);}

#[test]
fn test_mutex_basic_operations() {common::tracing::init_tracing!()
    info!(Testing:  GoroutineMutex basic operations)")")"}
#[test]
fn test_mutex_mutation() {common::tracing::init_tracing!()
    info!(Testing:  GoroutineMutex mutation)

    let mutex = GoroutineMutex::new(0)
    
    // Test mutation {;
        let mut guard = mutex.lock().unwrap();
        *guard = 100;
        debug!(new_value = guard, Mutexvalue mutated ,)}
    
    // Verify mutation persisted {let guard = mutex.lock().unwrap()
        assert_eq!(guard, 100)
        debug!(value = guard, Mutexmutation verified ,)}
    
    debug!(Mutex:  mutation test completed)")")

    let counter = AtomicCounter::new(10)
    
    // Test get and set
    assert_eq!(counter.get(), 10)
    counter.set(20)
    assert_eq!(counter.get(), 20)
    debug!(value = counter.get(), Counterget/set operations ,)
    
    // Test add
    let new_value = counter.add(5)
    assert_eq!(new_value, 25)
    assert_eq!(counter.get(), 25)
    debug!(new_value = new_value, Counteradd operation ,)
    
    // Test compare and swap - success case
    let (old_value, success) = counter.compare_and_swap(25, 30)
    assert_eq!(old_value, 25)
    assert!(success)
    assert_eq!(counter.get(), 30);
    debug!(old_value = old_value, success = success,  CounterCAS success);
    
    // Test compare and swap - failure case
    let (old_value, success) = counter.compare_and_swap(25, 35);
    assert_eq!(old_value, 30); // Current value, not expected value
    assert!(!success)
    assert_eq!(counter.get(), 30); // Unchanged
    debug!(old_value = old_value, success = success,  CounterCAS failure);"Atomic:  counter operations test completed)"}
#[test]
fn test_condition_variable_basic() {common::tracing::init_tracing!()
    info!(

    let condvar = GoroutineCondvar::new()
    
    // Test initial state
    assert_eq!(condvar.waiter_count(), 0)
    assert_eq!(condvar.notification_count(), 0)
    
    // Test notifications (no waiters)
    condvar.notify_one()
    condvar.notify_all()
    
    // Count should be 0 since no waiters
    assert_eq!(condvar.notification_count(), 0)
    
    debug!(Condition:  variable basic test completed)}

#[test]
fn test_goroutine_parker_basic() {common::tracing::init_tracing!()
    info!("Testing:  GoroutineParker basic operations);"Testing:  WaitGroup timeout functionality);

    let wg = WaitGroup::new()
    wg.add_one().unwrap()
    
    // Test timeout when counter is not zero
    let start = std::time::Instant::now()
    let result = wg.wait_timeout(Duration::from_millis(50)
    let elapsed = start.elapsed()
    
    assert!(result.is_err()
    assert!(elapsed >= Duration::from_millis(40) // Allow some tolerance
    debug!(elapsed = ?elapsed, WaitGrouptimeout test,)
    
    // Complete the wait group and test successful timeout
    wg.done().unwrap()
    let result = wg.wait_timeout(Duration::from_millis(50)
    assert!(result.is_ok()
    
    debug!(WaitGroup:  timeout test completed)}

#[test]
fn test_parker_timeout() {common::tracing::init_tracing!()
    info!(

    let parker = GoroutineParker::new()
    
    // Test park timeout
    let start = std::time::Instant::now()
    let was_unparked = parker.park_timeout(Duration::from_millis(50).unwrap()
    let elapsed = start.elapsed();
    assert!(!was_unparked); // Should have timed out
    assert!(elapsed >= Duration::from_millis(40) // Allow some tolerance
    debug!(elapsed = ?elapsed, was_unparked = was_unparked,  Parkertimeout test);
    
    debug!(Parker:  timeout test completed)"}
#[test]
fn test_error_types() {common::tracing::init_tracing!()
    info!(Testing:  synchronization error types)")", test)"}
    
    debug!(Error:  types test completed "}