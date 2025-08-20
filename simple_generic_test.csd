// Simple generic function test

// Basic identity function
slay identity[T](value T) T {
    vibez.spill("Inside identity function with value:", value)
    damn value
}

slay main() {
    vibez.spill("Starting simple generic test")
    
    // Test with explicit string type
    sus result tea = identity[tea]("hello")
    vibez.spill("Result:", result)
}
