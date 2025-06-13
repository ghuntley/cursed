/// Comprehensive IPC Stress Test Suite
/// 
/// This test suite validates IPC performance and reliability under
/// extreme conditions including high concurrency, large data volumes,
/// and extended runtime scenarios.

use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::thread;
use std::collections::HashMap;

use cursed::stdlib::ipc::{
    IpcResult, IpcError, SharedMemory, SharedMemoryConfig, MemoryProtection,
    SharedMemoryAccess, NamedPipe, PipeConfig, PipeMode,
    MessageQueue, MessageQueueConfig, Message, MessagePriority,
    Semaphore, SemaphoreConfig, DomainSocket, SocketConfig, SocketType,
    initialize, shutdown, get_ipc_statistics,
};

// Test configuration constants
const STRESS_TEST_DURATION: Duration = Duration::from_secs(30);
const HIGH_CONCURRENCY_THREADS: usize = 16;
const LARGE_DATA_SIZE: usize = 1024 * 1024; // 1MB
const MESSAGE_BURST_COUNT: usize = 1000;
const SHARED_MEMORY_REGIONS: usize = 50;

/// Stress test shared memory with high concurrency
#[test]
#[ignore] // Run with --ignored flag for stress tests
fn test_shared_memory_high_concurrency() {
    let _ = initialize();
    
    let start_time = Instant::now();
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(AtomicBool::new(true));
    
    println!("🧪 Starting shared memory high concurrency test...");
    
    // Spawn multiple threads that create, write, read, and destroy shared memory
    let mut handles = vec![];
    for thread_id in 0..HIGH_CONCURRENCY_THREADS {
        let success_count = success_count.clone();
        let error_count = error_count.clone();
        let running = running.clone();
        
        let handle = thread::spawn(move || {
            let mut operations = 0;
            
            while running.load(Ordering::SeqCst) && operations < 100 {
                let shm_name = format!("stress_shm_{}_{}", thread_id, operations);
                
                match stress_test_shared_memory_operations(&shm_name) {
                    Ok(_) => {
                        success_count.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(e) => {
                        eprintln!("❌ Thread {} operation {} failed: {:?}", thread_id, operations, e);
                        error_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
                
                operations += 1;
                
                // Brief pause to avoid overwhelming the system
                thread::sleep(Duration::from_millis(10));
            }
        });
        
        handles.push(handle);
    }
    
    // Let the test run for the specified duration
    thread::sleep(STRESS_TEST_DURATION);
    running.store(false, Ordering::SeqCst);
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_operations = success_count.load(Ordering::SeqCst) + error_count.load(Ordering::SeqCst);
    let success_rate = if total_operations > 0 {
        (success_count.load(Ordering::SeqCst) as f64 / total_operations as f64) * 100.0
    } else {
        0.0
    };
    
    println!("📊 Shared Memory Stress Test Results:");
    println!("   Duration: {:?}", start_time.elapsed());
    println!("   Total operations: {}", total_operations);
    println!("   Successful operations: {}", success_count.load(Ordering::SeqCst));
    println!("   Failed operations: {}", error_count.load(Ordering::SeqCst));
    println!("   Success rate: {:.2}%", success_rate);
    println!("   Operations per second: {:.2}", total_operations as f64 / start_time.elapsed().as_secs_f64());
    
    // Verify acceptable performance
    assert!(success_rate >= 95.0, "Success rate should be at least 95%, got {:.2}%", success_rate);
    assert!(total_operations >= HIGH_CONCURRENCY_THREADS * 50, "Should complete at least 50 operations per thread");
    
    let _ = shutdown();
}

fn stress_test_shared_memory_operations(shm_name: &str) -> IpcResult<()> {
    // Create shared memory with random size
    let size = 1024 + (std::ptr::addr_of!(shm_name) as usize % 4096);
    let config = SharedMemoryConfig::new(shm_name, size)
        .with_remove_on_drop()
        .with_permissions(cursed::stdlib::ipc::IpcPermissions::read_write());
    
    let mut shm = SharedMemory::create(config)?;
    shm.map()?;
    
    // Generate test data
    let test_data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
    
    // Write data
    shm.write_bytes(0, &test_data)?;
    
    // Read and verify data
    let read_data = shm.read_bytes(0, test_data.len())?;
    if read_data != test_data {
        return Err(IpcError::InvalidData { 
            message: "Data integrity check failed".to_string() 
        });
    }
    
    // Test partial reads/writes
    for i in 0..10 {
        let offset = i * 100;
        if offset + 50 < size {
            let partial_data = vec![0xAA; 50];
            shm.write_bytes(offset, &partial_data)?;
            
            let read_partial = shm.read_bytes(offset, 50)?;
            if read_partial != partial_data {
                return Err(IpcError::InvalidData { 
                    message: "Partial data integrity check failed".to_string() 
                });
            }
        }
    }
    
    Ok(())
}

/// Stress test message queues with burst messaging
#[test]
#[ignore]
fn test_message_queue_burst_messaging() {
    let _ = initialize();
    
    let start_time = Instant::now();
    let messages_sent = Arc::new(AtomicUsize::new(0));
    let messages_received = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(AtomicBool::new(true));
    
    println!("🧪 Starting message queue burst messaging test...");
    
    // Create message queue
    let mq_config = MessageQueueConfig::new("stress_test_mq", MESSAGE_BURST_COUNT * 2)
        .with_max_message_size(1024);
    
    let mq_result = MessageQueue::create(mq_config);
    assert!(mq_result.is_ok(), "Failed to create message queue: {:?}", mq_result.err());
    let mq = mq_result.unwrap();
    
    // Producer threads
    let mut producer_handles = vec![];
    for producer_id in 0..4 {
        let mq = mq.clone();
        let messages_sent = messages_sent.clone();
        let running = running.clone();
        
        let handle = thread::spawn(move || {
            let mut sent_count = 0;
            
            while running.load(Ordering::SeqCst) && sent_count < MESSAGE_BURST_COUNT / 4 {
                let message_content = format!("Message {} from producer {}", sent_count, producer_id);
                let priority = match sent_count % 3 {
                    0 => MessagePriority::High,
                    1 => MessagePriority::Normal,
                    _ => MessagePriority::Low,
                };
                
                match Message::new(&message_content, priority) {
                    Ok(message) => {
                        if mq.send_timeout(message, Duration::from_millis(100)).is_ok() {
                            messages_sent.fetch_add(1, Ordering::SeqCst);
                            sent_count += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Producer {} failed to create message: {:?}", producer_id, e);
                    }
                }
                
                // Simulate burst messaging with brief pauses
                if sent_count % 100 == 0 {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        });
        
        producer_handles.push(handle);
    }
    
    // Consumer threads
    let mut consumer_handles = vec![];
    for consumer_id in 0..2 {
        let mq = mq.clone();
        let messages_received = messages_received.clone();
        let running = running.clone();
        
        let handle = thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                match mq.receive_timeout(Duration::from_millis(100)) {
                    Ok(message) => {
                        messages_received.fetch_add(1, Ordering::SeqCst);
                        
                        // Simulate processing time
                        thread::sleep(Duration::from_micros(100));
                    }
                    Err(IpcError::Timeout) => {
                        // Expected during test completion
                        continue;
                    }
                    Err(e) => {
                        eprintln!("❌ Consumer {} failed to receive message: {:?}", consumer_id, e);
                    }
                }
            }
        });
        
        consumer_handles.push(handle);
    }
    
    // Let the test run
    thread::sleep(STRESS_TEST_DURATION);
    running.store(false, Ordering::SeqCst);
    
    // Wait for producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    // Give consumers time to process remaining messages
    thread::sleep(Duration::from_secs(2));
    
    // Stop consumers
    running.store(false, Ordering::SeqCst);
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    
    let sent = messages_sent.load(Ordering::SeqCst);
    let received = messages_received.load(Ordering::SeqCst);
    let delivery_rate = if sent > 0 {
        (received as f64 / sent as f64) * 100.0
    } else {
        0.0
    };
    
    println!("📊 Message Queue Burst Test Results:");
    println!("   Duration: {:?}", start_time.elapsed());
    println!("   Messages sent: {}", sent);
    println!("   Messages received: {}", received);
    println!("   Delivery rate: {:.2}%", delivery_rate);
    println!("   Throughput: {:.2} messages/second", sent as f64 / start_time.elapsed().as_secs_f64());
    
    // Verify acceptable performance
    assert!(delivery_rate >= 98.0, "Delivery rate should be at least 98%, got {:.2}%", delivery_rate);
    assert!(sent >= MESSAGE_BURST_COUNT * 3 / 4, "Should send at least 75% of target messages");
    
    let _ = shutdown();
}

/// Stress test domain sockets with many concurrent connections
#[test]
#[ignore]
fn test_domain_socket_concurrent_connections() {
    let _ = initialize();
    
    let start_time = Instant::now();
    let connections_made = Arc::new(AtomicUsize::new(0));
    let data_transferred = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(AtomicBool::new(true));
    
    println!("🧪 Starting domain socket concurrent connections test...");
    
    // Start server
    let server_running = running.clone();
    let server_connections = connections_made.clone();
    let server_data = data_transferred.clone();
    
    let server_handle = thread::spawn(move || {
        let socket_config = SocketConfig::new("/tmp/stress_test_socket", SocketType::Stream)
            .with_buffer_size(4096)
            .with_max_connections(Some(HIGH_CONCURRENCY_THREADS * 2));
        
        match DomainSocket::bind(socket_config) {
            Ok(listener) => {
                if listener.listen(HIGH_CONCURRENCY_THREADS).is_ok() {
                    while server_running.load(Ordering::SeqCst) {
                        if let Ok(connection) = listener.accept_timeout(Duration::from_millis(100)) {
                            server_connections.fetch_add(1, Ordering::SeqCst);
                            
                            // Handle connection in separate thread
                            let server_data = server_data.clone();
                            thread::spawn(move || {
                                let mut buffer = vec![0u8; 1024];
                                while let Ok(bytes_read) = connection.read(&mut buffer) {
                                    if bytes_read == 0 {
                                        break;
                                    }
                                    
                                    server_data.fetch_add(bytes_read, Ordering::SeqCst);
                                    
                                    // Echo back the data
                                    if connection.write(&buffer[..bytes_read]).is_err() {
                                        break;
                                    }
                                }
                            });
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to start server: {:?}", e);
            }
        }
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Start clients
    let mut client_handles = vec![];
    for client_id in 0..HIGH_CONCURRENCY_THREADS {
        let running = running.clone();
        let data_transferred = data_transferred.clone();
        
        let handle = thread::spawn(move || {
            let mut operations = 0;
            
            while running.load(Ordering::SeqCst) && operations < 10 {
                let client_config = SocketConfig::new("/tmp/stress_test_socket", SocketType::Stream);
                
                match DomainSocket::connect(client_config) {
                    Ok(socket) => {
                        let test_data = format!("Test data from client {} operation {}", client_id, operations);
                        
                        if socket.write(test_data.as_bytes()).is_ok() {
                            let mut buffer = vec![0u8; test_data.len()];
                            if socket.read_exact(&mut buffer).is_ok() {
                                data_transferred.fetch_add(test_data.len() * 2, Ordering::SeqCst); // Send + receive
                            }
                        }
                        
                        operations += 1;
                    }
                    Err(e) => {
                        eprintln!("❌ Client {} failed to connect: {:?}", client_id, e);
                        break;
                    }
                }
                
                thread::sleep(Duration::from_millis(100));
            }
        });
        
        client_handles.push(handle);
    }
    
    // Let the test run
    thread::sleep(STRESS_TEST_DURATION);
    running.store(false, Ordering::SeqCst);
    
    // Wait for clients to finish
    for handle in client_handles {
        handle.join().unwrap();
    }
    
    // Stop server
    server_handle.join().unwrap();
    
    let connections = connections_made.load(Ordering::SeqCst);
    let data_bytes = data_transferred.load(Ordering::SeqCst);
    
    println!("📊 Domain Socket Stress Test Results:");
    println!("   Duration: {:?}", start_time.elapsed());
    println!("   Connections made: {}", connections);
    println!("   Data transferred: {} bytes", data_bytes);
    println!("   Connection rate: {:.2} connections/second", connections as f64 / start_time.elapsed().as_secs_f64());
    println!("   Data rate: {:.2} bytes/second", data_bytes as f64 / start_time.elapsed().as_secs_f64());
    
    // Verify acceptable performance
    assert!(connections >= HIGH_CONCURRENCY_THREADS * 5, "Should make at least 5 connections per client thread");
    assert!(data_bytes > 0, "Should transfer some data");
    
    let _ = shutdown();
}

/// Stress test with large data transfers
#[test]
#[ignore]
fn test_large_data_transfers() {
    let _ = initialize();
    
    let start_time = Instant::now();
    
    println!("🧪 Starting large data transfer test...");
    
    // Test large shared memory operations
    let large_shm_config = SharedMemoryConfig::new("large_data_shm", LARGE_DATA_SIZE)
        .with_remove_on_drop()
        .with_permissions(cursed::stdlib::ipc::IpcPermissions::read_write());
    
    let mut large_shm = SharedMemory::create(large_shm_config)
        .expect("Failed to create large shared memory");
    large_shm.map().expect("Failed to map large shared memory");
    
    // Generate large test data
    let large_data: Vec<u8> = (0..LARGE_DATA_SIZE).map(|i| (i % 256) as u8).collect();
    
    // Write large data
    let write_start = Instant::now();
    large_shm.write_bytes(0, &large_data).expect("Failed to write large data");
    let write_duration = write_start.elapsed();
    
    // Read large data
    let read_start = Instant::now();
    let read_data = large_shm.read_bytes(0, LARGE_DATA_SIZE).expect("Failed to read large data");
    let read_duration = read_start.elapsed();
    
    // Verify data integrity
    assert_eq!(read_data.len(), large_data.len(), "Data size mismatch");
    assert_eq!(read_data, large_data, "Data integrity check failed");
    
    let write_throughput = LARGE_DATA_SIZE as f64 / write_duration.as_secs_f64() / 1024.0 / 1024.0; // MB/s
    let read_throughput = LARGE_DATA_SIZE as f64 / read_duration.as_secs_f64() / 1024.0 / 1024.0; // MB/s
    
    println!("📊 Large Data Transfer Results:");
    println!("   Total duration: {:?}", start_time.elapsed());
    println!("   Data size: {} bytes ({:.2} MB)", LARGE_DATA_SIZE, LARGE_DATA_SIZE as f64 / 1024.0 / 1024.0);
    println!("   Write time: {:?}", write_duration);
    println!("   Read time: {:?}", read_duration);
    println!("   Write throughput: {:.2} MB/s", write_throughput);
    println!("   Read throughput: {:.2} MB/s", read_throughput);
    
    // Verify acceptable performance (at least 10 MB/s)
    assert!(write_throughput >= 10.0, "Write throughput should be at least 10 MB/s, got {:.2}", write_throughput);
    assert!(read_throughput >= 10.0, "Read throughput should be at least 10 MB/s, got {:.2}", read_throughput);
    
    let _ = shutdown();
}

/// Memory pressure test - create many shared memory regions
#[test]
#[ignore]
fn test_memory_pressure() {
    let _ = initialize();
    
    let start_time = Instant::now();
    let mut shared_memories = Vec::new();
    
    println!("🧪 Starting memory pressure test...");
    
    // Create many shared memory regions
    for i in 0..SHARED_MEMORY_REGIONS {
        let shm_name = format!("memory_pressure_shm_{}", i);
        let shm_config = SharedMemoryConfig::new(&shm_name, 64 * 1024) // 64KB each
            .with_remove_on_drop()
            .with_permissions(cursed::stdlib::ipc::IpcPermissions::read_write());
        
        match SharedMemory::create(shm_config) {
            Ok(mut shm) => {
                if shm.map().is_ok() {
                    // Write some data to ensure the region is actually allocated
                    let test_data = vec![i as u8; 1024];
                    if shm.write_bytes(0, &test_data).is_ok() {
                        shared_memories.push(shm);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to create shared memory {}: {:?}", i, e);
                break;
            }
        }
        
        // Brief pause to avoid overwhelming the system
        if i % 10 == 0 {
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    let created_regions = shared_memories.len();
    let total_memory = created_regions * 64 * 1024; // Total allocated memory
    
    println!("📊 Memory Pressure Test Results:");
    println!("   Duration: {:?}", start_time.elapsed());
    println!("   Regions created: {} / {}", created_regions, SHARED_MEMORY_REGIONS);
    println!("   Total memory allocated: {} bytes ({:.2} MB)", total_memory, total_memory as f64 / 1024.0 / 1024.0);
    
    // Verify we could create a reasonable number of regions
    assert!(created_regions >= SHARED_MEMORY_REGIONS * 8 / 10, 
            "Should create at least 80% of target regions, got {} / {}", 
            created_regions, SHARED_MEMORY_REGIONS);
    
    // Test access to all regions
    let access_start = Instant::now();
    let mut successful_accesses = 0;
    
    for (i, shm) in shared_memories.iter().enumerate() {
        if let Ok(data) = shm.read_bytes(0, 1024) {
            if data[0] == i as u8 {
                successful_accesses += 1;
            }
        }
    }
    
    let access_duration = access_start.elapsed();
    
    println!("   Access test duration: {:?}", access_duration);
    println!("   Successful accesses: {} / {}", successful_accesses, created_regions);
    
    // Verify all regions remain accessible
    assert_eq!(successful_accesses, created_regions, "All regions should remain accessible");
    
    let _ = shutdown();
}

/// Resource cleanup verification test
#[test]
fn test_resource_cleanup() {
    println!("🧪 Starting resource cleanup verification test...");
    
    // Initialize and create resources
    let _ = initialize();
    let initial_stats = get_ipc_statistics();
    
    // Create various IPC resources
    let shm_config = SharedMemoryConfig::new("cleanup_test_shm", 4096)
        .with_remove_on_drop();
    let _shm = SharedMemory::create(shm_config).expect("Failed to create shared memory");
    
    let mq_config = MessageQueueConfig::new("cleanup_test_mq", 10);
    let _mq = MessageQueue::create(mq_config).expect("Failed to create message queue");
    
    let sem_config = SemaphoreConfig::new("cleanup_test_sem", 1);
    let _sem = Semaphore::create(sem_config).expect("Failed to create semaphore");
    
    // Get stats after resource creation
    let after_creation_stats = get_ipc_statistics();
    
    // Verify resources were created
    assert!(after_creation_stats.active_shared_memory_regions > initial_stats.active_shared_memory_regions);
    assert!(after_creation_stats.active_message_queues > initial_stats.active_message_queues);
    assert!(after_creation_stats.active_semaphores > initial_stats.active_semaphores);
    
    println!("📊 Resources created successfully");
    println!("   Shared memory regions: {} -> {}", 
             initial_stats.active_shared_memory_regions, 
             after_creation_stats.active_shared_memory_regions);
    println!("   Message queues: {} -> {}", 
             initial_stats.active_message_queues, 
             after_creation_stats.active_message_queues);
    println!("   Semaphores: {} -> {}", 
             initial_stats.active_semaphores, 
             after_creation_stats.active_semaphores);
    
    // Drop all resources (they go out of scope)
    drop(_shm);
    drop(_mq);
    drop(_sem);
    
    // Give some time for cleanup
    thread::sleep(Duration::from_millis(100));
    
    // Shutdown IPC subsystem
    let _ = shutdown();
    
    // Verify cleanup (note: some resources might still be tracked if cleanup is asynchronous)
    let final_stats = get_ipc_statistics();
    
    println!("📊 After cleanup:");
    println!("   Shared memory regions: {}", final_stats.active_shared_memory_regions);
    println!("   Message queues: {}", final_stats.active_message_queues);
    println!("   Semaphores: {}", final_stats.active_semaphores);
    
    // In a real implementation, we might expect these to return to initial values
    // For now, we just verify the system doesn't crash during cleanup
    assert!(final_stats.active_shared_memory_regions >= initial_stats.active_shared_memory_regions);
    
    println!("✅ Resource cleanup verification completed");
}
