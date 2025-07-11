// Simple error handling test
slay main() {
    vibez.spill("Testing simple error handling")
    
    // Test yikes (error value creation)
    sus my_error := yikes "Test error message"
    vibez.spill("Created error object")
    
    // Test shook (error propagation)
    // sus propagated := shook my_error
    // vibez.spill("Propagated error")
    
    vibez.spill("Error handling test completed")
}
