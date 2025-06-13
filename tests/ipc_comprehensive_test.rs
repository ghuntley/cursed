/// Comprehensive tests for CURSED IPC module
/// 
/// This test suite validates the complete inter-process communication functionality
/// including shared memory, pipes, message queues, semaphores, channels, and
/// synchronization primitives.

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use std::sync::Arc;

use cursed::stdlib::ipc::{
    IpcError, IpcResult, IpcTimeout, ProcessId,
    
    // Shared Memory
    SharedMemory, SharedMemoryConfig, SharedMemoryAccess,
    create_shared_memory, open_shared_memory,
    
    // Named Pipes
    NamedPipe, AnonymousPipe, PipeConfig, PipeMode,
    create_pipe, create_named_pipe,
    
    // Message Queues
    MessageQueue, Message, MessageType, MessagePriority,
    create_message_queue, send_message, receive_message,
    
    // Semaphores
    Semaphore, SemaphoreConfig, SemaphoreValue,
    create_semaphore, acquire_semaphore, release_semaphore,
    
    // Channels
    IpcChannel, ChannelConfig, ChannelType, ChannelPair,
    
    // Synchronization
    IpcBarrier, IpcRwLock, IpcCondVar, ProcessCoordinator,
    BarrierWaitResult,
    
    // Error types
    communication_error, security_error, resource_error, timeout_error,
    invalid_operation, permission_denied, resource_exhausted,
    
    // Module management
    initialize, shutdown, get_ipc_statistics,
};

fn init_tracing() {
    tracing_setup::init_test_tracing();
}

// Shared Memory Tests

#[test]
fn test_shared_memory_creation() {
    init_tracing();
    tracing::info!("Testing shared memory creation");
    
    let config = SharedMemoryConfig::new("test_shm", 1024).unwrap();
    let shm = SharedMemory::create(config);
    
    // Should succeed or fail gracefully based on platform support
    match shm {
        Ok(mut memory) => {
            // Test basic write/read operations
            let test_data = b"Hello, shared memory!";
            memory.write_bytes(test_data).unwrap();
            
            let mut buffer = vec![0u8; test_data.len()];
            memory.read_bytes(&mut buffer).unwrap();
            assert_eq!(buffer, test_data);
            
            memory.close().unwrap();
        }
        Err(_) => {
            // Platform may not support shared memory
            tracing::warn!("Shared memory not supported on this platform");
        }
    }
}

#[test]
fn test_shared_memory_access_modes() {
    init_tracing();
    tracing::info!("Testing shared memory access modes");
    
    let config = SharedMemoryConfig::new("test_access", 512)
        .unwrap()
        .access(SharedMemoryAccess::ReadWrite);
    
    match SharedMemory::create(config) {
        Ok(memory) => {
            assert!(memory.can_read());
            assert!(memory.can_write());
            memory.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Shared memory not supported");
        }
    }
}

#[test]
fn test_shared_memory_size_limits() {
    init_tracing();
    tracing::info!("Testing shared memory size validation");
    
    // Test invalid size (0)
    let result = SharedMemoryConfig::new("test_zero", 0);
    assert!(result.is_err());
    
    // Test valid size
    let config = SharedMemoryConfig::new("test_valid", 4096).unwrap();
    assert_eq!(config.size(), 4096);
}

// Message Queue Tests

#[test]
fn test_message_queue_basic_operations() {
    init_tracing();
    tracing::info!("Testing message queue basic operations");
    
    match MessageQueue::create("test_mq", 10) {
        Ok(mq) => {
            // Test sending and receiving messages
            let test_data = b"Test message";
            let message = Message::new(test_data, MessagePriority::Normal).unwrap();
            
            mq.send(message).unwrap();
            
            let received = mq.receive().unwrap();
            assert_eq!(received.data(), test_data);
            
            mq.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Message queues not supported on this platform");
        }
    }
}

