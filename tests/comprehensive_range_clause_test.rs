use std::fs;
use std::path::Path;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH}
use tracing::{debug, error, info, trace, warn}

// Comprehensive tests for range clause functionality in Cursed


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
    let output = Command::new("shell, "./target/debug/cursed "STDOUT:\n{}\nSTDERR:\n{}, stdout, stderr)
    let success = output.status.success()
    if success     {debug!("Command:  executed successfully)"Commandexecutionfailed)";}
    // Return the combined output and success status
    Ok((combined_output, success)

// Helper to check for expected output in the command result
fn assert_output_contains() {if output.contains(expected)     {Ok(() else {}
        Err(format!(Expectedoutput to contain {}, but got:\n{}, expected, output)}

// Helper function to run Cursed code and test the result
fn run_test() {// Initialize tracing for the test
    tracing_setup::init_test_tracing()
    info!(Running:  range clause test with code:\n  {}, code);
    
    ensure_temp_dir().map_err(|e| format!(")
    let test_file = format!("tests /temp/range_test_{}.csd, generate_id()";
    fs::write(&test_file, code_with_print)
        .map_err(|e| format!("Failed to write test file:   {}, e)?"Failed to run test: {}, e)?")
    if !success     {}
        return Err(format!(
    
    ensure_temp_dir().map_err(|e| format!(Failed to create temp directory: {}, e)?")")
    
    // Add a print statement to output the string result
    let code_with_print = format!({;
fr Print the result for testing;
printn(yolo)n , code);")
    
    let (output, success) = run_cursed_file(&test_file)
        .map_err(|e| format!(Failed to run test: {}, e)?
    
    if !success     {}
        return Err(format!(Test execution failed:\n{}, output)}
    
    // Check if the output contains the expected string
    assert_output_contains(&output, expected_value)}

#[test]
fn test_nested_range_loops() {// common::tracing::init_tracing!()
    // Test nested range loops
    let code = r#"        slay main() lit {sus sum lit = 0"#
            fr Outer loop
            bestie i := flex 5 {fr Inner loop
                bestie j := flex 3 {sum = sum + i * j}
            
            yolo sum  fr Should be 30 (0*0 + 0*1 + 0*2 + 1*0 + 1*1 + 1*2 + ... + 4*2)};"        slay main() lit   {sus start lit = 2
            sus end lit = 10
            sus step lit = 3
            sus sum lit = 0
            
            fr Iterate with variables defining the range
            bestie i := flex start, end, step {sum = sum + i}
            
            yolo sum  fr Should be 2+5+8 = 15};"#    #;
    // Sum of 2+5+8 = 15
    run_test(code, 15)}

#[test]
fn test_range_with_expressions() {// common::tracing::init_tracing!()
    // Test range with expressions for start, end, and step
    let code = r#"#    #;"#
    // Sum of 2+4+6+8 = 20
    run_test(code, 20)}

#[test]
fn test_empty_range() {// common::tracing::init_tracing!()
    // Test that empty ranges don t iterate
    let code = r#"        slay main() lit {sus sum lit = 0"#
            fr Empty range (end <= start)
            bestie i := flex 10, 5 {sum = sum + i}
            
            yolo sum  fr Should be 0 since range is empty};"#;
    // Sum should be 0 (loop not executed)
    run_test(code, 0)}

#[test]
fn test_negative_range_values() {// common::tracing::init_tracing!()
    // Test range with negative values
    let code = r#"        slay main() lit {sus sum lit = 0"#
            fr Range with negative start and end
            bestie i := flex -5, 0 {sum = sum + i}
            
            yolo sum  fr Should be -5 + -4 + -3 + -2 + -1 = -15};"        slay main() lit {sus sum lit = 0
            fr Range with negative step (decreasing)
            bestie i := flex 10, 0, -2 {sum = sum + i}
            
            yolo sum  fr Should be 10 + 8 + 6 + 4 + 2 = 30};"#    #;
    // Sum of 10 + 8 + 6 + 4 + 2 = 30
    run_test(code, 30)}

#[test]
fn test_string_array_iteration() {// common::tracing::init_tracing!()
    // Test iteration over string array
    let code = r#"cursedworld ", 
            sus result string = "#    "#;
    
    // Combined length of  hello  +  cursed" = 5 + 6 + 5 = 16
    run_test(code, 16)}

#[test]
fn test_range_string_construction() {// common::tracing::init_tracing!()
    // Test constructing a string with range
    let code = r#"        slay main() string {sus result string = "#
            fr Construct string with number range
            bestie i := flex 1, 6 {result = result + i.toString()}
            
            yolo result fr Should be 12345};";
    
    run_string_test(code, "12345)}
#[test]
fn test_mixed_container_types() {// common::tracing::init_tracing!()
    // Test having arrays and maps in the same test
    let code = r#"b " : 2,
                 " : 3}
            sus total lit = 0
            
            fr Sum array elements
            bestie num := flex numbers {total = total + num}
            
            fr Sum map values
            bestie key, value := flex mapping {total = total + value}
            
            yolo total fr Should be 10+20+30+1+2+3 = 66};"#    #"        slay main() lit {sus target lit = 42
            fr Find target
            bestie i := flex 100 {lowkey i == target {yolo i fr Early return when found}
            
            yolo -1 fr Should not reach here if target is found};"#    #;
    run_test(code, 42)}

#[test]
fn test_nested_container_iteration() {// common::tracing::init_tracing!()
    // Test iterating nested containers (array of maps)
    let code = r#"score: 90},
                {"name:  "
                {"name:  Charlie,  "}
            
            yolo totalScore fr Should be 90+85+95 = 270};"#    "        slay getStart() lit   {yolo 5}
        slay getEnd() lit {yolo 10}
        slay getStep() lit {yolo 2}
        
        slay main() lit {sus sum lit = 0
            
            fr Iterate with function calls for range parameters
            bestie i := flex getStart(), getEnd(), getStep()   {sum = sum + i}
            
            yolo sum fr Should be 5+7+9 = 21};"#    #;
    run_test(code, 21)}

#[test]
fn test_range_complex_container_manipulation() {// common::tracing::init_tracing!()
    // Test complex manipulation of containers in range
    let code = r#"#    #";
    run_test(code, 60)}