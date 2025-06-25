/// Comprehensive tests for channel select operations
/// Tests Go-like select statement functionality for non-blocking operations on multiple channels

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use cursed::runtime::channels::select::*;
use cursed::runtime::channels::{channel, buffered_channel, ChannelError, ChannelResult};

#[test]
fn test_select_receive_ready_channel() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx, rx) = buffered_channel::<i32>(1);
    
    // Send a value first
    tx.send(42).unwrap();
    
    let builder = SelectBuilder::new()
        .receive(1, rx);
    
    let result = selector.execute_select_nonblocking(builder).unwrap();
    
    assert_eq!(result.case_id, 1);
    match result.result {
        SelectResultValue::Received(value) => assert_eq!(value, 42),
        _ => panic!("Expected received value"),
    }
}

#[test]
fn test_select_send_ready_channel() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx, _rx) = buffered_channel::<i32>(1);
    
    let builder = SelectBuilder::new()
        .send(1, 42, tx);
    
    let result = selector.execute_select_nonblocking(builder).unwrap();
    
    assert_eq!(result.case_id, 1);
    match result.result {
        SelectResultValue::Sent => {},
        _ => panic!("Expected sent result"),
    }
}

#[test]
fn test_select_with_default_case() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx, rx) = channel::<i32>(); // Unbuffered channel
    
    let builder = SelectBuilder::new()
        .receive(1, rx)
        .send(2, 42, tx)
        .default();
    
    let result = selector.execute_select_nonblocking(builder).unwrap();
    
    // Should use default case since unbuffered channels would block
    match result.result {
        SelectResultValue::Default => {},
        _ => panic!("Expected default case to be used"),
    }
}

#[test]
fn test_select_multiple_channels_first_ready() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx1, rx1) = buffered_channel::<i32>(1);
    let (tx2, rx2) = buffered_channel::<i32>(1);
    let (tx3, rx3) = buffered_channel::<i32>(1);
    
    // Only send to first channel
    tx1.send(111).unwrap();
    
    let builder = SelectBuilder::new()
        .receive(1, rx1)
        .receive(2, rx2)
        .receive(3, rx3);
    
    let result = selector.execute_select_nonblocking(builder).unwrap();
    
    assert_eq!(result.case_id, 1);
    match result.result {
        SelectResultValue::Received(value) => assert_eq!(value, 111),
        _ => panic!("Expected to receive from first channel"),
    }
}

#[test]
fn test_select_with_priority() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx1, rx1) = buffered_channel::<i32>(1);
    let (tx2, rx2) = buffered_channel::<i32>(1);
    
    // Send to both channels
    tx1.send(111).unwrap();
    tx2.send(222).unwrap();
    
    let builder = SelectBuilder::new()
        .receive(1, rx1).priority(1)  // Lower priority
        .receive(2, rx2).priority(10); // Higher priority
    
    let result = selector.execute_select_nonblocking(builder).unwrap();
    
    // Higher priority case should be selected
    assert_eq!(result.case_id, 2);
    match result.result {
        SelectResultValue::Received(value) => assert_eq!(value, 222),
        _ => panic!("Expected to receive from high priority channel"),
    }
}

#[test]
fn test_select_blocking_until_ready() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx, rx) = channel::<i32>();
    
    let tx_clone = tx.clone();
    
    // Spawn thread to send after a delay
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        tx_clone.send(42).unwrap();
    });
    
    let start = Instant::now();
    let builder = SelectBuilder::new()
        .receive(1, rx);
    
    let result = selector.execute_select(builder).unwrap();
    let elapsed = start.elapsed();
    
    assert!(elapsed >= Duration::from_millis(40));
    assert_eq!(result.case_id, 1);
    match result.result {
        SelectResultValue::Received(value) => assert_eq!(value, 42),
        _ => panic!("Expected received value"),
    }
}

#[test]
fn test_select_with_timeout() {
    let mut selector = ChannelSelector::<i32>::new();
    let (_tx, rx) = channel::<i32>(); // No one will send
    
    let start = Instant::now();
    let builder = SelectBuilder::new()
        .receive(1, rx)
        .timeout(Duration::from_millis(100));
    
    let result = selector.execute_select(builder);
    let elapsed = start.elapsed();
    
    assert!(elapsed >= Duration::from_millis(90));
    assert!(result.is_err());
    match result.unwrap_err() {
        ChannelError::Timeout => {},
        err => panic!("Expected timeout error, got: {:?}", err),
    }
}

#[test]
fn test_select_closed_channel() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx, rx) = channel::<i32>();
    
    // Close the channel
    drop(tx);
    
    let builder = SelectBuilder::new()
        .receive(1, rx);
    
    let result = selector.execute_select_nonblocking(builder).unwrap();
    
    assert_eq!(result.case_id, 1);
    match result.result {
        SelectResultValue::Closed => {},
        _ => panic!("Expected closed result"),
    }
}

