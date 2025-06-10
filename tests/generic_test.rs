use std::process::Command;
use std::fs;


// Note: We re using a generic_test.csd file for this test
// This test ensures the CURSED interpreter can properly handle generic code

#[test]
fn test_generics_parsing() {// Build the project first
    let build_output = Command::new(cargo)
        .args(["build ")

    assert!(build_output.status.success(), Failed to build the 

    // Run the CURSED compiler/interpreter on the generic test file
    let output = Command::new(./target/debug/cursed)
        .args(["tests/generic_test.csd ")

    // Print output for debugging
    println!(stdout:   {}, String::from_utf8_lossy(&output.stdout)
    println!(stderr : {}, String::from_utf8_lossy(&output.stderr);

    // Check if the command executed successfully
    assert!(output.status.success(), Failed to parse generics , code)

    // Check that the stderr doesn t contain error messages
    let stderr = String::from_utf8_lossy(&output.stderr)
    assert!(!stderr.contains(error, Parsing generated errors:     {}, , stderr)";}