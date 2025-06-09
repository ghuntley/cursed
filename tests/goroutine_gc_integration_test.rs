//! Integration tests for goroutine-aware garbage collection
//!
//! These tests verify that the GC correctly handles concurrent goroutines,
//! properly scans goroutine stacks, and maintains memory safety in concurrent
//! environments.

use std::sync::{Arc, Mutex, atomic::{AtomicI32, AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;
use std::ffi::c_void;

use tracing::{debug, info, warn};
use cursed::memory::{
    GarbageCollector, GoroutineGarbageCollector, get_global_goroutine_gc,
    SafePointType, Traceable, Tag, Visitor
};
use cursed::runtime::goroutine::get_global_scheduler;

// Common test infrastructure
#[path = "common.rs"]
pub mod common;

macro_rules! init_tracing {
    () => {
        // Simple tracing setup for tests
        use tracing_subscriber;
        let _ = tracing_subscriber::fmt::try_init();
    };
}

pub struct Timer {
    _name: String,
    _start: std::time::Instant,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            _start: std::time::Instant::now(),
        }
    }
}

/// Test object that's safe to use across goroutines
#[derive(Debug, Clone)]
struct TestObject {
    id: usize,
    value: i32,
    references: Vec<usize>,
}

impl Traceable for TestObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for &ref_id in &self.references {
            visitor.visit_ptr(ref_id, Tag::Object);
        }
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + self.references.capacity() * std::mem::size_of::<usize>()
    }

    fn tag(&self) -> Tag {
        Tag::Object
    }
}

unsafe impl Send for TestObject {}
unsafe impl Sync for TestObject {}

/// Test basic goroutine registration and unregistration
#[test]
fn test_goroutine_registration() {
    init_tracing!();
    let _timer = Timer::new("goroutine_registration");

    let gc = Arc::new(GarbageCollector::new());
    let goroutine_gc = GoroutineGarbageCollector::new(gc);

    // Test registration
    goroutine_gc.register_goroutine(1, 0x1000, 0x2000);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 1);

    // Test multiple registrations
    goroutine_gc.register_goroutine(2, 0x3000, 0x2000);
    goroutine_gc.register_goroutine(3, 0x5000, 0x2000);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 3);

    // Test unregistration
    goroutine_gc.unregister_goroutine(2);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 2);

    // Test unregistering all
    goroutine_gc.unregister_goroutine(1);
    goroutine_gc.unregister_goroutine(3);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0);

    info!("Goroutine registration test passed");
}

/// Test goroutine-local GC roots
#[test]
fn test_goroutine_local_roots() {
    init_tracing!();
    let _timer = Timer::new("goroutine_local_roots");

    let gc = Arc::new(GarbageCollector::new());
    let goroutine_gc = GoroutineGarbageCollector::new(gc.clone());

    // Register a goroutine
    goroutine_gc.register_goroutine(1, 0x1000, 0x2000);

    // Allocate a test object
    let obj = TestObject { id: 1, value: 42, references: vec![] };
    let gc_obj = gc.allocate(obj);
    let obj_ptr = gc_obj.ptr();

    // Add as goroutine-local root
    goroutine_gc.add_goroutine_root(1, obj_ptr);

    let stats = goroutine_gc.get_goroutine_stats();
    assert_eq!(stats.total_local_roots, 1);

    // Object should be kept alive by goroutine root
    assert!(gc.is_alive(obj_ptr));

    // Remove the root
    goroutine_gc.remove_goroutine_root(1, obj_ptr);

    let stats = goroutine_gc.get_goroutine_stats();
    assert_eq!(stats.total_local_roots, 0);

    info!("Goroutine local roots test passed");
}

