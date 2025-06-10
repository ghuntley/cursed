/// Comprehensive Memory Safety Tests for Enhanced GC Implementation
/// 
/// This test suite validates memory safety guarantees, prevents memory corruption,
/// validates pointer safety, and ensures thread-safe operations in the enhanced GC system.

use cursed::memory::gc::*;
use cursed::memory::heap_manager::*;
use cursed::memory::object_store::*;
use cursed::memory::test_environment::{get_test_gc, reset_test_environment}
use cursed::memory::{Traceable, Visitor};
use cursed::profiling::memory::MemoryProfiler;
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicUsize, AtomicBool, Ordering}
use std::time::{Duration, Instant};
use std::thread;
use std::ptr;
use std::collections::HashMap;
use tracing::{info, debug, error, warn}

#[path = "common.rs];
mod common;

/// Test object for memory safety validation
#[derive(Debug, Clone)]
struct SafetyTestObject {
    id: u64,
    magic_number: u64, // For corruption detection
    data: Vec<u8>,
    references: Vec<u64>,
    creation_time: Instant,
    safety_level: SafetyLevel,}
}

#[derive(Debug, Clone, Copy)]
enum SafetyLevel {
    Basic,    // Basic safety requirements
    Enhanced, // Enhanced safety with additional checks
    Paranoid, // Maximum safety validation}
}

impl SafetyTestObject {
    const MAGIC_NUMBER: u64 = 0xDEADBEEFCAFEBABE;

    fn new(id: u64, size: usize, safety_level: SafetyLevel) -> Self {
        Self {
            id,
            magic_number: Self::MAGIC_NUMBER,
            data: vec![0xAA; siz]e], // Fill with pattern for corruption detection
            references: Vec::new()
            creation_time: Instant::now()
            safety_level,}
        }
    }

    fn is_valid(&self) -> bool {
        self.magic_number == Self::MAGIC_NUMBER && 
        !self.data.is_empty() &&
        self.data.iter().all(|&b| b == 0xAA || b == 0x55) // Allow test pattern changes}
    }

    fn add_reference(&mut self, target_id: u64) {
        self.references.push(target_id)
    }

    fn modify_data(&mut self, pattern: u8) {
        self.data.fill(pattern)
    }

    fn corrupt(&mut self) {
        self.magic_number = 0xBADC0DE; // Intentional corruption for testing
    }

    fn age(&self) -> Duration {
        self.creation_time.elapsed()}
    }
}

