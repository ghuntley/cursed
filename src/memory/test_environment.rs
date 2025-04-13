//! Test environment utilities for memory management
//!
//! This module provides specialized utilities for use in test environments
//! to help prevent deadlocks and improve test reliability when the
//! garbage collector is running in parallel tests.

use std::sync::{Arc, Mutex, RwLock};
use std::cell::RefCell;
use std::thread::ThreadId;
use std::thread;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::memory::gc::GarbageCollector;

/// Thread-local GC for testing purposes
static THREAD_LOCAL_GCS: Lazy<Mutex<HashMap<ThreadId, Arc<GarbageCollector>>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Get a thread-local GC for testing
/// 
/// This function returns a GC instance that is specific to the current thread,
/// which helps prevent deadlocks when multiple tests are running in parallel.
pub fn get_test_gc() -> Arc<GarbageCollector> {
    let thread_id = thread::current().id();
    
    let mut gcs = THREAD_LOCAL_GCS.lock().unwrap();
    
    // If we don't have a GC for this thread yet, create one
    if !gcs.contains_key(&thread_id) {
        // Create a new GC for this thread
        let gc = Arc::new(GarbageCollector::new());
        gcs.insert(thread_id, gc.clone());
        return gc;
    }
    
    // Return the existing GC for this thread
    gcs.get(&thread_id).unwrap().clone()
}

/// Thread-local flag to indicate we're in a test environment
pub fn is_test_environment() -> bool {
    // Check environment variable
    if std::env::var("RUNNING_TESTS").is_ok() {
        return true;
    }
    
    // Check for common test binary patterns in the executable name
    if let Ok(exe) = std::env::current_exe() {
        if let Some(file_name) = exe.file_name() {
            if let Some(name) = file_name.to_str() {
                return name.contains("test") || 
                       name.ends_with("-test") || 
                       name.contains("_test-");
            }
        }
    }
    
    // Check thread name for test indicators
    if let Some(thread_name) = thread::current().name() {
        return thread_name.contains("test");
    }
    
    false
}

/// Reset the test environment
/// 
/// This function clears all thread-local GCs and should be called at the end of each test.
pub fn reset_test_environment() {
    let mut gcs = THREAD_LOCAL_GCS.lock().unwrap();
    gcs.clear();
}