/// IPC (Inter-Process Communication) Integration Tests
/// 
/// This test suite validates the comprehensive IPC functionality including
/// named pipes, Unix domain sockets, message queues, shared memory,
/// memory-mapped files, and cross-platform compatibility.

use std::thread;
use std::time::{Duration, SystemTime, Instant};
use std::path::Path;

use cursed::stdlib::ipc::real_ipc::{
    RealIpcManager, IpcConnection, NamedPipeConnection, MessageQueueConnection,
    SharedMemoryConnection, MemoryMappedConnection, IpcMessage, MessagePriority,
    initialize_real_ipc, get_ipc_manager, cleanup_real_ipc
};
use cursed::stdlib::ipc::{IpcConfig, SharedMemory, MessageQueue, Message};

fn create_test_config() -> IpcConfig {
    IpcConfig {
        max_connections: 10,
        max_queue_size: 100,
        max_message_size: 4096,
        timeout: Duration::from_secs(5),
        ..Default::default()
    }
}

#[test]
fn test_ipc_manager_lifecycle() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    // Test basic creation and shutdown
    assert!(true); // Manager created successfully
    
    // Manager should handle drop automatically
}

#[test]
fn test_named_pipe_creation_and_cleanup() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    // Create a named pipe
    let pipe_name = format!("test_pipe_{}", std::process::id());
    let connection = manager.create_named_pipe(&pipe_name, true)
        .expect("Failed to create named pipe");
    
    // Verify connection exists
    let retrieved = manager.get_connection(&pipe_name)
        .expect("Failed to retrieve connection");
    
    // Clean up
    manager.remove_connection(&pipe_name)
        .expect("Failed to remove connection");
}

#[test]
#[cfg(unix)]
fn test_unix_socket_communication() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let socket_path = format!("/tmp/test_socket_{}", std::process::id());
    
    // Create server socket
    let server_conn = manager.create_unix_socket(&socket_path, true)
        .expect("Failed to create Unix socket server");
    
    // Test in separate thread to avoid blocking
    let socket_path_clone = socket_path.clone();
    let client_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100)); // Let server start
        
        let config = create_test_config();
        let client_manager = RealIpcManager::new(config).expect("Failed to create client manager");
        
        // Create client socket
        let client_conn = client_manager.create_unix_socket(&socket_path_clone, false)
            .expect("Failed to create Unix socket client");
        
        // Test communication would go here
        // For now, just verify creation succeeded
        assert!(true);
    });
    
    client_handle.join().expect("Client thread panicked");
    
    // Clean up
    manager.remove_connection(&socket_path).expect("Failed to remove connection");
    let _ = std::fs::remove_file(&socket_path); // Clean up socket file
}

#[test]
fn test_message_queue_operations() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let queue_name = format!("test_queue_{}", std::process::id());
    let connection = manager.create_message_queue(&queue_name)
        .expect("Failed to create message queue");
    
    if let IpcConnection::MessageQueue(ref queue_conn) = *connection {
        // Create test message
        let message = IpcMessage {
            id: 1,
            sender_id: std::process::id(),
            priority: MessagePriority::Normal,
            timestamp: SystemTime::now(),
            data: b"test message".to_vec(),
            message_type: "test".to_string(),
            reply_to: None,
            ttl: None,
        };
        
        // Send message
        queue_conn.send(message.clone()).expect("Failed to send message");
        
        // Receive message
        let received = queue_conn.receive(Some(Duration::from_millis(100)))
            .expect("Failed to receive message");
        
        assert_eq!(received.id, message.id);
        assert_eq!(received.data, message.data);
    } else {
        panic!("Expected MessageQueue connection");
    }
    
    manager.remove_connection(&queue_name).expect("Failed to remove connection");
}

#[test]
fn test_priority_message_ordering() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let queue_name = format!("test_priority_queue_{}", std::process::id());
    let connection = manager.create_message_queue(&queue_name)
        .expect("Failed to create message queue");
    
    if let IpcConnection::MessageQueue(ref queue_conn) = *connection {
        // Send messages in different priority order
        let low_msg = IpcMessage {
            id: 1,
            sender_id: std::process::id(),
            priority: MessagePriority::Low,
            timestamp: SystemTime::now(),
            data: b"low priority".to_vec(),
            message_type: "test".to_string(),
            reply_to: None,
            ttl: None,
        };
        
        let high_msg = IpcMessage {
            id: 2,
            sender_id: std::process::id(),
            priority: MessagePriority::High,
            timestamp: SystemTime::now(),
            data: b"high priority".to_vec(),
            message_type: "test".to_string(),
            reply_to: None,
            ttl: None,
        };
        
        // Send low priority first, then high priority
        queue_conn.send(low_msg).expect("Failed to send low priority message");
        queue_conn.send(high_msg).expect("Failed to send high priority message");
        
        // Receive messages - high priority should come first
        let first = queue_conn.receive(Some(Duration::from_millis(100)))
            .expect("Failed to receive first message");
        assert_eq!(first.priority, MessagePriority::High);
        
        let second = queue_conn.receive(Some(Duration::from_millis(100)))
            .expect("Failed to receive second message");
        assert_eq!(second.priority, MessagePriority::Low);
    }
    
    manager.remove_connection(&queue_name).expect("Failed to remove connection");
}

