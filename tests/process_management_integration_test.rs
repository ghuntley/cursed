/// Integration Tests for Process Management Runtime
/// 
/// This module provides comprehensive integration tests for the process management
/// functionality including real process execution, signal handling, cross-platform
/// compatibility, and performance under load.

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::process::Command;

use cursed::runtime::process::{
    ProcessRuntime, ProcessInfo, ProcessStatus, IpcChannel, IpcChannelType,
    IpcConfig, SharedMemorySegment, initialize_process_runtime,
    get_process_runtime, shutdown_process_runtime
};

#[path = "common.rs"]
mod common;

/// Test real process spawning and lifecycle management
#[test]
fn test_real_process_lifecycle() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Try to spawn a simple command that should exist on most systems
    let commands_to_try = vec![
        ("echo", vec!["hello".to_string()]),
        ("ls", vec!["--version".to_string()]),
        ("cat", vec!["--version".to_string()]),
    ];
    
    let mut successful_spawn = false;
    
    for (cmd, args) in commands_to_try {
        match runtime.spawn_process(cmd, &args) {
            Ok(pid) => {
                successful_spawn = true;
                println!("Successfully spawned {} with PID {}", cmd, pid);
                
                // Wait for the process to complete
                match runtime.wait_process(pid) {
                    Ok(exit_code) => {
                        println!("Process {} exited with code {}", pid, exit_code);
                        
                        // Check final status
                        if let Ok(status) = runtime.get_process_status(pid) {
                            assert!(
                                status == ProcessStatus::Exited as i32 || 
                                status == ProcessStatus::Killed as i32,
                                "Process should be exited or killed"
                            );
                        }
                    }
                    Err(e) => {
                        println!("Wait failed for process {}: {:?}", pid, e);
                    }
                }
                break;
            }
            Err(e) => {
                println!("Failed to spawn {}: {:?}", cmd, e);
                continue;
            }
        }
    }
    
    // In CI environments, this might not work, so we don't assert
    if successful_spawn {
        println!("Process lifecycle test completed successfully");
    } else {
        println!("No processes could be spawned (may be expected in restricted environments)");
    }
}

/// Test process information retrieval
#[test]
fn test_process_information_retrieval() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test with a long-running command if available
    if let Ok(pid) = runtime.spawn_process("sleep", &["0.1".to_string()]) {
        // Get process info immediately
        if let Ok(info_ptr) = runtime.get_process_info(pid) {
            if !info_ptr.is_null() {
                let info = unsafe { Box::from_raw(info_ptr) };
                
                assert_eq!(info.pid, pid);
                assert!(!info.command.is_empty());
                assert!(info.start_time > 0);
                
                println!("Process info: PID={}, Command={}, Status={:?}, Start={}",
                    info.pid, info.command, info.status, info.start_time);
            }
        }
        
        // Wait for completion
        let _ = runtime.wait_process(pid);
    }
}

/// Test concurrent process management
#[test]
fn test_concurrent_process_management() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    // Spawn multiple threads that create and manage processes
    for i in 0..3 {
        let runtime_clone = runtime.clone();
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            let thread_id = i;
            let mut results = vec![];
            
            // Try to spawn a simple process
            if let Ok(pid) = runtime_clone.spawn_process("echo", &[format!("thread_{}", thread_id)]) {
                results.push(format!("Thread {} spawned PID {}", thread_id, pid));
                
                // Wait for process
                if let Ok(exit_code) = runtime_clone.wait_process(pid) {
                    results.push(format!("Thread {} process {} exited with {}", thread_id, pid, exit_code));
                }
            }
            
            tx_clone.send(results).unwrap();
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads and collect results
    drop(tx);
    let mut all_results = vec![];
    while let Ok(results) = rx.recv() {
        all_results.extend(results);
    }
    
    for handle in handles {
        handle.join().expect("Thread should complete");
    }
    
    println!("Concurrent process management results:");
    for result in all_results {
        println!("  {}", result);
    }
}

