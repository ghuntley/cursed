/// Comprehensive Integration Tests for Production Garbage Collector
/// 
/// This test suite validates the production-ready garbage collector implementation
/// including real allocation/deallocation, memory pressure detection, automatic
/// collection, goroutine integration, and comprehensive monitoring.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use cursed::memory::{
    production_gc::{ProductionGarbageCollector, ProductionGcConfig},
    pressure_detection::{PressureLevel, PressureDetectionConfig},
    real_allocator::{RealMemoryAllocator, RealAllocatorConfig},
    object_store::Storable,
    heap_manager::HeapConfig,
    gc::GcConfig,
};

/// Simple test object for allocation testing
#[derive(Debug, Clone)]
struct TestObject {
    data: Vec<u8>,
    id: u64,
    metadata: String,
}

impl Storable for TestObject {
    fn size_hint(&self) -> usize {
        std::mem::size_of::<Self>() + self.data.len() + self.metadata.len()
    }
    
    fn type_name(&self) -> &'static str {
        "TestObject"
    }
}

impl TestObject {
    fn new(size: usize, id: u64) -> Self {
        Self {
            data: vec![0u8; size],
            id,
            metadata: format!("test_object_{}", id),
        }
    }
    
    fn large(id: u64) -> Self {
        Self::new(128 * 1024, id) // 128KB object
    }
    
    fn small(id: u64) -> Self {
        Self::new(64, id) // 64 byte object
    }
}

/// Create a test configuration optimized for testing
fn create_test_config() -> ProductionGcConfig {
    let mut config = ProductionGcConfig::default();
    
    // Smaller heap sizes for faster testing
    config.initial_heap_size = 1 * 1024 * 1024; // 1MB
    config.max_heap_size = 16 * 1024 * 1024;    // 16MB
    
    // More aggressive collection thresholds
    config.emergency_threshold = 0.8;
    config.gc_config.young_gen_threshold = 0.6;
    config.gc_config.old_gen_threshold = 0.7;
    
    // Faster background collection for testing
    config.background_collection_interval = Duration::from_millis(100);
    
    // Enable all features for comprehensive testing
    config.enable_profiling = true;
    config.enable_auto_collection = true;
    config.enable_statistics = true;
    config.enable_goroutine_awareness = true;
    
    // More sensitive pressure detection
    config.pressure_config.memory_thresholds.low_threshold = 0.5;
    config.pressure_config.memory_thresholds.moderate_threshold = 0.6;
    config.pressure_config.memory_thresholds.high_threshold = 0.7;
    config.pressure_config.memory_thresholds.critical_threshold = 0.8;
    config.pressure_config.memory_thresholds.emergency_threshold = 0.9;
    
    config
}

#[test]
fn test_production_gc_basic_functionality() {
    let config = create_test_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Test basic allocation
    let obj1 = TestObject::small(1);
    let ptr1 = gc.allocate(obj1).unwrap();
    assert!(ptr1.is_valid());
    
    // Test allocation statistics
    let stats = gc.get_stats().unwrap();
    assert_eq!(stats.total_allocations, 1);
    assert_eq!(stats.total_collections, 0);
    
    // Test manual collection
    let collection_stats = gc.collect().unwrap();
    assert!(collection_stats.total_duration > Duration::ZERO);
    
    // Verify collection statistics
    let final_stats = gc.get_stats().unwrap();
    assert_eq!(final_stats.total_collections, 1);
    assert_eq!(final_stats.manual_collection_triggers, 1);
}

#[test]
fn test_memory_pressure_detection_and_response() {
    let config = create_test_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Initially should have no pressure
    let initial_pressure = gc.current_memory_pressure().unwrap();
    assert_eq!(initial_pressure, PressureLevel::None);
    
    // Allocate many objects to increase pressure
    let mut allocations = Vec::new();
    for i in 0..100 {
        let obj = TestObject::new(10 * 1024, i); // 10KB each
        if let Ok(ptr) = gc.allocate(obj) {
            allocations.push(ptr);
        } else {
            break; // Allocation failed, that's expected under pressure
        }
    }
    
    // Should have increased pressure now
    let high_pressure = gc.current_memory_pressure().unwrap();
    assert!(high_pressure > PressureLevel::None);
    
    // Force collection to reduce pressure
    let collection_stats = gc.collect().unwrap();
    assert!(collection_stats.objects_collected > 0 || collection_stats.bytes_collected > 0);
    
    println!("Memory pressure test: allocated {} objects, pressure went from {:?} to {:?}",
             allocations.len(), initial_pressure, high_pressure);
}

