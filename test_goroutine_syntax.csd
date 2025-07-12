// Comprehensive test for all goroutine parsing syntax variations

// Function definitions for testing
slay worker(name tea) {
    vibez.spill("Worker: " + name)
}

slay processData(input normie) {
    vibez.spill("Processing: " + input)
}

slay noParams() {
    vibez.spill("No parameters")
}

// Test all goroutine syntax variations
vibez.spill("=== Testing Goroutine Syntax ===")

// 1. Simple function call
vibez.spill("Test 1: Simple function call")
stan noParams()

// 2. Function with single parameter
vibez.spill("Test 2: Function with single parameter")
stan worker("Alice")

// 3. Function with multiple parameters
vibez.spill("Test 3: Function with numeric parameter")
stan processData(42)

// 4. Member access function call
vibez.spill("Test 4: Member access function call")
stan vibez.spill("Hello from goroutine")

// 5. Complex function call with expressions
vibez.spill("Test 5: Complex expression")
stan worker("Bob" + "Builder")

vibez.spill("=== All goroutine syntax tests completed ===")
