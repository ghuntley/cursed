//! Test environment utilities for memory management
//!
//! This module provides utilities to help with testing the memory management system,
//! including ways to control garbage collection behavior and simulate various conditions.

use std::sync::{Arc, Mutex, RwLock};
use once_cell::sync::Lazy;

/// Global flag to indicate whether we're in a test environment
static IS_TEST_ENVIRONMENT: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(false));

/// Global test GC instance
static TEST_GC: Lazy<RwLock<Option<crate::memory::gc::GarbageCollector>>> = 
    Lazy::new(|| RwLock::new(None));

/// Get a GC instance for testing
pub fn get_test_gc() -> crate::memory::gc::GarbageCollector {
    // Try to get the existing test GC
    if let Ok(test_gc_lock) = TEST_GC.read() {
        if let Some(gc) = test_gc_lock.as_ref() {
            return gc.clone();
        }
    }
    
    // Create a new GC and store it
    let gc = crate::memory::gc::GarbageCollector::new();
    if let Ok(mut test_gc_lock) = TEST_GC.write() {
        *test_gc_lock = Some(gc.clone());
    }
    
    // Set test environment flag
    set_test_environment(true);
    
    gc
}

/// Configuration for the test environment
#[derive(Debug, Default)]
pub struct TestConfig {
    /// Whether to disable automatic garbage collection
    pub disable_gc: bool,
    /// Whether to simulate race conditions in the garbage collector
    pub simulate_races: bool,
    /// Whether to enable debug output even when DEBUG_MEMORY is off
    pub force_debug: bool,
    /// Whether to disable timeouts in deadlock detection
    pub disable_timeouts: bool,
}

/// Global test configuration
static TEST_CONFIG: Lazy<Mutex<TestConfig>> = Lazy::new(|| Mutex::new(TestConfig::default()));

/// Set the test environment flag
pub fn set_test_environment(is_test: bool) {
    if let Ok(mut test_env) = IS_TEST_ENVIRONMENT.write() {
        *test_env = is_test;
    }
}

/// Check if we're in a test environment
pub fn is_test_environment() -> bool {
    IS_TEST_ENVIRONMENT.read().map(|v| *v).unwrap_or(false)
}

/// Get a mutable reference to the test configuration
pub fn test_config() -> &'static Mutex<TestConfig> {
    &TEST_CONFIG
}

/// Reset the test environment to default configuration
pub fn reset_test_environment() {
    set_test_environment(true);
    if let Ok(mut config) = TEST_CONFIG.lock() {
        *config = TestConfig::default();
    }
}

/// Disable garbage collection in tests
pub fn disable_gc() {
    if let Ok(mut config) = TEST_CONFIG.lock() {
        config.disable_gc = true;
    }
}

/// Enable garbage collection in tests
pub fn enable_gc() {
    if let Ok(mut config) = TEST_CONFIG.lock() {
        config.disable_gc = false;
    }
}

/// Check if garbage collection is disabled
pub fn is_gc_disabled() -> bool {
    TEST_CONFIG.lock().map(|c| c.disable_gc).unwrap_or(false)
}

/// Simulate race conditions in the garbage collector
pub fn simulate_race_conditions(enable: bool) {
    if let Ok(mut config) = TEST_CONFIG.lock() {
        config.simulate_races = enable;
    }
}

/// Check if race condition simulation is enabled
pub fn is_race_simulation_enabled() -> bool {
    TEST_CONFIG.lock().map(|c| c.simulate_races).unwrap_or(false)
}

/// Force debug output in tests
pub fn force_debug(enable: bool) {
    if let Ok(mut config) = TEST_CONFIG.lock() {
        config.force_debug = enable;
    }
}

/// Check if debug output is forced
pub fn is_debug_forced() -> bool {
    TEST_CONFIG.lock().map(|c| c.force_debug).unwrap_or(false)
}

/// Disable timeouts in deadlock detection
pub fn disable_timeouts(disable: bool) {
    if let Ok(mut config) = TEST_CONFIG.lock() {
        config.disable_timeouts = disable;
    }
}

/// Check if timeouts are disabled
pub fn are_timeouts_disabled() -> bool {
    TEST_CONFIG.lock().map(|c| c.disable_timeouts).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_environment_config() {
        // Reset to default
        reset_test_environment();
        
        // Check defaults
        assert!(is_test_environment());
        assert!(!is_gc_disabled());
        assert!(!is_race_simulation_enabled());
        
        // Modify config
        disable_gc();
        simulate_race_conditions(true);
        
        // Check modified config
        assert!(is_gc_disabled());
        assert!(is_race_simulation_enabled());
        
        // Reset again
        reset_test_environment();
        
        // Check defaults again
        assert!(!is_gc_disabled());
        assert!(!is_race_simulation_enabled());
    }
}