// Comprehensive error handling test with proper CURSED syntax

// Function that might fail
slay divide_safely(x normie, y normie) tea {
    // Using proper CURSED variable declarations
    sus result := x / y
    
    // Return a string representation of the result
    damn "Result calculated"
}

// Function that demonstrates error creation
slay test_error_creation() tea {
    // Create an error value
    sus error := yikes "Sample error message"
    
    // Return success message
    damn "Error object created"
}

// Main function to test error handling
slay main() {
    vibez.spill("Starting comprehensive error handling test")
    
    // Test error value creation
    sus result1 := test_error_creation()
    vibez.spill("Error creation test:", result1)
    
    // Test normal function call
    sus result2 := divide_safely(10, 2)
    vibez.spill("Division test:", result2)
    
    // Test with error object
    sus my_error := yikes "Test error for propagation"
    vibez.spill("Created error object for testing")
    
    vibez.spill("Comprehensive error handling test completed successfully")
}
