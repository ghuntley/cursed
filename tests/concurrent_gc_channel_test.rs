use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use cursed::memory::{Traceable, Tag, Visitor, GarbageCollector, ThreadSafeGc, ConcurrentGarbageCollector};
use cursed::memory::concurrent_gc::ConcurrentGcConfig;
use cursed::runtime::channel_gc::ThreadSafeChannel;
use cursed::object_thread_safe::ThreadSafeObject;
use common::tracing::setup as init_tracing;

extern crate cursed;

#[path = "common/mod.rs"]
mod common;



// Initialize tracing for tests

// Test data object for channels
#[derive(Debug, Clone)]
struct TestData {
    value: Arc<Mutex<i32>>,
}

impl TestData {
    fn new(value: i32) -> Self {
        Self {
            value: Arc::new(Mutex::new(value)),
        }
    }
    
    fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }
    
    fn set_value(&self, value: i32) {
        *self.value.lock().unwrap() = value;
    }
}

impl Traceable for TestData {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // No references to trace
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

unsafe impl Send for TestData {}
unsafe impl Sync for TestData {}

#[test]
fn test_channel_with_concurrent_gc() {
    // Initialize tracing
    init_tracing();
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a concurrent garbage collector with a custom configuration
    let config = ConcurrentGcConfig {
        collection_interval_ms: 100,  // Collect every 100ms
        time_budget_ms: 50,          // 50ms per collection
        heap_threshold_bytes: 1024,   // 1KB threshold
        thread_count: 1,              // Single collector thread
        max_pause_ms: 10,             // 10ms max pause
        verbose_logging: true,        // Enable verbose logging
    };
    
    let concurrent_gc = ConcurrentGarbageCollector::with_config(gc.clone(), config);
    
    // Create a thread-safe channel
    let channel = ThreadSafeChannel::new("TestData".to_string(), 10);
    let channel_obj = ThreadSafeObject::Channel(Arc::new(channel));
    
    // Allocate some test data objects
    let mut data_objects = Vec::new();
    for i in 0..50 {
        let data = concurrent_gc.allocate(TestData::new(i));
        data_objects.push(data);
    }
    
    // Send half the objects through the channel
    for i in 0..25 {
        let channel_gc = concurrent_gc.allocate(channel_obj.clone());
        let obj = data_objects[i].clone();
        
        // Send the object through the channel
        if let Some(channel) = channel_gc.inner() {
            // Convert TestData to ThreadSafeObject::Integer
            let value = obj.inner().unwrap().get_value();
            let thread_safe_obj = ThreadSafeObject::Integer(value as i64);
            let obj_gc = concurrent_gc.allocate(thread_safe_obj);
            match channel.channel_send(obj_gc) {
                Ok(_) => {
                    println!("Sent object {} to channel", i);
                },
                Err(e) => {
                    println!("Failed to send object {}: {}", i, e);
                }
            }
        }
    }
    
    // Force a collection while objects are in the channel
    concurrent_gc.request_collection();
    
    // Sleep to allow collection to run
    thread::sleep(Duration::from_millis(200));
    
    // Receive objects from the channel
    let channel_gc = concurrent_gc.allocate(channel_obj.clone());
    let mut received_objects = Vec::new();
    
    for _ in 0..25 {
        if let Some(channel) = channel_gc.inner() {
            match channel.channel_receive(&gc) {
                Ok(obj) => {
                    received_objects.push(obj);
                },
                Err(e) => {
                    println!("Failed to receive object: {}", e);
                }
            }
        }
    }
    
    // Verify we received all objects
    assert_eq!(received_objects.len(), 25, "Should have received 25 objects");
    
    // Verify the objects are valid
    for (i, obj) in received_objects.iter().enumerate() {
        if let Some(data) = obj.inner() {
        // TestData objects are stored in ThreadSafeObject::Integer form
            match data {
            ThreadSafeObject::Integer(val) => {
                    assert_eq!(*val, i as i64, "Object value should match index");
                    },
                    _ => panic!("Expected integer object, got {:?}", data),
                }
            } else {
                panic!("Failed to access object {}", i);
            }
    }
    
    // Drop all objects except the channel
    drop(data_objects);
    drop(received_objects);
    
    // Force another collection
    concurrent_gc.request_collection();
    
    // Sleep to allow collection to run
    thread::sleep(Duration::from_millis(200));
    
    // Check GC statistics
    let stats = concurrent_gc.stats();
    println!("Final GC stats: {:?}", stats);
}

#[test]
fn test_concurrent_channel_operations() {
    // Initialize tracing
    init_tracing();
    
    // Create a garbage collector
    let gc = Arc::new(GarbageCollector::new());
    
    // Create a concurrent garbage collector
    let config = ConcurrentGcConfig {
        collection_interval_ms: 50,   // More frequent collections
        time_budget_ms: 10,           // Shorter time budget
        heap_threshold_bytes: 512,    // Lower threshold
        thread_count: 1,
        max_pause_ms: 5,
        verbose_logging: true,
    };
    
    let concurrent_gc = ConcurrentGarbageCollector::with_config(gc.clone(), config);
    
    // Create an unbuffered channel to force synchronization
    let channel = ThreadSafeChannel::new("TestData".to_string(), 0);
    let channel_obj = ThreadSafeObject::Channel(Arc::new(channel));
    let channel_gc = concurrent_gc.allocate(channel_obj.clone());
    
    // Spawn sender threads
    let num_threads = 4;
    let items_per_thread = 10;
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let concurrent_gc_clone = concurrent_gc.clone();
        let channel_gc_clone = channel_gc.clone();
        
        let handle = thread::spawn(move || {
            for i in 0..items_per_thread {
                let value = thread_id * 100 + i;
                let data = concurrent_gc_clone.allocate(TestData::new(value));
                
                // Send the data through the channel
                if let Some(channel) = channel_gc_clone.inner() {
                    // Convert TestData to ThreadSafeObject::Integer
                    let value = data.inner().unwrap().get_value();
                    let thread_safe_obj = ThreadSafeObject::Integer(value as i64);
                    let obj_gc = concurrent_gc_clone.allocate(thread_safe_obj);
                    match channel.channel_send(obj_gc) {
                        Ok(_) => {
                            println!("Thread {} sent value {}", thread_id, value);
                        },
                        Err(e) => {
                            println!("Thread {} failed to send value {}: {}", thread_id, value, e);
                        }
                    }
                }
                
                // Small sleep to allow interleaving
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        handles.push(handle);
    }
    
    // Spawn receiver threads
    let mut receiver_handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let gc_clone = gc.clone();
        let channel_gc_clone = channel_gc.clone();
        
        let handle = thread::spawn(move || {
            let mut received = Vec::new();
            
            for _ in 0..items_per_thread {
                if let Some(channel) = channel_gc_clone.inner() {
                    match channel.channel_receive(&gc_clone) {
                        Ok(obj) => {
                            if let Some(data) = obj.inner() {
                                match data {
                                    ThreadSafeObject::Integer(val) => {
                                        println!("Thread {} received value {}", thread_id, val);
                                        received.push(*val as i32);
                                    },
                                    _ => println!("Thread {} received non-integer object", thread_id),
                                }
                            }
                        },
                        Err(e) => {
                            println!("Thread {} failed to receive: {}", thread_id, e);
                        }
                    }
                }
            }
            
            received
        });
        
        receiver_handles.push(handle);
    }
    
    // Wait for all sender threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Wait for all receiver threads to complete and collect results
    let mut all_received = Vec::new();
    for handle in receiver_handles {
        let received = handle.join().unwrap();
        all_received.extend(received);
    }
    
    // Verify we received all items
    assert_eq!(all_received.len(), (num_threads * items_per_thread) as usize, 
               "Should have received all items");
    
    // Sort the received values for comparison
    all_received.sort();
    
    // Generate the expected values
    let mut expected = Vec::new();
    for thread_id in 0..num_threads {
        for i in 0..items_per_thread {
            expected.push(thread_id * 100 + i);
        }
    }
    expected.sort();
    
    // Verify all expected values were received
    assert_eq!(all_received, expected, "All expected values should have been received");
    
    // Force a final collection
    concurrent_gc.request_collection();
    
    // Sleep to allow collection to run
    thread::sleep(Duration::from_millis(200));
    
    // Check GC statistics
    let stats = concurrent_gc.stats();
    println!("Final GC stats for concurrent test: {:?}", stats);
}