//! Test environment for memory subsystem tests

use std::sync::{Arc, Mutex};
use crate::memory::gc::GarbageCollector;

/// Create a test garbage collector
pub fn create_test_gc() -> Arc<GarbageCollector> {
    Arc::new(GarbageCollector::new())
}

// Global test GC for testing
lazy_static::lazy_static! {
    static ref TEST_GC: Mutex<Option<Arc<GarbageCollector>>> = Mutex::new(None);
}

/// Get a reference to the global test GC
pub fn get_test_gc() -> Arc<GarbageCollector> {
    let mut test_gc = TEST_GC.lock().unwrap();
    if test_gc.is_none() {
        *test_gc = Some(create_test_gc());
    }
    test_gc.as_ref().unwrap().clone()
}

/// Reset the test environment
pub fn reset_test_environment() {
    *TEST_GC.lock().unwrap() = None;
}