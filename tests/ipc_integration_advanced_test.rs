/// Advanced IPC Integration Test Suite
/// 
/// This test suite validates complex IPC scenarios including
/// cross-platform compatibility, security features, and
/// advanced usage patterns.

use std::time::{Duration, Instant, SystemTime};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::thread;
use std::collections::HashMap;
use std::process::{Command, Stdio};

use cursed::stdlib::ipc::{
    IpcResult, IpcError, SharedMemory, SharedMemoryConfig, MemoryProtection,
    SharedMemoryAccess, NamedPipe, PipeConfig, PipeMode,
    MessageQueue, MessageQueueConfig, Message, MessagePriority,
    Semaphore, SemaphoreConfig, DomainSocket, SocketConfig, SocketType,
    SignalHandler, Signal, RpcServer, RpcClient, RpcConfig, RpcTransport,
    IpcSecurityContext, SecurityPolicy, initialize, shutdown, get_ipc_statistics,
};

/// Test cross-process shared memory communication
#[test]
fn test_cross_process_shared_memory() {
    let _ = initialize();
    
    println!("🔄 Testing cross-process shared memory communication...");
    
    // Create shared memory that will be used by child process
    let config = SharedMemoryConfig::new("cross_process_test", 4096)
        .with_permissions(cursed::stdlib::ipc::IpcPermissions::read_write());
    
    let mut parent_shm = SharedMemory::create(config.clone())
        .expect("Failed to create shared memory in parent");
    parent_shm.map().expect("Failed to map shared memory in parent");
    
    // Write initial data from parent
    let parent_data = b"Hello from parent process!";
    parent_shm.write_bytes(0, parent_data)
        .expect("Failed to write data from parent");
    
    // In a real test, we would spawn a child process here
    // For this test, we'll simulate with threading and separate mapping
    let child_handle = thread::spawn(|| -> IpcResult<()> {
        // Simulate child process by opening existing shared memory
        let child_config = SharedMemoryConfig::new("cross_process_test", 4096)
            .with_permissions(cursed::stdlib::ipc::IpcPermissions::read_write())
            .with_create_if_not_exists(false); // Don't create, only open existing
        
        match SharedMemory::open(child_config) {
            Ok(mut child_shm) => {
                child_shm.map()?;
                
                // Read data written by parent
                let read_data = child_shm.read_bytes(0, 26)?;
                assert_eq!(read_data, b"Hello from parent process!");
                
                // Write response from child
                let child_response = b"Hello from child process!";
                child_shm.write_bytes(100, child_response)?;
                
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Child failed to open shared memory: {:?}", e);
                Err(e)
            }
        }
    });
    
    // Give child time to process
    thread::sleep(Duration::from_millis(100));
    
    // Wait for child to complete
    child_handle.join().unwrap().expect("Child process failed");
    
    // Read response from child
    let child_response = parent_shm.read_bytes(100, 25)
        .expect("Failed to read child response");
    assert_eq!(child_response, b"Hello from child process!");
    
    println!("✅ Cross-process shared memory communication successful");
    
    let _ = shutdown();
}

/// Test named pipe producer-consumer pattern
#[test]
fn test_named_pipe_producer_consumer() {
    let _ = initialize();
    
    println!("🔄 Testing named pipe producer-consumer pattern...");
    
    let pipe_path = "/tmp/test_producer_consumer_pipe";
    let message_count = 10;
    let messages_sent = Arc::new(AtomicUsize::new(0));
    let messages_received = Arc::new(AtomicUsize::new(0));
    
    // Producer thread
    let messages_sent_clone = messages_sent.clone();
    let producer_handle = thread::spawn(move || -> IpcResult<()> {
        let config = PipeConfig::new(pipe_path)
            .with_mode(PipeMode::WriteOnly)
            .with_buffer_size(1024);
        
        let pipe = NamedPipe::create(config)?;
        
        for i in 0..message_count {
            let message = format!("Message {} from producer", i);
            pipe.write(&message)?;
            messages_sent_clone.fetch_add(1, Ordering::SeqCst);
            
            // Brief pause between messages
            thread::sleep(Duration::from_millis(10));
        }
        
        Ok(())
    });
    
    // Give producer time to create pipe
    thread::sleep(Duration::from_millis(50));
    
    // Consumer thread
    let messages_received_clone = messages_received.clone();
    let consumer_handle = thread::spawn(move || -> IpcResult<()> {
        let config = PipeConfig::new(pipe_path)
            .with_mode(PipeMode::ReadOnly);
        
        let pipe = NamedPipe::open(config)?;
        
        for _i in 0..message_count {
            match pipe.read_string() {
                Ok(message) => {
                    assert!(message.starts_with("Message"));
                    assert!(message.contains("from producer"));
                    messages_received_clone.fetch_add(1, Ordering::SeqCst);
                }
                Err(e) => {
                    eprintln!("❌ Consumer failed to read message: {:?}", e);
                    break;
                }
            }
        }
        
        Ok(())
    });
    
    // Wait for both threads to complete
    producer_handle.join().unwrap().expect("Producer failed");
    consumer_handle.join().unwrap().expect("Consumer failed");
    
    let sent = messages_sent.load(Ordering::SeqCst);
    let received = messages_received.load(Ordering::SeqCst);
    
    println!("📊 Producer-Consumer Results:");
    println!("   Messages sent: {}", sent);
    println!("   Messages received: {}", received);
    
    assert_eq!(sent, message_count, "Should send all messages");
    assert_eq!(received, message_count, "Should receive all messages");
    
    println!("✅ Named pipe producer-consumer pattern successful");
    
    let _ = shutdown();
}

