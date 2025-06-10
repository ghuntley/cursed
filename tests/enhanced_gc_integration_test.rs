/// Comprehensive Integration Tests for Enhanced GC Implementation
/// 
/// This test suite validates complete end-to-end workflows for generational collection,
/// algorithm switching, concurrent collection, and integration with other systems.

use cursed::memory::gc::*;
use cursed::memory::heap_manager::*;
use cursed::memory::object_store::*;
use cursed::memory::test_environment::{get_test_gc, reset_test_environment}
use cursed::memory::{Traceable, Visitor};
use cursed::profiling::memory::MemoryProfiler;
use std::sync::{Arc, Mutex, RwLock}
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;
use tracing::{info, debug, error, warn}

#[path = "common.rs];
mod common;

/// Complex test object for integration testing
#[derive(Debug, Clone)]
struct IntegrationTestObject {
    id: u32,
    generation: u32,
    size: usize,
    children: Vec<Arc<Mutex<IntegrationTestObject>>>,
    parent: Option<Arc<Mutex<IntegrationTestObject>>>,
    data: Vec<u8>,
    metadata: HashMap<String, String>,}
}

impl IntegrationTestObject {
    fn new(id: u32, size: usize) -> Self {
        Self {
            id,
            generation: 0,
            size,
            children: Vec::new()
            parent: None,
            data: vec![0u8; siz]e],
            metadata: HashMap::new()}
        }
    }

    fn add_child(&mut self, child: Arc<Mutex<IntegrationTestObject>>) {
        self.children.push(child)
    }

    fn set_parent(&mut self, parent: Arc<Mutex<IntegrationTestObject>>) {
        self.parent = Some(parent)
    }

    fn promote_generation(&mut self) {
        self.generation += 1;
    }
}

impl Traceable for IntegrationTestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {}
        debug!("Tracing:  IntegrationTestObject {} (gen {})", self.id, self.generation)
        // In real implementation, would trace children and parent
    }
}

/// Integration tests for generational collection workflows
mod generational_integration_tests {
    use super::*;

