// Debug select step by step

use std::sync::Arc;
use std::any::TypeId;

use cursed::runtime::channels::buffer::{RingBuffer, ChannelBuffer};
use cursed::runtime::channels::select::{Select, SelectResult, ChannelWrapper, TypedChannelOps, ChannelOps};

fn main() {
    println!("Debugging select step by step...");
    
    // Create channel
    let channel: Arc<dyn ChannelBuffer<i32>> = Arc::new(RingBuffer::new(3));
    
    // Create wrapper manually to test
    let wrapper = ChannelWrapper {
        type_id: TypeId::of::<i32>(),
        ops: Box::new(TypedChannelOps::new(channel.clone())),
    };
    
    println!("Channel ready to send: {}", wrapper.ops.can_send());
    
    // Test clone_value
    let test_value = Box::new(42i32) as Box<dyn std::any::Any + Send>;
    let cloned = wrapper.ops.clone_value(test_value.as_ref());
    
    match cloned {
        Some(cloned_val) => {
            println!("✓ Clone successful");
            
            // Test try_send
            match wrapper.ops.try_send(cloned_val) {
                Ok(()) => println!("✓ Send successful"),
                Err((_, err)) => println!("✗ Send failed: {:?}", err),
            }
        }
        None => {
            println!("✗ Clone failed - type mismatch or other issue");
        }
    }
    
    println!("Debug completed!");
}
