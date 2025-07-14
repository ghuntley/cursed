#!/usr/bin/env cursed

// Test file for function inlining optimization validation

yeet "testz"

// Small utility functions that should be inlined
slay add_numbers(a normie, b normie) normie {
    damn a + b
}

slay multiply_by_two(x normie) normie {
    damn x * 2
}

slay is_positive(n normie) lit {
    damn n > 0
}

// Generic function that should be inlined when enabled
slay generic_max[T](a T, b T) T {
    sus result T
    bruh (a > b) {
        result = a
    } nah {
        result = b
    }
    damn result
}

// Interface method that should be inlined when enabled
slay interface_method_impl(value normie) normie {
    damn value * 3
}

// Hot path function that should be prioritized for inlining
slay hot_path_calculation(x normie, y normie) normie {
    sus temp1 normie = add_numbers(x, y)
    sus temp2 normie = multiply_by_two(temp1)
    sus temp3 normie = interface_method_impl(temp2)
    damn temp3
}

// Test function with multiple call sites
slay test_multiple_calls() {
    test_start("Function Inlining Optimization Test")
    
    // Test basic function calls that should be inlined
    sus result1 normie = add_numbers(5, 3)
    assert_eq_int(result1, 8)
    
    sus result2 normie = multiply_by_two(10)
    assert_eq_int(result2, 20)
    
    sus result3 lit = is_positive(-5)
    assert_false(result3)
    
    sus result4 lit = is_positive(5)
    assert_true(result4)
    
    // Test generic function inlining
    sus max_result normie = generic_max(15, 25)
    assert_eq_int(max_result, 25)
    
    // Test interface method inlining
    sus interface_result normie = interface_method_impl(10)
    assert_eq_int(interface_result, 30)
    
    // Test hot path with multiple inlined calls
    sus hot_result normie = hot_path_calculation(5, 10)
    assert_eq_int(hot_result, 45)
    
    // Test loop with inlined calls for performance
    sus loop_result normie = 0
    bestie i := 0; i < 1000; i++ {
        loop_result += add_numbers(i, multiply_by_two(i))
    }
    assert_true(loop_result > 0)
    
    print_test_summary()
}

// Recursive function that should NOT be inlined
slay recursive_factorial(n normie) normie {
    bruh (n <= 1) {
        damn 1
    }
    damn n * recursive_factorial(n - 1)
}

// Large function that should NOT be inlined
slay large_function(x normie) normie {
    sus result normie = x
    // Many operations to make it large
    result += 1
    result *= 2
    result += 3
    result *= 4
    result += 5
    result *= 6
    result += 7
    result *= 8
    result += 9
    result *= 10
    result += 11
    result *= 12
    result += 13
    result *= 14
    result += 15
    result *= 16
    result += 17
    result *= 18
    result += 19
    result *= 20
    damn result
}

// Test function to validate inlining decisions
slay test_inlining_decisions() {
    test_start("Inlining Decision Validation")
    
    // Test that recursive functions are not inlined
    sus factorial_result normie = recursive_factorial(5)
    assert_eq_int(factorial_result, 120)
    
    // Test that large functions are not inlined
    sus large_result normie = large_function(2)
    assert_true(large_result > 0)
    
    print_test_summary()
}

// Main test runner
slay main() {
    vibez.spill("Starting Function Inlining Optimization Tests")
    
    test_multiple_calls()
    test_inlining_decisions()
    
    vibez.spill("Function Inlining Optimization Tests Complete")
}

// Performance benchmark function
slay benchmark_inlining_performance() {
    vibez.spill("Running inlining performance benchmark...")
    
    sus iterations normie = 100000
    sus result normie = 0
    
    bestie i := 0; i < iterations; i++ {
        result += hot_path_calculation(i, i + 1)
    }
    
    vibez.spill("Benchmark completed with result: " + result)
}