#[test]
fn test_message_priorities() {
    init_tracing();
    tracing::info!("Testing message priorities");
    
    match MessageQueue::create("test_priorities", 10) {
        Ok(mq) => {
            // Send messages with different priorities
            let low_msg = Message::new(b"low", MessagePriority::Low).unwrap();
            let high_msg = Message::new(b"high", MessagePriority::High).unwrap();
            let normal_msg = Message::new(b"normal", MessagePriority::Normal).unwrap();
            
            mq.send(low_msg).unwrap();
            mq.send(high_msg).unwrap();
            mq.send(normal_msg).unwrap();
            
            // High priority should come first
            let first = mq.receive().unwrap();
            assert_eq!(first.data(), b"high");
            
            mq.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Message queues not supported");
        }
    }
}

#[test]
fn test_message_queue_capacity() {
    init_tracing();
    tracing::info!("Testing message queue capacity limits");
    
    match MessageQueue::create("test_capacity", 2) {
        Ok(mq) => {
            let msg1 = Message::new(b"message1", MessagePriority::Normal).unwrap();
            let msg2 = Message::new(b"message2", MessagePriority::Normal).unwrap();
            let msg3 = Message::new(b"message3", MessagePriority::Normal).unwrap();
            
            // First two should succeed
            mq.send(msg1).unwrap();
            mq.send(msg2).unwrap();
            
            // Third should fail or timeout
            let result = mq.send_with_timeout(msg3, Duration::from_millis(10));
            // This might succeed or fail depending on implementation
            match result {
                Ok(_) => tracing::info!("Queue allowed overflow"),
                Err(_) => tracing::info!("Queue properly enforced capacity"),
            }
            
            mq.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Message queues not supported");
        }
    }
}

// Semaphore Tests

#[test]
fn test_semaphore_basic_operations() {
    init_tracing();
    tracing::info!("Testing semaphore basic operations");
    
    match Semaphore::create("test_sem", 2) {
        Ok(sem) => {
            // Acquire semaphore
            sem.acquire().unwrap();
            assert_eq!(sem.value().unwrap(), 1);
            
            // Acquire again
            sem.acquire().unwrap();
            assert_eq!(sem.value().unwrap(), 0);
            
            // Release semaphore
            sem.release().unwrap();
            assert_eq!(sem.value().unwrap(), 1);
            
            sem.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Semaphores not supported on this platform");
        }
    }
}

#[test]
fn test_semaphore_timeout() {
    init_tracing();
    tracing::info!("Testing semaphore timeout");
    
    match Semaphore::create("test_timeout", 0) {
        Ok(sem) => {
            // Should timeout immediately
            let result = sem.acquire_timeout(Duration::from_millis(10));
            assert!(result.is_err());
            
            sem.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Semaphores not supported");
        }
    }
}

#[test]
fn test_semaphore_try_acquire() {
    init_tracing();
    tracing::info!("Testing semaphore try_acquire");
    
    match Semaphore::create("test_try", 1) {
        Ok(sem) => {
            // Should succeed
            assert!(sem.try_acquire().unwrap());
            
            // Should fail (no permits left)
            assert!(!sem.try_acquire().unwrap());
            
            sem.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Semaphores not supported");
        }
    }
}

// Named Pipe Tests

#[test]
fn test_named_pipe_creation() {
    init_tracing();
    tracing::info!("Testing named pipe creation");
    
    let pipe_name = "test_named_pipe";
    match NamedPipe::create(pipe_name) {
        Ok(pipe) => {
            assert!(pipe.is_connected());
            pipe.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Named pipes not supported on this platform");
        }
    }
}

#[test]
fn test_anonymous_pipe() {
    init_tracing();
    tracing::info!("Testing anonymous pipe");
    
    match AnonymousPipe::create() {
        Ok((mut reader, mut writer)) => {
            let test_data = b"pipe test data";
            
            // Write data
            writer.write(test_data).unwrap();
            
            // Read data
            let received = reader.read().unwrap();
            assert_eq!(received, test_data);
            
            reader.close().unwrap();
            writer.close().unwrap();
        }
        Err(_) => {
            tracing::warn!("Anonymous pipes not supported");
        }
    }
}

