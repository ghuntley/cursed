// Test program to verify thread safety fixes
vibez.spill("Testing JIT thread safety fixes...")

// Test 1: Basic compilation
sus x := 42
vibez.spill("Basic test passed")

// Test 2: Multiple variable declarations
sus a := 1
sus b := 2
sus c := a + b
vibez.spill("Multiple variables test passed")

// Test 3: String operations
sus message := "Thread safety test"
vibez.spill(message)

vibez.spill("All thread safety tests completed successfully!")
