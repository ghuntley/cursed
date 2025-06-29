// Direct test of the vibez.spill() function implementation
use cursed::stdlib::vibez::print::spill;
use cursed::runtime::value::Value;

fn main() {
    println!("🧪 Testing vibez.spill() implementation directly");
    
    let test_values = vec![
        Value::String("Hello from CURSED language! 💀".to_string()),
        Value::Integer(42),
        Value::Bool(true),
        Value::Null,
    ];
    
    match spill(&test_values) {
        Ok(()) => println!("✅ spill() function works!"),
        Err(e) => println!("❌ spill() function failed: {}", e),
    }
}
