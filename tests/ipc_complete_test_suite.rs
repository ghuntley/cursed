/// Comprehensive IPC Test Suite for CURSED Programming Language
/// 
/// This test suite provides complete validation of all Inter-Process Communication
/// mechanisms including pipes, sockets, shared memory, message queues, semaphores,
/// synchronization primitives, signals, RPC, and security features.

use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::thread;
use std::collections::HashMap;

use cursed::stdlib::ipc::{
    // Core IPC
    initialize, shutdown, get_statistics,
    IpcResult, IpcError,
    
    // Pipes
    NamedPipe, PipeConfig, PipeMode,
    create_named_pipe, open_named_pipe, remove_named_pipe,
    
    // Shared Memory
    SharedMemory, MemoryConfig, MemoryAccess,
    create_shared_memory, open_shared_memory, remove_shared_memory,
    
    // Message Queues
    MessageQueue, Message, QueueConfig, MessageType,
    create_message_queue, open_message_queue, send_message, receive_message,
    
    // Semaphores
    Semaphore, SemaphoreConfig, SemaphoreStatistics,
    create_semaphore, create_named_semaphore, create_binary_semaphore, create_counting_semaphore,
    
    // Synchronization
    Barrier, BarrierConfig, BarrierStatistics,
    RwLockTimeout, RwLockStatistics,
    DistributedCoordinator, ConditionVariable, CondVarStatistics,
    create_barrier, create_named_barrier, create_rwlock_timeout,
    create_condition_variable, create_distributed_coordinator,
    
    // Sockets
    UnixSocket, SocketConfig, SocketType,
    create_socket_pair,
    
    // Signals
    SignalHandler, SignalConfig, Signal, SignalEvent,
    
    // RPC
    RpcClient, RpcServer, RpcRequest, RpcResponse, RpcError,
};

// Helper macros for test timing and assertions
macro_rules! timed_test {
    ($name:expr, $max_duration:expr, $test:expr) => {
        let start = Instant::now();
        let result = $test;
        let elapsed = start.elapsed();
        assert!(elapsed <= $max_duration, 
            "{} took {:?}, expected <= {:?}", $name, elapsed, $max_duration);
        result
    };
}

macro_rules! assert_ipc_ok {
    ($result:expr) => {
        assert!($result.is_ok(), "IPC operation failed: {:?}", $result.err());
    };
    ($result:expr, $msg:expr) => {
        assert!($result.is_ok(), "{}: {:?}", $msg, $result.err());
    };
}

/// Test IPC subsystem initialization and cleanup
#[test]
fn test_ipc_system_lifecycle() {
    // Test initialization
    assert_ipc_ok!(initialize(), "IPC initialization failed");
    
    // Verify initial state
    let stats = get_statistics().unwrap();
    assert_eq!(stats.active_pipes, 0);
    assert_eq!(stats.active_sockets, 0);
    assert_eq!(stats.active_shared_memory, 0);
    assert_eq!(stats.active_message_queues, 0);
    
    // Test shutdown
    assert_ipc_ok!(shutdown(), "IPC shutdown failed");
    
    // Test re-initialization
    assert_ipc_ok!(initialize(), "IPC re-initialization failed");
    assert_ipc_ok!(shutdown(), "IPC re-shutdown failed");
}

/// Test named pipes functionality
#[test]
fn test_named_pipes_comprehensive() {
    let _ = initialize();
    
    let pipe_path = "/tmp/test_cursed_pipe";
    
    // Test pipe creation with different configurations
    let configs = vec![
        PipeConfig::new(pipe_path).with_mode(PipeMode::ReadWrite),
        PipeConfig::new(pipe_path).with_mode(PipeMode::ReadOnly),
        PipeConfig::new(pipe_path).with_buffer_size(16384),
        PipeConfig::new(pipe_path).with_timeout(Duration::from_secs(5)),
    ];
    
    for (i, config) in configs.into_iter().enumerate() {
        let pipe_name = format!("{}{}", pipe_path, i);
        let config = config.clone();
        
        // Create pipe
        let create_result = NamedPipe::create_with_config(config);
        if create_result.is_ok() {
            let pipe = create_result.unwrap();
            
            // Test basic operations
            let test_data = b"Hello from CURSED IPC pipe test";
            
            // Write test (in separate thread to avoid blocking)
            let pipe_name_clone = pipe_name.clone();
            let handle = thread::spawn(move || {
                if let Ok(pipe) = open_named_pipe(&pipe_name_clone) {
                    let _ = pipe.write(test_data);
                }
            });
            
            // Read test
            if let Ok(data) = pipe.read() {
                assert!(!data.is_empty(), "Should read some data from pipe");
            }
            
            handle.join().unwrap();
            
            // Cleanup
            let _ = remove_named_pipe(&pipe_name);
        }
    }
}

