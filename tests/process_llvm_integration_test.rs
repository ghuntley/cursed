/// Comprehensive integration tests for process management LLVM compilation in CURSED
/// 
/// This test suite validates the complete LLVM integration for process operations
/// including process spawning, management, I/O operations, and error handling.

use cursed::codegen::llvm::{LlvmCodeGenerator, ProcessCompiler, ProcessIoOperation};
use cursed::codegen::llvm::expression_compiler::{LlvmValue, LlvmType};
use cursed::error::Error;

/// Test process FFI function declarations
#[test]
fn test_process_ffi_function_declarations() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Ensure FFI functions can be declared without errors
    let result = generator.declare_process_ffi_functions();
    assert!(result.is_ok(), "Failed to declare process FFI functions: {:?}", result.err());
}

/// Test basic process spawning compilation
#[test]
fn test_process_spawn_compilation() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Create test command and arguments
    let command = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%echo_cmd".to_string(),
        is_constant: true,
    };
    
    let args = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%hello_args".to_string(),
        is_constant: true,
    };
    
    // Test process spawning without config
    let result = generator.compile_spawn_process(&command, &args, None);
    assert!(result.is_ok(), "Failed to compile process spawn: {:?}", result.err());
    
    let process_handle = result.unwrap();
    assert_eq!(process_handle.value_type, LlvmType::Integer64);
    assert!(!process_handle.is_constant);
    assert!(process_handle.llvm_name.starts_with("%temp_"));
}

/// Test process spawning with configuration
#[test]
fn test_process_spawn_with_config() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    let command = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%ls_cmd".to_string(),
        is_constant: true,
    };
    
    let args = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%ls_args".to_string(),
        is_constant: true,
    };
    
    let config = LlvmValue {
        value_type: LlvmType::Pointer(Box::new(LlvmType::String)),
        llvm_name: "%process_config".to_string(),
        is_constant: false,
    };
    
    // Test process spawning with config
    let result = generator.compile_spawn_process(&command, &args, Some(&config));
    assert!(result.is_ok(), "Failed to compile process spawn with config: {:?}", result.err());
    
    let process_handle = result.unwrap();
    assert_eq!(process_handle.value_type, LlvmType::Integer64);
}

/// Test process wait operations
#[test]
fn test_process_wait_operations() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    let process_handle = LlvmValue {
        value_type: LlvmType::Integer64,
        llvm_name: "%process_1".to_string(),
        is_constant: false,
    };
    
    // Test wait without timeout
    let wait_result = generator.compile_wait_process(&process_handle, None);
    assert!(wait_result.is_ok(), "Failed to compile process wait: {:?}", wait_result.err());
    
    let wait_value = wait_result.unwrap();
    assert_eq!(wait_value.value_type, LlvmType::Integer32);
    
    // Test wait with timeout
    let timeout = LlvmValue {
        value_type: LlvmType::Integer64,
        llvm_name: "5000".to_string(), // 5 seconds
        is_constant: true,
    };
    
    let wait_timeout_result = generator.compile_wait_process(&process_handle, Some(&timeout));
    assert!(wait_timeout_result.is_ok(), "Failed to compile process wait with timeout: {:?}", wait_timeout_result.err());
}

/// Test process status checking
#[test]
fn test_process_status_check() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    let process_handle = LlvmValue {
        value_type: LlvmType::Integer64,
        llvm_name: "%process_2".to_string(),
        is_constant: false,
    };
    
    let status_result = generator.compile_process_status(&process_handle);
    assert!(status_result.is_ok(), "Failed to compile process status check: {:?}", status_result.err());
    
    let status_value = status_result.unwrap();
    assert_eq!(status_value.value_type, LlvmType::Integer32);
    assert!(!status_value.is_constant);
}

