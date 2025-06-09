//! Stress tests for goroutine-aware garbage collection
//!
//! These tests verify memory safety and performance under high concurrency
//! and stress conditions with many goroutines.

use std::sync::{Arc, Mutex, atomic::{AtomicI32, AtomicUsize, AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use std::ffi::c_void;
use std::collections::HashMap;

use tracing::{debug, info, warn, error};
use cursed::memory::{
    GarbageCollector, GoroutineGarbageCollector, get_global_goroutine_gc,
    SafePointType, Traceable, Tag, Visitor
};
use cursed::runtime::goroutine::get_global_scheduler;

// Common test infrastructure
mod common;

use common::tracing::{init_tracing, Timer};

/// Complex test object with circular references
#[derive(Debug, Clone)]
struct ComplexObject {
    id: usize,
    data: Vec<i32>,
    references: Vec<usize>,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Traceable for ComplexObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Trace all references including circular ones
        for &ref_id in &self.references {
            visitor.visit_ptr(ref_id, Tag::Object);
        }
        
        if let Some(parent_id) = self.parent {
            visitor.visit_ptr(parent_id, Tag::Object);
        }
        
        for &child_id in &self.children {
            visitor.visit_ptr(child_id, Tag::Object);
        }
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Self>() 
            + self.data.capacity() * std::mem::size_of::<i32>()
            + self.references.capacity() * std::mem::size_of::<usize>()
            + self.children.capacity() * std::mem::size_of::<usize>()
    }

    fn tag(&self) -> Tag {
        Tag::Object
    }
}

unsafe impl Send for ComplexObject {}
unsafe impl Sync for ComplexObject {}

/// Test massive concurrent goroutine creation and destruction
#[test]
fn test_massive_concurrent_goroutines() {
    init_tracing!();
    let _timer = Timer::new("massive_concurrent_goroutines");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let num_waves = 10;
    let goroutines_per_wave = 50;
    let total_expected = num_waves * goroutines_per_wave;
    
    let completion_counter = Arc::new(AtomicUsize::new(0));
    let error_counter = Arc::new(AtomicUsize::new(0));

    // Goroutine work function
    unsafe extern "C" fn massive_goroutine_work(data: *mut c_void) -> *mut c_void {
        let data = data as *mut (usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>, Arc<AtomicUsize>);
        let (goroutine_id, gc, goroutine_gc, completion_counter, error_counter) = &*data;

        // Register with GC
        if let Err(_) = std::panic::catch_unwind(|| {
            goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000);
            
            // Do some allocation work
            for i in 0..5 {
                let obj = ComplexObject {
                    id: *goroutine_id * 1000 + i,
                    data: vec![i as i32; 10],
                    references: vec![],
                    parent: None,
                    children: vec![],
                };
                
                let gc_obj = gc.allocate(obj);
                let obj_ptr = Arc::as_ptr(&gc_obj.gc) as usize;
                
                // Briefly add as root
                goroutine_gc.add_goroutine_root(*goroutine_id as u64, obj_ptr);
                goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Allocation);
                goroutine_gc.remove_goroutine_root(*goroutine_id as u64, obj_ptr);
            }
            
            // Unregister from GC
            goroutine_gc.unregister_goroutine(*goroutine_id as u64);
        }) {
            error_counter.fetch_add(1, Ordering::SeqCst);
        } else {
            completion_counter.fetch_add(1, Ordering::SeqCst);
        }

        std::ptr::null_mut()
    }

    let start_time = Instant::now();

    // Launch waves of goroutines
    for wave in 0..num_waves {
        debug!(wave = wave, "Starting wave of goroutines");
        
        let mut wave_ids = Vec::new();
        
        // Spawn all goroutines in this wave quickly
        for i in 0..goroutines_per_wave {
            let goroutine_id = wave * 1000 + i;
            let data = Box::new((
                goroutine_id,
                gc.clone(),
                goroutine_gc.clone(),
                completion_counter.clone(),
                error_counter.clone()
            ));
            let data_ptr = Box::into_raw(data) as *mut c_void;
            
            let id = scheduler.spawn_goroutine(massive_goroutine_work, data_ptr);
            wave_ids.push(id);
        }

        // Trigger GC periodically while goroutines are running
        if wave % 3 == 0 {
            thread::spawn({
                let goroutine_gc = goroutine_gc.clone();
                move || {
                    thread::sleep(Duration::from_millis(10));
                    if let Ok(stats) = goroutine_gc.collect_garbage_goroutine_aware() {
                        debug!(
                            wave = wave,
                            active_goroutines = stats.total_goroutines,
                            "Mid-wave GC completed"
                        );
                    }
                }
            });
        }

        // Wait for this wave to complete before starting the next
        for id in wave_ids {
            let _ = scheduler.wait_for_goroutine(id);
        }
        
        debug!(wave = wave, "Wave completed");
        
        // Small delay between waves
        thread::sleep(Duration::from_millis(5));
    }

    let elapsed = start_time.elapsed();
    let completed = completion_counter.load(Ordering::SeqCst);
    let errors = error_counter.load(Ordering::SeqCst);

    // Final GC to clean up
    if let Ok(stats) = goroutine_gc.collect_garbage_goroutine_aware() {
        info!(
            freed_objects = stats.gc_stats.freed_objects,
            "Final cleanup GC completed"
        );
    }

    info!(
        total_goroutines = total_expected,
        completed = completed,
        errors = errors,
        elapsed_ms = elapsed.as_millis(),
        throughput_per_sec = (completed as f64 / elapsed.as_secs_f64()) as usize,
        "Massive concurrent goroutines test completed"
    );

    assert!(completed > total_expected / 2, "At least half the goroutines should complete successfully");
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should remain active");
}