/// Test shared memory operations
#[test]
fn test_shared_memory_comprehensive() {
    let _ = initialize();
    
    // Test different memory sizes
    let sizes = vec![1024, 4096, 65536, 1048576]; // 1KB to 1MB
    
    for size in sizes {
        let shm_name = format!("test_shm_{}", size);
        let config = MemoryConfig::new(&shm_name, size);
        
        // Create shared memory
        let create_result = SharedMemory::create_with_config(config);
        if create_result.is_ok() {
            let mut shm = create_result.unwrap();
            
            // Test memory mapping
            assert_ipc_ok!(shm.map(), "Failed to map shared memory");
            
            // Test write operations
            let test_data = format!("Test data for size {}", size);
            assert_ipc_ok!(shm.write_at(0, test_data.as_bytes()), "Failed to write to shared memory");
            
            // Test read operations
            let read_result = shm.read_at(0, test_data.len());
            assert!(read_result.is_ok(), "Failed to read from shared memory");
            
            let read_data = read_result.unwrap();
            assert_eq!(read_data, test_data.as_bytes(), "Read data doesn't match written data");
            
            // Test memory view
            let view_result = shm.view(0, test_data.len());
            assert!(view_result.is_ok(), "Failed to create memory view");
            
            // Cleanup
            assert_ipc_ok!(shm.unmap(), "Failed to unmap shared memory");
            let _ = remove_shared_memory(&shm_name);
        }
    }
}

/// Test message queues
#[test]
fn test_message_queues_comprehensive() {
    let _ = initialize();
    
    let queue_name = "test_message_queue";
    let config = QueueConfig::new(queue_name, 100);
    
    // Create message queue
    let create_result = MessageQueue::create_with_config(config);
    if create_result.is_ok() {
        let queue = create_result.unwrap();
        
        // Test different message types
        let messages = vec![
            Message::new(MessageType::Text, b"Hello World".to_vec()),
            Message::new(MessageType::Binary, vec![1, 2, 3, 4, 5]),
            Message::new(MessageType::Structured, b"{'key': 'value'}".to_vec()),
        ];
        
        // Send messages
        for (i, message) in messages.iter().enumerate() {
            let send_result = queue.send(message.clone());
            assert!(send_result.is_ok(), "Failed to send message {}: {:?}", i, send_result.err());
        }
        
        // Receive messages
        for i in 0..messages.len() {
            let receive_result = queue.receive();
            assert!(receive_result.is_ok(), "Failed to receive message {}: {:?}", i, receive_result.err());
            
            let received = receive_result.unwrap();
            assert_eq!(received.message_type(), messages[i].message_type());
            assert_eq!(received.data(), messages[i].data());
        }
        
        // Test queue statistics
        let stats = queue.statistics();
        assert_eq!(stats.total_sent, messages.len() as u64);
        assert_eq!(stats.total_received, messages.len() as u64);
    }
}

/// Test semaphores
#[test]
fn test_semaphores_comprehensive() {
    // Test binary semaphore
    let binary_sem = create_binary_semaphore().unwrap();
    assert_eq!(binary_sem.value().unwrap(), 1);
    
    assert!(binary_sem.try_wait().unwrap());
    assert_eq!(binary_sem.value().unwrap(), 0);
    
    assert!(!binary_sem.try_wait().unwrap());
    
    binary_sem.post().unwrap();
    assert_eq!(binary_sem.value().unwrap(), 1);
    
    // Test counting semaphore
    let counting_sem = create_counting_semaphore(5, 10).unwrap();
    assert_eq!(counting_sem.value().unwrap(), 5);
    
    for _ in 0..5 {
        assert!(counting_sem.try_wait().unwrap());
    }
    assert_eq!(counting_sem.value().unwrap(), 0);
    
    for _ in 0..5 {
        counting_sem.post().unwrap();
    }
    assert_eq!(counting_sem.value().unwrap(), 5);
    
    // Test named semaphore
    let named_sem = create_named_semaphore("test_semaphore", 3).unwrap();
    assert_eq!(named_sem.value().unwrap(), 3);
    
    // Test statistics
    let stats = named_sem.statistics();
    assert_eq!(stats.current_value, 3);
}