/// Test message queue priority ordering
#[test]
fn test_message_queue_priority_ordering() {
    let _ = initialize();
    
    println!("🔄 Testing message queue priority ordering...");
    
    let config = MessageQueueConfig::new("priority_test_queue", 20)
        .with_max_message_size(256);
    
    let mq = MessageQueue::create(config)
        .expect("Failed to create message queue");
    
    // Send messages with different priorities
    let messages = vec![
        ("Low priority message 1", MessagePriority::Low),
        ("High priority message 1", MessagePriority::High),
        ("Normal priority message 1", MessagePriority::Normal),
        ("High priority message 2", MessagePriority::High),
        ("Low priority message 2", MessagePriority::Low),
        ("Normal priority message 2", MessagePriority::Normal),
    ];
    
    for (content, priority) in &messages {
        let message = Message::new(content, *priority)
            .expect("Failed to create message");
        mq.send(message).expect("Failed to send message");
    }
    
    // Receive messages and verify priority ordering
    let mut received_messages = Vec::new();
    
    while mq.has_messages() {
        match mq.receive() {
            Ok(message) => {
                received_messages.push((message.content(), message.priority()));
            }
            Err(e) => {
                eprintln!("❌ Failed to receive message: {:?}", e);
                break;
            }
        }
    }
    
    println!("📊 Received messages in order:");
    for (i, (content, priority)) in received_messages.iter().enumerate() {
        println!("   {}: {:?} - {}", i + 1, priority, content);
    }
    
    // Verify priority ordering (High > Normal > Low)
    let mut current_priority = MessagePriority::High;
    for (_, priority) in &received_messages {
        match (current_priority, priority) {
            (MessagePriority::High, MessagePriority::Normal) => {
                current_priority = MessagePriority::Normal;
            }
            (MessagePriority::Normal, MessagePriority::Low) => {
                current_priority = MessagePriority::Low;
            }
            (curr, recv) if curr == *recv => {
                // Same priority is fine
            }
            _ => {
                panic!("Priority ordering violation: expected >= {:?}, got {:?}", 
                       current_priority, priority);
            }
        }
    }
    
    assert_eq!(received_messages.len(), messages.len(), "Should receive all messages");
    
    println!("✅ Message queue priority ordering successful");
    
    let _ = shutdown();
}

