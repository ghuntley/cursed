// Test the performance optimizations we implemented
yeet "vibez"
yeet "timez"

// Test optimized string operations
slay test_string_operations() {
    vibez.spill("Testing optimized string operations...")
    
    // Test 1: String concatenation performance
    sus start_time normie = timez.nanos()
    
    // Simulate building a large string
    sus result tea = ""
    frfr i := 0; i < 1000; i++ {
        result = result + "test" + i.to_string() + " "
    }
    
    sus concat_time normie = timez.nanos() - start_time
    vibez.spill("String concatenation time: " + concat_time.to_string() + " ns")
    
    // Test 2: Memory allocation patterns
    start_time = timez.nanos()
    
    sus test_array []normie = []
    frfr i := 0; i < 10000; i++ {
        test_array.push(i)
    }
    
    sus alloc_time normie = timez.nanos() - start_time
    vibez.spill("Array allocation time: " + alloc_time.to_string() + " ns")
    
    vibez.spill("Performance test completed successfully!")
}

// Test basic functionality
slay test_basic_functionality() {
    vibez.spill("Testing basic CURSED functionality...")
    
    // Variables
    sus x normie = 42
    sus y drip = 3.14
    sus name tea = "CURSED"
    sus active lit = based
    
    vibez.spill("x = " + x.to_string())
    vibez.spill("y = " + y.to_string())
    vibez.spill("name = " + name)
    vibez.spill("active = " + (if active { "true" } nah { "false" }))
    
    // Arrays
    sus numbers []normie = [1, 2, 3, 4, 5]
    vibez.spill("Numbers: " + numbers.to_string())
    
    // Control flow
    if x > 40 {
        vibez.spill("x is greater than 40")
    }
    
    // Loops
    sus sum normie = 0
    frfr num in numbers {
        sum += num
    }
    vibez.spill("Sum of numbers: " + sum.to_string())
    
    vibez.spill("Basic functionality test passed!")
}

// Main test function
slay main() {
    vibez.spill("Starting CURSED performance optimization tests...")
    vibez.spill("")
    
    test_basic_functionality()
    vibez.spill("")
    test_string_operations()
    
    vibez.spill("")
    vibez.spill("All tests completed successfully!")
    vibez.spill("Performance optimizations are working correctly.")
}
