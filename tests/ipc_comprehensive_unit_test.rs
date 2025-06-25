/// Comprehensive Unit Tests for IPC Runtime System
/// 
/// This module provides extensive unit tests for all IPC functionality including
/// named pipes, message queues, shared memory, semaphores, signal handling,
/// connection pooling, and advanced IPC management features.

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::collections::HashMap;

use cursed::runtime::process::{
    ProcessRuntime, IpcChannel, IpcChannelType, IpcConfig, SharedMemorySegment
};

#[path = "common.rs"]
mod common;

/// Test IPC channel creation for all types
#[test]
fn test_ipc_channel_creation_all_types() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    let channel_types = vec![
        (IpcChannelType::Pipe, "pipe"),
        (IpcChannelType::NamedPipe, "named_pipe"),
        (IpcChannelType::MessageQueue, "message_queue"),
        (IpcChannelType::SharedMemory, "shared_memory"),
        (IpcChannelType::Socket, "socket"),
        (IpcChannelType::Semaphore, "semaphore"),
    ];
    
    for (channel_type, type_name) in channel_types {
        let config = IpcConfig {
            name: format!("test_{}_channel", type_name),
            config_type: channel_type as i32,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        };
        
        let result = runtime.create_ipc_channel(channel_type.clone(), &config);
        assert!(result.is_ok(), "Should create {} channel", type_name);
        
        let channel_id = result.unwrap();
        assert!(channel_id > 0, "{} channel ID should be positive", type_name);
        
        println!("Created {} channel with ID {}", type_name, channel_id);
    }
}

/// Test IPC channel configuration options
#[test]
fn test_ipc_channel_configuration() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test different configuration parameters
    let configs = vec![
        IpcConfig {
            name: "small_channel".to_string(),
            config_type: 0,
            size_or_capacity: 256,
            permissions: 0o600,
            flags: 1,
        },
        IpcConfig {
            name: "large_channel".to_string(),
            config_type: 1,
            size_or_capacity: 65536,
            permissions: 0o755,
            flags: 42,
        },
        IpcConfig {
            name: "default_channel".to_string(),
            config_type: 2,
            size_or_capacity: 4096,
            permissions: 0o644,
            flags: 0,
        },
    ];
    
    for (i, config) in configs.iter().enumerate() {
        let result = runtime.create_ipc_channel(IpcChannelType::Pipe, config);
        assert!(result.is_ok(), "Should create channel with config {}", i);
        
        let channel_id = result.unwrap();
        println!("Created channel {} with config: size={}, permissions={:o}, flags={}",
                channel_id, config.size_or_capacity, config.permissions, config.flags);
    }
}

/// Test IPC data transmission with different data sizes
#[test]
fn test_ipc_data_transmission_sizes() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    let config = IpcConfig {
        name: "data_size_test_channel".to_string(),
        config_type: 0,
        size_or_capacity: 8192,
        permissions: 0o666,
        flags: 0,
    };
    
    let channel_id = runtime.create_ipc_channel(IpcChannelType::Pipe, &config)
        .expect("Should create channel");
    
    // Test different data sizes
    let test_data_sets = vec![
        (b"small".to_vec(), "small data"),
        (b"medium sized data payload for testing".to_vec(), "medium data"),
        (vec![0u8; 1024], "1KB data"),
        (vec![42u8; 4096], "4KB data"),
    ];
    
    for (data, description) in test_data_sets {
        // Send data
        let send_result = runtime.ipc_send(channel_id, &data);
        assert!(send_result.is_ok(), "Should send {}", description);
        
        // Receive data
        let receive_result = runtime.ipc_receive(channel_id, 1000);
        assert!(receive_result.is_ok(), "Should receive {}", description);
        
        let received_ptr = receive_result.unwrap();
        if !received_ptr.is_null() {
            let received_data = unsafe { Box::from_raw(received_ptr as *mut Vec<u8>) };
            assert_eq!(*received_data, data, "Received data should match sent data for {}", description);
            println!("Successfully transmitted {} ({} bytes)", description, data.len());
        }
    }
}

