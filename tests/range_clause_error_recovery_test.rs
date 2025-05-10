//! Tests for range clause error recovery implementation

use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// Generate a unique ID for test files
fn generate_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

// Import tracing setup
#[path = "tracing_setup.rs"]
#[macro_use]
mod tracing_setup;
use tracing::{debug, error, info, trace, warn};

// Create a temporary directory for test files if it doesn't exist
fn ensure_temp_dir() -> std::io::Result<()> {
    let temp_dir = Path::new("tests/temp");
    if !temp_dir.exists() {
        fs::create_dir(temp_dir)?;
    }
    Ok()
}

/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = "debug")]
fn run_cursed_file(file_path: &str) -> io::Result<(String, bool)> {
    debug!("Running CURSED file: {}", file_path);
    let output = Command::new("./target/debug/cursed")
        .arg(file_path)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    // Combine stdout and stderr for debugging
    let combined_output = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
    
    let success = output.status.success();
    if success {
        debug!("Command executed successfully");
    } else {
        warn!(status = ?output.status, "Command execution failed");
    }

    // Return the combined output and success status
    Ok((combined_output, success))
}

// Helper to check for expected output in the command result
fn assert_output_contains(output: &str, expected: &str) -> Result<(), String> {
    if output.contains(expected) {
        Ok(())
    } else {
        Err(format!("Expected output to contain '{}', but got:\n{}", expected, output))
    }
}