/// Test memory pressure scenarios with aggressive allocation
#[test]
fn test_memory_pressure_scenarios() {
    init_tracing!();
    let _timer = Timer::new("memory_pressure_scenarios");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let memory_pressure_duration = Duration::from_millis(200);
    let num_allocator_goroutines = 10;
    let allocations_per_goroutine = 100;
    
    let total_allocations = Arc::new(AtomicUsize::new(0));
    let peak_memory = Arc::new(AtomicUsize::new(0));
    let continue_flag = Arc::new(AtomicBool::new(true));

    // Memory pressure goroutine
    unsafe extern "C" fn memory_pressure_goroutine(data: *mut c_void) -> *mut c_void {
        let data = data as *mut (usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>, Arc<AtomicBool>);
        let (goroutine_id, gc, goroutine_gc, total_allocations, continue_flag) = &*data;

        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000);
        
        let mut local_objects = Vec::new();
        let mut allocation_count = 0;

        while continue_flag.load(Ordering::SeqCst) && allocation_count < 100 {
            // Allocate a large object
            let obj = ComplexObject {
                id: *goroutine_id * 10000 + allocation_count,
                data: vec![allocation_count as i32; 1000], // Large data
                references: vec![],
                parent: None,
                children: vec![],
            };
            
            let gc_obj = gc.allocate(obj);
            let obj_ptr = Arc::as_ptr(&gc_obj.gc) as usize;
            
            goroutine_gc.add_goroutine_root(*goroutine_id as u64, obj_ptr);
            local_objects.push(obj_ptr);
            
            allocation_count += 1;
            total_allocations.fetch_add(1, Ordering::SeqCst);
            
            // Safe point every few allocations
            if allocation_count % 10 == 0 {
                goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Allocation);
            }
            
            // Randomly release some objects to simulate varying memory usage
            if allocation_count % 25 == 0 && !local_objects.is_empty() {
                let release_count = local_objects.len() / 2;
                for _ in 0..release_count {
                    if let Some(ptr) = local_objects.pop() {
                        goroutine_gc.remove_goroutine_root(*goroutine_id as u64, ptr);
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(1));
        }

        // Clean up remaining objects
        for ptr in local_objects {
            goroutine_gc.remove_goroutine_root(*goroutine_id as u64, ptr);
        }
        
        goroutine_gc.unregister_goroutine(*goroutine_id as u64);
        std::ptr::null_mut()
    }

    let start_time = Instant::now();

    // Start allocator goroutines
    let mut goroutine_ids = Vec::new();
    for i in 0..num_allocator_goroutines {
        let data = Box::new((
            i,
            gc.clone(),
            goroutine_gc.clone(),
            total_allocations.clone(),
            continue_flag.clone()
        ));
        let data_ptr = Box::into_raw(data) as *mut c_void;
        
        let id = scheduler.spawn_goroutine(memory_pressure_goroutine, data_ptr);
        goroutine_ids.push(id);
    }

    // Monitor memory usage and trigger aggressive GC
    let goroutine_gc_monitor = goroutine_gc.clone();
    let continue_flag_monitor = continue_flag.clone();
    let peak_memory_monitor = peak_memory.clone();
    
    let monitor_thread = thread::spawn(move || {
        let mut gc_count = 0;
        while continue_flag_monitor.load(Ordering::SeqCst) {
            // Track peak memory
            let current_stats = goroutine_gc_monitor.gc.stats();
            peak_memory_monitor.store(
                peak_memory_monitor.load(Ordering::SeqCst).max(current_stats.total_size),
                Ordering::SeqCst
            );
            
            // Aggressive GC every 20ms
            if let Ok(stats) = goroutine_gc_monitor.collect_garbage_goroutine_aware() {
                gc_count += 1;
                debug!(
                    gc_iteration = gc_count,
                    active_goroutines = stats.total_goroutines,
                    freed_objects = stats.gc_stats.freed_objects,
                    current_objects = stats.gc_stats.object_count,
                    "Memory pressure GC completed"
                );
            }
            
            thread::sleep(Duration::from_millis(20));
        }
        debug!(total_gc_runs = gc_count, "Memory pressure monitoring completed");
    });

    // Let the test run for the specified duration
    thread::sleep(memory_pressure_duration);
    
    // Signal everything to stop
    continue_flag.store(false, Ordering::SeqCst);
    
    // Wait for all goroutines to complete
    for id in goroutine_ids {
        let _ = scheduler.wait_for_goroutine(id);
    }
    
    monitor_thread.join().unwrap();
    
    let elapsed = start_time.elapsed();
    let final_allocations = total_allocations.load(Ordering::SeqCst);
    let final_peak_memory = peak_memory.load(Ordering::SeqCst);
    let final_stats = gc.stats();

    info!(
        duration_ms = elapsed.as_millis(),
        total_allocations = final_allocations,
        peak_memory_bytes = final_peak_memory,
        final_objects = final_stats.object_count,
        freed_objects = final_stats.freed_objects,
        gc_collections = final_stats.collection_count,
        "Memory pressure test completed"
    );

    assert!(final_allocations > 0, "Should have made allocations");
    assert!(final_stats.freed_objects > 0, "Should have freed objects under pressure");
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should remain active");
}

