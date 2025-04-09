//! Integration tests for the CURSED standard library

use std::process::Command;
use std::path::Path;

/// Test that the vibez (fmt) package works correctly
#[test]
fn test_vibez_package() {
    // Build the project first
    assert!(Command::new("cargo").args(["build"]).status().unwrap().success());
    
    // Run the test file
    let output = Command::new("./target/debug/cursed")
        .arg("tests/stdlib_basic_test.csd")
        .output()
        .expect("Failed to execute test");
    
    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print output for debugging
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    
    // Check that the program exits successfully
    assert!(output.status.success(), "Program execution failed");
    
    // Check that the output contains expected strings
    assert!(stdout.contains("Testing Standard Library"), "Missing standard test output");
    assert!(stdout.contains("Simple format: number 42"), "Format string not working");
    assert!(stdout.contains("Value: 3.14"), "Float formatting not working");
}

/// Test that the stringz (strings) package works correctly
#[test]
fn test_stringz_package() {
    // Build the project first
    assert!(Command::new("cargo").args(["build"]).status().unwrap().success());
    
    // Run the test file
    let output = Command::new("./target/debug/cursed")
        .arg("tests/stringz_test.csd")
        .output()
        .expect("Failed to execute test");
    
    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print output for debugging
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    
    // Check that the program exits successfully
    assert!(output.status.success(), "Program execution failed");
    
    // Check that the output contains expected strings
    assert!(stdout.contains("Contains: 'world' is in the string"), "Contains function not working");
    assert!(stdout.contains("Contains: 'universe' is NOT in the string"), "Contains function not working");
    assert!(stdout.contains("HasPrefix 'Hello': based"), "HasPrefix function not working");
    assert!(stdout.contains("HasPrefix 'Hi': cap"), "HasPrefix function not working");
    assert!(stdout.contains("Split result"), "Split function not working");
    assert!(stdout.contains("HELLO"), "ToUpper function not working");
    assert!(stdout.contains("hello"), "ToLower function not working");
}

/// Test that the mathz (math) package works correctly
#[test]
fn test_mathz_package() {
    // Build the project first
    assert!(Command::new("cargo").args(["build"]).status().unwrap().success());
    
    // Run the test file
    let output = Command::new("./target/debug/cursed")
        .arg("tests/mathz_test.csd")
        .output()
        .expect("Failed to execute test");
    
    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print output for debugging
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    
    // Check that the program exits successfully
    assert!(output.status.success(), "Program execution failed");
    
    // Check that the output contains expected strings
    assert!(stdout.contains("PI ="), "Pi constant not working");
    assert!(stdout.contains("E ="), "E constant not working");
    assert!(stdout.contains("sqrt(25) = 5"), "Sqrt function not working");
    assert!(stdout.contains("abs(-10.5) = 10.5"), "Abs function not working");
    assert!(stdout.contains("ceil(3.7) = 4"), "Ceil function not working");
    assert!(stdout.contains("floor(3.7) = 3"), "Floor function not working");
    assert!(stdout.contains("pow(2, 10) = 1024"), "Pow function not working");
}

/// Test that the timez (time) package works correctly
#[test]
fn test_timez_package() {
    // Build the project first
    assert!(Command::new("cargo").args(["build"]).status().unwrap().success());
    
    // Run the test file
    let output = Command::new("./target/debug/cursed")
        .arg("tests/timez_test.csd")
        .output()
        .expect("Failed to execute test");
    
    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print output for debugging
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    
    // Check that the program exits successfully
    assert!(output.status.success(), "Program execution failed");
    
    // Check that the output contains expected strings
    assert!(stdout.contains("Current time"), "Now function not working");
    assert!(stdout.contains("Unix timestamp"), "UnixTimestamp function not working");
    assert!(stdout.contains("Second"), "Duration constants not working");
    assert!(stdout.contains("Sleeping for 10ms"), "Sleep function not shown");
    assert!(stdout.contains("Awake now"), "Sleep function not working");
    assert!(stdout.contains("Duration"), "DurationFromSecs function not working");
}

