// Hello World in CURSED compiled to WebAssembly
// This demonstrates basic WASM output with browser integration

yeet "vibez"

// Main function - entry point for WASM
slay main_character() {
    vibez.spill("Hello from CURSED WebAssembly! 🚀")
    vibez.spill("This is running in your browser via WASM")
    
    // Demo basic arithmetic
    sus x drip = 42
    sus y drip = 13
    sus result drip = x + y
    
    vibez.spill("42 + 13 =", result)
    
    // Demo string operations
    sus greeting tea = "Welcome to"
    sus language tea = "CURSED"
    vibez.spill(greeting, language, "programming!")
    
    damn 0 // Return success code
}

// Additional exportable function for JS interop
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

// Function to demonstrate CURSED's unique syntax in WASM
slay cursed_demo() {
    vibez.spill("Demonstrating CURSED syntax in WebAssembly:")
    
    // Variables with slang types
    sus is_based lit = based  // boolean true
    sus not_cringe lit = !cringe  // boolean false
    sus vibe_level drip = 9001  // integer
    sus chaos_factor tea = "maximum"  // string
    
    ready (is_based) {
        vibez.spill("This code is absolutely", chaos_factor, "chaos level:", vibe_level)
    } otherwise {
        vibez.spill("Something went wrong in the matrix")
    }
    
    // Loop demo
    sus i drip = 1
    bestie (i <= 3) {
        vibez.spill("Iteration number:", i)
        i = i + 1
    }
}

// Export functions for JavaScript interop
// These will be available as cursedModule.add_numbers(x, y) etc.
export slay get_version() tea {
    damn "CURSED WebAssembly v1.0.0"
}

export slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}
