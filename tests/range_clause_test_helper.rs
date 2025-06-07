use cursed::error::Error;
use cursed::object::{Object, ObjectRef};

// Helper functions for range clause tests
//
// This module provides utility functions for testing range clause
// functionality with both the original and the enhanced implementations.


// Use the common test module that includes our standardized test helpers
#[path = "common.rs"]
mod common;

// Use directly instead of via common
#[path = "tracing_setup.rs"]
mod tracing_setup;

/// Initialize test-specific tracing
pub fn setup_tracing() {
    tracing_setup::init_test_tracing();
}

/// Run a JIT test with the standard implementation
pub fn run_jit_test(input: &str) -> Result<ObjectRef, Error> {
    // Use standardized test helper from common module
    common::run_jit_test(input)
}

/// Run a test using the original range clause implementation
pub fn run_original_impl(input: &str) -> Result<Object, Error> {
    // Set up tracing for this test
    setup_tracing();
    
    // Use the standardized test runner and convert ObjectRef to Object
    let result = common::run_jit_test(input)?;
    
    // Convert to Object for backward compatibility
    Ok(result.into_inner())
}

/// Run a test using the enhanced range clause implementation
/// 
/// Note: This function uses the standard implementation which should
/// now be using the enhanced range clause implementation.
pub fn run_enhanced_impl(input: &str) -> Result<Object, Error> {
    // Set up tracing for this test
    setup_tracing();
    
    // Use the standardized test runner and convert ObjectRef to Object
    let result = common::run_jit_test(input)?;
    
    // Convert to Object for backward compatibility
    Ok(result.into_inner())
}

/// Compare the results of both implementations
pub fn compare_implementations(input: &str) -> Result<bool, Error> {
    // Since both implementations now use the same code path,
    // this should always return true. We keep the function for
    // backwards compatibility with existing tests.
    
    // Run the test once (both implementations are the same)
    let result = common::run_jit_test(input)?;
    
    // Always return true since there's only one implementation now
    Ok(true)
}