/// Test process kill operations
#[test]
fn test_process_kill_operations() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    let process_handle = LlvmValue {
        value_type: LlvmType::Integer64,
        llvm_name: "%process_3".to_string(),
        is_constant: false,
    };
    
    // Test kill without signal (default SIGTERM)
    let kill_result = generator.compile_kill_process(&process_handle, None);
    assert!(kill_result.is_ok(), "Failed to compile process kill: {:?}", kill_result.err());
    
    let kill_value = kill_result.unwrap();
    assert_eq!(kill_value.value_type, LlvmType::Integer32);
    
    // Test kill with specific signal
    let signal = LlvmValue {
        value_type: LlvmType::Integer32,
        llvm_name: "9".to_string(), // SIGKILL
        is_constant: true,
    };
    
    let kill_signal_result = generator.compile_kill_process(&process_handle, Some(&signal));
    assert!(kill_signal_result.is_ok(), "Failed to compile process kill with signal: {:?}", kill_signal_result.err());
}

/// Test all process I/O operations
#[test]
fn test_process_io_operations() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    let process_handle = LlvmValue {
        value_type: LlvmType::Integer64,
        llvm_name: "%process_io".to_string(),
        is_constant: false,
    };
    
    let data_buffer = LlvmValue {
        value_type: LlvmType::Pointer(Box::new(LlvmType::String)),
        llvm_name: "%io_buffer".to_string(),
        is_constant: false,
    };
    
    // Test all I/O operations
    let operations = [
        (ProcessIoOperation::Read, "read operation"),
        (ProcessIoOperation::Write, "write operation"),
        (ProcessIoOperation::Close, "close operation"),
        (ProcessIoOperation::Flush, "flush operation"),
    ];
    
    for (operation, description) in &operations {
        let result = generator.compile_process_io(&process_handle, *operation, Some(&data_buffer));
        assert!(result.is_ok(), "Failed to compile {}: {:?}", description, result.err());
        
        let io_value = result.unwrap();
        assert_eq!(io_value.value_type, LlvmType::Integer64);
        assert!(!io_value.is_constant);
    }
    
    // Test I/O operations without data
    for (operation, description) in &operations {
        let result = generator.compile_process_io(&process_handle, *operation, None);
        assert!(result.is_ok(), "Failed to compile {} without data: {:?}", description, result.err());
    }
}

/// Test complete process lifecycle compilation
#[test]
fn test_complete_process_lifecycle() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // 1. Spawn process
    let command = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%python_cmd".to_string(),
        is_constant: true,
    };
    
    let args = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%python_script".to_string(),
        is_constant: true,
    };
    
    let spawn_result = generator.compile_spawn_process(&command, &args, None);
    assert!(spawn_result.is_ok());
    let process_handle = spawn_result.unwrap();
    
    // 2. Write to process stdin
    let input_data = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%input_text".to_string(),
        is_constant: true,
    };
    
    let write_result = generator.compile_process_io(
        &process_handle,
        ProcessIoOperation::Write,
        Some(&input_data),
    );
    assert!(write_result.is_ok());
    
    // 3. Read from process stdout
    let output_buffer = LlvmValue {
        value_type: LlvmType::Pointer(Box::new(LlvmType::String)),
        llvm_name: "%output_buffer".to_string(),
        is_constant: false,
    };
    
    let read_result = generator.compile_process_io(
        &process_handle,
        ProcessIoOperation::Read,
        Some(&output_buffer),
    );
    assert!(read_result.is_ok());
    
    // 4. Check process status
    let status_result = generator.compile_process_status(&process_handle);
    assert!(status_result.is_ok());
    
    // 5. Wait for process completion
    let timeout = LlvmValue {
        value_type: LlvmType::Integer64,
        llvm_name: "10000".to_string(), // 10 seconds
        is_constant: true,
    };
    
    let wait_result = generator.compile_wait_process(&process_handle, Some(&timeout));
    assert!(wait_result.is_ok());
    
    // 6. Close process (cleanup)
    let close_result = generator.compile_process_io(
        &process_handle,
        ProcessIoOperation::Close,
        None,
    );
    assert!(close_result.is_ok());
}

