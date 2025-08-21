yeet "vibez"

// Test built-in macros
slay main() {
    vibez.spill("=== Testing Macro System ===")
    
    // Test debug_print! macro
    sus debug_enabled lit = based
    debug_print!("This is a debug message")
    debug_print!("Debug value:", 42)
    
    // Test assert! macro
    assert!(2 + 2 == 4, "Math should work!")
    assert!(based == based, "Boolean comparison")
    
    // Test vec! macro for creating arrays
    sus numbers []drip = vec![1, 2, 3, 4, 5]
    vibez.spill("Vector created:", numbers)
    
    // Test format! macro
    sus name tea = "CURSED"
    sus version drip = 1
    sus message tea = format!("Welcome to {} v{}", name, version)
    vibez.spill("Formatted message:", message)
    
    // Test derive! macro (simulated)
    vibez.spill("Derive macro would generate JSON serialization code")
    
    vibez.spill("=== Macro system test completed! ===")
}
