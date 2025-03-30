use std::path::Path;
use std::fs;
use std::process::Command;
use std::io::{self, Write};
use std::collections::HashSet;

// This is an integration test for the CURSED language JIT execution
// It reads the test files in tests/jit/ and runs them through the CURSED compiler
// then checks for successful execution

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

/// List of tests that are known to fail due to unimplemented features
fn known_failing_tests() -> HashSet<&'static str> {
    let mut failures = HashSet::new();
    // While loop should work now that we've implemented assignment expressions
    // failures.insert("tests/jit/while_loop.csd"); // Assignment in while loop not implemented
    failures.insert("tests/jit/complex_test.csd"); // Contains other unimplemented features
    // failures.insert("tests/jit/if_else.csd"); // Issue with token parsing
    return failures;
}

/// Tests JIT execution of the puts function with an integer argument
#[test]
fn test_puts_integer() {
    let test_file = "tests/jit/puts_integer.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    assert!(output.contains("42"), "Expected output containing '42', got:\n{}", output);
}

/// Tests JIT execution of the println function with a string argument
#[test]
fn test_println_string() {
    let test_file = "tests/jit/println_string.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    assert!(output.contains("Hello, CURSED!"), 
            "Expected output containing 'Hello, CURSED!', got:\n{}", output);
}

/// Tests JIT execution of variable declarations and arithmetic
#[test]
fn test_variable_arithmetic() {
    let test_file = "tests/jit/variable_arithmetic.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    assert!(output.contains("15"), 
            "Expected output containing '15' (5 + 10), got:\n{}", output);
}

/// Tests JIT execution of if-else statements with parentheses around the condition (optional)
#[test]
fn test_if_else() {
    let test_file = "tests/jit/if_else.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    // Only check for success if the test is expected to pass
    if !known_failing_tests().contains(test_file) {
        assert!(success, "Execution failed. Output:\n{}", output);
        assert!(output.contains("42"), 
                "Expected output containing '42' (from the if block), got:\n{}", output);
    } else {
        println!("Skipping assertion for known failing test: {}", test_file);
    }
}

/// Tests JIT execution of while loops
#[test]
fn test_while_loop() {
    let test_file = "tests/jit/while_loop.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    // This test should now pass with our implementation of assignments in while loops
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Check that we get outputs for 0, 1, 2, 3, 4
    assert!(output.contains("0"), "Expected output to contain '0'");
    assert!(output.contains("1"), "Expected output to contain '1'");
    assert!(output.contains("2"), "Expected output to contain '2'");
    assert!(output.contains("3"), "Expected output to contain '3'");
    assert!(output.contains("4"), "Expected output to contain '4'");
    
    // But not 5, since we loop while counter < 5
    assert!(!output.contains("Execution Output ---\n5"), 
           "Output should not contain '5' as we loop while counter < 5");
}

/// Tests JIT execution of the complex test with multiple language features
#[test]
#[ignore = "contains features not implemented yet"]
fn test_complex() {
    let test_file = "tests/jit/complex_test.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    // This test contains unimplemented features
    println!("Complex test output: {}", output);
}

/// Tests JIT execution of if-else statements without parentheses around the condition
#[test]
fn test_if_no_parens() {
    let test_file = "tests/jit/if_no_parens.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    assert!(output.contains("42"), 
            "Expected output containing '42' (from the if block), got:\n{}", output);
}

/// Tests line comments (fr fr) parsing and execution
#[test]
fn test_line_comments() {
    let test_file = "tests/jit/line_comments.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Verify that both puts statements were executed and comments were ignored
    assert!(output.contains("42"), "Expected output containing '42', got:\n{}", output);
    assert!(output.contains("100"), "Expected output containing '100', got:\n{}", output);
    
    // Make sure no comment text is in the output
    assert!(!output.contains("This is a line comment"), 
            "Comment text should not be in output: {}", output);
    assert!(!output.contains("This is a comment after code"), 
            "Comment text should not be in output: {}", output);
}

/// Tests block comments (no cap / on god) parsing and execution
#[test]
fn test_block_comments() {
    let test_file = "tests/jit/block_comments.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Verify that all puts statements were executed and comments were ignored
    assert!(output.contains("42"), "Expected output containing '42', got:\n{}", output);
    assert!(output.contains("100"), "Expected output containing '100', got:\n{}", output);
    assert!(output.contains("200"), "Expected output containing '200', got:\n{}", output);
    
    // Make sure no comment text is in the output
    assert!(!output.contains("This is a block comment"), 
            "Comment text should not be in output: {}", output);
    assert!(!output.contains("Multi-line block comment"), 
            "Comment text should not be in output: {}", output);
    assert!(!output.contains("Nested block comment"), 
            "Comment text should not be in output: {}", output);
}

/// Tests mixed line and block comments parsing and execution
#[test]
fn test_mixed_comments() {
    let test_file = "tests/jit/mixed_comments.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Verify that all puts statements were executed and comments were ignored
    assert!(output.contains("42"), "Expected output containing '42', got:\n{}", output);
    assert!(output.contains("100"), "Expected output containing '100', got:\n{}", output);
    assert!(output.contains("200"), "Expected output containing '200', got:\n{}", output);
    assert!(output.contains("300"), "Expected output containing '300', got:\n{}", output);
    
    // Make sure no comment text is in the output
    assert!(!output.contains("Line comment at the start"), 
            "Comment text should not be in output: {}", output);
    assert!(!output.contains("Block comment before code"), 
            "Comment text should not be in output: {}", output);
}

/// Tests JIT execution of constant declarations using facts keyword
#[test]
fn test_facts_constant() {
    let test_file = "tests/jit/facts_constant.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    
    // Test area calculation using integer constants (5 * 3 = 15)
    assert!(output.contains("15"), 
            "Expected area output containing '15', got:\n{}", output);
    
    // Test string constant
    assert!(output.contains("Hello, CURSED!"), 
            "Expected output containing 'Hello, CURSED!', got:\n{}", output);
    
    // Test integer constant
    assert!(output.contains("42"), 
            "Expected output containing '42', got:\n{}", output);
    
    // Test boolean constant used in conditional
    assert!(output.contains("Feature is enabled"), 
            "Expected output containing 'Feature is enabled', got:\n{}", output);
}

/// Run all JIT tests in the directory that are expected to pass
#[test]
fn test_all_jit_files() {
    let jit_dir = "tests/jit";
    assert!(Path::new(jit_dir).is_dir(), "JIT test directory not found");
    let failing_tests = known_failing_tests();
    
    for entry in fs::read_dir(jit_dir).expect("Failed to read JIT test directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        // Skip non-CURSED files
        if path.extension().and_then(|ext| ext.to_str()) != Some("csd") {
            continue;
        }
        
        let path_str = path.to_str().expect("Invalid path");
        
        // Skip tests known to fail
        if failing_tests.contains(path_str) {
            println!("Skipping known failing test: {:?}", path);
            continue;
        }
        
        println!("Testing: {:?}", path);
        let (output, success) = run_cursed_file(path_str)
            .expect("Failed to run CURSED compiler");
        
        assert!(success, "Execution of {:?} failed. Output:\n{}", path, output);
        println!("Test passed: {:?}", path);
    }
} 