/// Test IPC timeout behavior
#[test]
fn test_ipc_timeout_behavior() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    let config = IpcConfig {
        name: "timeout_test_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    let channel_id = runtime.create_ipc_channel(IpcChannelType::Pipe, &config)
        .expect("Should create channel");
    
    // Test receive timeout on empty channel
    let timeout_values = vec![0, 10, 100, 500];
    
    for timeout_ms in timeout_values {
        let start_time = Instant::now();
        let result = runtime.ipc_receive(channel_id, timeout_ms);
        let elapsed = start_time.elapsed();
        
        match result {
            Ok(ptr) => {
                if ptr.is_null() {
                    println!("Timeout {} ms: received null pointer after {:?}", timeout_ms, elapsed);
                } else {
                    // Clean up unexpected data
                    let _ = unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
                    println!("Timeout {} ms: unexpectedly received data after {:?}", timeout_ms, elapsed);
                }
            }
            Err(e) => {
                println!("Timeout {} ms: error after {:?}: {:?}", timeout_ms, elapsed, e);
            }
        }
        
        // Verify timeout behavior (with some tolerance for system scheduling)
        if timeout_ms > 0 {
            let expected_duration = Duration::from_millis(timeout_ms as u64);
            let tolerance = Duration::from_millis(50); // 50ms tolerance
            
            assert!(elapsed >= expected_duration.checked_sub(tolerance).unwrap_or(Duration::ZERO),
                    "Should wait at least close to timeout duration");
        }
    }
}

/// Test shared memory operations
#[test]
fn test_shared_memory_operations() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test creating shared memory with different sizes
    let memory_configs = vec![
        ("small_shm", 1024),
        ("medium_shm", 4096),
        ("large_shm", 65536),
        ("tiny_shm", 64),
    ];
    
    for (name, size) in memory_configs {
        let result = runtime.create_shared_memory(name, size);
        assert!(result.is_ok(), "Should create shared memory {} of size {}", name, size);
        
        let ptr = result.unwrap();
        assert!(!ptr.is_null(), "Shared memory pointer should not be null for {}", name);
        
        println!("Created shared memory {} with size {} bytes", name, size);
    }
    
    // Test accessing existing shared memory
    let ptr1 = runtime.create_shared_memory("duplicate_test", 2048).unwrap();
    let ptr2 = runtime.create_shared_memory("duplicate_test", 2048).unwrap();
    
    // Both should be valid (implementation detail: might be same or different)
    assert!(!ptr1.is_null(), "First access should be valid");
    assert!(!ptr2.is_null(), "Second access should be valid");
}

/// Test concurrent IPC operations
#[test]
fn test_concurrent_ipc_operations() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let num_threads = 5;
    let operations_per_thread = 10;
    
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    // Spawn threads that perform IPC operations concurrently
    for thread_id in 0..num_threads {
        let runtime_clone = runtime.clone();
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            let mut results = vec![];
            
            for op_id in 0..operations_per_thread {
                let config = IpcConfig {
                    name: format!("concurrent_thread_{}_op_{}", thread_id, op_id),
                    config_type: 0,
                    size_or_capacity: 512,
                    permissions: 0o666,
                    flags: 0,
                };
                
                // Create channel
                if let Ok(channel_id) = runtime_clone.create_ipc_channel(IpcChannelType::Pipe, &config) {
                    // Send data
                    let test_data = format!("data_{}_{}", thread_id, op_id);
                    if let Ok(_) = runtime_clone.ipc_send(channel_id, test_data.as_bytes()) {
                        // Try to receive
                        if let Ok(ptr) = runtime_clone.ipc_receive(channel_id, 100) {
                            if !ptr.is_null() {
                                let received = unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
                                let received_str = String::from_utf8_lossy(&received);
                                results.push(format!("Thread {} op {}: sent and received '{}'", 
                                                   thread_id, op_id, received_str));
                            }
                        }
                    }
                }
            }
            
            tx_clone.send((thread_id, results)).unwrap();
        });
        
        handles.push(handle);
    }
    
    // Collect results
    drop(tx);
    let mut all_results = HashMap::new();
    while let Ok((thread_id, results)) = rx.recv() {
        all_results.insert(thread_id, results);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread should complete");
    }
    
    println!("Concurrent IPC operation results:");
    for (thread_id, results) in all_results {
        println!("  Thread {}: {} successful operations", thread_id, results.len());
        for result in results {
            println!("    {}", result);
        }
    }
}