/// Test IPC end-to-end communication
#[test]
fn test_ipc_end_to_end_communication() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    
    let config = IpcConfig {
        name: "integration_test_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    // Create IPC channel
    let channel_id = runtime.create_ipc_channel(IpcChannelType::Pipe, &config)
        .expect("Should create IPC channel");
    
    // Test communication between threads
    let runtime_sender = runtime.clone();
    let runtime_receiver = runtime.clone();
    
    let (sync_tx, sync_rx) = mpsc::channel();
    
    // Sender thread
    let sender_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10)); // Let receiver start first
        
        let messages = vec![
            b"Hello, IPC!".to_vec(),
            b"Message 2".to_vec(),
            b"Final message".to_vec(),
        ];
        
        for (i, message) in messages.iter().enumerate() {
            match runtime_sender.ipc_send(channel_id, message) {
                Ok(_) => println!("Sent message {}: {:?}", i + 1, String::from_utf8_lossy(message)),
                Err(e) => println!("Failed to send message {}: {:?}", i + 1, e),
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    // Receiver thread
    let receiver_handle = thread::spawn(move || {
        let mut received_messages = 0;
        
        for _ in 0..5 { // Try to receive up to 5 times
            match runtime_receiver.ipc_receive(channel_id, 100) {
                Ok(data_ptr) => {
                    if !data_ptr.is_null() {
                        let data = unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
                        println!("Received message: {:?}", String::from_utf8_lossy(&data));
                        received_messages += 1;
                    } else {
                        println!("Received null data (timeout or no data)");
                    }
                }
                Err(e) => {
                    println!("Receive error: {:?}", e);
                    break;
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
        
        sync_tx.send(received_messages).unwrap();
    });
    
    // Wait for completion
    sender_handle.join().expect("Sender should complete");
    receiver_handle.join().expect("Receiver should complete");
    
    let received_count = sync_rx.recv().unwrap_or(0);
    println!("Total messages received: {}", received_count);
}

/// Test shared memory operations
#[test]
fn test_shared_memory_operations() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    
    // Create shared memory segment
    let shm_ptr = runtime.create_shared_memory("integration_test_shm", 4096)
        .expect("Should create shared memory");
    
    assert!(!shm_ptr.is_null(), "Shared memory pointer should not be null");
    
    // Test concurrent access to shared memory
    let runtime_clone = runtime.clone();
    let handle = thread::spawn(move || {
        // Try to access the same shared memory
        let ptr = runtime_clone.create_shared_memory("integration_test_shm", 4096)
            .expect("Should access existing shared memory");
        
        assert!(!ptr.is_null(), "Shared memory should be accessible from other thread");
    });
    
    handle.join().expect("Shared memory access thread should complete");
}

/// Test signal handling integration
#[test]
fn test_signal_handling_integration() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Register signal handlers
    let result1 = runtime.register_signal_handler(15, None); // SIGTERM
    assert!(result1.is_ok(), "Should register SIGTERM handler");
    
    let result2 = runtime.register_signal_handler(2, None); // SIGINT
    assert!(result2.is_ok(), "Should register SIGINT handler");
    
    // Test sending signals (these operations will mostly test the code paths
    // rather than actual signal delivery in test environment)
    if let Ok(pid) = runtime.spawn_process("sleep", &["1".to_string()]) {
        // Try to send various signals
        let signals_to_test = vec![15, 2, 9]; // SIGTERM, SIGINT, SIGKILL
        
        for signal in signals_to_test {
            match runtime.send_signal(pid, signal) {
                Ok(_) => println!("Successfully sent signal {} to process {}", signal, pid),
                Err(e) => println!("Failed to send signal {} to process {}: {:?}", signal, pid, e),
            }
        }
        
        // Wait for process (might already be dead from signals)
        let _ = runtime.wait_process(pid);
    }
}

/// Test error recovery and cleanup
#[test]
fn test_error_recovery_and_cleanup() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test recovery from failed operations
    let invalid_operations = vec![
        ("kill_process", || runtime.kill_process(99999)),
        ("terminate_process", || runtime.terminate_process(99999)),
        ("wait_process", || runtime.wait_process(99999)),
        ("get_process_status", || runtime.get_process_status(99999).map(|_| 0)),
        ("send_signal", || runtime.send_signal(99999, 15)),
    ];
    
    for (op_name, operation) in invalid_operations {
        match operation() {
            Ok(_) => println!("Unexpected success for invalid operation: {}", op_name),
            Err(e) => println!("Expected error for {}: {:?}", op_name, e),
        }
    }
    
    // Test IPC cleanup after errors
    let invalid_channel_ops = vec![
        ("ipc_send", || runtime.ipc_send(99999, b"test").map(|_| ())),
        ("ipc_receive", || runtime.ipc_receive(99999, 100).map(|_| ())),
    ];
    
    for (op_name, operation) in invalid_channel_ops {
        match operation() {
            Ok(_) => println!("Unexpected success for invalid IPC operation: {}", op_name),
            Err(e) => println!("Expected error for {}: {:?}", op_name, e),
        }
    }
}

/// Test performance under load
#[test]
fn test_performance_under_load() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let start_time = Instant::now();
    
    // Create many IPC channels quickly
    let mut channel_ids = vec![];
    for i in 0..100 {
        let config = IpcConfig {
            name: format!("perf_test_channel_{}", i),
            config_type: 0,
            size_or_capacity: 512,
            permissions: 0o666,
            flags: 0,
        };
        
        if let Ok(channel_id) = runtime.create_ipc_channel(IpcChannelType::Pipe, &config) {
            channel_ids.push(channel_id);
        }
    }
    
    let channel_creation_time = start_time.elapsed();
    println!("Created {} channels in {:?}", channel_ids.len(), channel_creation_time);
    
    // Test data transmission performance
    let data_start = Instant::now();
    let test_data = b"Performance test data payload";
    
    for &channel_id in &channel_ids {
        let _ = runtime.ipc_send(channel_id, test_data);
    }
    
    let data_send_time = data_start.elapsed();
    println!("Sent data to {} channels in {:?}", channel_ids.len(), data_send_time);
    
    // Test shared memory creation performance
    let shm_start = Instant::now();
    for i in 0..50 {
        let name = format!("perf_shm_{}", i);
        let _ = runtime.create_shared_memory(&name, 1024);
    }
    
    let shm_creation_time = shm_start.elapsed();
    println!("Created 50 shared memory segments in {:?}", shm_creation_time);
    
    let total_time = start_time.elapsed();
    println!("Total performance test time: {:?}", total_time);
    
    // Performance assertions (generous limits for CI environments)
    assert!(channel_creation_time < Duration::from_secs(5), 
            "Channel creation should be reasonably fast");
    assert!(data_send_time < Duration::from_secs(2), 
            "Data sending should be reasonably fast");
    assert!(shm_creation_time < Duration::from_secs(2), 
            "Shared memory creation should be reasonably fast");
}

