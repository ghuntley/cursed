// Simple test for advanced CURSED features
yeet "testz"

// Basic pattern matching (simplified syntax)
slay pattern_demo() {
    sus value drip = 42
    
    // Basic pattern matching that should work
    ready (value > 0) {
        vibez.spill("Positive value:", value)
    } otherwise {
        vibez.spill("Non-positive value")
    }
    
    // Array operations
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus length drip = len(numbers)
    vibez.spill("Array length:", length)
    
    // Function calls
    sus result drip = add_numbers(5, 7)
    vibez.spill("Function result:", result)
}

// Simple function definition
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

// Main function
slay main() {
    vibez.spill("=== Simple Advanced Features Test ===")
    pattern_demo()
    vibez.spill("=== Test Complete ===")
}
