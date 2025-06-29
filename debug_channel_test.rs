// Debug channel operations

use std::sync::Arc;

use cursed::runtime::channels::buffer::{RingBuffer, ChannelBuffer};

fn main() {
    println!("Testing channel operations...");
    
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    
    println!("Initial state:");
    println!("  len: {}", channel.len());
    println!("  capacity: {}", channel.capacity());
    println!("  is_empty: {}", channel.is_empty());
    println!("  is_full: {}", channel.is_full());
    println!("  is_closed: {}", channel.is_closed());
    
    // Test can_send logic
    let can_send = !channel.is_full() && !channel.is_closed();
    println!("  can_send: {}", can_send);
    
    // Try sending
    println!("\nTrying to send...");
    match channel.try_push(42) {
        Ok(()) => println!("✓ Send successful"),
        Err((value, err)) => println!("✗ Send failed: {:?}", err),
    }
    
    println!("After send:");
    println!("  len: {}", channel.len());
    println!("  is_empty: {}", channel.is_empty());
    println!("  is_full: {}", channel.is_full());
    
    // Test can_receive logic
    let can_receive = !channel.is_empty();
    println!("  can_receive: {}", can_receive);
    
    // Try receiving
    println!("\nTrying to receive...");
    match channel.try_pop() {
        Ok(Some(value)) => println!("✓ Received: {}", value),
        Ok(None) => println!("• No data available"),
        Err(err) => println!("✗ Receive failed: {:?}", err),
    }
    
    println!("Test completed!");
}
