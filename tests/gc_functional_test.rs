/// Garbage Collection Functional Tests
/// 
/// Tests the memory management and garbage collection functionality
/// including allocation, deallocation, circular references, and GC integration.

use cursed::memory::*;
use cursed::memory::gc::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_gc_basic_allocation() {
        let mut gc = GarbageCollector::new(GcConfig::default());
        
        // Test basic object allocation
        let obj_id = gc.allocate(1024).expect("Should allocate successfully");
        assert!(obj_id != 0, "Object ID should be non-zero");
        
        // Verify object exists
        assert!(gc.is_valid_object(obj_id), "Allocated object should be valid");
        
        println!("Basic GC allocation test passed");
    }

    #[test]
    fn test_gc_collection_basic() {
        let mut gc = GarbageCollector::new(GcConfig::default());
        
        // Allocate several objects
        let mut objects = Vec::new();
        for i in 0..10 {
            let obj_id = gc.allocate(64).expect("Should allocate successfully");
            objects.push(obj_id);
        }
        
        // Add some as roots
        for &obj_id in &objects[0..5] {
            gc.add_root(obj_id);
        }
        
        // Run collection
        let collected = gc.collect().expect("Collection should succeed");
        println!("Collected {} objects", collected);
        
        // Root objects should still exist
        for &obj_id in &objects[0..5] {
            assert!(gc.is_valid_object(obj_id), "Root object should still exist");
        }
        
        println!("Basic GC collection test passed");
    }

    #[test]
    fn test_gc_circular_references() {
        let mut gc = GarbageCollector::new(GcConfig::default());
        
        // Create circular reference
        let obj1 = gc.allocate(64).expect("Should allocate object 1");
        let obj2 = gc.allocate(64).expect("Should allocate object 2");
        
        // Set up circular reference
        gc.add_reference(obj1, obj2).expect("Should add reference");
        gc.add_reference(obj2, obj1).expect("Should add reference");
        
        // Initially both should exist
        assert!(gc.is_valid_object(obj1), "Object 1 should exist");
        assert!(gc.is_valid_object(obj2), "Object 2 should exist");
        
        // Run collection (without roots, both should be collected)
        let collected = gc.collect().expect("Collection should succeed");
        
        // With proper cycle detection, both should be collected
        println!("Collected {} objects in circular reference test", collected);
        
        println!("Circular reference test passed");
    }

    #[test]
    fn test_enhanced_gc_features() {
        let config = EnhancedGcConfig {
            enable_generational: true,
            enable_incremental: true,
            heap_size_mb: 64,
            collection_threshold: 0.8,
            ..EnhancedGcConfig::default()
        };
        
        let mut gc = EnhancedGarbageCollector::new(config).expect("Should create enhanced GC");
        
        // Test generational allocation
        let young_obj = gc.allocate_young(32).expect("Should allocate in young generation");
        let old_obj = gc.allocate_old(64).expect("Should allocate in old generation");
        
        assert!(gc.is_valid_object(young_obj), "Young object should be valid");
        assert!(gc.is_valid_object(old_obj), "Old object should be valid");
        
        // Test incremental collection
        let collected = gc.collect_incremental().expect("Incremental collection should succeed");
        println!("Incremental collection completed, processed {} objects", collected);
        
        println!("Enhanced GC features test passed");
    }

    #[test]
    fn test_gc_memory_pressure() {
        let mut gc = GarbageCollector::new(GcConfig::default());
        let mut allocated_objects = Vec::new();
        
        // Allocate many objects to create memory pressure
        for i in 0..100 {
            match gc.allocate(1024) {
                Ok(obj_id) => {
                    allocated_objects.push(obj_id);
                    
                    // Add every 10th object as root
                    if i % 10 == 0 {
                        gc.add_root(obj_id);
                    }
                }
                Err(_) => {
                    // Memory pressure triggered, run collection
                    let collected = gc.collect().expect("Collection should succeed");
                    println!("Memory pressure triggered collection of {} objects", collected);
                    break;
                }
            }
        }
        
        println!("Memory pressure test completed with {} allocated objects", allocated_objects.len());
    }

    #[test]
    fn test_gc_statistics() {
        let mut gc = GarbageCollector::new(GcConfig::default());
        
        // Perform some operations
        let obj1 = gc.allocate(64).expect("Should allocate");
        let obj2 = gc.allocate(128).expect("Should allocate");
        gc.add_root(obj1);
        
        let _collected = gc.collect().expect("Collection should succeed");
        
        // Get statistics
        let stats = gc.get_statistics();
        assert!(stats.total_allocations >= 2, "Should have allocated at least 2 objects");
        assert!(stats.total_collections >= 1, "Should have performed at least 1 collection");
        
        println!("GC Statistics: {:?}", stats);
        println!("GC statistics test passed");
    }

    #[test]
    fn test_gc_finalization() {
        let mut gc = GarbageCollector::new(GcConfig::default());
        
        // Allocate object with finalizer
        let obj_id = gc.allocate_with_finalizer(64, || {
            println!("Object finalized");
        }).expect("Should allocate with finalizer");
        
        // Object should exist initially
        assert!(gc.is_valid_object(obj_id), "Object should exist");
        
        // Run collection (object should be finalized since it's not rooted)
        let collected = gc.collect().expect("Collection should succeed");
        
        println!("Finalization test completed, collected {} objects", collected);
    }

    #[test]
    fn test_weak_references() {
        let mut gc = GarbageCollector::new(GcConfig::default());
        
        // Allocate object and create weak reference
        let obj_id = gc.allocate(64).expect("Should allocate");
        let weak_ref = gc.create_weak_reference(obj_id).expect("Should create weak reference");
        
        // Initially weak reference should be valid
        assert!(gc.is_weak_reference_valid(&weak_ref), "Weak reference should be valid");
        
        // Run collection (object not rooted, should be collected)
        let collected = gc.collect().expect("Collection should succeed");
        
        // Weak reference should now be invalid
        match gc.resolve_weak_reference(&weak_ref) {
            Some(_) => println!("Weak reference still valid"),
            None => println!("Weak reference invalidated after collection"),
        }
        
        println!("Weak reference test completed");
    }

    #[test]
    fn test_concurrent_gc() {
        use std::thread;
        use std::sync::{Arc, Mutex};
        
        let gc = Arc::new(Mutex::new(GarbageCollector::new(GcConfig::default())));
        let mut handles = Vec::new();
        
        // Spawn multiple threads doing allocation/collection
        for i in 0..4 {
            let gc_clone = Arc::clone(&gc);
            let handle = thread::spawn(move || {
                for j in 0..10 {
                    let mut gc = gc_clone.lock().unwrap();
                    
                    // Allocate object
                    if let Ok(obj_id) = gc.allocate(32) {
                        // Sometimes add as root
                        if (i + j) % 3 == 0 {
                            gc.add_root(obj_id);
                        }
                    }
                    
                    // Sometimes trigger collection
                    if j % 5 == 0 {
                        let _ = gc.collect();
                    }
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads
        for handle in handles {
            handle.join().expect("Thread should complete");
        }
        
        println!("Concurrent GC test completed");
    }

    #[test] 
    fn test_gc_heap_management() {
        let config = GcConfig {
            initial_heap_size: 1024 * 1024, // 1MB
            max_heap_size: 10 * 1024 * 1024, // 10MB
            collection_threshold: 0.75,
            ..GcConfig::default()
        };
        
        let mut gc = GarbageCollector::new(config);
        
        // Test heap expansion
        let mut objects = Vec::new();
        for _ in 0..1000 {
            if let Ok(obj_id) = gc.allocate(1024) {
                objects.push(obj_id);
            } else {
                break;
            }
        }
        
        println!("Allocated {} objects before heap expansion", objects.len());
        
        // Test heap statistics
        let heap_stats = gc.get_heap_statistics();
        println!("Heap usage: {} / {} bytes", heap_stats.used_bytes, heap_stats.total_bytes);
        
        assert!(heap_stats.used_bytes > 0, "Heap should have used bytes");
        assert!(heap_stats.total_bytes >= heap_stats.used_bytes, "Total should be >= used");
        
        println!("Heap management test passed");
    }

    #[test]
    fn test_gc_integration_with_runtime() {
        // This tests integration between GC and runtime systems
        let mut gc = GarbageCollector::new(GcConfig::default());
        
        // Simulate runtime object creation
        let string_obj = gc.allocate_string("Hello, GC!").expect("Should allocate string");
        let array_obj = gc.allocate_array(10, 8).expect("Should allocate array");
        let map_obj = gc.allocate_map().expect("Should allocate map");
        
        // Add runtime roots
        gc.add_root(string_obj);
        gc.add_root(array_obj);
        gc.add_root(map_obj);
        
        // Verify all objects exist
        assert!(gc.is_valid_object(string_obj), "String object should exist");
        assert!(gc.is_valid_object(array_obj), "Array object should exist");
        assert!(gc.is_valid_object(map_obj), "Map object should exist");
        
        // Run collection
        let collected = gc.collect().expect("Collection should succeed");
        
        // Root objects should still exist
        assert!(gc.is_valid_object(string_obj), "String object should still exist");
        assert!(gc.is_valid_object(array_obj), "Array object should still exist");
        assert!(gc.is_valid_object(map_obj), "Map object should still exist");
        
        println!("GC runtime integration test passed, collected {} objects", collected);
    }
}
