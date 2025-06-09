//! Comprehensive tests for the buffered channel system

use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Duration;

#[path = "../tests/common.rs"]
mod common;

use cursed::runtime::buffered_channel::{BufferedChannel, ThreadSafeBufferedChannel, ChannelStats};
use cursed::object::Object;
use cursed::error::Error;

/// Test basic buffered channel operations
#[test]
fn test_buffered_channel_basic_operations() {
    common::tracing::setup();
    
    let mut channel = BufferedChannel::new("int".to_string(), 3);
    
    // Test initial state
    assert_eq!(channel.capacity(), 3);
    assert_eq!(channel.len(), 0);
    assert!(channel.is_empty());
    assert!(!channel.is_full());
    assert!(!channel.is_closed());
    assert_eq!(channel.available_space(), 3);
    
    // Test sending values
    assert!(channel.try_send(Object::Integer(1)).unwrap());
    assert_eq!(channel.len(), 1);
    assert_eq!(channel.available_space(), 2);
    
    assert!(channel.try_send(Object::Integer(2)).unwrap());
    assert!(channel.try_send(Object::Integer(3)).unwrap());
    assert!(channel.is_full());
    assert_eq!(channel.available_space(), 0);
    
    // Test buffer full behavior
    assert!(!channel.try_send(Object::Integer(4)).unwrap());
    
    // Test receiving values
    assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(1));
    assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(2));
    assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(3));
    
    // Test empty buffer behavior
    assert!(channel.try_receive().unwrap().is_none());
    assert!(channel.is_empty());
}

/// Test channel closing behavior
#[test]
fn test_buffered_channel_close_behavior() {
    common::tracing::setup();
    
    let mut channel = BufferedChannel::new("int".to_string(), 2);
    
    // Add some values before closing
    channel.try_send(Object::Integer(1)).unwrap();
    channel.try_send(Object::Integer(2)).unwrap();
    
    // Close the channel
    channel.close();
    assert!(channel.is_closed());
    
    // Test that sending fails after close
    let result = channel.try_send(Object::Integer(3));
    assert!(result.is_err());
    
    // Test that we can still receive existing values
    assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(1));
    assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(2));
    
    // Test that receiving fails on empty closed channel
    let result = channel.try_receive();
    assert!(result.is_err());
}

/// Test thread-safe buffered channel operations
#[test]
fn test_thread_safe_buffered_channel() {
    common::tracing::setup();
    
    let channel = ThreadSafeBufferedChannel::new("int".to_string(), 5);
    
    // Test basic operations
    assert!(channel.try_send(Object::Integer(42)).unwrap());
    assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(42));
    
    // Test statistics
    let stats = channel.stats();
    assert_eq!(stats.capacity, 5);
    assert_eq!(stats.current_length, 0);
    assert!(!stats.is_closed);
    assert_eq!(stats.available_space, 5);
    
    // Test channel properties
    assert_eq!(channel.capacity(), 5);
    assert_eq!(channel.len(), 0);
    assert!(!channel.is_closed());
}