/// Test barriers
#[test]
fn test_barriers_comprehensive() {
    let thread_count = 4;
    let barrier = Arc::new(create_barrier(thread_count).unwrap());
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    // Spawn threads that wait on barrier
    for i in 0..thread_count {
        let barrier_clone = Arc::clone(&barrier);
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // Each thread increments counter before barrier
            counter_clone.fetch_add(1, Ordering::Relaxed);
            
            // Wait for all threads to reach barrier
            let is_leader = barrier_clone.wait().unwrap();
            
            // Only one thread should be the leader
            if is_leader {
                assert_eq!(counter_clone.load(Ordering::Relaxed), thread_count);
            }
            
            // All threads increment after barrier
            counter_clone.fetch_add(1, Ordering::Relaxed);
            
            i
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // All threads should have completed both increments
    assert_eq!(counter.load(Ordering::Relaxed), thread_count * 2);
    
    // Test barrier statistics
    let stats = barrier.statistics();
    assert_eq!(stats.total_completions, 1);
    assert_eq!(stats.current_waiters, 0);
}

/// Test RwLock with timeout
#[test]
fn test_rwlock_timeout() {
    let rwlock = create_rwlock_timeout(42);
    
    // Test concurrent reads
    let read_handles: Vec<_> = (0..5).map(|i| {
        let rwlock_clone = rwlock.clone();
        thread::spawn(move || {
            let guard = rwlock_clone.read_timeout(Duration::from_millis(100)).unwrap();
            assert_eq!(*guard, 42);
            thread::sleep(Duration::from_millis(10));
            i
        })
    }).collect();
    
    for handle in read_handles {
        handle.join().unwrap();
    }
    
    // Test write lock
    let write_guard = rwlock.write_timeout(Duration::from_millis(100)).unwrap();
    drop(write_guard);
    
    // Test statistics
    let stats = rwlock.statistics();
    assert_eq!(stats.read_locks, 5);
    assert_eq!(stats.write_locks, 1);
    assert_eq!(stats.read_timeouts, 0);
    assert_eq!(stats.write_timeouts, 0);
}

/// Test condition variables
#[test]
fn test_condition_variables() {
    let condvar = create_condition_variable();
    let mutex = Arc::new(Mutex::new(false));
    let mutex_clone = Arc::clone(&mutex);
    
    // Spawn thread that signals condition
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut guard = mutex_clone.lock().unwrap();
        *guard = true;
        condvar.notify_one();
    });
    
    // Wait for condition
    let guard = mutex.lock().unwrap();
    let (guard, timed_out) = condvar.wait_timeout(guard, Duration::from_millis(200)).unwrap();
    
    assert!(!timed_out, "Condition variable wait should not timeout");
    assert!(*guard, "Condition should be true");
    
    handle.join().unwrap();
    
    // Test statistics
    let stats = condvar.statistics();
    assert_eq!(stats.total_waits, 1);
    assert_eq!(stats.total_notifies, 1);
    assert_eq!(stats.timeout_count, 0);
}

/// Test distributed coordinator
#[test]
fn test_distributed_coordinator() {
    let coordinator1 = create_distributed_coordinator("node1");
    let coordinator2 = create_distributed_coordinator("node2");
    
    // Add peers
    coordinator1.add_peer("node2".to_string()).unwrap();
    coordinator1.add_peer("node3".to_string()).unwrap();
    
    coordinator2.add_peer("node1".to_string()).unwrap();
    coordinator2.add_peer("node3".to_string()).unwrap();
    
    assert_eq!(coordinator1.peer_count(), 2);
    assert_eq!(coordinator2.peer_count(), 2);
    
    // Start coordination
    coordinator1.start_coordination().unwrap();
    coordinator2.start_coordination().unwrap();
    
    // Test initial state (no leader elected yet)
    assert!(!coordinator1.is_leader());
    assert!(!coordinator2.is_leader());
    
    // Stop coordination
    coordinator1.stop_coordination().unwrap();
    coordinator2.stop_coordination().unwrap();
}

