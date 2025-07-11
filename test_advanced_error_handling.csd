// Advanced error handling test
yeet "testz"

// Function that might fail
slay divide_by_zero(x normie, y normie) normie {
    fam y == 0 {
        damn yikes "Division by zero error"
    }
    damn x / y
}

// Function that uses shook for error propagation
slay safe_divide(a normie, b normie) normie {
    sus result := shook divide_by_zero(a, b)
    damn result
}

// Test error handling
slay test_error_handling() {
    test_start("Error handling tests")
    
    // Test normal division
    sus normal_result := safe_divide(10, 2)
    assert_eq_int(normal_result, 5)
    
    // Test error propagation
    sus error_result := safe_divide(10, 0)
    // Should contain error information
    
    print_test_summary()
}

slay main() {
    vibez.spill("Testing advanced error handling")
    test_error_handling()
}
