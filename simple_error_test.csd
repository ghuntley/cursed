// Simple error handling test without testz dependency
vibez.spill("Testing basic error handling")

// Test 1: Basic yikes error creation
yikes test_error := "Basic error message"
vibez.spill("Created error:", test_error)

// Test 2: Simple function with error return
slay test_divide(a normie, b normie) normie {
    vibe_check b == 0 {
        yikes divide_error := "Cannot divide by zero"
        damn 0
    }
    damn a / b
}

sus result := test_divide(10, 2)
vibez.spill("Division result:", result)

// Test 3: Error propagation
slay test_propagation() yikes {
    yikes prop_error := "Propagated error"
    damn prop_error shook
}

sus propagated := test_propagation()
vibez.spill("Propagated error:", propagated)

// Test 4: Error recovery
sus recovery_worked lit := cap
fam {
    yikes panic_error := "Test panic"
    vibez.spill("This should not print if recovery works")
} sus caught {
    vibez.spill("Caught error:", caught)
    recovery_worked = based
}

vibez.spill("Recovery worked:", recovery_worked)

vibez.spill("All basic error handling tests completed")
