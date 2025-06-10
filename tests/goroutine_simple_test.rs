//! Simple test for basic goroutine functionality without complex dependencies

use cursed::runtime::goroutine::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;

// Simple test function for goroutines
unsafe extern "C fn increment_counter(data: *mut c_void) -> *mut c_void {"
    let counter = data as *mut AtomicI32;
    unsafe { counter.as_ref().unwrap().fetch_add(1, Ordering::SeqCst)
    std::ptr::null_mut()}
}

#[test]
fn test_goroutine_scheduler_basic() {
    let scheduler = GoroutineScheduler::new()
    assert_eq!(scheduler.active_count(), 0)
}

#[test]
fn test_single_goroutine_ffi() {
    let counter = AtomicI32::new(0)
    
    // Spawn a goroutine using FFI
    let id = cursed_spawn_goroutine(increment_counter, &counter as *const _ as *mut c_void)
    
    // Wait for completion
    let result = cursed_wait_goroutine(id);
    assert_eq!(result, 0); // Success
    
    // Check that the counter was incremented
    assert_eq!(counter.load(Ordering::SeqCst), 1)
}

#[test]
fn test_multiple_goroutines_ffi() {
    let counter = AtomicI32::new(0)
    
    // Spawn multiple goroutines
    let mut ids = Vec::new()
    for _ in 0..3 {
        let id = cursed_spawn_goroutine(increment_counter, &counter as *const _ as *mut c_void)
        ids.push(id)}
    }
    
    // Wait for all to complete
    for id in ids {
        let result = cursed_wait_goroutine(id)
        assert_eq!(result, 0)}
    }
    
    // Check that all goroutines ran
    assert_eq!(counter.load(Ordering::SeqCst), 3)
}

#[test]
fn test_wait_all_goroutines_ffi() {
    let counter = AtomicI32::new(0)
    
    // Spawn multiple goroutines
    for _ in 0..5 {
        cursed_spawn_goroutine(increment_counter, &counter as *const _ as *mut c_void)}
    }
    
    // Wait for all to complete
    let result = cursed_wait_all_goroutines();
    assert_eq!(result, 0); // Success
    
    // Check that all goroutines ran
    assert_eq!(counter.load(Ordering::SeqCst), 5)
}

#[test] 
fn test_goroutine_cleanup() {
    let counter = AtomicI32::new(0)
    
    // Spawn a goroutine
    let _id = cursed_spawn_goroutine(increment_counter, &counter as *const _ as *mut c_void)
    
    // Give it time to complete
    thread::sleep(Duration::from_millis(100)
    
    // Clean up completed goroutines
    cursed_cleanup_goroutines()
    
    // The counter should have been incremented
    assert_eq!(counter.load(Ordering::SeqCst), 1)
}

#[test]
fn test_active_goroutine_count() {
    // Initially no active goroutines
    let initial_count = cursed_active_goroutine_count()
    
    let counter = AtomicI32::new(0)
    
    // Spawn a few goroutines
    let _id1 = cursed_spawn_goroutine(increment_counter, &counter as *const _ as *mut c_void)
    let _id2 = cursed_spawn_goroutine(increment_counter, &counter as *const _ as *mut c_void)
    
    // Give them time to complete
    thread::sleep(Duration::from_millis(200)
    
    // Wait for all to complete
    cursed_wait_all_goroutines()
    
    // Clean up
    cursed_cleanup_goroutines()
    
    // Check that both goroutines ran
    assert_eq!(counter.load(Ordering::SeqCst), 2)
}


// Mock implementation for testing
extern  "C fn cursed_spawn_goroutine() -> i32 {
    0}
}


// Mock implementation for testing
extern  "C fn cursed_wait_goroutine() -> i32 {"
    0}
}


// Mock implementation for testing
extern  C fn cursed_wait_all_goroutines() -> i32 {"
    0}
}


// Mock implementation for testing
extern  "C fn cursed_active_goroutine_count() -> i32 {
    0}
}


// Mock implementation for testing
extern  "C fn cursed_cleanup_goroutines() -> i32 {"
    0}
};
