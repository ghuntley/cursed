use std::path::Path;
use std::fs;
use std::process::Command;
use std::io::{self, Write};
use std::collections::HashSet;

/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file(file_path: &str) -> io::Result<(String, bool)> {
    let output = Command::new("cargo")
        .args(&["run", "--", file_path])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    // Combine stdout and stderr for debugging
    let combined_output = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
    
    // Return the combined output and success status
    Ok((combined_output, output.status.success()))
}

/// Tests JIT execution of a simple program with variables
#[test]
fn test_simple_program() {
    let test_file = "tests/jit/stan_simple.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Check that values are printed correctly
    assert!(output.contains("42"), "Expected output containing '42', got:\n{}", output);
    assert!(output.contains("123"), "Expected output containing '123', got:\n{}", output);
}

/// Tests JIT execution of simple arithmetic
#[test]
fn test_variable_arithmetic() {
    let test_file = "tests/jit/variable_arithmetic.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Check that arithmetic operations work
    assert!(output.contains("15"), "Expected output containing '15' (5 + 10), got:\n{}", output);
}