//! Stress tests for goroutine-aware garbage collection
//!
//! These tests verify memory safety and performance under high concurrency
//! and stress conditions with many goroutines.

use std::sync::  ::Arc, Mutex, atomic::::AtomicI32, AtomicUsize, AtomicBool, Ordering;
use std::thread;
use std::time::{Duration, Instant;}
use std::ffi::c_void;
use std::collections::HashMap;

use tracing::{debug, info, warn, error}
use cursed::memory::{GarbageCollector, GoroutineGarbageCollector, get_global_goroutine_gc,}
    SafePointType, Traceable, Tag, Visitor}


// Common test infrastructure;
mod common;

use common::tracing::{init_tracing, Timer}

/// Complex test object with circular references
#[derive(Debug, Clone)]
struct ComplexObject {id: usize,}
    data: Vec<i32>,
    references: Vec<usize>,
    parent: Option<usize>,
    children: Vec<usize>

impl Traceable for ComplexObject       {fn trace(} {// Trace all references including circular ones)}
        for &ref_id in &self.references   {visitor.visit_ptr(ref_id, Tag::Object}})
        
        if let Some(parent_id) = self.parent     {visitor.visit_ptr(parent_id, Tag::Object}})
        
        for &child_id in &self.children   {visitor.visit_ptr(child_id, Tag::Object}})

    fn size() {std::mem::size_of::<Self>(} )
            + self.data.capacity() * std::mem::size_of::<i32>()
            + self.references.capacity() * std::mem::size_of::<usize>()
            + self.children.capacity() * std::mem::size_of::<usize>()}

    fn tag() {Tag::Object}

unsafe impl Send for ComplexObject       {}
unsafe impl Sync for ComplexObject       {}

/// Test massive concurrent goroutine creation and destruction
#[test]
fn test_massive_concurrent_goroutines() {common::tracing::init_tracing!(})
    let _timer = Timer::new(massive_concurrent_goroutines);
    let gc = Arc::new(GarbageCollector::new();)
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone();;))
    let num_waves = 10;
    let goroutines_per_wave = 50;
    let total_expected = num_waves * goroutines_per_wave;
    
    let completion_counter = Arc::new(AtomicUsize::new(0);)
    let error_counter = Arc::new(AtomicUsize::new(0);)
    // Goroutine work function
    unsafe extern  C  fn massive_goroutine_work() {let data = data as *mut(usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>, Arc<AtomicUsize>})
        let (goroutine_id, gc, goroutine_gc, completion_counter, error_counter) = &*data;

        // Register with GC
        if let Err(_) = std::panic::catch_unwind(||     {goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000}))
            
            // Do some allocation work
            for i in 0..5   {let obj = ComplexObject {id: *goroutine_id * 1000 + i,;}}
                    data: vec![i as i32; 1]
fn test_memory_pressure_scenarios(} {common::tracing::init_tracing!(}))
    let _timer = Timer::new(memory_pressure_scenarios);
    let gc = Arc::new(GarbageCollector::new();)
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone();))
    let memory_pressure_duration = Duration::from_millis(200);
    let num_allocator_goroutines = 10;
    let allocations_per_goroutine = 100;
    
    let total_allocations = Arc::new(AtomicUsize::new(0);)
    let peak_memory = Arc::new(AtomicUsize::new(0);)
    let continue_flag = Arc::new(AtomicBool::new(true);)
    // Memory pressure goroutine
    unsafe extern  C fn memory_pressure_goroutine() {let data = data as *mut(usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicUsize>, Arc<AtomicBool>})
        let (goroutine_id, gc, goroutine_gc, total_allocations, continue_flag) = &*data;

        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000);
        let mut local_objects = Vec::new();
        let mut allocation_count = 0;

        while continue_flag.load(Ordering::SeqCst) && allocation_count < 100     {// Allocate a large object}
            let obj = ComplexObject {id: *goroutine_id * 10000 + allocation_count,;}
                data: vec![allocation_count as i32; 100]
fn test_circular_references_with_goroutines(} {common::tracing::init_tracing!(}))
    let _timer = Timer::new(circular_references_with_goroutines);
    let gc = Arc::new(GarbageCollector::new();)
    let scheduler = get_global_scheduler();
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone();))
    let num_goroutines = 5;
    let cycles_per_goroutine = 3;
    let objects_per_cycle = 4;
    
    let reference_map = Arc::new(Mutex::new(HashMap::<usize, Vec<usize>>::new();))
    // Goroutine that creates circular reference cycles
    unsafe extern  C fn circular_ref_goroutine() {let data = data as *mut (})
            usize, 
            Arc<GarbageCollector>, 
            Arc<GoroutineGarbageCollector>, 
            Arc<Mutex<HashMap<usize, Vec<usize>>>>);
        let (goroutine_id, gc, goroutine_gc, reference_map) = &*data;

        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000);
        for cycle in 0..3   {debug!(goroutine_id = goroutine_id, cycle = cycle,  Creating )}
            
            let mut cycle_objects = Vec::new(})
            
            // Create objects in the cycle
            for i in 0..4    {let obj_id = *goroutine_id * 1000 + cycle * 100 + i;}
                let obj = ComplexObject {id: obj_id,;}
                    data: vec![i as i32; 1]
                let (next_id, _} = cycle_objects[next_i])
                
                if let Ok(mut ref_map) = reference_map.lock()     {ref_map.entry(current_id}.or_insert_with(Vec::new).push(next_id)})
            
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Allocation);
            // Let the cycle exist for a moment
            thread::sleep(Duration::from_millis(5);)
            // Remove all objects from roots (making the cycle unreachable)
            for (_, obj_ptr) in cycle_objects   {goroutine_gc.remove_goroutine_root(*goroutine_id as u64, obj_ptr}})
            
            goroutine_gc.goroutine_safe_point(*goroutine_id as u64, SafePointType::Yield)}
        
        goroutine_gc.unregister_goroutine(*goroutine_id as u64);
        std::ptr::null_mut()}

    // Start goroutines that create circular references
    let mut goroutine_ids = Vec::new();
    for i in 0..num_goroutines   {let data = Box::new(})
            i,
            gc.clone();
            goroutine_gc.clone();
            reference_map.clone();
        let data_ptr = Box::into_raw(data) as *mut c_void;
        
        let id = scheduler.spawn_goroutine(circular_ref_goroutine, data_ptr);
        goroutine_ids.push(id)}

    // Monitor and trigger GC while circular references are being created
    let gc_monitor_goroutine_gc = goroutine_gc.clone();
    let gc_monitor_thread = thread::spawn(move ||     {for i in 0..10   {thread::sleep(Duration::from_millis(20})))}
            
            if let Ok(stats) = gc_monitor_goroutine_gc.collect_garbage_goroutine_aware()     {debug!(})
                    gc_iteration = i,
                    active_goroutines = stats.total_goroutines,
                    freed_objects = stats.gc_stats.freed_objects,;
                     Circular  reference GC completed)"})
             Final circular reference cleanup completed);" references with goroutines test ", fixed
    assert!(final_stats.freed_objects > 0, ", cycles)"
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, , " goroutines should remain ")
    assert!(total_allocations > 0, Should have made , allocations)", objects)"
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0, No goroutines should remain "}")
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0,  , " goroutines should remain active "fixed")