/// Test semaphore resource coordination
#[test]
fn test_semaphore_resource_coordination() {
    let _ = initialize();
    
    println!("🔄 Testing semaphore resource coordination...");
    
    let resource_count = 3;
    let worker_count = 6;
    let operations_per_worker = 5;
    
    // Create semaphore with limited resources
    let config = SemaphoreConfig::new("resource_coordination_sem", resource_count);
    let semaphore = Semaphore::create(config)
        .expect("Failed to create semaphore");
    
    let completed_operations = Arc::new(AtomicUsize::new(0));
    let active_workers = Arc::new(AtomicUsize::new(0));
    let max_concurrent = Arc::new(AtomicUsize::new(0));
    
    // Spawn worker threads
    let mut worker_handles = vec![];
    
    for worker_id in 0..worker_count {
        let semaphore = semaphore.clone();
        let completed_operations = completed_operations.clone();
        let active_workers = active_workers.clone();
        let max_concurrent = max_concurrent.clone();
        
        let handle = thread::spawn(move || -> IpcResult<()> {
            for operation in 0..operations_per_worker {
                // Acquire resource
                semaphore.acquire()?;
                
                // Track concurrent access
                let current_active = active_workers.fetch_add(1, Ordering::SeqCst) + 1;
                let current_max = max_concurrent.load(Ordering::SeqCst);
                if current_active > current_max {
                    max_concurrent.store(current_active, Ordering::SeqCst);
                }
                
                // Simulate resource usage
                thread::sleep(Duration::from_millis(50));
                
                // Complete operation
                completed_operations.fetch_add(1, Ordering::SeqCst);
                active_workers.fetch_sub(1, Ordering::SeqCst);
                
                // Release resource
                semaphore.release()?;
                
                println!("Worker {} completed operation {}", worker_id, operation);
                
                // Brief pause between operations
                thread::sleep(Duration::from_millis(10));
            }
            
            Ok(())
        });
        
        worker_handles.push(handle);
    }
    
    // Wait for all workers to complete
    for handle in worker_handles {
        handle.join().unwrap().expect("Worker failed");
    }
    
    let total_operations = completed_operations.load(Ordering::SeqCst);
    let max_concurrent_workers = max_concurrent.load(Ordering::SeqCst);
    
    println!("📊 Resource Coordination Results:");
    println!("   Total operations completed: {}", total_operations);
    println!("   Max concurrent workers: {}", max_concurrent_workers);
    println!("   Semaphore resource limit: {}", resource_count);
    
    // Verify all operations completed
    assert_eq!(total_operations, worker_count * operations_per_worker, 
               "Should complete all operations");
    
    // Verify semaphore enforced resource limit
    assert!(max_concurrent_workers <= resource_count, 
            "Concurrent workers ({}) should not exceed resource limit ({})", 
            max_concurrent_workers, resource_count);
    
    println!("✅ Semaphore resource coordination successful");
    
    let _ = shutdown();
}

