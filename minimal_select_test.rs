// Minimal test to identify hanging point

use std::sync::Arc;

use cursed::runtime::channels::buffer::{RingBuffer, ChannelBuffer};
use cursed::runtime::channels::select::Select;

fn main() {
    println!("Step 1: Creating channel");
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    
    println!("Step 2: Creating select");
    let mut select = Select::new();
    
    println!("Step 3: Adding send case");
    select.send(1, channel.clone(), 42);
    
    println!("Step 4: Adding default case to prevent hanging");
    select.default_case();
    
    println!("Step 5: Executing select");
    match select.execute() {
        Ok(result) => {
            println!("Step 6: Select result: {:?}", result);
        }
        Err(e) => {
            println!("Step 6: Select error: {:?}", e);
        }
    }
    
    println!("Step 7: Done");
}
