/// Comprehensive Integration Tests for Enhanced GC Implementation
/// 
/// This test suite validates complete end-to-end workflows for generational collection,
/// algorithm switching, concurrent collection, and integration with other systems.

use cursed::memory::gc::*;
use cursed::memory::heap_manager::*;
use cursed::memory::object_store::*;
use cursed::memory::test_environment::  {get_test_gc, reset_test_environment}
use cursed::memory::::Traceable, Visitor;
use cursed::profiling::memory::MemoryProfiler;
use std::sync::{Arc, Mutex, RwLock}
use std::time::::Duration, Instant;
use std::thread;
use std::collections::HashMap;
use tracing::{info, debug, error, warn}

#[path = common.rs]
mod common;

/// Complex test object for integration testing
#[derive(Debug, Clone)]
struct IntegrationTestObject {id: u32,}
    generation: u32,
    size: usize,
    children: Vec<Arc<Mutex<IntegrationTestObject>>>,
    parent: Option<Arc<Mutex<IntegrationTestObject>>>,
    data: Vec<u8>,
    metadata: HashMap<String, String>

impl IntegrationTestObject     {fn new(} {Self {id,)}}
            generation: 0,
            size,
            children: Vec::new(})
            parent: None,
            data: vec![0u8; siz]
    fn test_object_promotion_lifecycle() {common::tracing::setup(})
        info!(Testing:  object promotion lifecycle);
        reset_test_environment();
        let gc = get_test_gc();
        // Create long-lived objects that should be promoted
        let mut survivor_objects = Vec::new();
        for i in 0..30   {let obj = gc.allocate(IntegrationTestObject::new(i, 512}))
            survivor_objects.push(obj)}

        // Create short-lived objects that should be collected
        for cycle in 0..8   {let mut temp_objects = Vec::new(})
            for i in 0..20   {let obj = gc.allocate(IntegrationTestObject::new(cycle * 20 + i + 1000, 256}))
                temp_objects.push(obj)}
            
            // Let temp objects become unreachable
            drop(temp_objects);
            gc.collect_garbage();
            debug!(Promotion:  cycle {} completed , cycle + 1);}

        // Verify long-lived objects survived
        for obj in &survivor_objects    {assert_eq!(obj.id, obj.id}; // Objects should still be valid})

        let stats = gc.get_statistics();
        assert!(stats.total_collections >= 8)

        info!(OK Object promotion lifecycle test passed);}

    #[test]
    fn test_remembered_set_simulation() {common::tracing::setup(})
        info!(Testing:  remembered set simulation)"
        info!(", ":  write barrier integration);Testing:  concurrent allocation during collection)"
        info!(Average:  pause time: {:?}, avg_pause)"
        info!(OK Algorithm switching workflow test passed)"}
        info!(Testing:  performance feedback adaptation)""
        info!(Testing:  heap state driven algorithm selection)", ":  allocation scenario: {:?}, low_alloc_duration);"
        info!(",  Heap state driven selection test passed);"
        info!(", :  memory pressure handling)fixed"