/// Test safe point coordination
#[test]
fn test_safe_point_coordination() {
    init_tracing!();
    let _timer = Timer::new("safe_point_coordination");

    let gc = Arc::new(GarbageCollector::new());
    let goroutine_gc = GoroutineGarbageCollector::new(gc);

    let coordinator = &goroutine_gc.safe_point_coordinator;

    // Test basic safe point functionality
    coordinator.request_safe_points();
    
    // Simulate goroutine reaching safe point
    let reached_safe_point = coordinator.goroutine_at_safe_point(1);
    assert!(reached_safe_point);

    // Release safe points
    coordinator.release_safe_points();
    
    // Goroutine should no longer be required to stay at safe point
    let reached_safe_point = coordinator.goroutine_at_safe_point(1);
    assert!(!reached_safe_point);

    info!("Safe point coordination test passed");
}

/// Test concurrent goroutines with GC
#[test]
fn test_concurrent_goroutines_with_gc() {
    init_tracing!();
    let _timer = Timer::new("concurrent_goroutines_with_gc");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let num_goroutines = 5;
    let objects_per_goroutine = 10;
    let total_objects = num_goroutines * objects_per_goroutine;

    let allocated_count = Arc::new(AtomicUsize::new(0));
    let collected_count = Arc::new(AtomicUsize::new(0));

    // Function to run in each goroutine
    unsafe extern "C" fn goroutine_work(data: *mut c_void) -> *mut c_void {
        let data = data as *mut (usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>);
        let (goroutine_id, gc, goroutine_gc, allocated_count) = &*data;

        debug!(goroutine_id = goroutine_id, "Goroutine starting work");

        // Register with GC
        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000);

        // Allocate objects
        for i in 0..10 {
            let obj = TestObject {
                id: *goroutine_id * 1000 + i,
                value: i as i32,
                references: vec![],
            };
            
            let gc_obj = gc.allocate(obj.clone());
            let obj_ptr = gc_obj.ptr();
            
            // Add as goroutine-local root temporarily
            goroutine_gc.add_goroutine_root(*goroutine_id as u64, obj_ptr);
            allocated_count.fetch_add(1, Ordering::SeqCst);
            
            // Safe point: allocation
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Allocation);
            
            // Simulate some work
            thread::sleep(Duration::from_millis(1));
            
            // Remove the root to allow collection
            goroutine_gc.remove_goroutine_root(*goroutine_id as u64, obj_ptr);
        }

        // Unregister from GC
        goroutine_gc.unregister_goroutine(*goroutine_id as u64);

        debug!(goroutine_id = goroutine_id, "Goroutine finished work");
        std::ptr::null_mut()
    }

    // Spawn goroutines
    let mut goroutine_ids = Vec::new();
    for i in 0..num_goroutines {
        let data = Box::new((i, gc.clone(), goroutine_gc.clone(), allocated_count.clone()));
        let data_ptr = Box::into_raw(data) as *mut c_void;
        
        let id = scheduler.spawn_goroutine(goroutine_work, data_ptr);
        goroutine_ids.push(id);
        
        debug!(goroutine_id = id, "Spawned goroutine");
    }

    // Let goroutines run for a bit
    thread::sleep(Duration::from_millis(100));

    // Trigger goroutine-aware GC multiple times
    for i in 0..3 {
        debug!(iteration = i, "Triggering goroutine-aware GC");
        
        match goroutine_gc.collect_garbage_goroutine_aware() {
            Ok(stats) => {
                info!(
                    iteration = i,
                    goroutines = stats.total_goroutines,
                    stack_roots = stats.stack_roots_found,
                    "GC completed successfully"
                );
            },
            Err(e) => {
                warn!(iteration = i, error = %e, "GC failed");
            }
        }
        
        thread::sleep(Duration::from_millis(50));
    }

    // Wait for all goroutines to complete
    for id in goroutine_ids {
        if let Err(e) = scheduler.wait_for_goroutine(id) {
            warn!(goroutine_id = id, error = %e, "Failed to wait for goroutine");
        }
    }

    let final_allocated = allocated_count.load(Ordering::SeqCst);
    info!(allocated_objects = final_allocated, "Test completed");

    assert_eq!(final_allocated, total_objects);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0);

    info!("Concurrent goroutines with GC test passed");
}

