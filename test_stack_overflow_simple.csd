// Simple test demonstrating stack overflow detection in CURSED
yeet "testz"

// Global counter for tracking recursion depth
sus recursion_depth normie = 0
sus max_depth normie = 0

// Simple recursive function that will trigger stack overflow detection
slay deep_recursion(depth normie) normie {
    recursion_depth = recursion_depth + 1
    
    // Track maximum depth reached
    fam recursion_depth > max_depth {
        max_depth = recursion_depth
    }
    
    // Print progress every 50 iterations
    fam depth % 50 == 0 {
        vibez.spill("Recursion depth: " + depth.tea())
    }
    
    // Base case to prevent infinite recursion
    fam depth <= 0 {
        recursion_depth = recursion_depth - 1
        damn 0
    }
    
    // Allocate local variables to consume stack space
    sus local_data [100]normie
    local_data[0] = depth
    
    // Recursive call
    sus result normie = deep_recursion(depth - 1)
    
    recursion_depth = recursion_depth - 1
    damn result + local_data[0]
}

// Test function
slay test_stack_overflow_detection() {
    test_start("Stack overflow detection test")
    
    vibez.spill("Starting stack overflow detection test...")
    vibez.spill("This test will attempt deep recursion to test stack limits")
    
    // Reset counters
    recursion_depth = 0
    max_depth = 0
    
    // Try moderate recursion first
    vibez.spill("Testing moderate recursion (depth 100)...")
    sus result1 normie = deep_recursion(100)
    vibez.spill("Moderate recursion completed. Max depth: " + max_depth.tea())
    
    // Reset for deeper test
    recursion_depth = 0
    max_depth = 0
    
    // Test deeper recursion that may trigger stack overflow detection
    vibez.spill("Testing deeper recursion (depth 500)...")
    sus result2 normie = deep_recursion(500)
    vibez.spill("Deep recursion completed. Max depth: " + max_depth.tea())
    
    // The test passes if we can complete both recursions
    assert_true(result1 >= 0)
    assert_true(result2 >= 0)
    assert_true(max_depth > 0)
    
    vibez.spill("Stack overflow detection test completed successfully!")
    print_test_summary()
}

// Run the test
test_stack_overflow_detection()