#[test]
fn test_select_concurrent_operations() {
    use std::sync::Mutex;
    
    let selector = Arc::new(Mutex::new(ChannelSelector::<i32>::new()));
    let (tx1, rx1) = buffered_channel::<i32>(10);
    let (tx2, rx2) = buffered_channel::<i32>(10);
    
    let mut handles = vec![];
    
    // Spawn multiple threads doing select operations
    for i in 0..5 {
        let selector_clone = selector.clone();
        let rx1_clone = rx1.clone();
        let rx2_clone = rx2.clone();
        
        let handle = thread::spawn(move || {
            let builder = SelectBuilder::new()
                .receive(1, rx1_clone)
                .receive(2, rx2_clone)
                .timeout(Duration::from_millis(200));
            
            let mut sel = selector_clone.lock().unwrap();
            sel.execute_select(builder)
        });
        
        handles.push(handle);
    }
    
    // Send some values
    for i in 0..3 {
        tx1.send(i).unwrap();
        tx2.send(i + 100).unwrap();
    }
    
    // Collect results
    let mut successful_receives = 0;
    for handle in handles {
        match handle.join().unwrap() {
            Ok(result) => {
                match result.result {
                    SelectResultValue::Received(_) => successful_receives += 1,
                    _ => {},
                }
            },
            Err(ChannelError::Timeout) => {
                // Some operations may timeout, which is expected
            },
            Err(err) => panic!("Unexpected error: {:?}", err),
        }
    }
    
    // Should have received some values
    assert!(successful_receives > 0);
}

#[test]
fn test_select_send_on_full_channel() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx, _rx) = buffered_channel::<i32>(1);
    
    // Fill the buffer
    tx.send(1).unwrap();
    
    let builder = SelectBuilder::new()
        .send(1, 42, tx)
        .default();
    
    let result = selector.execute_select_nonblocking(builder).unwrap();
    
    // Should use default case since channel is full
    match result.result {
        SelectResultValue::Default => {},
        _ => panic!("Expected default case due to full channel"),
    }
}

#[test]
fn test_select_mixed_send_receive() {
    let mut selector = ChannelSelector::<i32>::new();
    let (tx1, rx1) = buffered_channel::<i32>(1);
    let (tx2, rx2) = buffered_channel::<i32>(1);
    
    // Prepare one channel for receive, another for send
    tx1.send(42).unwrap(); // rx1 can receive
    // tx2/rx2 buffer is empty, can send
    
    let builder = SelectBuilder::new()
        .receive(1, rx1)
        .send(2, 100, tx2);
    
    let result = selector.execute_select_nonblocking(builder).unwrap();
    
    // Either operation should succeed
    match result.result {
        SelectResultValue::Received(42) if result.case_id == 1 => {},
        SelectResultValue::Sent if result.case_id == 2 => {},
        _ => panic!("Expected either receive or send to succeed"),
    }
}

#[test]
fn test_select_statistics() {
    let selector = ChannelSelector::<i32>::new();
    let stats = selector.get_stats();
    
    assert_eq!(stats.active_selects, 0);
    assert_eq!(stats.next_select_id, 1);
}

#[test]
fn test_select_handle_cancellation() {
    let handle = SelectHandle::new(123);
    
    assert!(!handle.is_cancelled());
    handle.cancel();
    assert!(handle.is_cancelled());
    
    // Wait should return timeout error for cancelled handle
    match handle.wait() {
        Err(ChannelError::Timeout) => {},
        _ => panic!("Expected timeout error for cancelled handle"),
    }
}

#[test]
fn test_select_builder_configuration() {
    use crate::runtime::channels::channel;
    
    let (tx, rx) = channel::<i32>();
    let builder = SelectBuilder::new()
        .send(1, 42, tx)
        .receive(2, rx)
        .timeout(Duration::from_millis(500))
        .no_randomize();
    
    assert_eq!(builder.case_count(), 2);
    assert!(builder.has_timeout());
    assert_eq!(builder.get_timeout(), Some(Duration::from_millis(500)));
}

#[test]
fn test_select_empty_cases() {
    let mut selector = ChannelSelector::<i32>::new();
    let builder = SelectBuilder::new();
    
    let result = selector.execute_select_nonblocking(builder);
    assert!(result.is_err());
    
    match result.unwrap_err() {
        ChannelError::InvalidState => {},
        err => panic!("Expected InvalidState error, got: {:?}", err),
    }
}

#[test]
fn test_select_performance_many_channels() {
    let mut selector = ChannelSelector::<i32>::new();
    let mut channels = vec![];
    
    // Create many channels, some with data
    for i in 0..20 {
        let (tx, rx) = buffered_channel::<i32>(1);
        if i % 3 == 0 {
            tx.send(i).unwrap(); // Every 3rd channel has data
        }
        channels.push((tx, rx));
    }
    
    let mut builder = SelectBuilder::new();
    for (i, (_tx, rx)) in channels.into_iter().enumerate() {
        builder = builder.receive(i as u64, rx);
    }
    
    let start = Instant::now();
    let result = selector.execute_select_nonblocking(builder).unwrap();
    let elapsed = start.elapsed();
    
    // Should complete quickly even with many channels
    assert!(elapsed < Duration::from_millis(100));
    
    // Should receive from one of the channels with data
    match result.result {
        SelectResultValue::Received(value) => {
            assert!(value % 3 == 0); // Should be from a channel we sent to
        },
        _ => panic!("Expected to receive a value"),
    }
}
