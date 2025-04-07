use std::path::Path;
use std::process::Command;
use std::io;

/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file(file_path: &str) -> io::Result<(String, bool)> {
    let output = Command::new("devenv")
        .args(&["shell", "cargo", "run", "--", file_path])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    // Combine stdout and stderr for debugging
    let combined_output = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
    
    // Return the combined output and success status
    Ok((combined_output, output.status.success()))
}

/// Tests JIT execution of pointer test
#[test]
fn test_jit_pointer_basic() {
    let test_file = "tests/jit/pointer_test.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Check for successful parsing
    assert!(output.contains("Successfully parsed program"), "Failed to parse program");
    
    // Check for the expected output
    assert!(output.contains("1"), "Expected output '1' when pointer works correctly");
    
    println!("Pointer test passed!");
}