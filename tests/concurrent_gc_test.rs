use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use cursed::memory::{Traceable, Tag, Visitor, GarbageCollector, ThreadSafeGc, ConcurrentGarbageCollector};
use cursed::memory::concurrent_gc::ConcurrentGcConfig;

extern crate cursed;

mod common;



// Use tracing module
// Import common modules
// mod common; // Already declared above

// Test object for the concurrent GC test
#[derive(Debug, Clone)]
struct TestObject {
    value: Arc<Mutex<i32>>,
    next: Arc<Mutex<Option<ThreadSafeGc<TestObject>>>>,
}

impl TestObject {
    fn new(value: i32) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
            next: Arc::new(Mutex::new(None)),
        }
    }
    
    fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }
    
    fn set_value(&self, value: i32) {
        *self.value.lock().unwrap() = value;
    }
    
    fn set_next(&self, next: ThreadSafeGc<TestObject>) {
        *self.next.lock().unwrap() = Some(next);
    }
    
    fn get_next(&self) -> Option<ThreadSafeGc<TestObject>> {
        self.next.lock().unwrap().clone()
    }
}

impl Traceable for TestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Trace the next object if there is one
        if let Some(next) = &*self.next.lock().unwrap() {
            visitor.visit_ptr(next.id(), Tag::Object);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

// Must be Send + Sync for ThreadSafeGc
unsafe impl Send for TestObject {}
unsafe impl Sync for TestObject {}

#[test]
fn test_concurrent_gc_basic() {
    // Set up tracing
    common::tracing::setup();
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new();
    
    // Create a concurrent garbage collector with a custom configuration
    let config = ConcurrentGcConfig {
        collection_interval_ms: 100,   // Collect every 100ms
        time_budget_ms: 50,            // 50ms per collection
        heap_threshold_bytes: 1024,    // 1KB threshold
        thread_count: 1,               // Single collector thread
        max_pause_ms: 10,              // 10ms max pause
        verbose_logging: true,         // Enable verbose logging
    };
    
    let concurrent_gc = ConcurrentGarbageCollector::with_config(gc.clone(), config);
    
    // Allocate some objects
    let mut objects = Vec::new();
    for i in 0..50 {
        let obj = concurrent_gc.allocate(TestObject::new(i);
        objects.push(obj);
    }
    
    // Create some links between objects to form chains
    for i in 0..objects.len()-1 {
        objects[i].inner().unwrap().set_next(objects[i+1].clone();
    }
    
    // Create a cycle to test cycle collection
    objects.last().unwrap().inner().unwrap().set_next(objects[0].clone();
    
    // Sleep to allow collector to run
    thread::sleep(Duration::from_millis(300);
    
    // Verify all objects are still accessible
    for (i, obj) in objects.iter().enumerate() {
        assert_eq!(obj.inner().unwrap().get_value(), i as i32);
    }
    
    // Keep a reference to one object in the chain
    let first_obj = objects[0].clone();
    
    // Drop all our references to the objects
    drop(objects);
    
    // Sleep to allow collector to run
    thread::sleep(Duration::from_millis(300);
    
    // The objects should still be accessible through the chain
    // since we have a reference to the first one
    let mut current = Some(first_obj);
    let mut count = 0;
    
    while let Some(obj) = current {
        if let Some(inner) = obj.inner() {
            assert_eq!(inner.get_value(), count);
            current = inner.get_next();
            count += 1;
            
            // Avoid infinite loop due to cycle
            if count > 50 {
                break;
            }
        } else {
            panic!("Object should be accessible");
        }
    }
    
    // Now drop our reference to the first object
    drop(first_obj);
    
    // Force collection
    concurrent_gc.request_collection();
    
    // Sleep to allow collector to run multiple times
    thread::sleep(Duration::from_millis(500);
    
    // Check GC statistics
    let stats = concurrent_gc.stats();
    println!("GC Stats: {:?}", stats);
}

#[test]
fn test_concurrent_gc_stress() {
    // Set up tracing
    common::tracing::setup();
    
    // Create a garbage collector with a low threshold to trigger frequent collections
    let gc = Arc::new(GarbageCollector::new();
    
    let config = ConcurrentGcConfig {
        collection_interval_ms: 50,    // Very frequent collections
        time_budget_ms: 20,            // Short time budget
        heap_threshold_bytes: 512,     // Very low threshold
        thread_count: 1,
        max_pause_ms: 5,               // Very short pauses
        verbose_logging: true,
    };
    
    let concurrent_gc = ConcurrentGarbageCollector::with_config(gc.clone(), config);
    
    // Spawn multiple threads to allocate objects concurrently
    let thread_count = 4;
    let iterations_per_thread = 100;
    
    let mut handles = Vec::new();
    
    for t in 0..thread_count {
        let cgc = concurrent_gc.clone();
        
        let handle = thread::spawn(move || {
            let mut local_objects = Vec::new();
            
            for i in 0..iterations_per_thread {
                // Allocate an object
                let obj = cgc.allocate(TestObject::new(t * 1000 + i);
                
                // Sometimes create a chain
                if !local_objects.is_empty() && i % 10 == 0 {
                    local_objects.last().unwrap().inner().unwrap().set_next(obj.clone();
                }
                
                local_objects.push(obj);
                
                // Occasionally drop some objects
                if i % 20 == 19 {
                    local_objects.drain(0..local_objects.len()/2);
                }
                
                // Small sleep to give other threads a chance
                if i % 10 == 0 {
                    thread::sleep(Duration::from_millis(1);
                }
            }
            
            // Return the remaining objects
            local_objects
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut all_objects = Vec::new();
    for handle in handles {
        let objects = handle.join().unwrap();
        all_objects.extend(objects);
    }
    
    // Verify all objects are still accessible
    for obj in &all_objects {
        assert!(obj.inner().is_some())
    }
    
    // Drop half the objects
    all_objects.drain(0..all_objects.len()/2);
    
    // Force a collection
    concurrent_gc.request_collection();
    
    // Sleep to allow collection to complete
    thread::sleep(Duration::from_millis(200);
    
    // Verify remaining objects are still accessible
    for obj in &all_objects {
        assert!(obj.inner().is_some())
    }
    
    // Drop all objects
    drop(all_objects);
    
    // Force final collection
    concurrent_gc.request_collection();
    
    // Sleep to allow collection to complete
    thread::sleep(Duration::from_millis(200);
    
    // Check GC statistics
    let stats = concurrent_gc.stats();
    println!("Final GC Stats after stress test: {:?}", stats);
}