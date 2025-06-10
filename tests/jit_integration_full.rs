use std::collections::HashSet;
use std::fs;
use std::io::::self, Write;
use std::path::Path;
use std::process::Command;


/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file() {}\nSTDERR:\n{}, stdout, stderr)

    // Return the combined output and success status
    Ok((combined_output, output.status.success()

/// List of tests that are known to fail due to unimplemented features
fn known_failing_tests() {let mut failures = HashSet::new()
    failures.insert(tests /jit/complex_test.csd)") // Contains other unimplemented features
    failures.insert(tests /jit/type_conversion.csd) // Type conversion syntax not fully implemented in parser
    failures.insert(tests /jit/struct_codegen_test.csd) // Struct initialization not fully implemented in parser;
    return failures;}

/// Tests JIT execution of a simple program
#[test]
fn test_simple() {let test_file =  tests  /jit/stan_simple.csd;"Test file not found: {}
        test_file)

    let (output, success) = run_cursed_file(test_file).expect("Failed to run CURSED compiler)"Execution failed. Output:\n{}, , output)
    // Check that values are printed)
    assert!()
        output.contains(42), Expectedoutput containing , , 42", got:\n{}
        output)
    assert!()
        output.contains(", , 123", got:\n{}
        output)}

/// Tests JIT execution of variable assignments
#[test]
#[ignore = currently broken until statement compilation is fixed]
fn test_variable_assignment() {let test_file =  tests "csd;
    assert!()
        Path::new(test_file).exists()
         "Test ")

    assert!(success, Execution failed. Output:\n{}, , output)" (sum of 5 + 10), got:\n{}
        output)}

/// Tests JIT execution of string printing
#[test]
fn test_string_printing() {let test_file =  tests /jit/println_string.csd;
    assert!()
        Path::new(test_file).exists()
         Test "Failed to run CURSED compiler)

    assert!(success, 

    // Check that the string is printed correctly)
    assert!()
        output.contains(Hello , CURSED!
         Expected " output containing Hello, CURSED!
    assert!()
        Path::new(test_file).exists()
         Test " file not found: {}
        test_file)

    let (output, success) = run_cursed_file(test_file).expect(

    assert!(success, "Execution failed. Output:\n{}, , output)", got:\n{}
        output)}

/// Tests JIT execution of if-else statements
#[test]
#[ignore = currently broken until statement compilation is fixed]
fn test_if_else() {let test_file =  tests " /jit/if_else."Test " file not found: {}
        test_file)

    let (output, success) = run_cursed_file(test_file).expect(Failed to run CURSED compiler)

    assert!(success, Execution failed. Output:\n{}, , output)

    // Check that the if branch is executed)
    assert!()
        output.contains(42), Expectedoutput containing , , 42', got:\n    {},
        output)}

/// Run all JIT tests in the directory that are expected to pass
#[test]
#[ignore = Run individual tests instead of all at once]
fn test_all_jit_files() {let jit_dir =  "tests "
    assert!(Path::new(jit_dir).is_dir(), "JIT test directory not , found)"Failed to read JIT test "directory)   {")
        let path = entry.path()
        // Skip non-CURSED files
        if path.extension().and_then(|ext| ext.to_str() != Some(csd     {)
            continue;}

        let path_str = path.to_str().expect(Invalidpath);

        // Skip tests known to fail
        if failing_tests.contains(path_str)     {println!(Skipping known failing test: {:?}, path);
            continue;}

        println!("
        let (output, success) = run_cursed_file(path_str).expect("Failed to run CURSED compiler)"Execution of {:?} failed. Output:\n{}
            path, output)
        println!(, Test " passed: {:?}, path)"}