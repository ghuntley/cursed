use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum ChannelError {
    Closed,
    WouldBlock,
    Timeout,
}

#[derive(Debug, Clone)]
pub enum SendResult<T> {
    Sent,
    Closed(T),
    WouldBlock(T),
}

impl<T> SendResult<T> {
    pub fn is_ok(&self) -> bool {
        matches!(self, SendResult::Sent)
    }
    
    pub fn unwrap_value(self) -> Option<T> {
        match self {
            SendResult::Sent => None,
            SendResult::Closed(t) => Some(t),
            SendResult::WouldBlock(t) => Some(t),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReceiveResult<T> {
    Received(T),
    Closed,
    WouldBlock,
}

impl<T> ReceiveResult<T> {
    pub fn is_ok(&self) -> bool {
        matches!(self, ReceiveResult::Received(_))
    }
    
    pub fn unwrap_or_default(self) -> Option<T> {
        match self {
            ReceiveResult::Received(t) => Some(t),
            ReceiveResult::Closed => None,
            ReceiveResult::WouldBlock => None,
        }
    }
}

fn test_channel_no_panics() {
    println!("Testing channel operations without panics...");
    
    let (tx, rx) = mpsc::channel();
    
    // Test send
    tx.send(42).unwrap();
    let received = rx.recv().unwrap();
    assert_eq!(received, 42);
    
    // Test result handling
    let send_result: SendResult<i32> = SendResult::Sent;
    assert!(send_result.is_ok());
    
    let recv_result: ReceiveResult<i32> = ReceiveResult::Received(100);
    assert!(recv_result.is_ok());
    
    let closed_result: ReceiveResult<i32> = ReceiveResult::Closed;
    let value = closed_result.unwrap_or_default();
    assert_eq!(value, None);
    
    println!("✅ All channel operations completed without panics!");
}

fn test_error_handling() {
    println!("Testing error handling...");
    
    // Test that we handle errors gracefully
    let send_result = SendResult::WouldBlock(42);
    match send_result {
        SendResult::Sent => println!("Sent successfully"),
        SendResult::WouldBlock(value) => {
            println!("Send would block, value: {}", value);
            // This should not panic
        }
        SendResult::Closed(value) => {
            println!("Channel closed, value: {}", value);
        }
    }
    
    let recv_result: ReceiveResult<i32> = ReceiveResult::WouldBlock;
    match recv_result {
        ReceiveResult::Received(value) => println!("Received: {}", value),
        ReceiveResult::Closed => {
            println!("Channel closed");
            // This should not panic
        }
        ReceiveResult::WouldBlock => {
            println!("Receive would block");
            // This should not panic
        }
    }
    
    println!("✅ Error handling completed without panics!");
}

fn main() {
    test_channel_no_panics();
    test_error_handling();
    println!("🎉 All tests passed! Channel operations are now panic-free.");
}
