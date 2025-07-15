use std::sync::Arc;
use cursed::runtime::{
    Runtime, RuntimeConfig, GoroutineSchedulerWrapper, SchedulerConfig,
    create_runtime_with_scheduler, create_runtime_with_default_scheduler
};

fn main() {
    println!("Testing scheduler integration...");
    
    // Test 1: Create runtime with default scheduler
    match create_runtime_with_default_scheduler() {
        Ok(runtime) => {
            println!("✅ Successfully created runtime with default scheduler");
            
            // Test spawning a goroutine
            match runtime.spawn_goroutine(|| {
                println!("Hello from goroutine!");
            }) {
                Ok(goroutine_id) => {
                    println!("✅ Successfully spawned goroutine {}", goroutine_id);
                }
                Err(e) => {
                    println!("❌ Failed to spawn goroutine: {}", e);
                }
            }
            
            // Test getting stats
            match runtime.get_stats() {
                Ok(stats) => {
                    println!("✅ Runtime stats: {} active goroutines, {} total created", 
                             stats.active_goroutines, stats.total_goroutines_created);
                }
                Err(e) => {
                    println!("❌ Failed to get stats: {}", e);
                }
            }
            
            // Test shutdown
            match runtime.shutdown() {
                Ok(_) => {
                    println!("✅ Runtime shutdown successfully");
                }
                Err(e) => {
                    println!("❌ Failed to shutdown runtime: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create runtime: {}", e);
        }
    }
    
    // Test 2: Create runtime with custom scheduler config
    let runtime_config = RuntimeConfig::development();
    let scheduler_config = SchedulerConfig {
        num_workers: 4,
        enable_debugging: true,
        ..Default::default()
    };
    
    match create_runtime_with_scheduler(runtime_config, scheduler_config) {
        Ok(runtime) => {
            println!("✅ Successfully created runtime with custom scheduler");
            
            // Test basic operations
            if runtime.is_running() {
                println!("✅ Runtime is running");
            }
            
            // Clean shutdown
            let _ = runtime.shutdown();
        }
        Err(e) => {
            println!("❌ Failed to create runtime with custom scheduler: {}", e);
        }
    }
    
    println!("Scheduler integration test completed!");
}
