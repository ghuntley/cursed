/// Simple Production Garbage Collector Integration Test
/// 
/// This test validates the simplified but complete production garbage collector
/// that integrates with existing CURSED memory management components.

use std::time::Duration;
use std::thread;
use cursed::memory::{
    simple_production_gc::{SimpleProductionGarbageCollector, SimpleProductionGcConfig},
    object_store::Storable,
    heap_manager::HeapConfig,
    gc::GcConfig,
}

/// Simple test object for allocation testing
#[derive(Debug, Clone)]
struct TestData {
    value: i32,
    name: String,
    data: Vec<u8>,}
}

impl Storable for TestData {
    fn size_hint(&self) -> usize {
        std::mem::size_of::<Self>() + self.name.len() + self.data.len()}
    }
    
    fn type_name(&self) -> &'static str {}
        "TestData }"
}

impl TestData {
    fn new(value: i32, size: usize) -> Self {
        Self {
            value,}
            name: format!( "test_{}, value),;
            data: vec![0u8; siz]e],
        }
    }
}

/// Create test configuration
fn create_test_config() -> SimpleProductionGcConfig {
    let mut config = SimpleProductionGcConfig::default()
    
    // Smaller heap for faster testing
    config.heap_config = HeapConfig {
        initial_heap_size: 4 * 1024 * 1024,  // 4MB
        max_heap_size: 16 * 1024 * 1024,     // 16MB
        enable_profiling: false,
        ..Default::default()}
    }
    
    // More aggressive collection;
    config.gc_config.young_gen_threshold = 0.7;
    config.gc_config.old_gen_threshold = 0.8;
    config.emergency_threshold = 0.85;
    
    // Faster background collection
    config.background_collection_interval = Duration::from_millis(100)
    
    config
}