/// Test error handling in process operations
#[test]
fn test_process_error_handling() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Test with invalid process handle
    let invalid_handle = LlvmValue {
        value_type: LlvmType::Integer64,
        llvm_name: "-1".to_string(), // Invalid handle
        is_constant: true,
    };
    
    // All operations should still compile but may fail at runtime
    let operations = [
        ("wait", || generator.compile_wait_process(&invalid_handle, None)),
        ("status", || generator.compile_process_status(&invalid_handle)),
        ("kill", || generator.compile_kill_process(&invalid_handle, None)),
        ("io_read", || generator.compile_process_io(&invalid_handle, ProcessIoOperation::Read, None)),
    ];
    
    for (name, operation) in operations.iter() {
        let result = operation();
        assert!(result.is_ok(), "Process operation {} should compile even with invalid handle: {:?}", name, result.err());
    }
}

/// Test process operations with different value types
#[test]
fn test_process_operations_with_various_types() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Test with different command types
    let commands = [
        LlvmValue {
            value_type: LlvmType::String,
            llvm_name: "\"echo\"".to_string(),
            is_constant: true,
        },
        LlvmValue {
            value_type: LlvmType::Pointer(Box::new(LlvmType::String)),
            llvm_name: "%cmd_ptr".to_string(),
            is_constant: false,
        },
    ];
    
    let args = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "\"test\"".to_string(),
        is_constant: true,
    };
    
    for (i, command) in commands.iter().enumerate() {
        let result = generator.compile_spawn_process(command, &args, None);
        assert!(result.is_ok(), "Failed to compile spawn with command type {}: {:?}", i, result.err());
    }
}

/// Test concurrent process operations
#[test]
fn test_concurrent_process_operations() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    // Simulate multiple processes being managed
    let process_handles = (0..5).map(|i| LlvmValue {
        value_type: LlvmType::Integer64,
        llvm_name: format!("%process_{}", i),
        is_constant: false,
    }).collect::<Vec<_>>();
    
    // Test that we can generate operations for multiple processes
    for (i, handle) in process_handles.iter().enumerate() {
        let status_result = generator.compile_process_status(handle);
        assert!(status_result.is_ok(), "Failed to compile status for process {}: {:?}", i, status_result.err());
        
        let wait_result = generator.compile_wait_process(handle, None);
        assert!(wait_result.is_ok(), "Failed to compile wait for process {}: {:?}", i, wait_result.err());
    }
}

/// Test integration with error propagation (? operator)
#[test]
fn test_process_error_propagation_integration() {
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    let command = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%cmd_that_might_fail".to_string(),
        is_constant: true,
    };
    
    let args = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%risky_args".to_string(),
        is_constant: true,
    };
    
    // Test that process operations return Result types suitable for ? operator
    let spawn_result = generator.compile_spawn_process(&command, &args, None);
    assert!(spawn_result.is_ok());
    
    let process_handle = spawn_result.unwrap();
    
    // The returned values should be compatible with error propagation
    assert_eq!(process_handle.value_type, LlvmType::Integer64);
    assert!(!process_handle.is_constant);
}

/// Performance test for process operation compilation
#[test]
fn test_process_compilation_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    let mut generator = LlvmCodeGenerator::new().unwrap();
    
    let command = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%perf_cmd".to_string(),
        is_constant: true,
    };
    
    let args = LlvmValue {
        value_type: LlvmType::String,
        llvm_name: "%perf_args".to_string(),
        is_constant: true,
    };
    
    // Compile many process operations to test performance
    for i in 0..100 {
        let result = generator.compile_spawn_process(&command, &args, None);
        assert!(result.is_ok(), "Performance test failed at iteration {}: {:?}", i, result.err());
    }
    
    let duration = start.elapsed();
    println!("Compiled 100 process spawn operations in {:?}", duration);
    
    // Should compile reasonably quickly (adjust threshold as needed)
    assert!(duration.as_millis() < 1000, "Process compilation took too long: {:?}", duration);
}
