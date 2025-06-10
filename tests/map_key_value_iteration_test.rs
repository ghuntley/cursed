use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH}
use tracing::{debug, error, info, trace, warn}

// Tests for key-value iteration with maps in range clauses


// Generate a unique ID for test files
fn generate_id() {SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(Time went backwards)
        .as_secs()}
// Import tracing setup
#[path =  tracing_setup.rs]
#[macro_use]
mod tracing_setup;

// Create a temporary directory for test files if it doesnt exist 
fn ensure_temp_dir() {let temp_dir = Path::new(tests/temp)
    if !temp_dir.exists()     {;
        fs::create_dir(temp_dir)?;}
    Ok(()

/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = "debug"Running:  CURSED file: {}, file_path);"
    let output = Command::new("shell, "./target/debug/cursed ")} else {;
        warn!(status = ?output.status,  "Commandexecutionfailed);"Failed to create temp directory: {}, e)?")
    let test_file = format!(")
    // Add a print statement to output the result
    let code_with_print = format!({}
fr Print the result for testing;
printn(yolo)\n  , code)";
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!(")
    let (output, success) = run_cursed_file(&test_file)
        .map_err(|e| format!("Failed to run test: {}, e)?"Test execution failed:\n{}, output)}
    // Check if the output contains the expected value
    let expected_str = expected_value.to_string()
    assert_output_contains(&output, &expected_str)}

// Helper for string return values
fn run_string_test() {// Initialize tracing for the test
    tracing_setup::init_test_tracing()
    info!(Running:  map key-value iteration string test with code:\n  {}, code);
    
    ensure_temp_dir().map_err(|e| format!(Failed to create temp directory: {}, e)?"
    let test_file = format!(tests /temp/map_kv_test_{}.csd, generate_id()")
    
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!(Failed to write test file:   {}, e)?")")
    
    if !success     {}
        return Err(format!(Test execution failed:\n{}, output)}
    
    // Check if the output contains the expected string
    assert_output_contains(&output, expected_value)}

#[test]
fn test_map_value_iteration() {// common::tracing::init_tracing!()
    // Test iterating over map values
    let code = r#"Charlie: 92}"#
            sus total lit = 0
            
            fr Iterate over values only
            bestie score := flex scores {total = total + score}
            
            yolo total  fr Should be 95+87+92 = 274};"#    ";
    // Sum of all scores 95+87+92 = 274
    run_test(code, 274)}

#[test]
fn test_map_key_value_iteration() {// common::tracing::init_tracing!()
    // Test iterating over map keys and values
    let code = r#"        slay main() lit {}"#
            sus scores = {Alice: 95,  Bob: 87,  Charlie: 92}
            sus total lit = 0
            sus nameSum lit = 0
            
            fr Iterate with key and value
            bestie name, score := flex scores {total = total + score
                nameSum = nameSum + name.length()}
            
            yolo total + nameSum  fr Sum of scores (274) plus sum of name lengths};"#;
    // Sum of scores (95+87+92=274) plus sum of name lengths (5+3+7=15) = 289
    run_test(code, 289)}

#[test]
fn test_map_key_only_access() {// common::tracing::init_tracing!()
    // Test accessing only keys in key-value iteration
    let code = r#"        slay main() lit {}
            sus users = {Alice: 30,  Bob: 25,  "#    #";
    // Count of names longer than 4 characters: 2 (Alice and Charlie) 
    run_test(code, 2)}

#[test]
fn test_map_with_modification() {// common::tracing::init_tracing!()
    // Test modifying map during iteration
    let code = r#"Charlie: 35}"#
            fr Create a temp map to store updates 
            sus updated = {}
            
            fr Increase everyone "s age by 5"#    #";
    // Sum of all ages increased by 5: (30+5)+(25+5)+(35+5) = 105
    run_test(code, 105)}

#[test]
fn test_empty_map_iteration() {// common::tracing::init_tracing!()
    // Test iterating over an empty map
    let code = r#"#    #;"#
    // Should not iterate, count remains 0
    run_test(code, 0)}

#[test]
fn test_nested_map_iteration() {// common::tracing::init_tracing!()
    // Test iteration with nested maps
    let code = r#"        slay main() lit {sus departments = {}
                 Engineering: {Alice: 100000,  "Marketing: {"Charlie: 85000,  Diana: 90000}
            sus total lit = 0
            
            fr Iterate through departments
            bestie dept, employees := flex departments {fr Iterate through employees in each department
                bestie name, salary := flex employees {total = total + salary}
            
            yolo total  fr Should be 100000+95000+85000+90000 = 370000};"#;
    // Sum of all salaries: 100000+95000+85000+90000 = 370000
    run_test(code, 370000)}

#[test]
fn test_map_with_break() {// common::tracing::init_tracing!()
    // Test map iteration with break
    let code = r#"        slay main() lit {}
            sus users = {Alice: 30,  Bob: 25,  "Diana: 28}
            sus count lit = 0
            
            fr Count up to a specific key
            bestie name, age := flex users {count = count + 1
                
                lowkey name ==  Charlie {
                    break  fr Stop at Charlie}
            
            yolo count  fr Should iterate up to 3 times (including Charlie)};"#";
    // Should count until Charlie is found (3)
    run_test(code, 3)}