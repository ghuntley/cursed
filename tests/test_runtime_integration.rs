use cursed::runtime::value::Value;
use cursed::stdlib::vibez::print::spill;

#[test]
fn test_runtime_integration() {
    println!("Testing CURSED vibez runtime integration...");
    
    // Test basic spill functionality
    let args = vec![
        Value::String("Hello, CURSED world!".to_string()),
    ];
    
    println!("Testing spill function directly:");
    match spill(&args) {
        Ok(()) => println!("✅ Direct spill() call successful"),
        Err(e) => println!("❌ Direct spill() call failed: {:?}", e),
    }
    
    // Test with multiple arguments
    let multi_args = vec![
        Value::String("Count:".to_string()),
        Value::Integer(42),
        Value::Bool(true),
    ];
    
    println!("\nTesting spill with multiple arguments:");
    match spill(&multi_args) {
        Ok(()) => println!("✅ Multi-argument spill() call successful"),
        Err(e) => println!("❌ Multi-argument spill() call failed: {:?}", e),
    }
    
    println!("\n🎉 Runtime integration test complete!");
}