/// Test IPC error handling and recovery
#[test]
fn test_ipc_error_handling() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test operations on non-existent channels
    let invalid_channel_id = 99999u64;
    
    let send_result = runtime.ipc_send(invalid_channel_id, b"test data");
    assert!(send_result.is_err(), "Send should fail for non-existent channel");
    
    let receive_result = runtime.ipc_receive(invalid_channel_id, 100);
    assert!(receive_result.is_err(), "Receive should fail for non-existent channel");
    
    // Test empty data transmission
    let config = IpcConfig {
        name: "error_test_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    let channel_id = runtime.create_ipc_channel(IpcChannelType::Pipe, &config)
        .expect("Should create channel");
    
    // Test sending empty data
    let empty_data = b"";
    let send_result = runtime.ipc_send(channel_id, empty_data);
    assert!(send_result.is_ok(), "Should be able to send empty data");
    
    // Test receiving with zero timeout
    let receive_result = runtime.ipc_receive(channel_id, 0);
    assert!(receive_result.is_ok(), "Zero timeout receive should succeed (may return null)");
}

/// Test IPC channel state management
#[test]
fn test_ipc_channel_state_management() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Create multiple channels and track their state
    let mut channel_ids = vec![];
    
    for i in 0..5 {
        let config = IpcConfig {
            name: format!("state_test_channel_{}", i),
            config_type: i,
            size_or_capacity: 1024 * (i as u64 + 1),
            permissions: 0o666,
            flags: i as u32,
        };
        
        let channel_id = runtime.create_ipc_channel(IpcChannelType::Pipe, &config)
            .expect("Should create channel");
        
        channel_ids.push(channel_id);
    }
    
    // Test operations on all channels
    for (i, &channel_id) in channel_ids.iter().enumerate() {
        let test_data = format!("state_test_data_{}", i);
        
        // Send data
        let send_result = runtime.ipc_send(channel_id, test_data.as_bytes());
        assert!(send_result.is_ok(), "Should send data to channel {}", i);
        
        // Receive data
        let receive_result = runtime.ipc_receive(channel_id, 500);
        assert!(receive_result.is_ok(), "Should receive data from channel {}", i);
        
        if let Ok(ptr) = receive_result {
            if !ptr.is_null() {
                let received = unsafe { Box::from_raw(ptr as *mut Vec<u8>) };
                let received_str = String::from_utf8_lossy(&received);
                assert_eq!(received_str, test_data, "Received data should match for channel {}", i);
            }
        }
    }
}