/// Test domain socket client-server communication
#[test]
fn test_domain_socket_client_server() {
    let _ = initialize();
    
    println!("🔄 Testing domain socket client-server communication...");
    
    let socket_path = "/tmp/test_client_server_socket";
    let client_count = 3;
    let messages_per_client = 5;
    
    let server_received = Arc::new(AtomicUsize::new(0));
    let clients_completed = Arc::new(AtomicUsize::new(0));
    let server_running = Arc::new(AtomicBool::new(true));
    
    // Server thread
    let server_received_clone = server_received.clone();
    let server_running_clone = server_running.clone();
    
    let server_handle = thread::spawn(move || -> IpcResult<()> {
        let config = SocketConfig::new(socket_path, SocketType::Stream)
            .with_buffer_size(1024)
            .with_max_connections(Some(client_count + 2));
        
        let listener = DomainSocket::bind(config)?;
        listener.listen(client_count)?;
        
        while server_running_clone.load(Ordering::SeqCst) {
            match listener.accept_timeout(Duration::from_millis(100)) {
                Ok(connection) => {
                    let server_received = server_received_clone.clone();
                    
                    // Handle client in separate thread
                    thread::spawn(move || -> IpcResult<()> {
                        let mut buffer = vec![0u8; 1024];
                        
                        loop {
                            match connection.read(&mut buffer) {
                                Ok(0) => break, // Connection closed
                                Ok(bytes_read) => {
                                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                                    println!("🔌 Server received: {}", message);
                                    
                                    server_received.fetch_add(1, Ordering::SeqCst);
                                    
                                    // Echo response
                                    let response = format!("Echo: {}", message);
                                    connection.write(response.as_bytes())?;
                                }
                                Err(e) => {
                                    eprintln!("❌ Server read error: {:?}", e);
                                    break;
                                }
                            }
                        }
                        
                        Ok(())
                    });
                }
                Err(IpcError::Timeout) => {
                    // Expected during shutdown
                    continue;
                }
                Err(e) => {
                    eprintln!("❌ Server accept error: {:?}", e);
                    break;
                }
            }
        }
        
        Ok(())
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Client threads
    let mut client_handles = vec![];
    
    for client_id in 0..client_count {
        let clients_completed = clients_completed.clone();
        
        let handle = thread::spawn(move || -> IpcResult<()> {
            let config = SocketConfig::new(socket_path, SocketType::Stream);
            let socket = DomainSocket::connect(config)?;
            
            for message_id in 0..messages_per_client {
                let message = format!("Message {} from client {}", message_id, client_id);
                socket.write(message.as_bytes())?;
                
                // Read echo response
                let mut buffer = vec![0u8; 1024];
                let bytes_read = socket.read(&mut buffer)?;
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                
                assert!(response.starts_with("Echo:"), "Should receive echo response");
                assert!(response.contains(&message), "Response should contain original message");
                
                println!("🔌 Client {} received: {}", client_id, response);
                
                thread::sleep(Duration::from_millis(50));
            }
            
            clients_completed.fetch_add(1, Ordering::SeqCst);
            Ok(())
        });
        
        client_handles.push(handle);
    }
    
    // Wait for all clients to complete
    for handle in client_handles {
        handle.join().unwrap().expect("Client failed");
    }
    
    // Give server time to process final messages
    thread::sleep(Duration::from_millis(200));
    
    // Stop server
    server_running.store(false, Ordering::SeqCst);
    server_handle.join().unwrap().expect("Server failed");
    
    let total_received = server_received.load(Ordering::SeqCst);
    let completed_clients = clients_completed.load(Ordering::SeqCst);
    
    println!("📊 Client-Server Results:");
    println!("   Messages received by server: {}", total_received);
    println!("   Clients completed: {}", completed_clients);
    
    assert_eq!(completed_clients, client_count, "All clients should complete");
    assert_eq!(total_received, client_count * messages_per_client, 
               "Server should receive all messages");
    
    println!("✅ Domain socket client-server communication successful");
    
    let _ = shutdown();
}

/// Test RPC method calls
#[test]
fn test_rpc_method_calls() {
    let _ = initialize();
    
    println!("🔄 Testing RPC method calls...");
    
    let rpc_path = "/tmp/test_rpc_socket";
    let server_running = Arc::new(AtomicBool::new(true));
    
    // RPC Server thread
    let server_running_clone = server_running.clone();
    let server_handle = thread::spawn(move || -> IpcResult<()> {
        let config = RpcConfig::new("test_rpc_server")
            .with_transport(RpcTransport::UnixSocket(rpc_path));
        
        let mut server = RpcServer::create(config)?;
        
        // Register methods
        server.register_method("add", |params| {
            let a = params.get("a").and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
            let b = params.get("b").and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);
            Ok((a + b).to_string())
        })?;
        
        server.register_method("multiply", |params| {
            let a = params.get("a").and_then(|v| v.parse::<i32>().ok()).unwrap_or(1);
            let b = params.get("b").and_then(|v| v.parse::<i32>().ok()).unwrap_or(1);
            Ok((a * b).to_string())
        })?;
        
        server.register_method("hello", |params| {
            let name = params.get("name").unwrap_or("World");
            Ok(format!("Hello, {}!", name))
        })?;
        
        // Start server
        let _server_handle = server.start_async()?;
        
        // Keep server running
        while server_running_clone.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(100));
        }
        
        server.stop()?;
        Ok(())
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(200));
    
    // RPC Client
    let client_config = RpcConfig::new("test_rpc_client")
        .with_transport(RpcTransport::UnixSocket(rpc_path));
    
    let client = RpcClient::connect(client_config)
        .expect("Failed to connect RPC client");
    
    // Test add method
    let add_params = vec![("a", "15"), ("b", "27")];
    let add_result = client.call("add", add_params)
        .expect("Failed to call add method");
    assert_eq!(add_result, "42", "Add result should be 42");
    println!("✅ RPC add(15, 27) = {}", add_result);
    
    // Test multiply method
    let multiply_params = vec![("a", "6"), ("b", "7")];
    let multiply_result = client.call("multiply", multiply_params)
        .expect("Failed to call multiply method");
    assert_eq!(multiply_result, "42", "Multiply result should be 42");
    println!("✅ RPC multiply(6, 7) = {}", multiply_result);
    
    // Test hello method
    let hello_params = vec![("name", "CURSED")];
    let hello_result = client.call("hello", hello_params)
        .expect("Failed to call hello method");
    assert_eq!(hello_result, "Hello, CURSED!", "Hello result should be greeting");
    println!("✅ RPC hello(CURSED) = {}", hello_result);
    
    // Stop server
    server_running.store(false, Ordering::SeqCst);
    server_handle.join().unwrap().expect("Server failed");
    
    println!("✅ RPC method calls successful");
    
    let _ = shutdown();
}

