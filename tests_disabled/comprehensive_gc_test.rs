/// Comprehensive garbage collection test suite
/// Tests all GC algorithms, generational collection, and advanced features

#[path = "common.rs"]
pub mod common;

use cursed::memory::{
    GarbageCollector, GcConfig, CollectionAlgorithm, CollectionTrigger, CollectionGeneration,
    GenerationalCollector, GenerationalConfig, MarkSweepCollector, CopyingCollector,
    IncrementalCollector, CycleDetector, Gc, WeakGc, Tag, Traceable, Visitor
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{info, debug, error, warn};

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[derive(Debug, Clone)]
struct ComprehensiveTestObject {
    pub id: u32,
    pub generation: u8,
    pub data: Vec<u8>,
    pub strong_refs: Vec<Gc<ComprehensiveTestObject>>,
    pub weak_refs: Vec<WeakGc<ComprehensiveTestObject>>,
    pub metadata: HashMap<String, String>,
}

impl Traceable for ComprehensiveTestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for strong_ref in &self.strong_refs {
            strong_ref.trace(visitor);
        }
        // Weak references are not traced to avoid preventing collection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test mark-and-sweep collection algorithm
    #[test]
    fn test_mark_sweep_collection() {
        init_tracing!();
        info!("Testing mark-and-sweep collection");

        let config = GcConfig {
            algorithm: CollectionAlgorithm::MarkSweep,
            generational: false,
            incremental: false,
            concurrent: false,
            ..GcConfig::default()
        };

        let gc = GarbageCollector::new(config).unwrap();

        // Create object graph for mark-and-sweep testing
        let root = ComprehensiveTestObject {
            id: 0,
            generation: 0,
            data: vec![0; 1024],
            strong_refs: vec![],
            weak_refs: vec![],
            metadata: HashMap::new(),
        };

        let mut root_gc = gc.allocate(root).unwrap();

        // Create reachable objects
        for i in 1..=10 {
            let reachable = ComprehensiveTestObject {
                id: i,
                generation: 0,
                data: vec![i as u8; 512],
                strong_refs: vec![],
                weak_refs: vec![],
                metadata: HashMap::new(),
            };
            let reachable_gc = gc.allocate(reachable).unwrap();
            root_gc.get_mut().unwrap().strong_refs.push(reachable_gc);
        }

        // Create unreachable objects
        for i in 100..110 {
            let unreachable = ComprehensiveTestObject {
                id: i,
                generation: 0,
                data: vec![i as u8; 512],
                strong_refs: vec![],
                weak_refs: vec![],
                metadata: HashMap::new(),
            };
            let _unreachable_gc = gc.allocate(unreachable).unwrap();
        }

        gc.add_root(root_gc.clone()).unwrap();

        let stats_before = gc.get_stats().unwrap();
        debug!("Objects before mark-sweep: {}", stats_before.objects_allocated);

        // Perform mark-and-sweep collection
        let collection_result = gc.collect_with_algorithm(CollectionAlgorithm::MarkSweep);
        assert!(collection_result.is_ok());

        let stats_after = gc.get_stats().unwrap();
        debug!("Objects after mark-sweep: {}", stats_after.objects_allocated - stats_after.objects_collected);

        // Reachable objects should survive, unreachable should be collected
        assert!(stats_after.objects_collected >= 10); // Unreachable objects
        assert_eq!(root_gc.id, 0); // Root should survive
        assert_eq!(root_gc.strong_refs.len(), 10); // Referenced objects should survive

        info!("Mark-and-sweep collection test passed");
    }

    /// Test copying collector algorithm
    #[test]
    fn test_copying_collection() {
        init_tracing!();
        info!("Testing copying collection");

        let config = GcConfig {
            algorithm: CollectionAlgorithm::Copying,
            generational: false,
            ..GcConfig::default()
        };

        let gc = GarbageCollector::new(config).unwrap();

        // Create objects suitable for copying collection (short-lived)
        let mut young_objects = Vec::new();
        for i in 0..50 {
            let young_obj = ComprehensiveTestObject {
                id: i,
                generation: 0, // Young generation
                data: vec![i as u8; 256],
                strong_refs: vec![],
                weak_refs: vec![],
                metadata: HashMap::new(),
            };
            young_objects.push(gc.allocate(young_obj).unwrap());
        }

        // Keep half as roots, let half be collectible
        for i in 0..25 {
            gc.add_root(young_objects[i].clone()).unwrap();
        }

        let stats_before = gc.get_stats().unwrap();
        debug!("Objects before copying: {}", stats_before.objects_allocated);

        // Perform copying collection
        let collection_result = gc.collect_with_algorithm(CollectionAlgorithm::Copying);
        assert!(collection_result.is_ok());

        let stats_after = gc.get_stats().unwrap();
        debug!("Objects after copying: {}", stats_after.objects_allocated - stats_after.objects_collected);

        // Copying collection should have moved/collected objects
        assert!(stats_after.objects_collected > 0);
        assert!(stats_after.bytes_copied > 0);

        // Rooted objects should still be accessible
        for i in 0..25 {
            assert_eq!(young_objects[i].id, i);
        }

        info!("Copying collection test passed");
    }

    /// Test incremental collection algorithm
    #[test]
    fn test_incremental_collection() {
        init_tracing!();
        info!("Testing incremental collection");

        let config = GcConfig {
            algorithm: CollectionAlgorithm::Incremental,
            max_pause_time: Duration::from_millis(10), // Very short pauses
            ..GcConfig::default()
        };

        let gc = GarbageCollector::new(config).unwrap();

        // Create workload suitable for incremental collection
        let mut incremental_objects = Vec::new();
        for i in 0..100 {
            let obj = ComprehensiveTestObject {
                id: i,
                generation: if i < 50 { 0 } else { 1 },
                data: vec![i as u8; 512],
                strong_refs: vec![],
                weak_refs: vec![],
                metadata: HashMap::new(),
            };
            incremental_objects.push(gc.allocate(obj).unwrap());
        }

        // Create some inter-object references
        for i in 0..20 {
            let target_idx = (i + 10) % incremental_objects.len();
            incremental_objects[i].get_mut().unwrap().strong_refs.push(
                incremental_objects[target_idx].clone()
            );
        }

        // Add some roots
        for i in (0..incremental_objects.len()).step_by(5) {
            gc.add_root(incremental_objects[i].clone()).unwrap();
        }

        let collection_start = Instant::now();

        // Perform incremental collection
        let collection_result = gc.collect_with_algorithm(CollectionAlgorithm::Incremental);
        assert!(collection_result.is_ok());

        let collection_time = collection_start.elapsed();
        let stats = gc.get_stats().unwrap();

        debug!("Incremental collection time: {:?}", collection_time);
        debug!("Incremental work phases: {}", stats.incremental_phases);

        // Incremental collection should have multiple phases
        assert!(stats.incremental_phases > 1);
        // Should have reasonable pause time
        assert!(stats.average_pause_time < Duration::from_millis(50));

        info!("Incremental collection test passed");
    }

    /// Test cycle detection algorithm
    #[test]
    fn test_cycle_detection() {
        init_tracing!();
        info!("Testing cycle detection");

        let config = GcConfig {
            algorithm: CollectionAlgorithm::CycleDetection,
            ..GcConfig::default()
        };

        let gc = GarbageCollector::new(config).unwrap();

        // Create cyclic reference structure
        let obj1 = ComprehensiveTestObject {
            id: 1,
            generation: 0,
            data: vec![1; 256],
            strong_refs: vec![],
            weak_refs: vec![],
            metadata: HashMap::new(),
        };
        let obj2 = ComprehensiveTestObject {
            id: 2,
            generation: 0,
            data: vec![2; 256],
            strong_refs: vec![],
            weak_refs: vec![],
            metadata: HashMap::new(),
        };
        let obj3 = ComprehensiveTestObject {
            id: 3,
            generation: 0,
            data: vec![3; 256],
            strong_refs: vec![],
            weak_refs: vec![],
            metadata: HashMap::new(),
        };

        let gc_obj1 = gc.allocate(obj1).unwrap();
        let gc_obj2 = gc.allocate(obj2).unwrap();
        let gc_obj3 = gc.allocate(obj3).unwrap();

        // Create cycle: obj1 -> obj2 -> obj3 -> obj1
        gc_obj1.get_mut().unwrap().strong_refs.push(gc_obj2.clone());
        gc_obj2.get_mut().unwrap().strong_refs.push(gc_obj3.clone());
        gc_obj3.get_mut().unwrap().strong_refs.push(gc_obj1.clone());

        let stats_before = gc.get_stats().unwrap();

        // Drop external references to create unreachable cycle
        drop(gc_obj1);
        drop(gc_obj2);
        drop(gc_obj3);

        // Perform cycle detection collection
        let collection_result = gc.collect_with_algorithm(CollectionAlgorithm::CycleDetection);
        assert!(collection_result.is_ok());

        let stats_after = gc.get_stats().unwrap();
        debug!("Cycles detected: {}", stats_after.cycles_detected);
        debug!("Cycles collected: {}", stats_after.cycles_collected);

        // Should have detected and collected the cycle
        assert!(stats_after.cycles_detected > 0);
        assert!(stats_after.cycles_collected > 0);
        assert!(stats_after.objects_collected >= 3);

        info!("Cycle detection test passed");
    }

    /// Test generational garbage collection
    #[test]
    fn test_generational_collection() {
        init_tracing!();
        info!("Testing generational collection");

        let config = GcConfig {
            generational: true,
            young_gen_threshold: 0.6,
            old_gen_threshold: 0.8,
            algorithm: CollectionAlgorithm::Adaptive,
            ..GcConfig::default()
        };

        let gc = GarbageCollector::new(config).unwrap();

        // Create young generation objects (short-lived)
        let mut young_objects = Vec::new();
        for i in 0..100 {
            let young_obj = ComprehensiveTestObject {
                id: i,
                generation: 0,
                data: vec![i as u8; 128],
                strong_refs: vec![],
                weak_refs: vec![],
                metadata: HashMap::new(),
            };
            young_objects.push(gc.allocate(young_obj).unwrap());
        }

        // Create old generation objects (long-lived)
        let mut old_objects = Vec::new();
        for i in 1000..1020 {
            let old_obj = ComprehensiveTestObject {
                id: i,
                generation: 1,
                data: vec![(i % 256) as u8; 512],
                strong_refs: vec![],
                weak_refs: vec![],
                metadata: HashMap::new(),
            };
            old_objects.push(gc.allocate(old_obj).unwrap());
        }

        // Add old objects as roots to make them long-lived
        for old_obj in &old_objects {
            gc.add_root(old_obj.clone()).unwrap();
        }

        // Perform young generation collection
        let young_collection_result = gc.collect_generation(CollectionGeneration::Young);
        assert!(young_collection_result.is_ok());

        let stats_after_young = gc.get_stats().unwrap();
        debug!("Young generation collections: {}", stats_after_young.young_collections);

        // Most young objects should be collected
        assert!(stats_after_young.young_collections > 0);

        // Old objects should survive
        for old_obj in &old_objects {
            assert_eq!(old_obj.generation, 1);
        }

        // Perform full collection
        let full_collection_result = gc.collect_generation(CollectionGeneration::Full);
        assert!(full_collection_result.is_ok());

        let final_stats = gc.get_stats().unwrap();
        assert!(final_stats.full_collections > 0);

        info!("Generational collection test passed");
    }

    /// Test weak references and finalization
    #[test]
    fn test_weak_references() {
        init_tracing!();
        info!("Testing weak references");

        let config = GcConfig::default();
        let gc = GarbageCollector::new(config).unwrap();

        // Create object with strong reference
        let strong_obj = ComprehensiveTestObject {
            id: 1,
            generation: 0,
            data: vec![1; 256],
            strong_refs: vec![],
            weak_refs: vec![],
            metadata: HashMap::new(),
        };

        let strong_gc = gc.allocate(strong_obj).unwrap();
        let weak_gc = WeakGc::downgrade(&strong_gc);

        // Weak reference should be valid while strong reference exists
        assert!(weak_gc.upgrade().is_some());

        // Drop strong reference
        drop(strong_gc);

        // Trigger collection
        let _collection_result = gc.collect_garbage();

        // Weak reference should now be invalid
        assert!(weak_gc.upgrade().is_none());

        info!("Weak references test passed");
    }

    /// Test concurrent collection with multiple threads
    #[test]
    fn test_concurrent_collection() {
        init_tracing!();
        info!("Testing concurrent collection");

        let config = GcConfig {
            concurrent: true,
            ..GcConfig::default()
        };

        let gc = Arc::new(GarbageCollector::new(config).unwrap());

        // Spawn multiple threads for concurrent allocation and collection
        let handles: Vec<_> = (0..4).map(|thread_id| {
            let gc_clone = gc.clone();
            std::thread::spawn(move || {
                let mut thread_objects = Vec::new();
                
                // Each thread allocates objects
                for i in 0..50 {
                    let obj = ComprehensiveTestObject {
                        id: thread_id * 1000 + i,
                        generation: 0,
                        data: vec![thread_id as u8; 256],
                        strong_refs: vec![],
                        weak_refs: vec![],
                        metadata: HashMap::new(),
                    };
                    thread_objects.push(gc_clone.allocate(obj).unwrap());
                }

                // Create some inter-thread references
                if thread_id > 0 && !thread_objects.is_empty() {
                    // Reference objects from other threads
                    for i in 0..10 {
                        let ref_obj = ComprehensiveTestObject {
                            id: thread_id * 1000 + i + 500,
                            generation: 0,
                            data: vec![thread_id as u8; 128],
                            strong_refs: vec![thread_objects[i % thread_objects.len()].clone()],
                            weak_refs: vec![],
                            metadata: HashMap::new(),
                        };
                        let _ref_gc = gc_clone.allocate(ref_obj).unwrap();
                    }
                }

                // Trigger collection from each thread
                let _collection_result = gc_clone.collect_garbage();

                thread_objects.len()
            })
        }).collect();

        // Wait for all threads to complete
        let mut total_objects = 0;
        for handle in handles {
            total_objects += handle.join().unwrap();
        }

        let final_stats = gc.get_stats().unwrap();
        assert!(final_stats.objects_allocated >= total_objects);
        assert!(final_stats.concurrent_collections > 0);

        info!("Concurrent collection test passed");
    }

    /// Test memory pressure and emergency collection
    #[test]
    fn test_memory_pressure_handling() {
        init_tracing!();
        info!("Testing memory pressure handling");

        let config = GcConfig {
            emergency_threshold: 0.7, // Lower threshold for testing
            allocation_pressure_ratio: 0.6,
            ..GcConfig::default()
        };

        let gc = GarbageCollector::new(config).unwrap();

        // Create memory pressure by allocating many large objects
        let mut pressure_objects = Vec::new();
        for i in 0..200 {
            let large_obj = ComprehensiveTestObject {
                id: i,
                generation: 0,
                data: vec![i as u8; 2048], // 2KB objects
                strong_refs: vec![],
                weak_refs: vec![],
                metadata: HashMap::new(),
            };
            pressure_objects.push(gc.allocate(large_obj).unwrap());

            // Periodically check if emergency collection triggered
            if i % 50 == 0 {
                let stats = gc.get_stats().unwrap();
                if stats.emergency_collections > 0 {
                    debug!("Emergency collection triggered at object {}", i);
                    break;
                }
            }
        }

        let final_stats = gc.get_stats().unwrap();
        
        // Should have handled memory pressure
        assert!(final_stats.emergency_collections > 0 || 
                final_stats.pressure_triggered_collections > 0);
        assert!(final_stats.collections_performed > 0);

        info!("Memory pressure handling test passed");
    }

    /// Test collection performance and statistics
    #[test]
    fn test_collection_performance() {
        init_tracing!();
        info!("Testing collection performance");

        let config = GcConfig::default();
        let gc = GarbageCollector::new(config).unwrap();

        let perf_start = Instant::now();

        // Create workload for performance testing
        let mut perf_objects = Vec::new();
        for i in 0..500 {
            let obj = ComprehensiveTestObject {
                id: i,
                generation: if i < 250 { 0 } else { 1 },
                data: vec![(i % 256) as u8; 512],
                strong_refs: vec![],
                weak_refs: vec![],
                metadata: HashMap::new(),
            };
            perf_objects.push(gc.allocate(obj).unwrap());
        }

        let allocation_time = perf_start.elapsed();
        debug!("Allocation time for 500 objects: {:?}", allocation_time);

        // Create some references
        for i in 0..100 {
            let target = (i + 50) % perf_objects.len();
            perf_objects[i].get_mut().unwrap().strong_refs.push(
                perf_objects[target].clone()
            );
        }

        // Add roots
        for i in (0..perf_objects.len()).step_by(10) {
            gc.add_root(perf_objects[i].clone()).unwrap();
        }

        // Measure collection performance
        let collection_start = Instant::now();
        let collection_result = gc.collect_garbage();
        let collection_time = collection_start.elapsed();
        
        assert!(collection_result.is_ok());
        debug!("Collection time: {:?}", collection_time);

        let stats = gc.get_stats().unwrap();
        debug!("Performance stats: {:?}", stats);

        // Performance assertions
        assert!(allocation_time < Duration::from_secs(1));
        assert!(collection_time < Duration::from_secs(1));
        assert!(stats.average_pause_time < Duration::from_millis(200));
        assert!(stats.throughput > 100.0); // Objects per second

        info!("Collection performance test passed");
    }

    /// Comprehensive stress test with all features
    #[test]
    #[ignore] // Run with --ignored flag for stress tests
    fn test_comprehensive_stress() {
        init_tracing!();
        info!("Starting comprehensive GC stress test");

        let config = GcConfig {
            algorithm: CollectionAlgorithm::Adaptive,
            generational: true,
            incremental: true,
            concurrent: true,
            goroutine_aware: false,
            young_gen_threshold: 0.6,
            old_gen_threshold: 0.8,
            emergency_threshold: 0.9,
            max_pause_time: Duration::from_millis(50),
            allocation_pressure_ratio: 0.7,
            adaptive_algorithm_selection: true,
        };

        let gc = Arc::new(GarbageCollector::new(config).unwrap());

        // Multi-threaded stress test
        let stress_handles: Vec<_> = (0..6).map(|thread_id| {
            let gc_clone = gc.clone();
            std::thread::spawn(move || {
                let mut thread_objects = Vec::new();
                let mut cycle_objects = Vec::new();
                
                for round in 0..100 {
                    let pattern = round % 5;
                    
                    match pattern {
                        0 => {
                            // Many small objects
                            for i in 0..100 {
                                let small_obj = ComprehensiveTestObject {
                                    id: thread_id * 10000 + round * 100 + i,
                                    generation: 0,
                                    data: vec![thread_id as u8; 64],
                                    strong_refs: vec![],
                                    weak_refs: vec![],
                                    metadata: HashMap::new(),
                                };
                                thread_objects.push(gc_clone.allocate(small_obj).unwrap());
                            }
                        }
                        1 => {
                            // Few large objects
                            for i in 0..20 {
                                let large_obj = ComprehensiveTestObject {
                                    id: thread_id * 10000 + round * 100 + i,
                                    generation: 1,
                                    data: vec![thread_id as u8; 4096],
                                    strong_refs: vec![],
                                    weak_refs: vec![],
                                    metadata: HashMap::new(),
                                };
                                thread_objects.push(gc_clone.allocate(large_obj).unwrap());
                            }
                        }
                        2 => {
                            // Create cycles
                            if cycle_objects.len() >= 3 {
                                for i in 0..cycle_objects.len() {
                                    let next = (i + 1) % cycle_objects.len();
                                    cycle_objects[i].get_mut().unwrap().strong_refs.push(
                                        cycle_objects[next].clone()
                                    );
                                }
                                cycle_objects.clear();
                            }
                            
                            for i in 0..10 {
                                let cycle_obj = ComprehensiveTestObject {
                                    id: thread_id * 10000 + round * 100 + i,
                                    generation: 0,
                                    data: vec![thread_id as u8; 256],
                                    strong_refs: vec![],
                                    weak_refs: vec![],
                                    metadata: HashMap::new(),
                                };
                                cycle_objects.push(gc_clone.allocate(cycle_obj).unwrap());
                            }
                        }
                        3 => {
                            // Mixed sizes with references
                            for i in 0..50 {
                                let size = match i % 4 {
                                    0 => 128,
                                    1 => 256,
                                    2 => 512,
                                    _ => 1024,
                                };
                                let mixed_obj = ComprehensiveTestObject {
                                    id: thread_id * 10000 + round * 100 + i,
                                    generation: if size > 512 { 1 } else { 0 },
                                    data: vec![thread_id as u8; size],
                                    strong_refs: vec![],
                                    weak_refs: vec![],
                                    metadata: HashMap::new(),
                                };
                                let mixed_gc = gc_clone.allocate(mixed_obj).unwrap();
                                
                                // Add references to existing objects
                                if !thread_objects.is_empty() && i % 5 == 0 {
                                    let target_idx = i % thread_objects.len();
                                    mixed_gc.get_mut().unwrap().strong_refs.push(
                                        thread_objects[target_idx].clone()
                                    );
                                }
                                
                                thread_objects.push(mixed_gc);
                            }
                        }
                        _ => {
                            // Cleanup phase - drop most objects
                            thread_objects.truncate(thread_objects.len() / 4);
                            
                            // Force collection
                            let _result = gc_clone.collect_garbage();
                        }
                    }

                    // Periodic collection
                    if round % 10 == 0 {
                        let _result = gc_clone.collect_garbage();
                    }

                    // Brief pause to prevent overwhelming
                    if round % 25 == 0 {
                        std::thread::sleep(Duration::from_millis(2));
                    }
                }

                thread_objects.len()
            })
        }).collect();

        // Wait for all stress threads
        let mut total_surviving = 0;
        for handle in stress_handles {
            total_surviving += handle.join().unwrap();
        }

        // Final collection and verification
        let final_collection_result = gc.collect_garbage();
        assert!(final_collection_result.is_ok());

        let final_stats = gc.get_stats().unwrap();
        
        info!("Comprehensive stress test completed");
        debug!("Final stats: {:?}", final_stats);
        debug!("Total surviving objects: {}", total_surviving);

        // Stress test assertions
        assert!(final_stats.collections_performed >= 50);
        assert!(final_stats.objects_allocated >= 1000);
        assert!(final_stats.total_allocated_bytes >= 1_000_000); // At least 1MB
        assert!(final_stats.objects_collected > 0);
        assert!(final_stats.total_freed_bytes > 0);

        // Should have used multiple algorithms
        assert!(final_stats.algorithm_switches > 0 || 
                final_stats.concurrent_collections > 0);

        info!("Comprehensive GC stress test passed");
    }
}
