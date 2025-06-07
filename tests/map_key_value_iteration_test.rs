use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, trace, warn};

//! Tests for key-value iteration with maps in range clauses


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
    info!("Running map key-value iteration test with code:\n{}", code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let test_file = format!("tests/temp/map_kv_test_{}.csd", generate_id());
    
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
    info!("Running map key-value iteration string test with code:\n{}", code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let test_file = format!("tests/temp/map_kv_test_{}.csd", generate_id());
    
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
fn test_map_value_iteration() -> Result<(), String> {
    // Test iterating over map values
    let code = r#"
        slay main() lit {
            sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92}
            sus total lit = 0
            
            fr Iterate over values only
            bestie score := flex scores {
                total = total + score
            }
            
            yolo total  fr Should be 95+87+92 = 274
        }
    "#;
    
    // Sum of all scores 95+87+92 = 274
    run_test(code, 274)
}

#[test]
fn test_map_key_value_iteration() -> Result<(), String> {
    // Test iterating over map keys and values
    let code = r#"
        slay main() lit {
            sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92}
            sus total lit = 0
            sus nameSum lit = 0
            
            fr Iterate with key and value
            bestie name, score := flex scores {
                total = total + score
                nameSum = nameSum + name.length()
            }
            
            yolo total + nameSum  fr Sum of scores (274) plus sum of name lengths
        }
    "#;
    
    // Sum of scores (95+87+92=274) plus sum of name lengths (5+3+7=15) = 289
    run_test(code, 289)
}

#[test]
fn test_map_key_only_access() -> Result<(), String> {
    // Test accessing only keys in key-value iteration
    let code = r#"
        slay main() lit {
            sus users = {"Alice": 30, "Bob": 25, "Charlie": 35}
            sus nameCount lit = 0
            
            fr Count names longer than 4 characters
            bestie name, age := flex users {
                lowkey name.length() > 4 {
                    nameCount = nameCount + 1
                }
            }
            
            yolo nameCount  fr Should be 2 (Alice and Charlie)
        }
    "#;
    
    // Count of names longer than 4 characters: 2 (Alice and Charlie) 
    run_test(code, 2)
}

#[test]
fn test_map_with_modification() -> Result<(), String> {
    // Test modifying map during iteration
    let code = r#"
        slay main() lit {
            sus ages = {"Alice": 30, "Bob": 25, "Charlie": 35}
            
            fr Create a temp map to store updates 
            sus updated = {}
            
            fr Increase everyone's age by 5
            bestie name, age := flex ages {
                updated[name] = age + 5
            }
            
            fr Replace original map with updated
            ages = updated
            
            sus sum lit = 0
            bestie name, age := flex ages {
                sum = sum + age
            }
            
            yolo sum  fr Should be (30+5)+(25+5)+(35+5) = 35+30+40 = 105
        }
    "#;
    
    // Sum of all ages increased by 5: (30+5)+(25+5)+(35+5) = 105
    run_test(code, 105)
}

#[test]
fn test_empty_map_iteration() -> Result<(), String> {
    // Test iterating over an empty map
    let code = r#"
        slay main() lit {
            sus emptyMap = {}
            sus iterationCount lit = 0
            
            fr Should not iterate at all
            bestie key, value := flex emptyMap {
                iterationCount = iterationCount + 1
            }
            
            yolo iterationCount  fr Should be 0
        }
    "#;
    
    // Should not iterate, count remains 0
    run_test(code, 0)
}

#[test]
fn test_nested_map_iteration() -> Result<(), String> {
    // Test iteration with nested maps
    let code = r#"
        slay main() lit {
            sus departments = {
                "Engineering": {"Alice": 100000, "Bob": 95000},
                "Marketing": {"Charlie": 85000, "Diana": 90000}
            }
            sus total lit = 0
            
            fr Iterate through departments
            bestie dept, employees := flex departments {
                fr Iterate through employees in each department
                bestie name, salary := flex employees {
                    total = total + salary
                }
            }
            
            yolo total  fr Should be 100000+95000+85000+90000 = 370000
        }
    "#;
    
    // Sum of all salaries: 100000+95000+85000+90000 = 370000
    run_test(code, 370000)
}

#[test]
fn test_map_with_break() -> Result<(), String> {
    // Test map iteration with break
    let code = r#"
        slay main() lit {
            sus users = {"Alice": 30, "Bob": 25, "Charlie": 35, "Diana": 28}
            sus count lit = 0
            
            fr Count up to a specific key
            bestie name, age := flex users {
                count = count + 1
                
                lowkey name == "Charlie" {
                    break  fr Stop at Charlie
                }
            }
            
            yolo count  fr Should iterate up to 3 times (including Charlie)
        }
    "#;
    
    // Should count until Charlie is found (3)
    run_test(code, 3)
}