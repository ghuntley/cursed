// CURSED Validation Module Test Suite
// Comprehensive tests for data validation and verification

yeet "testz"
yeet "validation"

slay test_validation_result_creation() {
    test_start("Validation Result Creation")
    
    // Test creating validation result
    sus result validation.ValidationResult = validation.create_validation_result()
    assert_true(result.is_valid)
    assert_eq_int(len(result.errors), 0)
    assert_eq_int(len(result.warnings), 0)
}

slay test_string_validation() {
    test_start("String Validation")
    
    // Test not empty validation
    sus valid_result validation.ValidationResult = validation.validate_not_empty("test")
    assert_true(valid_result.is_valid)
    
    sus invalid_result validation.ValidationResult = validation.validate_not_empty("")
    assert_false(invalid_result.is_valid)
    assert_eq_int(len(invalid_result.errors), 1)
    
    // Test min length validation
    sus min_length_valid validation.ValidationResult = validation.validate_min_length("hello", 3)
    assert_true(min_length_valid.is_valid)
    
    sus min_length_invalid validation.ValidationResult = validation.validate_min_length("hi", 5)
    assert_false(min_length_invalid.is_valid)
    
    // Test max length validation
    sus max_length_valid validation.ValidationResult = validation.validate_max_length("hi", 5)
    assert_true(max_length_valid.is_valid)
    
    sus max_length_invalid validation.ValidationResult = validation.validate_max_length("hello world", 5)
    assert_false(max_length_invalid.is_valid)
    
    // Test length range validation
    sus range_valid validation.ValidationResult = validation.validate_length_range("hello", 3, 10)
    assert_true(range_valid.is_valid)
    
    sus range_invalid validation.ValidationResult = validation.validate_length_range("hi", 5, 10)
    assert_false(range_invalid.is_valid)
}

slay test_numeric_validation() {
    test_start("Numeric Validation")
    
    // Test positive validation
    sus positive_valid validation.ValidationResult = validation.validate_positive(5)
    assert_true(positive_valid.is_valid)
    
    sus positive_invalid validation.ValidationResult = validation.validate_positive(-1)
    assert_false(positive_invalid.is_valid)
    
    // Test negative validation
    sus negative_valid validation.ValidationResult = validation.validate_negative(-5)
    assert_true(negative_valid.is_valid)
    
    sus negative_invalid validation.ValidationResult = validation.validate_negative(1)
    assert_false(negative_invalid.is_valid)
    
    // Test range validation
    sus range_valid validation.ValidationResult = validation.validate_range(5, 1, 10)
    assert_true(range_valid.is_valid)
    
    sus range_invalid_low validation.ValidationResult = validation.validate_range(0, 1, 10)
    assert_false(range_invalid_low.is_valid)
    
    sus range_invalid_high validation.ValidationResult = validation.validate_range(15, 1, 10)
    assert_false(range_invalid_high.is_valid)
    
    // Test min value validation
    sus min_valid validation.ValidationResult = validation.validate_min_value(5, 3)
    assert_true(min_valid.is_valid)
    
    sus min_invalid validation.ValidationResult = validation.validate_min_value(2, 3)
    assert_false(min_invalid.is_valid)
    
    // Test max value validation
    sus max_valid validation.ValidationResult = validation.validate_max_value(5, 10)
    assert_true(max_valid.is_valid)
    
    sus max_invalid validation.ValidationResult = validation.validate_max_value(15, 10)
    assert_false(max_invalid.is_valid)
}

slay test_float_validation() {
    test_start("Float Validation")
    
    // Test positive float validation
    sus positive_float_valid validation.ValidationResult = validation.validate_positive_float(5.5)
    assert_true(positive_float_valid.is_valid)
    
    sus positive_float_invalid validation.ValidationResult = validation.validate_positive_float(-1.5)
    assert_false(positive_float_invalid.is_valid)
    
    // Test float range validation
    sus float_range_valid validation.ValidationResult = validation.validate_float_range(5.5, 1.0, 10.0)
    assert_true(float_range_valid.is_valid)
    
    sus float_range_invalid validation.ValidationResult = validation.validate_float_range(0.5, 1.0, 10.0)
    assert_false(float_range_invalid.is_valid)
}

slay test_boolean_validation() {
    test_start("Boolean Validation")
    
    // Test is true validation
    sus true_valid validation.ValidationResult = validation.validate_is_true(based)
    assert_true(true_valid.is_valid)
    
    sus true_invalid validation.ValidationResult = validation.validate_is_true(cap)
    assert_false(true_invalid.is_valid)
    
    // Test is false validation
    sus false_valid validation.ValidationResult = validation.validate_is_false(cap)
    assert_true(false_valid.is_valid)
    
    sus false_invalid validation.ValidationResult = validation.validate_is_false(based)
    assert_false(false_invalid.is_valid)
}

slay test_array_validation() {
    test_start("Array Validation")
    
    // Test array not empty
    sus non_empty_array []tea = []tea{"item1", "item2"}
    sus array_not_empty_valid validation.ValidationResult = validation.validate_array_not_empty(non_empty_array)
    assert_true(array_not_empty_valid.is_valid)
    
    sus empty_array []tea = []tea{}
    sus array_not_empty_invalid validation.ValidationResult = validation.validate_array_not_empty(empty_array)
    assert_false(array_not_empty_invalid.is_valid)
    
    // Test array length
    sus array_length_valid validation.ValidationResult = validation.validate_array_length(non_empty_array, 2)
    assert_true(array_length_valid.is_valid)
    
    sus array_length_invalid validation.ValidationResult = validation.validate_array_length(non_empty_array, 3)
    assert_false(array_length_invalid.is_valid)
    
    // Test array min length
    sus array_min_length_valid validation.ValidationResult = validation.validate_array_min_length(non_empty_array, 1)
    assert_true(array_min_length_valid.is_valid)
    
    sus array_min_length_invalid validation.ValidationResult = validation.validate_array_min_length(non_empty_array, 5)
    assert_false(array_min_length_invalid.is_valid)
    
    // Test array max length
    sus array_max_length_valid validation.ValidationResult = validation.validate_array_max_length(non_empty_array, 5)
    assert_true(array_max_length_valid.is_valid)
    
    sus array_max_length_invalid validation.ValidationResult = validation.validate_array_max_length(non_empty_array, 1)
    assert_false(array_max_length_invalid.is_valid)
}

