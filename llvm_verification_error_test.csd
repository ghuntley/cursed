# Test file to trigger LLVM verification errors for testing the new error handling system

# This should create some basic IR that might fail verification in certain conditions
sus broken_function() {
    # Intentionally incomplete function that might cause verification issues
    sus x drip = 42
    # Missing return statement that could cause verification failure
}

# Test basic variable
sus test_var drip = 123

# Output something
vibez.spill("Testing LLVM verification error handling")