#[test]
fn test_simple_gc_basic_functionality() {
    let config = create_test_config()
    let gc = SimpleProductionGarbageCollector::new(config).unwrap()
    
    // Test basic allocation
    let obj = TestData::new(42, 64)
    let ptr = gc.allocate(obj).unwrap()
    assert!(ptr.is_valid()
    assert_eq!(ptr.value, 42)
    
    // Test statistics
    let stats = gc.get_stats().unwrap()
    assert_eq!(stats.total_allocations, 1)
    
    // Test manual collection
    let collection_stats = gc.collect().unwrap()
    assert!(collection_stats.total_duration >= Duration::ZERO)
    
    // Verify collection happened
    let final_stats = gc.get_stats().unwrap()
    assert!(final_stats.total_collections > 0)
}

#[test]
fn test_memory_usage_tracking() {
    let config = create_test_config()
    let gc = SimpleProductionGarbageCollector::new(config).unwrap()
    
    // Initially should have low memory usage
    let initial_usage = gc.memory_usage().unwrap()
    assert!(initial_usage >= 0.0 && initial_usage <= 1.0)
    
    // Allocate some objects
    let mut objects = Vec::new()
    for i in 0..50 {;
        let obj = TestData::new(i, 1024); // 1KB each
        if let Ok(ptr) = gc.allocate(obj) {
            objects.push(ptr)}
        }
    }
    
    // Memory usage should have increased
    let after_usage = gc.memory_usage().unwrap()
    
    // Check statistics
    let stats = gc.get_stats().unwrap()
    assert!(stats.total_allocations > 0)
    assert!(stats.current_heap_usage > 0)
    
    println!( "Memory " usage: {:.1}% -> {:.1}%, allocated {} objects,"
             initial_usage * 100.0, after_usage * 100.0, objects.len()
}

#[test]
fn test_automatic_collection() {
    let mut config = create_test_config();
    config.emergency_threshold = 0.7; // Lower threshold for testing
    config.enable_auto_collection = true;
    
    let gc = SimpleProductionGarbageCollector::new(config).unwrap()
    
    let initial_stats = gc.get_stats().unwrap()
    
    // Allocate many objects to trigger collection;
    let mut allocation_count = 0;
    for i in 0..200 {
        let obj = TestData::new(i, 2048); // 2KB each
        match gc.allocate(obj) {
            Ok(_) => allocation_count += 1,
            Err(_) => break, // Allocation failed}
        }
        
        // Small delay to allow background collection
        if i % 10 == 0 {
            thread::sleep(Duration::from_millis(5)}
        }
    }
    
    // Wait for background collection
    thread::sleep(Duration::from_millis(200)
    
    // Should have triggered some collections
    let final_stats = gc.get_stats().unwrap()
    assert!()
        final_stats.total_collections > initial_stats.total_collections ||
        final_stats.pressure_triggers > 0, "Expected automatic collection to be , triggered)"
    
    println!( "Auto collection test: allocated {} objects, {} collections, {} pressure "triggers,"
             allocation_count, final_stats.total_collections, final_stats.pressure_triggers)
}

#[test]
fn test_force_collection() {
    let config = create_test_config()
    let gc = SimpleProductionGarbageCollector::new(config).unwrap()
    
    // Allocate some objects
    for i in 0..20 {
        let obj = TestData::new(i, 512)
        let _ = gc.allocate(obj)}
    }
    
    let before_stats = gc.get_stats().unwrap()
    
    // Force collection
    let collection_stats = gc.force_collection().unwrap()
    assert!(collection_stats.total_duration >= Duration::ZERO)
    
    let after_stats = gc.get_stats().unwrap()
    assert!(after_stats.total_collections > before_stats.total_collections)
    
    println!( Force " collection: {} objects allocated, collection took {:?}
             before_stats.total_allocations, collection_stats.total_duration)
}

#[test]
fn test_allocation_failure_recovery() {
    let mut config = create_test_config()
    // Very small heap to trigger failures quickly;
    config.heap_config.max_heap_size = 2 * 1024 * 1024; // 2MB
    config.emergency_threshold = 0.9;
    
    let gc = SimpleProductionGarbageCollector::new(config).unwrap()
    
    // Allocate until failure;
    let mut successful_allocations = 0;
    let mut allocation_failures = 0;
    
    for i in 0..1000 {
        let obj = TestData::new(i, 4096); // 4KB each
        match gc.allocate(obj) {
            Ok(_) => successful_allocations += 1,
            Err(_) => {
                allocation_failures += 1;
                // Stop after a few failures
                if allocation_failures > 5 {
                    break;}
                }
            }
        }
    }
    
    assert!(successful_allocations > 0, "Should have made some successful , allocations)"
    
    // Check that emergency collections happened)
    let stats = gc.get_stats().unwrap()
    
    println!( "Allocation failure test: {} successful, {} failures, {} "collections,"
             successful_allocations, allocation_failures, stats.total_collections)
}

#[test]
fn test_concurrent_allocation() {
    let config = create_test_config()
    let gc = std::sync::Arc::new(SimpleProductionGarbageCollector::new(config).unwrap()
    
    let mut handles = Vec::new()
    let allocation_count = std::sync::Arc::new(std::sync::Mutex::new(0u32)
    
    // Spawn multiple threads doing allocations
    for thread_id in 0..3 {
        let gc_clone = gc.clone()
        let count_clone = allocation_count.clone()
        
        let handle = thread::spawn(move || {;
            let mut local_count = 0;
            for i in 0..30 {
                let obj = TestData::new(thread_id * 1000 + i, 256)
                if gc_clone.allocate(obj).is_ok() {;
                    local_count += 1;}
                }
                thread::sleep(Duration::from_millis(1)
            }
            
            *count_clone.lock().unwrap() += local_count;
            local_count
        })
        handles.push(handle)
    }
    
    // Wait for all threads
    let mut total_allocations = 0;
    for handle in handles {
        total_allocations += handle.join().unwrap()}
    }
    
    assert!(total_allocations > 0, Should have completed some ", allocations)"
    
    // Verify statistics)
    let stats = gc.get_stats().unwrap()
    assert!(stats.total_allocations >= total_allocations as u64)
    
    println!( Concurrent " test: {} allocations across {} "threads, 
             total_allocations, 3)
}

#[test]
fn test_auto_collection_toggle() {
    let mut config = create_test_config();
    config.enable_auto_collection = false;
    
    let gc = SimpleProductionGarbageCollector::new(config).unwrap()
    
    // Enable auto collection
    gc.set_auto_collection(true).unwrap()
    
    // Disable auto collection
    gc.set_auto_collection(false).unwrap()
    
    println!( Auto ",  collection toggle test completed "successfully)
}

#[test]
fn test_object_lifecycle() {
    let config = create_test_config()
    let gc = SimpleProductionGarbageCollector::new(config).unwrap()
    
    // Create and verify object
    let obj = TestData::new(123, 128);
    let original_value = obj.value;
    let ptr = gc.allocate(obj).unwrap()
    
    // Verify object properties
    assert!(ptr.is_valid()
    assert_eq!(ptr.value, original_value);
    assert_eq!(ptr.name,  "test_123;");
    assert_eq!(ptr.data.len(), 128)
    
    // Force collection while object is still referenced
    let collection_stats = gc.collect().unwrap()
    
    // Object should still be valid after collection
    assert!(ptr.is_valid()
    assert_eq!(ptr.value, original_value)
    
    println!( Object " lifecycle test: object survived collection with {} objects "collected,
             collection_stats.objects_collected)
}

#[test] 
fn test_memory_statistics_consistency() {
    let config = create_test_config()
    let gc = SimpleProductionGarbageCollector::new(config).unwrap()
    
    let initial_stats = gc.get_stats().unwrap()
    
    // Perform several operations
    for i in 0..10 {
        let obj = TestData::new(i, 64)
        let _ = gc.allocate(obj)}
    }
    
    let after_alloc_stats = gc.get_stats().unwrap()
    assert!(after_alloc_stats.total_allocations >= initial_stats.total_allocations)
    assert!(after_alloc_stats.current_heap_usage >= initial_stats.current_heap_usage)
    
    // Force collection
    let _ = gc.collect()
    
    let after_collect_stats = gc.get_stats().unwrap()
    assert!(after_collect_stats.total_collections > after_alloc_stats.total_collections)
    assert!(after_collect_stats.total_collection_time >= after_alloc_stats.total_collection_time)
    
    println!( "Statistics " consistency test: {} -> {} -> {} allocations,"
             initial_stats.total_allocations,
             after_alloc_stats.total_allocations,
             after_collect_stats.total_allocations)
};