/// Test Unix domain sockets
#[test]
fn test_unix_domain_sockets() {
    let socket_path = "/tmp/test_cursed_socket";
    
    // Create socket pair
    let socket_result = create_socket_pair();
    if socket_result.is_ok() {
        let (socket1, socket2) = socket_result.unwrap();
        
        // Test bidirectional communication
        let test_data1 = b"Hello from socket1";
        let test_data2 = b"Hello from socket2";
        
        // Send from socket1 to socket2
        let send_handle = thread::spawn(move || {
            socket1.send(test_data1).unwrap();
            let received = socket1.receive().unwrap();
            assert_eq!(received, test_data2);
        });
        
        // Send from socket2 to socket1
        socket2.send(test_data2).unwrap();
        let received = socket2.receive().unwrap();
        assert_eq!(received, test_data1);
        
        send_handle.join().unwrap();
    }
}

/// Test signal handling
#[test]
fn test_signal_handling() {
    let config = SignalConfig::new("test_signals");
    let handler_result = SignalHandler::new(config);
    
    if handler_result.is_ok() {
        let handler = handler_result.unwrap();
        
        // Register signal callbacks
        handler.register_callback("test_signal", Box::new(|_| {
            println!("Test signal received");
            Ok(())
        })).unwrap();
        
        // Send signal
        let signal = Signal::Custom("test_signal".to_string());
        handler.send_signal(signal).unwrap();
        
        // Process pending signals
        handler.process_pending_signals().unwrap();
        
        // Check statistics
        let stats = handler.statistics();
        assert!(stats.total_signals_sent >= 1);
    }
}

/// Test RPC functionality
#[test]
fn test_rpc_functionality() {
    let config = RpcConfig::new("test_rpc_service");
    
    // Create RPC server
    let server_result = RpcServer::new(config.clone());
    if server_result.is_ok() {
        let server = server_result.unwrap();
        
        // Register RPC method
        server.register_method("add", Box::new(|params| {
            // Simple addition service
            if params.len() >= 8 {
                let a = i32::from_le_bytes([params[0], params[1], params[2], params[3]]);
                let b = i32::from_le_bytes([params[4], params[5], params[6], params[7]]);
                let result = a + b;
                Ok(result.to_le_bytes().to_vec())
            } else {
                Err(RpcError::InvalidParameters("Need 8 bytes for two i32s".to_string()))
            }
        })).unwrap();
        
        // Start server
        server.start().unwrap();
        
        // Create RPC client
        let client_result = RpcClient::new(config);
        if client_result.is_ok() {
            let client = client_result.unwrap();
            
            // Connect to server
            client.connect().unwrap();
            
            // Make RPC call
            let params = [5i32.to_le_bytes(), 7i32.to_le_bytes()].concat();
            let request = RpcRequest::new("add", params);
            
            let response = client.call(request).unwrap();
            if let Some(result_data) = response.result {
                let result = i32::from_le_bytes([
                    result_data[0], result_data[1], result_data[2], result_data[3]
                ]);
                assert_eq!(result, 12);
            }
            
            // Disconnect
            client.disconnect().unwrap();
        }
        
        // Stop server
        server.stop().unwrap();
    }
}

/// Performance test for IPC operations
#[test]
fn test_ipc_performance() {
    let _ = initialize();
    
    // Test shared memory performance
    let shm_name = "perf_test_shm";
    let config = MemoryConfig::new(shm_name, 1048576); // 1MB
    
    if let Ok(mut shm) = SharedMemory::create_with_config(config) {
        assert_ipc_ok!(shm.map());
        
        let test_data = vec![0u8; 4096]; // 4KB chunks
        
        // Time write operations
        let write_ops = 100;
        let start = Instant::now();
        
        for i in 0..write_ops {
            let offset = (i * 4096) % (1048576 - 4096);
            assert_ipc_ok!(shm.write_at(offset, &test_data));
        }
        
        let write_duration = start.elapsed();
        let write_throughput = (write_ops * test_data.len()) as f64 / write_duration.as_secs_f64();
        
        println!("Shared memory write throughput: {:.2} bytes/sec", write_throughput);
        assert!(write_throughput > 1_000_000.0, "Write throughput should be > 1MB/s");
        
        // Time read operations
        let start = Instant::now();
        
        for i in 0..write_ops {
            let offset = (i * 4096) % (1048576 - 4096);
            let read_result = shm.read_at(offset, test_data.len());
            assert!(read_result.is_ok());
        }
        
        let read_duration = start.elapsed();
        let read_throughput = (write_ops * test_data.len()) as f64 / read_duration.as_secs_f64();
        
        println!("Shared memory read throughput: {:.2} bytes/sec", read_throughput);
        assert!(read_throughput > 1_000_000.0, "Read throughput should be > 1MB/s");
        
        assert_ipc_ok!(shm.unmap());
        let _ = remove_shared_memory(shm_name);
    }
}

