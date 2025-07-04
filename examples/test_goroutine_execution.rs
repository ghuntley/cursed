use cursed::runtime::goroutine::{initialize_global_scheduler, stan, GoroutineState, get_global_scheduler};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn test_basic_goroutine_execution() {
    // Initialize the global scheduler
    initialize_global_scheduler().expect("Failed to initialize scheduler");
    
    // Create a shared value to test if the goroutine actually executes
    let executed = Arc::new(Mutex::new(false));
    let executed_clone = executed.clone();
    
    // Spawn a goroutine that sets the shared value to true
    let goroutine_id = stan(move || {
        println!("Goroutine is executing!");
        *executed_clone.lock().unwrap() = true;
        println!("Goroutine execution completed!");
    }).expect("Failed to spawn goroutine");
    
    println!("Spawned goroutine with ID: {}", goroutine_id);
    
    // Give the scheduler some time to execute the goroutine
    thread::sleep(Duration::from_millis(100));
    
    // Check if the goroutine actually executed
    let was_executed = *executed.lock().unwrap();
    println!("Goroutine executed: {}", was_executed);
    
    // If the implementation works, was_executed should be true
    if was_executed {
        println!("✅ SUCCESS: Goroutine execution is working!");
    } else {
        println!("❌ FAILURE: Goroutine did not execute");
    }
    
    // Check goroutine state if we can access the scheduler
    if let Some(scheduler) = get_global_scheduler() {
        println!("✅ Global scheduler is available");
        // Additional state checking could be done here
    } else {
        println!("❌ Global scheduler not available");
    }
}

fn main() {
    println!("🧪 Testing CURSED Goroutine Execution Implementation");
    println!("==================================================");
    
    test_basic_goroutine_execution();
    
    println!("==================================================");
    println!("🏁 Test completed");
}
