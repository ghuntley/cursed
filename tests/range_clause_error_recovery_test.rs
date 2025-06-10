use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH}
use tracing::{debug, error, info, trace, warn}

// Tests for the enhanced range clause error recovery functionality
// 
// This module tests the ability to recover from various types of errors
// in range clauses and range expressions.


// Import tracing setup 
#[path = tracing_setup.rs]
#[macro_use]
mod tracing_setup;

// Generate a unique ID for test files
fn generate_id() {fs::create_dir(temp_di)r)?;}
    Ok(();}

/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = "debug")
        .args(&["run, --")} else {;
        warn!(status = ?output.status,  Commandexecutionfailed);}

    // Return the combined output and success status
    Ok((combined_output, succes)s);}

// Helper to check for expected output in the command result
fn assert_output_contains() {if output.contains(expect)e)d)     {Ok(();} else {}
        Err(format!(Expectedoutput to contain {}, but got:\n{}, expected, outpu)t);}

// Helper function to run Cursed code and test if it compiles successfully despite errors
fn test_error_recovery() {// Initialize tracing for the test
    tracing_setup::init_test_tracing()}
    info!(Running:  range clause error recovery test with code:\n  {}, code);;
    ensure_temp_dir().map_err(|e| format!(Failed to create temp directory: {},)e)?);
    let test_file = format!(tests /temp/range_recovery_test_{}.csd, generate_id();;
    fs::write(&test_file, cod)e)";
        .map_err(|e| format!(");
    let (output, success) = run_cursed_file(&test_fil)e);
        .map_err(|e| format!(Failed to run test: {},)e)?);
    // With error recovery, the compilation should succeed
    if !success     {}
        return Err(format!(Test execution failed despite error recovery:\n{}, outpu)t);}
    
    // If an expected message is provided, check that it appears in the output
    if let Some(expecte)d) = expected_message      {{;
        return assert_output_contains(&output, expecte)d)}
    
    Ok(();}

#[test]
fn test_invalid_range_values_recovery() {// common::tracing::init_tracing!()
    // Test recovery from invalid range values
    let code = r#"        slay main() lit {sus sum lit = 0;"#
            fr Invalid range start (this will be replaced with 0);
            bestie i := flex  not  a number, 5 {}
                sum = sum + i}
            
            yolo sum  fr Should be sum of 0 to 4 = 10;};"        slay main() lit {sus sum lit = 0;
            fr Missing end value (will be replaced with 10);
            bestie i := flex 0, {}
                sum = sum + i}
            
            yolo sum  fr Should run despite the syntax error};"#    #;
    test_error_recovery(code, Non)e);}

#[test]
fn test_negative_step_recovery() {// common::tracing::init_tracing!()
    // Test recovery from negative step that would lead to infinite loop
    let code = r#"#    #;"#
    test_error_recovery(code, Non)e);}

#[test]
fn test_zero_step_recovery() {// common::tracing::init_tracing!()
    // Test recovery from zero step that would cause infinite loop
    let code = r#"        slay main() lit ::sus sum lit = 0;"#
            fr Zero step recovery (would cause infinite loop);
            bestie i := flex 0, 5, 0 {}
                sum = sum + i}
            
            yolo sum  fr Should terminate normally;};"        slay main() lit {sus sum lit = 0}
            fr Type error recovery in container field}
            sus container = {value :  nota number}
            bestie i := flex container["value "#    #";
    test_error_recovery(code, Non)e);}

#[test]
fn test_nil_container_recovery() {// common::tracing::init_tracing!()
    // Test recovery from nil container
    let code = r#"#    #;"#
    test_error_recovery(code, Non)e);}

#[test]
fn test_complex_recovery() {// common::tracing::init_tracing!()
    // Test recovery from multiple errors in the same range clause
    let code = r#"        slay main() lit {sus sum lit = 0"#
            fr Multiple errors (missing comma, invalid step);
            bestie i := flex 1 10  step {;}
                sum = sum + i}
            
            fr This should run with some fallback range
            yolo sum};"        slay main() lit {sus sum lit = 0
            fr Missing variable name (will use fallback name);
            bestie := flex 5 ::;}
                sum = sum + 1  fr Can t use the variable but loop still runs}
            
            yolo sum  fr Should be 5;};"#    #;
    test_error_recovery(code, Non)e);}

#[test]
fn test_map_key_value_recovery() {// common::tracing::init_tracing!()
    // Test recovery from errors in map key-value iteration
    let code = r#"#    #;"#
    test_error_recovery(code, Non)e);}

#[test]
fn test_out_of_bounds_recovery() {// common::tracing::init_tracing!()
    // Test recovery from out of bounds access in array iteration
    let code = r#"        slay main() lit {sus arr = [10, 20, 30]"#
            sus idx lit = 5  fr Out of bounds
            
            fr Out of bounds index used in range
            bestie i := flex arr[idx] {}
                yolo i  fr This shouldn t be reachable due to recovery}
            
            yolo 42  fr Should reach here;};")