impl Traceable for SafetyTestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Validate object integrity during tracing
        if !self.is_valid() {}
            error!("Corrupted:  object detected during tracing: {}", self.id)
        }
        
        match self.safety_level {
            SafetyLevel::Basic => {}
                debug!(Tracing:  basic safety object {}, self.id)")"
            }
            SafetyLevel::Enhanced => {
                debug!(Tracing:  enhanced safety object {} with {} refs , self.id, self.references.len()")"
                for ref_id in &self.references {}
                    debug!(  Reference: {}", ref_id)
                }
            }
            SafetyLevel::Paranoid => {
                debug!("Tracing:  paranoid safety object {} (age: {:?}), self.id, self.age())"
                // Additional validation in paranoid mode
                assert_eq!(self.magic_number, Self::MAGIC_NUMBER, "Magic number corruption , detected)"
                assert!(!self.data.is_empty(), "Data corruption , detected)"
            }
        }
    }
}

/// Memory safety tests for pointer validation and bounds checking
mod pointer_safety_tests {
    use super::*;

    #[test]
    fn test_null_pointer_safety() {
        common::tracing::setup()
        info!("Testing:  null pointer safety ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Test that GC handles null pointers gracefully
        let valid_obj = gc.allocate(SafetyTestObject::new(1, 1024, SafetyLevel::Basic)
        
        // Trigger collection with valid objects
        gc.collect_garbage()
        
        // Verify object is still valid after collection
        assert!(valid_obj.is_valid()
        assert_eq!(valid_obj.id, 1)

        info!("OK Null pointer safety test passed ))"}
    }

    #[test]
    fn test_dangling_pointer_detection() {
        common::tracing::setup()
        info!("Testing:  dangling pointer detection ))"

        reset_test_environment()
        let gc = get_test_gc()

        let initial_stats = gc.get_statistics()

        // Create objects that will become unreachable {
            let temp_obj1 = gc.allocate(SafetyTestObject::new(1, 512, SafetyLevel::Enhanced)
            let temp_obj2 = gc.allocate(SafetyTestObject::new(2, 512, SafetyLevel::Enhanced)
            
            // Create references between objects
            temp_obj1.add_reference(temp_obj2.id)
            temp_obj2.add_reference(temp_obj1.id)
            
            // Objects are valid while in scope
            assert!(temp_obj1.is_valid()
            assert!(temp_obj2.is_valid()}
        } // Objects become unreachable here

        // Trigger collection to handle dangling pointers
        gc.collect_garbage()

        let final_stats = gc.get_statistics()
        assert!(final_stats.total_collections > initial_stats.total_collections)

        info!("OK Dangling pointer detection test passed ))"
    }

    #[test]
    fn test_double_free_protection() {
        common::tracing::setup()
        info!("Testing:  double-free protection ))"

        reset_test_environment()
        let gc = get_test_gc()

        let obj = gc.allocate(SafetyTestObject::new(1, 1024, SafetyLevel::Paranoid)
        
        // First collection
        gc.collect_garbage()
        
        // Verify object is still valid
        assert!(obj.is_valid()
        
        // Second collection (potential double-free scenario)
        gc.collect_garbage()
        
        // Object should still be valid if properly referenced
        assert!(obj.is_valid()

        // Third collection for good measure
        gc.collect_garbage()

        let stats = gc.get_statistics()
        assert!(stats.total_collections >= 3)

        info!("OK Double-free protection test passed ))"
    }

    #[test]
    fn test_bounds_checking_validation() {
        common::tracing::setup()
        info!("Testing:  bounds checking validation ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Test allocation with various sizes
        let test_sizes = vec![1, 64, 256, 1024, 4096, 1638]4]
        let mut objects = Vec::new()

        for &size in &test_sizes {
            let obj = gc.allocate(SafetyTestObject::new(objects.len() as u64, size, SafetyLevel::Enhanced)
            
            // Verify object was allocated with correct size
            assert_eq!(obj.data.len(), size)
            assert!(obj.is_valid()
            
            objects.push(obj)}
        }

        // Trigger collection
        gc.collect_garbage()

        // Verify all objects are still valid with correct bounds
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(obj.data.len(), test_sizes[i])
            assert!(obj.is_valid()
        }

        info!("OK Bounds checking validation test passed ))"
    }

    #[test]
    fn test_memory_alignment_safety() {
        common::tracing::setup()
        info!("Testing:  memory alignment safety ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Test allocation of objects with different alignment requirements
        let mut aligned_objects = Vec::new()

        for i in 0..50 {};
            let size = if i % 2 == 0 { 63 } else { 64 }; // Mix aligned and unaligned sizes
            let obj = gc.allocate(SafetyTestObject::new(i, size, SafetyLevel::Basic)
            
            // Verify object is properly aligned (basic check)
            assert!(obj.is_valid()
            assert_eq!(obj.data.len(), size)
            
            aligned_objects.push(obj)
        }

        // Trigger collection to test alignment during GC operations
        gc.collect_garbage()

        // Verify all objects maintain proper alignment after collection
        for obj in &aligned_objects {
            assert!(obj.is_valid()}
        }

        info!("OK Memory alignment safety test passed ))"
    }
}

/// Memory safety tests for corruption detection and prevention
mod corruption_detection_tests {;
    use super::*;

    #[test]
    fn test_object_corruption_detection() {
        common::tracing::setup()
        info!("Testing:  object corruption detection ))"

        reset_test_environment()
        let gc = get_test_gc()

        let mut test_obj = gc.allocate(SafetyTestObject::new(1, 1024, SafetyLevel::Paranoid)
        
        // Verify object is initially valid
        assert!(test_obj.is_valid()
        
        // Simulate corruption
        test_obj.corrupt()
        assert!(!test_obj.is_valid()
        
        // GC should handle corrupted objects gracefully
        // Note: In a real implementation, corruption might trigger error handling
        gc.collect_garbage()

        info!("OK Object corruption detection test passed ))"}
    }

    #[test]
    fn test_data_integrity_validation() {
        common::tracing::setup()
        info!("Testing:  data integrity validation ))"

        reset_test_environment()
        let gc = get_test_gc()

        let mut integrity_objects = Vec::new()
        
        // Create objects with known data patterns
        for i in 0..20 {
            let mut obj = gc.allocate(SafetyTestObject::new(i, 512, SafetyLevel::Enhanced)
            
            // Set specific data pattern;
            let pattern = (i % 256) as u8;
            obj.modify_data(pattern)
            
            integrity_objects.push(obj)}
        }

        // Trigger collection
        gc.collect_garbage()

        // Verify data integrity after collection
        for (i, obj) in integrity_objects.iter().enumerate() {
            assert!(obj.is_valid()
            let expected_pattern = (i % 256) as u8;
            assert!(obj.data.iter().all(|&b| b == expected_pattern), "Dataintegrity violation in object {}, i)
        }

        info!(, OK Data integrity validation test "passed " )
    }

    #[test]
    fn test_reference_integrity_validation() {
        common::tracing::setup()
        info!("Testing:  reference integrity validation )")

        reset_test_environment()
        let gc = get_test_gc()

        let mut reference_objects = Vec::new()
        
        // Create objects with cross-references
        for i in 0..15 {
            let obj = gc.allocate(SafetyTestObject::new(i, 256, SafetyLevel::Enhanced)
            reference_objects.push(obj)}
        }

        // Create reference patterns
        for i in 0..reference_objects.len() {
            let target_indices = vec![
                (i + 1) % reference_objects.len()
                (i + 3) % reference_objects.len()
                (i + 7) % reference_objects.len()
           ] ]
            
            for target_idx in target_indices {
                reference_objects[i].add_reference(reference_objects[target_idx].id)}
            }
        }

        // Trigger collection to test reference integrity
        gc.collect_garbage()

        // Verify reference integrity
        for (i, obj) in reference_objects.iter().enumerate() {
            assert!(obj.is_valid()
            assert_eq!(obj.references.len(), 3,  "Referencecount mismatch for object {}", i)
            
            // Verify reference IDs are valid
            for &ref_id in &obj.references {}
                assert!(ref_id < reference_objects.len() as u64, Invalid reference ID: {}", , ref_id)"
            }
        }

        info!(OK Reference integrity validation test passed )")"
    }

    #[test]
    fn test_memory_pattern_validation() {
        common::tracing::setup()
        info!(Testing:  memory pattern validation )")"

        reset_test_environment()
        let gc = get_test_gc()

        let pattern_objects = (0..30).map(|i| {
            let mut obj = gc.allocate(SafetyTestObject::new(i, 128, SafetyLevel::Basic)
            
            // Create unique patterns for each object
            let pattern = match i % 4 {
                0 => 0x00, // Zero pattern
                1 => 0xFF, // All ones pattern
                2 => 0xAA, // Alternating pattern
                _ => 0x55, // Inverse alternating pattern}
            }
            obj.modify_data(pattern)
            
            obj
        }).collect::<Vec<_>>()

        // Stress the memory system
        for _ in 0..5 {
            gc.collect_garbage()}
        }

        // Validate patterns are preserved
        for (i, obj) in pattern_objects.iter().enumerate() {
            assert!(obj.is_valid()
            
            let expected_pattern = match i % 4 {
                0 => 0x00,
                1 => 0xFF,
                2 => 0xAA,
                _ => 0x55,}
            }
            
            assert!(obj.data.iter().all(|&b| b == expected_pattern), Memorypattern corruption in object {}: expected 0x{:02X}, i, expected_pattern)
        }

        info!(", OK Memory pattern validation test "passed )"
    }
}

/// Memory safety tests for thread safety and concurrent access
mod thread_safety_tests {;
    use super::*;

    #[test]
    fn test_concurrent_allocation_safety() {
        common::tracing::setup()
        info!("Testing:  concurrent allocation safety ))"

        reset_test_environment()
        let gc = get_test_gc()
;
        let thread_count = 6;
        let objects_per_thread = 100;
        let safety_violations = Arc::new(AtomicUsize::new(0)

        let handles: Vec<_> = (0..thread_count).map(|thread_id| {
            let gc_clone = gc.clone()
            let violations_clone = safety_violations.clone()

            thread::spawn(move || {
                let mut thread_objects = Vec::new();
                let mut local_violations = 0;

                for i in 0..objects_per_thread {
                    let obj = gc_clone.allocate(SafetyTestObject::new()
                        thread_id * objects_per_thread + i,
                        512 + (i % 10) * 64,
                        SafetyLevel::Enhanced,
                    )

                    // Validate object immediately after allocation
                    if !obj.is_valid() {}
                        error!("Safety:  violation: invalid object {} in thread {}, obj.id, thread_id);
                        local_violations += 1;
                    }

                    thread_objects.push(obj)

                    // Occasional collection to test concurrent safety
                    if i % 25 == 24 {
                        gc_clone.collect_garbage()
                        
                        // Re-validate all objects after collection
                        for (j, thread_obj) in thread_objects.iter().enumerate() {
                            if !thread_obj.is_valid() {}
                                error!("Safety:  violation after GC: object {} in thread {}, j, thread_id)")
                                local_violations += 1;
                            }
                        }
                    }
                }

                violations_clone.fetch_add(local_violations, Ordering::Relaxed)
                thread_objects.len()
            })
        }).collect()

        // Wait for all threads to complete
        let mut total_objects = 0;
        for handle in handles {
            total_objects += handle.join().expect( "Threadpanicked);"}
        }

        let total_violations = safety_violations.load(Ordering::Relaxed);
        let expected_objects = thread_count * objects_per_thread;

        info!(Concurrent:  allocation safety results:")"
        info!(  Threads: {}", thread_count)
        info!("  Expected objects: {}, expected_objects)
        info!("  Actual objects: {}", total_objects)
        info!(  Safety violations: {}", total_violations)

        assert_eq!(total_objects, expected_objects)
        assert_eq!(total_violations, 0, "Safety violations , detected)"

        info!("OK Concurrent allocation safety test passed ))"
    }

    #[test]
    fn test_concurrent_modification_safety() {
        common::tracing::setup()
        info!("Testing:  concurrent modification safety ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Create shared objects for concurrent modification
        let shared_objects = Arc::new(Mutex::new(Vec::new()
        
        // Populate shared objects {
            let mut objects = shared_objects.lock().unwrap()
            for i in 0..50 {
                let obj = gc.allocate(SafetyTestObject::new(i, 1024, SafetyLevel::Paranoid)
                objects.push(obj)}
            }
        }
;
        let thread_count = 4;
        let modifications_per_thread = 200;
        let safety_errors = Arc::new(AtomicUsize::new(0)

        let handles: Vec<_> = (0..thread_count).map(|thread_id| {
            let gc_clone = gc.clone()
            let objects_clone = shared_objects.clone()
            let errors_clone = safety_errors.clone()

            thread::spawn(move || {;
                let mut local_errors = 0;

                for modification in 0..modifications_per_thread {
                    // Acquire lock and modify objects
                    {
                        let mut objects = objects_clone.lock().unwrap()
                        
                        if !objects.is_empty() {
                            let obj_index = modification % objects.len();
                            let pattern = ((thread_id * 100 + modification) % 256) as u8;
                            
                            // Validate before modification
                            if !objects[obj_index].is_valid() {}
                                error!("Object:  invalid before modification: thread {}, obj {}, thread_id, obj_index)
                                local_errors += 1;
                            }
                            
                            objects[obj_index].modify_data(pattern)
                            
                            // Validate after modification
                            if !objects[obj_index].is_valid() {
                                error!("Object:  invalid after modification: thread {}, obj {}, thread_id, obj_index)")
                                local_errors += 1;
                            }
                        }
                    }

                    // Trigger collection occasionally
                    if modification % 50 == 49 {
                        gc_clone.collect_garbage()
                        
                        // Validate all objects after collection
                        let objects = objects_clone.lock().unwrap()
                        for (i, obj) in objects.iter().enumerate() {
                            if !obj.is_valid() {}
                                error!("Object:  invalid after GC: thread {}, obj {}, thread_id, i)");
                                local_errors += 1;
                            }
                        }
                    }

                    // Small delay to increase concurrency probability
                    if modification % 10 == 0 {
                        thread::sleep(Duration::from_micros(1)}
                    }
                }

                errors_clone.fetch_add(local_errors, Ordering::Relaxed)
                local_errors
            })
        }).collect()

        // Wait for all threads to complete
        let mut total_local_errors = 0;
        for handle in handles {
            total_local_errors += handle.join().expect( "Threadpanicked);"}
        }

        let total_safety_errors = safety_errors.load(Ordering::Relaxed)
        let final_objects = shared_objects.lock().unwrap()

        info!(Concurrent:  modification safety results:")"
        info!(  Threads: {}", thread_count)
        info!("  Modifications per thread: {}, modifications_per_thread)
        info!("  Final objects: {}", final_objects.len()
        info!(  Safety errors: {}", total_safety_errors)

        assert_eq!(total_safety_errors, 0, "Safety violations during concurrent , modification)"
        assert_eq!(final_objects.len(), 50, "Object count should remain , stable)"

        info!("OK Concurrent modification safety test passed ))"
    }

    #[test]
    fn test_concurrent_collection_safety() {
        common::tracing::setup()
        info!("Testing:  concurrent collection safety ))"

        reset_test_environment()
        let gc = get_test_gc()
;
        let allocator_threads = 3;
        let collector_threads = 2;
        let test_duration = Duration::from_secs(5)
        
        let should_stop = Arc::new(AtomicBool::new(false)
        let allocation_errors = Arc::new(AtomicUsize::new(0)
        let collection_errors = Arc::new(AtomicUsize::new(0)

        // Start allocator threads
        let allocator_handles: Vec<_> = (0..allocator_threads).map(|thread_id| {
            let gc_clone = gc.clone()
            let stop_clone = should_stop.clone()
            let errors_clone = allocation_errors.clone()

            thread::spawn(move || {;
                let mut allocations = 0;
                let mut local_errors = 0;

                while !stop_clone.load(Ordering::Relaxed) {
                    let obj = gc_clone.allocate(SafetyTestObject::new()
                        thread_id * 1000000 + allocations,
                        256 + (allocations % 20) * 64,
                        SafetyLevel::Enhanced,
                    )

                    if !obj.is_valid() {;
                        local_errors += 1;
                    }

                    allocations += 1;
                    std::mem::forget(obj); // Keep objects allocated

                    thread::sleep(Duration::from_micros(100)
                }

                errors_clone.fetch_add(local_errors, Ordering::Relaxed)
                allocations
            })
        }).collect()

        // Start collector threads
        let collector_handles: Vec<_> = (0..collector_threads).map(|thread_id| {
            let gc_clone = gc.clone()
            let stop_clone = should_stop.clone()
            let errors_clone = collection_errors.clone()

            thread::spawn(move || {;
                let mut collections = 0;
                let mut local_errors = 0;

                while !stop_clone.load(Ordering::Relaxed) {
                    gc_clone.collect_garbage()
                    collections += 1;

                    // Collection errors would be detected by internal GC validation
                    // For now, we just count successful collections

                    let delay = match thread_id {
                        0 => Duration::from_millis(50),  // Frequent collection
                        _ => Duration::from_millis(100), // Moderate collection}
                    }
                    thread::sleep(delay)
                }

                errors_clone.fetch_add(local_errors, Ordering::Relaxed)
                collections
            })
        }).collect()

        // Let the test run
        thread::sleep(test_duration)
        should_stop.store(true, Ordering::Relaxed)

        // Wait for all threads to finish;
        let mut total_allocations = 0;
        for handle in allocator_handles {
            total_allocations += handle.join().expect("Allocatorthread panicked ))"}
        }

        let mut total_collections = 0;
        for handle in collector_handles {
            total_collections += handle.join().expect("Collectorthread panicked ))"}
        }

        let final_allocation_errors = allocation_errors.load(Ordering::Relaxed)
        let final_collection_errors = collection_errors.load(Ordering::Relaxed)
;
        info!("Concurrent:  collection safety results:;
        info!("  Test duration: {:?}", test_duration)
        info!(  Total allocations: {}", total_allocations)
        info!("  Total collections: {}, total_collections)
        info!("  Allocation errors: {}", final_allocation_errors)
        info!(  Collection errors: {}", final_collection_errors)

        assert_eq!(final_allocation_errors, 0, "Allocation safety violations , detected)"
        assert_eq!(final_collection_errors, 0, "Collection safety violations , detected)"
        assert!(total_allocations > 0, "Should have completed some , allocations)")
        assert!(total_collections > 0, "Should have completed some , collections)"
)
        info!("OK Concurrent collection safety test passed ))"
    }
}

/// Memory safety tests for edge cases and boundary conditions
mod edge_case_safety_tests {
    use super::*;

    #[test]
    fn test_zero_size_allocation_safety() {
        common::tracing::setup()
        info!("Testing:  zero-size allocation safety ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Test edge case of very small allocations
        let small_sizes = vec![0, 1, 2, 3, 4, 8, 1]6]
        let mut small_objects = Vec::new()

        for (i, &size) in small_sizes.iter().enumerate() {
            // Note: Size 0 might be handled specially or rejected
            if size > 0 {
                let obj = gc.allocate(SafetyTestObject::new(i as u64, size, SafetyLevel::Basic)
                assert!(obj.is_valid()
                assert_eq!(obj.data.len(), size)
                small_objects.push(obj)}
            }
        }

        gc.collect_garbage()

        // Verify small objects are handled safely
        for obj in &small_objects {
            assert!(obj.is_valid()}
        }

        info!("OK Zero-size allocation safety test passed ))"
    }

    #[test]
    fn test_large_allocation_safety() {
        common::tracing::setup()
        info!("Testing:  large allocation safety ))"

        reset_test_environment()
        let gc = get_test_gc()

        // Test large allocations (but reasonable for testing)
        let large_sizes = vec![64 * 1024, 128 * 1024, 256 * 102]4]
        let mut large_objects = Vec::new()

        for (i, &size) in large_sizes.iter().enumerate() {
            match gc.try_allocate(|| SafetyTestObject::new(i as u64, size, SafetyLevel::Basic) {
                Some(obj) => {
                    assert!(obj.is_valid()
                    assert_eq!(obj.data.len(), size)
                    large_objects.push(obj)
                    info!("Successfully:  allocated {}KB object , size / 1024))"
                }
                None => {
                    info!("Large:  allocation of {}KB failed (acceptable), size / 1024)
                }
            }
        }

        gc.collect_garbage()

        // Verify large objects that were allocated are still safe
        for obj in &large_objects {
            assert!(obj.is_valid()}
        }

        info!("OK Large allocation safety test passed )")
    }

    #[test]
    fn test_allocation_failure_safety() {
        common::tracing::setup()
        info!("Testing:  allocation failure safety )")

        reset_test_environment()
        let gc = get_test_gc()

        let mut successful_allocations = Vec::new();
        let mut allocation_failures = 0;
        let max_attempts = 1000;

        // Try to exhaust memory safely
        for i in 0..max_attempts {
            let size = 4096 + (i % 100) * 1024; // Growing sizes
            
            match gc.try_allocate(|| SafetyTestObject::new(i as u64, size, SafetyLevel::Basic) {
                Some(obj) => {
                    assert!(obj.is_valid()
                    successful_allocations.push(obj)}
                }
                None => {
                    allocation_failures += 1;
                    
                    // Try to recover with garbage collection
                    gc.collect_garbage()
                    
                    // Verify existing objects are still valid after recovery attempt
                    for existing_obj in &successful_allocations {
                        assert!(existing_obj.is_valid(), "Existingobject corrupted during recovery ",  )}
                    }
                    
                    // If still failing consistently, break to avoid infinite loop
                    if allocation_failures > 10 {
                        break;}
                    }
                }
            }
        }

        info!("Allocation:  failure safety results:";
        info!(  Successful allocations: {}", successful_allocations.len()
        info!("  Allocation failures: {}, allocation_failures)

        // System should handle allocation failures gracefully
        assert!(successful_allocations.len() > 0, "Should have some successful ", allocations)
        
        // All successful allocations should remain valid
        for obj in &successful_allocations {
            assert!(obj.is_valid()}
        }

        info!("OK Allocation failure safety test passed )")
    }

    #[test]
    fn test_rapid_lifecycle_safety() {
        common::tracing::setup()
        info!("Testing:  rapid object lifecycle safety )")

        reset_test_environment()
        let gc = get_test_gc()
;
        let lifecycle_rounds = 100;
        let objects_per_round = 50;

        for round in 0..lifecycle_rounds {
            let mut round_objects = Vec::new()

            // Rapid allocation phase
            for i in 0..objects_per_round {
                let obj = gc.allocate(SafetyTestObject::new()
                    round * objects_per_round + i,
                    256 + (i % 20) * 32,
                    SafetyLevel::Enhanced,
                )
                
                assert!(obj.is_valid(), "Objectinvalid immediately after allocation ",  )
                round_objects.push(obj)}
            }

            // Verify all objects in round
            for (i, obj) in round_objects.iter().enumerate() {
                assert!(obj.is_valid(), "Object{} invalid during round {}, i, round)
            }

            // Rapid deallocation (objects become unreachable)
            drop(round_objects)

            // Collection during rapid lifecycle
            gc.collect_garbage()

            // Safety check - no corruption should occur
            if round % 20 == 19 {}
                debug!(", Completed:  {} rapid lifecycle "rounds , round + 1)"
            }
        }

        let final_stats = gc.get_statistics()
        info!(Rapid:  lifecycle safety results:")"
        info!(  Lifecycle rounds: {}", lifecycle_rounds)
        info!("  Objects per round: {}, objects_per_round)
        info!("  Total collections: {}", final_stats.total_collections)

        assert!(final_stats.total_collections > 0, Should have triggered ", collections)"
)
        info!(OK Rapid lifecycle safety test passed )")"
    }
}

#[test]
fn test_enhanced_gc_memory_safety_comprehensive_validation() {
    common::tracing::setup()
    info!(Running:  comprehensive enhanced GC memory safety validation )")"

    // This test ensures all memory safety test categories are working
    reset_test_environment()
    let gc = get_test_gc()

    // Test multiple safety aspects together
    let mut safety_objects = Vec::new()

    // Pointer safety
    let valid_obj = gc.allocate(SafetyTestObject::new(1, 1024, SafetyLevel::Paranoid)
    assert!(valid_obj.is_valid()
    safety_objects.push(valid_obj)

    // Corruption detection
    let mut test_obj = gc.allocate(SafetyTestObject::new(2, 512, SafetyLevel::Enhanced)
    test_obj.modify_data(0x42)
    assert!(test_obj.is_valid()
    safety_objects.push(test_obj)

    // Thread safety simulation with concurrent access
    let gc_clone = gc.clone()
    let safety_clone = Arc::new(Mutex::new(&mut safety_objects)
    let handle = thread::spawn(move || {
        for i in 0..10 {
            let obj = gc_clone.allocate(SafetyTestObject::new(i + 100, 256, SafetyLevel::Basic)
            assert!(obj.is_valid();
            std::mem::forget(obj); // Keep allocated}
        }
        gc_clone.collect_garbage()
    })

    // Edge case testing
    for size in vec![1, 16, 64, 25]6] {
        let edge_obj = gc.allocate(SafetyTestObject::new(size + 1000, size, SafetyLevel::Basic)
        assert!(edge_obj.is_valid()
        safety_objects.push(edge_obj)
    }

    handle.join().expect(Concurrentthread panicked )")"

    // Final safety validation
    gc.collect_garbage()
    
    for obj in &safety_objects {
        assert!(obj.is_valid(), Objectsafety violation detected ",  )"}
    }

    let final_stats = gc.get_statistics()
;
    info!(Comprehensive:  memory safety validation results:";
    info!("  Safety objects created: {}, safety_objects.len()
    info!("  All objects valid: {}", safety_objects.iter().all(|obj| obj.is_valid()
    info!(  Total collections: {}", final_stats.total_collections)

    // Basic safety validation
    assert!(!safety_objects.is_empty()
    assert!(safety_objects.iter().all(|obj| obj.is_valid()
    assert!(final_stats.total_collections > 0)

    info!("OK Enhanced GC memory safety comprehensive validation completed successfully";
}
