# Comprehensive Error Propagation Test for CURSED
yeet "testz"

# Test function that returns Result<normie, tea>
slay divide_safe(a normie, b normie) (normie, yikes) {
    vibe_check b {
        mood 0:
            damn 0, yikes("Division by zero error")
        basic:
            damn a / b, cringe
    }
}

# Test function that returns Result<tea, tea>
slay read_file_safe(filename tea) (tea, yikes) {
    vibe_check filename {
        mood "":
            damn "", yikes("Empty filename")
        mood "invalid.txt":
            damn "", yikes("File not found")
        basic:
            damn "File content: " + filename, cringe
    }
}

# Function using shook operator for error propagation
slay process_numbers_shook(a normie, b normie) (normie, yikes) {
    # This should automatically propagate errors using shook operator
    sus result = divide_safe(a, b) shook
    damn result * 2, cringe
}

# Function using manual error handling (for comparison)
slay process_numbers_manual(a normie, b normie) (normie, yikes) {
    sus result, err = divide_safe(a, b)
    vibe_check err != cringe {
        damn 0, err
    }
    damn result * 2, cringe
}

# Function with multiple error propagation steps
slay complex_processing_shook(filename tea, divisor normie) (tea, yikes) {
    # Chain multiple operations with shook
    sus content = read_file_safe(filename) shook
    sus number = divide_safe(42, divisor) shook
    damn content + " Result: " + string(number), cringe
}

# Function testing Option-like behavior (if supported)
slay safe_array_access(arr []normie, index normie) (normie, yikes) {
    vibe_check index >= 0 && index < len(arr) {
        damn arr[index], cringe
    }
    damn 0, yikes("Index out of bounds")
}

# Function using nested error propagation
slay nested_operations_shook(a normie, b normie, c normie) (normie, yikes) {
    sus step1 = divide_safe(a, b) shook
    sus step2 = divide_safe(step1, c) shook
    sus step3 = divide_safe(step2, 2) shook
    damn step3, cringe
}

# Test cases for error propagation
slay test_error_propagation() {
    test_start("Error Propagation Comprehensive Tests")
    
    # Test 1: Successful operation with shook
    sus result1, err1 = process_numbers_shook(10, 2)
    assert_eq_int(result1, 10)  # (10 / 2) * 2 = 10
    assert_true(err1 == cringe)
    
    # Test 2: Error propagation with shook
    sus result2, err2 = process_numbers_shook(10, 0)
    assert_eq_int(result2, 0)
    assert_true(err2 != cringe)
    assert_eq_string(err2.message(), "Division by zero error")
    
    # Test 3: Compare shook vs manual handling (successful case)
    sus manual_result, manual_err = process_numbers_manual(8, 4)
    sus shook_result, shook_err = process_numbers_shook(8, 4)
    assert_eq_int(manual_result, shook_result)
    assert_true(manual_err == shook_err)
    
    # Test 4: Compare shook vs manual handling (error case)
    sus manual_result_err, manual_err_err = process_numbers_manual(8, 0)
    sus shook_result_err, shook_err_err = process_numbers_shook(8, 0)
    assert_eq_int(manual_result_err, shook_result_err)
    assert_eq_string(manual_err_err.message(), shook_err_err.message())
    
    # Test 5: Complex error propagation chain
    sus complex_result, complex_err = complex_processing_shook("test.txt", 3)
    assert_true(complex_err == cringe)
    assert_eq_string(complex_result, "File content: test.txt Result: 14")
    
    # Test 6: Error in first step of complex chain
    sus complex_err_result, complex_err_err = complex_processing_shook("", 3)
    assert_true(complex_err_err != cringe)
    assert_eq_string(complex_err_err.message(), "Empty filename")
    
    # Test 7: Error in second step of complex chain
    sus complex_err2_result, complex_err2_err = complex_processing_shook("valid.txt", 0)
    assert_true(complex_err2_err != cringe)
    assert_eq_string(complex_err2_err.message(), "Division by zero error")
    
    # Test 8: Nested error propagation - success case
    sus nested_result, nested_err = nested_operations_shook(16, 2, 2)
    assert_eq_int(nested_result, 2)  # ((16/2)/2)/2 = 2
    assert_true(nested_err == cringe)
    
    # Test 9: Nested error propagation - error case
    sus nested_err_result, nested_err_err = nested_operations_shook(16, 0, 2)
    assert_true(nested_err_err != cringe)
    assert_eq_string(nested_err_err.message(), "Division by zero error")
    
    # Test 10: Array access with error handling
    sus arr = [1, 2, 3, 4, 5]
    sus safe_result, safe_err = safe_array_access(arr, 2)
    assert_eq_int(safe_result, 3)
    assert_true(safe_err == cringe)
    
    # Test 11: Array access error case
    sus unsafe_result, unsafe_err = safe_array_access(arr, 10)
    assert_eq_int(unsafe_result, 0)
    assert_true(unsafe_err != cringe)
    assert_eq_string(unsafe_err.message(), "Index out of bounds")
    
    print_test_summary()
}

# Run the comprehensive test
test_error_propagation()
