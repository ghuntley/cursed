use cursed::runtime::{Runtime, RuntimeStats};

fn test_runtime() {
    println!("Testing CURSED runtime initialization...");
    
    match Runtime::new() {
        Ok(runtime) => {
            println!("✓ Runtime initialized successfully");
            println!("Runtime stats: {:?}", runtime.get_stats());
        }
        Err(e) => println!("✗ Runtime initialization failed: {:?}", e),
    }
}

fn test_memory_manager() {
    println!("\nTesting memory manager...");
    
    match cursed::runtime::initialize_memory_manager() {
        Ok(_) => {
            println!("✓ Memory manager initialized successfully");
            
            // Test memory allocation
            match cursed::runtime::allocate(64) {
                Ok(handle) => {
                    println!("✓ Memory allocation successful: {:?}", handle);
                }
                Err(e) => println!("✗ Memory allocation failed: {:?}", e),
            }
        }
        Err(e) => println!("✗ Memory manager initialization failed: {:?}", e),
    }
}

fn main() {
    test_runtime();
    test_memory_manager();
}