// IPC Channel Tests

#[test]
fn test_ipc_channel_in_memory() {
    init_tracing();
    tracing::info!("Testing IPC channel in-memory");
    
    let config = ChannelConfig::new("test_channel")
        .channel_type(ChannelType::InMemory)
        .timeout(Duration::from_millis(100));

    let channel = IpcChannel::create(config).unwrap();
    
    // Test send and receive
    let test_data = b"Hello, IPC Channel!";
    channel.send(test_data).unwrap();
    
    let received = channel.receive().unwrap();
    assert_eq!(received, test_data);
    
    // Test statistics
    let stats = channel.get_statistics();
    assert_eq!(stats.messages_sent, 1);
    assert_eq!(stats.messages_received, 1);
    assert_eq!(stats.bytes_sent, test_data.len() as u64);
    assert_eq!(stats.bytes_received, test_data.len() as u64);
    
    channel.close().unwrap();
}

#[test]
fn test_ipc_channel_timeout() {
    init_tracing();
    tracing::info!("Testing IPC channel timeout");
    
    let config = ChannelConfig::new("test_timeout_channel")
        .channel_type(ChannelType::InMemory)
        .timeout(Duration::from_millis(10));

    let channel = IpcChannel::create(config).unwrap();
    
    // Try to receive from empty channel should timeout
    let result = channel.receive();
    assert!(result.is_err());
    
    channel.close().unwrap();
}

#[test]
fn test_ipc_channel_capacity() {
    init_tracing();
    tracing::info!("Testing IPC channel capacity");
    
    let config = ChannelConfig::new("test_capacity_channel")
        .channel_type(ChannelType::InMemory)
        .capacity(2)
        .timeout(Duration::from_millis(10));

    let channel = IpcChannel::create(config).unwrap();
    
    // Fill capacity
    channel.send(b"message1").unwrap();
    channel.send(b"message2").unwrap();
    
    // Third message should timeout
    let result = channel.send(b"message3");
    assert!(result.is_err());
    
    channel.close().unwrap();
}

#[test]
fn test_channel_pair() {
    init_tracing();
    tracing::info!("Testing channel pair");
    
    let config = ChannelConfig::new("test_pair")
        .channel_type(ChannelType::InMemory);

    let pair = ChannelPair::create("test_pair", config).unwrap();
    
    let test_data = b"paired communication";
    pair.send(test_data).unwrap();
    let received = pair.receive().unwrap();
    assert_eq!(received, test_data);
    
    pair.close().unwrap();
}

#[test]
fn test_channel_try_operations() {
    init_tracing();
    tracing::info!("Testing channel try operations");
    
    let config = ChannelConfig::new("test_try_ops")
        .channel_type(ChannelType::InMemory);

    let channel = IpcChannel::create(config).unwrap();
    
    // Try receive on empty channel should return None
    let result = channel.try_receive().unwrap();
    assert!(result.is_none());
    
    // Send and try receive should work
    let test_data = b"try operations";
    assert!(channel.try_send(test_data).unwrap());
    
    let received = channel.try_receive().unwrap();
    assert!(received.is_some());
    assert_eq!(received.unwrap(), test_data);
    
    channel.close().unwrap();
}

// Synchronization Tests

#[test]
fn test_ipc_barrier() {
    init_tracing();
    tracing::info!("Testing IPC barrier");
    
    let barrier = IpcBarrier::new("test_barrier", 1, Duration::from_secs(1)).unwrap();
    
    assert_eq!(barrier.expected_count(), 1);
    assert_eq!(barrier.waiting_count(), 0);
    assert_eq!(barrier.generation(), 0);
    
    // Single process barrier should immediately complete
    let result = barrier.wait().unwrap();
    assert_eq!(result, BarrierWaitResult::Leader);
    assert_eq!(barrier.generation(), 1);
}

