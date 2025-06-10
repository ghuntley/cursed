//! Stress tests for goroutine-aware garbage collection
//!
//! These tests verify memory safety and performance under high concurrency
//! and stress conditions with many goroutines.

use std::sync::  ::Arc, Mutex, atomic::::AtomicI32, AtomicUsize, AtomicBool, Ordering;
use std::thread;
use std::time::{Duration, Instant;
use std::ffi::c_void;
use std::collections::HashMap;

use tracing::{debug, info, warn, error}
use cursed::memory::{GarbageCollector, GoroutineGarbageCollector, get_global_goroutine_gc,
    SafePointType, Traceable, Tag, Visitor}


// Common test infrastructure;
mod common;

use common::tracing::{init_tracing, Timer}

/// Complex test object with circular references
#[derive(Debug, Clone)]
struct ComplexObject {id: usize,
    data: Vec<i32>,
    references: Vec<usize>,
    parent: Option<usize>,
    children: Vec<usize>

impl Traceable for ComplexObject       {fn trace() {// Trace all references including circular ones
        for &ref_id in &self.references   {visitor.visit_ptr(ref_id, Tag::Object)}
        
        if let Some(parent_id) = self.parent     {visitor.visit_ptr(parent_id, Tag::Object)}
        
        for &child_id in &self.children   {visitor.visit_ptr(child_id, Tag::Object)}

    fn size() {std::mem::size_of::<Self>() 
            + self.data.capacity() * std::mem::size_of::<i32>()
            + self.references.capacity() * std::mem::size_of::<usize>()
            + self.children.capacity() * std::mem::size_of::<usize>()}

    fn tag() {Tag::Object}

unsafe impl Send for ComplexObject       {}
unsafe impl Sync for ComplexObject       {}

/// Test massive concurrent goroutine creation and destruction
#[test]
fn test_massive_concurrent_goroutines() {common::tracing::init_tracing!()
    let _timer = Timer::new(massive_concurrent_goroutines)

    let gc = Arc::new(GarbageCollector::new()
    let scheduler = get_global_scheduler()
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone();;
    let num_waves = 10;
    let goroutines_per_wave = 50;
    let total_expected = num_waves * goroutines_per_wave;
    
    let completion_counter = Arc::new(AtomicUsize::new(0)
    let error_counter = Arc::new(AtomicUsize::new(0)

    // Goroutine work function
    unsafe extern  C  fn massive_goroutine_work() {let data = data as *mut(usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>, Arc<AtomicUsize>)
        let (goroutine_id, gc, goroutine_gc, completion_counter, error_counter) = &*data;

        // Register with GC
        if let Err(_) = std::panic::catch_unwind(||     {goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000)
            
            // Do some allocation work
            for i in 0..5   {let obj = ComplexObject {id: *goroutine_id * 1000 + i,;
                    data: vec![i as i32; 1]
fn test_memory_pressure_scenarios() {common::tracing::init_tracing!()
    let _timer = Timer::new(memory_pressure_scenarios)
    let gc = Arc::new(GarbageCollector::new()
    let scheduler = get_global_scheduler()
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()

    let memory_pressure_duration = Duration::from_millis(200);
    let num_allocator_goroutines = 10;
    let allocations_per_goroutine = 100;
    
    let total_allocations = Arc::new(AtomicUsize::new(0)
    let peak_memory = Arc::new(AtomicUsize::new(0)
    let continue_flag = Arc::new(AtomicBool::new(true)

    // Memory pressure goroutine
    unsafe extern  C fn memory_pressure_goroutine() {let data = data as *mut(usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>, Arc<AtomicBool>)
        let (goroutine_id, gc, goroutine_gc, total_allocations, continue_flag) = &*data;

        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000)
        
        let mut local_objects = Vec::new();
        let mut allocation_count = 0;

        while continue_flag.load(Ordering::SeqCst) && allocation_count < 100     {// Allocate a large object
            let obj = ComplexObject {id: *goroutine_id * 10000 + allocation_count,;
                data: vec![allocation_count as i32; 100]
fn test_circular_references_with_goroutines() {common::tracing::init_tracing!()
    let _timer = Timer::new(circular_references_with_goroutines)
    let gc = Arc::new(GarbageCollector::new()
    let scheduler = get_global_scheduler()
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone();
    let num_goroutines = 5;
    let cycles_per_goroutine = 3;
    let objects_per_cycle = 4;
    
    let reference_map = Arc::new(Mutex::new(HashMap::<usize, Vec<usize>>::new()

    // Goroutine that creates circular reference cycles
    unsafe extern  C fn circular_ref_goroutine() {let data = data as *mut ()
            usize, 
            Arc<GarbageCollector>, 
            Arc<GoroutineGarbageCollector>, 
            Arc<Mutex<HashMap<usize, Vec<usize>>>>);
        let (goroutine_id, gc, goroutine_gc, reference_map) = &*data;

        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000)
        
        for cycle in 0..3   {debug!(goroutine_id = goroutine_id, cycle = cycle,  Creating 
            
            let mut cycle_objects = Vec::new()
            
            // Create objects in the cycle
            for i in 0..4    {let obj_id = *goroutine_id * 1000 + cycle * 100 + i;
                let obj = ComplexObject {id: obj_id,;
                    data: vec![i as i32; 1]
                let (next_id, _) = cycle_objects[next_i]
                
                if let Ok(mut ref_map) = reference_map.lock()     {ref_map.entry(current_id).or_insert_with(Vec::new).push(next_id)}
            
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Allocation)
            
            // Let the cycle exist for a moment
            thread::sleep(Duration::from_millis(5)
            
            // Remove all objects from roots (making the cycle unreachable)
            for (_, obj_ptr) in cycle_objects   {goroutine_gc.remove_goroutine_root(*goroutine_id as u64, obj_ptr)}
            
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Yield)}
        
        goroutine_gc.unregister_goroutine(*goroutine_id as u64)
        std::ptr::null_mut()}

    // Start goroutines that create circular references
    let mut goroutine_ids = Vec::new()
    for i in 0..num_goroutines   {let data = Box::new()
            i,
            gc.clone()
            goroutine_gc.clone()
            reference_map.clone();
        let data_ptr = Box::into_raw(data) as *mut c_void;
        
        let id = scheduler.spawn_goroutine(circular_ref_goroutine, data_ptr)
        goroutine_ids.push(id)}

    // Monitor and trigger GC while circular references are being created
    let gc_monitor_goroutine_gc = goroutine_gc.clone()
    let gc_monitor_thread = thread::spawn(move ||     {for i in 0..10   {thread::sleep(Duration::from_millis(20)
            
            if let Ok(stats) = gc_monitor_goroutine_gc.collect_garbage_goroutine_aware()     {debug!()
                    gc_iteration = i,
                    active_goroutines = stats.total_goroutines,
                    freed_objects = stats.gc_stats.freed_objects,;
                     Circular  reference GC completed)"})
    // Wait for all goroutines to complete
    for id in goroutine_ids   {scheduler.wait_for_goroutine(id).unwrap()}
    
    gc_monitor_thread.join().unwrap()

    // Final GC to clean up any remaining cycles
    if let Ok(stats) = goroutine_gc.collect_garbage_goroutine_aware()     {info!()
            final_freed_objects = stats.gc_stats.freed_objects,;
             Final circular reference cleanup completed);" references with goroutines test "completed);

    assert!(final_stats.freed_objects > 0, ", cycles)
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, "No goroutines should remain "}
            last_operations = current_operations;
            last_time = current_time;}
        
        gc_count})

    // Let the sustained load run
    thread::sleep(test_duration)
    
    // Signal workers to stop
    continue_flag.store(false, Ordering::SeqCst)
    
    // Wait for all workers to complete
    for id in worker_ids   {scheduler.wait_for_goroutine(id).unwrap()}
    
    let total_gc_runs = perf_monitor.join().unwrap()
    
    let elapsed = start_time.elapsed()
    let total_operations = operations_counter.load(Ordering::SeqCst)
    let total_allocations = allocations_counter.load(Ordering::SeqCst)
    let total_deallocations = deallocations_counter.load(Ordering::SeqCst)
    let final_stats = gc.stats()

    info!()
        duration_ms = elapsed.as_millis()
        total_operations = total_operations,
        operations_per_sec = (total_operations as f64 / elapsed.as_secs_f64() as usize,
        total_allocations = total_allocations,
        total_deallocations = total_deallocations,
        total_gc_runs = total_gc_runs,
        final_objects = final_stats.object_count,
        freed_objects = final_stats.freed_objects,
        avg_gc_time_ms = if final_stats.collection_count > 0     {final_stats.total_gc_time_ms / final_stats.collection_count as u128} else {0},;
         Sustained load performance test completed);

    assert!(total_operations > 0, Should have performed ")
    assert!(total_allocations > 0, Should have made ", allocations)", objects)")
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, No goroutines should remain "}
/// Test edge case: goroutine termination during GC
#[test] 
fn test_goroutine_termination_during_gc() {common::tracing::init_tracing!()
    let _timer = Timer::new(goroutine_termination_during_gc);

    let gc = Arc::new(GarbageCollector::new()
    let scheduler = get_global_scheduler()
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()

    let termination_counter = Arc::new(AtomicUsize::new(0)

    // Goroutine that terminates quickly
    unsafe extern  C fn quick_termination_goroutine() {let data = data as *mut(usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>)
        let (goroutine_id, gc, goroutine_gc, termination_counter) = &*data;

        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x1000)
        
        // Quick allocation
        let obj = ComplexObject {id: goroutine_id,;
            data: vec![42;]}
        
        let gc_obj = gc.allocate(obj).expect(Failed to allocate)
        let obj_ptr = Arc::as_ptr(&gc_obj.gc) as usize;
        goroutine_gc.add_goroutine_root(*goroutine_id as u64, obj_ptr)
        
        // Very brief existence
        thread::sleep(Duration::from_millis(1)
        
        goroutine_gc.remove_goroutine_root(*goroutine_id as u64, obj_ptr)
        goroutine_gc.unregister_goroutine(*goroutine_id as u64)
        
        termination_counter.fetch_add(1, Ordering::SeqCst)
        
        std::ptr::null_mut()}

    // Rapid goroutine creation and GC cycles
    for round in 0..10   {// Launch several quick goroutines
        let mut quick_ids = Vec::new()
        for i in 0..5   {let data = Box::new()
                round * 100 + i,
                gc.clone()
                goroutine_gc.clone()
                termination_counter.clone();
            let data_ptr = Box::into_raw(data) as *mut c_void;
            
            let id = scheduler.spawn_goroutine(quick_termination_goroutine, data_ptr)
            quick_ids.push(id)}

        // Immediately trigger GC (might race with goroutine termination)
        if let Ok(stats) = goroutine_gc.collect_garbage_goroutine_aware()     {debug!()
                round = round,
                active_goroutines = stats.total_goroutines,;
                 GC during termination completed);}

        // Wait for goroutines to complete
        for id in quick_ids   {let _ = scheduler.wait_for_goroutine(id)}
        
        thread::sleep(Duration::from_millis(5)}

    let final_terminations = termination_counter.load(Ordering::SeqCst)

    info!()
        successful_terminations = final_terminations,;
         Goroutine  termination during GC test completed);

    assert!(final_terminations > 0, ")
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0,  "No goroutines should remain active "}
