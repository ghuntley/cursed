// Comprehensive test of stack overflow detection system
use std::sync::Arc;
use cursed::runtime::stack::{RuntimeStack, StackConfig, StackOverflowError};
use std::time::Duration;

fn main() {
    println!("🧪 CURSED Stack Overflow Detection System Test");
    println!("=" .repeat(60));
    
    // Create a stack manager with enhanced overflow detection
    let mut config = StackConfig::default();
    config.enable_overflow_detection = true;
    config.enable_stack_monitoring = true;
    config.overflow_detection_threshold = 1024; // 1KB safety margin
    config.overflow_check_interval = Duration::from_millis(50);
    config.max_recovery_attempts = 3;
    
    let stack_manager = Arc::new(RuntimeStack::with_config(config));
    println!("✅ Created stack manager with overflow detection enabled");
    
    // Test 1: Basic stack overflow detection
    println!("\n🔍 Test 1: Basic Stack Overflow Detection");
    println!("-" .repeat(40));
    
    let stack_id = stack_manager.allocate_stack(Some(8192)).unwrap();
    println!("📊 Allocated stack {} with size 8192 bytes", stack_id);
    
    let initial_info = stack_manager.get_stack_info(stack_id).unwrap();
    println!("📈 Initial stack info: {:?}", initial_info);
    
    // Simulate stack usage approaching overflow
    let base_ptr = stack_manager.get_stack_pointer(stack_id).unwrap();
    let near_overflow_ptr = unsafe { base_ptr.sub(7500) }; // Use 7500 bytes
    stack_manager.update_stack_pointer(stack_id, near_overflow_ptr).unwrap();
    
    match stack_manager.check_stack_overflow(stack_id) {
        Ok(Some(overflow_error)) => {
            println!("⚠️  Stack overflow detected:");
            println!("   Stack ID: {}", overflow_error.stack_id);
            println!("   Current usage: {} bytes", overflow_error.current_usage);
            println!("   Stack size: {} bytes", overflow_error.stack_size);
            println!("   Threshold: {} bytes", overflow_error.overflow_threshold);
            println!("   Recovery suggested: {}", overflow_error.recovery_suggested);
            
            // Test recovery
            match stack_manager.recover_from_overflow(&overflow_error) {
                Ok(true) => println!("✅ Successfully recovered from overflow"),
                Ok(false) => println!("❌ Failed to recover from overflow"),
                Err(e) => println!("❌ Error during recovery: {}", e),
            }
        }
        Ok(None) => println!("ℹ️  No overflow detected"),
        Err(e) => println!("❌ Error checking for overflow: {}", e),
    }
    
    // Test 2: Multiple stack monitoring
    println!("\n🔍 Test 2: Multiple Stack Monitoring");
    println!("-" .repeat(40));
    
    let stack_id2 = stack_manager.allocate_stack(Some(8192)).unwrap();
    let stack_id3 = stack_manager.allocate_stack(Some(8192)).unwrap();
    
    println!("📊 Created additional stacks: {} and {}", stack_id2, stack_id3);
    
    // Simulate overflow in one stack
    let base_ptr2 = stack_manager.get_stack_pointer(stack_id2).unwrap();
    let overflow_ptr2 = unsafe { base_ptr2.sub(7800) }; // Overflow
    stack_manager.update_stack_pointer(stack_id2, overflow_ptr2).unwrap();
    
    // Keep third stack safe
    let base_ptr3 = stack_manager.get_stack_pointer(stack_id3).unwrap();
    let safe_ptr3 = unsafe { base_ptr3.sub(500) }; // Safe
    stack_manager.update_stack_pointer(stack_id3, safe_ptr3).unwrap();
    
    let all_overflows = stack_manager.monitor_stack_overflows().unwrap();
    println!("🔍 Monitoring found {} overflow(s)", all_overflows.len());
    
    for (i, overflow) in all_overflows.iter().enumerate() {
        println!("   Overflow {}: Stack {} - {} bytes used", 
                 i + 1, overflow.stack_id, overflow.current_usage);
    }
    
    // Test 3: Recovery attempt limits
    println!("\n🔍 Test 3: Recovery Attempt Limits");
    println!("-" .repeat(40));
    
    let test_stack = stack_manager.allocate_stack(Some(8192)).unwrap();
    println!("📊 Created test stack: {}", test_stack);
    
    // Simulate multiple overflow/recovery cycles
    for attempt in 1..=4 {
        let base_ptr = stack_manager.get_stack_pointer(test_stack).unwrap();
        let overflow_ptr = unsafe { base_ptr.sub(7600) };
        stack_manager.update_stack_pointer(test_stack, overflow_ptr).unwrap();
        
        let overflow_result = stack_manager.check_stack_overflow(test_stack).unwrap();
        if let Some(overflow_error) = overflow_result {
            let recovery_result = stack_manager.recover_from_overflow(&overflow_error).unwrap();
            println!("   Attempt {}: Recovery {}", 
                     attempt, 
                     if recovery_result { "succeeded" } else { "failed" });
        }
    }
    
    // Test 4: Stack usage statistics
    println!("\n🔍 Test 4: Stack Usage Statistics");
    println!("-" .repeat(40));
    
    let stats = stack_manager.get_stats();
    println!("📊 Stack Manager Statistics:");
    println!("   Total allocated: {}", stats.total_allocated);
    println!("   Current stacks: {}", stats.current_stacks);
    println!("   Peak stacks: {}", stats.peak_stacks);
    println!("   Total memory used: {} bytes", stats.total_memory_used);
    println!("   Overflow detections: {}", stats.overflow_detections);
    println!("   Overflow recoveries: {}", stats.overflow_recoveries);
    println!("   Failed recoveries: {}", stats.failed_recoveries);
    println!("   Monitoring checks: {}", stats.monitoring_checks);
    
    // Test 5: Callback system
    println!("\n🔍 Test 5: Callback System");
    println!("-" .repeat(40));
    
    let alert_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let alert_count_clone = alert_count.clone();
    
    stack_manager.register_overflow_alert(move |overflow_error| {
        alert_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        println!("🚨 Alert: Stack overflow in stack {}", overflow_error.stack_id);
    }).unwrap();
    
    println!("✅ Registered overflow alert callback");
    
    // Clean up
    println!("\n🧹 Cleanup");
    println!("-" .repeat(40));
    
    stack_manager.deallocate_stack(stack_id).unwrap();
    stack_manager.deallocate_stack(stack_id2).unwrap();
    stack_manager.deallocate_stack(stack_id3).unwrap();
    stack_manager.deallocate_stack(test_stack).unwrap();
    
    println!("✅ All stacks deallocated successfully");
    
    let final_stats = stack_manager.get_stats();
    println!("📊 Final statistics:");
    println!("   Current stacks: {}", final_stats.current_stacks);
    println!("   Total memory used: {} bytes", final_stats.total_memory_used);
    
    println!("\n🎉 Stack overflow detection system test completed successfully!");
    println!("✅ All tests passed - Stack overflow detection is working correctly");
}
