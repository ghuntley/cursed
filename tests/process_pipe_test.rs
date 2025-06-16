/// Comprehensive tests for CURSED process pipe functionality
///
/// This test suite validates all pipe operations including stdin, stdout, stderr
/// pipe creation, real data transmission, error handling, and cross-platform compatibility.

use cursed::stdlib::process::{
    exec_slay::ProcessCommand,
    exec_vibez::VibezCommand,
    enhanced_exec_slay_complete::EnhancedSlayCommand,
    enhanced_exec_vibez_complete::EnhancedVibezCommand,
    error::ProcessResult,
};
use std::io::{Write, Read, BufRead, BufReader};
use std::time::Duration;
use std::thread;

/// Test basic stdin pipe functionality
#[test]
fn test_stdin_pipe_basic() {
    let mut cmd = ProcessCommand::new("cat");
    
    // Get stdin pipe and write data
    let mut stdin = cmd.stdin_pipe().expect("Failed to create stdin pipe");
    
    // Write test data
    let test_data = b"Hello from stdin pipe!";
    stdin.write_all(test_data).expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    
    // Close stdin to signal end of input
    drop(stdin);
    
    // Wait for process and get output
    let output = cmd.wait_output().expect("Failed to get process output");
    assert_eq!(output.stdout, test_data);
}

/// Test basic stdout pipe functionality
#[test]
fn test_stdout_pipe_basic() {
    let mut cmd = ProcessCommand::new("echo")
        .arg("Hello from stdout pipe!");
    
    // Get stdout pipe and read data
    let mut stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    
    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer).expect("Failed to read from stdout");
    
    assert!(buffer.contains("Hello from stdout pipe!"));
}

/// Test basic stderr pipe functionality  
#[test]
fn test_stderr_pipe_basic() {
    // Create command that writes to stderr
    let mut cmd = if cfg!(windows) {
        let mut cmd = ProcessCommand::new("cmd");
        cmd.arg("/C").arg("echo Error message 1>&2");
        cmd
    } else {
        let mut cmd = ProcessCommand::new("sh");
        cmd.arg("-c").arg("echo 'Error message' >&2");
        cmd
    };
    
    // Get stderr pipe and read data
    let mut stderr = cmd.stderr_pipe().expect("Failed to create stderr pipe");
    
    let mut buffer = String::new();
    stderr.read_to_string(&mut buffer).expect("Failed to read from stderr");
    
    assert!(buffer.contains("Error message"));
}

/// Test VibezCommand pipe functionality
#[test]
fn test_vibez_command_pipes() {
    let mut cmd = VibezCommand::new("echo").arg("VibezCommand test");
    
    // Test stdout pipe
    let mut stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer).expect("Failed to read from stdout");
    assert!(buffer.contains("VibezCommand test"));
}

/// Test EnhancedSlayCommand pipe functionality
#[test] 
fn test_enhanced_slay_command_pipes() {
    let mut cmd = EnhancedSlayCommand::new("echo")
        .arg("EnhancedSlay test");
    
    // Test stdout pipe
    let mut stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer).expect("Failed to read from stdout");
    assert!(buffer.contains("EnhancedSlay test"));
}

/// Test EnhancedVibezCommand pipe functionality
#[test]
fn test_enhanced_vibez_command_pipes() {
    let mut cmd = EnhancedVibezCommand::new("echo")
        .arg("EnhancedVibez test");
    
    // Test stdout pipe
    let mut stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer).expect("Failed to read from stdout");
    assert!(buffer.contains("EnhancedVibez test"));
}

/// Test interactive communication through pipes
#[test]
fn test_interactive_pipe_communication() {
    let mut cmd = if cfg!(windows) {
        ProcessCommand::new("findstr").arg("test")
    } else {
        ProcessCommand::new("grep").arg("test")
    };
    
    // Get both stdin and stdout pipes
    let mut stdin = cmd.stdin_pipe().expect("Failed to create stdin pipe");
    let mut stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    
    // Send data to stdin
    writeln!(stdin, "this is a test line").expect("Failed to write to stdin");
    writeln!(stdin, "this line has no match").expect("Failed to write to stdin");
    writeln!(stdin, "another test line").expect("Failed to write to stdin");
    
    // Close stdin to signal end
    drop(stdin);
    
    // Read from stdout
    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer).expect("Failed to read from stdout");
    
    // Should contain lines with "test"
    assert!(buffer.contains("this is a test line"));
    assert!(buffer.contains("another test line"));
    assert!(!buffer.contains("this line has no match"));
}

