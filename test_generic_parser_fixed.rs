use std::process::Command;

fn main() {
    println!("Testing generic parser fixes...");
    
    // Test 1: Basic identifier parsing
    let result = Command::new("cargo")
        .args(&["test", "--lib", "parse_identifier", "--", "--nocapture"])
        .output()
        .expect("Failed to execute test");
    
    println!("Test 1 - Basic identifier parsing:");
    println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&result.stderr));
    
    // Test 2: Simple type parsing
    let result = Command::new("cargo")
        .args(&["test", "--lib", "parse_type", "--", "--nocapture"])
        .output()
        .expect("Failed to execute test");
    
    println!("Test 2 - Simple type parsing:");
    println!("stdout: {}", String::from_utf8_lossy(&result.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&result.stderr));
    
    println!("Generic parser fix tests completed.");
}
