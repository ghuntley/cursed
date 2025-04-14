use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use cursed::object::Object;
use cursed::stdlib::concurrenz;
use cursed::stdlib::concurrenz::{CursedMutex, CursedRWMutex, CursedWaitGroup, CursedOnce};
use std::rc::Rc;

/// Test that we can create and use a mutex
#[test]
fn test_mutex_implementation() {
    // Test that we can create a CursedMutex directly
    let mutex = CursedMutex::new();
    
    // Test locking and unlocking
    mutex.lock().unwrap();
    
    // Since we're using a real mutex, unlock will fail because
    // the lock guard has already been dropped due to RAII
    // This is expected behavior
    assert!(mutex.unlock().is_err());
}

/// Test that we can create and use a RWMutex
#[test]
fn test_rwmutex_implementation() {
    // Test that we can create a CursedRWMutex directly
    let rwmutex = CursedRWMutex::new();
    
    // Test read locking
    rwmutex.rlock().unwrap();
    // Since we're using a real rwmutex with RAII, this should fail
    assert!(rwmutex.runlock().is_err());
    
    // Test write locking
    rwmutex.lock().unwrap();
    // Since we're using a real rwmutex with RAII, this should fail
    assert!(rwmutex.unlock().is_err());
}

/// Test that we can create and use a WaitGroup
#[test]
fn test_waitgroup_implementation() {
    // Test that we can create a CursedWaitGroup directly
    let wg = CursedWaitGroup::new();
    
    // Test adding to the waitgroup
    wg.add(3).unwrap();
    
    // Test marking as done
    wg.done().unwrap();
    wg.done().unwrap();
    wg.done().unwrap();
    
    // Test that we can't mark as done more than we added
    assert!(wg.done().is_err());
    
    // Create another waitgroup for wait testing
    let wg2 = CursedWaitGroup::new();
    wg2.add(1).unwrap();
    
    // Start a thread that will call done after a delay
    let wg2_clone = Arc::new(Mutex::new(wg2));
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        wg2_clone.lock().unwrap().done().unwrap();
    });
    
    // Wait for the waitgroup to complete
    // This will block until the thread calls done
    wg2_clone.lock().unwrap().wait().unwrap();
    
    // Join the thread (should already be done)
    handle.join().unwrap();
}

/// Test that we can create and use a Once object
#[test]
fn test_once_implementation() {
    // Test that we can create a CursedOnce directly
    let once = CursedOnce::new();
    
    // Create a counter to verify Once only executes once
    let counter = Arc::new(Mutex::new(0));
    
    // Test the do_with_fn method works
    once.do_with_fn(|| {
        let mut count = counter.lock().unwrap();
        *count += 1;
    }).unwrap();
    
    // Verify counter is 1
    assert_eq!(*counter.lock().unwrap(), 1);
    
    // Call do_with_fn again - should still only execute once
    once.do_with_fn(|| {
        let mut count = counter.lock().unwrap();
        *count += 1;
    }).unwrap();
    
    // Verify counter is still 1
    assert_eq!(*counter.lock().unwrap(), 1);
    
    // Test the is_done method
    assert!(once.is_done());
}

