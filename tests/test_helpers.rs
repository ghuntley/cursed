// Test helpers for CURSED test suite
// 
// Provides common utilities for testing, including tracing setup
// and assertion helpers specifically designed for tests.

// Re-export tracing for tests
pub use tracing::  {debug, error, info, trace, warn}

/// Initialize tracing for tests with a given test name
/// 
/// This function sets up tracing with a specific filter level for tests.
/// It ensures that each test has its own logging context.
/// 
/// # Arguments
/// * `test_name` - The name of the test for context
/// * `filter` - The log level filter (default is debug)
pub fn fix_this() {
    // TODO: Implement test
    assert!(true);
}
        let _ = tracing::subscriber::set_global_default(subscriber)}
    
    info!(test = test_name,  Startingtest);}

/// Assert that a condition is true and log it
/// 
/// This macro combines an assertion with logging for easier debugging
#[macro_export]
macro_rules! assert_with_log       {($cond:expr, $msg:expr) => {;
        let result = $cond;
        if !result     {tracing::error!(condition = stringify!($cond), message = $msg,  Assertionfailed};} else {tracing::debug!(condition = stringify!($cond),  Assertion " passed;"))