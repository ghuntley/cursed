/// Advanced Integration Tests for IPC System
/// 
/// This module provides comprehensive integration tests for advanced IPC features
/// including signal boost, connection pooling, process coordination, performance
/// under load, and real-world usage scenarios.

use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex, mpsc, Barrier};
use std::thread;
use std::collections::HashMap;

use cursed::runtime::process::{
    ProcessRuntime, IpcChannelType, IpcConfig, initialize_process_runtime,
    get_process_runtime, shutdown_process_runtime
};

#[path = "common.rs"]
mod common;

/// Test advanced IPC communication patterns
#[test]
fn test_advanced_ipc_communication_patterns() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    
    // Test producer-consumer pattern
    let producer_consumer_test = || {
        let config = IpcConfig {
            name: "producer_consumer_channel".to_string(),
            config_type: 0,
            size_or_capacity: 4096,
            permissions: 0o666,
            flags: 0,
        };
        
        let channel_id = runtime.create_ipc_channel(IpcChannelType::MessageQueue, &config)
            .expect("Should create producer-consumer channel");
        
        let runtime_producer = runtime.clone();
        let runtime_consumer = runtime.clone();
        let barrier = Arc::new(Barrier::new(2));
        let barrier_producer = barrier.clone();
        let barrier_consumer = barrier.clone();
        
        // Producer thread
        let producer_handle = thread::spawn(move || {
            barrier_producer.wait();
            
            let messages = vec![
                "Message 1: Hello",
                "Message 2: World", 
                "Message 3: IPC",
                "Message 4: Test",
                "Message 5: Complete",
            ];
            
            for (i, message) in messages.iter().enumerate() {
                match runtime_producer.ipc_send(channel_id, message.as_bytes()) {
                    Ok(_) => println!("Producer sent: {}", message),
                    Err(e) => println!("Producer failed to send message {}: {:?}", i, e),
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
        
        // Consumer thread
        let consumer_handle = thread::spawn(move || {
            barrier_consumer.wait();
            
            let mut received_messages = vec![];
            
            for i in 0..10 { // Try to receive up to 10 messages
                match runtime_consumer.ipc_receive(channel_id, 200) {
                    Ok(data_ptr) => {
                        if !data_ptr.is_null() {
                            let data = unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
                            let message = String::from_utf8_lossy(&data);
                            println!("Consumer received: {}", message);
                            received_messages.push(message.to_string());
                        } else {
                            println!("Consumer received null (timeout or no data)");
                        }
                    }
                    Err(e) => {
                        println!("Consumer receive error: {:?}", e);
                        break;
                    }
                }
            }
            
            received_messages
        });
        
        producer_handle.join().expect("Producer should complete");
        let received = consumer_handle.join().expect("Consumer should complete");
        
        println!("Producer-Consumer test: {} messages received", received.len());
        received.len()
    };
    
    let received_count = producer_consumer_test();
    assert!(received_count > 0, "Should receive at least some messages");
}

/// Test IPC connection pooling simulation
#[test]
fn test_ipc_connection_pooling() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let pool_size = 5;
    let clients = 10;
    
    // Create a pool of IPC channels
    let mut channel_pool = vec![];
    for i in 0..pool_size {
        let config = IpcConfig {
            name: format!("pool_channel_{}", i),
            config_type: 0,
            size_or_capacity: 2048,
            permissions: 0o666,
            flags: 0,
        };
        
        if let Ok(channel_id) = runtime.create_ipc_channel(IpcChannelType::Pipe, &config) {
            channel_pool.push(channel_id);
        }
    }
    
    println!("Created connection pool with {} channels", channel_pool.len());
    
    // Simulate multiple clients using the pool
    let pool = Arc::new(Mutex::new(channel_pool));
    let mut client_handles = vec![];
    let (tx, rx) = mpsc::channel();
    
    for client_id in 0..clients {
        let runtime_clone = runtime.clone();
        let pool_clone = pool.clone();
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            let mut operations_completed = 0;
            
            for op in 0..3 {
                // Get a channel from the pool
                let channel_id = {
                    let mut pool_guard = pool_clone.lock().unwrap();
                    if !pool_guard.is_empty() {
                        Some(pool_guard.remove(0))
                    } else {
                        None
                    }
                };
                
                if let Some(ch_id) = channel_id {
                    // Perform operation
                    let message = format!("Client {} operation {}", client_id, op);
                    
                    if runtime_clone.ipc_send(ch_id, message.as_bytes()).is_ok() {
                        operations_completed += 1;
                    }
                    
                    // Return channel to pool
                    {
                        let mut pool_guard = pool_clone.lock().unwrap();
                        pool_guard.push(ch_id);
                    }
                    
                    thread::sleep(Duration::from_millis(10));
                } else {
                    // No channels available, wait and retry
                    thread::sleep(Duration::from_millis(5));
                }
            }
            
            tx_clone.send((client_id, operations_completed)).unwrap();
        });
        
        client_handles.push(handle);
    }
    
    // Collect results
    drop(tx);
    let mut total_operations = 0;
    while let Ok((client_id, ops)) = rx.recv() {
        total_operations += ops;
        println!("Client {} completed {} operations", client_id, ops);
    }
    
    // Wait for all clients
    for handle in client_handles {
        handle.join().expect("Client thread should complete");
    }
    
    println!("Connection pooling test: {} total operations completed", total_operations);
    assert!(total_operations > 0, "Should complete some operations through connection pool");
}

