use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, trace, warn};

// Tests for the enhanced range clause error recovery functionality
// 
// This module tests the ability to recover from various types of errors
// in range clauses and range expressions.


// Import tracing setup 
#[path = "tracing_setup.rs"]
#[macro_use]
mod tracing_setup;

// Generate a unique ID for test files
fn generate_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

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
    let output = Command::new("cargo")
        .args(&["run", "--bin", "cursed", "--", file_path])
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

// Helper function to run Cursed code and test if it compiles successfully despite errors
fn test_error_recovery(code: &str, expected_message: Option<&str>) -> Result<(), String> {
    // Initialize tracing for the test
    tracing_setup::init_test_tracing();
    info!("Running range clause error recovery test with code:\n{}", code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let test_file = format!("tests/temp/range_recovery_test_{}.csd", generate_id());
    
    fs::write(&test_file, code)
        .map_err(|e| format!("Failed to write test file: {}", e))?;
    
    let (output, success) = run_cursed_file(&test_file)
        .map_err(|e| format!("Failed to run test: {}", e))?;
    
    // With error recovery, the compilation should succeed
    if !success {
        return Err(format!("Test execution failed despite error recovery:\n{}", output));
    }
    
    // If an expected message is provided, check that it appears in the output
    if let Some(expected) = expected_message {
        return assert_output_contains(&output, expected);
    }
    
    Ok(())
}

#[test]
fn test_invalid_range_values_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from invalid range values
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Invalid range start (this will be replaced with 0)
            bestie i := flex "not a number", 5 {
                sum = sum + i
            }
            
            yolo sum  fr Should be sum of 0 to 4 = 10
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_missing_end_value_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from missing end value
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Missing end value (will be replaced with 10)
            bestie i := flex 0, {
                sum = sum + i
            }
            
            yolo sum  fr Should run despite the syntax error
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_negative_step_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from negative step that would lead to infinite loop
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Infinite loop prevention (step is negative but start < end)
            bestie i := flex 0, 10, -1 {
                sum = sum + i
            }
            
            yolo sum  fr Should terminate normally
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_zero_step_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from zero step that would cause infinite loop
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Zero step recovery (would cause infinite loop)
            bestie i := flex 0, 5, 0 {
                sum = sum + i
            }
            
            yolo sum  fr Should terminate normally
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_type_error_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from type errors in range parameters
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Type error recovery in container field
            sus container = {"value": "not a number"}
            bestie i := flex container["value"] {
                sum = sum + i  fr This would fail without recovery
            }
            
            yolo sum  fr Should compile and run despite type error
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_nil_container_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from nil container
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Nil container recovery
            sus container tea[] = cap  fr Nil array
            bestie value := flex container {
                sum = sum + value  fr This won't execute due to recovery
            }
            
            yolo sum  fr Should be 0 (empty iteration)
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_complex_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from multiple errors in the same range clause
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Multiple errors (missing comma, invalid step)
            bestie i := flex 1 10 "step" {
                sum = sum + i
            }
            
            fr This should run with some fallback range
            yolo sum
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_range_variable_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from missing range variable
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Missing variable name (will use fallback name)
            bestie := flex 5 {
                sum = sum + 1  fr Can't use the variable but loop still runs
            }
            
            yolo sum  fr Should be 5
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_map_key_value_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from errors in map key-value iteration
    let code = r#""
        slay main() lit {
            sus sum lit = 0
            
            fr Invalid map but iteration should work with empty result
            sus mymap tea[lit]lit = [1, 2, 3]  fr This is an array, not a map
            
            bestie key, value := flex mymap {
                sum = sum + value
            }
            
            yolo sum  fr Should be 0 (empty iteration due to recovery)
        }
    "#";
    
    test_error_recovery(code, None)
}

#[test]
fn test_out_of_bounds_recovery() -> Result<(), String> {
    // init_tracing!();
    // Test recovery from out of bounds access in array iteration
    let code = r#""
        slay main() lit {
            sus arr = [10, 20, 30]
            sus idx lit = 5  fr Out of bounds
            
            fr Out of bounds index used in range
            bestie i := flex arr[idx] {
                yolo i  fr This shouldn't be reachable due to recovery
            }
            
            yolo 42  fr Should reach here
        }
    "#";
    
    test_error_recovery(code, Some("42"))
}