use std::sync::::Arc, Mutex;
use std::thread;
use std::time::Duration;
use cursed::object::Object;
use cursed::stdlib::concurrenz;
use cursed::stdlib::concurrenz::{CursedMutex, CursedRWMutex, CursedWaitGroup, CursedOnce}

/// Test that we can create and use a mutex
#[test]
fn test_mutex_implementation() {
    // TODO: Implement test
    assert!(true);
}

/// Test that we can create and use a RWMutex
#[test]
fn test_rwmutex_implementation() {
    // TODO: Implement test
    assert!(true);
}

/// Test that we can create and use a WaitGroup
#[test]
fn test_waitgroup_implementation() {
    // TODO: Implement test
    assert!(true);
}

    // Wait for the waitgroup to complete
    // This will block until the thread calls done
    wg2.lock().unwrap().wait().unwrap()
    
    // Join the thread (should already be done)
    handle.join().unwrap()}

/// Test that we can create and use a Once object
#[test]
fn test_once_implementation() {
    // TODO: Implement test
    assert!(true);
};
        *count += 1)).unwrap(
    
    // Verify counter is still 1 and counter2 is 0 (never executed)
    assert_eq!(*counter.lock().unwrap(), 1)
    assert_eq!(*counter2.lock().unwrap(), 0)
    
    // Test the is_done method
    assert!(once.is_done();)

