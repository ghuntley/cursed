// End-to-end CURSED compilation test
// This program tests the complete LLVM backend implementation

// Basic variable declarations
sus message tea = "Hello from CURSED LLVM Backend!"
sus count drip = 10
sus active lit = based

// Simple function
slay greet(name tea) {
    vibez.spill("Greetings", name)
    vibez.spill("Message:", message)
}

// Math function with return value
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

// Main program logic
slay main() drip {
    vibez.spill("🚀 CURSED LLVM Backend Test")
    
    greet("Developer")
    
    // Test arithmetic
    sus result drip = 5 * 8 + 2
    vibez.spill("Arithmetic result:", result)
    
    // Test conditionals
    ready (result > 40) {
        vibez.spill("Result is large!")
    } otherwise {
        vibez.spill("Result is small!")
    }
    
    // Test loops
    sus i drip = 0
    bestie (i < 3) {
        vibez.spill("Loop iteration:", i)
        i = i + 1
    }
    
    // Test recursive function
    sus fib_result drip = fibonacci(5)
    vibez.spill("Fibonacci(5):", fib_result)
    
    vibez.spill("✅ All tests completed successfully!")
    damn 0
}