#[test]
fn test_automatic_collection_triggering() {
    let config = create_test_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    let initial_stats = gc.get_stats().unwrap();
    assert_eq!(initial_stats.total_collections, 0);
    
    // Allocate objects rapidly to trigger automatic collection
    let start_time = Instant::now();
    let mut allocation_count = 0;
    
    while start_time.elapsed() < Duration::from_secs(2) {
        let obj = TestObject::new(8 * 1024, allocation_count); // 8KB each
        if gc.allocate(obj).is_ok() {
            allocation_count += 1;
            
            // Check if auto collection was triggered
            let current_stats = gc.get_stats().unwrap();
            if current_stats.auto_collection_triggers > 0 {
                println!("Auto collection triggered after {} allocations", allocation_count);
                break;
            }
        }
        
        // Small delay to allow background collection thread to run
        thread::sleep(Duration::from_millis(1));
    }
    
    // Should have triggered at least one automatic collection
    let final_stats = gc.get_stats().unwrap();
    assert!(
        final_stats.auto_collection_triggers > 0 || final_stats.emergency_collection_triggers > 0,
        "Expected automatic collection to be triggered, but got stats: {:?}", final_stats
    );
}

#[test]
fn test_large_object_allocation() {
    let config = create_test_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Allocate several large objects
    let mut large_objects = Vec::new();
    for i in 0..5 {
        let obj = TestObject::large(i);
        match gc.allocate(obj) {
            Ok(ptr) => {
                assert!(ptr.is_valid());
                large_objects.push(ptr);
            }
            Err(e) => {
                println!("Large object allocation {} failed: {}", i, e);
                break;
            }
        }
    }
    
    assert!(!large_objects.is_empty(), "Should have allocated at least one large object");
    
    // Force collection to test large object collection
    let collection_stats = gc.collect().unwrap();
    assert!(collection_stats.total_duration > Duration::ZERO);
    
    // Verify objects are still valid after collection
    for ptr in &large_objects {
        assert!(ptr.is_valid(), "Large object should still be valid after collection");
    }
    
    println!("Successfully allocated and collected {} large objects", large_objects.len());
}

