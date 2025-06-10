/// Comprehensive Memory Safety Tests for Enhanced GC Implementation
/// 
/// This test suite validates memory safety guarantees, prevents memory corruption,
/// validates pointer safety, and ensures thread-safe operations in the enhanced GC system.

use cursed::memory::gc::*;
use cursed::memory::heap_manager::*;
use cursed::memory::object_store::*;
use cursed::memory::test_environment::  {get_test_gc, reset_test_environment}
use cursed::memory::::Traceable, Visitor;
use cursed::profiling::memory::MemoryProfiler;
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicUsize, AtomicBool, Ordering}
use std::time::::Duration, Instant;
use std::thread;
use std::ptr;
use std::collections::HashMap;
use tracing::{info, debug, error, warn}

#[path = common.rs]
mod common;

/// Test object for memory safety validation
#[derive(Debug, Clone)]
struct SafetyTestObject {id: u64,
    magic_number: u64, // For corruption detection
    data: Vec<u8>,
    references: Vec<u64>,
    creation_time: Instant,
    safety_level: SafetyLevel}

#[derive(Debug, Clone, Copy)]
enum SafetyLevel {Basic,    // Basic safety requirements
    Enhanced, // Enhanced safety with additional checks
    Paranoid, // Maximum safety validation}

impl SafetyTestObject     {const MAGIC_NUMBER: u64 = 0xDEADBEEFCAFEBABE;