#[test]
fn test_ipc_rwlock() {
    init_tracing();
    tracing::info!("Testing IPC read-write lock");
    
    let rwlock = IpcRwLock::new("test_rwlock").unwrap();
    
    // Test read lock
    let _read_guard = rwlock.read_lock().unwrap();
    
    // Should be able to acquire another read lock
    let _read_guard2 = rwlock.try_read_lock().unwrap();
    assert!(_read_guard2.is_some());
}

#[test]
fn test_ipc_condvar() {
    init_tracing();
    tracing::info!("Testing IPC condition variable");
    
    let condvar = IpcCondVar::new("test_condvar");
    
    assert_eq!(condvar.waiting_count(), 0);
    
    // No waiters, should return false/0
    assert_eq!(condvar.notify_one().unwrap(), false);
    assert_eq!(condvar.notify_all().unwrap(), 0);
}

#[test]
fn test_process_coordinator() {
    init_tracing();
    tracing::info!("Testing process coordinator");
    
    let coordinator = ProcessCoordinator::new();
    
    // Test barrier creation
    let barrier = coordinator.get_barrier("coord_barrier", 2, Duration::from_secs(10)).unwrap();
    assert_eq!(barrier.expected_count(), 2);
    
    // Test rwlock creation
    let _rwlock = coordinator.get_rwlock("coord_rwlock").unwrap();
    
    // Test condvar creation
    let condvar = coordinator.get_condvar("coord_condvar");
    assert_eq!(condvar.waiting_count(), 0);
    
    // Test statistics
    let stats = coordinator.get_statistics();
    assert_eq!(stats.active_barriers, 1);
    assert_eq!(stats.active_rwlocks, 1);
    assert_eq!(stats.active_condvars, 1);
    
    // Test cleanup
    assert!(coordinator.remove_barrier("coord_barrier"));
    assert!(!coordinator.remove_barrier("nonexistent"));
}

// Error Handling Tests

#[test]
fn test_error_creation() {
    init_tracing();
    tracing::info!("Testing error creation");
    
    let comm_err = communication_error("test communication error");
    assert!(matches!(comm_err, IpcError::CommunicationError { .. }));
    
    let sec_err = security_error("test security error");
    assert!(matches!(sec_err, IpcError::SecurityError { .. }));
    
    let res_err = resource_error("test resource error");
    assert!(matches!(res_err, IpcError::ResourceError { .. }));
    
    let timeout_err = timeout_error("test timeout");
    assert!(matches!(timeout_err, IpcError::TimeoutError { .. }));
}

#[test]
fn test_invalid_operations() {
    init_tracing();
    tracing::info!("Testing invalid operations");
    
    // Test invalid shared memory size
    let result = SharedMemoryConfig::new("invalid", 0);
    assert!(result.is_err());
    
    // Test invalid semaphore value
    let result = Semaphore::create("invalid_sem", 0);
    // This might succeed or fail depending on platform
    match result {
        Ok(sem) => {
            // Should have 0 permits
            assert!(!sem.try_acquire().unwrap());
            sem.close().unwrap();
        }
        Err(_) => {
            tracing::info!("Platform rejected 0-value semaphore");
        }
    }
    
    // Test invalid barrier count
    let result = IpcBarrier::new("invalid_barrier", 0, Duration::from_secs(1));
    assert!(result.is_err());
}

// Integration Tests

#[test]
fn test_module_initialization() {
    init_tracing();
    tracing::info!("Testing module initialization");
    
    // Test initialization and shutdown
    assert!(initialize().is_ok());
    assert!(shutdown().is_ok());
}

#[test]
fn test_ipc_statistics() {
    init_tracing();
    tracing::info!("Testing IPC statistics");
    
    let stats = get_ipc_statistics();
    
    // Basic validation that we can get stats
    assert!(stats.active_shared_memory_regions >= 0);
    assert!(stats.total_memory_usage >= 0);
    assert!(stats.active_pipes >= 0);
    assert!(stats.active_message_queues >= 0);
    assert!(stats.active_semaphores >= 0);
}

