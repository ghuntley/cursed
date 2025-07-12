// Minimal error handling test

vibez.spill("Starting error handling test...")

// Test yikes error creation
yikes test_error := "Basic error message"
vibez.spill("Error created:", test_error)

// Test fam error recovery
fam {
    vibez.spill("This should print")
} sus catch_error {
    vibez.spill("Error caught:", catch_error)
}

vibez.spill("Error handling test complete")
