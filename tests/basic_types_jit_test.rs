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

/// Tests JIT execution of basic types
#[test]
fn test_basic_types_jit() {
    let test_file = "tests/jit/basic_types_test.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Check that compilation was successful
    assert!(output.contains("Compilation successful"), "Compilation failed");
    
    // Check LLVM IR for correct types
    assert!(output.contains("store i1 true"), "Boolean 'based' not correctly compiled");
    assert!(output.contains("store i64 42"), "Integer not correctly compiled");
    assert!(output.contains("store double 3.140000"), "Float not correctly compiled");
    assert!(output.contains("Hello, CURSED!"), "String not correctly compiled");
    assert!(output.contains("store i32 67"), "Character 'C' not correctly compiled");
    
    println!("All basic types test passed!");
}