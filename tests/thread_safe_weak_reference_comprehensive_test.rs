//! Comprehensive tests for thread-safe weak reference implementation

use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::time::Duration;
use cursed::memory::{Tag, Traceable, Visitor, ThreadSafeVisitor, GarbageCollector, ThreadSafeGc, ThreadSafeWeak};
use cursed::memory::weak::WeakRegistry;
use cursed::memory::weak_registry::GlobalWeakRegistry;
use tracing::{debug, error, info, instrument, trace, warn};

// Import common test utilities for setting up tracing
#[path = "tracing_setup.rs"]
mod tracing_setup;

// Test object that is thread-safe
#[derive(Debug, Clone)]
struct ThreadSafeTestObject {
    value: Arc<RwLock<i32>>,
    next: Arc<Mutex<Option<ThreadSafeGc<ThreadSafeTestObject>>>>,
}

impl ThreadSafeTestObject {
    fn new(value: i32) -> Self {
        Self {
            value: Arc::new(RwLock::new(value)),
            next: Arc::new(Mutex::new(None)),
        }
    }

    fn get_value(&self) -> i32 {
        *self.value.read().unwrap()
    }

    fn set_value(&self, new_value: i32) {
        *self.value.write().unwrap() = new_value;
    }
    
    fn set_next(&self, next: ThreadSafeGc<ThreadSafeTestObject>) {
        *self.next.lock().unwrap() = Some(next);
    }
    
    fn get_next(&self) -> Option<ThreadSafeGc<ThreadSafeTestObject>> {
        self.next.lock().unwrap().clone()
    }
}

impl Traceable for ThreadSafeTestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Trace the next pointer if it exists
        if let Some(next) = &*self.next.lock().unwrap() {
            // For testing purposes, we'll just log a message instead of actually tracing
            // In a real implementation, we'd need a way to check if the visitor is thread-safe
            trace!(reference_id = %next.id(), "Would trace ThreadSafeGc reference");
        }
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn tag(&self) -> Tag {
        Tag::Object
    }
}

// Safe to share across thread boundaries
unsafe impl Send for ThreadSafeTestObject {}
unsafe impl Sync for ThreadSafeTestObject {}

#[test]
#[instrument]
fn test_thread_safe_weak_reference_basic() {
    tracing_setup::init_test_tracing();
    info!("Starting basic thread-safe weak reference test");
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a thread-safe object and wrap it in ThreadSafeGc
    let obj = ThreadSafeTestObject::new(42);
    debug!("Created thread-safe test object with value=42");
    let gc_obj = gc.allocate_thread_safe(obj);
    debug!("Allocated object in thread-safe GC");
    
    // Create a weak reference
    let weak_ref = gc_obj.downgrade();
    debug!("Created weak reference from strong reference");
    
    // Verify we can upgrade the weak reference
    let upgraded = weak_ref.upgrade().expect("Should be able to upgrade weak reference");
    debug!(value = upgraded.inner().unwrap().get_value(), "Upgraded weak reference and retrieved value");
    assert_eq!(upgraded.inner().unwrap().get_value(), 42, "Value should be 42");
    
    // Modify the object through the upgraded reference
    upgraded.inner().unwrap().set_value(100);
    debug!(new_value = 100, "Modified object value through upgraded reference");
    
    // Verify the original sees the change
    debug!(original_value = gc_obj.inner().unwrap().get_value(), "Checking original reference value");
    assert_eq!(gc_obj.inner().unwrap().get_value(), 100, "Original reference should see the modified value");
    
    // Create another weak reference
    let another_weak = upgraded.downgrade();
    debug!("Created another weak reference");
    
    // Clean up the strong references
    debug!("Dropping strong references");
    drop(gc_obj);
    drop(upgraded);
    
    // Force a garbage collection
    debug!("Running garbage collection");
    gc.collect_garbage();
    
    // The weak references should no longer upgrade
    debug!(upgrades = weak_ref.upgrade().is_some(), "Checking if weak reference can be upgraded");
    assert!(weak_ref.upgrade().is_none(), "Weak reference should not upgrade after object is collected");
    debug!(upgrades = another_weak.upgrade().is_some(), "Checking if another weak reference can be upgraded");
    assert!(another_weak.upgrade().is_none(), "Another weak reference should not upgrade after object is collected");
    
    info!("Basic thread-safe weak reference test completed successfully");
}