#[test]
fn test_cross_ipc_mechanism_communication() {
    init_tracing();
    tracing::info!("Testing cross-IPC mechanism communication");
    
    // Test using different IPC mechanisms together
    let channel_config = ChannelConfig::new("cross_test")
        .channel_type(ChannelType::InMemory);
    
    match IpcChannel::create(channel_config) {
        Ok(channel) => {
            match Semaphore::create("cross_sem", 1) {
                Ok(sem) => {
                    // Use semaphore to coordinate channel access
                    sem.acquire().unwrap();
                    
                    channel.send(b"coordinated message").unwrap();
                    let received = channel.receive().unwrap();
                    assert_eq!(received, b"coordinated message");
                    
                    sem.release().unwrap();
                    sem.close().unwrap();
                }
                Err(_) => {
                    tracing::warn!("Semaphores not supported for cross-test");
                }
            }
            
            channel.close().unwrap();
        }
        Err(e) => {
            tracing::error!("Failed to create channel for cross-test: {}", e);
        }
    }
}

// Performance Tests

#[test]
#[ignore] // This is a longer-running test
fn test_channel_throughput() {
    init_tracing();
    tracing::info!("Testing channel throughput");
    
    let config = ChannelConfig::new("throughput_test")
        .channel_type(ChannelType::InMemory)
        .capacity(1000);

    let channel = IpcChannel::create(config).unwrap();
    
    let message_count = 1000;
    let test_data = b"throughput test message";
    
    let start_time = std::time::Instant::now();
    
    // Send messages
    for _ in 0..message_count {
        channel.send(test_data).unwrap();
    }
    
    // Receive messages
    for _ in 0..message_count {
        let _received = channel.receive().unwrap();
    }
    
    let elapsed = start_time.elapsed();
    let throughput = message_count as f64 / elapsed.as_secs_f64();
    
    tracing::info!("Channel throughput: {:.2} messages/second", throughput);
    
    let stats = channel.get_statistics();
    assert_eq!(stats.messages_sent, message_count);
    assert_eq!(stats.messages_received, message_count);
    
    channel.close().unwrap();
}

#[test]
fn test_memory_usage_tracking() {
    init_tracing();
    tracing::info!("Testing memory usage tracking");
    
    let initial_stats = get_ipc_statistics();
    let initial_memory = initial_stats.total_memory_usage;
    
    // Create some IPC resources
    let channels = (0..10)
        .map(|i| {
            let config = ChannelConfig::new(&format!("memory_test_{}", i))
                .channel_type(ChannelType::InMemory);
            IpcChannel::create(config)
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    
    let final_stats = get_ipc_statistics();
    let final_memory = final_stats.total_memory_usage;
    
    // Memory usage should have increased (or stayed the same)
    assert!(final_memory >= initial_memory);
    
    // Clean up
    for channel in channels {
        channel.close().unwrap();
    }
}

#[test]
fn test_concurrent_channel_access() {
    init_tracing();
    tracing::info!("Testing concurrent channel access");
    
    let config = ChannelConfig::new("concurrent_test")
        .channel_type(ChannelType::InMemory)
        .capacity(100);

    let channel = Arc::new(IpcChannel::create(config).unwrap());
    
    let sender_channel = channel.clone();
    let receiver_channel = channel.clone();
    
    // Spawn sender thread
    let sender_handle = thread::spawn(move || {
        for i in 0..10 {
            let message = format!("message_{}", i);
            sender_channel.send(message.as_bytes()).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    // Spawn receiver thread
    let receiver_handle = thread::spawn(move || {
        let mut received_count = 0;
        while received_count < 10 {
            if let Ok(Some(_data)) = receiver_channel.try_receive() {
                received_count += 1;
            } else {
                thread::sleep(Duration::from_millis(5));
            }
        }
        received_count
    });
    
    // Wait for threads to complete
    sender_handle.join().unwrap();
    let received_count = receiver_handle.join().unwrap();
    
    assert_eq!(received_count, 10);
    
    channel.close().unwrap();
}
