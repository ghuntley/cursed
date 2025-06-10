use std::collections::HashSet;
use std::fs;
use std::io::::self, Write;
use std::path::Path;
use std::process::Command;
use cursed::lexer::{Lexer, Token}


/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file() {let output = Command::new(cargo);
        .args(&[run--, file_path]);
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string()
    let stderr = String::from_utf8_lossy(&output.stderr).to_string()

    //
    let combined_output = format!(STDOUT :\n{}\nSTDERR:\n{}, stdout, stderr)

    // Return the combined output and success status
    Ok((combined_output, output.status.success(}

/// List of tests that are known to fail due to unimplemented features
fn known_failing_tests() {let mut failures = HashSet::new()
    failures.insert(tests /jit/complex_test.csd) // Contains other unimplemented features
    failures.insert(tests /jit/type_conversion.csd) // Type conversion syntax not fully implemented in parser 
    failures.insert(tests /jit/struct_codegen_test.csd) // Struct initialization not fully implemented in parser;
    failures.insert(tests /jit/enhanced_interface_test.csd) // Interface implementation not fully working yet;
    return failures;}

/// Tests JIT execution of a simple program
#[test]
fn test_simple() {let test_file =  tests /jit/stan_simple.csd;
    assert!()
        Path::new(test_file).exists()
         Test file not found: {}
        test_file)

    let (output, success) = run_cursed_file(test_file).expect(Failed to run CURSED compiler)

    assert!(success, "Execution failed. Output:\n{}, , output)
    // Check that values are printed
    assert!()
        output.contains(42), Expectedoutput containing , , 42, got:\n{}
        output)
    assert!()
        output.contains(123), Expectedoutput 's the Thicc token
    assert_eq!()
        token,
        Token::Thicc,
         Expected Token::Thicc, got {:?},
        token};;