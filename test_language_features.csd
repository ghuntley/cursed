// Test core language features that work now

// Variable declarations and basic operations
sus x drip = 42
sus y drip = 24  
sus result drip = x + y

// Basic output
vibez.spill("Basic arithmetic:", result)

// Simple function
slay multiply(a drip, b drip) drip {
    damn a * b
}

sus product drip = multiply(6, 7)
vibez.spill("Multiplication result:", product)

// Arrays
sus numbers []drip = [1, 2, 3]
vibez.spill("First number:", numbers[0])

// Control flow
ready (result > 50) {
    vibez.spill("Result is greater than 50")
} otherwise {
    vibez.spill("Result is 50 or less")
}

// Loop
sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration:", i)
    i = i + 1
}

vibez.spill("=== Core Features Working ===")
