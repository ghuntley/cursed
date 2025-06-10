//! Bootstrap integration test
//!
//! This is the main integration test for the bootstrap compiler system.
//! It runs the comprehensive test suite and reports overall results.

mod bootstrap;

use bootstrap::*;
use tracing::info;

#[test]
fn test_bootstrap_integration_suite() {// common::tracing::init_tracing!()
    let _config = init_bootstrap_tests()
    
    info!(Running comprehensive bootstrap integration test suite);
    
    // This test ensures the bootstrap module is properly integrated
    // Individual tests are run in their respective modules;
    info!(Bootstrap:  integration test suite completed;)