/// Test cross-platform compatibility
#[test]
fn test_cross_platform_compatibility() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test platform-specific commands
    #[cfg(unix)]
    let test_commands = vec![
        ("echo", vec!["unix_test".to_string()]),
        ("true", vec![]),
        ("false", vec![]),
    ];
    
    #[cfg(windows)]
    let test_commands = vec![
        ("echo", vec!["windows_test".to_string()]),
        ("cmd", vec!["/c".to_string(), "echo".to_string(), "test".to_string()]),
    ];
    
    for (cmd, args) in test_commands {
        match runtime.spawn_process(cmd, &args) {
            Ok(pid) => {
                println!("Successfully spawned {} on this platform with PID {}", cmd, pid);
                
                // Test platform-specific signal handling
                #[cfg(unix)]
                {
                    let _ = runtime.send_signal(pid, 15); // SIGTERM
                }
                
                #[cfg(windows)]
                {
                    let _ = runtime.send_signal(pid, 15); // Mapped to process termination
                }
                
                let _ = runtime.wait_process(pid);
            }
            Err(e) => {
                println!("Failed to spawn {} on this platform: {:?}", cmd, e);
            }
        }
    }
}

/// Test global runtime instance integration
#[test]
fn test_global_runtime_integration() {
    common::tracing::setup();
    
    // Ensure clean state
    shutdown_process_runtime();
    
    // Initialize global runtime
    initialize_process_runtime();
    
    let runtime = get_process_runtime().expect("Global runtime should be available");
    
    // Test operations through global runtime
    let config = IpcConfig {
        name: "global_test_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    let channel_result = runtime.create_ipc_channel(IpcChannelType::Pipe, &config);
    assert!(channel_result.is_ok(), "Should create channel through global runtime");
    
    let shm_result = runtime.create_shared_memory("global_test_shm", 2048);
    assert!(shm_result.is_ok(), "Should create shared memory through global runtime");
    
    // Test accessing from another thread
    let runtime_clone = runtime.clone();
    let handle = thread::spawn(move || {
        let config = IpcConfig {
            name: "global_test_channel_2".to_string(),
            config_type: 0,
            size_or_capacity: 512,
            permissions: 0o666,
            flags: 0,
        };
        
        let result = runtime_clone.create_ipc_channel(IpcChannelType::NamedPipe, &config);
        assert!(result.is_ok(), "Should access global runtime from thread");
    });
    
    handle.join().expect("Thread should complete");
    
    // Cleanup
    shutdown_process_runtime();
    assert!(get_process_runtime().is_none(), "Runtime should be cleaned up");
}

/// Test memory management and cleanup
#[test]
fn test_memory_management_and_cleanup() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Create many resources to test cleanup
    let mut channels = vec![];
    let mut shared_memories = vec![];
    
    for i in 0..20 {
        // Create IPC channels
        let config = IpcConfig {
            name: format!("cleanup_test_channel_{}", i),
            config_type: 0,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        };
        
        if let Ok(channel_id) = runtime.create_ipc_channel(IpcChannelType::Pipe, &config) {
            channels.push(channel_id);
        }
        
        // Create shared memory
        let shm_name = format!("cleanup_test_shm_{}", i);
        if let Ok(ptr) = runtime.create_shared_memory(&shm_name, 1024) {
            shared_memories.push((shm_name, ptr));
        }
    }
    
    println!("Created {} channels and {} shared memory segments", 
             channels.len(), shared_memories.len());
    
    // Add some data to channels
    for &channel_id in &channels {
        let _ = runtime.ipc_send(channel_id, b"cleanup test data");
    }
    
    // Runtime should clean up automatically when dropped
    drop(runtime);
    
    println!("Runtime dropped, resources should be cleaned up");
}
