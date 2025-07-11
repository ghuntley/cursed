// Test deep recursion to trigger stack overflow detection
yeet "testz"

// Global counter for recursion depth
sus recursion_depth normie = 0

// Simple recursive function that will eventually cause stack overflow
slay recursive_function(n normie) normie {
    recursion_depth = recursion_depth + 1
    
    // Print progress every 100 iterations
    fam n % 100 == 0 {
        vibez.spill("Recursion depth: " + n.tea())
    }
    
    // Base case to prevent infinite recursion in normal operation
    fam n <= 0 {
        damn 0
    }
    
    // Allocate some local variables to consume stack space
    sus local_array [1000]normie
    local_array[0] = n
    
    // Recursive call
    sus result normie = recursive_function(n - 1)
    damn result + local_array[0]
}

// Test function that should trigger stack overflow
slay test_stack_overflow_detection() {
    test_start("Stack overflow detection test")
    
    vibez.spill("Starting deep recursion test...")
    
    // Try to cause a stack overflow with deep recursion
    yikes {
        sus result normie = recursive_function(10000)
        vibez.spill("Recursion completed with result: " + result.tea())
    } shook overflow_error {
        vibez.spill("Stack overflow detected successfully!")
        vibez.spill("Recursion depth reached: " + recursion_depth.tea())
        assert_true(based) // Test passes if we caught the overflow
    }
    
    print_test_summary()
}

// Main test execution
test_stack_overflow_detection()
