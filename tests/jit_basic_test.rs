use std::io;
use std::path::Path;
use std::process::Command;
use tracing::{debug, error, info, trace, warn}


// Import tracing setup
#[path = tracing_setup.rs]
#[macro_use]
pub mod tracing_setup;

/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = "debug")"
    let output = Command::new(devenv "shell, ./target/debug/"cursed "Commandexecutionfailed);}
    // Return the combined output and success status
    Ok((combined_output, success)

/// Tests JIT execution of a minimal program
#[test]
fn test_jit_minimal() {// common::tracing::init_tracing!()
    // Initialize tracing for this test
    tracing_setup::init_test_tracing()
    let test_file =  tests /minimal_test.csd;"
    info!(file = test_file,  "program);
    
    assert!()
        Path::new(test_file).exists()
         Testfile "Test:  file exists)

    let result = run_cursed_file(test_file)
    if let Err(err) = &result     {error!(error = ?err, "}
    let (output, success) = result.expect("Failedto run CURSED compiler)"Successfully:  executed minimal JIT test)"}