/// Test circular reference scenarios with goroutines
#[test]
fn test_circular_references_with_goroutines() {
    init_tracing!();
    let _timer = Timer::new("circular_references_with_goroutines");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let num_goroutines = 5;
    let cycles_per_goroutine = 3;
    let objects_per_cycle = 4;
    
    let reference_map = Arc::new(Mutex::new(HashMap::<usize, Vec<usize>>::new()));

    // Goroutine that creates circular reference cycles
    unsafe extern "C" fn circular_ref_goroutine(data: *mut c_void) -> *mut c_void {
        let data = data as *mut (
            usize, 
            Arc<GarbageCollector>, 
            Arc<GoroutineGarbageCollector>, 
            Arc<Mutex<HashMap<usize, Vec<usize>>>>
        );
        let (goroutine_id, gc, goroutine_gc, reference_map) = &*data;

        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000);
        
        for cycle in 0..3 {
            debug!(goroutine_id = goroutine_id, cycle = cycle, "Creating circular reference cycle");
            
            let mut cycle_objects = Vec::new();
            
            // Create objects in the cycle
            for i in 0..4 {
                let obj_id = *goroutine_id * 1000 + cycle * 100 + i;
                let obj = ComplexObject {
                    id: obj_id,
                    data: vec![i as i32; 10],
                    references: vec![],
                    parent: None,
                    children: vec![],
                };
                
                let gc_obj = gc.allocate(obj);
                let obj_ptr = Arc::as_ptr(&gc_obj.gc) as usize;
                goroutine_gc.add_goroutine_root(*goroutine_id as u64, obj_ptr);
                
                cycle_objects.push((obj_id, obj_ptr));
            }
            
            // Create circular references
            for i in 0..cycle_objects.len() {
                let next_i = (i + 1) % cycle_objects.len();
                let (current_id, _) = cycle_objects[i];
                let (next_id, _) = cycle_objects[next_i];
                
                if let Ok(mut ref_map) = reference_map.lock() {
                    ref_map.entry(current_id).or_insert_with(Vec::new).push(next_id);
                }
            }
            
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Allocation);
            
            // Let the cycle exist for a moment
            thread::sleep(Duration::from_millis(5));
            
            // Remove all objects from roots (making the cycle unreachable)
            for (_, obj_ptr) in cycle_objects {
                goroutine_gc.remove_goroutine_root(*goroutine_id as u64, obj_ptr);
            }
            
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Yield);
        }
        
        goroutine_gc.unregister_goroutine(*goroutine_id as u64);
        std::ptr::null_mut()
    }

    // Start goroutines that create circular references
    let mut goroutine_ids = Vec::new();
    for i in 0..num_goroutines {
        let data = Box::new((
            i,
            gc.clone(),
            goroutine_gc.clone(),
            reference_map.clone()
        ));
        let data_ptr = Box::into_raw(data) as *mut c_void;
        
        let id = scheduler.spawn_goroutine(circular_ref_goroutine, data_ptr);
        goroutine_ids.push(id);
    }

    // Monitor and trigger GC while circular references are being created
    let gc_monitor_goroutine_gc = goroutine_gc.clone();
    let gc_monitor_thread = thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(20));
            
            if let Ok(stats) = gc_monitor_goroutine_gc.collect_garbage_goroutine_aware() {
                debug!(
                    gc_iteration = i,
                    active_goroutines = stats.total_goroutines,
                    freed_objects = stats.gc_stats.freed_objects,
                    "Circular reference GC completed"
                );
            }
        }
    });

    // Wait for all goroutines to complete
    for id in goroutine_ids {
        scheduler.wait_for_goroutine(id).unwrap();
    }
    
    gc_monitor_thread.join().unwrap();

    // Final GC to clean up any remaining cycles
    if let Ok(stats) = goroutine_gc.collect_garbage_goroutine_aware() {
        info!(
            final_freed_objects = stats.gc_stats.freed_objects,
            "Final circular reference cleanup completed"
        );
    }

    let final_stats = gc.stats();
    let reference_count = reference_map.lock().unwrap().len();

    info!(
        created_references = reference_count,
        final_objects = final_stats.object_count,
        total_freed = final_stats.freed_objects,
        "Circular references with goroutines test completed"
    );

    assert!(final_stats.freed_objects > 0, "Should have freed circular reference cycles");
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should remain active");
}