/// Test that the vibe_life (os) package works correctly
#[test]
fn test_vibe_life_package() {
    // Build the project first
    assert!(Command::new("cargo").args(["build"]).status().unwrap().success());
    
    // Run the test file
    let output = Command::new("./target/debug/cursed")
        .arg("tests/vibe_life_test.csd")
        .output()
        .expect("Failed to execute test");
    
    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print output for debugging
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    
    // Check that the program exits successfully
    assert!(output.status.success(), "Program execution failed");
    
    // Check that the output contains expected strings
    assert!(stdout.contains("Command line arguments"), "Args function not working");
    assert!(stdout.contains("Environment variable"), "Getenv/Setenv functions not working");
    assert!(stdout.contains("Current directory"), "Getwd function not working");
    assert!(stdout.contains("File exists?"), "Exists function not working");
}

/// Test that the dropz (io) package works correctly
#[test]
fn test_dropz_package() {
    // Build the project first
    assert!(Command::new("cargo").args(["build"]).status().unwrap().success());
    
    // Run the test file
    let output = Command::new("./target/debug/cursed")
        .arg("tests/dropz_test.csd")
        .output()
        .expect("Failed to execute test");
    
    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print output for debugging
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    
    // Check that the program exits successfully
    assert!(output.status.success(), "Program execution failed");
    
    // Check that the output contains expected strings
    assert!(stdout.contains("wrote test file"), "WriteFile function not working");
    assert!(stdout.contains("Read"), "ReadFile function not working");
    assert!(stdout.contains("File content"), "ReadFileString function not working");
    assert!(stdout.contains("Seeked to position"), "Seek function not working");
}

/// Test that the web_vibez (net/http) package works correctly
#[test]
fn test_web_vibez_package() {
    // Build the project first
    assert!(Command::new("cargo").args(["build"]).status().unwrap().success());
    
    // Run the test file with a timeout since it involves network operations
    let output = Command::new("./target/debug/cursed")
        .arg("tests/web_vibez_test.csd")
        .output()
        .expect("Failed to execute test");
    
    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print output for debugging
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    
    // Check that the program exits successfully
    assert!(output.status.success(), "Program execution failed");
    
    // Check that the output contains expected strings for HTTP client
    assert!(stdout.contains("Testing HTTP client"), "HTTP client test not started");
    assert!(stdout.contains("Response Status"), "HTTP client GET request failed");
    assert!(stdout.contains("Content-Type"), "HTTP response headers not working");
    
    // Check HTTP server output
    assert!(stdout.contains("Testing HTTP server"), "HTTP server test not started");
    assert!(stdout.contains("Starting HTTP server"), "HTTP server not started");
    assert!(stdout.contains("Response from root"), "Root endpoint test failed");
    assert!(stdout.contains("Response from API"), "API endpoint test failed");
    assert!(stdout.contains("Response from echo"), "Echo endpoint test failed");
    assert!(stdout.contains("Server gracefully shut down"), "Server shutdown failed");
}

/// Test that the concurrenz (sync) package works correctly
#[test]
fn test_concurrenz_package() {
    // Build the project first
    assert!(Command::new("cargo").args(["build"]).status().unwrap().success());
    
    // Run the test file
    let output = Command::new("./target/debug/cursed")
        .arg("tests/concurrenz_test.csd")
        .output()
        .expect("Failed to execute test");
    
    // Convert output to string
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Print output for debugging
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    
    // Check that the program exits successfully
    assert!(output.status.success(), "Program execution failed");
    
    // Check that the output contains expected strings
    assert!(stdout.contains("Testing concurrenz package"), "Concurrenz package not loaded");
    assert!(stdout.contains("Mutex test passed"), "Mutex test failed");
    assert!(stdout.contains("RWMutex test passed"), "RWMutex test failed");
    assert!(stdout.contains("WaitGroup test passed"), "WaitGroup test failed");
    assert!(stdout.contains("Once test passed"), "Once test failed");
    assert!(stdout.contains("All concurrenz tests completed successfully"), "Not all tests passed");
}