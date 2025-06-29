// Debug can_send logic

use std::sync::Arc;

use cursed::runtime::channels::buffer::{RingBuffer, ChannelBuffer};
use cursed::runtime::channels::select::Select;

fn main() {
    println!("Testing can_send logic...");
    
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    
    println!("Before adding to select:");
    println!("  is_full: {}", channel.is_full());
    println!("  is_closed: {}", channel.is_closed());
    println!("  can_send (expected): {}", !channel.is_full() && !channel.is_closed());
    
    let mut select = Select::new();
    select.send(1, channel.clone(), 42);
    
    // Try to send directly to channel to make sure it works
    println!("\nTesting direct channel send:");
    match channel.try_push(43) {
        Ok(()) => println!("✓ Direct send successful"),
        Err((_, err)) => println!("✗ Direct send failed: {:?}", err),
    }
    
    println!("After direct send:");
    println!("  len: {}", channel.len());
    println!("  is_full: {}", channel.is_full());
    
    // Remove the data
    match channel.try_pop() {
        Ok(Some(val)) => println!("✓ Popped: {}", val),
        _ => println!("✗ Pop failed"),
    }
    
    // Now add default case and test
    select.default_case();
    
    println!("\nExecuting select with both send and default:");
    match select.execute() {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}
