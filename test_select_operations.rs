// Test file for the fixed async/channel select operations

use std::sync::Arc;
use std::time::Duration;

use cursed::runtime::channels::buffer::{RingBuffer, UnbufferedChannel, ChannelBuffer};
use cursed::runtime::channels::select::{Select, SelectResult};

fn test_select_send_operation() {
    // Create a buffered channel
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    
    // Create a select operation with a send case
    let mut select = Select::new();
    select.send(1, channel.clone(), 42);
    
    // Execute the select - should complete immediately as buffer has space
    let result = select.execute();
    assert!(result.is_ok());
    
    match result.unwrap() {
        SelectResult::SendCompleted(case_index) => {
            println!("Send completed on case {}", case_index);
            assert_eq!(case_index, 0); // First case
        }
        _ => panic!("Expected SendCompleted result"),
    }
}

fn test_select_receive_operation() {
    // Create a buffered channel and add some data
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    channel.try_push(123).unwrap();
    
    // Create a select operation with a receive case
    let mut select = Select::new();
    select.receive(1, channel.clone());
    
    // Execute the select - should complete immediately as buffer has data
    let result = select.execute();
    assert!(result.is_ok());
    
    match result.unwrap() {
        SelectResult::ReceiveCompleted(case_index, value) => {
            println!("Received value on case {}: {:?}", case_index, value);
            assert_eq!(case_index, 0); // First case
            // Verify the value can be downcast back to i32
            let typed_value = value.downcast_ref::<i32>().unwrap();
            assert_eq!(*typed_value, 123);
        }
        _ => panic!("Expected ReceiveCompleted result"),
    }
}

fn test_select_default_case() {
    // Create an empty unbuffered channel
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(UnbufferedChannel::new());
    
    // Create a select operation with receive and default cases
    let mut select = Select::new();
    select.receive(1, channel.clone());
    select.default_case();
    
    // Execute the select - should execute default case since no data available
    let result = select.execute();
    assert!(result.is_ok());
    
    match result.unwrap() {
        SelectResult::DefaultExecuted => {
            println!("Default case executed");
        }
        _ => panic!("Expected DefaultExecuted result"),
    }
}

fn test_select_with_timeout() {
    // Create an empty unbuffered channel
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(UnbufferedChannel::new());
    
    // Create a select operation with timeout
    let mut select = Select::new();
    select.receive(1, channel.clone());
    select.timeout(Duration::from_millis(10));
    
    // Execute the select - should timeout
    let result = select.execute();
    assert!(result.is_ok());
    
    match result.unwrap() {
        SelectResult::Timeout => {
            println!("Select operation timed out");
        }
        _ => panic!("Expected Timeout result"),
    }
}

fn test_multiple_channel_select() {
    // Create multiple channels
    let channel1: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(1));
    let channel2: Arc<dyn ChannelBuffer<String>> = Arc::new(RingBuffer::new(1));
    
    // Add data to second channel
    channel2.try_push("hello".to_string()).unwrap();
    
    // Create a select operation with multiple cases
    let mut select = Select::new();
    select.receive(1, channel1.clone());
    select.receive(2, channel2.clone());
    
    // Execute the select - should receive from channel2
    let result = select.execute();
    assert!(result.is_ok());
    
    match result.unwrap() {
        SelectResult::ReceiveCompleted(case_index, value) => {
            println!("Received on case {}: {:?}", case_index, value);
            // Should be case index 1 (second case, 0-indexed)
            assert_eq!(case_index, 1);
            let typed_value = value.downcast_ref::<String>().unwrap();
            assert_eq!(typed_value, "hello");
        }
        _ => panic!("Expected ReceiveCompleted result"),
    }
}

fn main() {
    println!("Testing select operations...");
    
    test_select_send_operation();
    println!("✓ Send operation test passed");
    
    test_select_receive_operation();
    println!("✓ Receive operation test passed");
    
    test_select_default_case();
    println!("✓ Default case test passed");
    
    test_select_with_timeout();
    println!("✓ Timeout test passed");
    
    test_multiple_channel_select();
    println!("✓ Multiple channel test passed");
    
    println!("All tests passed! Select operations are working correctly.");
}
