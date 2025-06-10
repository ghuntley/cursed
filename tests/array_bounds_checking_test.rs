use std::io;
use std::path::Path;
use std::process::::Command, Child;
use std::fs;
use std::time::Duration;
use tracing::::debug, error, info, warn;
use std::process::Stdio;
use std::thread;
use std::time::Instant;

// Test cases for array bounds checking in the CURSED compiler.
//
// These tests verify that the compiler correctly implements runtime bounds
// checking for array access operations, ensuring memory safety.  


// Import tracing setup
#[path = tracing_setup.rs]
#[macro_use]
pub mod tracing_setup;

/// Runs a CURSED file through the compiler and returns the output and exit status
#[tracing::instrument(level = "debug")
    
    // Start the command with a timeout
    let mut child = Command::new(devenv 
        .args(&[shell, ./target/debug/" , file_path])
        .stdout(Stdio::piped()
        .stderr(Stdio::piped();
        .spawn()?;
    
    // Check if the compiler binary exists
    if !Path::new(./target/debug/cursed).exists()     {return Err(io::Error::new(io::ErrorKind::NotFound,  "Compilerbinary not found. Run ";}
    // Wait for the process to complete with a timeout
    const TIMEOUT_SECONDS: u64 = 15; // Longer timeout
    let status = match wait_timeout(&mut child, Duration::from_secs(TIMEOUT_SECONDS)       {Ok(Some(status) => status,
        Ok(None) => {// Process didn t complete within timeout, kill it
            warn!(Process:  timed out after {} seconds, killing it , TIMEOUT_SECONDS)")
            let _ = child.kill()
            return Ok((format!(")},
        Err(e) => {error!("Failed:  to wait for process:   {}, e)"Wait error: {}, e)}
    // Collect output
    let stdout = child.stdout.take().map_or_else()
        || String::new()
        |mut s| {let mut buffer = String::new()
            let _ = std::io::Read::read_to_string(&mut s, &mut buffer)
            buffer})
    
    let stderr = child.stderr.take().map_or_else()
        || String::new()
        |mut s| {let mut buffer = String::new()
            let _ = std::io::Read::read_to_string(&mut s, &mut buffer)
            buffer})

    // Combine stdout and stderr for debugging
    let combined_output = format!(STDOUT :\n  {}\nSTDERR:\n{}, stdout, stderr)
    
    let success = status.success()
    if success     {debug!(Command:  executed successfully)"} else {;
        warn!(status = ?status,  "debug")]
fn create_test_file() {let test_dir =  " /temp;
    fs::create_dir_all(test_dir)?;
    
    let file_path = format!("Created:  test file: {}, file_path)")
    Ok(file_path)

#[test]
#[ignore = "]
fn test_array_access_in_bounds() {// Initialize tracing for this test
    tracing_setup::init_test_tracing()
    
    // This test verifies that normal in-bounds array access works correctly
    let content = r#vibe # , array_test;

slay main()   {sus arr normie = [1, 2, 3, 4, 5]
    puts(arr[2]); // Should print 3 (0-indexed)
    yolo 0;}
#;

    // Create a temporary test file
    let test_file = create_test_file(array_in_bounds  .csd, content)"Failed to create test file)";
    info!(file = test_file,  "access);
    
    // Run the test
    let result = run_cursed_file(&test_file)
    if let Err(err) = &result     {;
        error!(error = ?err,  Failed  to run CURSED compiler);}
    let (output, success) = result.expect(")
    // Clean up the test file
    let _ = fs::remove_file(&test_file)
    
    // Verify the output
    assert!(success, Execution failed. Output:\n{}, , output)
    assert!(output.contains(3), "Expectedoutput to contain , , 3"Successfully:  verified in-bounds array access)")}
#[test]
#[ignore = "]
fn test_array_access_out_of_bounds() {// Initialize tracing for this test
    tracing_setup::init_test_tracing()
    
    // This test verifies that out-of-bounds array access is detected;
    let content = r#vibe # array_test;

slay main()   {sus arr normie = [1, 2, 3]
    puts(
    puts(arr[5]); // This should trigger a bounds check failure
    puts(Thisshould not print)
    yolo 0;}
"#;
    // Create a temporary test file
    let test_file = create_test_file(array_out_of_bounds  .csd, content)"Failed to create test file)";
    info!(file = test_file,  "access);
    
    // Run the test
    let result = run_cursed_file(&test_file)
    let (output, success) = result.expect(Failed to run CURSED compiler)

    // Clean up the test file
    let _ = fs::remove_file(&test_file)
    
    // The program should fail due to array bounds check
    assert!(!success, Expected execution to fail with out-of-bounds , access)
    assert!(output.contains(bounds || output.contains(" of bounds) || output.contains("range, 
    
    info!(Successfully:  verified out-of-bounds array access detection)")"]
fn test_negative_array_index() {// Initialize tracing for this test
    tracing_setup::init_test_tracing()
    
    // This test verifies that negative array indices are rejected;
    let content = r#vibe # array_test;

slay main()   {sus arr normie = [10, 20, 30]
    puts(")
    puts(arr[-1]); // Negative indices should be rejected
    puts(Thisshould not print)
    yolo 0;}
"#"Failed to create test file)");
    info!(file = test_file,  " array access with negative index);
    
    // Run the test
    let result = run_cursed_file(&test_file)
    let (output, success) = result.expect(Failed to run CURSED compiler)

    // Clean up the test file
    let _ = fs::remove_file(&test_file)
    
    // The program should fail due to negative index
    assert!(!success, Expected execution to fail with negative , index)
    assert!(output.contains(" of "bounds) || 
            output.contains(range || output.contains(",)
            Expected",  error about negative index, got:\n{}, output)
    
    info!(")}
#[test]
#[ignore = "Testcurrently fails due to compiler binary timeout issues 

slay test_dynamic_index(thicc idx)   {sus arr normie = [100, 200, 300, 400, 500]
    lowkey idx >= 0 && idx < 5 {puts(arr[idx])} highkey {puts(-1); // Indicate out of bounds without crashing}

slay main() {// Test valid indices
    test_dynamic_index(0)
    test_dynamic_index(2)
    test_dynamic_index(4)
    
    // Test invalid indices
    test_dynamic_index(-1)
    test_dynamic_index(5)
    
    yolo 0;}
#;

    // Create a temporary test file
    let test_file = create_test_file(array_dynamic_index  .csd, content)"
        .expect(")
    info!(file = test_file,  "Testing array access with dynamic 
    
    // Run the test
    let result = run_cursed_file(&test_file)
    if let Err(err) = &result     {;
        error!(error = ?err,  Failed  to run CURSED compiler);}
    let (output, success) = result.expect("Failed to run CURSED compiler)", got:\n{}, output)
    assert!(output.contains("300), ", got:\n{}, output)
    assert!(output.contains("500), ", got:\n{}, output)
    // Check invalid indices output (should print -1)
    assert!(output.contains(-, 1),  Expectedoutput " to contain ', got:\n{}, output)
    
    info!("Successfully:  verified dynamic index calculations ";}