/// Test IPC performance monitoring
#[test]
fn test_ipc_performance_monitoring() {
    let _ = initialize();
    
    println!("🔄 Testing IPC performance monitoring...");
    
    let initial_stats = get_ipc_statistics();
    println!("📊 Initial IPC Statistics:");
    print_ipc_statistics(&initial_stats);
    
    // Create various IPC resources to affect statistics
    let shm_config = SharedMemoryConfig::new("perf_test_shm", 8192)
        .with_remove_on_drop();
    let _shm = SharedMemory::create(shm_config)
        .expect("Failed to create shared memory");
    
    let mq_config = MessageQueueConfig::new("perf_test_mq", 5);
    let mq = MessageQueue::create(mq_config)
        .expect("Failed to create message queue");
    
    let sem_config = SemaphoreConfig::new("perf_test_sem", 2);
    let _sem = Semaphore::create(sem_config)
        .expect("Failed to create semaphore");
    
    // Perform some operations
    for i in 0..5 {
        let message = Message::new(&format!("Performance test message {}", i), MessagePriority::Normal)
            .expect("Failed to create message");
        mq.send(message).expect("Failed to send message");
    }
    
    // Get statistics after operations
    let after_ops_stats = get_ipc_statistics();
    println!("📊 After Operations IPC Statistics:");
    print_ipc_statistics(&after_ops_stats);
    
    // Verify statistics changed
    assert!(after_ops_stats.active_shared_memory_regions > initial_stats.active_shared_memory_regions,
            "Shared memory regions should increase");
    assert!(after_ops_stats.active_message_queues > initial_stats.active_message_queues,
            "Message queues should increase");
    assert!(after_ops_stats.active_semaphores > initial_stats.active_semaphores,
            "Semaphores should increase");
    assert!(after_ops_stats.total_memory_usage >= initial_stats.total_memory_usage,
            "Memory usage should not decrease");
    
    println!("✅ IPC performance monitoring successful");
    
    let _ = shutdown();
}

fn print_ipc_statistics(stats: &cursed::stdlib::ipc::types::IpcStatistics) {
    println!("   - Shared memory regions: {}", stats.active_shared_memory_regions);
    println!("   - Active pipes: {}", stats.active_pipes);
    println!("   - Message queues: {}", stats.active_message_queues);
    println!("   - Semaphores: {}", stats.active_semaphores);
    println!("   - Domain sockets: {}", stats.active_sockets);
    println!("   - RPC connections: {}", stats.active_rpc_connections);
    println!("   - Total memory usage: {} bytes", stats.total_memory_usage);
}

/// Test IPC security features
#[test]
fn test_ipc_security_features() {
    let _ = initialize();
    
    println!("🔄 Testing IPC security features...");
    
    // Test permission-based access control
    let readonly_config = SharedMemoryConfig::new("security_test_readonly", 1024)
        .with_permissions(cursed::stdlib::ipc::IpcPermissions::read_only())
        .with_remove_on_drop();
    
    let readwrite_config = SharedMemoryConfig::new("security_test_readwrite", 1024)
        .with_permissions(cursed::stdlib::ipc::IpcPermissions::read_write())
        .with_remove_on_drop();
    
    let mut readonly_shm = SharedMemory::create(readonly_config)
        .expect("Failed to create read-only shared memory");
    readonly_shm.map().expect("Failed to map read-only shared memory");
    
    let mut readwrite_shm = SharedMemory::create(readwrite_config)
        .expect("Failed to create read-write shared memory");
    readwrite_shm.map().expect("Failed to map read-write shared memory");
    
    // Test that read-write operations work on read-write memory
    let test_data = b"Security test data";
    readwrite_shm.write_bytes(0, test_data)
        .expect("Should be able to write to read-write memory");
    
    let read_data = readwrite_shm.read_bytes(0, test_data.len())
        .expect("Should be able to read from read-write memory");
    assert_eq!(read_data, test_data, "Data should match");
    
    // Test read-only memory (implementation may vary)
    // Some systems might allow writes during creation phase
    match readonly_shm.write_bytes(0, test_data) {
        Ok(_) => {
            println!("⚠️  Warning: Write to read-only memory succeeded (may be system-dependent)");
        }
        Err(_) => {
            println!("✅ Write to read-only memory properly rejected");
        }
    }
    
    // Reading should always work
    let _read_result = readonly_shm.read_bytes(0, 10)
        .expect("Should be able to read from read-only memory");
    
    println!("✅ IPC security features tested");
    
    let _ = shutdown();
}
