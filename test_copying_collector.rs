/// Test implementation for the copying garbage collector
use std::sync::Arc;
use cursed::memory::{ObjectRegistry, CopyingCollector, ObjectMetadata, ObjectIdGenerator};

fn main() {
    println!("Testing Copying Garbage Collector Implementation");
    
    // Create object registry
    let registry = Arc::new(ObjectRegistry::new());
    
    // Create copying collector
    match CopyingCollector::new(registry.clone()) {
        Ok(collector) => {
            println!("✓ Successfully created copying collector");
            
            // Test object registration
            let id_gen = ObjectIdGenerator::new();
            let obj_id = id_gen.next();
            let metadata = ObjectMetadata::new(obj_id, 64, "TestObject".to_string());
            
            if let Err(e) = registry.register(metadata) {
                println!("✗ Failed to register object: {}", e);
                return;
            }
            println!("✓ Successfully registered test object: {}", obj_id);
            
            // Test object size retrieval
            match collector.get_object_size(obj_id) {
                Ok(Some(size)) => println!("✓ Retrieved object size: {} bytes", size),
                Ok(None) => println!("✗ Object not found"),
                Err(e) => println!("✗ Error retrieving object size: {}", e),
            }
            
            // Test forwarding functionality
            match collector.is_object_forwarded(obj_id) {
                Ok(forwarded) => println!("✓ Object forwarding status: {}", forwarded),
                Err(e) => println!("✗ Error checking forwarding: {}", e),
            }
            
            // Test space management
            match collector.should_collect() {
                Ok(should) => println!("✓ Collection needed: {}", should),
                Err(e) => println!("✗ Error checking collection need: {}", e),
            }
            
            match collector.available_space() {
                Ok(space) => println!("✓ Available space: {} bytes", space),
                Err(e) => println!("✗ Error getting available space: {}", e),
            }
            
            // Test efficiency estimation
            match collector.estimate_copying_efficiency() {
                Ok(efficiency) => println!("✓ Copying efficiency: {:.2}", efficiency),
                Err(e) => println!("✗ Error estimating efficiency: {}", e),
            }
            
            // Test allocation
            match collector.allocate(128, 8) {
                Ok(Some(ptr)) => println!("✓ Successfully allocated 128 bytes at: {:?}", ptr),
                Ok(None) => println!("? Allocation returned None (might need collection)"),
                Err(e) => println!("✗ Error during allocation: {}", e),
            }
            
            println!("\n✓ All copying collector basic functionality tests completed!");
        }
        Err(e) => {
            println!("✗ Failed to create copying collector: {}", e);
        }
    }
}
