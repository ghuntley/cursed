// Test error handling runtime integration

vibez.spill("Testing error handling runtime...")

// Test basic error creation
yikes error_test := "This is a test error"
vibez.spill("Created error: error_test")

// Test error propagation with shook operator
sus result := (shook error_test)
vibez.spill("Propagated error with shook operator")

// Test error recovery with fam
fam {
    vibez.spill("In fam recovery block")
    yikes another_error := "Another error for testing"
    vibez.spill("Created another error in fam block")
}

vibez.spill("Error handling test complete")
