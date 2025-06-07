use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;


/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file(file_path: &str) -> io::Result<(String, bool)> {
    let output = Command::new("devenv")
        .args(&["shell", "./target/debug/cursed", file_path])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    //
    let combined_output = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);

    // Return the combined output and success status
    Ok((combined_output, output.status.success()))
}

/// List of tests that are known to fail due to unimplemented features
fn known_failing_tests() -> HashSet<&'static str> {
    let mut failures = HashSet::new();
    failures.insert("tests/jit/complex_test.csd"); // Contains other unimplemented features
    failures.insert("tests/jit/type_conversion.csd"); // Type conversion syntax not fully implemented in parser
    failures.insert("tests/jit/struct_codegen_test.csd"); // Struct initialization not fully implemented in parser
    return failures;
}

/// Tests JIT execution of a simple program
#[test]
fn test_simple() {
    let test_file = "tests/jit/stan_simple.csd";
    assert!(
        Path::new(test_file).exists(),
        "Test file not found: {}",
        test_file
    );

    let (output, success) = run_cursed_file(test_file).expect("Failed to run CURSED compiler");

    assert!(success, "Execution failed. Output:\n{}", output);

    // Check that values are printed
    assert!(
        output.contains("42"),
        "Expected output containing '42', got:\n{}",
        output
    );
    assert!(
        output.contains("123"),
        "Expected output containing '123', got:\n{}",
        output
    );
}

/// Tests JIT execution of variable assignments
#[test]
#[ignore = "currently broken until statement compilation is fixed"]
fn test_variable_assignment() {
    let test_file = "tests/jit/variable_arithmetic.csd";
    assert!(
        Path::new(test_file).exists(),
        "Test file not found: {}",
        test_file
    );

    let (output, success) = run_cursed_file(test_file).expect("Failed to run CURSED compiler");

    assert!(success, "Execution failed. Output:\n{}", output);

    // Check that arithmetic works correctly
    assert!(
        output.contains("15"),
        "Expected output containing '15' (sum of 5 + 10), got:\n{}",
        output
    );
}

/// Tests JIT execution of string printing
#[test]
fn test_string_printing() {
    let test_file = "tests/jit/println_string.csd";
    assert!(
        Path::new(test_file).exists(),
        "Test file not found: {}",
        test_file
    );

    let (output, success) = run_cursed_file(test_file).expect("Failed to run CURSED compiler");

    assert!(success, "Execution failed. Output:\n{}", output);

    // Check that the string is printed correctly
    assert!(
        output.contains("Hello, CURSED!"),
        "Expected output containing 'Hello, CURSED!', got:\n{}",
        output
    );
}

/// Tests JIT execution of integer literal printing
#[test]
fn test_integer_printing() {
    let test_file = "tests/jit/puts_integer.csd";
    assert!(
        Path::new(test_file).exists(),
        "Test file not found: {}",
        test_file
    );

    let (output, success) = run_cursed_file(test_file).expect("Failed to run CURSED compiler");

    assert!(success, "Execution failed. Output:\n{}", output);

    // Check that the integer is printed correctly
    assert!(
        output.contains("42"),
        "Expected output containing '42', got:\n{}",
        output
    );
}

/// Tests JIT execution of if-else statements
#[test]
#[ignore = "currently broken until statement compilation is fixed"]
fn test_if_else() {
    let test_file = "tests/jit/if_else.csd";
    assert!(
        Path::new(test_file).exists(),
        "Test file not found: {}",
        test_file
    );

    let (output, success) = run_cursed_file(test_file).expect("Failed to run CURSED compiler");

    assert!(success, "Execution failed. Output:\n{}", output);

    // Check that the if branch is executed
    assert!(
        output.contains("42"),
        "Expected output containing '42', got:\n{}",
        output
    );
}

/// Run all JIT tests in the directory that are expected to pass
#[test]
#[ignore = "Run individual tests instead of all at once"]
fn test_all_jit_files() {
    let jit_dir = "tests/jit";
    assert!(Path::new(jit_dir).is_dir(), "JIT test directory not found");
    let failing_tests = known_failing_tests();

    for entry in fs::read_dir(jit_dir).expect("Failed to read JIT test directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        // Skip non-CURSED files
        if path.extension().and_then(|ext| ext.to_str()) != Some("csd") {
            continue;
        }

        let path_str = path.to_str().expect("Invalid path");

        // Skip tests known to fail
        if failing_tests.contains(path_str) {
            println!("Skipping known failing test: {:?}", path);
            continue;
        }

        println!("Testing: {:?}", path);
        let (output, success) = run_cursed_file(path_str).expect("Failed to run CURSED compiler");

        assert!(
            success,
            "Execution of {:?} failed. Output:\n{}",
            path, output
        );
        println!("Test passed: {:?}", path);
    }
}
