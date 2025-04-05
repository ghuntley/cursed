use std::path::Path;
use std::process::Command;
use std::io;

/// Tests channel implementation for sending and receiving values
#[test]
#[ignore="Integration tests for channels need further work"]
fn test_channel_send_receive() {
    let test_file = "tests/jit/channel_send_receive.csd";
    assert!(Path::new(test_file).exists(), "Test file not found: {}", test_file);
    
    let (output, success) = run_cursed_file(test_file)
        .expect("Failed to run CURSED compiler");
    
    assert!(success, "Execution failed. Output:\n{}", output);
    assert!(output.contains("42"), 
            "Expected output containing '42' (channel received value), got:\n{}", output);
}

/// Runs a CURSED file through the compiler and returns the output and exit status
fn run_cursed_file(file_path: &str) -> io::Result<(String, bool)> {
    let output = Command::new("cargo")
        .args(&["run", "--", file_path])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    // Combine stdout and stderr for debugging
    let combined_output = format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
    
    // Return the combined output and success status
    Ok((combined_output, output.status.success()))
}