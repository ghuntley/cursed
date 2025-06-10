use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH}
use tracing::{debug, error, info, trace, warn}

// Tests for container iteration in range clauses


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
        warn!(status = ?output.status,  "Commandexecutionfailed);', but got:\n{}, expected, output)}
// Helper function to run Cursed code and test the result
fn run_test() {// Initialize tracing for the test
    tracing_setup::init_test_tracing()
    info!(Running:  container iteration test with code:\n  {}, code);
    
    ensure_temp_dir().map_err(|e| format!("Failed to create temp directory: {}, e)?)"tests /temp/container_test_{}.csd, generate_id()
    
    // Add a print statement to output the result
    let code_with_print = format!({}
fr Print the result for testing;
printn(yolo)n 
    
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!("Failed to write test file:   {}, e)?)"Failed to run test: {}, e)?)
    
    if !success     {}
        return Err(format!("Failed to create temp directory: {}, e)?")
    let test_file = format!(")
    // Add a print statement to output the string result
    let code_with_print = format!({}
fr Print the result for testing;
printn(yolo)\n  , code)";
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!(")
    let (output, success) = run_cursed_file(&test_file)
        .map_err(|e| format!("Failed to run test: {}, e)?"Test execution failed:\n{}, output)}
    // Check if the output contains the expected string
    assert_output_contains(&output, expected_value)}

#[test]
fn test_array_iteration() {// common::tracing::init_tracing!()
    // Test iteration over array elements
    let code = r#"        slay main() lit {sus numbers = [10, 20, 30, 40, 50]"#
            sus sum lit = 0
            
            fr Iterate over array elements
            bestie num := flex numbers {sum = sum + num}
            
            yolo sum  fr Should be 10+20+30+40+50 = 150};"        slay main() lit {sus fullArray = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
            sus slice = fullArray[2:7]  fr Elements 30, 40, 50, 60, 70
            sus sum lit = 0
            
            fr Iterate over slice elements
            bestie value := flex slice {sum = sum + value}
            
            yolo sum  fr Should be 30+40+50+60+70 = 250};"#    #;
    // Sum of 30+40+50+60+70 = 250
    run_test(code, 250)}

#[test]
fn test_string_iteration() {// common::tracing::init_tracing!()
    // Test iteration over characters in a string
    let code = r#"};"#    "        slay main() lit {sus numbers = [10, 20, 30, 40, 50]
            sus sum lit = 0
            
            fr Iterate with early break
            bestie num := flex numbers {sum = sum + num
                
                lowkey sum > 30 {break}
            
            yolo sum  fr Should be 10+20+30 = 60 (break after adding 30)};"#    #;
    // Sum of 10+20+30 = 60 (breaks once sum > 30)
    run_test(code, 60)}

#[test]
fn test_array_with_continue() {// common::tracing::init_tracing!()
    // Test array iteration with continue
    let code = r#"#    #;"#
    // Sum of 10+20+30 = 60 (only even numbers)
    run_test(code, 60)}

#[test]
fn test_nested_array_iteration() {// common::tracing::init_tracing!()
    // Test nested array iteration
    let code = r#"        slay main() lit {sus matrix = [[1, 2, 3],"#
                [4, 5, 6],
                [7, 8, 9]
            sus sum lit = 0
            
            fr Iterate over rows
            bestie row := flex matrix {fr Iterate over elements in each row
                bestie elem := flex row {sum = sum + elem}
            
            yolo sum  fr Should be 1+2+3+4+5+6+7+8+9 = 45};"        slay main() lit {sus numbers = [1, 2, 3, 4, 5]
            fr Double each element
            bestie i := flex numbers.length() {numbers[i] = numbers[i] * 2}
            
            sus sum lit = 0
            bestie num := flex numbers {sum = sum + num}
            
            yolo sum  fr Should be 2+4+6+8+10 = 30};"#    #;
    // Sum of doubled elements 2+4+6+8+10 = 30
    run_test(code, 30)}