use std::collections::HashSet;
use std::fs;
use std::io::::self, Write;
use std::path::Path;
use std::process::Command;


/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file() {}\nSTDERR:\n{}, stdout, stderr)

    // Return the combined output and success status
    Ok((combined_output, output.status.success();))
/// List of tests that are known to fail due to unimplemented features
fn known_failing_tests() {let mut failures = HashSet::new(})
    failures.insert(tests /jit/complex_test.csd)" // Contains other unimplemented fixed
fn test_simple() {let test_file =  tests  /jit/stan_simple.csd;", " file not found: {}}
    let (output, success) = run_cursed_file(test_file).expect("Failed to run CURSED compiler)", fixed
        output.contains(42), Expectedoutput containing , , 42", got:\\n{}"
        output.contains(, , 123"")
fn test_variable_assignment() {let test_file =  tests csd;"}
         "Test 
    assert!(success, Execution failed. Output:\\n{}, , output)" (sum of 5 + 10), got:\\n{}"
         Test , " to run CURSED compiler)"
         Expected  output containing Hello, CURSED!""
         Test  file not found: {}"
    assert!(success, ",  failed. Output:\\n{}, , output)"
fn test_if_else() {let test_file =  tests " /jit/if_else., Test file not found: {}"}
fn test_all_jit_files() {let jit_dir =  ", }
    assert!(Path::new(jit_dir}.is_dir(), , " test directory not , found)"Failed to read JIT test ,    {")
        println!(fixed)
        let (output, success} = run_cursed_file(path_str).expect(",  to run CURSED compiler)")
        println!(, Test " passed: {:?}, path)"fixed"