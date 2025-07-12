// Minimal error handling test
vibez.spill("Testing basic error handling")

// Test 1: Basic yikes error creation
yikes test_error := "Basic error message"
vibez.spill("Created error:", test_error)

// Test 2: Simple function
slay test_function() {
    yikes func_error := "Function error"
    vibez.spill("Function error:", func_error)
}

test_function()

// Test 3: Error recovery
fam {
    yikes panic_error := "Test panic"
    vibez.spill("In protected block")
} sus caught {
    vibez.spill("Caught error:", caught)
}

vibez.spill("Test completed")