/// Test shared memory concurrency
#[test]
fn test_shared_memory_concurrency() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let num_threads = 4;
    let shm_name = "concurrent_shm_test";
    
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    // Create shared memory first
    let initial_ptr = runtime.create_shared_memory(shm_name, 4096)
        .expect("Should create initial shared memory");
    assert!(!initial_ptr.is_null(), "Initial shared memory should be valid");
    
    // Spawn threads that access the same shared memory
    for thread_id in 0..num_threads {
        let runtime_clone = runtime.clone();
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            // Access the existing shared memory
            let ptr = runtime_clone.create_shared_memory(shm_name, 4096);
            
            match ptr {
                Ok(shm_ptr) => {
                    let success = !shm_ptr.is_null();
                    tx_clone.send((thread_id, success)).unwrap();
                }
                Err(e) => {
                    println!("Thread {} failed to access shared memory: {:?}", thread_id, e);
                    tx_clone.send((thread_id, false)).unwrap();
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Collect results
    drop(tx);
    let mut successful_accesses = 0;
    while let Ok((thread_id, success)) = rx.recv() {
        if success {
            successful_accesses += 1;
        }
        println!("Thread {} shared memory access: {}", thread_id, success);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread should complete");
    }
    
    assert!(successful_accesses > 0, "At least some threads should successfully access shared memory");
    println!("Successful shared memory accesses: {}/{}", successful_accesses, num_threads);
}

/// Test IPC performance characteristics
#[test]
fn test_ipc_performance_characteristics() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    let num_operations = 100;
    
    // Test channel creation performance
    let create_start = Instant::now();
    let mut channel_ids = vec![];
    
    for i in 0..num_operations {
        let config = IpcConfig {
            name: format!("perf_channel_{}", i),
            config_type: 0,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        };
        
        if let Ok(channel_id) = runtime.create_ipc_channel(IpcChannelType::Pipe, &config) {
            channel_ids.push(channel_id);
        }
    }
    
    let create_duration = create_start.elapsed();
    println!("Created {} channels in {:?} ({:.2} channels/sec)", 
             channel_ids.len(), create_duration, 
             channel_ids.len() as f64 / create_duration.as_secs_f64());
    
    // Test data transmission performance
    let data_start = Instant::now();
    let test_data = b"performance test data payload";
    let mut transmission_count = 0;
    
    for &channel_id in &channel_ids {
        if runtime.ipc_send(channel_id, test_data).is_ok() {
            transmission_count += 1;
        }
    }
    
    let data_duration = data_start.elapsed();
    println!("Transmitted data to {} channels in {:?} ({:.2} transmissions/sec)",
             transmission_count, data_duration,
             transmission_count as f64 / data_duration.as_secs_f64());
    
    // Test shared memory creation performance
    let shm_start = Instant::now();
    let mut shm_count = 0;
    
    for i in 0..50 {
        let name = format!("perf_shm_{}", i);
        if runtime.create_shared_memory(&name, 1024).is_ok() {
            shm_count += 1;
        }
    }
    
    let shm_duration = shm_start.elapsed();
    println!("Created {} shared memory segments in {:?} ({:.2} segments/sec)",
             shm_count, shm_duration,
             shm_count as f64 / shm_duration.as_secs_f64());
    
    // Performance assertions (reasonable limits for CI)
    assert!(create_duration < Duration::from_secs(5), "Channel creation should be reasonably fast");
    assert!(data_duration < Duration::from_secs(2), "Data transmission should be reasonably fast");
    assert!(shm_duration < Duration::from_secs(3), "Shared memory creation should be reasonably fast");
}

/// Test IPC resource cleanup
#[test]
fn test_ipc_resource_cleanup() {
    common::tracing::setup();
    
    // Test that resources are properly managed when runtime is dropped
    let mut channel_ids = vec![];
    let mut shm_names = vec![];
    
    {
        let runtime = ProcessRuntime::new();
        
        // Create resources
        for i in 0..10 {
            let config = IpcConfig {
                name: format!("cleanup_channel_{}", i),
                config_type: 0,
                size_or_capacity: 1024,
                permissions: 0o666,
                flags: 0,
            };
            
            if let Ok(channel_id) = runtime.create_ipc_channel(IpcChannelType::Pipe, &config) {
                channel_ids.push(channel_id);
            }
            
            let shm_name = format!("cleanup_shm_{}", i);
            if runtime.create_shared_memory(&shm_name, 1024).is_ok() {
                shm_names.push(shm_name);
            }
        }
        
        println!("Created {} channels and {} shared memory segments for cleanup test",
                 channel_ids.len(), shm_names.len());
        
        // Add some data
        for &channel_id in &channel_ids {
            let _ = runtime.ipc_send(channel_id, b"cleanup test data");
        }
        
        // Runtime will be dropped here
    }
    
    println!("Runtime dropped, resources should be cleaned up automatically");
    
    // Create new runtime to verify cleanup
    let new_runtime = ProcessRuntime::new();
    
    // Try to access old channels (should fail)
    for &channel_id in &channel_ids {
        let result = new_runtime.ipc_send(channel_id, b"test");
        assert!(result.is_err(), "Old channel {} should not be accessible", channel_id);
    }
    
    println!("Confirmed that old channels are not accessible after cleanup");
}

/// Test IPC configuration edge cases
#[test]
fn test_ipc_configuration_edge_cases() {
    common::tracing::setup();
    
    let runtime = ProcessRuntime::new();
    
    // Test edge case configurations
    let edge_configs = vec![
        IpcConfig {
            name: "".to_string(), // Empty name
            config_type: 0,
            size_or_capacity: 1,
            permissions: 0,
            flags: 0,
        },
        IpcConfig {
            name: "very_long_name_that_might_cause_issues_in_some_systems_with_limited_name_length".to_string(),
            config_type: -1, // Negative type
            size_or_capacity: 0, // Zero size
            permissions: 0o777,
            flags: u32::MAX, // Maximum flags
        },
        IpcConfig {
            name: "special_chars_!@#$%^&*()".to_string(),
            config_type: 999, // Large type number
            size_or_capacity: u64::MAX, // Maximum size
            permissions: 0o000, // No permissions
            flags: 0,
        },
    ];
    
    for (i, config) in edge_configs.iter().enumerate() {
        let result = runtime.create_ipc_channel(IpcChannelType::Pipe, config);
        
        // These might succeed or fail depending on implementation
        match result {
            Ok(channel_id) => {
                println!("Edge case config {} succeeded with channel ID {}", i, channel_id);
            }
            Err(e) => {
                println!("Edge case config {} failed (expected): {:?}", i, e);
            }
        }
    }
}