/// Test memory leak prevention with goroutine lifecycles
#[test]
fn test_memory_leak_prevention() {
    init_tracing!();
    let _timer = Timer::new("memory_leak_prevention");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let initial_stats = gc.stats();
    let leak_counter = Arc::new(AtomicI32::new(0));

    // Function that allocates objects and then terminates
    unsafe extern "C" fn leaky_goroutine(data: *mut c_void) -> *mut c_void {
        let data = data as *mut (usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicI32>);
        let (goroutine_id, gc, goroutine_gc, counter) = &*data;

        // Register with GC
        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000);

        // Allocate objects without keeping references
        for i in 0..5 {
            let obj = TestObject {
                id: *goroutine_id * 100 + i,
                value: i as i32,
                references: vec![],
            };
            
            let _gc_obj = gc.allocate(obj);
            counter.fetch_add(1, Ordering::SeqCst);
            
            // Safe point after allocation
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Allocation);
        }

        // Unregister from GC (objects should be collected)
        goroutine_gc.unregister_goroutine(*goroutine_id as u64);

        std::ptr::null_mut()
    }

    // Spawn multiple short-lived goroutines
    for round in 0..3 {
        debug!(round = round, "Starting round of leaky goroutines");
        
        let mut goroutine_ids = Vec::new();
        
        for i in 0..5 {
            let data = Box::new((
                round * 10 + i,
                gc.clone(),
                goroutine_gc.clone(),
                leak_counter.clone()
            ));
            let data_ptr = Box::into_raw(data) as *mut c_void;
            
            let id = scheduler.spawn_goroutine(leaky_goroutine, data_ptr);
            goroutine_ids.push(id);
        }

        // Wait for goroutines to complete
        for id in goroutine_ids {
            scheduler.wait_for_goroutine(id).unwrap();
        }

        // Force garbage collection
        match goroutine_gc.collect_garbage_goroutine_aware() {
            Ok(stats) => {
                debug!(round = round, freed_objects = stats.gc_stats.freed_objects, "GC round completed");
            },
            Err(e) => {
                warn!(round = round, error = %e, "GC round failed");
            }
        }
    }

    let final_stats = gc.stats();
    let total_allocated = leak_counter.load(Ordering::SeqCst);

    info!(
        initial_objects = initial_stats.object_count,
        final_objects = final_stats.object_count,
        total_allocated = total_allocated,
        freed_objects = final_stats.freed_objects,
        "Memory leak prevention test completed"
    );

    // Verify that memory was properly collected
    assert!(final_stats.freed_objects > 0, "Some objects should have been freed");
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should be active");

    info!("Memory leak prevention test passed");
}

