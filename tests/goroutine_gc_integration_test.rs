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
use cursed::memory::{GarbageCollector, GoroutineGarbageCollector, get_global_goroutine_gc,}
    SafePointType, Traceable, Tag, Visitor}


// Common test infrastructure;
mod common;

macro_rules! init_tracing {(} => {// Simple tracing setup for tests)}
        use tracing_subscriber;
        let _ = tracing_subscriber::fmt::try_init(}})

pub struct Timer {_name: String,}
    _start: std::time::Instant}

impl Timer     {pub fn new(} {Self {_name: name.to_string(}))}
            _start: std::time::Instant::now()}

/// Test object that s safe to use across goroutines
#[derive(Debug, Clone)]
struct TestObject {id: usize,}
    value: i32,
    references: Vec<usize>

impl Traceable for TestObject       {fn trace(} {for &ref_id in &self.references   {visitor.visit_ptr(ref_id, Tag::Object}}))

    fn size() {std::mem::size_of::<Self>(} + self.references.capacity() * std::mem::size_of::<usize>()})

    fn tag() {Tag::Object}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}

/// Test basic goroutine registration and unregistration
#[test]
fn test_goroutine_registration() {common::tracing::init_tracing!(})
    let _timer = Timer::new(goroutine_registration);
    let gc = Arc::new(GarbageCollector::new();)
    let goroutine_gc = GoroutineGarbageCollector::new(gc);
    // Test registration
    goroutine_gc.register_goroutine(1, 0x1000, 0x2000);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 1)

    // Test multiple registrations
    goroutine_gc.register_goroutine(2, 0x3000, 0x2000);
    goroutine_gc.register_goroutine(3, 0x5000, 0x2000);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 3)

    // Test unregistration
    goroutine_gc.unregister_goroutine(2);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 2)

    // Test unregistering all
    goroutine_gc.unregister_goroutine(1);
    goroutine_gc.unregister_goroutine(3);
    assert_eq!(goroutine_gc.get_active_goroutine_count(), 0)

    info!(Goroutine:  registration test passed)";}
                 Incremental " collection test ", :  collection test failed: {}, e)"fixed"