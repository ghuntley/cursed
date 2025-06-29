// Simple debug for select hanging

use std::sync::Arc;

use cursed::runtime::channels::buffer::{RingBuffer, ChannelBuffer};
use cursed::runtime::channels::select::Select;

fn main() {
    println!("Debugging simple select...");
    
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    
    println!("Creating select...");
    let mut select = Select::new();
    
    println!("Adding send case...");
    select.send(1, channel.clone(), 42);
    
    println!("About to execute select...");
    
    // Execute select with some debug info
    match select.execute() {
        Ok(result) => {
            println!("✓ Select executed successfully: {:?}", result);
        }
        Err(e) => {
            println!("✗ Select failed: {:?}", e);
        }
    }
    
    println!("Select completed!");
}
