// Comprehensive test for goroutine parsing

// Define a simple function
slay worker(name tea) {
    vibez.spill("Worker " + name + " started")
}

// Define a function with parameters
slay processData(input normie, output tea) {
    vibez.spill("Processing data: " + input)
}

// Test goroutine spawning
vibez.spill("Starting goroutines...")

// Simple function call goroutine
stan worker("A")

// Function call with parameters
stan processData(42, "result")

// Test member access goroutine
stan vibez.spill("Hello from goroutine")

vibez.spill("Goroutines started")