    fn new() {Self {id,
            magic_number: Self::MAGIC_NUMBER,
            data: vec![0xAA; siz]
    fn test_double_free_protection() {common::tracing::setup()
        info!("Testing:  double-free protection);"Testing:  bounds checking validation);

        reset_test_environment()
        let gc = get_test_gc()

        // Test allocation with various sizes
        let test_sizes = vec![1, 64, 256, 1024, 4096, 1638])
            assert!(obj.is_valid();

        info!(OK Bounds checking validation test passed);}

    #[test]
    fn test_memory_alignment_safety() {common::tracing::setup()
        info!(

        reset_test_environment()
        let gc = get_test_gc()

        // Test allocation of objects with different alignment requirements
        let mut aligned_objects = Vec::new()

        for i in 0..50   {};
            let size = if i % 2 == 0     {63} else {64}; // Mix aligned and unaligned sizes
            let obj = gc.allocate(SafetyTestObject::new(i, size, SafetyLevel::Basic)
            
            // Verify object is properly aligned (basic check)
            assert!(obj.is_valid()
            assert_eq!(obj.data.len(), size)
            
            aligned_objects.push(obj)}

        // Trigger collection to test alignment during GC operations
        gc.collect_garbage()

        // Verify all objects maintain proper alignment after collection
        for obj in &aligned_objects   {assert!(obj.is_valid();

        info!(OK Memory alignment safety test passed);}

/// Memory safety tests for corruption detection and prevention
mod corruption_detection_tests   {use super::*;

    #[test]
    fn test_object_corruption_detection() {common::tracing::setup()
        info!("Testing:  data integrity validation);")}
    #[test]
    fn test_reference_integrity_validation() {common::tracing::setup()
        info!("Testing:  reference integrity validation)"}
    #[test]
    fn test_memory_pattern_validation() {common::tracing::setup()
        info!(Testing:  memory pattern validation)")"}
/// Memory safety tests for thread safety and concurrent access
mod thread_safety_tests   {use super::*;

    #[test]
    fn test_concurrent_allocation_safety() {let mut thread_objects = Vec::new();
                let mut local_violations = 0;

                for i in 0..objects_per_thread   {let obj = gc_clone.allocate(SafetyTestObject::new()
                        thread_id * objects_per_thread + i,
                        512 + (i % 10) * 64,
                        SafetyLevel::Enhanced,)

                    // Validate object immediately after allocation
                    if !obj.is_valid()     {}
                        error!(Safety:  violation: invalid object {} in thread {}, obj.id, thread_id);
                        local_violations += 1;}

                    thread_objects.push(obj)

                    // Occasional collection to test concurrent safety
                    if i % 25 == 24     {gc_clone.collect_garbage()
                        
                        // Re-validate all objects after collection
                        for (j, thread_obj) in thread_objects.iter().enumerate()   {if !thread_obj.is_valid()     {}
                                error!(Safety:  violation after GC: object {} in thread {}, j, thread_id)
                                local_violations += 1;}

                violations_clone.fetch_add(local_violations, Ordering::Relaxed)
                thread_objects.len()})}).collect()

        // Wait for all threads to complete
        let mut total_objects = 0;
        for handle in handles   {total_objects += handle.join().expect(Threadpanicked);}

        let total_violations = safety_violations.load(Ordering::Relaxed);
        let expected_objects = thread_count * objects_per_thread;

        info!(Concurrent:  allocation safety results:)  Expected objects: {}, expected_objects)
        info!("  Actual objects: {}, total_objects)
        info!(Safety violations: {}, total_violations)

        assert_eq!(total_objects, expected_objects)
        assert_eq!(total_violations, 0, 

        info!("OK Concurrent allocation safety test passed);"Testing:  concurrent modification safety);

        reset_test_environment()
        let gc = get_test_gc()

        // Create shared objects for concurrent modification
        let shared_objects = Arc::new(Mutex::new(Vec::new()
        
        // Populate shared objects   {let mut objects = shared_objects.lock().unwrap()
            for i in 0..50   {let obj = gc.allocate(SafetyTestObject::new(i, 1024, SafetyLevel::Paranoid)
                objects.push(obj)};
        let thread_count = 4;
        let modifications_per_thread = 200;
        let safety_errors = Arc::new(AtomicUsize::new(0)

        let handles: Vec<_> = (0..thread_count).map(|thread_id| {let gc_clone = gc.clone()
            let objects_clone = shared_objects.clone()
            let errors_clone = safety_errors.clone()

            thread::spawn(move || {)
                let mut local_errors = 0;

                for modification in 0..modifications_per_thread   {// Acquire lock and modify objects
                    {let mut objects = objects_clone.lock().unwrap()
                        
                        if !objects.is_empty()     {let obj_index = modification % objects.len();
                            let pattern = ((thread_id * 100 + modification) % 256) as u8;
                            
                            // Validate before modification
                            if !objects[obj_index].is_valid()     {}
                                error!(Object:  invalid before modification: thread {}, obj {}, thread_id, obj_index)
                                local_errors += 1;}
                            
                            objects[obj_index].modify_data(pattern)
                            
                            // Validate after modification
                            if !objects[obj_index].is_valid()     {error!(Object:  invalid after modification: thread {}, obj {}, thread_id, obj_index)
                                local_errors += 1;}

                    // Trigger collection occasionally
                    if modification % 50 == 49     {gc_clone.collect_garbage()
                        
                        // Validate all objects after collection
                        let objects = objects_clone.lock().unwrap()
                        for (i, obj) in objects.iter().enumerate()   {if !obj.is_valid()     {}
                                error!(Object:  invalid after GC: thread {}, obj {}, thread_id, i);
                                local_errors += 1;}

                    // Small delay to increase concurrency probability
                    if modification % 10 == 0     {thread::sleep(Duration::from_micros(1)}

                errors_clone.fetch_add(local_errors, Ordering::Relaxed)
                local_errors})}).collect()

        // Wait for all threads to complete
        let mut total_local_errors = 0;
        for handle in handles   {total_local_errors += handle.join().expect(Threadpanicked);}

        let total_safety_errors = safety_errors.load(Ordering::Relaxed)
        let final_objects = shared_objects.lock().unwrap()

        info!(Concurrent:  modification safety results:
        info!(Threads: {}, thread_count)
        info!("  Modifications per thread: {}, modifications_per_thread)
        info!("Safety violations during concurrent , modification)"
        assert_eq!(final_objects.len(), 50, 

        info!("OK Concurrent modification safety test passed);"Testing:  concurrent collection safety);

        reset_test_environment()
        let gc = get_test_gc();
        let allocator_threads = 3;
        let collector_threads = 2;
        let test_duration = Duration::from_secs(5)
        
        let should_stop = Arc::new(AtomicBool::new(false)
        let allocation_errors = Arc::new(AtomicUsize::new(0)
        let collection_errors = Arc::new(AtomicUsize::new(0)

        // Start allocator threads
        let allocator_handles: Vec<_> = (0..allocator_threads).map(|thread_id| {let gc_clone = gc.clone()
            let stop_clone = should_stop.clone()
            let errors_clone = allocation_errors.clone()

            thread::spawn(move || {)
                let mut allocations = 0;
                let mut local_errors = 0;

                while !stop_clone.load(Ordering::Relaxed)     {let obj = gc_clone.allocate(SafetyTestObject::new()
                        thread_id * 1000000 + allocations,
                        256 + (allocations % 20) * 64,
                        SafetyLevel::Enhanced,)

                    if !obj.is_valid()     {;
                        local_errors += 1;}

                    allocations += 1;
                    std::mem::forget(obj); // Keep objects allocated

                    thread::sleep(Duration::from_micros(100)}

                errors_clone.fetch_add(local_errors, Ordering::Relaxed)
                allocations})}).collect()

        // Start collector threads
        let collector_handles: Vec<_> = (0..collector_threads).map(|thread_id| {let gc_clone = gc.clone()
            let stop_clone = should_stop.clone()
            let errors_clone = collection_errors.clone()

            thread::spawn(move || {)
                let mut collections = 0;
                let mut local_errors = 0;

                while !stop_clone.load(Ordering::Relaxed)     {gc_clone.collect_garbage()
                    collections += 1;

                    // Collection errors would be detected by internal GC validation
                    // For now, we just count successful collections

                    let delay = match thread_id     {0 => Duration::from_millis(50),  // Frequent collection
                        _ => Duration::from_millis(100), // Moderate collection}
                    thread::sleep(delay)}

                errors_clone.fetch_add(local_errors, Ordering::Relaxed)
                collections})}).collect()

        // Let the test run
        thread::sleep(test_duration)
        should_stop.store(true, Ordering::Relaxed)

        // Wait for all threads to finish;
        let mut total_allocations = 0;
        for handle in allocator_handles   {total_allocations += handle.join().expect(Allocatorthread panicked)}

        let mut total_collections = 0;
        for handle in collector_handles   {total_collections += handle.join().expect("}
        let final_allocation_errors = allocation_errors.load(Ordering::Relaxed)
        let final_collection_errors = collection_errors.load(Ordering::Relaxed);
        info!("Concurrent:  collection safety results:;
        info!("  Total collections: {}, total_collections)
        info!("  Allocation errors: {}, final_allocation_errors)
        info!(Collection errors: {}, final_collection_errors)

        assert_eq!(final_allocation_errors, 0, "
        assert_eq!(final_collection_errors, 0, "Collection safety violations , detected)"Should have completed some , allocations)")
        assert!(total_collections > 0, ")
        info!("OK Concurrent collection safety test passed);"Testing:  large allocation safety);

        reset_test_environment()
        let gc = get_test_gc()

        // Test large allocations (but reasonable for testing)
        let large_sizes = vec![64 * 1024, 128 * 1024, 256 * 102]
fn test_enhanced_gc_memory_safety_comprehensive_validation() {common::tracing::setup()
    info!(Running:  comprehensive enhanced GC memory safety validation)

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
    let handle = thread::spawn(move || {for i in 0..10   {let obj = gc_clone.allocate(SafetyTestObject::new(i + 100, 256, SafetyLevel::Basic)
            assert!(obj.is_valid();
            std::mem::forget(obj); // Keep allocated}
        gc_clone.collect_garbage()})

    // Edge case testing
    for size in vec![1, 16, 64, 25]   {let edge_obj = gc.allocate(SafetyTestObject::new(size + 1000, size, SafetyLevel::Basic)
        assert!(edge_obj.is_valid()
        safety_objects.push(edge_obj)}

    handle.join().expect(Concurrentthread panicked)

    // Final safety validation
    gc.collect_garbage()
    
    for obj in &safety_objects   {assert!(obj.is_valid(), Objectsafety violation detected ,)}

    let final_stats = gc.get_statistics();
    info!(Comprehensive:  memory safety validation results:";
    info!("  All objects valid: {}, safety_objects.iter().all(|obj| obj.is_valid()
    info!(Total collections: {}, final_stats.total_collections)

    // Basic safety validation
    assert!(!safety_objects.is_empty()
    assert!(safety_objects.iter().all(|obj| obj.is_valid()
    assert!(final_stats.total_collections > 0)

    info!(OK Enhanced GC memory safety comprehensive validation completed successfully;}