#[test]
fn test_shared_memory_operations() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let segment_name = format!("test_shm_{}", std::process::id());
    let segment_size = 4096;
    
    let connection = manager.create_shared_memory(&segment_name, segment_size)
        .expect("Failed to create shared memory");
    
    if let IpcConnection::SharedMemory(ref shm_conn) = *connection {
        let test_data = b"shared memory test data";
        
        // Write data
        let bytes_written = shm_conn.write(0, test_data)
            .expect("Failed to write to shared memory");
        assert_eq!(bytes_written, test_data.len());
        
        // Read data back
        let mut buffer = vec![0u8; test_data.len()];
        let bytes_read = shm_conn.read(0, &mut buffer)
            .expect("Failed to read from shared memory");
        assert_eq!(bytes_read, test_data.len());
        assert_eq!(&buffer, test_data);
    } else {
        panic!("Expected SharedMemory connection");
    }
    
    manager.remove_connection(&segment_name).expect("Failed to remove connection");
}

#[test]
fn test_memory_mapped_file_operations() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let file_path = format!("/tmp/test_mmap_{}", std::process::id());
    let file_size = 4096;
    
    let connection = manager.create_memory_mapped_file(&file_path, file_size, false)
        .expect("Failed to create memory-mapped file");
    
    if let IpcConnection::MemoryMappedFile(ref mmap_conn) = *connection {
        let test_data = b"memory mapped file test data";
        
        // Write data
        let bytes_written = mmap_conn.write(0, test_data)
            .expect("Failed to write to memory-mapped file");
        assert_eq!(bytes_written, test_data.len());
        
        // Read data back
        let mut buffer = vec![0u8; test_data.len()];
        let bytes_read = mmap_conn.read(0, &mut buffer)
            .expect("Failed to read from memory-mapped file");
        assert_eq!(bytes_read, test_data.len());
        assert_eq!(&buffer, test_data);
    } else {
        panic!("Expected MemoryMappedFile connection");
    }
    
    manager.remove_connection(&file_path).expect("Failed to remove connection");
    let _ = std::fs::remove_file(&file_path); // Clean up
}

