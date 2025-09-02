vibe main
yeet "vibez"
yeet "mathz"

// Test basic error handling scenarios
slay safe_divide(a normie, b normie) normie {
    ready (b == 0) {
        vibez.spill("Error: Division by zero prevented")
        damn -1  // Return error code
    }
    damn a / b
}

slay test_error_conditions() {
    vibez.spill("=== Basic Error Handling Tests ===")
    
    // Test 1: Normal operation
    sus result1 = safe_divide(10, 2)
    vibez.spill("Safe division 10/2 =", result1)
    
    // Test 2: Error condition  
    sus result2 = safe_divide(10, 0)
    vibez.spill("Safe division 10/0 result:", result2)
    
    // Test 3: Edge cases
    sus result3 = safe_divide(0, 5)
    vibez.spill("Safe division 0/5 =", result3)
    
    sus result4 = safe_divide(-10, 2)
    vibez.spill("Safe division -10/2 =", result4)
}

slay validate_input(value normie) normie {
    ready (value < 0) {
        vibez.spill("Warning: Negative value detected:", value)
        damn mathz.abs_normie(value)
    }
    
    ready (value > 1000) {
        vibez.spill("Warning: Large value detected:", value)  
        damn 1000
    }
    
    damn value
}

slay test_input_validation() {
    vibez.spill("=== Input Validation Tests ===")
    
    sus test_values normie[value] = normie[value]{-5, 42, 1500, 0, 999, -100}
    
    bestie i := 0; i < len(test_values); i++ {
        sus original = test_values[i]
        sus validated = validate_input(original)
        vibez.spill("Input:", original, "-> Validated:", validated)
    }
}

slay process_array_safe(arr normie[value]) {
    vibez.spill("=== Safe Array Processing ===")
    
    ready (len(arr) == 0) {
        vibez.spill("Error: Empty array provided")
        damn
    }
    
    vibez.spill("Processing array of length:", len(arr))
    
    sus sum normie = 0
    sus max_val normie = arr[0]
    sus min_val normie = arr[0]
    
    bestie i := 0; i < len(arr); i++ {
        sus val = arr[i]
        sum = sum + val
        
        ready (val > max_val) {
            max_val = val
        }
        
        ready (val < min_val) {
            min_val = val
        }
    }
    
    vibez.spill("Array sum:", sum)
    vibez.spill("Array max:", max_val)  
    vibez.spill("Array min:", min_val)
    vibez.spill("Array average:", sum / len(arr))
}

slay test_error_recovery() {
    vibez.spill("=== Error Recovery Tests ===")
    
    // Test empty array handling
    sus empty_array normie[value] = normie[value]{}
    process_array_safe(empty_array)
    
    // Test normal array
    sus normal_array normie[value] = normie[value]{1, 5, -3, 8, 2}
    process_array_safe(normal_array)
    
    // Test single element array
    sus single_array normie[value] = normie[value]{42}
    process_array_safe(single_array)
}

slay main() {
    vibez.spill("=== Error Handling Test Suite ===")
    
    test_error_conditions()
    test_input_validation()
    test_error_recovery()
    
    vibez.spill("All error handling tests completed")
}