// Helper to run a test with an invalid map that should gracefully recover
fn run_map_recovery_test(code: &str, expected_value: i64) -> Result<(), String> {
    // Initialize tracing for the test
    tracing_setup::init_test_tracing();
    info!("Running map recovery test with code:\n{}", code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let test_file = format!("tests/temp/map_recovery_test_{}.csd", generate_id());
    
    // Add a print statement to output the result
    let code_with_print = format!("{}
fr Print the result for testing
printn(yolo)\n", code);
    
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!("Failed to write test file: {}", e))?;
    
    let (output, success) = run_cursed_file(&test_file)
        .map_err(|e| format!("Failed to run test: {}", e))?;
    
    // For recovery tests, we expect successful compilation even with invalid maps
    if !success {
        return Err(format!("Test execution failed (should recover):\n{}", output));
    }
    
    // Check if the output contains the expected value
    let expected_str = expected_value.to_string();
    assert_output_contains(&output, &expected_str)
}

// Helper for container recovery tests
fn run_container_recovery_test(code: &str, expected_value: i64) -> Result<(), String> {
    // Initialize tracing for the test
    tracing_setup::init_test_tracing();
    info!("Running container recovery test with code:\n{}", code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let test_file = format!("tests/temp/container_recovery_test_{}.csd", generate_id());
    
    // Add a print statement to output the result
    let code_with_print = format!("{}
fr Print the result for testing
printn(yolo)\n", code);
    
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!("Failed to write test file: {}", e))?;
    
    let (output, success) = run_cursed_file(&test_file)
        .map_err(|e| format!("Failed to run test: {}", e))?;
    
    // For recovery tests, we expect successful compilation even with invalid containers
    if !success {
        return Err(format!("Test execution failed (should recover):\n{}", output));
    }
    
    // Check if the output contains the expected value
    let expected_str = expected_value.to_string();
    assert_output_contains(&output, &expected_str)
}

#[test]
fn test_nil_map_recovery() -> Result<(), String> {
    // Test iteration over a nil map that should recover gracefully
    let code = r#"
        slay main() lit {
            sus my_map tea[lit]lit = cap  fr Nil map
            sus count lit = 0
            
            fr Should handle nil map gracefully
            bestie key, value := flex my_map {
                count = count + 1  fr This won't execute but shouldn't crash
            }
            
            yolo 42  fr Should reach here after recovering
        }
    "#;
    
    // Should return 42 after successfully recovering
    run_map_recovery_test(code, 42)
}

#[test]
fn test_wrong_type_map_recovery() -> Result<(), String> {
    // Test iteration over a non-map type that should recover gracefully
    let code = r#"
        slay main() lit {
            sus not_a_map lit = 123  fr Not a map at all
            sus count lit = 0
            
            fr Should handle non-map gracefully
            bestie key, value := flex not_a_map {
                count = count + 1  fr This won't execute but shouldn't crash
            }
            
            yolo 42  fr Should reach here after recovering
        }
    "#;
    
    // Should return 42 after successfully recovering
    run_map_recovery_test(code, 42)
}

#[test]
fn test_type_mismatch_map_recovery() -> Result<(), String> {
    // Test iteration over a map with different key/value types than expected
    let code = r#"
        slay main() lit {
            sus string_map = {
                "one": "value1",
                "two": "value2"
            }
            sus sum lit = 0
            
            fr Should handle type mismatches gracefully
            bestie key, value := flex string_map {
                fr Treating string values as numbers
                lowkey value != cap {
                    sum = sum + 1  fr Just count valid entries
                }
            }
            
            yolo sum  fr Should be 2 after counting entries
        }
    "#;
    
    // Should return 2 (number of entries) after recovering from type mismatches
    run_map_recovery_test(code, 2)
}

#[test]
fn test_nil_container_recovery() -> Result<(), String> {
    // Test iteration over a nil array/slice that should recover gracefully
    let code = r#"
        slay main() lit {
            sus my_array []lit = cap  fr Nil array
            sus count lit = 0
            
            fr Should handle nil array gracefully
            bestie item := flex my_array {
                count = count + 1  fr This won't execute but shouldn't crash
            }
            
            yolo 42  fr Should reach here after recovering
        }
    "#;
    
    // Should return 42 after successfully recovering
    run_container_recovery_test(code, 42)
}

#[test]
fn test_wrong_type_container_recovery() -> Result<(), String> {
    // Test iteration over a non-container type that should recover gracefully
    let code = r#"
        slay main() lit {
            sus not_an_array lit = 123  fr Not an array at all
            sus count lit = 0
            
            fr Should handle non-array gracefully
            bestie item := flex not_an_array {
                count = count + 1  fr This won't execute but shouldn't crash
            }
            
            yolo 42  fr Should reach here after recovering
        }
    "#;
    
    // Should return 42 after successfully recovering
    run_container_recovery_test(code, 42)
}

#[test]
fn test_out_of_bounds_recovery() -> Result<(), String> {
    // Test handling out-of-bounds access in array iteration
    let code = r#"
        slay main() lit {
            sus broken_array = [1, 2, 3]
            sus sum lit = 0
            
            fr Force out of bounds by manipulating internal length
            fr (This test is a bit contrived since we can't easily force out-of-bounds in safe code)
            bestie i := flex 5 {  fr Iterate more times than array length
                lowkey i < broken_array.length() {
                    sum = sum + broken_array[i]  fr Safe access
                } highkey {
                    sum = sum + 0  fr Recovered access for out-of-bounds
                }
            }
            
            yolo sum  fr Should be 1+2+3 = 6
        }
    "#;
    
    // Should return 6 after handling potential out-of-bounds access
    run_container_recovery_test(code, 6)
}

#[test]
fn test_mixed_recovery_strategies() -> Result<(), String> {
    // Test using both map and container recovery in the same function
    let code = r#"
        slay main() lit {
            sus null_map tea[lit]lit = cap
            sus null_array []lit = cap
            sus count lit = 0
            
            fr Handle map iteration recovery
            bestie key, value := flex null_map {
                count = count + 1  fr Won't execute
            }
            
            fr Handle container iteration recovery
            bestie item := flex null_array {
                count = count + 1  fr Won't execute
            }
            
            yolo 84  fr Should reach here after double recovery
        }
    "#;
    
    // Should return 84 after recovering from both iterations
    run_container_recovery_test(code, 84)
}

#[test]
fn test_iterator_advancement_error_recovery() -> Result<(), String> {
    // Test recovery from errors when advancing map iterator
    let code = r#"
        slay main() lit {
            sus small_map = {
                "a": 1,
                "b": 2
            }
            sus count lit = 0
            
            fr Iterator advancement errors would be handled
            bestie key, value := flex small_map {
                count = count + value
                
                fr If an advancement error happened here, we'd still continue
            }
            
            yolo count  fr Should be 1+2 = 3
        }
    "#;
    
    // Should return 3 after successfully iterating the map
    run_map_recovery_test(code, 3)
}

#[test]
fn test_dynamic_container_recovery() -> Result<(), String> {
    // Test dynamic container creation with potential errors
    let code = r#"
        slay createContainer(size lit) []lit {
            lowkey size < 0 {
                yolo cap  fr Return nil for invalid size
            }
            
            sus result []lit = []
            bestie i := flex size {
                result.push(i * 2)
            }
            
            yolo result
        }
        
        slay main() lit {
            sus sum lit = 0
            
            fr Container from valid size
            sus valid = createContainer(3)
            bestie item := flex valid {
                sum = sum + item
            }
            
            fr Container from invalid size - should recover
            sus invalid = createContainer(-1)
            bestie item := flex invalid {
                sum = sum + item  fr This won't execute but shouldn't crash
            }
            
            yolo sum  fr Should be 0+2+4 = 6
        }
    "#;
    
    // Should return 6 after handling both valid and invalid containers
    run_container_recovery_test(code, 6)
}