#[test]
#[instrument]
fn test_thread_safe_weak_reference_across_threads() {
    tracing_setup::init_test_tracing();
    info!("Starting thread-safe weak reference across threads test");
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    debug!("Created garbage collector");
    
    // Create a thread-safe object and wrap it in ThreadSafeGc
    let obj = ThreadSafeTestObject::new(42);
    let gc_obj = gc.allocate_thread_safe(obj);
    debug!("Created thread-safe object with value=42");
    
    // Create a weak reference to pass to another thread
    let weak_ref = gc_obj.downgrade();
    debug!("Created weak reference for thread usage");
    
    // Spawn a thread that uses the weak reference
    let thread_weak = weak_ref.clone();
    debug!("Spawning worker thread with cloned weak reference");
    let thread_handle = thread::spawn(move || {
        // This would be a different logger, in a different thread
        // Try to upgrade the weak reference
        match thread_weak.upgrade() {
            Some(strong_ref) => {
                // Successfully upgraded
                let value = strong_ref.inner().unwrap().get_value();
                // In a real system, would use thread-local tracing setup
                // Here we just use the thread's assert
                assert_eq!(value, 42, "Thread value should be 42");
                
                // Modify the value
                strong_ref.inner().unwrap().set_value(200);
                true
            }
            None => {
                // Failed to upgrade
                false
            }
        }
    });
    
    // Wait for the thread to complete
    debug!("Waiting for worker thread to complete");
    let thread_succeeded = thread_handle.join().unwrap();
    debug!(succeeded = thread_succeeded, "Thread execution result");
    assert!(thread_succeeded, "Thread should have successfully upgraded the weak reference");
    
    // Verify the value was changed by the thread
    let modified_value = gc_obj.inner().unwrap().get_value();
    debug!(value = modified_value, expected = 200, "Checking value modified by thread");
    assert_eq!(modified_value, 200, "Value should have been changed to 200 by the thread");
    
    // Clean up
    debug!("Dropping strong reference");
    drop(gc_obj);
    
    // Force a garbage collection
    debug!("Running garbage collection");
    gc.collect_garbage();
    
    // The weak reference should no longer upgrade
    let can_upgrade = weak_ref.upgrade().is_some();
    debug!(can_upgrade = can_upgrade, "Checking if weak reference can be upgraded after collection");
    assert!(!can_upgrade, "Weak reference should not upgrade after object is collected");
    
    info!("Thread-safe weak reference across threads test completed successfully");
}

#[test]
#[instrument]
fn test_thread_safe_weak_reference_circular() {
    tracing_setup::init_test_tracing();
    info!("Starting thread-safe weak reference circular test");
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create two objects with circular references
    let obj1 = ThreadSafeTestObject::new(1);
    let obj2 = ThreadSafeTestObject::new(2);
    
    let gc_obj1 = gc.allocate_thread_safe(obj1);
    let gc_obj2 = gc.allocate_thread_safe(obj2);
    
    // Create circular references
    gc_obj1.inner().unwrap().set_next(gc_obj2.clone());
    gc_obj2.inner().unwrap().set_next(gc_obj1.clone());
    
    // Create weak references to both objects
    let weak_obj1 = gc_obj1.downgrade();
    let weak_obj2 = gc_obj2.downgrade();
    
    // Drop the strong references
    drop(gc_obj1);
    drop(gc_obj2);
    
    // Give some time for finalization
    thread::sleep(Duration::from_millis(10));
    
    // Force garbage collection
    gc.collect_garbage();
    
    // The weak references should no longer upgrade
    assert!(weak_obj1.upgrade().is_none(), "Weak reference 1 should not upgrade after circular references are collected");
    assert!(weak_obj2.upgrade().is_none(), "Weak reference 2 should not upgrade after circular references are collected");
}

