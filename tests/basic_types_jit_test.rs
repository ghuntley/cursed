use std::io;
use std::path::Path;
use std::process::Command;
use tracing::{debug, error, info, instrument, trace, warn};


// Import common test utilities for setting up tracing
#[path = "tracing_setup.rs"]
mod tracing_setup;

/// Runs a CURSED file through the compiler and returns the output and exit status
#[instrument(skip(file_path), fields(path = %file_path))]
fn run_cursed_file(file_path: &str) -> io::Result<(String, bool)> {
    debug!("Running CURSED compiler on file");
    let output = Command::new("devenv")
        .args(&["shell", "cargo", "run", "--", file_path])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    let success = output.status.success();
    if !success {
        warn!(status = ?output.status, "Command execution failed");
    } else {
        debug!(status = ?output.status, "Command executed successfully");
    }

    // Combine stdout and stderr for debugging
    let combined_output = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
    trace!(output_length = combined_output.len(), "Captured command output");

    // Return the combined output and success status
    Ok((combined_output, success))
}

/// Tests JIT execution of basic types
#[test]
#[ignore = "Requires external execution environment"]
#[instrument]
fn test_basic_types_jit() {
    tracing_setup::init_test_tracing();
    info!("Starting basic types JIT test");
    let test_file = "tests/jit/basic_types_test.csd";
    debug!(test_file = %test_file, "Checking test file existence");
    
    let file_exists = Path::new(test_file).exists();
    if !file_exists {
        error!(test_file = %test_file, "Test file not found");
    }
    assert!(
        file_exists,
        "Test file not found: {}",
        test_file
    );
    debug!("Test file exists");

    debug!("Running CURSED compiler on test file");
    let (output, success) = run_cursed_file(test_file).expect("Failed to run CURSED compiler");
    
    if !success {
        error!("Execution failed");
    }
    assert!(success, "Execution failed. Output:\n{}", output);
    debug!("Execution completed successfully");

    // Check that compilation was successful
    debug!("Verifying compilation success");
    let compilation_successful = output.contains("Compilation successful");
    if !compilation_successful {
        error!("Compilation failed");
    }
    assert!(
        compilation_successful,
        "Compilation failed: {}\n",
        output
    );
    debug!("Compilation was successful");

    // Check LLVM IR for boolean value
    debug!("Checking boolean type in LLVM IR");
    let boolean_correct = output.contains("store i1 true") || output.contains("store i1 1");
    if !boolean_correct {
        error!("Boolean 'based' not correctly compiled as i1 true");
    }
    assert!(
        boolean_correct,
        "Boolean 'based' not correctly compiled as i1 true: {}\n",
        output
    );
    debug!("Boolean type verified");

    // Check LLVM IR for correct types
    debug!("Checking integer type in LLVM IR");
    let integer_correct = output.contains("store i64 42");
    if !integer_correct {
        error!("Integer not correctly compiled as i64");
    }
    assert!(
        integer_correct,
        "Integer not correctly compiled as i64: {}\n",
        output
    );
    debug!("Integer type verified");

    debug!("Checking float type in LLVM IR");
    let float_correct = output.contains("store double 3.140000");
    if !float_correct {
        error!("Float not correctly compiled as double");
    }
    assert!(
        float_correct,
        "Float not correctly compiled as double: {}\n",
        output
    );
    debug!("Float type verified");

    debug!("Checking string type in LLVM IR");
    let string_correct = output.contains("Hello, CURSED!");
    if !string_correct {
        error!("String not correctly compiled");
    }
    assert!(
        string_correct,
        "String not correctly compiled: {}\n",
        output
    );
    debug!("String type verified");

    debug!("Checking character type in LLVM IR");
    let char_correct = output.contains("store i32 67") || output.contains("store i8 67");
    if !char_correct {
        error!("Character 'C' not correctly compiled");
    }
    assert!(
        char_correct,
        "Character 'C' not correctly compiled: {}\n",
        output
    );
    debug!("Character type verified");

    info!("All basic types test passed successfully!");
}