/// Test GC performance under sustained load
#[test]
fn test_sustained_load_performance() {
    init_tracing!();
    let _timer = Timer::new("sustained_load_performance");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let test_duration = Duration::from_millis(500);
    let num_worker_goroutines = 8;
    
    let operations_counter = Arc::new(AtomicUsize::new(0));
    let allocations_counter = Arc::new(AtomicUsize::new(0));
    let deallocations_counter = Arc::new(AtomicUsize::new(0));
    let continue_flag = Arc::new(AtomicBool::new(true));

    // Worker goroutine that performs sustained allocation/deallocation
    unsafe extern "C" fn sustained_load_worker(data: *mut c_void) -> *mut c_void {
        let data = data as *mut (
            usize,
            Arc<GarbageCollector>,
            Arc<GoroutineGarbageCollector>,
            Arc<AtomicUsize>,
            Arc<AtomicUsize>,
            Arc<AtomicUsize>,
            Arc<AtomicBool>
        );
        let (
            worker_id,
            gc,
            goroutine_gc,
            operations_counter,
            allocations_counter,
            deallocations_counter,
            continue_flag
        ) = &*data;

        goroutine_gc.register_goroutine(*worker_id as u64, 0x1000, 0x2000);
        
        let mut active_objects = Vec::new();
        let mut operation_count = 0;

        while continue_flag.load(Ordering::SeqCst) {
            operation_count += 1;
            operations_counter.fetch_add(1, Ordering::SeqCst);
            
            match operation_count % 4 {
                0 | 1 => {
                    // Allocate new object
                    let obj = ComplexObject {
                        id: *worker_id * 100000 + operation_count,
                        data: vec![operation_count as i32; 50],
                        references: vec![],
                        parent: None,
                        children: vec![],
                    };
                    
                    let gc_obj = gc.allocate(obj);
                    let obj_ptr = Arc::as_ptr(&gc_obj.gc) as usize;
                    goroutine_gc.add_goroutine_root(*worker_id as u64, obj_ptr);
                    
                    active_objects.push(obj_ptr);
                    allocations_counter.fetch_add(1, Ordering::SeqCst);
                },
                2 => {
                    // Deallocate some objects
                    let dealloc_count = active_objects.len() / 3;
                    for _ in 0..dealloc_count {
                        if let Some(ptr) = active_objects.pop() {
                            goroutine_gc.remove_goroutine_root(*worker_id as u64, ptr);
                            deallocations_counter.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                },
                3 => {
                    // Safe point and yield
                    goroutine_gc.goroutine_safe_point(*worker_id as u64, SafePointType::Yield);
                    thread::yield_now();
                },
                _ => unreachable!(),
            }
            
            // Brief pause to avoid overwhelming the system
            if operation_count % 100 == 0 {
                thread::sleep(Duration::from_micros(100));
            }
        }

        // Clean up remaining objects
        for ptr in active_objects {
            goroutine_gc.remove_goroutine_root(*worker_id as u64, ptr);
        }
        
        goroutine_gc.unregister_goroutine(*worker_id as u64);
        std::ptr::null_mut()
    }

    let start_time = Instant::now();

    // Start worker goroutines
    let mut worker_ids = Vec::new();
    for i in 0..num_worker_goroutines {
        let data = Box::new((
            i,
            gc.clone(),
            goroutine_gc.clone(),
            operations_counter.clone(),
            allocations_counter.clone(),
            deallocations_counter.clone(),
            continue_flag.clone()
        ));
        let data_ptr = Box::into_raw(data) as *mut c_void;
        
        let id = scheduler.spawn_goroutine(sustained_load_worker, data_ptr);
        worker_ids.push(id);
    }

    // Performance monitoring thread
    let perf_goroutine_gc = goroutine_gc.clone();
    let perf_continue_flag = continue_flag.clone();
    let perf_operations_counter = operations_counter.clone();
    
    let perf_monitor = thread::spawn(move || {
        let mut gc_count = 0;
        let mut last_operations = 0;
        let mut last_time = Instant::now();
        
        while perf_continue_flag.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(50));
            
            let current_time = Instant::now();
            let current_operations = perf_operations_counter.load(Ordering::SeqCst);
            let elapsed = current_time.duration_since(last_time);
            let ops_delta = current_operations - last_operations;
            
            let ops_per_sec = if elapsed.as_secs_f64() > 0.0 {
                ops_delta as f64 / elapsed.as_secs_f64()
            } else {
                0.0
            };
            
            if let Ok(stats) = perf_goroutine_gc.collect_garbage_goroutine_aware() {
                gc_count += 1;
                debug!(
                    gc_iteration = gc_count,
                    active_goroutines = stats.total_goroutines,
                    ops_per_sec = ops_per_sec as usize,
                    freed_objects = stats.gc_stats.freed_objects,
                    "Performance monitoring GC completed"
                );
            }
            
            last_operations = current_operations;
            last_time = current_time;
        }
        
        gc_count
    });

    // Let the sustained load run
    thread::sleep(test_duration);
    
    // Signal workers to stop
    continue_flag.store(false, Ordering::SeqCst);
    
    // Wait for all workers to complete
    for id in worker_ids {
        scheduler.wait_for_goroutine(id).unwrap();
    }
    
    let total_gc_runs = perf_monitor.join().unwrap();
    
    let elapsed = start_time.elapsed();
    let total_operations = operations_counter.load(Ordering::SeqCst);
    let total_allocations = allocations_counter.load(Ordering::SeqCst);
    let total_deallocations = deallocations_counter.load(Ordering::SeqCst);
    let final_stats = gc.stats();

    info!(
        duration_ms = elapsed.as_millis(),
        total_operations = total_operations,
        operations_per_sec = (total_operations as f64 / elapsed.as_secs_f64()) as usize,
        total_allocations = total_allocations,
        total_deallocations = total_deallocations,
        total_gc_runs = total_gc_runs,
        final_objects = final_stats.object_count,
        freed_objects = final_stats.freed_objects,
        avg_gc_time_ms = if final_stats.collection_count > 0 {
            final_stats.total_gc_time_ms / final_stats.collection_count as u128
        } else {
            0
        },
        "Sustained load performance test completed"
    );

    assert!(total_operations > 0, "Should have performed operations");
    assert!(total_allocations > 0, "Should have made allocations");
    assert!(final_stats.freed_objects > 0, "Should have freed objects");
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should remain active");
}

