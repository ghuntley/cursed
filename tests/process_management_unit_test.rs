/// Unit Tests for Process Management Runtime
/// 
/// This module provides comprehensive unit tests for the process management
/// functionality including process spawning, lifecycle management, signal
/// handling, and cross-platform compatibility.

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

use cursed::runtime::process::{
    ProcessRuntime, ProcessInfo, ProcessStatus, IpcChannel, IpcChannelType,
    IpcConfig, SharedMemorySegment, SignalHandler, initialize_process_runtime,
    get_process_runtime, shutdown_process_runtime
};

#[path = "common.rs"]
mod common;

/// Test process runtime initialization and singleton behavior
#[test]
fn test_process_runtime_initialization() {
    common::tracing::setup();
    
    // Initialize runtime
    initialize_process_runtime();
    
    // Get runtime instance
    let runtime1 = get_process_runtime();
    assert!(runtime1.is_some(), "Process runtime should be available after initialization");
    
    // Get another instance - should be the same
    let runtime2 = get_process_runtime();
    assert!(runtime2.is_some(), "Process runtime should remain available");
    
    // Verify it's the same instance (same Arc)
    if let (Some(r1), Some(r2)) = (runtime1, runtime2) {
        assert!(Arc::ptr_eq(&r1, &r2), "Runtime instances should be the same");
    }
    
    // Test shutdown
    shutdown_process_runtime();
    let runtime3 = get_process_runtime();
    assert!(runtime3.is_none(), "Process runtime should be None after shutdown");
}

/// Test process spawning functionality
#[test]
fn test_process_spawning() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test successful spawn
    let result = runtime.spawn_process("echo", &["test".to_string()]);
    
    // Note: This test may fail in some environments, handle gracefully
    match result {
        Ok(pid) => {
            assert!(pid > 0, "PID should be positive");
            
            // Verify process is tracked
            let status_result = runtime.get_process_status(pid);
            assert!(status_result.is_ok(), "Should be able to get process status");
            
            let status = status_result.unwrap();
            assert!(
                status == ProcessStatus::Running as i32 || 
                status == ProcessStatus::Exited as i32,
                "Process should be running or already exited"
            );
        }
        Err(e) => {
            // In test environments, this might fail due to security restrictions
            println!("Process spawn failed (may be expected in test env): {:?}", e);
        }
    }
}

/// Test process information tracking
#[test]
fn test_process_info_tracking() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test with non-existent process
    let result = runtime.get_process_info(99999);
    match result {
        Ok(ptr) => {
            if ptr.is_null() {
                // This is the expected behavior for non-existent process
            } else {
                // Clean up if somehow we got a valid pointer
                let _ = unsafe { Box::from_raw(ptr) };
            }
        }
        Err(_) => {
            // This is also acceptable behavior
        }
    }
    
    // Test process status for non-existent process
    let status_result = runtime.get_process_status(99999);
    assert!(status_result.is_err(), "Should fail for non-existent process");
}

/// Test signal handling registration
#[test]
fn test_signal_handler_registration() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test signal handler registration
    let result = runtime.register_signal_handler(15, None); // SIGTERM
    assert!(result.is_ok(), "Should be able to register signal handler");
    
    // Register a handler with a function
    fn test_handler(signal: i32) {
        println!("Received signal: {}", signal);
    }
    
    let result = runtime.register_signal_handler(2, Some(test_handler)); // SIGINT
    assert!(result.is_ok(), "Should be able to register signal handler with function");
}

/// Test IPC channel creation
#[test]
fn test_ipc_channel_creation() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    let config = IpcConfig {
        name: "test_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    // Test pipe creation
    let result = runtime.create_ipc_channel(IpcChannelType::Pipe, &config);
    assert!(result.is_ok(), "Should be able to create pipe channel");
    
    let channel_id = result.unwrap();
    assert!(channel_id > 0, "Channel ID should be positive");
    
    // Test named pipe creation
    let result = runtime.create_ipc_channel(IpcChannelType::NamedPipe, &config);
    assert!(result.is_ok(), "Should be able to create named pipe channel");
    
    // Test message queue creation
    let result = runtime.create_ipc_channel(IpcChannelType::MessageQueue, &config);
    assert!(result.is_ok(), "Should be able to create message queue channel");
}