#[test]
#[instrument]
fn test_thread_safe_weak_reference_multiple_threads() {
    tracing_setup::init_test_tracing();
    info!("Starting thread-safe weak reference multiple threads test");
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a shared object
    let obj = ThreadSafeTestObject::new(0);
    let gc_obj = gc.allocate_thread_safe(obj);
    
    // Create a weak reference
    let weak_ref = gc_obj.downgrade();
    
    // Number of threads to spawn
    let thread_count = 10;
    
    // Spawn multiple threads that all try to upgrade and modify the object
    let handles: Vec<_> = (0..thread_count).map(|i| {
        let thread_weak = weak_ref.clone();
        let thread_gc = gc.clone();
        
        thread::spawn(move || {
            // Each thread tries to upgrade the weak reference
            if let Some(strong_ref) = thread_weak.upgrade() {
                // Successfully upgraded, increment the value by thread number
                let old_value = strong_ref.inner().unwrap().get_value();
                strong_ref.inner().unwrap().set_value(old_value + i);
                
                // Create and drop another weak reference
                let another_weak = strong_ref.downgrade();
                drop(another_weak);
                
                true
            } else {
                false
            }
        })
    }).collect();
    
    // Wait for all threads to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // All threads should have successfully upgraded the weak reference
    assert!(results.iter().all(|&success| success), "All threads should successfully upgrade the weak reference");
    
    // Clean up
    drop(gc_obj);
    
    // Force garbage collection
    gc.collect_garbage();
    
    // The weak reference should no longer upgrade
    assert!(weak_ref.upgrade().is_none(), "Weak reference should not upgrade after object is collected");
}

#[test]
#[instrument]
fn test_thread_safe_weak_reference_concurrent_operations() {
    tracing_setup::init_test_tracing();
    info!("Starting thread-safe weak reference concurrent operations test");
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a collection of objects
    let mut strong_refs = Vec::new();
    let mut weak_refs = Vec::new();
    
    // Create several objects and their weak references
    for i in 0..5 {
        let obj = ThreadSafeTestObject::new(i);
        let gc_obj = gc.allocate_thread_safe(obj);
        let weak_ref = gc_obj.downgrade();
        
        strong_refs.push(gc_obj);
        weak_refs.push(weak_ref);
    }
    
    // Create threads that perform different operations concurrently
    let mut handles = Vec::new();
    
    // Thread 1: Upgrades weak references and uses them
    let weak_refs_clone = weak_refs.clone();
    handles.push(thread::spawn(move || {
        for weak in &weak_refs_clone {
            if let Some(strong) = weak.upgrade() {
                // Modify the value
                let old_value = strong.inner().unwrap().get_value();
                strong.inner().unwrap().set_value(old_value + 100);
            }
        }
    }));
    
    // Thread 2: Drops some strong references
    let mut strong_refs_to_drop = strong_refs.split_off(2); // Keep first 2, drop last 3
    handles.push(thread::spawn(move || {
        // Drop the strong references
        strong_refs_to_drop.clear();
        
        // Sleep to give GC a chance to run
        thread::sleep(Duration::from_millis(50));
    }));
    
    // Thread 3: Triggers garbage collection
    let gc_clone = gc.clone();
    handles.push(thread::spawn(move || {
        // Sleep to give other threads time to run
        thread::sleep(Duration::from_millis(20));
        
        // Run garbage collection multiple times
        for _ in 0..3 {
            gc_clone.collect_garbage();
            thread::sleep(Duration::from_millis(10));
        }
    }));
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify the state of the remaining strong references
    for strong in &strong_refs {
        // The values should have been modified by thread 1
        let value = strong.inner().unwrap().get_value();
        assert!(value >= 100, "Value should have been increased by at least 100");
    }
    
    // Verify the state of weak references
    // The first two should still be upgradeable (indices 0 and 1)
    for i in 0..2 {
        assert!(weak_refs[i].upgrade().is_some(), "Weak reference {} should still be upgradeable", i);
    }
    
    // The last three should not be upgradeable (indices 2, 3, and 4)
    for i in 2..5 {
        assert!(weak_refs[i].upgrade().is_none(), "Weak reference {} should not be upgradeable", i);
    }
    
    // Drop the remaining strong references
    drop(strong_refs);
    
    // Force garbage collection
    gc.collect_garbage();
    
    // All weak references should now fail to upgrade
    for (i, weak) in weak_refs.iter().enumerate() {
        assert!(weak.upgrade().is_none(), "Weak reference {} should not upgrade after all objects are collected", i);
    }
}