// Test basic fam functionality
vibez.spill("Testing basic fam")

// Simple fam block
fam {
    yikes test_error := "Simple error"
    vibez.spill("In fam block")
} sus caught {
    vibez.spill("Caught:", caught)
}

vibez.spill("Fam test completed")