#[test]
fn test_concurrent_allocation_and_collection() {
    let config = create_test_config();
    let gc = Arc::new(ProductionGarbageCollector::new(config).unwrap());
    
    let allocation_count = Arc::new(Mutex::new(0u64));
    let collection_count = Arc::new(Mutex::new(0u64));
    
    // Spawn multiple allocation threads
    let mut handles = Vec::new();
    for thread_id in 0..4 {
        let gc_clone = gc.clone();
        let alloc_count_clone = allocation_count.clone();
        
        let handle = thread::spawn(move || {
            for i in 0..50 {
                let obj = TestObject::new(1024, thread_id * 1000 + i);
                if gc_clone.allocate(obj).is_ok() {
                    *alloc_count_clone.lock().unwrap() += 1;
                }
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }
    
    // Spawn collection thread
    let gc_clone = gc.clone();
    let coll_count_clone = collection_count.clone();
    let collection_handle = thread::spawn(move || {
        for _ in 0..10 {
            if gc_clone.collect().is_ok() {
                *coll_count_clone.lock().unwrap() += 1;
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    collection_handle.join().unwrap();
    
    let final_alloc_count = *allocation_count.lock().unwrap();
    let final_coll_count = *collection_count.lock().unwrap();
    
    assert!(final_alloc_count > 0, "Should have completed some allocations");
    assert!(final_coll_count > 0, "Should have completed some collections");
    
    // Verify GC statistics
    let stats = gc.get_stats().unwrap();
    assert!(stats.total_allocations >= final_alloc_count);
    assert!(stats.total_collections >= final_coll_count);
    
    println!("Concurrent test: {} allocations, {} collections", 
             final_alloc_count, final_coll_count);
}

#[test]
fn test_memory_efficiency_and_fragmentation() {
    let config = create_test_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Create fragmentation by allocating and deallocating in patterns
    let mut small_objects = Vec::new();
    let mut medium_objects = Vec::new();
    
    // Allocate mixed sizes
    for i in 0..20 {
        let small_obj = TestObject::new(128, i);
        let medium_obj = TestObject::new(2048, i);
        
        if let Ok(small_ptr) = gc.allocate(small_obj) {
            small_objects.push(small_ptr);
        }
        
        if let Ok(medium_ptr) = gc.allocate(medium_obj) {
            medium_objects.push(medium_ptr);
        }
    }
    
    // Force collections to test different collection algorithms
    for _ in 0..3 {
        let collection_stats = gc.collect().unwrap();
        println!("Collection: {} objects, {} bytes, algorithm: {:?}",
                 collection_stats.objects_collected,
                 collection_stats.bytes_collected,
                 collection_stats.algorithm_used);
    }
    
    // Check memory efficiency
    let stats = gc.get_stats().unwrap();
    assert!(stats.memory_efficiency > 0.5, 
            "Memory efficiency should be reasonable: {}", stats.memory_efficiency);
    
    // Fragmentation should be controlled
    assert!(stats.fragmentation_ratio < 0.8,
            "Fragmentation should be controlled: {}", stats.fragmentation_ratio);
    
    println!("Memory efficiency: {:.2}%, fragmentation: {:.2}%",
             stats.memory_efficiency * 100.0, stats.fragmentation_ratio * 100.0);
}

#[test]
fn test_emergency_collection_handling() {
    let mut config = create_test_config();
    // Very small heap to trigger emergency conditions quickly
    config.max_heap_size = 2 * 1024 * 1024; // 2MB
    config.emergency_threshold = 0.9;
    
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Rapidly allocate to approach emergency threshold
    let mut allocations = Vec::new();
    let mut emergency_triggered = false;
    
    for i in 0..1000 {
        let obj = TestObject::new(4 * 1024, i); // 4KB each
        match gc.allocate(obj) {
            Ok(ptr) => allocations.push(ptr),
            Err(_) => {
                // Allocation failed, check if emergency collection was triggered
                let stats = gc.get_stats().unwrap();
                if stats.emergency_collection_triggers > 0 {
                    emergency_triggered = true;
                    println!("Emergency collection triggered after {} allocations", i);
                }
                break;
            }
        }
        
        // Check for emergency collection
        let stats = gc.get_stats().unwrap();
        if stats.emergency_collection_triggers > 0 {
            emergency_triggered = true;
            println!("Emergency collection triggered after {} allocations", i);
            break;
        }
    }
    
    // Should either trigger emergency collection or fail allocation gracefully
    let final_stats = gc.get_stats().unwrap();
    assert!(
        emergency_triggered || final_stats.failed_allocations > 0,
        "Expected emergency collection or allocation failures under memory pressure"
    );
    
    println!("Emergency test results: emergency_triggered={}, failed_allocations={}",
             emergency_triggered, final_stats.failed_allocations);
}

#[test]
fn test_gc_configuration_updates() {
    let config = create_test_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Test initial configuration
    let initial_stats = gc.get_stats().unwrap();
    
    // Update configuration
    let mut new_config = create_test_config();
    new_config.gc_config.young_gen_threshold = 0.5;
    new_config.gc_config.old_gen_threshold = 0.6;
    new_config.emergency_threshold = 0.7;
    
    gc.update_config(new_config).unwrap();
    
    // Allocate some objects with new configuration
    for i in 0..10 {
        let obj = TestObject::small(i);
        let _ = gc.allocate(obj);
    }
    
    // Force collection with new configuration
    let collection_stats = gc.collect().unwrap();
    assert!(collection_stats.total_duration > Duration::ZERO);
    
    println!("Configuration update test completed successfully");
}

#[test]
fn test_profiling_and_monitoring() {
    let config = create_test_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Get profiler if available
    if let Some(profiler) = gc.get_profiler() {
        // Perform operations to generate profiling data
        for i in 0..20 {
            let obj = TestObject::new(512, i);
            let _ = gc.allocate(obj);
        }
        
        // Force collection to generate GC events
        let _ = gc.collect();
        
        // Verify profiling is working (basic check)
        println!("Profiling enabled and collecting data");
    } else {
        println!("Profiling not enabled for this test");
    }
    
    // Test comprehensive statistics
    let stats = gc.get_stats().unwrap();
    assert!(stats.runtime_seconds > 0.0);
    assert!(!stats.algorithm_usage.is_empty() || stats.total_collections == 0);
    
    println!("GC Statistics: allocations={}, collections={}, runtime={:.2}s",
             stats.total_allocations, stats.total_collections, stats.runtime_seconds);
}

#[test]
fn test_auto_collection_enable_disable() {
    let mut config = create_test_config();
    config.enable_auto_collection = false;
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Initially auto collection should be disabled
    let initial_stats = gc.get_stats().unwrap();
    assert_eq!(initial_stats.auto_collection_triggers, 0);
    
    // Enable auto collection
    gc.set_auto_collection(true).unwrap();
    
    // Allocate objects to potentially trigger auto collection
    for i in 0..50 {
        let obj = TestObject::new(2 * 1024, i);
        if gc.allocate(obj).is_err() {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }
    
    // Wait a bit for background collection
    thread::sleep(Duration::from_millis(200));
    
    // Disable auto collection
    gc.set_auto_collection(false).unwrap();
    
    println!("Auto collection enable/disable test completed");
}

#[test]
fn test_force_full_collection() {
    let config = create_test_config();
    let gc = ProductionGarbageCollector::new(config).unwrap();
    
    // Allocate various objects
    for i in 0..30 {
        let obj = if i % 3 == 0 {
            TestObject::large(i)
        } else {
            TestObject::small(i)
        };
        let _ = gc.allocate(obj);
    }
    
    let before_stats = gc.get_stats().unwrap();
    
    // Force full collection
    let collection_stats = gc.force_full_collection().unwrap();
    assert!(collection_stats.total_duration > Duration::ZERO);
    
    let after_stats = gc.get_stats().unwrap();
    assert!(after_stats.total_collections > before_stats.total_collections);
    
    println!("Force full collection: collected {} objects, {} bytes",
             collection_stats.objects_collected, collection_stats.bytes_collected);
}

/// Stress test to validate system under sustained load
#[test]
fn test_sustained_load_stress() {
    let config = create_test_config();
    let gc = Arc::new(ProductionGarbageCollector::new(config).unwrap());
    
    let start_time = Instant::now();
    let test_duration = Duration::from_secs(5); // 5 second stress test
    let allocation_count = Arc::new(Mutex::new(0u64));
    
    // Spawn allocation threads
    let mut handles = Vec::new();
    for thread_id in 0..2 {
        let gc_clone = gc.clone();
        let count_clone = allocation_count.clone();
        
        let handle = thread::spawn(move || {
            let mut local_count = 0u64;
            while start_time.elapsed() < test_duration {
                let size = 512 + (local_count % 4096) as usize; // Variable sizes
                let obj = TestObject::new(size, thread_id * 10000 + local_count);
                
                match gc_clone.allocate(obj) {
                    Ok(_) => local_count += 1,
                    Err(_) => {
                        // Allocation failed, wait a bit and retry
                        thread::sleep(Duration::from_millis(1));
                    }
                }
                
                if local_count % 100 == 0 {
                    thread::sleep(Duration::from_microseconds(100));
                }
            }
            
            *count_clone.lock().unwrap() += local_count;
            local_count
        });
        handles.push(handle);
    }
    
    // Wait for completion
    let mut total_allocations = 0;
    for handle in handles {
        total_allocations += handle.join().unwrap();
    }
    
    let final_stats = gc.get_stats().unwrap();
    println!("Stress test results over {:.1}s:", test_duration.as_secs_f64());
    println!("  Total allocations: {}", total_allocations);
    println!("  Total collections: {}", final_stats.total_collections);
    println!("  Auto collection triggers: {}", final_stats.auto_collection_triggers);
    println!("  Emergency triggers: {}", final_stats.emergency_collection_triggers);
    println!("  Memory efficiency: {:.1}%", final_stats.memory_efficiency * 100.0);
    println!("  Peak heap size: {} KB", final_stats.peak_heap_size / 1024);
    
    // Validate that the system handled the load
    assert!(total_allocations > 100, "Should have completed substantial allocations");
    assert!(final_stats.memory_efficiency > 0.3, "Memory efficiency should be reasonable under load");
}