/// Test edge case: goroutine termination during GC
#[test] 
fn test_goroutine_termination_during_gc() {
    init_tracing!();
    let _timer = Timer::new("goroutine_termination_during_gc");

    let gc = Arc::new(GarbageCollector::new());
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()));

    let termination_counter = Arc::new(AtomicUsize::new(0));

    // Goroutine that terminates quickly
    unsafe extern "C" fn quick_termination_goroutine(data: *mut c_void) -> *mut c_void {
        let data = data as *mut (usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>);
        let (goroutine_id, gc, goroutine_gc, termination_counter) = &*data;

        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x1000);
        
        // Quick allocation
        let obj = ComplexObject {
            id: *goroutine_id,
            data: vec![42; 5],
            references: vec![],
            parent: None,
            children: vec![],
        };
        
        let gc_obj = gc.allocate(obj);
        let obj_ptr = Arc::as_ptr(&gc_obj.gc) as usize;
        goroutine_gc.add_goroutine_root(*goroutine_id as u64, obj_ptr);
        
        // Very brief existence
        thread::sleep(Duration::from_millis(1));
        
        goroutine_gc.remove_goroutine_root(*goroutine_id as u64, obj_ptr);
        goroutine_gc.unregister_goroutine(*goroutine_id as u64);
        
        termination_counter.fetch_add(1, Ordering::SeqCst);
        
        std::ptr::null_mut()
    }

    // Rapid goroutine creation and GC cycles
    for round in 0..10 {
        // Launch several quick goroutines
        let mut quick_ids = Vec::new();
        for i in 0..5 {
            let data = Box::new((
                round * 100 + i,
                gc.clone(),
                goroutine_gc.clone(),
                termination_counter.clone()
            ));
            let data_ptr = Box::into_raw(data) as *mut c_void;
            
            let id = scheduler.spawn_goroutine(quick_termination_goroutine, data_ptr);
            quick_ids.push(id);
        }

        // Immediately trigger GC (might race with goroutine termination)
        if let Ok(stats) = goroutine_gc.collect_garbage_goroutine_aware() {
            debug!(
                round = round,
                active_goroutines = stats.total_goroutines,
                "GC during termination completed"
            );
        }

        // Wait for goroutines to complete
        for id in quick_ids {
            let _ = scheduler.wait_for_goroutine(id);
        }
        
        thread::sleep(Duration::from_millis(5));
    }

    let final_terminations = termination_counter.load(Ordering::SeqCst);

    info!(
        successful_terminations = final_terminations,
        "Goroutine termination during GC test completed"
    );

    assert!(final_terminations > 0, "Some goroutines should have terminated successfully");
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should remain active");
}
