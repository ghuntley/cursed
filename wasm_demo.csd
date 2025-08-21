// CURSED WebAssembly Demo Program
// Demonstrates core language features compiled to WASM
yeet "vibez"

// Main entry point for WASM
slay main() drip {
    vibez.spill("🚀 Welcome to CURSED compiled to WebAssembly!")
    vibez.spill("This demonstrates the CURSED language running in WASM")
    
    // Demonstrate basic data types
    demo_data_types()
    
    // Demonstrate control flow
    demo_control_flow()
    
    // Demonstrate functions
    demo_functions()
    
    // Demonstrate arithmetic
    demo_arithmetic()
    
    vibez.spill("✅ CURSED WASM demo completed successfully!")
    damn 0
}

// Demonstrate CURSED data types
slay demo_data_types() {
    vibez.spill("")
    vibez.spill("📊 CURSED Data Types Demo:")
    
    // Boolean types with slang names
    sus is_based lit = based
    sus is_cringe lit = cringe
    vibez.spill("  Boolean: based =", is_based, ", cringe =", is_cringe)
    
    // Integer type (drip)
    sus vibe_level drip = 1337
    vibez.spill("  Integer (drip):", vibe_level)
    
    // String type (tea) 
    sus greeting tea = "Hello from CURSED WASM!"
    vibez.spill("  String (tea):", greeting)
    
    // Array demonstration
    // sus numbers []drip = [1, 2, 3, 4, 5]
    // vibez.spill("  Array length:", numbers.len)
}

// Demonstrate control flow structures
slay demo_control_flow() {
    vibez.spill("")
    vibez.spill("🔀 Control Flow Demo:")
    
    // If-else with CURSED keywords
    sus condition lit = based
    ready (condition) {
        vibez.spill("  Condition is based! ✅")
    } otherwise {
        vibez.spill("  This shouldn't print")
    }
    
    // While loop with bestie keyword
    vibez.spill("  Loop demo:")
    sus counter drip = 1
    bestie (counter <= 3) {
        vibez.spill("    Iteration", counter)
        counter = counter + 1
    }
}

// Demonstrate function calls and return values
slay demo_functions() {
    vibez.spill("")
    vibez.spill("⚡ Function Demo:")
    
    sus result1 drip = add_two_numbers(15, 27)
    vibez.spill("  15 + 27 =", result1)
    
    sus result2 drip = multiply_numbers(6, 7) 
    vibez.spill("  6 * 7 =", result2)
    
    sus result3 drip = fibonacci(8)
    vibez.spill("  fibonacci(8) =", result3)
    
    sus result4 lit = is_even(42)
    vibez.spill("  is_even(42) =", result4)
}

// Demonstrate arithmetic operations
slay demo_arithmetic() {
    vibez.spill("")
    vibez.spill("🧮 Arithmetic Demo:")
    
    sus a drip = 100
    sus b drip = 25
    
    vibez.spill("  a =", a, ", b =", b)
    vibez.spill("  a + b =", a + b)
    vibez.spill("  a - b =", a - b)
    vibez.spill("  a * b =", a * b)
    vibez.spill("  a / b =", a / b)
    vibez.spill("  a % b =", a % b)
}

// Helper functions for demonstrations

slay add_two_numbers(x drip, y drip) drip {
    damn x + y
}

slay multiply_numbers(x drip, y drip) drip {
    damn x * y
}

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay is_even(n drip) lit {
    damn (n % 2) == 0
}

// Export additional functions for JavaScript interop
export slay get_version() drip {
    damn 1
}

export slay calculate(a drip, b drip) drip {
    damn a + b
}

export slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}
