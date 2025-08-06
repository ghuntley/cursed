// Simple Windows compatibility test
sus name tea = "Windows Test"
sus value drip = 42

vibez.spill("Testing CURSED on Windows!")
vibez.spill(name)
vibez.spill("Value: " + tea(value))

// Test basic functionality that could be Windows-specific
slay test_function() {
    vibez.spill("Function call works on Windows")
}

test_function()
vibez.spill("Windows cross-compilation test complete!")
