// Test Stage 1: Simple Module System
// Testing vibe (import) and yeet (export)

// Import standard library modules (if available)
vibe "stdlib::io"
vibe "stdlib::math"

// Test basic I/O operations
slay test_io() {
    sus message: lit = "Testing I/O operations"
    print(message)
    
    // Test if print variations work
    println("This should have a newline")
}

// Test basic math operations (if math module available)
slay test_math() {
    sus a: normie = 10
    sus b: normie = 3
    
    sus sum: normie = a + b
    sus diff: normie = a - b
    sus prod: normie = a * b
    
    print(sum)
    print(diff) 
    print(prod)
}

// Main execution
test_io()
test_math()