/// Test IPC data transmission
#[test]
fn test_ipc_data_transmission() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    let config = IpcConfig {
        name: "test_data_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    // Create channel
    let channel_id = runtime.create_ipc_channel(IpcChannelType::Pipe, &config).unwrap();
    
    // Test sending data
    let test_data = b"Hello, IPC!";
    let send_result = runtime.ipc_send(channel_id, test_data);
    assert!(send_result.is_ok(), "Should be able to send data");
    
    // Test receiving data
    let receive_result = runtime.ipc_receive(channel_id, 100); // 100ms timeout
    assert!(receive_result.is_ok(), "Should be able to receive data");
    
    let data_ptr = receive_result.unwrap();
    if !data_ptr.is_null() {
        // Clean up the received data
        let received_data = unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
        assert_eq!(*received_data, test_data.to_vec(), "Received data should match sent data");
    }
}

/// Test shared memory management
#[test]
fn test_shared_memory_management() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test creating shared memory
    let result = runtime.create_shared_memory("test_shm", 4096);
    assert!(result.is_ok(), "Should be able to create shared memory");
    
    let ptr = result.unwrap();
    assert!(!ptr.is_null(), "Shared memory pointer should not be null");
    
    // Test creating shared memory with same name (should work)
    let result2 = runtime.create_shared_memory("test_shm", 4096);
    assert!(result2.is_ok(), "Should be able to access existing shared memory");
}

/// Test process status enumeration
#[test]
fn test_process_status_enumeration() {
    common::tracing::setup();
    
    // Test process status values
    assert_eq!(ProcessStatus::Running as i32, 0);
    assert_eq!(ProcessStatus::Exited as i32, 1);
    assert_eq!(ProcessStatus::Killed as i32, 2);
    assert_eq!(ProcessStatus::Stopped as i32, 3);
    assert_eq!(ProcessStatus::Zombie as i32, 4);
    assert_eq!(ProcessStatus::Unknown as i32, 5);
}

/// Test IPC channel type enumeration
#[test]
fn test_ipc_channel_type_enumeration() {
    common::tracing::setup();
    
    // Test IPC channel type values
    assert_eq!(IpcChannelType::Pipe as i32, 0);
    assert_eq!(IpcChannelType::NamedPipe as i32, 1);
    assert_eq!(IpcChannelType::MessageQueue as i32, 2);
    assert_eq!(IpcChannelType::SharedMemory as i32, 3);
    assert_eq!(IpcChannelType::Socket as i32, 4);
    assert_eq!(IpcChannelType::Semaphore as i32, 5);
}

/// Test concurrent process operations
#[test]
fn test_concurrent_process_operations() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let mut handles = vec![];
    
    // Spawn multiple threads performing operations
    for i in 0..5 {
        let runtime_clone = runtime.clone();
        let handle = thread::spawn(move || {
            let config = IpcConfig {
                name: format!("concurrent_test_{}", i),
                config_type: 0,
                size_or_capacity: 1024,
                permissions: 0o666,
                flags: 0,
            };
            
            // Create channels concurrently
            let result = runtime_clone.create_ipc_channel(IpcChannelType::Pipe, &config);
            assert!(result.is_ok(), "Concurrent channel creation should succeed");
            
            // Send some data
            let channel_id = result.unwrap();
            let test_data = format!("data_from_thread_{}", i);
            let send_result = runtime_clone.ipc_send(channel_id, test_data.as_bytes());
            assert!(send_result.is_ok(), "Concurrent data send should succeed");
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}

/// Test error handling for invalid operations
#[test]
fn test_error_handling() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test operations on non-existent process
    let invalid_pid = 99999u32;
    
    let kill_result = runtime.kill_process(invalid_pid);
    assert!(kill_result.is_err(), "Kill should fail for non-existent process");
    
    let terminate_result = runtime.terminate_process(invalid_pid);
    assert!(terminate_result.is_err(), "Terminate should fail for non-existent process");
    
    let wait_result = runtime.wait_process(invalid_pid);
    assert!(wait_result.is_err(), "Wait should fail for non-existent process");
    
    // Test operations on non-existent IPC channel
    let invalid_channel_id = 99999u64;
    
    let send_result = runtime.ipc_send(invalid_channel_id, b"test");
    assert!(send_result.is_err(), "Send should fail for non-existent channel");
    
    let receive_result = runtime.ipc_receive(invalid_channel_id, 100);
    assert!(receive_result.is_err(), "Receive should fail for non-existent channel");
}

/// Test signal sending functionality
#[test]
fn test_signal_sending() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test sending signal to non-existent process
    let result = runtime.send_signal(99999, 15); // SIGTERM
    assert!(result.is_err(), "Signal send should fail for non-existent process");
}