/// Test concurrent pipe operations
#[test]
fn test_concurrent_pipe_operations() {
    let mut cmd = ProcessCommand::new("cat");
    
    let mut stdin = cmd.stdin_pipe().expect("Failed to create stdin pipe");
    let mut stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    
    // Spawn thread to write to stdin
    let stdin_handle = thread::spawn(move || {
        for i in 0..5 {
            writeln!(stdin, "Line {}", i).expect("Failed to write line");
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Read from stdout in main thread
    let stdout_handle = thread::spawn(move || {
        let mut buffer = String::new();
        stdout.read_to_string(&mut buffer).expect("Failed to read from stdout");
        buffer
    });
    
    // Wait for both operations
    stdin_handle.join().expect("Stdin thread panicked");
    let output = stdout_handle.join().expect("Stdout thread panicked");
    
    // Verify all lines were transmitted
    for i in 0..5 {
        assert!(output.contains(&format!("Line {}", i)));
    }
}

/// Test large data transmission through pipes
#[test]
fn test_large_data_pipe_transmission() {
    let mut cmd = ProcessCommand::new("cat");
    
    let mut stdin = cmd.stdin_pipe().expect("Failed to create stdin pipe");
    let mut stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    
    // Generate large test data (10KB)
    let large_data = "0123456789".repeat(1024);
    
    // Write data in separate thread
    let large_data_clone = large_data.clone();
    let stdin_handle = thread::spawn(move || {
        stdin.write_all(large_data_clone.as_bytes()).expect("Failed to write large data");
        stdin.flush().expect("Failed to flush stdin");
    });
    
    // Read data in separate thread
    let stdout_handle = thread::spawn(move || {
        let mut buffer = Vec::new();
        stdout.read_to_end(&mut buffer).expect("Failed to read large data");
        String::from_utf8(buffer).expect("Invalid UTF-8")
    });
    
    // Wait for completion
    stdin_handle.join().expect("Stdin thread panicked");
    let received_data = stdout_handle.join().expect("Stdout thread panicked");
    
    // Verify data integrity
    assert_eq!(received_data, large_data);
}

/// Test pipe error handling
#[test]
fn test_pipe_error_handling() {
    // Test with non-existent command
    let mut cmd = ProcessCommand::new("non_existent_command_12345");
    
    // Should fail to create pipes
    assert!(cmd.stdin_pipe().is_err());
    assert!(cmd.stdout_pipe().is_err());
    assert!(cmd.stderr_pipe().is_err());
}

/// Test pipe cleanup and resource management
#[test]
fn test_pipe_resource_cleanup() {
    let mut cmd = ProcessCommand::new("echo").arg("cleanup test");
    
    // Create pipes
    let stdin = cmd.stdin_pipe().expect("Failed to create stdin pipe");
    let stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    let stderr = cmd.stderr_pipe().expect("Failed to create stderr pipe");
    
    // Explicitly drop pipes
    drop(stdin);
    drop(stdout);
    drop(stderr);
    
    // Process should still be accessible
    let _status = cmd.wait().expect("Failed to wait for process");
}

/// Test buffered pipe operations
#[test]
fn test_buffered_pipe_operations() {
    let mut cmd = ProcessCommand::new("cat");
    
    let mut stdin = cmd.stdin_pipe().expect("Failed to create stdin pipe");
    let stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    
    // Write multiple lines
    writeln!(stdin, "Line 1").expect("Failed to write line 1");
    writeln!(stdin, "Line 2").expect("Failed to write line 2");
    writeln!(stdin, "Line 3").expect("Failed to write line 3");
    drop(stdin);
    
    // Read lines using buffered reader
    let reader = BufReader::new(stdout);
    let lines: Vec<String> = reader.lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read lines");
    
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "Line 1");
    assert_eq!(lines[1], "Line 2");  
    assert_eq!(lines[2], "Line 3");
}

/// Test pipe timeouts and non-blocking operations
#[test]
fn test_pipe_timeouts() {
    // Test with a long-running command
    let mut cmd = if cfg!(windows) {
        let mut cmd = ProcessCommand::new("ping");
        cmd.arg("127.0.0.1").arg("-n").arg("1");
        cmd
    } else {
        let mut cmd = ProcessCommand::new("ping");
        cmd.arg("-c").arg("1").arg("127.0.0.1");
        cmd
    };
    
    let stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    
    // Should complete within reasonable time
    let start = std::time::Instant::now();
    let mut buffer = String::new();
    let _result = stdout.take(1024).read_to_string(&mut buffer);
    let elapsed = start.elapsed();
    
    // Should complete within 10 seconds
    assert!(elapsed < Duration::from_secs(10));
}

/// Test cross-platform compatibility
#[test]
fn test_cross_platform_compatibility() {
    // Test with platform-specific commands
    let mut cmd = if cfg!(windows) {
        ProcessCommand::new("cmd").arg("/C").arg("echo Windows pipe test")
    } else {
        ProcessCommand::new("echo").arg("Unix pipe test")
    };
    
    let mut stdout = cmd.stdout_pipe().expect("Failed to create stdout pipe");
    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer).expect("Failed to read from stdout");
    
    if cfg!(windows) {
        assert!(buffer.contains("Windows pipe test"));
    } else {
        assert!(buffer.contains("Unix pipe test"));
    }
}

/// Test pipe with command chaining
#[test]
fn test_pipe_command_chaining() {
    // Create a command that generates output
    let mut cmd1 = ProcessCommand::new("echo").arg("pipe chain test");
    let stdout1 = cmd1.stdout_pipe().expect("Failed to create first stdout pipe");
    
    // Create a command that processes the output
    let mut cmd2 = if cfg!(windows) {
        ProcessCommand::new("findstr").arg("chain")
    } else {
        ProcessCommand::new("grep").arg("chain")
    };
    
    let mut stdin2 = cmd2.stdin_pipe().expect("Failed to create second stdin pipe");
    let mut stdout2 = cmd2.stdout_pipe().expect("Failed to create second stdout pipe");
    
    // Connect pipes: cmd1 stdout -> cmd2 stdin
    let mut buffer1 = String::new();
    let reader1 = stdout1;
    thread::spawn(move || {
        let mut buf = String::new();
        reader1.take(1024).read_to_string(&mut buf).expect("Failed to read from first command");
        stdin2.write_all(buf.as_bytes()).expect("Failed to write to second command");
        stdin2.flush().expect("Failed to flush second stdin");
    });
    
    // Read final output
    let mut final_output = String::new();
    stdout2.read_to_string(&mut final_output).expect("Failed to read final output");
    
    assert!(final_output.contains("chain"));
}

/// Integration test for all pipe functionality
#[test]
fn test_comprehensive_pipe_integration() {
    // Test ProcessCommand
    let mut cmd1 = ProcessCommand::new("echo").arg("ProcessCommand test");
    let mut stdout1 = cmd1.stdout_pipe().expect("ProcessCommand stdout failed");
    let mut buffer1 = String::new();
    stdout1.read_to_string(&mut buffer1).expect("Read failed");
    assert!(buffer1.contains("ProcessCommand test"));
    
    // Test VibezCommand  
    let mut cmd2 = VibezCommand::new("echo").arg("VibezCommand test");
    let mut stdout2 = cmd2.stdout_pipe().expect("VibezCommand stdout failed");
    let mut buffer2 = String::new();
    stdout2.read_to_string(&mut buffer2).expect("Read failed");
    assert!(buffer2.contains("VibezCommand test"));
    
    // Test EnhancedSlayCommand
    let mut cmd3 = EnhancedSlayCommand::new("echo").arg("EnhancedSlay test");
    let mut stdout3 = cmd3.stdout_pipe().expect("EnhancedSlay stdout failed");
    let mut buffer3 = String::new();
    stdout3.read_to_string(&mut buffer3).expect("Read failed");
    assert!(buffer3.contains("EnhancedSlay test"));
    
    // Test EnhancedVibezCommand
    let mut cmd4 = EnhancedVibezCommand::new("echo").arg("EnhancedVibez test");
    let mut stdout4 = cmd4.stdout_pipe().expect("EnhancedVibez stdout failed");
    let mut buffer4 = String::new();
    stdout4.read_to_string(&mut buffer4).expect("Read failed");
    assert!(buffer4.contains("EnhancedVibez test"));
}
