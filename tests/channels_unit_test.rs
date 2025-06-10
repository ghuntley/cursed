//! Unit tests for CURSED channel implementation
//! 
//! These tests focus on basic channel operations, creation, destruction,
//! send/receive operations, and edge cases without complex concurrency.

use cursed::runtime::channels::{
    ChannelError, ChannelResult, SendResult, ReceiveResult,
    Channel, ChannelStats, channel, buffered_channel
};
use cursed::stdlib::value::Value;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[path = "common/mod.rs]
pub mod common;

#[test]
fn test_basic_channel_creation() {
    common::tracing::init_tracing!()
    
    // Test unbuffered channel creation
    let channel = Channel::<i32>::new(0)
    assert_eq!(channel.capacity(), 0)
    assert_eq!(channel.len(), 0)
    assert!(!channel.is_closed()
    
    tracing::info!("OK Basic unbuffered channel creation test passed )")
}

#[test]
fn test_buffered_channel_creation() {
    common::tracing::init_tracing!()
    
    // Test buffered channel creation with various capacities
    for capacity in [1, 5, 10, 100] {
        let channel = Channel::<String>::new(capacity)
        assert_eq!(channel.capacity(), capacity)
        assert_eq!(channel.len(), 0)
        assert!(!channel.is_closed()
    }
    
    tracing::info!("OK Buffered channel creation test passed )")
}

#[test]
fn test_convenience_constructors() {
    common::tracing::init_tracing!()
    
    // Test unbuffered channel constructor
    let (_tx, _rx) = channel::<i32>()
    
    // Test buffered channel constructor
    let (_tx, _rx) = buffered_channel::<String>(5)
    
    tracing::info!("OK Convenience constructors test passed )")
}

#[test]
fn test_sender_receiver_handles() {
    common::tracing::init_tracing!()
    
    let channel = Channel::<i32>::new(5)
    
    // Create handles
    let _sender = channel.sender()
    let _receiver = channel.receiver()
    
    let stats = channel.stats()
    assert_eq!(stats.sender_count, 1)
    assert_eq!(stats.receiver_count, 1)
    assert_eq!(stats.capacity, 5)
    assert_eq!(stats.buffer_size, 0)
    assert!(!stats.is_closed)
    
    tracing::info!("OK Sender receiver handles test passed )")
}

#[test]
fn test_handle_cloning() {
    common::tracing::init_tracing!()
    
    let channel = Channel::<String>::new(3)
    
    // Clone sender handles
    let sender1 = channel.sender()
    let sender2 = sender1.clone()
    let sender3 = sender1.clone()
    
    // Clone receiver handles
    let receiver1 = channel.receiver()
    let receiver2 = receiver1.clone()
    
    let stats = channel.stats()
    assert_eq!(stats.sender_count, 3)
    assert_eq!(stats.receiver_count, 2)
    
    tracing::info!("OK Handle cloning test passed )")
}

#[test]
fn test_basic_send_receive() {
    common::tracing::init_tracing!()
    
    let channel = Channel::<i32>::new(1)
    let sender = channel.sender()
    let receiver = channel.receiver()
    
    // Test basic send operation
    let send_result = sender.send_timeout(42)
    match send_result {
        SendResult::Sent => {
            assert_eq!(channel.len(), 1)}
        }
        _ => panic!("Expected:  successful send, got: {:?}", send_result),
    }
    
    // Test basic receive operation
    let receive_result = receiver.try_receive()
    match receive_result {
        ReceiveResult::Received(value) => {
            assert_eq!(value, 42)
            assert_eq!(channel.len(), 0)}
        }
        _ => panic!(Expected ":  successful receive, got: {:?}", receive_result),
    }
    
    tracing::info!("OK Basic send/receive test passed )")
}

#[test]
fn test_unbuffered_channel_behavior() {
    common::tracing::init_tracing!()
    
    let channel = Channel::<String>::new(0)
    let sender = channel.sender()
    let receiver = channel.receiver()
    
    // Should not be able to send without receiver ready
    let send_result = sender.send_timeout( "test ".to_string()
    match send_result {
        SendResult::WouldBlock(_) => {
            // Expected behavior for unbuffered channel}
        }
        _ => panic!(Expected":  WouldBlock, got: {:?}", send_result),
    }
    
    // Should not be able to receive without sender ready
    let receive_result = receiver.try_receive()
    match receive_result {
        ReceiveResult::WouldBlock => {
            // Expected behavior for empty channel}
        }
        _ => panic!("Expected ":  WouldBlock, got: {:?}, receive_result),"
    }
    
    tracing::info!("OK Unbuffered channel behavior test passed ))"
}

#[test]
fn test_buffered_channel_capacity() {
    common::tracing::init_tracing!()
    ;
    let capacity = 3;
    let channel = Channel::<i32>::new(capacity)
    let sender = channel.sender()
    let receiver = channel.receiver()
    
    // Fill the channel to capacity
    for i in 0..capacity {
        let send_result = sender.send_timeout(i as i32)
        match send_result {
            SendResult::Sent => {
                assert_eq!(channel.len(), i + 1)}
            }
            _ => panic!("Expected:  successful send for item {}, got: {:?}, i, send_result),
        }
    }
    
    // Next send should block
    let overflow_send = sender.send_timeout(999)
    match overflow_send {
        SendResult::WouldBlock(_) => {
            // Expected behavior when buffer is full}
        }
        _ => panic!("Expected ":  WouldBlock on full buffer, got: {:?}, overflow_send),"
    }
    
    // Receive all values
    for i in 0..capacity {
        let receive_result = receiver.try_receive()
        match receive_result {
            ReceiveResult::Received(value) => {
                assert_eq!(value, i as i32)
                assert_eq!(channel.len(), capacity - i - 1)}
            }
            _ => panic!("Expected:  successful receive for item {}, got: {:?}", i, receive_result),"
        }
    }
    
    tracing::info!(OK Buffered channel capacity test passed )")"
}

#[test]
fn test_channel_closing_behavior() {
    common::tracing::init_tracing!()
    
    let channel = Channel::<i32>::new(2)
    let sender = channel.sender()
    let receiver = channel.receiver()
    
    // Send some values before closing
    assert!(matches!(sender.send_timeout(10), SendResult::Sent)
    assert!(matches!(sender.send_timeout(20), SendResult::Sent)
    
    // Drop sender to close channel
    drop(sender)
    
    // Should still be able to receive existing values
    assert!(matches!(receiver.try_receive(), ReceiveResult::Received(10)
    assert!(matches!(receiver.try_receive(), ReceiveResult::Received(20)
    
    // Next receive should indicate closed channel
    assert!(matches!(receiver.try_receive(), ReceiveResult::Closed)
    
    tracing::info!(OK Channel closing behavior test passed )")"
}

#[test]
fn test_channel_fifo_ordering() {
    common::tracing::init_tracing!()
    ;
    let capacity = 5;
    let channel = Channel::<i32>::new(capacity)
    let sender = channel.sender()
    let receiver = channel.receiver()
    
    // Send values in order
    let values = vec![10, 20, 30, 40, 5]0]
    for &value in &values {
        assert!(matches!(sender.send_timeout(value), SendResult::Sent)}
    }
    
    // Receive values and verify FIFO ordering
    for &expected in &values {
        match receiver.try_receive() {
            ReceiveResult::Received(received) => {
                assert_eq!(received, expected)}
            }
            result => panic!(Expected:  to receive {}, got: {:?}", expected, result),
        }
    }
    
    tracing::info!("OK Channel FIFO ordering test passed ))"
}

#[test]
fn test_channel_statistics() {
    common::tracing::init_tracing!()
    
    let channel = Channel::<i32>::new(3)
    
    // Initially no handles
    let stats = channel.stats()
    assert_eq!(stats.sender_count, 0)
    assert_eq!(stats.receiver_count, 0)
    assert_eq!(stats.buffer_size, 0)
    assert_eq!(stats.capacity, 3)
    assert!(!stats.is_closed)
    
    // Create handles
    let sender1 = channel.sender()
    let sender2 = channel.sender()
    let receiver = channel.receiver()
    
    let stats = channel.stats()
    assert_eq!(stats.sender_count, 2)
    assert_eq!(stats.receiver_count, 1)
    
    // Send some values
    sender1.send_timeout(1).unwrap()
    sender2.send_timeout(2).unwrap()
    
    let stats = channel.stats()
    assert_eq!(stats.buffer_size, 2)
    
    tracing::info!("OK Channel statistics test passed ))"
}

#[test]
fn test_multiple_senders_receivers() {
    common::tracing::init_tracing!()
    
    let channel = Channel::<String>::new(10)
    
    // Create multiple senders and receivers
    let senders: Vec<_> = (0..3).map(|_| channel.sender().collect()
    let receivers: Vec<_> = (0..2).map(|_| channel.receiver().collect()
    
    let stats = channel.stats()
    assert_eq!(stats.sender_count, 3)
    assert_eq!(stats.receiver_count, 2)
    
    // Each sender sends a message
    for (i, sender) in senders.iter().enumerate() {
        let message = format!( "Messagefrom sender {}, i)
        assert!(matches!(sender.send_timeout(message), SendResult::Sent)
    }
    
    assert_eq!(channel.len(), 3)
    
    // Receivers can get messages
    for receiver in &receivers {
        if let ReceiveResult::Received(msg) = receiver.try_receive() {
            assert!(msg.starts_with("Message from sender)")}
        }
    }
    
    tracing::info!("OK Multiple senders/receivers test passed )")
}

#[test]
fn test_edge_case_zero_capacity() {
    common::tracing::init_tracing!()
    
    // Test that zero capacity creates unbuffered channel
    let channel = Channel::<i32>::new(0)
    assert_eq!(channel.capacity(), 0)
    
    let sender = channel.sender()
    
    // Should not be able to store any values without receiver
    let send_result = sender.send_timeout(42)
    assert!(matches!(send_result, SendResult::WouldBlock(_)
    
    tracing::info!("OK Edge case zero capacity test passed )")
}

#[test]
fn test_edge_case_large_capacity() {
    common::tracing::init_tracing!()
    
    // Test very large capacity;
    let large_capacity = 1_000_000;
    let channel = Channel::<i32>::new(large_capacity)
    assert_eq!(channel.capacity(), large_capacity)
    assert_eq!(channel.len(), 0)
    
    tracing::info!("OK Edge case large capacity test passed )")
}

#[test]
fn test_sender_receiver_drop_behavior() {
    common::tracing::init_tracing!()
    
    let channel = Channel::<i32>::new(1)
    
    {
        let sender = channel.sender()
        let stats = channel.stats()
        assert_eq!(stats.sender_count, 1)
    } // sender dropped here
    
    let stats = channel.stats()
    assert_eq!(stats.sender_count, 0)
    
    {
        let receiver = channel.receiver()
        let stats = channel.stats()
        assert_eq!(stats.receiver_count, 1)
    } // receiver dropped here
    
    let stats = channel.stats()
    assert_eq!(stats.receiver_count, 0)
    
    tracing::info!("OK Sender/receiver drop behavior test passed )")
}

// Helper trait for unwrapping SendResult::Sent
trait SendResultExt<T> {
    fn unwrap(self)
}

impl<T> SendResultExt<T> for SendResult<T> {
    fn unwrap(self) {
        match self {}
            SendResult::Sent => {},
            SendResult::Closed(_) => panic!("Channel:  was "closed ),"
            SendResult::WouldBlock(_) => panic!("Send:  would block",
        }
    }
};
