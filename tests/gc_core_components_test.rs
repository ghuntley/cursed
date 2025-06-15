/// Comprehensive tests for garbage collection core components
/// Tests memory allocation, deallocation, collection cycles, leak detection, and performance

#[path = "common.rs"]
pub mod common;

use cursed::memory::{
    GarbageCollector, GcConfig, CollectionAlgorithm, CollectionTrigger, 
    HeapManager, HeapConfig, ObjectRegistry, ObjectStore, RootSetManager,
    Gc, Tag, Traceable, Visitor
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, debug, error};

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[derive(Debug, Clone)]
struct TestObject {
    pub id: u32,
    pub data: Vec<u8>,
    pub children: Vec<Gc<TestObject>>,
}

impl Traceable for TestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for child in &self.children {
            child.trace(visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic GC initialization and configuration
    #[test]
    fn test_gc_initialization() {
        init_tracing!();
        info!("Testing GC initialization");

        let config = GcConfig {
            algorithm: CollectionAlgorithm::MarkSweep,
            generational: true,
            incremental: false,
            concurrent: false,
            goroutine_aware: false,
            young_gen_threshold: 0.75,
            old_gen_threshold: 0.85,
            emergency_threshold: 0.95,
            max_pause_time: Duration::from_millis(100),
            allocation_pressure_ratio: 0.8,
            adaptive_algorithm_selection: false,
        };

        let gc = GarbageCollector::new(config);
        assert!(gc.is_ok());
        
        let gc = gc.unwrap();
        let stats = gc.get_stats().unwrap();
        assert_eq!(stats.collections_performed, 0);
        assert_eq!(stats.objects_collected, 0);
        
        info!("GC initialization test passed");
    }

    /// Test memory allocation and deallocation
    #[test]
    fn test_memory_allocation_deallocation() {
        init_tracing!();
        info!("Testing memory allocation and deallocation");

        let config = GcConfig::default();
        let gc = GarbageCollector::new(config).unwrap();

        // Allocate test objects
        let mut allocated_objects = Vec::new();
        for i in 0..100 {
            let test_obj = TestObject {
                id: i,
                data: vec![i as u8; 1024], // 1KB per object
                children: vec![],
            };
            
            let gc_obj = gc.allocate(test_obj).unwrap();
            allocated_objects.push(gc_obj);
        }

        let stats = gc.get_stats().unwrap();
        assert!(stats.objects_allocated >= 100);
        assert!(stats.total_allocated_bytes >= 100 * 1024);

        // Test that objects are properly allocated
        for (i, obj) in allocated_objects.iter().enumerate() {
            assert_eq!(obj.id, i as u32);
            assert_eq!(obj.data.len(), 1024);
        }

        // Trigger collection to test deallocation
        let collection_result = gc.collect_garbage();
        assert!(collection_result.is_ok());

        info!("Memory allocation and deallocation test passed");
    }

    /// Test garbage collection cycles with reachability
    #[test] 
    fn test_gc_collection_cycles() {
        init_tracing!();
        info!("Testing garbage collection cycles");

        let config = GcConfig::default();
        let gc = GarbageCollector::new(config).unwrap();

        // Create objects with references
        let root = TestObject {
            id: 0,
            data: vec![0; 512],
            children: vec![],
        };
        let mut root_gc = gc.allocate(root).unwrap();

        // Create child objects
        for i in 1..=10 {
            let child = TestObject {
                id: i,
                data: vec![i as u8; 512],
                children: vec![],
            };
            let child_gc = gc.allocate(child).unwrap();
            root_gc.get_mut().unwrap().children.push(child_gc);
        }

        // Add root to root set
        gc.add_root(root_gc.clone()).unwrap();

        let stats_before = gc.get_stats().unwrap();
        debug!("Objects before collection: {}", stats_before.objects_allocated);

        // Trigger collection - root and children should survive
        let collection_result = gc.collect_garbage();
        assert!(collection_result.is_ok());

        let stats_after = gc.get_stats().unwrap();
        debug!("Objects after collection: {}", stats_after.objects_allocated - stats_after.objects_collected);

        // Root and its children should still be alive
        assert_eq!(root_gc.id, 0);
        assert_eq!(root_gc.children.len(), 10);

        info!("GC collection cycles test passed");
    }

    /// Test memory leak detection by creating unreachable objects
    #[test]
    fn test_memory_leak_detection() {
        init_tracing!();
        info!("Testing memory leak detection");

        let config = GcConfig::default();
        let gc = GarbageCollector::new(config).unwrap();

        let initial_stats = gc.get_stats().unwrap();

        // Create unreachable objects (potential leaks)
        for i in 0..50 {
            let leak_obj = TestObject {
                id: i + 1000,
                data: vec![0; 2048],
                children: vec![],
            };
            let _leaked = gc.allocate(leak_obj).unwrap();
            // Intentionally not keeping reference - should be collected
        }

        let stats_after_alloc = gc.get_stats().unwrap();
        assert!(stats_after_alloc.objects_allocated >= initial_stats.objects_allocated + 50);

        // Trigger collection - leaked objects should be collected
        let collection_result = gc.collect_garbage();
        assert!(collection_result.is_ok());

        let final_stats = gc.get_stats().unwrap();
        assert!(final_stats.objects_collected > 0);
        
        // Memory should be reclaimed
        assert!(final_stats.total_freed_bytes > 0);

        info!("Memory leak detection test passed");
    }

    /// Test GC performance characteristics
    #[test]
    fn test_gc_performance() {
        init_tracing!();
        info!("Testing GC performance characteristics");

        let config = GcConfig::default();
        let gc = GarbageCollector::new(config).unwrap();

        let allocation_start = Instant::now();
        
        // Allocate many objects to test performance
        let mut objects = Vec::new();
        for i in 0..1000 {
            let obj = TestObject {
                id: i,
                data: vec![i as u8; 512],
                children: vec![],
            };
            objects.push(gc.allocate(obj).unwrap());
        }

        let allocation_time = allocation_start.elapsed();
        debug!("Allocation time for 1000 objects: {:?}", allocation_time);

        // Test collection performance
        let collection_start = Instant::now();
        let collection_result = gc.collect_garbage();
        let collection_time = collection_start.elapsed();
        
        assert!(collection_result.is_ok());
        debug!("Collection time: {:?}", collection_time);

        // Performance assertions
        assert!(allocation_time < Duration::from_secs(1), "Allocation too slow");
        assert!(collection_time < Duration::from_millis(500), "Collection too slow");

        let stats = gc.get_stats().unwrap();
        assert!(stats.average_pause_time < Duration::from_millis(100));

        info!("GC performance test passed");
    }

    /// Test edge cases and error handling
    #[test]
    fn test_gc_edge_cases() {
        init_tracing!();
        info!("Testing GC edge cases and error handling");

        let config = GcConfig::default();
        let gc = GarbageCollector::new(config).unwrap();

        // Test with zero-sized objects
        let empty_obj = TestObject {
            id: 0,
            data: vec![],
            children: vec![],
        };
        let empty_gc = gc.allocate(empty_obj);
        assert!(empty_gc.is_ok());

        // Test with very large objects
        let large_obj = TestObject {
            id: 1,
            data: vec![0; 1024 * 1024], // 1MB
            children: vec![],
        };
        let large_gc = gc.allocate(large_obj);
        assert!(large_gc.is_ok());

        // Test multiple rapid collections
        for _ in 0..5 {
            let result = gc.collect_garbage();
            assert!(result.is_ok());
        }

        // Test collection with no objects
        let initial_stats = gc.get_stats().unwrap();
        let result = gc.collect_garbage();
        assert!(result.is_ok());
        let final_stats = gc.get_stats().unwrap();
        // Should complete successfully even with nothing to collect

        info!("GC edge cases test passed");
    }

    /// Test circular reference handling
    #[test]
    fn test_circular_references() {
        init_tracing!();
        info!("Testing circular reference handling");

        let config = GcConfig {
            algorithm: CollectionAlgorithm::CycleDetection,
            ..GcConfig::default()
        };
        let gc = GarbageCollector::new(config).unwrap();

        // Create circular reference
        let obj1 = TestObject {
            id: 1,
            data: vec![1; 256],
            children: vec![],
        };
        let obj2 = TestObject {
            id: 2,
            data: vec![2; 256],
            children: vec![],
        };

        let gc_obj1 = gc.allocate(obj1).unwrap();
        let gc_obj2 = gc.allocate(obj2).unwrap();

        // Create circular references
        gc_obj1.get_mut().unwrap().children.push(gc_obj2.clone());
        gc_obj2.get_mut().unwrap().children.push(gc_obj1.clone());

        // Remove external references to create unreachable cycle
        drop(gc_obj1);
        drop(gc_obj2);

        let stats_before = gc.get_stats().unwrap();
        
        // Collection should detect and collect the cycle
        let collection_result = gc.collect_garbage();
        assert!(collection_result.is_ok());

        let stats_after = gc.get_stats().unwrap();
        assert!(stats_after.objects_collected > stats_before.objects_collected);

        info!("Circular reference handling test passed");
    }

    /// Test concurrent collection safety
    #[test]
    fn test_concurrent_collection_safety() {
        init_tracing!();
        info!("Testing concurrent collection safety");

        let config = GcConfig {
            concurrent: true,
            ..GcConfig::default()
        };
        let gc = Arc::new(GarbageCollector::new(config).unwrap());

        let handles: Vec<_> = (0..4).map(|thread_id| {
            let gc_clone = gc.clone();
            std::thread::spawn(move || {
                // Each thread allocates objects
                for i in 0..100 {
                    let obj = TestObject {
                        id: thread_id * 1000 + i,
                        data: vec![(thread_id as u8); 128],
                        children: vec![],
                    };
                    let _allocated = gc_clone.allocate(obj).unwrap();
                }
                
                // Trigger collection from each thread
                let _result = gc_clone.collect_garbage();
            })
        }).collect();

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        let final_stats = gc.get_stats().unwrap();
        assert!(final_stats.objects_allocated >= 400);

        info!("Concurrent collection safety test passed");
    }

    /// Stress test with high allocation pressure
    #[test]
    #[ignore] // Run with --ignored flag for stress tests
    fn test_high_allocation_pressure() {
        init_tracing!();
        info!("Testing high allocation pressure stress scenario");

        let config = GcConfig {
            allocation_pressure_ratio: 0.5, // Trigger more frequent collections
            emergency_threshold: 0.8,
            ..GcConfig::default()
        };
        let gc = GarbageCollector::new(config).unwrap();

        // Stress test with rapid allocation
        for round in 0..10 {
            debug!("Stress test round {}", round);
            
            let mut round_objects = Vec::new();
            for i in 0..500 {
                let obj = TestObject {
                    id: round * 1000 + i,
                    data: vec![(round as u8); 1024],
                    children: vec![],
                };
                round_objects.push(gc.allocate(obj).unwrap());
            }

            // Keep some objects alive, let others be collected
            if round % 2 == 0 {
                drop(round_objects);
            }

            // Force collection
            let _result = gc.collect_garbage();
        }

        let final_stats = gc.get_stats().unwrap();
        assert!(final_stats.collections_performed > 0);
        assert!(final_stats.objects_collected > 0);

        info!("High allocation pressure stress test passed");
    }
}