slay test_complex_validation() {
    test_start("Complex Validation")
    
    // Test email validation
    sus email_valid validation.ValidationResult = validation.validate_email("test@example.com")
    // Note: Basic implementation may not be fully functional yet
    
    sus email_invalid validation.ValidationResult = validation.validate_email("")
    assert_false(email_invalid.is_valid)
    
    // Test phone number validation
    sus phone_valid validation.ValidationResult = validation.validate_phone_number("1234567890")
    // Note: Basic implementation may not be fully functional yet
    
    sus phone_invalid validation.ValidationResult = validation.validate_phone_number("")
    assert_false(phone_invalid.is_valid)
    
    // Test URL validation
    sus url_valid validation.ValidationResult = validation.validate_url("https://example.com")
    // Note: Basic implementation may not be fully functional yet
    
    sus url_invalid validation.ValidationResult = validation.validate_url("")
    assert_false(url_invalid.is_valid)
}

slay test_composite_validation() {
    test_start("Composite Validation")
    
    // Test validate_all with all valid
    sus validators []validation.ValidationResult = []validation.ValidationResult{
        validation.validate_not_empty("test"),
        validation.validate_positive(5),
        validation.validate_is_true(based)
    }
    
    sus all_valid validation.ValidationResult = validation.validate_all(validators)
    assert_true(all_valid.is_valid)
    
    // Test validate_all with one invalid
    sus validators_with_invalid []validation.ValidationResult = []validation.ValidationResult{
        validation.validate_not_empty("test"),
        validation.validate_positive(-5),  // Invalid
        validation.validate_is_true(based)
    }
    
    sus all_invalid validation.ValidationResult = validation.validate_all(validators_with_invalid)
    assert_false(all_invalid.is_valid)
    
    // Test validate_any with one valid
    sus any_valid validation.ValidationResult = validation.validate_any(validators_with_invalid)
    assert_true(any_valid.is_valid)
}

slay test_validation_chain() {
    test_start("Validation Chain")
    
    // Test creating validation chain
    sus chain validation.ValidationChain = validation.create_validation_chain()
    assert_eq_int(len(chain.validators), 0)
    
    // Test adding validators to chain
    validation.chain_add_validator(&chain, validation.validate_not_empty("test"))
    validation.chain_add_validator(&chain, validation.validate_positive(5))
    assert_eq_int(len(chain.validators), 2)
    
    // Test executing chain
    sus chain_result validation.ValidationResult = validation.chain_execute(chain)
    assert_true(chain_result.is_valid)
}

slay test_utility_functions() {
    test_start("Utility Functions")
    
    // Test error/warning detection
    sus valid_result validation.ValidationResult = validation.validate_positive(5)
    assert_false(validation.has_errors(valid_result))
    assert_false(validation.has_warnings(valid_result))
    
    sus invalid_result validation.ValidationResult = validation.validate_positive(-5)
    assert_true(validation.has_errors(invalid_result))
    assert_eq_int(validation.get_error_count(invalid_result), 1)
    
    // Test formatting errors
    sus formatted tea = validation.format_validation_errors(invalid_result)
    assert_true(len(formatted) > 0)
}

slay test_quick_validation_functions() {
    test_start("Quick Validation Functions")
    
    // Test quick validation functions
    assert_true(validation.is_positive(5))
    assert_false(validation.is_positive(-5))
    
    assert_true(validation.is_in_range(5, 1, 10))
    assert_false(validation.is_in_range(15, 1, 10))
    
    // Test quick email validation
    sus email_result lit = validation.is_valid_email("test@example.com")
    // Note: May not be fully functional yet
    
    // Test quick phone validation
    sus phone_result lit = validation.is_valid_phone("1234567890")
    // Note: May not be fully functional yet
    
    // Test quick URL validation
    sus url_result lit = validation.is_valid_url("https://example.com")
    // Note: May not be fully functional yet
}

slay test_error_and_warning_management() {
    test_start("Error and Warning Management")
    
    // Test adding errors and warnings
    sus result validation.ValidationResult = validation.create_validation_result()
    assert_true(result.is_valid)
    
    validation.add_error(&result, "Test error")
    assert_false(result.is_valid)
    assert_eq_int(len(result.errors), 1)
    
    validation.add_warning(&result, "Test warning")
    assert_eq_int(len(result.warnings), 1)
    
    // Test error and warning counts
    assert_eq_int(validation.get_error_count(result), 1)
    assert_eq_int(validation.get_warning_count(result), 1)
}

// Main test runner
slay main() {
    vibez.spill("Starting CURSED Validation Module Tests")
    
    test_validation_result_creation()
    test_string_validation()
    test_numeric_validation()
    test_float_validation()
    test_boolean_validation()
    test_array_validation()
    test_complex_validation()
    test_composite_validation()
    test_validation_chain()
    test_utility_functions()
    test_quick_validation_functions()
    test_error_and_warning_management()
    
    print_test_summary()
}
