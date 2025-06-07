use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, trace, warn};

//! Tests for container iteration in range clauses


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

// Create a temporary directory for test files if it doesn't exist
fn ensure_temp_dir() -> std::io::Result<()> {
    let temp_dir = Path::new("tests/temp");
    if !temp_dir.exists() {
        fs::create_dir(temp_dir)?;
    }
    Ok(())
}

/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = "debug")]
fn run_cursed_file(file_path: &str) -> io::Result<(String, bool)> {
    debug!("Running CURSED file: {}", file_path);
    let output = Command::new("devenv")
        .args(&["shell", "./target/debug/cursed", file_path])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string());
    let stderr = String::from_utf8_lossy(&output.stderr).to_string());

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

// Helper function to run Cursed code and test the result
fn run_test(code: &str, expected_value: i64) -> Result<(), String> {
    // Initialize tracing for the test
    tracing_setup::init_test_tracing();
    info!("Running container iteration test with code:\n{}", code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let test_file = format!("tests/temp/container_test_{}.csd", generate_id());
    
    // Add a print statement to output the result
    let code_with_print = format!("{}
fr Print the result for testing
printn(yolo)\n", code);
    
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!("Failed to write test file: {}", e))?;
    
    let (output, success) = run_cursed_file(&test_file)
        .map_err(|e| format!("Failed to run test: {}", e))?;
    
    if !success {
        return Err(format!("Test execution failed:\n{}", output));
    }
    
    // Check if the output contains the expected value
    let expected_str = expected_value.to_string());
    assert_output_contains(&output, &expected_str)
}

// Helper for string return values
fn run_string_test(code: &str, expected_value: &str) -> Result<(), String> {
    // Initialize tracing for the test
    tracing_setup::init_test_tracing();
    info!("Running container iteration string test with code:\n{}", code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let test_file = format!("tests/temp/container_test_{}.csd", generate_id());
    
    // Add a print statement to output the string result
    let code_with_print = format!("{}
fr Print the result for testing
printn(yolo)\n", code);
    
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!("Failed to write test file: {}", e))?;
    
    let (output, success) = run_cursed_file(&test_file)
        .map_err(|e| format!("Failed to run test: {}", e))?;
    
    if !success {
        return Err(format!("Test execution failed:\n{}", output));
    }
    
    // Check if the output contains the expected string
    assert_output_contains(&output, expected_value)
}

#[test]
fn test_array_iteration() -> Result<(), String> {
    // Test iteration over array elements
    let code = r#"
        slay main() lit {
            sus numbers = [10, 20, 30, 40, 50]
            sus sum lit = 0
            
            fr Iterate over array elements
            bestie num := flex numbers {
                sum = sum + num
            }
            
            yolo sum  fr Should be 10+20+30+40+50 = 150
        }
    "#;
    
    // Sum of 10+20+30+40+50 = 150
    run_test(code, 150)
}

#[test]
fn test_slice_iteration() -> Result<(), String> {
    // Test iteration over slice
    let code = r#"
        slay main() lit {
            sus fullArray = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
            sus slice = fullArray[2:7]  fr Elements 30, 40, 50, 60, 70
            sus sum lit = 0
            
            fr Iterate over slice elements
            bestie value := flex slice {
                sum = sum + value
            }
            
            yolo sum  fr Should be 30+40+50+60+70 = 250
        }
    "#;
    
    // Sum of 30+40+50+60+70 = 250
    run_test(code, 250)
}

#[test]
fn test_string_iteration() -> Result<(), String> {
    // Test iteration over characters in a string
    let code = r#"
        slay main() lit {
            sus text = "CURSED"
            sus count lit = 0
            
            fr Iterate over characters in string
            bestie char := flex text {
                count = count + 1
            }
            
            yolo count  fr Should be 6 (length of "CURSED")
        }
    "#;
    
    // Count of characters in "CURSED" = 6
    run_test(code, 6)
}

#[test]
fn test_array_with_break() -> Result<(), String> {
    // Test array iteration with break
    let code = r#"
        slay main() lit {
            sus numbers = [10, 20, 30, 40, 50]
            sus sum lit = 0
            
            fr Iterate with early break
            bestie num := flex numbers {
                sum = sum + num
                
                lowkey sum > 30 {
                    break
                }
            }
            
            yolo sum  fr Should be 10+20+30 = 60 (break after adding 30)
        }
    "#;
    
    // Sum of 10+20+30 = 60 (breaks once sum > 30)
    run_test(code, 60)
}

#[test]
fn test_array_with_continue() -> Result<(), String> {
    // Test array iteration with continue
    let code = r#"
        slay main() lit {
            sus numbers = [10, 15, 20, 25, 30]
            sus sum lit = 0
            
            fr Iterate with continue for odd values
            bestie num := flex numbers {
                lowkey num % 2 != 0 {
                    continue  fr Skip odd numbers
                }
                
                sum = sum + num
            }
            
            yolo sum  fr Should be 10+20+30 = 60 (only even numbers)
        }
    "#;
    
    // Sum of 10+20+30 = 60 (only even numbers)
    run_test(code, 60)
}

#[test]
fn test_nested_array_iteration() -> Result<(), String> {
    // Test nested array iteration
    let code = r#"
        slay main() lit {
            sus matrix = [
                [1, 2, 3],
                [4, 5, 6],
                [7, 8, 9]
            ]
            sus sum lit = 0
            
            fr Iterate over rows
            bestie row := flex matrix {
                fr Iterate over elements in each row
                bestie elem := flex row {
                    sum = sum + elem
                }
            }
            
            yolo sum  fr Should be 1+2+3+4+5+6+7+8+9 = 45
        }
    "#;
    
    // Sum of all elements 1+2+3+4+5+6+7+8+9 = 45
    run_test(code, 45)
}

#[test]
fn test_array_with_modification() -> Result<(), String> {
    // Test modifying array elements during iteration
    let code = r#"
        slay main() lit {
            sus numbers = [1, 2, 3, 4, 5]
            
            fr Double each element
            bestie i := flex numbers.length() {
                numbers[i] = numbers[i] * 2
            }
            
            sus sum lit = 0
            bestie num := flex numbers {
                sum = sum + num
            }
            
            yolo sum  fr Should be 2+4+6+8+10 = 30
        }
    "#;
    
    // Sum of doubled elements 2+4+6+8+10 = 30
    run_test(code, 30)
}