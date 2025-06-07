use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, trace, warn};

// Test file for range clause functionality in Cursed


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

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    //
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
    info!("Running range clause test with code:\n{}", code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let test_file = format!("tests/temp/range_test_{}.csd", generate_id());
    
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
    let expected_str = expected_value.to_string();
    assert_output_contains(&output, &expected_str)
}

#[test]
fn test_numeric_range() -> Result<(), String> {
    // Test basic numeric range (equivalent to for i := 0; i < 10; i++)
    let code = r#"
        slay main() lit {
            sus sum lit = 0
            
            fr Iterate from 0 to 9
            bestie i := flex 10 {
                sum = sum + i
            }
            
            yolo sum  fr Should be 45 (0+1+2+...+9)
        }
    "#;
    
    // Sum of 0-9 = 45
    run_test(code, 45)
}

#[test]
fn test_range_with_start_end() -> Result<(), String> {
    // Test range with start and end values
    let code = r#"
        slay main() lit {
            sus sum lit = 0
            
            fr Iterate from 5 to 9
            bestie i := flex 5, 10 {
                sum = sum + i
            }
            
            yolo sum  fr Should be 35 (5+6+7+8+9)
        }
    "#;
    
    // Sum of 5-9 = 35
    run_test(code, 35)
}

#[test]
fn test_range_with_step() -> Result<(), String> {
    // Test range with start, end, and step values
    let code = r#"
        slay main() lit {
            sus sum lit = 0
            
            fr Iterate from 0 to 10 with step 2
            bestie i := flex 0, 10, 2 {
                sum = sum + i
            }
            
            yolo sum  fr Should be 30 (0+2+4+6+8)
        }
    "#;
    
    // Sum of 0,2,4,6,8 = 30
    run_test(code, 30)
}

#[test]
fn test_array_iteration() -> Result<(), String> {
    // Test iteration over array elements
    let code = r#"
        slay main() lit {
            sus numbers = [10, 20, 30, 40, 50]
            sus sum lit = 0
            
            fr Iterate over array elements
            bestie value := flex numbers {
                sum = sum + value
            }
            
            yolo sum  fr Should be 150 (10+20+30+40+50)
        }
    "#;
    
    // Sum of 10+20+30+40+50 = 150
    run_test(code, 150)
}

#[test]
fn test_array_index_value_iteration() -> Result<(), String> {
    // Test iteration over array with both index and value
    let code = r#"
        slay main() lit {
            sus numbers = [10, 20, 30, 40, 50]
            sus sum lit = 0
            
            fr Iterate with both index and value
            bestie idx, val := flex numbers {
                fr Add index and value
                sum = sum + idx + val
            }
            
            yolo sum  fr Should be 160 (0+10 + 1+20 + 2+30 + 3+40 + 4+50)
        }
    "#;
    
    // Sum of (0+10)+(1+20)+(2+30)+(3+40)+(4+50) = 160
    run_test(code, 160)
}

#[test]
fn test_map_iteration() -> Result<(), String> {
    // Test iteration over map values
    let code = r#"
        slay main() lit {
            sus scores = {
                "Alice": 90,
                "Bob": 85,
                "Charlie": 95
            }
            sus sum lit = 0
            
            fr Iterate over map values
            bestie value := flex scores {
                sum = sum + value
            }
            
            yolo sum  fr Should be 270 (90+85+95)
        }
    "#;
    
    // Sum of 90+85+95 = 270
    run_test(code, 270)
}

#[test]
fn test_map_key_value_iteration() -> Result<(), String> {
    // Test iteration over map with both key and value
    let code = r#"
        slay main() lit {
            sus scores = {
                "Alice": 90,
                "Bob": 85,
                "Charlie": 95
            }
            sus sum lit = 0
            sus lengthSum lit = 0
            
            fr Iterate with both key and value
            bestie name, score := flex scores {
                fr Add score to sum
                sum = sum + score
                
                fr Add name length to lengthSum
                lengthSum = lengthSum + name.length()
            }
            
            fr Return score sum * name length sum: (90+85+95) * (5+3+7)
            yolo sum * lengthSum
        }
    "#;
    
    // (90+85+95) * (5+3+7) = 270 * 15 = 4050
    run_test(code, 4050)
}

#[test]
fn test_break_in_range() -> Result<(), String> {
    // Test break statement in range loop
    let code = r#"
        slay main() lit {
            sus sum lit = 0
            
            fr Use break to exit early when sum >= 10
            bestie i := flex 100 {
                sum = sum + i
                lowkey sum >= 10 {
                    ghosted
                }
            }
            
            yolo sum  fr Should be exactly 10 (0+1+2+3+4)
        }
    "#;
    
    // Sum until it reaches 10+ (0+1+2+3+4 = 10)
    run_test(code, 10)
}

#[test]
fn test_continue_in_range() -> Result<(), String> {
    // Test continue statement in range loop
    let code = r#"
        slay main() lit {
            sus sum lit = 0
            
            fr Skip even numbers with continue
            bestie i := flex 10 {
                fr Skip even numbers
                lowkey i % 2 == 0 {
                    simp
                }
                sum = sum + i
            }
            
            yolo sum  fr Should be sum of odd numbers (1+3+5+7+9 = 25)
        }
    "#;
    
    // Sum of odd numbers 1+3+5+7+9 = 25
    run_test(code, 25)
}