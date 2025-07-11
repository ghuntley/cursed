// Test error handling compilation
// Test yikes (error value expression)
sus error_msg := yikes "This is a test error"
vibez.spill("Created error:", error_msg)

// Test shook (error propagation)
slay risky_function() tea {
    damn yikes "Function failed"
}

slay test_shook() tea {
    sus result := shook risky_function()
    damn result
}

// Test with main function
slay main() {
    vibez.spill("Testing error handling compilation")
    
    // Test error value creation
    sus my_error := yikes "Sample error message"
    vibez.spill("Error created successfully")
    
    // Test error propagation
    sus propagated := shook test_shook()
    vibez.spill("Error propagation test completed")
}