/// Test concurrent access to buffered channels
#[test]
fn test_concurrent_buffered_channel_access() {
    common::tracing::setup();
    
    let channel = Arc::new(ThreadSafeBufferedChannel::new("int".to_string(), 10));
    let send_count = Arc::new(AtomicUsize::new(0));
    let receive_count = Arc::new(AtomicUsize::new(0));
    
    let producer_channel = Arc::clone(&channel);
    let producer_count = Arc::clone(&send_count);
    
    let consumer_channel = Arc::clone(&channel);
    let consumer_count = Arc::clone(&receive_count);
    
    // Start producer thread
    let producer = thread::spawn(move || {
        for i in 0..20 {
            while !producer_channel.try_send(Object::Integer(i)).unwrap() {
                thread::sleep(Duration::from_millis(1));
            }
            producer_count.fetch_add(1, Ordering::SeqCst);
        }
        producer_channel.close().unwrap();
    });
    
    // Start consumer thread
    let consumer = thread::spawn(move || {
        loop {
            match consumer_channel.try_receive() {
                Ok(Some(_)) => {
                    consumer_count.fetch_add(1, Ordering::SeqCst);
                }
                Ok(None) => {
                    // Would block, try again
                    thread::sleep(Duration::from_millis(1));
                }
                Err(_) => {
                    // Channel closed
                    break;
                }
            }
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    assert_eq!(send_count.load(Ordering::SeqCst), 20);
    assert_eq!(receive_count.load(Ordering::SeqCst), 20);
    assert!(channel.is_closed());
}

/// Test blocking operations with timeouts
#[test]
fn test_blocking_operations_with_timeout() {
    common::tracing::setup();
    
    let channel = ThreadSafeBufferedChannel::new("int".to_string(), 1);
    
    // Fill the channel
    channel.send(Object::Integer(1)).unwrap();
    
    // Test send timeout
    let timeout_result = channel.send_timeout(Object::Integer(2), Duration::from_millis(10));
    assert_eq!(timeout_result.unwrap(), false); // Should timeout
    
    // Test receive timeout with empty channel after receiving the value
    channel.receive().unwrap(); // Remove the value
    let timeout_result = channel.receive_timeout(Duration::from_millis(10));
    assert!(timeout_result.unwrap().is_none()); // Should timeout
}

/// Test channel buffer overflow and underflow scenarios
#[test]
fn test_buffer_overflow_underflow_scenarios() {
    common::tracing::setup();
    
    let mut channel = BufferedChannel::new("string".to_string(), 2);
    
    // Test overflow scenario
    assert!(channel.try_send(Object::String("first".to_string())).unwrap());
    assert!(channel.try_send(Object::String("second".to_string())).unwrap());
    
    // Buffer is full, next send should fail
    assert!(!channel.try_send(Object::String("third".to_string())).unwrap());
    
    // Test underflow scenario
    assert_eq!(channel.try_receive().unwrap().unwrap(), Object::String("first".to_string()));
    assert_eq!(channel.try_receive().unwrap().unwrap(), Object::String("second".to_string()));
    
    // Buffer is empty, next receive should return None
    assert!(channel.try_receive().unwrap().is_none());
}

/// Test circular buffer efficiency
#[test]
fn test_circular_buffer_efficiency() {
    common::tracing::setup();
    
    let mut channel = BufferedChannel::new("int".to_string(), 3);
    
    // Fill and empty the buffer multiple times to test circular behavior
    for cycle in 0..5 {
        let base = cycle * 10;
        
        // Fill the buffer
        for i in 0..3 {
            assert!(channel.try_send(Object::Integer(base + i)).unwrap());
        }
        assert!(channel.is_full());
        
        // Empty the buffer
        for i in 0..3 {
            assert_eq!(channel.try_receive().unwrap().unwrap(), Object::Integer(base + i));
        }
        assert!(channel.is_empty());
    }
}

/// Test channel statistics accuracy
#[test]
fn test_channel_statistics() {
    common::tracing::setup();
    
    let channel = ThreadSafeBufferedChannel::new("float".to_string(), 5);
    
    // Initial stats
    let stats = channel.stats();
    assert_eq!(stats.element_type, "float");
    assert_eq!(stats.capacity, 5);
    assert_eq!(stats.current_length, 0);
    assert!(!stats.is_closed);
    assert_eq!(stats.available_space, 5);
    assert_eq!(stats.send_waiters, 0);
    assert_eq!(stats.recv_waiters, 0);
    
    // Add some values
    channel.send(Object::Float(1.5)).unwrap();
    channel.send(Object::Float(2.5)).unwrap();
    
    let stats = channel.stats();
    assert_eq!(stats.current_length, 2);
    assert_eq!(stats.available_space, 3);
    
    // Close and check stats
    channel.close().unwrap();
    let stats = channel.stats();
    assert!(stats.is_closed);
}

/// Test multiple data types in channels
#[test]
fn test_multiple_data_types() {
    common::tracing::setup();
    
    // Test with integers
    let mut int_channel = BufferedChannel::new("int".to_string(), 2);
    assert!(int_channel.try_send(Object::Integer(123)).unwrap());
    assert_eq!(int_channel.try_receive().unwrap().unwrap(), Object::Integer(123));
    
    // Test with floats
    let mut float_channel = BufferedChannel::new("float".to_string(), 2);
    assert!(float_channel.try_send(Object::Float(3.14)).unwrap());
    assert_eq!(float_channel.try_receive().unwrap().unwrap(), Object::Float(3.14));
    
    // Test with booleans
    let mut bool_channel = BufferedChannel::new("bool".to_string(), 2);
    assert!(bool_channel.try_send(Object::Boolean(true)).unwrap());
    assert_eq!(bool_channel.try_receive().unwrap().unwrap(), Object::Boolean(true));
    
    // Test with strings
    let mut string_channel = BufferedChannel::new("string".to_string(), 2);
    assert!(string_channel.try_send(Object::String("test".to_string())).unwrap());
    assert_eq!(string_channel.try_receive().unwrap().unwrap(), Object::String("test".to_string()));
}

/// Test channel clone behavior
#[test]
fn test_channel_clone_behavior() {
    common::tracing::setup();
    
    let channel1 = ThreadSafeBufferedChannel::new("int".to_string(), 3);
    let channel2 = channel1.clone();
    
    // Both references should point to the same channel
    channel1.send(Object::Integer(42)).unwrap();
    assert_eq!(channel2.receive().unwrap(), Object::Integer(42));
    
    // Closing from one reference should affect the other
    channel1.close().unwrap();
    assert!(channel2.is_closed());
}

/// Test error conditions and edge cases
#[test]
fn test_error_conditions_and_edge_cases() {
    common::tracing::setup();
    
    // Test zero capacity channel
    let zero_cap_channel = BufferedChannel::new("int".to_string(), 0);
    assert_eq!(zero_cap_channel.capacity(), 0);
    assert!(zero_cap_channel.is_full()); // Zero capacity is always full
    
    // Test very large capacity
    let large_cap_channel = BufferedChannel::new("int".to_string(), 1_000_000);
    assert_eq!(large_cap_channel.capacity(), 1_000_000);
    assert!(!large_cap_channel.is_full());
    
    // Test channel with different element types
    let byte_channel = BufferedChannel::new("byte".to_string(), 10);
    assert_eq!(byte_channel.element_type(), "byte");
    
    let custom_channel = BufferedChannel::new("custom_type".to_string(), 5);
    assert_eq!(custom_channel.element_type(), "custom_type");
}

/// Test performance characteristics
#[test]
fn test_performance_characteristics() {
    common::tracing::setup();
    
    let channel = ThreadSafeBufferedChannel::new("int".to_string(), 1000);
    
    // Measure time for bulk operations
    let start = std::time::Instant::now();
    
    // Send many values
    for i in 0..1000 {
        channel.send(Object::Integer(i)).unwrap();
    }
    
    // Receive many values
    for _ in 0..1000 {
        channel.receive().unwrap();
    }
    
    let elapsed = start.elapsed();
    tracing::info!("Bulk operations completed in {:?}", elapsed);
    
    // Performance should be reasonable (this is a rough check)
    assert!(elapsed < Duration::from_secs(1), "Operations took too long: {:?}", elapsed);
}

/// Test integration with existing channel system
#[test]
fn test_integration_with_existing_system() {
    common::tracing::setup();
    
    let buffered_channel = BufferedChannel::new("int".to_string(), 5);
    
    // Test conversion to Object (if implemented)
    // This would test integration with the existing Object system
    assert_eq!(buffered_channel.element_type(), "int");
    assert_eq!(buffered_channel.capacity(), 5);
    
    // Test that the buffered channel maintains compatibility
    // with the existing channel interface expectations
    let mut channel = buffered_channel;
    channel.try_send(Object::Integer(1)).unwrap();
    assert_eq!(channel.len(), 1);
    
    let received = channel.try_receive().unwrap().unwrap();
    assert_eq!(received, Object::Integer(1));
    assert_eq!(channel.len(), 0);
}