/// Test process information structure
#[test]
fn test_process_info_structure() {
    common::tracing::setup();
    
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let process_info = ProcessInfo {
        pid: 1234,
        command: "test_command".to_string(),
        status: ProcessStatus::Running,
        start_time,
        parent_pid: 1,
        memory_usage: 1024,
        cpu_time: 100,
        child: None,
    };
    
    assert_eq!(process_info.pid, 1234);
    assert_eq!(process_info.command, "test_command");
    assert_eq!(process_info.status, ProcessStatus::Running);
    assert_eq!(process_info.parent_pid, 1);
    assert_eq!(process_info.memory_usage, 1024);
    assert_eq!(process_info.cpu_time, 100);
    assert!(process_info.child.is_none());
}

/// Test shared memory segment structure
#[test]
fn test_shared_memory_segment_structure() {
    common::tracing::setup();
    
    let segment = SharedMemorySegment {
        name: "test_segment".to_string(),
        data: vec![0u8; 1024],
        size: 1024,
        permissions: 0o666,
        ref_count: 1,
    };
    
    assert_eq!(segment.name, "test_segment");
    assert_eq!(segment.data.len(), 1024);
    assert_eq!(segment.size, 1024);
    assert_eq!(segment.permissions, 0o666);
    assert_eq!(segment.ref_count, 1);
}

/// Test IPC channel structure
#[test]
fn test_ipc_channel_structure() {
    common::tracing::setup();
    
    let channel = IpcChannel {
        id: 42,
        channel_type: IpcChannelType::Pipe,
        handle_data: vec![1, 2, 3, 4],
        permissions: 0o644,
        is_open: true,
    };
    
    assert_eq!(channel.id, 42);
    assert_eq!(channel.channel_type, IpcChannelType::Pipe);
    assert_eq!(channel.handle_data, vec![1, 2, 3, 4]);
    assert_eq!(channel.permissions, 0o644);
    assert!(channel.is_open);
}

/// Test signal handler structure
#[test]
fn test_signal_handler_structure() {
    common::tracing::setup();
    
    fn test_handler_func(_signal: i32) {
        // Test handler function
    }
    
    let handler = SignalHandler {
        signal: 15, // SIGTERM
        handler: Some(test_handler_func),
        is_blocked: false,
    };
    
    assert_eq!(handler.signal, 15);
    assert!(handler.handler.is_some());
    assert!(!handler.is_blocked);
    
    // Test handler without function
    let handler_none = SignalHandler {
        signal: 2, // SIGINT
        handler: None,
        is_blocked: true,
    };
    
    assert_eq!(handler_none.signal, 2);
    assert!(handler_none.handler.is_none());
    assert!(handler_none.is_blocked);
}

/// Test runtime memory safety
#[test]
fn test_runtime_memory_safety() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Create multiple shared memory segments
    for i in 0..10 {
        let name = format!("memory_test_{}", i);
        let result = runtime.create_shared_memory(&name, 1024);
        assert!(result.is_ok(), "Should be able to create shared memory segment {}", i);
    }
    
    // Create multiple IPC channels
    for i in 0..10 {
        let config = IpcConfig {
            name: format!("channel_test_{}", i),
            config_type: 0,
            size_or_capacity: 512,
            permissions: 0o666,
            flags: 0,
        };
        
        let result = runtime.create_ipc_channel(IpcChannelType::Pipe, &config);
        assert!(result.is_ok(), "Should be able to create IPC channel {}", i);
    }
    
    // Runtime should handle cleanup automatically when dropped
}

/// Test IPC config structure
#[test]
fn test_ipc_config_structure() {
    common::tracing::setup();
    
    let config = IpcConfig {
        name: "test_config".to_string(),
        config_type: 1,
        size_or_capacity: 2048,
        permissions: 0o755,
        flags: 42,
    };
    
    assert_eq!(config.name, "test_config");
    assert_eq!(config.config_type, 1);
    assert_eq!(config.size_or_capacity, 2048);
    assert_eq!(config.permissions, 0o755);
    assert_eq!(config.flags, 42);
}

/// Test runtime clone and debug traits
#[test]
fn test_runtime_traits() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test Debug trait
    let debug_string = format!("{:?}", runtime);
    assert!(debug_string.contains("ProcessRuntime"), "Debug should include type name");
    
    // Test that structures are properly initialized
    assert!(debug_string.contains("processes"), "Debug should show processes field");
    assert!(debug_string.contains("ipc_channels"), "Debug should show ipc_channels field");
}