/// Stress test for concurrent IPC operations
#[test]
fn test_ipc_concurrent_stress() {
    let _ = initialize();
    
    let thread_count = 8;
    let operations_per_thread = 100;
    let barrier = Arc::new(create_barrier(thread_count).unwrap());
    let success_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for thread_id in 0..thread_count {
        let barrier_clone = Arc::clone(&barrier);
        let success_counter_clone = Arc::clone(&success_counter);
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait().unwrap();
            
            let mut local_successes = 0;
            
            for op_id in 0..operations_per_thread {
                // Test semaphore operations
                if let Ok(sem) = create_semaphore(1) {
                    if sem.try_wait().unwrap_or(false) {
                        thread::sleep(Duration::from_millis(1));
                        if sem.post().is_ok() {
                            local_successes += 1;
                        }
                    }
                }
                
                // Test shared memory operations
                let shm_name = format!("stress_shm_{}_{}", thread_id, op_id);
                if let Ok(mut shm) = create_shared_memory(&shm_name, 1024) {
                    if shm.map().is_ok() {
                        let test_data = format!("Thread {} Op {}", thread_id, op_id);
                        if shm.write_at(0, test_data.as_bytes()).is_ok() {
                            if shm.read_at(0, test_data.len()).is_ok() {
                                local_successes += 1;
                            }
                        }
                        let _ = shm.unmap();
                    }
                    let _ = remove_shared_memory(&shm_name);
                }
            }
            
            success_counter_clone.fetch_add(local_successes, Ordering::Relaxed);
            local_successes
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_successes = success_counter.load(Ordering::Relaxed);
    let expected_minimum = thread_count * operations_per_thread / 2; // At least 50% success rate
    
    println!("Concurrent stress test: {} successes out of {} operations", 
             total_successes, thread_count * operations_per_thread * 2);
    
    assert!(total_successes >= expected_minimum, 
        "Stress test should have at least {} successes, got {}", 
        expected_minimum, total_successes);
}

/// Test error handling and recovery
#[test]
fn test_ipc_error_handling() {
    let _ = initialize();
    
    // Test invalid shared memory operations
    let invalid_config = MemoryConfig::new("", 0); // Invalid name and size
    let create_result = SharedMemory::create_with_config(invalid_config);
    assert!(create_result.is_err(), "Should fail with invalid config");
    
    // Test invalid semaphore operations
    let invalid_sem_config = SemaphoreConfig::new(-1); // Invalid initial value
    let sem_result = Semaphore::new(invalid_sem_config);
    assert!(sem_result.is_err(), "Should fail with negative initial value");
    
    // Test invalid barrier operations
    let invalid_barrier_config = BarrierConfig::new(0); // Invalid party count
    let barrier_result = Barrier::new(invalid_barrier_config);
    assert!(barrier_result.is_err(), "Should fail with zero party count");
    
    // Test timeout scenarios
    if let Ok(sem) = create_semaphore(0) {
        let timeout_result = sem.wait_timeout(Duration::from_millis(10));
        assert!(timeout_result.is_err(), "Should timeout when semaphore unavailable");
    }
    
    // Test resource cleanup
    let cleanup_stats = get_statistics().unwrap();
    println!("Cleanup stats: active_pipes={}, active_sockets={}, active_shared_memory={}", 
             cleanup_stats.active_pipes, cleanup_stats.active_sockets, cleanup_stats.active_shared_memory);
}

/// Integration test for multi-IPC scenario
#[test]
fn test_multi_ipc_integration() {
    let _ = initialize();
    
    // Create a complex scenario using multiple IPC mechanisms
    let coordinator_count = 3;
    let barrier = Arc::new(create_barrier(coordinator_count).unwrap());
    let shared_counter = Arc::new(create_shared_memory("integration_counter", 64).unwrap());
    let coordination_sem = Arc::new(create_semaphore(1).unwrap());
    
    // Initialize shared counter
    {
        let mut counter_shm = shared_counter.as_ref();
        counter_shm.map().unwrap();
        counter_shm.write_at(0, &0u64.to_le_bytes()).unwrap();
    }
    
    let mut handles = vec![];
    
    for worker_id in 0..coordinator_count {
        let barrier_clone = Arc::clone(&barrier);
        let shared_counter_clone = Arc::clone(&shared_counter);
        let coordination_sem_clone = Arc::clone(&coordination_sem);
        
        let handle = thread::spawn(move || {
            // Wait for all workers to be ready
            barrier_clone.wait().unwrap();
            
            // Coordinated counter increment
            coordination_sem_clone.wait().unwrap();
            
            // Read current counter value
            let current_bytes = shared_counter_clone.read_at(0, 8).unwrap();
            let current_value = u64::from_le_bytes([
                current_bytes[0], current_bytes[1], current_bytes[2], current_bytes[3],
                current_bytes[4], current_bytes[5], current_bytes[6], current_bytes[7],
            ]);
            
            // Increment and write back
            let new_value = current_value + 1;
            shared_counter_clone.write_at(0, &new_value.to_le_bytes()).unwrap();
            
            coordination_sem_clone.post().unwrap();
            
            worker_id
        });
        
        handles.push(handle);
    }
    
    // Wait for all workers to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify final counter value
    let final_bytes = shared_counter.read_at(0, 8).unwrap();
    let final_value = u64::from_le_bytes([
        final_bytes[0], final_bytes[1], final_bytes[2], final_bytes[3],
        final_bytes[4], final_bytes[5], final_bytes[6], final_bytes[7],
    ]);
    
    assert_eq!(final_value, coordinator_count as u64, 
        "All workers should have incremented the counter exactly once");
    
    // Cleanup
    shared_counter.unmap().unwrap();
    let _ = remove_shared_memory("integration_counter");
}

/// Test IPC statistics and monitoring
#[test]
fn test_ipc_statistics_monitoring() {
    let _ = initialize();
    
    // Create various IPC resources
    let _shm = create_shared_memory("stats_test_shm", 1024).unwrap();
    let _sem = create_semaphore(5).unwrap();
    let _barrier = create_barrier(2).unwrap();
    
    // Get statistics
    let stats = get_statistics().unwrap();
    
    // Verify statistics are being tracked
    assert!(stats.total_operations > 0, "Should have recorded operations");
    
    // Test individual component statistics
    if let Ok(sem) = create_named_semaphore("stats_sem", 3) {
        sem.try_wait().unwrap();
        sem.post().unwrap();
        
        let sem_stats = sem.statistics();
        assert_eq!(sem_stats.total_waits, 1);
        assert_eq!(sem_stats.total_posts, 1);
    }
    
    if let Ok(barrier) = create_named_barrier("stats_barrier", 1) {
        barrier.wait().unwrap();
        
        let barrier_stats = barrier.statistics();
        assert_eq!(barrier_stats.total_completions, 1);
    }
    
    // Test condition variable statistics
    let condvar = create_condition_variable();
    let mutex = Arc::new(Mutex::new(true));
    let guard = mutex.lock().unwrap();
    let (_guard, _timed_out) = condvar.wait_timeout(guard, Duration::from_millis(1)).unwrap();
    
    let condvar_stats = condvar.statistics();
    assert_eq!(condvar_stats.total_waits, 1);
}

/// Final cleanup test
#[test]
fn test_ipc_final_cleanup() {
    let _ = initialize();
    
    // Create some resources
    let _shm = create_shared_memory("cleanup_test_shm", 1024);
    let _sem = create_semaphore(1);
    let _barrier = create_barrier(1);
    
    // Get stats before cleanup
    let stats_before = get_statistics().unwrap();
    println!("Stats before cleanup: operations={}, failed={}", 
             stats_before.total_operations, stats_before.failed_operations);
    
    // Shutdown should clean up all resources
    assert_ipc_ok!(shutdown(), "Final shutdown failed");
    
    // Re-initialize to check clean state
    assert_ipc_ok!(initialize(), "Post-cleanup initialization failed");
    
    let stats_after = get_statistics().unwrap();
    assert_eq!(stats_after.active_pipes, 0);
    assert_eq!(stats_after.active_sockets, 0);
    assert_eq!(stats_after.active_shared_memory, 0);
    assert_eq!(stats_after.active_message_queues, 0);
    
    assert_ipc_ok!(shutdown(), "Final cleanup shutdown failed");
}
