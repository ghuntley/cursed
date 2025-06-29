// Debug ready cases

use std::sync::Arc;

use cursed::runtime::channels::buffer::{RingBuffer, ChannelBuffer};
use cursed::runtime::channels::select::Select;

fn main() {
    println!("Testing ready cases logic...");
    
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    
    let mut select = Select::new();
    select.send(1, channel.clone(), 42);
    
    // Test with send case only (no default)
    println!("Test 1: Send case only");
    match select.execute() {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Now test with receive case only
    println!("\nTest 2: Receive case only (should timeout or hang)");
    channel.try_push(99).unwrap(); // Add data
    
    let mut select2 = Select::new();
    select2.receive(2, channel.clone());
    
    match select2.execute() {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}
