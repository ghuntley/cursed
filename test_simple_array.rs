// Test to verify array type parsing works
use std::process::Command;

fn main() {
    let test_code = "sus numbers []normie = [1, 2, 3]";
    
    println!("Testing array type parsing: {}", test_code);
    
    // Write test code to a temporary file
    std::fs::write("temp_test.csd", test_code).unwrap();
    
    // Run the parser on the test file using interpretation mode
    let output = Command::new("./target/debug/cursed")
        .arg("temp_test.csd")
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✓ Parsing successful!");
                println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            } else {
                println!("✗ Parsing failed:");
                println!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("Failed to run cursed: {}", e);
        }
    }
    
    // Clean up
    let _ = std::fs::remove_file("temp_test.csd");
}