/// Test process coordination using IPC
#[test]
fn test_process_coordination_ipc() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    
    // Create coordination channels
    let coordinator_config = IpcConfig {
        name: "coordinator_channel".to_string(),
        config_type: 0,
        size_or_capacity: 1024,
        permissions: 0o666,
        flags: 0,
    };
    
    let coord_channel = runtime.create_ipc_channel(IpcChannelType::MessageQueue, &coordinator_config)
        .expect("Should create coordinator channel");
    
    let worker_configs: Vec<_> = (0..3).map(|i| {
        IpcConfig {
            name: format!("worker_channel_{}", i),
            config_type: 0,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        }
    }).collect();
    
    let worker_channels: Vec<_> = worker_configs.iter()
        .map(|config| runtime.create_ipc_channel(IpcChannelType::Pipe, config))
        .collect::<Result<Vec<_>, _>>()
        .expect("Should create worker channels");
    
    // Coordinator thread
    let runtime_coordinator = runtime.clone();
    let coord_handle = thread::spawn(move || {
        let tasks = vec!["task_1", "task_2", "task_3", "task_4", "task_5"];
        let mut task_index = 0;
        let mut completed_tasks = 0;
        
        // Distribute initial tasks
        for &worker_channel in &worker_channels {
            if task_index < tasks.len() {
                let task = format!("TASK:{}", tasks[task_index]);
                if runtime_coordinator.ipc_send(worker_channel, task.as_bytes()).is_ok() {
                    println!("Coordinator assigned {} to worker", tasks[task_index]);
                    task_index += 1;
                }
            }
        }
        
        // Listen for completion and assign new tasks
        for _ in 0..10 { // Listen for up to 10 messages
            match runtime_coordinator.ipc_receive(coord_channel, 500) {
                Ok(data_ptr) => {
                    if !data_ptr.is_null() {
                        let data = unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
                        let message = String::from_utf8_lossy(&data);
                        
                        if message.starts_with("COMPLETED:") {
                            completed_tasks += 1;
                            println!("Coordinator received completion: {}", message);
                            
                            // Assign next task if available
                            if task_index < tasks.len() {
                                // Find available worker (simplified)
                                for &worker_channel in &worker_channels {
                                    let task = format!("TASK:{}", tasks[task_index]);
                                    if runtime_coordinator.ipc_send(worker_channel, task.as_bytes()).is_ok() {
                                        println!("Coordinator assigned {} to worker", tasks[task_index]);
                                        task_index += 1;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        }
        
        completed_tasks
    });
    
    // Worker threads
    let mut worker_handles = vec![];
    for (worker_id, &worker_channel) in worker_channels.iter().enumerate() {
        let runtime_worker = runtime.clone();
        
        let handle = thread::spawn(move || {
            let mut tasks_completed = 0;
            
            for _ in 0..5 { // Try to process up to 5 tasks
                match runtime_worker.ipc_receive(worker_channel, 300) {
                    Ok(data_ptr) => {
                        if !data_ptr.is_null() {
                            let data = unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
                            let message = String::from_utf8_lossy(&data);
                            
                            if message.starts_with("TASK:") {
                                let task_name = message.strip_prefix("TASK:").unwrap_or("unknown");
                                println!("Worker {} processing {}", worker_id, task_name);
                                
                                // Simulate work
                                thread::sleep(Duration::from_millis(50));
                                
                                // Report completion
                                let completion = format!("COMPLETED:{}:{}", worker_id, task_name);
                                if runtime_worker.ipc_send(coord_channel, completion.as_bytes()).is_ok() {
                                    tasks_completed += 1;
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            
            tasks_completed
        });
        
        worker_handles.push(handle);
    }
    
    // Wait for all threads
    let coordinator_completed = coord_handle.join().expect("Coordinator should complete");
    let mut total_worker_completed = 0;
    
    for (worker_id, handle) in worker_handles.into_iter().enumerate() {
        let worker_completed = handle.join().expect("Worker should complete");
        total_worker_completed += worker_completed;
        println!("Worker {} completed {} tasks", worker_id, worker_completed);
    }
    
    println!("Process coordination test: Coordinator saw {} completions, Workers completed {} tasks",
             coordinator_completed, total_worker_completed);
}

/// Test IPC performance under sustained load
#[test]
fn test_ipc_performance_under_load() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let num_channels = 20;
    let messages_per_channel = 50;
    let message_size = 1024;
    
    // Create channels
    let mut channels = vec![];
    for i in 0..num_channels {
        let config = IpcConfig {
            name: format!("load_test_channel_{}", i),
            config_type: 0,
            size_or_capacity: message_size * 2,
            permissions: 0o666,
            flags: 0,
        };
        
        if let Ok(channel_id) = runtime.create_ipc_channel(IpcChannelType::Pipe, &config) {
            channels.push(channel_id);
        }
    }
    
    println!("Created {} channels for load testing", channels.len());
    
    let start_time = Instant::now();
    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();
    
    // Spawn threads to stress test each channel
    for (thread_id, &channel_id) in channels.iter().enumerate() {
        let runtime_clone = runtime.clone();
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            let mut successful_operations = 0;
            let test_data = vec![42u8; message_size];
            
            for msg_id in 0..messages_per_channel {
                // Send message
                let send_start = Instant::now();
                if runtime_clone.ipc_send(channel_id, &test_data).is_ok() {
                    let send_duration = send_start.elapsed();
                    
                    // Try to receive message
                    let recv_start = Instant::now();
                    if let Ok(data_ptr) = runtime_clone.ipc_receive(channel_id, 200) {
                        let recv_duration = recv_start.elapsed();
                        
                        if !data_ptr.is_null() {
                            let received = unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
                            if received.len() == test_data.len() {
                                successful_operations += 1;
                            }
                        }
                        
                        // Log slow operations
                        if send_duration > Duration::from_millis(10) || recv_duration > Duration::from_millis(10) {
                            println!("Thread {} msg {}: slow operation (send: {:?}, recv: {:?})",
                                   thread_id, msg_id, send_duration, recv_duration);
                        }
                    }
                }
                
                // Brief pause to avoid overwhelming the system
                if msg_id % 10 == 0 {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            
            tx_clone.send((thread_id, successful_operations)).unwrap();
        });
        
        handles.push(handle);
    }
    
    // Collect results
    drop(tx);
    let mut total_successful = 0;
    let mut results = HashMap::new();
    
    while let Ok((thread_id, successful)) = rx.recv() {
        total_successful += successful;
        results.insert(thread_id, successful);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Load test thread should complete");
    }
    
    let total_time = start_time.elapsed();
    let total_expected = num_channels * messages_per_channel;
    let success_rate = (total_successful as f64 / total_expected as f64) * 100.0;
    
    println!("Load test results:");
    println!("  Total time: {:?}", total_time);
    println!("  Successful operations: {}/{} ({:.1}%)", total_successful, total_expected, success_rate);
    println!("  Operations per second: {:.1}", total_successful as f64 / total_time.as_secs_f64());
    
    for (thread_id, successful) in results {
        let thread_success_rate = (successful as f64 / messages_per_channel as f64) * 100.0;
        println!("  Thread {}: {}/{} ({:.1}%)", thread_id, successful, messages_per_channel, thread_success_rate);
    }
    
    // Performance assertions
    assert!(success_rate > 50.0, "Should have at least 50% success rate under load");
    assert!(total_time < Duration::from_secs(30), "Load test should complete in reasonable time");
}

/// Test IPC with global runtime integration
#[test]
fn test_ipc_global_runtime_integration() {
    common::tracing::setup();
    
    // Clean up any existing runtime
    shutdown_process_runtime();
    
    // Initialize global runtime
    initialize_process_runtime();
    
    let runtime = get_process_runtime().expect("Global runtime should be available");
    
    // Test creating channels through global runtime
    let channels = (0..5).map(|i| {
        let config = IpcConfig {
            name: format!("global_ipc_test_{}", i),
            config_type: 0,
            size_or_capacity: 1024,
            permissions: 0o666,
            flags: 0,
        };
        
        runtime.create_ipc_channel(IpcChannelType::Pipe, &config)
    }).collect::<Result<Vec<_>, _>>().expect("Should create channels through global runtime");
    
    println!("Created {} channels through global runtime", channels.len());
    
    // Test access from multiple threads
    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();
    
    for (thread_id, &channel_id) in channels.iter().enumerate() {
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            // Access global runtime from thread
            if let Some(thread_runtime) = get_process_runtime() {
                let test_data = format!("global_test_data_{}", thread_id);
                
                let send_result = thread_runtime.ipc_send(channel_id, test_data.as_bytes());
                let receive_result = thread_runtime.ipc_receive(channel_id, 100);
                
                let success = send_result.is_ok() && receive_result.is_ok();
                if let Ok(data_ptr) = receive_result {
                    if !data_ptr.is_null() {
                        let _ = unsafe { Box::from_raw(data_ptr as *mut Vec<u8>) };
                    }
                }
                
                tx_clone.send((thread_id, success)).unwrap();
            } else {
                tx_clone.send((thread_id, false)).unwrap();
            }
        });
        
        handles.push(handle);
    }
    
    // Collect results
    drop(tx);
    let mut successful_threads = 0;
    while let Ok((thread_id, success)) = rx.recv() {
        if success {
            successful_threads += 1;
        }
        println!("Thread {} global runtime access: {}", thread_id, success);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread should complete");
    }
    
    println!("Global runtime integration: {}/{} threads successful", successful_threads, channels.len());
    assert!(successful_threads > 0, "At least some threads should successfully use global runtime");
    
    // Cleanup
    shutdown_process_runtime();
}

/// Test IPC error recovery under stress
#[test]
fn test_ipc_error_recovery_under_stress() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let num_threads = 8;
    let operations_per_thread = 25;
    
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    // Spawn threads that perform operations with intentional errors
    for thread_id in 0..num_threads {
        let runtime_clone = runtime.clone();
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            let mut successful_operations = 0;
            let mut error_recoveries = 0;
            
            for op_id in 0..operations_per_thread {
                // Mix of valid and invalid operations
                if op_id % 3 == 0 {
                    // Invalid operation (non-existent channel)
                    let invalid_channel = (thread_id * 1000 + op_id) as u64;
                    let error_result = runtime_clone.ipc_send(invalid_channel, b"error test");
                    
                    if error_result.is_err() {
                        error_recoveries += 1;
                        
                        // Try valid operation after error
                        let config = IpcConfig {
                            name: format!("recovery_channel_{}_{}", thread_id, op_id),
                            config_type: 0,
                            size_or_capacity: 512,
                            permissions: 0o666,
                            flags: 0,
                        };
                        
                        if let Ok(channel_id) = runtime_clone.create_ipc_channel(IpcChannelType::Pipe, &config) {
                            if runtime_clone.ipc_send(channel_id, b"recovery test").is_ok() {
                                successful_operations += 1;
                            }
                        }
                    }
                } else {
                    // Valid operation
                    let config = IpcConfig {
                        name: format!("stress_channel_{}_{}", thread_id, op_id),
                        config_type: 0,
                        size_or_capacity: 1024,
                        permissions: 0o666,
                        flags: 0,
                    };
                    
                    if let Ok(channel_id) = runtime_clone.create_ipc_channel(IpcChannelType::Pipe, &config) {
                        if runtime_clone.ipc_send(channel_id, b"stress test").is_ok() {
                            successful_operations += 1;
                        }
                    }
                }
                
                // Brief pause
                if op_id % 5 == 0 {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            
            tx_clone.send((thread_id, successful_operations, error_recoveries)).unwrap();
        });
        
        handles.push(handle);
    }
    
    // Collect results
    drop(tx);
    let mut total_successful = 0;
    let mut total_recoveries = 0;
    
    while let Ok((thread_id, successful, recoveries)) = rx.recv() {
        total_successful += successful;
        total_recoveries += recoveries;
        println!("Thread {}: {} successful operations, {} error recoveries", 
                 thread_id, successful, recoveries);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Stress test thread should complete");
    }
    
    println!("Error recovery stress test: {} total successful operations, {} error recoveries",
             total_successful, total_recoveries);
    
    assert!(total_successful > 0, "Should have some successful operations despite errors");
    assert!(total_recoveries > 0, "Should recover from errors");
}

/// Test shared memory under concurrent access patterns
#[test]
fn test_shared_memory_concurrent_access_patterns() {
    common::tracing::setup();
    
    let runtime = Arc::new(ProcessRuntime::new());
    let shm_segments = vec!["reader_writer_shm", "producer_consumer_shm", "multi_access_shm"];
    
    // Create shared memory segments
    for segment_name in &shm_segments {
        let result = runtime.create_shared_memory(segment_name, 8192);
        assert!(result.is_ok(), "Should create shared memory segment {}", segment_name);
    }
    
    let num_accessors = 6;
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    
    // Spawn threads with different access patterns
    for accessor_id in 0..num_accessors {
        let runtime_clone = runtime.clone();
        let tx_clone = tx.clone();
        let segments = shm_segments.clone();
        
        let handle = thread::spawn(move || {
            let mut access_count = 0;
            
            for round in 0..10 {
                for segment_name in &segments {
                    // Try to access shared memory
                    match runtime_clone.create_shared_memory(segment_name, 8192) {
                        Ok(ptr) => {
                            if !ptr.is_null() {
                                access_count += 1;
                                
                                // Simulate different access patterns
                                match accessor_id % 3 {
                                    0 => {
                                        // Reader pattern - just access
                                        thread::sleep(Duration::from_millis(1));
                                    }
                                    1 => {
                                        // Writer pattern - longer access
                                        thread::sleep(Duration::from_millis(2));
                                    }
                                    2 => {
                                        // Quick access pattern
                                        // No additional sleep
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Err(e) => {
                            println!("Accessor {} failed to access {}: {:?}", 
                                   accessor_id, segment_name, e);
                        }
                    }
                }
                
                // Brief pause between rounds
                thread::sleep(Duration::from_millis(5));
            }
            
            tx_clone.send((accessor_id, access_count)).unwrap();
        });
        
        handles.push(handle);
    }
    
    // Collect results
    drop(tx);
    let mut total_accesses = 0;
    while let Ok((accessor_id, accesses)) = rx.recv() {
        total_accesses += accesses;
        println!("Accessor {} completed {} shared memory accesses", accessor_id, accesses);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().expect("Shared memory accessor thread should complete");
    }
    
    let expected_accesses = num_accessors * 10 * shm_segments.len();
    let success_rate = (total_accesses as f64 / expected_accesses as f64) * 100.0;
    
    println!("Shared memory concurrent access test: {}/{} accesses ({:.1}% success rate)",
             total_accesses, expected_accesses, success_rate);
    
    assert!(success_rate > 80.0, "Should have high success rate for shared memory access");
}