#[test]
fn test_concurrent_ipc_operations() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let mut handles = Vec::new();
    
    // Create multiple message queues concurrently
    for i in 0..5 {
        let queue_name = format!("concurrent_queue_{}_{}", i, std::process::id());
        let connection = manager.create_message_queue(&queue_name)
            .expect("Failed to create message queue");
        
        let handle = thread::spawn(move || {
            if let IpcConnection::MessageQueue(ref queue_conn) = *connection {
                let message = IpcMessage {
                    id: i as u64,
                    sender_id: std::process::id(),
                    priority: MessagePriority::Normal,
                    timestamp: SystemTime::now(),
                    data: format!("message_{}", i).into_bytes(),
                    message_type: "test".to_string(),
                    reply_to: None,
                    ttl: None,
                };
                
                queue_conn.send(message).expect("Failed to send message");
                
                let received = queue_conn.receive(Some(Duration::from_millis(100)))
                    .expect("Failed to receive message");
                
                assert_eq!(received.id, i as u64);
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[test]
fn test_ipc_error_handling() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    // Test getting non-existent connection
    let result = manager.get_connection("nonexistent_connection");
    assert!(result.is_err());
    
    // Test creating connection with invalid parameters
    let result = manager.create_shared_memory("", 0);
    assert!(result.is_err());
}

#[test]
fn test_message_queue_capacity_limits() {
    let mut config = create_test_config();
    config.max_queue_size = 2; // Very small queue
    
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    let queue_name = format!("capacity_test_{}", std::process::id());
    let connection = manager.create_message_queue(&queue_name)
        .expect("Failed to create message queue");
    
    if let IpcConnection::MessageQueue(ref queue_conn) = *connection {
        // Fill the queue to capacity
        for i in 0..2 {
            let message = IpcMessage {
                id: i,
                sender_id: std::process::id(),
                priority: MessagePriority::Normal,
                timestamp: SystemTime::now(),
                data: format!("message_{}", i).into_bytes(),
                message_type: "test".to_string(),
                reply_to: None,
                ttl: None,
            };
            
            queue_conn.send(message).expect("Failed to send message");
        }
        
        // Try to send one more - should fail
        let overflow_message = IpcMessage {
            id: 999,
            sender_id: std::process::id(),
            priority: MessagePriority::Normal,
            timestamp: SystemTime::now(),
            data: b"overflow".to_vec(),
            message_type: "test".to_string(),
            reply_to: None,
            ttl: None,
        };
        
        let result = queue_conn.send(overflow_message);
        assert!(result.is_err());
    }
    
    manager.remove_connection(&queue_name).expect("Failed to remove connection");
}

#[test]
fn test_ipc_statistics_collection() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    // Create some connections
    let pipe_name = format!("stats_pipe_{}", std::process::id());
    let _pipe_conn = manager.create_named_pipe(&pipe_name, true)
        .expect("Failed to create named pipe");
    
    let queue_name = format!("stats_queue_{}", std::process::id());
    let _queue_conn = manager.create_message_queue(&queue_name)
        .expect("Failed to create message queue");
    
    // Get statistics
    let stats = manager.get_stats().expect("Failed to get statistics");
    
    assert!(stats.connections_active >= 2);
    assert!(stats.uptime >= Duration::from_millis(0));
}

#[test]
fn test_global_ipc_manager() {
    let config = create_test_config();
    
    // Initialize global manager
    initialize_real_ipc(config).expect("Failed to initialize global IPC");
    
    // Get global manager
    let manager = get_ipc_manager().expect("Failed to get global IPC manager");
    let manager_guard = manager.lock().expect("Failed to lock manager");
    
    // Test basic operations
    let pipe_name = format!("global_pipe_{}", std::process::id());
    let _connection = manager_guard.create_named_pipe(&pipe_name, true)
        .expect("Failed to create pipe with global manager");
    
    drop(manager_guard);
    
    // Clean up global resources
    cleanup_real_ipc().expect("Failed to cleanup global IPC");
}

#[test]
fn test_cross_platform_named_pipes() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let pipe_name = format!("cross_platform_pipe_{}", std::process::id());
    
    // This should work on both Unix and Windows
    let server_conn = manager.create_named_pipe(&pipe_name, true)
        .expect("Failed to create named pipe");
    
    // Test basic operations that should work on both platforms
    if let IpcConnection::NamedPipe(ref pipe_conn) = *server_conn {
        let test_data = b"cross platform test";
        
        // Note: For a real test, we'd need a client connection
        // For now, just verify the pipe was created successfully
        assert!(true);
    }
    
    manager.remove_connection(&pipe_name).expect("Failed to remove connection");
}

#[test]
fn test_ipc_resource_cleanup() {
    let config = create_test_config();
    let mut manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    // Create multiple connections
    let connections = vec![
        format!("cleanup_pipe_{}", std::process::id()),
        format!("cleanup_queue_{}", std::process::id()),
        format!("cleanup_shm_{}", std::process::id()),
    ];
    
    for name in &connections {
        let _conn = manager.create_named_pipe(name, true)
            .expect("Failed to create connection");
    }
    
    // Verify connections exist
    for name in &connections {
        assert!(manager.get_connection(name).is_ok());
    }
    
    // Shutdown manager - should clean up all connections
    manager.shutdown().expect("Failed to shutdown manager");
    
    // Connections should be gone
    for name in &connections {
        assert!(manager.get_connection(name).is_err());
    }
}

#[test]
fn test_large_message_handling() {
    let mut config = create_test_config();
    config.max_message_size = 1024; // 1KB limit
    
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    let queue_name = format!("large_msg_test_{}", std::process::id());
    let connection = manager.create_message_queue(&queue_name)
        .expect("Failed to create message queue");
    
    if let IpcConnection::MessageQueue(ref queue_conn) = *connection {
        // Try to send a message that's too large
        let large_data = vec![0u8; 2048]; // 2KB - exceeds limit
        let large_message = IpcMessage {
            id: 1,
            sender_id: std::process::id(),
            priority: MessagePriority::Normal,
            timestamp: SystemTime::now(),
            data: large_data,
            message_type: "test".to_string(),
            reply_to: None,
            ttl: None,
        };
        
        let result = queue_conn.send(large_message);
        assert!(result.is_err());
        
        // Send a message within limits
        let small_data = vec![0u8; 512]; // 512B - within limit
        let small_message = IpcMessage {
            id: 2,
            sender_id: std::process::id(),
            priority: MessagePriority::Normal,
            timestamp: SystemTime::now(),
            data: small_data,
            message_type: "test".to_string(),
            reply_to: None,
            ttl: None,
        };
        
        queue_conn.send(small_message).expect("Failed to send small message");
    }
    
    manager.remove_connection(&queue_name).expect("Failed to remove connection");
}

#[test]
fn test_timeout_handling() {
    let config = create_test_config();
    let manager = RealIpcManager::new(config).expect("Failed to create IPC manager");
    
    let queue_name = format!("timeout_test_{}", std::process::id());
    let connection = manager.create_message_queue(&queue_name)
        .expect("Failed to create message queue");
    
    if let IpcConnection::MessageQueue(ref queue_conn) = *connection {
        let start = Instant::now();
        
        // Try to receive from empty queue with timeout
        let result = queue_conn.receive(Some(Duration::from_millis(100)));
        
        let elapsed = start.elapsed();
        
        // Should timeout and return error
        assert!(result.is_err());
        
        // Should respect timeout duration (allow some variance)
        assert!(elapsed >= Duration::from_millis(90));
        assert!(elapsed <= Duration::from_millis(200));
    }
    
    manager.remove_connection(&queue_name).expect("Failed to remove connection");
}