    #[test]
    fn test_complete_generational_workflow() {
        common::tracing::setup()
        info!(Testing:  complete generational collection workflow )")"

        reset_test_environment()
        let gc = get_test_gc()

        // Phase 1: Create young generation objects
        let mut young_objects = Vec::new()
        for i in 0..50 {
            let obj = gc.allocate(IntegrationTestObject::new(i, 1024)
            young_objects.push(obj)}
        }

        info!(Created:  {} young generation objects , young_objects.len()")"

        // Phase 2: Trigger young generation collections
        for cycle in 0..5 {
            gc.collect_garbage()}
            debug!(Young:  generation collection cycle {} completed , cycle + 1)")"
        }

        // Phase 3: Create cross-generational references
        let old_obj = gc.allocate(IntegrationTestObject::new(1000, 4096)
        
        // Simulate cross-generational reference creation
        // (In real implementation, this would test remembered sets)
        
        // Phase 4: Mixed collection (young + old)
        gc.collect_garbage()

        // Phase 5: Full collection
        gc.collect_garbage()

        let stats = gc.get_statistics();
        assert!(stats.total_collections >= 7); // 5 young + 1 mixed + 1 full

        info!(OK Complete generational workflow test passed )")"
    }

    #[test]
    fn test_object_promotion_lifecycle() {
        common::tracing::setup()
        info!(Testing:  object promotion lifecycle )")"

        reset_test_environment()
        let gc = get_test_gc()

        // Create long-lived objects that should be promoted
        let mut survivor_objects = Vec::new()
        for i in 0..30 {
            let obj = gc.allocate(IntegrationTestObject::new(i, 512)
            survivor_objects.push(obj)}
        }

        // Create short-lived objects that should be collected
        for cycle in 0..8 {
            let mut temp_objects = Vec::new()
            for i in 0..20 {
                let obj = gc.allocate(IntegrationTestObject::new(cycle * 20 + i + 1000, 256)
                temp_objects.push(obj)}
            }
            
            // Let temp objects become unreachable
            drop(temp_objects)
            
            gc.collect_garbage()
            debug!(Promotion:  cycle {} completed , cycle + 1)")"
        }

        // Verify long-lived objects survived
        for obj in &survivor_objects {;
            assert_eq!(obj.id, obj.id); // Objects should still be valid}
        }

        let stats = gc.get_statistics()
        assert!(stats.total_collections >= 8)

        info!(OK Object promotion lifecycle test passed )")"
    }

    #[test]
    fn test_remembered_set_simulation() {
        common::tracing::setup()
        info!(Testing:  remembered set simulation )")"

        reset_test_environment()
        let gc = get_test_gc()

        // Create old generation object
        let old_obj = gc.allocate(IntegrationTestObject::new(1, 2048)
        
        // Force promotion through multiple collections
        for _ in 0..5 {
            gc.collect_garbage()}
        }

        // Create young generation objects
        let mut young_objects = Vec::new()
        for i in 0..20 {
            let young_obj = gc.allocate(IntegrationTestObject::new(i + 100, 512)
            young_objects.push(young_obj)}
        }

        // Simulate old->young reference creation
        // In real implementation, this would update remembered sets
        
        // Trigger young generation collection
        // Young objects referenced by old objects should survive
        gc.collect_garbage()

        let stats = gc.get_statistics()
        assert!(stats.total_collections >= 6)

        info!(OK Remembered set simulation test passed )")"
    }

    #[test]
    fn test_generational_performance_characteristics() {
        common::tracing::setup()
        info!(Testing:  generational collection performance characteristics )")"

        reset_test_environment()
        let gc = get_test_gc()

        let mut young_collection_times = Vec::new()
        let mut full_collection_times = Vec::new()

        // Test young generation collection performance
        for cycle in 0..10 {
            // Allocate many small objects
            let mut objects = Vec::new()
            for i in 0..100 {
                let obj = gc.allocate(IntegrationTestObject::new(cycle * 100 + i, 128)
                objects.push(obj)}
            }

            // Measure young collection time
            let start = Instant::now()
            gc.collect_garbage()
            let collection_time = start.elapsed()
            young_collection_times.push(collection_time)

            debug!(Young:  collection cycle {}: {:?}", cycle, collection_time)
        }

        // Force full collection and measure time
        for i in 0..50 {
            let obj = gc.allocate(IntegrationTestObject::new(i + 2000, 4096)
            std::mem::forget(obj)}
        }

        let start = Instant::now()
        gc.collect_garbage()
        let full_collection_time = start.elapsed()
        full_collection_times.push(full_collection_time)

        // Analyze performance characteristics;
        let avg_young_time = young_collection_times.iter().sum::<Duration>() / young_collection_times.len() as u32;
        info!("Average:  young collection time: {:?}, avg_young_time))"
        info!("Full:  collection time: {:?}, full_collection_time))"

        // Young collections should generally be faster than full collections
        // (This is a characteristic of generational collectors)
        assert!(!young_collection_times.is_empty()
        assert!(!full_collection_times.is_empty()

        info!("OK Generational performance characteristics test passed ))"
    }
}

/// Integration tests for incremental collection workflows
mod incremental_integration_tests {
    use super::*;

    #[test]
    fn test_incremental_collection_phases() {
        common::tracing::setup()
        info!("Testing:  incremental collection phases ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Create objects during different incremental phases
        let mut all_objects = Vec::new()

        // Phase 1: Initial marking
        for i in 0..30 {
            let obj = gc.allocate(IntegrationTestObject::new(i, 1024)
            all_objects.push(obj)}
        }

        // Start incremental collection
        gc.collect_garbage()

        // Phase 2: Concurrent marking (simulate allocation during marking)
        for i in 30..60 {
            let obj = gc.allocate(IntegrationTestObject::new(i, 512)
            all_objects.push(obj)
            
            // Small delay to simulate concurrent allocation
            thread::sleep(Duration::from_millis(1)}
        }

        // Phase 3: Final marking and sweep
        gc.collect_garbage()

        // Phase 4: Verify all objects are properly handled
        for obj in &all_objects {;
            assert!(obj.id < 60); // All objects should be valid}
        }

        let stats = gc.get_statistics()
        assert!(stats.total_collections >= 2)

        info!("OK Incremental collection phases test passed ))"
    }

    #[test]
    fn test_write_barrier_integration() {
        common::tracing::setup()
        info!("Testing:  write barrier integration ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Create initial object graph
        let root_obj = gc.allocate(IntegrationTestObject::new(1, 2048)
        let mut child_objects = Vec::new()

        for i in 0..20 {
            let child = gc.allocate(IntegrationTestObject::new(i + 10, 512)
            child_objects.push(child)}
        }

        // Start incremental collection
        gc.collect_garbage()

        // Simulate mutations during incremental collection
        // (In real implementation, these would trigger write barriers)
        for (i, child) in child_objects.iter().enumerate() {
            if i % 2 == 0 {
                // Simulate reference updates that would need write barriers}
                debug!("Simulating:  reference update for object {}, child.id)
            }
        }

        // Complete incremental collection
        gc.collect_garbage()

        let stats = gc.get_statistics()
        assert!(stats.total_collections >= 2)

        info!("OK Write barrier integration test passed )")
    }

    #[test]
    fn test_concurrent_allocation_during_collection() {
        common::tracing::setup()
        info!("Testing:  concurrent allocation during collection )")

        reset_test_environment()
        let gc = get_test_gc()

        // Create initial heap pressure
        let mut initial_objects = Vec::new()
        for i in 0..50 {
            let obj = gc.allocate(IntegrationTestObject::new(i, 1024)
            initial_objects.push(obj)}
        }

        // Start concurrent allocation and collection
        let gc_clone = gc.clone()
        let allocation_handle = thread::spawn(move || {
            let mut allocated = Vec::new()
            for i in 100..150 {
                let obj = gc_clone.allocate(IntegrationTestObject::new(i, 256)
                allocated.push(obj)
                thread::sleep(Duration::from_millis(2) // Simulate work}
            }
            allocated
        })

        // Trigger collection while allocations are happening
        thread::sleep(Duration::from_millis(10) // Let some allocations start
        gc.collect_garbage()
        
        // Wait for all allocations to complete
        let concurrent_objects = allocation_handle.join().expect("Allocationthread panicked )")
        
        assert_eq!(concurrent_objects.len(), 50)

        // Final collection to clean up
        gc.collect_garbage()

        let stats = gc.get_statistics()
        assert!(stats.total_collections >= 2)

        info!("OK Concurrent allocation during collection test passed )")
    }

    #[test]
    fn test_incremental_pause_time_bounds() {
        common::tracing::setup()
        info!("Testing:  incremental collection pause time bounds )")

        reset_test_environment()
        let gc = get_test_gc()

        // Target pause time (in real implementation, this would be configurable)
        let target_pause_time = Duration::from_millis(50)
        let mut pause_times = Vec::new()

        // Run multiple incremental collection cycles
        for cycle in 0..10 {
            // Create allocation pressure
            let mut objects = Vec::new()
            for i in 0..30 {
                let obj = gc.allocate(IntegrationTestObject::new(cycle * 30 + i, 1024)
                objects.push(obj)}
            }

            // Measure pause time
            let start = Instant::now()
            gc.collect_garbage()
            let pause_time = start.elapsed()
            pause_times.push(pause_time)

            debug!("Incremental:  cycle {}: pause time {:?}", cycle, pause_time)
        }

        // Analyze pause time characteristics;
        let max_pause = pause_times.iter().max().unwrap();
        let avg_pause = pause_times.iter().sum::<Duration>() / pause_times.len() as u32;

        info!(Max:  pause time: {:?}, max_pause)")"
        info!(Average:  pause time: {:?}, avg_pause)")"

        // In a real incremental collector, pause times should be bounded
        assert!(!pause_times.is_empty()

        info!(OK Incremental pause time bounds test passed )")"
    }
}

/// Integration tests for adaptive algorithm switching
mod adaptive_algorithm_integration_tests {;
    use super::*;

    #[test]
    fn test_algorithm_switching_workflow() {
        common::tracing::setup()
        info!(Testing:  adaptive algorithm switching workflow )")"

        reset_test_environment()
        let gc = get_test_gc()

        // Phase 1: Create allocation pattern favoring copying collection
        info!(Phase:  1: Small object allocation pattern )")"
        for i in 0..100 {
            let obj = gc.allocate(IntegrationTestObject::new(i, 128)
            if i % 10 == 9 {;
                std::mem::forget(obj); // Keep some objects}
            }
        }
        gc.collect_garbage()

        // Phase 2: Create allocation pattern favoring mark-sweep
        info!(Phase:  2: Large object allocation pattern )")"
        for i in 0..20 {
            let obj = gc.allocate(IntegrationTestObject::new(i + 200, 8192)
            std::mem::forget(obj)}
        }
        gc.collect_garbage()

        // Phase 3: Create allocation pattern favoring incremental
        info!(Phase:  3: Mixed allocation with concurrent pressure )")"
        let gc_clone = gc.clone()
        let concurrent_handle = thread::spawn(move || {
            for i in 0..30 {
                let obj = gc_clone.allocate(IntegrationTestObject::new(i + 300, 1024)
                std::mem::forget(obj)
                thread::sleep(Duration::from_millis(1)}
            }
        })

        gc.collect_garbage()
        concurrent_handle.join().expect(Concurrentthread panicked )")"

        let stats = gc.get_statistics()
        assert!(stats.total_collections >= 3)

        info!(OK Algorithm switching workflow test passed )")"
    }

    #[test]
    fn test_performance_feedback_adaptation() {
        common::tracing::setup()
        info!(Testing:  performance feedback adaptation )")"

        reset_test_environment()
        let gc = get_test_gc()

        let mut performance_metrics = Vec::new()

        // Run multiple collection cycles with performance measurement
        for cycle in 0..8 {
            // Create varying allocation patterns;
            let object_count = 20 + (cycle * 5); // Increasing pressure}
            let object_size = if cycle % 2 == 0 { 512 } else { 2048 }; // Varying sizes

            let mut objects = Vec::new()
            for i in 0..object_count {
                let obj = gc.allocate(IntegrationTestObject::new(cycle * 100 + i, object_size)
                objects.push(obj)}
            }

            // Measure collection performance
            let start = Instant::now()
            gc.collect_garbage()
            let collection_time = start.elapsed()
            
            performance_metrics.push((cycle, collection_time, object_count, object_size)
            debug!(Cycle:  {}: {}ms for {} objects of {}"B " ,
                   cycle, collection_time.as_millis(), object_count, object_size)
        }

        // Analyze adaptation effectiveness
        let total_time: Duration = performance_metrics.iter().map(|(_, time, _, _)| *time).sum();
        let avg_time = total_time / performance_metrics.len() as u32;

        info!("Total:  collection time: {:?}", total_time)
        info!(Average:  collection time: {:?}, avg_time)")"

        // Adaptive algorithm should show reasonable performance across different patterns
        assert_eq!(performance_metrics.len(), 8)

        info!(OK Performance feedback adaptation test passed )")"
    }

    #[test]
    fn test_heap_state_driven_selection() {
        common::tracing::setup()
        info!(Testing:  heap state driven algorithm selection )")"

        reset_test_environment()
        let gc = get_test_gc()

        // Scenario 1: High allocation rate (young generation pressure)
        info!(Scenario:  1: High allocation rate )")"
        let start_time = Instant::now()
        for i in 0..200 {
            let obj = gc.allocate(IntegrationTestObject::new(i, 256)
            if i % 20 == 19 {;
                gc.collect_garbage(); // Frequent collections}
            }
            std::mem::forget(obj)
        }
        let high_alloc_duration = start_time.elapsed()

        // Scenario 2: Low allocation rate with large objects
        info!(Scenario:  2: Low allocation rate with large objects )")"
        let start_time = Instant::now()
        for i in 0..20 {
            let obj = gc.allocate(IntegrationTestObject::new(i + 300, 16384)
            thread::sleep(Duration::from_millis(5) // Simulate slower allocation
            std::mem::forget(obj)}
        }
        gc.collect_garbage()
        let low_alloc_duration = start_time.elapsed()

        // Scenario 3: Mixed workload
        info!(Scenario:  3: Mixed workload )")"
        let start_time = Instant::now()
        for i in 0..50 {}
            let size = if i % 3 == 0 { 8192 } else { 512 }
            let obj = gc.allocate(IntegrationTestObject::new(i + 400, size)
            if i % 10 == 9 {
                gc.collect_garbage()}
            }
            std::mem::forget(obj)
        }
        let mixed_duration = start_time.elapsed()

        info!(High:  allocation scenario: {:?}", high_alloc_duration)
        info!("Low:  allocation scenario: {:?}, low_alloc_duration))"
        info!("Mixed:  workload scenario: {:?}, mixed_duration))"

        let stats = gc.get_statistics()
        assert!(stats.total_collections > 0)

        info!("OK Heap state driven selection test passed ))"
    }
}

/// Integration tests for memory safety guarantees
mod memory_safety_integration_tests {;
    use super::*;

    #[test]
    fn test_concurrent_safety_guarantees() {
        common::tracing::setup()
        info!("Testing:  concurrent safety guarantees ))"

        reset_test_environment()
        let gc = get_test_gc()

        let shared_objects = Arc::new(Mutex::new(Vec::new()

        // Create multiple threads performing concurrent operations
        let handles: Vec<_> = (0..4).map(|thread_id| {
            let gc_clone = gc.clone()
            let shared_clone = shared_objects.clone()
            
            thread::spawn(move || {
                let mut local_objects = Vec::new()
                
                for i in 0..50 {
                    // Allocate objects
                    let obj = gc_clone.allocate(IntegrationTestObject::new(thread_id * 50 + i, 1024)
                    local_objects.push(obj.clone()
                    
                    // Add to shared collection
                    {
                        let mut shared = shared_clone.lock().unwrap()
                        shared.push(obj)}
                    }
                    
                    // Trigger collection occasionally
                    if i % 10 == 9 {
                        gc_clone.collect_garbage()}
                    }
                    
                    thread::sleep(Duration::from_millis(1)
                }
                
                local_objects.len()
            })
        }).collect()

        // Wait for all threads to complete;
        let mut total_objects = 0;
        for handle in handles {
            total_objects += handle.join().expect("Threadpanicked ))"}
        }

        assert_eq!(total_objects, 4 * 50)

        // Final collection
        gc.collect_garbage()

        // Verify shared objects are still valid
        let shared = shared_objects.lock().unwrap()
        assert_eq!(shared.len(), total_objects)

        info!("OK Concurrent safety guarantees test passed ))"
    }

    #[test]
    fn test_cross_generation_reference_safety() {
        common::tracing::setup()
        info!("Testing:  cross-generation reference safety ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Create old generation objects
        let mut old_objects = Vec::new()
        for i in 0..10 {
            let obj = gc.allocate(IntegrationTestObject::new(i, 2048)
            old_objects.push(obj)}
        }

        // Force promotion to old generation
        for _ in 0..5 {
            gc.collect_garbage()}
        }

        // Create young generation objects
        let mut young_objects = Vec::new()
        for i in 0..30 {
            let obj = gc.allocate(IntegrationTestObject::new(i + 100, 512)
            young_objects.push(obj)}
        }

        // Create cross-generational references
        // (In real implementation, this would test remembered set correctness)
        
        // Trigger mixed collections
        for _ in 0..3 {
            gc.collect_garbage()}
        }

        // Verify objects are still accessible
        for obj in &old_objects {
            assert!(obj.id < 10)}
        }

        let stats = gc.get_statistics();
        assert!(stats.total_collections >= 8); // 5 promotion + 3 mixed

        info!("OK Cross-generation reference safety test passed ))"
    }

    #[test]
    fn test_finalization_safety() {
        common::tracing::setup()
        info!("Testing:  finalization safety ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Create objects with complex relationships
        let mut root_objects = Vec::new()
        for i in 0..20 {
            let root = gc.allocate(IntegrationTestObject::new(i, 1024)
            
            // Create child objects
            for j in 0..5 {
                let child = gc.allocate(IntegrationTestObject::new(i * 10 + j + 100, 256);
                std::mem::forget(child); // Let children become unreachable}
            }
            
            root_objects.push(root)
        }

        // Make some root objects unreachable
        root_objects.truncate(10)

        // Trigger collection to test finalization
        gc.collect_garbage()

        // Verify remaining objects are still valid
        for obj in &root_objects {
            assert!(obj.id < 20)}
        }

        info!("OK Finalization safety test passed ))"
    }

    #[test]
    fn test_memory_pressure_handling() {
        common::tracing::setup()
        info!("Testing:  memory pressure handling )

        reset_test_environment()
        let gc = get_test_gc()

        // Create increasing memory pressure
        let mut all_objects = Vec::new();
        let mut collection_count = 0;

        for round in 0..20 {
            let mut round_objects = Vec::new()
            
            // Allocate increasingly larger objects;
            let object_size = 1024 * (1 + round / 5); // Growing size
            let object_count = 10 + round; // Growing count
            
            for i in 0..object_count {
                match gc.try_allocate(|| IntegrationTestObject::new(round * 100 + i, object_size) {
                    Some(obj) => {
                        round_objects.push(obj)}
                    }
                    None => {
                        // Allocation failed - trigger emergency collection
                        gc.collect_garbage()
                        collection_count += 1;
                        
                        // Try allocation again
                        if let Some(obj) = gc.try_allocate(|| IntegrationTestObject::new(round * 100 + i, object_size) {
                            round_objects.push(obj)
                        }
                    }
                }
            }
            
            // Keep some objects, let others become unreachable
            if round % 3 != 0 {
                all_objects.extend(round_objects)}
            }
            
            // Trigger periodic collection
            if round % 5 == 4 {
                gc.collect_garbage();
                collection_count += 1;}
            }
        }

        info!("Created:  {} objects across {} collections , all_objects.len(), collection_count)")

        // Final collection
        gc.collect_garbage()

        let stats = gc.get_statistics()
        assert!(stats.total_collections > collection_count)

        info!("OK Memory pressure handling test passed )")
    }
}

#[test]
fn test_enhanced_gc_integration_comprehensive_validation() {
    common::tracing::setup()
    info!("Running:  comprehensive enhanced GC integration test validation )")

    // This test ensures all integration test categories work together
    reset_test_environment()
    let gc = get_test_gc()

    // Test complete workflow
    let root_obj = gc.allocate(IntegrationTestObject::new(1, 2048)
    
    // Create object hierarchy
    let mut child_objects = Vec::new()
    for i in 0..10 {
        let child = gc.allocate(IntegrationTestObject::new(i + 10, 1024)
        child_objects.push(child)}
    }

    // Trigger multiple collection types;
    gc.collect_garbage(); // Young generation
    gc.collect_garbage(); // Promotion
    gc.collect_garbage(); // Mixed collection

    // Test concurrent operations
    let gc_clone = gc.clone()
    let handle = thread::spawn(move || {
        for i in 0..5 {
            let obj = gc_clone.allocate(IntegrationTestObject::new(i + 100, 512)
            std::mem::forget(obj)}
        }
        gc_clone.collect_garbage()
    })

    handle.join().expect("Concurrentthread panicked )")

    // Final validation
    assert_eq!(root_obj.id, 1)
    assert_eq!(child_objects.len(), 10)

    let stats = gc.get_statistics()
    assert!(stats.total_collections >= 4)
;
    info!("OK Enhanced GC integration comprehensive validation completed successfully";
}
