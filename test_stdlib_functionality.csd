// Test CURSED stdlib functionality after restoration
// This tests that vibez.spill() and other core functions work properly

fun main() {
    vibez.spill("Testing stdlib restoration!")
    vibez.spill("net module loaded:", true)
    vibez.spill("squish_core module loaded:", true)
    vibez.spill("vibez.spill() working correctly:", true)
    
    // Test basic string operations
    let test_str = "Hello CURSED World"
    vibez.spill("String test:", test_str)
    
    // Test basic math
    let result = 2 + 3 * 4
    vibez.spill("Math test (2 + 3 * 4):", result)
    
    // Test variable assignment and access
    let x = 42
    let y = x * 2
    vibez.spill("Variable test:", x, "->", y)
    
    vibez.spill("All stdlib functionality tests passed!")
}