/// Test stress scenario with many short-lived goroutines
#[test]
fn test_stress_many_short_lived_goroutines() {
    init_tracing!();
    let _timer = Timer::new("stress_many_short_lived_goroutines");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let total_goroutines = 20;
    let rounds = 3;
    let completion_counter = Arc::new(AtomicI32::new(0));

    // Simple goroutine function
    unsafe extern "C" fn stress_goroutine(data: *mut c_void) -> *mut c_void {
        let data = data as *mut (usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicI32>);
        let (goroutine_id, gc, goroutine_gc, counter) = &*data;

        // Register with GC
        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x1000);

        // Quick allocation and deallocation pattern
        for i in 0..3 {
            let obj = TestObject {
                id: *goroutine_id * 10 + i,
                value: i as i32,
                references: vec![],
            };
            
            let gc_obj = gc.allocate(obj);
            let obj_ptr = gc_obj.ptr();
            
            // Briefly add and remove as root
            goroutine_gc.add_goroutine_root(*goroutine_id as u64, obj_ptr);
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Allocation);
            goroutine_gc.remove_goroutine_root(*goroutine_id as u64, obj_ptr);
        }

        // Unregister from GC
        goroutine_gc.unregister_goroutine(*goroutine_id as u64);
        counter.fetch_add(1, Ordering::SeqCst);

        std::ptr::null_mut()
    }

    for round in 0..rounds {
        debug!(round = round, "Starting stress test round");
        
        let mut goroutine_ids = Vec::new();
        
        // Spawn many goroutines quickly
        for i in 0..total_goroutines {
            let data = Box::new((
                round * 1000 + i,
                gc.clone(),
                goroutine_gc.clone(),
                completion_counter.clone()
            ));
            let data_ptr = Box::into_raw(data) as *mut c_void;
            
            let id = scheduler.spawn_goroutine(stress_goroutine, data_ptr);
            goroutine_ids.push(id);
        }

        // Let them run concurrently
        thread::sleep(Duration::from_millis(10));

        // Trigger GC while goroutines are running
        if let Ok(stats) = goroutine_gc.collect_garbage_goroutine_aware() {
            debug!(
                round = round,
                active_goroutines = stats.total_goroutines,
                "Mid-round GC completed"
            );
        }

        // Wait for completion
        for id in goroutine_ids {
            scheduler.wait_for_goroutine(id).unwrap();
        }

        // GC after round completion
        if let Ok(stats) = goroutine_gc.collect_garbage_goroutine_aware() {
            debug!(
                round = round,
                freed_objects = stats.gc_stats.freed_objects,
                "End-of-round GC completed"
            );
        }
    }

    let completed = completion_counter.load(Ordering::SeqCst);
    let expected = (total_goroutines * rounds) as i32;

    assert_eq!(completed, expected, "All goroutines should have completed");
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should remain active");

    info!(
        completed_goroutines = completed,
        "Stress test with many short-lived goroutines passed"
    );
}

