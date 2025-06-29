/// Test minimal runtime functionality
use std::sync::Arc;

// Import from source files directly
#[path = "src/runtime/memory.rs"]
mod memory;

#[path = "src/runtime/gc.rs"]
mod gc;

#[path = "src/runtime/stack.rs"]
mod stack;

#[path = "src/error_types.rs"]
mod error;

#[path = "src/memory/mod.rs"]
mod memory_base;

use memory::{MemoryManager, MemoryConfig};
use gc::{GarbageCollector, GcConfig};
use stack::RuntimeStack;

fn main() {
    println!("Testing Phase 2B Runtime System...");
    
    // Test 1: GarbageCollector initialization
    println!("1. Testing GarbageCollector initialization...");
    let stack_manager = Arc::new(RuntimeStack::new());
    let gc_config = GcConfig::default();
    
    match GarbageCollector::new(gc_config, stack_manager.clone()) {
        Ok(gc) => println!("   ✓ GarbageCollector initialized successfully"),
        Err(e) => println!("   ✗ GarbageCollector initialization failed: {}", e),
    }
    
    // Test 2: MemoryManager initialization
    println!("2. Testing MemoryManager initialization...");
    let memory_config = MemoryConfig::default();
    
    match MemoryManager::new(memory_config, stack_manager.clone()) {
        Ok(_manager) => println!("   ✓ MemoryManager initialized successfully"),
        Err(e) => println!("   ✗ MemoryManager initialization failed: {}", e),
    }
    
    // Test 3: Runtime integration
    println!("3. Testing runtime integration...");
    // This would test initialization of both together
    let gc_config = GcConfig::default();
    let memory_config = MemoryConfig::default();
    
    match (GarbageCollector::new(gc_config, stack_manager.clone()), 
           MemoryManager::new(memory_config, stack_manager.clone())) {
        (Ok(_gc), Ok(_manager)) => println!("   ✓ Runtime integration successful"),
        (Err(e), _) => println!("   ✗ GC integration failed: {}", e),
        (_, Err(e)) => println!("   ✗ Memory manager integration failed: {}", e),
    }
    
    println!("Phase 2B Runtime System Test Complete");
}
