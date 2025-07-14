# Simple test to verify error handling keywords work
yikes test_error := "Simple error message"
vibez.spill("Error created successfully!")

# Test error propagation
slay create_error() {
    yikes propagated_error := "Propagated error"
    shook propagated_error
}

# Test error recovery
fam {
    create_error()
} catch caught_error {
    vibez.spill("Caught: " + caught_error)
}

vibez.spill("Error handling test completed")
