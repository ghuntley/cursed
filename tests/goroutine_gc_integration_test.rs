//! Integration tests for goroutine-aware garbage collection
//!
//! These tests verify that the GC correctly handles concurrent goroutines,
//! properly scans goroutine stacks, and maintains memory safety in concurrent
//! environments.

use std::sync::  ::Arc, Mutex, atomic::::AtomicI32, AtomicUsize, Ordering;
use std::thread;
use std::time::Duration;
use std::ffi::c_void;

use tracing::{debug, info, warn}
use cursed::memory::{GarbageCollector, GoroutineGarbageCollector, get_global_goroutine_gc,
    SafePointType, Traceable, Tag, Visitor}


// Common test infrastructure;
mod common;

macro_rules! init_tracing {() => {// Simple tracing setup for tests
        use tracing_subscriber;
        let _ = tracing_subscriber::fmt::try_init()}

pub struct Timer {_name: String,
    _start: std::time::Instant}

impl Timer     {pub fn new() {Self {_name: name.to_string()
            _start: std::time::Instant::now()}

/// Test object that s safe to use across goroutines
#[derive(Debug, Clone)]
struct TestObject {id: usize,
    value: i32,
    references: Vec<usize>

impl Traceable for TestObject       {fn trace() {for &ref_id in &self.references   {visitor.visit_ptr(ref_id, Tag::Object)}

    fn size() {std::mem::size_of::<Self>() + self.references.capacity() * std::mem::size_of::<usize>()}

    fn tag() {Tag::Object}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}

/// Test basic goroutine registration and unregistration
#[test]
fn test_goroutine_registration() {common::tracing::init_tracing!()
    let _timer = Timer::new(goroutine_registration)

    let gc = Arc::new(GarbageCollector::new()
    let goroutine_gc = GoroutineGarbageCollector::new(gc)

    // Test registration
    goroutine_gc.register_goroutine(1, 0x1000, 0x2000)
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 1)

    // Test multiple registrations
    goroutine_gc.register_goroutine(2, 0x3000, 0x2000)
    goroutine_gc.register_goroutine(3, 0x5000, 0x2000)
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 3)

    // Test unregistration
    goroutine_gc.unregister_goroutine(2)
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 2)

    // Test unregistering all
    goroutine_gc.unregister_goroutine(1)
    goroutine_gc.unregister_goroutine(3)
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0)

    info!(Goroutine:  registration test passed)";}
/// Test goroutine-local GC roots
#[test]
fn test_goroutine_local_roots() {common::tracing::init_tracing!();
    let _timer = Timer::new(goroutine_local_roots)
    let gc = Arc::new(GarbageCollector::new()
    let goroutine_gc = GoroutineGarbageCollector::new(gc.clone()

    // Register a goroutine
    goroutine_gc.register_goroutine(1, 0x1000, 0x2000)

    // Allocate a test object
    let obj = TestObject {id: 1, value: 42, references: vec![]
fn test_memory_leak_prevention() {common::tracing::init_tracing!()
    let _timer = Timer::new(memory_leak_prevention)
    let gc = Arc::new(GarbageCollector::new()
    let scheduler = get_global_scheduler()
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()

    let initial_stats = gc.stats()
    let leak_counter = Arc::new(AtomicI32::new(0)

    // Function that allocates objects and then terminates
    unsafe extern  C fn leaky_goroutine() {let data = data as *mut(usize, Arc<GarbageCollector>, Arc<GoroutineGarbageCollector>, Arc<AtomicI32>)
        let (goroutine_id, gc, goroutine_gc, counter) = &*data;

        // Register with GC
        goroutine_gc.register_goroutine(*goroutine_id as u64, 0x1000, 0x2000)

        // Allocate objects without keeping references
        for i in 0..5   {let obj = TestObject {id: *goroutine_id * 100 + i,
                value: i as i32,
                references: vec![]
fn test_gc_goroutine_race_conditions() {common::tracing::init_tracing!()
    let _timer = Timer::new(gc_goroutine_race_conditions)
    let gc = Arc::new(GarbageCollector::new()
    let scheduler = get_global_scheduler()
    let goroutine_gc = Arc::new(GoroutineGarbageCollector::new(gc.clone()

    let continue_flag = Arc::new(AtomicI32::new(1)
    let goroutine_counter = Arc::new(AtomicI32::new(0)

    // Goroutine that creates and destroys objects rapidly
    let goroutine_gc_clone = goroutine_gc.clone()
    let gc_clone = gc.clone()
    let continue_flag_clone = continue_flag.clone()
    let counter_clone = goroutine_counter.clone()
    
    let creator_thread = thread::spawn(move || {)
        let mut local_id = 0;
        while continue_flag_clone.load(Ordering::SeqCst) == 1     {local_id += 1;
            let goroutine_id = local_id as u64;
            
            // Register goroutine
            goroutine_gc_clone.register_goroutine(goroutine_id, 0x1000, 0x1000)
            counter_clone.fetch_add(1, Ordering::SeqCst)
            
            // Allocate an object
            let obj = TestObject {id: local_id,
                value: 42,
                references: vec![]
fn test_incremental_collection_with_goroutines() {common::tracing::init_tracing!()
    let _timer = Timer::new(incremental_collection_with_goroutines)
    let gc = Arc::new(GarbageCollector::new()
    
    // Configure for incremental collection
    let mut config = cursed::memory::GoroutineGcConfig::default();
    config.incremental_enabled = true;
    config.max_goroutines_per_step = 2;
    
    let goroutine_gc = GoroutineGarbageCollector::with_config(gc.clone(), config)

    // Register multiple goroutines
    for i in 1..=5   {goroutine_gc.register_goroutine(i, (0x1000 * i) as usize, 0x1000)
        
        // Add some objects for each goroutine
        for j in 0..3   {let obj = TestObject {id: (i * 100 + j) as usize,
                value: j as i32,
                references: vec![]};
            let gc_obj = gc.allocate(obj).expect(Failed to allocate)
            let obj_ptr = gc_obj.as_ptr()
            goroutine_gc.add_goroutine_root(i, obj_ptr)}

    // Test that incremental collection works
    match goroutine_gc.collect_garbage_goroutine_aware()     {Ok(stats) => {assert_eq!(stats.total_goroutines, 5, Should process all , goroutines)
            assert!(stats.stack_roots_found > 0, Should find stack ")
            info!()
                goroutines = stats.total_goroutines,
                stack_roots = stats.stack_roots_found,;
                 Incremental " collection test "Incremental:  collection test failed: {}, e)")"}
    // Clean up
    for i in 1..=5   {goroutine_gc.unregister_goroutine(i)}
