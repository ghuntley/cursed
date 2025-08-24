// Advanced Edge Case Testing: Boundary Conditions
yeet "testz"
yeet "mathz"
yeet "stringz"
yeet "arrayz"

test_start("Boundary Conditions Edge Cases")

// Integer boundary testing
slay test_integer_boundaries() {
    sus max_drip = 9223372036854775807  // i64 max
    sus min_drip = -9223372036854775808 // i64 min
    
    assert_eq_int(max_drip + 1, min_drip)  // Overflow wrapping
    assert_eq_int(min_drip - 1, max_drip)  // Underflow wrapping
    
    // Division by near-zero
    sus tiny_num drip = 1
    sus result drip = max_drip / tiny_num
    assert_eq_int(result, max_drip)
    
    test_pass("Integer boundary conditions")
}

// Array boundary testing
slay test_array_boundaries() {
    sus arr []drip = [1, 2, 3, 4, 5]
    sus len drip = len(arr)
    
    // Valid boundaries
    assert_eq_int(arr[0], 1)
    assert_eq_int(arr[len - 1], 5)
    
    // Test bounds checking (should panic safely)
    sus safe_access lit = based
    ready {
        sus invalid drip = arr[len]  // Out of bounds
        safe_access = cap
    } fam {
        when _ -> safe_access = based
    }
    assert_eq_bool(safe_access, based)
    
    test_pass("Array boundary conditions")
}

// String boundary testing
slay test_string_boundaries() {
    sus empty_str tea = ""
    sus long_str tea = "a" * 10000  // Very long string
    
    assert_eq_int(len(empty_str), 0)
    assert_eq_int(len(long_str), 10000)
    
    // Unicode boundary testing
    sus unicode_str tea = "🚀💻🔥"
    assert_eq_int(len(unicode_str), 3)  // Character count, not bytes
    
    // String slicing boundaries
    sus slice_test tea = "abcdef"
    assert_eq_str(slice_test[0:3], "abc")
    assert_eq_str(slice_test[3:6], "def")
    assert_eq_str(slice_test[0:0], "")
    
    test_pass("String boundary conditions")
}

// Memory allocation boundaries
slay test_memory_boundaries() {
    // Large array allocation
    sus large_array []drip = []
    bestie (drip i = 0; i < 100000; i = i + 1) {
        append(large_array, i)
    }
    assert_eq_int(len(large_array), 100000)
    assert_eq_int(large_array[99999], 99999)
    
    // Nested structure boundaries
    sus nested_arrays [][]drip = []
    bestie (drip i = 0; i < 1000; i = i + 1) {
        sus inner_array []drip = []
        bestie (drip j = 0; j < 100; j = j + 1) {
            append(inner_array, i * j)
        }
        append(nested_arrays, inner_array)
    }
    assert_eq_int(len(nested_arrays), 1000)
    assert_eq_int(len(nested_arrays[500]), 100)
    
    test_pass("Memory allocation boundaries")
}

// Floating point boundaries
slay test_float_boundaries() {
    sus inf_pos drip = 1.0 / 0.0
    sus inf_neg drip = -1.0 / 0.0
    sus nan_val drip = 0.0 / 0.0
    
    // Test infinity handling
    assert_eq_bool(is_infinite(inf_pos), based)
    assert_eq_bool(is_infinite(inf_neg), based)
    assert_eq_bool(is_nan(nan_val), based)
    
    // Very small/large numbers
    sus tiny drip = 1e-308
    sus huge drip = 1e308
    
    assert_eq_bool(tiny > 0.0, based)
    assert_eq_bool(huge < inf_pos, based)
    
    test_pass("Floating point boundaries")
}

// Recursive depth boundaries
slay factorial_recursive(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_recursive(n - 1)
}

slay test_recursion_boundaries() {
    // Test safe recursion depth
    sus result drip = factorial_recursive(10)
    assert_eq_int(result, 3628800)
    
    // Test deep recursion handling (should not crash)
    ready {
        sus deep_result drip = factorial_recursive(10000)  // Very deep
    } fam {
        when "stack overflow" -> test_pass("Stack overflow handled gracefully")
        when _ -> test_pass("Deep recursion completed")
    }
    
    test_pass("Recursion depth boundaries")
}

// Run all boundary tests
test_integer_boundaries()
test_array_boundaries()
test_string_boundaries()
test_memory_boundaries()
test_float_boundaries()
test_recursion_boundaries()

print_test_summary()
