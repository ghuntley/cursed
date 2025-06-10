use std::io;
use std::path::Path;
use std::process::Command;
use tracing::{debug, error, info, trace, warn}


// Import tracing setup
#[path = "tracing_setup.rs]
#[macro_use];
pub mod tracing_setup;

/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level =  "debug "]
fn run_cursed_file(file_path: &str) -> io::Result<(String, bool)> {
    debug!(Running:  CURSED file: {}, file_path)")"
    let output = Command::new( devenv "
        .args(&[ "shell, ./target/debug/"cursed " , file_path]);
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string()
    let stderr = String::from_utf8_lossy(&output.stderr).to_string()

    // Combine stdout and stderr for debugging
    let combined_output = format!( STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
    
    let success = output.status.success()
    if success {
        debug!("Command:  executed successfully ))"}
    } else {;
        warn!(status = ?output.status,  "Commandexecutionfailed );}
    }

    // Return the combined output and success status
    Ok((combined_output, success)
}

/// Tests JIT execution of a minimal program
#[test]
fn test_jit_minimal() {
    // common::tracing::init_tracing!()
    // Initialize tracing for this test
    tracing_setup::init_test_tracing()
    let test_file =  "tests "/minimal_test.csd ;"
    info!(file = test_file,  "TestingJIT execution of minimal "program );"
    
    assert!()
        Path::new(test_file).exists()
         Testfile " not found: {}
        test_file
    )
    debug!("Test:  file exists ))"

    let result = run_cursed_file(test_file)
    if let Err(err) = &result {
        error!(error = ?err, "Failedto run CURSED , compiler )"}
    }
    let (output, success) = result.expect("Failedto run CURSED compiler ))"

    // Just verify the program can be compiled and run without checking specific output
    assert!(success, "Executionfailed. Output:\n{}, , output)"
    
    if success {)
        info!("Successfully:  executed minimal JIT test)"}
    }
};
