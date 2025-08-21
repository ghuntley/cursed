// Test program for real LLVM backend
// Simple CURSED program to validate LLVM IR generation

// Variable declarations
sus x drip = 42
sus name tea = "CURSED Developer"
sus active lit = based

// Function declaration
slay greet() {
    vibez.spill("Hello from CURSED!")
    vibez.spill(name)
    vibez.spill("X is:", x)
}

// Math function
slay add(a drip, b drip) drip {
    damn a + b
}

// Main logic
slay main() drip {
    greet()
    
    sus result drip = add(10, 20)
    vibez.spill("Result:", result)
    
    // Control flow
    ready (result > 25) {
        vibez.spill("Result is large!")
    } otherwise {
        vibez.spill("Result is small!")
    }
    
    // Loop example
    sus i drip = 0
    bestie (i < 3) {
        vibez.spill("Count:", i)
        i = i + 1
    }
    
    damn 0
}