/// Test race conditions between GC and goroutine creation/destruction
#[test]
fn test_gc_goroutine_race_conditions() {
    init_tracing!();
    let _timer = Timer::new("gc_goroutine_race_conditions");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let continue_flag = Arc::new(AtomicI32::new(1));
    let goroutine_counter = Arc::new(AtomicI32::new(0));

    // Goroutine that creates and destroys objects rapidly
    let goroutine_gc_clone = goroutine_gc.clone();
    let gc_clone = gc.clone();
    let continue_flag_clone = continue_flag.clone();
    let counter_clone = goroutine_counter.clone();
    
    let creator_thread = thread::spawn(move || {
        let mut local_id = 0;
        while continue_flag_clone.load(Ordering::SeqCst) == 1 {
            local_id += 1;
            let goroutine_id = local_id as u64;
            
            // Register goroutine
            goroutine_gc_clone.register_goroutine(goroutine_id, 0x1000, 0x1000);
            counter_clone.fetch_add(1, Ordering::SeqCst);
            
            // Allocate an object
            let obj = TestObject {
                id: local_id,
                value: 42,
                references: vec![],
            };
            let gc_obj = gc_clone.allocate(obj);
            let obj_ptr = gc_obj.ptr();
            
            // Add as root
            goroutine_gc_clone.add_goroutine_root(goroutine_id, obj_ptr);
            
            // Safe point
            goroutine_gc_clone.goroutine_safe_point(goroutine_id, SafePointType::Allocation);
            
            // Remove root and unregister
            goroutine_gc_clone.remove_goroutine_root(goroutine_id, obj_ptr);
            goroutine_gc_clone.unregister_goroutine(goroutine_id);
            
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Thread that triggers GC repeatedly
    let goroutine_gc_clone2 = goroutine_gc.clone();
    let continue_flag_clone2 = continue_flag.clone();
    
    let gc_thread = thread::spawn(move || {
        let mut gc_count = 0;
        while continue_flag_clone2.load(Ordering::SeqCst) == 1 {
            gc_count += 1;
            
            if let Ok(stats) = goroutine_gc_clone2.collect_garbage_goroutine_aware() {
                debug!(
                    gc_iteration = gc_count,
                    active_goroutines = stats.total_goroutines,
                    "Race condition GC completed"
                );
            }
            
            thread::sleep(Duration::from_millis(5));
        }
        debug!(total_gc_runs = gc_count, "GC thread completed");
    });

    // Let the race condition test run for a short time
    thread::sleep(Duration::from_millis(100));
    
    // Signal threads to stop
    continue_flag.store(0, Ordering::SeqCst);
    
    // Wait for threads to complete
    creator_thread.join().unwrap();
    gc_thread.join().unwrap();

    let total_goroutines_created = goroutine_counter.load(Ordering::SeqCst);
    
    // Verify system is in a clean state
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should remain registered");
    
    info!(
        total_goroutines_created = total_goroutines_created,
        "Race condition test passed"
    );
}

/// Test conservative stack scanning functionality
#[test]
fn test_conservative_stack_scanning() {
    init_tracing!();
    let _timer = Timer::new("conservative_stack_scanning");

    let gc = Arc::new(GarbageCollector::new());
    let goroutine_gc = GoroutineGarbageCollector::new(gc.clone());

    // Create a test object that we'll put on the stack
    let obj = TestObject {
        id: 12345,
        value: 99,
        references: vec![],
    };
    let gc_obj = gc.allocate(obj);
    let obj_ptr = gc_obj.ptr();

    // Register a goroutine
    goroutine_gc.register_goroutine(1, 0x1000, 0x2000);

    // Simulate the object pointer being on the stack by adding it as a local root
    goroutine_gc.add_goroutine_root(1, obj_ptr);

    // Test goroutine-aware collection
    match goroutine_gc.collect_garbage_goroutine_aware() {
        Ok(stats) => {
            assert!(stats.stack_roots_found > 0, "Stack scanning should find roots");
            assert_eq!(stats.total_goroutines, 1, "Should scan one goroutine");
            
            info!(
                stack_roots = stats.stack_roots_found,
                "Conservative stack scanning test passed"
            );
        },
        Err(e) => {
            panic!("Stack scanning test failed: {}", e);
        }
    }

    // Object should still be alive due to stack root
    assert!(gc.is_alive(obj_ptr), "Object should be kept alive by stack root");

    // Clean up
    goroutine_gc.remove_goroutine_root(1, obj_ptr);
    goroutine_gc.unregister_goroutine(1);
}

/// Test incremental collection with goroutines
#[test]
fn test_incremental_collection_with_goroutines() {
    init_tracing!();
    let _timer = Timer::new("incremental_collection_with_goroutines");

    let gc = Arc::new(GarbageCollector::new());
    
    // Configure for incremental collection
    let mut config = cursed::memory::GoroutineGcConfig::default();
    config.incremental_enabled = true;
    config.max_goroutines_per_step = 2;
    
    let goroutine_gc = GoroutineGarbageCollector::with_config(gc.clone(), config);

    // Register multiple goroutines
    for i in 1..=5 {
        goroutine_gc.register_goroutine(i, (0x1000 * i) as usize, 0x1000);
        
        // Add some objects for each goroutine
        for j in 0..3 {
            let obj = TestObject {
                id: (i * 100 + j) as usize,
                value: j as i32,
                references: vec![],
            };
            let gc_obj = gc.allocate(obj);
            let obj_ptr = gc_obj.ptr();
            goroutine_gc.add_goroutine_root(i, obj_ptr);
        }
    }

    // Test that incremental collection works
    match goroutine_gc.collect_garbage_goroutine_aware() {
        Ok(stats) => {
            assert_eq!(stats.total_goroutines, 5, "Should process all goroutines");
            assert!(stats.stack_roots_found > 0, "Should find stack roots");
            
            info!(
                goroutines = stats.total_goroutines,
                stack_roots = stats.stack_roots_found,
                "Incremental collection test passed"
            );
        },
        Err(e) => {
            panic!("Incremental collection test failed: {}", e);
        }
    }

    // Clean up
    for i in 1..=5 {
        goroutine_gc.unregister_goroutine(i);
    }
}
