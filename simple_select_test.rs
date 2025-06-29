// Simple test for select operations

use std::sync::Arc;

use cursed::runtime::channels::buffer::{RingBuffer, ChannelBuffer};
use cursed::runtime::channels::select::{Select, SelectResult};

fn main() {
    println!("Testing basic select functionality...");
    
    // Test 1: Send operation
    println!("Test 1: Send operation");
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    
    let mut select = Select::new();
    select.send(1, channel.clone(), 42);
    
    let result = select.execute();
    match result {
        Ok(SelectResult::SendCompleted(case_index)) => {
            println!("✓ Send completed on case {}", case_index);
        }
        Ok(other) => {
            println!("✗ Unexpected result: {:?}", other);
        }
        Err(e) => {
            println!("✗ Error: {:?}", e);
        }
    }
    
    // Test 2: Default case only
    println!("\nTest 2: Default case only");
    let mut select2 = Select::new();
    select2.default_case();
    
    let result2 = select2.execute();
    match result2 {
        Ok(SelectResult::DefaultExecuted) => {
            println!("✓ Default case executed");
        }
        Ok(other) => {
            println!("✗ Unexpected result: {:?}", other);
        }
        Err(e) => {
            println!("✗ Error: {:?}", e);
        }
    }
    
    println!("\nBasic tests completed!");
}
