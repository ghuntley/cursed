vibe main
yeet "vibez"
yeet "stringz"

// Test boundary and error conditions
slay safe_array_access(arr normie[value], index normie) normie {
    ready (len(arr) == 0) {
        vibez.spill("Error: Cannot access empty array")
        damn -1
    }
    
    ready (index < 0) {
        vibez.spill("Error: Negative index:", index)
        damn -1
    }
    
    ready (index >= len(arr)) {
        vibez.spill("Error: Index out of bounds:", index, ">=", len(arr))
        damn -1
    }
    
    damn arr[index]
}

slay safe_string_access(str tea, pos normie) tea {
    sus str_len = stringz.length(str)
    
    ready (str_len == 0) {
        vibez.spill("Error: Cannot access empty string")
        damn ""
    }
    
    ready (pos < 0) {
        vibez.spill("Error: Negative string position:", pos)
        damn ""
    }
    
    ready (pos >= str_len) {
        vibez.spill("Error: String position out of bounds:", pos, ">=", str_len)
        damn ""
    }
    
    damn stringz.substring(str, pos, pos + 1)
}

slay test_array_boundary_errors() {
    vibez.spill("=== Array Boundary Error Tests ===")
    
    sus test_array normie[value] = normie[value]{10, 20, 30, 40, 50}
    vibez.spill("Test array:", test_array, "length:", len(test_array))
    
    // Valid access tests
    sus val1 = safe_array_access(test_array, 0)
    vibez.spill("arr[0] =", val1)
    
    sus val2 = safe_array_access(test_array, 2)
    vibez.spill("arr[2] =", val2)
    
    sus val3 = safe_array_access(test_array, len(test_array) - 1)
    vibez.spill("arr[last] =", val3)
    
    // Error condition tests
    sus err1 = safe_array_access(test_array, -1)
    vibez.spill("arr[-1] result:", err1)
    
    sus err2 = safe_array_access(test_array, len(test_array))
    vibez.spill("arr[len] result:", err2)
    
    sus err3 = safe_array_access(test_array, 100)
    vibez.spill("arr[100] result:", err3)
    
    // Empty array test
    sus empty normie[value] = normie[value]{}
    sus err4 = safe_array_access(empty, 0)
    vibez.spill("empty[0] result:", err4)
}

slay test_string_boundary_errors() {
    vibez.spill("=== String Boundary Error Tests ===")
    
    sus test_string = "CURSED"
    sus str_len = stringz.length(test_string)
    vibez.spill("Test string:", test_string, "length:", str_len)
    
    // Valid access tests
    sus char1 = safe_string_access(test_string, 0)
    vibez.spill("str[0] =", char1)
    
    sus char2 = safe_string_access(test_string, 2)
    vibez.spill("str[2] =", char2)
    
    sus char3 = safe_string_access(test_string, str_len - 1)
    vibez.spill("str[last] =", char3)
    
    // Error condition tests
    sus err1 = safe_string_access(test_string, -1)
    vibez.spill("str[-1] result:", err1)
    
    sus err2 = safe_string_access(test_string, str_len)
    vibez.spill("str[len] result:", err2)
    
    sus err3 = safe_string_access(test_string, 100)
    vibez.spill("str[100] result:", err3)
    
    // Empty string test
    sus empty_str = ""
    sus err4 = safe_string_access(empty_str, 0)
    vibez.spill("empty_str[0] result:", err4)
}

slay safe_division_with_logging(a normie, b normie) normie {
    vibez.spill("Attempting division:", a, "/", b)
    
    ready (b == 0) {
        vibez.spill("Division by zero error prevented")
        vibez.spill("Returning error code -999")
        damn -999
    }
    
    sus result = a / b
    vibez.spill("Division successful, result:", result)
    damn result
}

slay test_mathematical_error_conditions() {
    vibez.spill("=== Mathematical Error Condition Tests ===")
    
    // Normal operations
    sus res1 = safe_division_with_logging(20, 4)
    sus res2 = safe_division_with_logging(15, 3)
    
    // Error conditions
    sus err1 = safe_division_with_logging(10, 0)
    sus err2 = safe_division_with_logging(-5, 0)
    
    // Edge cases
    sus edge1 = safe_division_with_logging(0, 5)
    sus edge2 = safe_division_with_logging(1, 1)
    
    vibez.spill("Results summary:")
    vibez.spill("Normal results:", res1, res2)
    vibez.spill("Error results:", err1, err2)
    vibez.spill("Edge results:", edge1, edge2)
}

slay test_nested_error_conditions() {
    vibez.spill("=== Nested Error Condition Tests ===")
    
    sus test_array normie[value] = normie[value]{1, 2, 0, 4, 5}
    
    bestie i := 0; i < len(test_array); i++ {
        sus current_val = safe_array_access(test_array, i)
        
        ready (current_val == -1) {
            vibez.spill("Array access failed for index:", i)
            simp
        }
        
        vibez.spill("Processing element", i, ":", current_val)
        
        // Try to use as divisor
        sus division_result = safe_division_with_logging(100, current_val)
        
        ready (division_result == -999) {
            vibez.spill("Division failed with element:", current_val)
        } basic {
            vibez.spill("Division result:", division_result)
        }
    }
}

slay main() {
    vibez.spill("=== Boundary Error Condition Tests ===")
    
    test_array_boundary_errors()
    test_string_boundary_errors()
    test_mathematical_error_conditions()
    test_nested_error_conditions()
    
    vibez.spill("All boundary error condition tests completed")
}
