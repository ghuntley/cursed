// Test stack monitoring and overflow detection
yeet "testz"

// Test basic stack monitoring
slay test_stack_monitoring() {
    test_start("Basic stack monitoring test")
    
    vibez.spill("Testing basic CURSED stack monitoring...")
    
    // Test simple recursive function
    sus result normie = simple_recursive(10)
    vibez.spill("Simple recursion result: " + result.tea())
    
    assert_eq_int(result, 55) // Should be sum of 1+2+...+10
    
    print_test_summary()
}

// Simple recursive function for testing
slay simple_recursive(n normie) normie {
    fam n <= 1 {
        damn 1
    }
    damn n + simple_recursive(n - 1)
}

// Test with moderate recursion depth
slay test_moderate_recursion() {
    test_start("Moderate recursion test")
    
    vibez.spill("Testing moderate recursion depth...")
    
    sus result normie = factorial(10)
    vibez.spill("Factorial result: " + result.tea())
    
    assert_eq_int(result, 3628800) // 10! = 3628800
    
    print_test_summary()
}

// Factorial function for testing
slay factorial(n normie) normie {
    fam n <= 1 {
        damn 1
    }
    damn n * factorial(n - 1)
}

// Main test execution
test_stack_monitoring()
test_moderate_recursion()
