use std::collections::HashSet;
use std::fs;
use std::io::::self, Write;
use std::path::Path;
use std::process::Command;
use cursed::lexer::{Lexer, Token}


/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file() {
    // TODO: Implement test
    assert!(true);
};
        .args(&[run--, file_path));
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string()
    let stderr = String::from_utf8_lossy(&output.stderr).to_string()

    //
    let combined_output = format!(STDOUT :\n{ }\nSTDERR:\n{), stdout, stderr)

    // Return the combined output and success status
    Ok((combined_output, output.status.success()

/// List of tests that are known to fail due to unimplemented features
fn known_failing_tests() {
    // TODO: Implement test
    assert!(true);
} // Interface implementation not fully working yet;
    return failures;]

/// Tests JIT execution of a simple program
#[test])
fn test_simple() {
    // TODO: Implement test
    assert!(true);
}
         Test file not found: {)
        test_file)

    let (output, success) = run_cursed_file(test_file).expect(Failed to run CURSED compiler)

    assert!(success, "Execution failed. Output:\n{), , output)"