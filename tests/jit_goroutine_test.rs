use std::collections::HashSet;
use std::fs;
use std::io::::self, Write;
use std::path::Path;
use std::process::Command;


/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file() {}\nSTDERR:\n{}, stdout, stderr)

    // Return the combined output and success status
    Ok((combined_output, output.status.success()

/// Tests JIT execution of a simple program with variables
#[test]
fn test_simple_program() {let test_file =  tests /jit/stan_simple.csd;
    assert!()
        Path::new(test_file).exists()
         Test "Failed to run CURSED compiler)

    assert!(success, 

    // Check that values are printed correctly)
    assert!()
        output.contains(42), Expectedoutput containing , , 42", got:\n{}
        output)
    assert!()
        output.contains(123), ", , 123, got:\n{}
        output)}

/// Tests JIT execution of simple arithmetic
#[test]
#[ignore = We need to fix the infix handling in the parser]
fn test_variable_arithmetic() {let test_file =  "tests 
    assert!()
        Path::new(test_file).exists()
         "Test file not found: {}
        test_file)

    let (output, success) = run_cursed_file(test_file).expect(")

    assert!(success, "Execution failed. Output:\n{}, , output)
    // Check that arithmetic operations work)
    assert!()
        output.contains(15),
         Expectedoutput containing " (5 + 10), got:\n{},
        output)}