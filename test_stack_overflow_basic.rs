// Test basic stack overflow detection
use std::sync::Arc;
use cursed::runtime::stack::{RuntimeStack, StackConfig};

fn main() {
    println!("Testing stack overflow detection...");
    
    // Create a stack manager with overflow detection enabled
    let mut config = StackConfig::default();
    config.enable_overflow_detection = true;
    config.overflow_detection_threshold = 1024; // Small threshold for testing
    
    let stack_manager = Arc::new(RuntimeStack::with_config(config));
    
    // Allocate a small stack
    let stack_id = stack_manager.allocate_stack(Some(4096)).unwrap();
    println!("Allocated stack with ID: {}", stack_id);
    
    // Get initial stack info
    let initial_info = stack_manager.get_stack_info(stack_id).unwrap();
    println!("Initial stack info: {:?}", initial_info);
    
    // Simulate stack usage by updating stack pointer
    let base_ptr = stack_manager.get_stack_pointer(stack_id).unwrap();
    println!("Base stack pointer: {:?}", base_ptr);
    
    // Simulate stack growth by moving pointer toward overflow
    let near_overflow_ptr = unsafe { base_ptr.sub(3500) }; // Near 4096 - 1024 threshold
    stack_manager.update_stack_pointer(stack_id, near_overflow_ptr).unwrap();
    
    // Check for overflow
    match stack_manager.check_stack_overflow(stack_id) {
        Ok(Some(overflow_error)) => {
            println!("Stack overflow detected: {}", overflow_error);
            println!("Recovery suggested: {}", overflow_error.recovery_suggested);
            
            // Try to recover
            match stack_manager.recover_from_overflow(&overflow_error) {
                Ok(true) => println!("Successfully recovered from overflow"),
                Ok(false) => println!("Failed to recover from overflow"),
                Err(e) => println!("Error during recovery: {}", e),
            }
        }
        Ok(None) => println!("No overflow detected"),
        Err(e) => println!("Error checking for overflow: {}", e),
    }
    
    // Test monitoring all stacks
    match stack_manager.monitor_stack_overflows() {
        Ok(overflows) => {
            println!("Found {} overflow(s) during monitoring", overflows.len());
            for overflow in overflows {
                println!("  - {}", overflow);
            }
        }
        Err(e) => println!("Error during monitoring: {}", e),
    }
    
    // Get final statistics
    let stats = stack_manager.get_stats();
    println!("Final stack statistics: {:?}", stats);
    
    // Clean up
    stack_manager.deallocate_stack(stack_id).unwrap();
    println!("Stack deallocated successfully");
    
    println!("Stack overflow detection test completed!");
}
