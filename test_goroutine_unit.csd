// Unit test for goroutine parsing with 'stan' keyword

// Test that we can parse and execute goroutine statements
slay testFunction() {
    vibez.spill("Test function called")
}

// Test simple goroutine
stan testFunction()

// Test goroutine with parameters
stan vibez.spill("Hello from goroutine")

vibez.spill("Program complete")
