use cursed::{CursedExecutionEngine, CursedValue, RuntimeMemoryManager};

fn main() {
    println!("Testing type exports...");
    
    // Test that types can be constructed
    match CursedExecutionEngine::new() {
        Ok(_engine) => println!("✓ CursedExecutionEngine: Available"),
        Err(e) => println!("✗ CursedExecutionEngine: {}", e),
    }
    
    // Test CursedValue variants
    let _value = CursedValue::Integer(42);
    println!("✓ CursedValue: Available");
    
    println!("✓ All types exported successfully!");
}
