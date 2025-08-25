fr fr Comprehensive Validation Framework Test Suite
fr fr Tests ALL public functions with edge cases, error conditions, and performance testing

yeet "validationz"
yeet "testz"

fr fr ===== TEST HELPERS =====

slay test_validation_passes(result ValidationResult, test_name tea) {
    check !result.is_valid {
        testz.fail("Expected validation to pass for test: " + test_name)
    }
    testz.pass("✓ " + test_name)
}

slay test_validation_fails(result ValidationResult, test_name tea) {
    check result.is_valid {
        testz.fail("Expected validation to fail for test: " + test_name)
    }
    testz.pass("✓ " + test_name)
}

slay test_error_count(result ValidationResult, expected normie, test_name tea) {
    sus actual normie = get_error_count(result)
    check actual != expected {
        testz.fail("Expected " + core.int_to_string(expected) + " errors, got " + core.int_to_string(actual) + " for: " + test_name)
    }
    testz.pass("✓ " + test_name)
}

fr fr ===== VALIDATION RESULT TESTS =====

slay test_validation_result_creation() {
    testz.test_group("Validation Result Creation")
    
    sus result ValidationResult = new_validation_result()
    testz.assert_true(result.is_valid, "New validation result should be valid by default")
    testz.assert_eq_int(get_error_count(result), 0, "New validation result should have no errors")
    testz.assert_eq_int(get_warning_count(result), 0, "New validation result should have no warnings")
}

slay test_add_error_functionality() {
    testz.test_group("Add Error Functionality")
    
    sus result ValidationResult = new_validation_result()
    
    add_error(&result, "test_field", "Test error message", ERR_REQUIRED, "test_value")
    
    testz.assert_false(result.is_valid, "Result should be invalid after adding error")
    testz.assert_true(has_errors(result), "Result should have errors")
    testz.assert_eq_int(get_error_count(result), 1, "Should have exactly 1 error")
    
    fr fr Test adding multiple errors
    add_error(&result, "field2", "Second error", ERR_TYPE_MISMATCH, "value2")
    testz.assert_eq_int(get_error_count(result), 2, "Should have 2 errors after adding second")
}

slay test_add_warning_functionality() {
    testz.test_group("Add Warning Functionality")
    
    sus result ValidationResult = new_validation_result()
    
    add_warning(&result, "test_field", "Test warning", ERR_CUSTOM, "test_value")
    
    testz.assert_true(result.is_valid, "Result should still be valid after adding warning")
    testz.assert_eq_int(get_warning_count(result), 1, "Should have exactly 1 warning")
    
    fr fr Add error and verify both counts
    add_error(&result, "error_field", "Error message", ERR_REQUIRED, "")
    testz.assert_eq_int(get_error_count(result), 1, "Should have 1 error")
    testz.assert_eq_int(get_warning_count(result), 1, "Should still have 1 warning")
}

fr fr ===== STRING VALIDATION TESTS =====

slay test_string_required_validation() {
    testz.test_group("String Required Validation")
    
    fr fr Test valid non-empty string
    sus valid_result ValidationResult = validate_string_required("hello", "test_field")
    test_validation_passes(valid_result, "Non-empty string should pass required validation")
    
    fr fr Test empty string
    sus empty_result ValidationResult = validate_string_required("", "test_field")
    test_validation_fails(empty_result, "Empty string should fail required validation")
    test_error_count(empty_result, 1, "Empty string should have 1 error")
    
    fr fr Test whitespace-only string (should still be considered valid since it's not empty)
    sus whitespace_result ValidationResult = validate_string_required("   ", "test_field")
    test_validation_passes(whitespace_result, "Whitespace string should pass required validation")
}

slay test_string_length_validation() {
    testz.test_group("String Length Validation")
    
    fr fr Test valid length
    sus valid_result ValidationResult = validate_string_length("hello", 3, 10, "test_field")
    test_validation_passes(valid_result, "String within length bounds should pass")
    
    fr fr Test too short
    sus short_result ValidationResult = validate_string_length("hi", 3, 10, "test_field")
    test_validation_fails(short_result, "String shorter than minimum should fail")
    test_error_count(short_result, 1, "Too short string should have 1 error")
    
    fr fr Test too long
    sus long_result ValidationResult = validate_string_length("this is way too long", 3, 10, "test_field")
    test_validation_fails(long_result, "String longer than maximum should fail")
    test_error_count(long_result, 1, "Too long string should have 1 error")
    
    fr fr Test boundary conditions
    sus exact_min_result ValidationResult = validate_string_length("123", 3, 10, "test_field")
    test_validation_passes(exact_min_result, "String exactly at minimum length should pass")
    
    sus exact_max_result ValidationResult = validate_string_length("1234567890", 3, 10, "test_field")
    test_validation_passes(exact_max_result, "String exactly at maximum length should pass")
}

slay test_email_validation() {
    testz.test_group("Email Validation")
    
    fr fr Test valid emails
    sus valid_email1 ValidationResult = validate_email("test@example.com", "email_field")
    test_validation_passes(valid_email1, "Valid email should pass validation")
    
    sus valid_email2 ValidationResult = validate_email("user.name@domain.co.uk", "email_field")
    test_validation_passes(valid_email2, "Valid email with subdomain should pass")
    
    fr fr Test invalid emails
    sus no_at ValidationResult = validate_email("testexample.com", "email_field")
    test_validation_fails(no_at, "Email without @ should fail")
    
    sus no_domain ValidationResult = validate_email("test@", "email_field")
    test_validation_fails(no_domain, "Email without domain should fail")
    
    sus no_extension ValidationResult = validate_email("test@domain", "email_field")
    test_validation_fails(no_extension, "Email without domain extension should fail")
    
    sus empty_email ValidationResult = validate_email("", "email_field")
    test_validation_fails(empty_email, "Empty email should fail")
    
    fr fr Test email length limits
    sus long_email ValidationResult = validate_email("a" + stringz.repeat("very", 60) + "@example.com", "email_field")
    test_validation_fails(long_email, "Email over 254 characters should fail")
}

slay test_url_validation() {
    testz.test_group("URL Validation")
    
    fr fr Test valid URLs
    sus http_url ValidationResult = validate_url("http://example.com", "url_field")
    test_validation_passes(http_url, "Valid HTTP URL should pass")
    
    sus https_url ValidationResult = validate_url("https://secure.example.com/path", "url_field")
    test_validation_passes(https_url, "Valid HTTPS URL should pass")
    
    fr fr Test invalid URLs
    sus no_protocol ValidationResult = validate_url("example.com", "url_field")
    test_validation_fails(no_protocol, "URL without protocol should fail")
    
    sus ftp_protocol ValidationResult = validate_url("ftp://files.example.com", "url_field")
    test_validation_fails(ftp_protocol, "FTP URL should fail (only HTTP/HTTPS allowed)")
    
    sus empty_url ValidationResult = validate_url("", "url_field")
    test_validation_fails(empty_url, "Empty URL should fail")
}

fr fr ===== NUMERIC VALIDATION TESTS =====

slay test_int_range_validation() {
    testz.test_group("Integer Range Validation")
    
    fr fr Test valid range
    sus valid_int ValidationResult = validate_int_range(50, 1, 100, "int_field")
    test_validation_passes(valid_int, "Integer within range should pass")
    
    fr fr Test boundary values
    sus min_boundary ValidationResult = validate_int_range(1, 1, 100, "int_field")
    test_validation_passes(min_boundary, "Integer at minimum boundary should pass")
    
    sus max_boundary ValidationResult = validate_int_range(100, 1, 100, "int_field")
    test_validation_passes(max_boundary, "Integer at maximum boundary should pass")
    
    fr fr Test out of range
    sus too_small ValidationResult = validate_int_range(0, 1, 100, "int_field")
    test_validation_fails(too_small, "Integer below minimum should fail")
    
    sus too_large ValidationResult = validate_int_range(101, 1, 100, "int_field")
    test_validation_fails(too_large, "Integer above maximum should fail")
}

slay test_positive_int_validation() {
    testz.test_group("Positive Integer Validation")
    
    sus positive_int ValidationResult = validate_positive_int(42, "int_field")
    test_validation_passes(positive_int, "Positive integer should pass")
    
    sus zero_int ValidationResult = validate_positive_int(0, "int_field")
    test_validation_fails(zero_int, "Zero should fail positive validation")
    
    sus negative_int ValidationResult = validate_positive_int(-5, "int_field")
    test_validation_fails(negative_int, "Negative integer should fail positive validation")
}

slay test_float_range_validation() {
    testz.test_group("Float Range Validation")
    
    sus valid_float ValidationResult = validate_float_range(3.14, 0.0, 10.0, "float_field")
    test_validation_passes(valid_float, "Float within range should pass")
    
    sus min_float ValidationResult = validate_float_range(0.0, 0.0, 10.0, "float_field")
    test_validation_passes(min_float, "Float at minimum should pass")
    
    sus max_float ValidationResult = validate_float_range(10.0, 0.0, 10.0, "float_field")
    test_validation_passes(max_float, "Float at maximum should pass")
    
    sus below_range ValidationResult = validate_float_range(-1.0, 0.0, 10.0, "float_field")
    test_validation_fails(below_range, "Float below range should fail")
    
    sus above_range ValidationResult = validate_float_range(11.0, 0.0, 10.0, "float_field")
    test_validation_fails(above_range, "Float above range should fail")
}

fr fr ===== ARRAY VALIDATION TESTS =====

slay test_array_length_validation() {
    testz.test_group("Array Length Validation")
    
    sus valid_array [tea] = ["one", "two", "three"]
    sus valid_result ValidationResult = validate_array_length(valid_array, 1, 5, "array_field")
    test_validation_passes(valid_result, "Array within length bounds should pass")
    
    sus short_array [tea] = []
    sus short_result ValidationResult = validate_array_length(short_array, 1, 5, "array_field")
    test_validation_fails(short_result, "Array shorter than minimum should fail")
    
    sus long_array [tea] = ["1", "2", "3", "4", "5", "6"]
    sus long_result ValidationResult = validate_array_length(long_array, 1, 5, "array_field")
    test_validation_fails(long_result, "Array longer than maximum should fail")
}

slay test_array_not_empty_validation() {
    testz.test_group("Array Not Empty Validation")
    
    sus non_empty_array [tea] = ["item"]
    sus valid_result ValidationResult = validate_array_not_empty(non_empty_array, "array_field")
    test_validation_passes(valid_result, "Non-empty array should pass")
    
    sus empty_array [tea] = []
    sus empty_result ValidationResult = validate_array_not_empty(empty_array, "array_field")
    test_validation_fails(empty_result, "Empty array should fail not-empty validation")
}

slay test_array_unique_validation() {
    testz.test_group("Array Unique Validation")
    
    sus unique_array [tea] = ["a", "b", "c", "d"]
    sus unique_result ValidationResult = validate_array_unique(unique_array, "array_field")
    test_validation_passes(unique_result, "Array with unique elements should pass")
    
    sus duplicate_array [tea] = ["a", "b", "c", "b"]
    sus duplicate_result ValidationResult = validate_array_unique(duplicate_array, "array_field")
    test_validation_fails(duplicate_result, "Array with duplicate elements should fail")
    
    sus single_item [tea] = ["single"]
    sus single_result ValidationResult = validate_array_unique(single_item, "array_field")
    test_validation_passes(single_result, "Single element array should pass uniqueness")
}

fr fr ===== VALIDATION CHAIN TESTS =====

slay test_validation_chains() {
    testz.test_group("Validation Chains")
    
    fr fr Test successful chain
    sus chain ValidationChain = new_validation_chain("username", "john_doe")
    chain = chain_required(&chain)
    chain = chain_min_length(&chain, 3)
    chain = chain_max_length(&chain, 20)
    
    sus chain_result ValidationResult = chain_get_result(chain)
    test_validation_passes(chain_result, "Valid username should pass all chain validations")
    
    fr fr Test failing chain
    sus fail_chain ValidationChain = new_validation_chain("username", "")
    fail_chain = chain_required(&fail_chain)
    fail_chain = chain_min_length(&fail_chain, 3)
    
    sus fail_result ValidationResult = chain_get_result(fail_chain)
    test_validation_fails(fail_result, "Empty username should fail chain validation")
    
    fr fr Test email chain
    sus email_chain ValidationChain = new_validation_chain("email", "test@example.com")
    email_chain = chain_required(&email_chain)
    email_chain = chain_email(&email_chain)
    
    sus email_result ValidationResult = chain_get_result(email_chain)
    test_validation_passes(email_result, "Valid email should pass email chain")
}

fr fr ===== COMPOSITE VALIDATION TESTS =====

slay test_combine_results() {
    testz.test_group("Combine Results")
    
    fr fr Create multiple validation results
    sus result1 ValidationResult = validate_string_required("test", "field1")
    sus result2 ValidationResult = validate_int_range(50, 1, 100, "field2")
    sus result3 ValidationResult = validate_email("invalid-email", "field3")
    
    sus results []ValidationResult = [result1, result2, result3]
    sus combined ValidationResult = combine_results(results)
    
    fr fr Should fail because result3 (email) is invalid
    test_validation_fails(combined, "Combined results should fail if any individual result fails")
    testz.assert_true(get_error_count(combined) > 0, "Combined results should have errors from failed validations")
}

fr fr ===== HELPER FUNCTION TESTS =====

slay test_pattern_matching() {
    testz.test_group("Pattern Matching")
    
    fr fr Test email pattern
    testz.assert_true(matches_pattern("test@example.com", "email"), "Valid email should match email pattern")
    testz.assert_false(matches_pattern("not-an-email", "email"), "Invalid email should not match email pattern")
    
    fr fr Test URL pattern
    testz.assert_true(matches_pattern("http://example.com", "url"), "Valid URL should match URL pattern")
    testz.assert_false(matches_pattern("not-a-url", "url"), "Invalid URL should not match URL pattern")
    
    fr fr Test numeric pattern
    testz.assert_true(matches_pattern("12345", "numeric"), "Numeric string should match numeric pattern")
    testz.assert_false(matches_pattern("", "numeric"), "Empty string should not match numeric pattern")
}

slay test_is_numeric() {
    testz.test_group("Numeric Detection")
    
    testz.assert_true(is_numeric("123"), "Simple number should be numeric")
    testz.assert_true(is_numeric("0"), "Zero should be numeric")
    testz.assert_false(is_numeric(""), "Empty string should not be numeric")
    testz.assert_false(is_numeric("abc"), "Letters should not be numeric")
    testz.assert_true(is_numeric("456789"), "Multi-digit number should be numeric")
}

fr fr ===== ERROR FORMATTING TESTS =====

slay test_error_formatting() {
    testz.test_group("Error Formatting")
    
    fr fr Test formatting valid result
    sus valid_result ValidationResult = new_validation_result()
    sus valid_message tea = format_errors(valid_result)
    testz.assert_eq_string(valid_message, "Validation passed", "Valid result should format correctly")
    
    fr fr Test formatting invalid result
    sus invalid_result ValidationResult = new_validation_result()
    add_error(&invalid_result, "test_field", "Test error", ERR_REQUIRED, "")
    
    sus error_message tea = format_errors(invalid_result)
    testz.assert_true(stringz.contains(error_message, "Validation failed"), "Error message should contain failure text")
    testz.assert_true(stringz.contains(error_message, "test_field"), "Error message should contain field name")
    testz.assert_true(stringz.contains(error_message, "Test error"), "Error message should contain error text")
}

slay test_warning_formatting() {
    testz.test_group("Warning Formatting")
    
    fr fr Test formatting result with no warnings
    sus no_warnings ValidationResult = new_validation_result()
    sus empty_message tea = format_warnings(no_warnings)
    testz.assert_eq_string(empty_message, "", "No warnings should return empty string")
    
    fr fr Test formatting result with warnings
    sus with_warnings ValidationResult = new_validation_result()
    add_warning(&with_warnings, "test_field", "Test warning", ERR_CUSTOM, "value")
    
    sus warning_message tea = format_warnings(with_warnings)
    testz.assert_true(stringz.contains(warning_message, "Validation warnings"), "Warning message should contain warnings text")
    testz.assert_true(stringz.contains(warning_message, "test_field"), "Warning message should contain field name")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_validation_performance() {
    testz.test_group("Performance Tests")
    
    fr fr Test large string validation performance
    sus large_string tea = stringz.repeat("a", 10000)
    sus start_time drip = timez.now_milliseconds()
    
    sus i normie = 0
    bestie i < 1000 {
        sus result ValidationResult = validate_string_length(large_string, 1, 20000, "perf_field")
        i = i + 1
    }
    
    sus end_time drip = timez.now_milliseconds()
    sus duration drip = end_time - start_time
    
    testz.assert_true(duration < 5000, "1000 large string validations should complete in under 5 seconds")
    vibez.spill("Performance: 1000 string validations took " + core.int_to_string(duration) + "ms")
}

slay test_array_validation_performance() {
    testz.test_group("Array Performance Tests")
    
    fr fr Test large array uniqueness validation
    sus large_array [tea] = []
    sus j normie = 0
    bestie j < 1000 {
        large_array = arrayz.append(large_array, "item_" + core.int_to_string(j))
        j = j + 1
    }
    
    sus start_time drip = timez.now_milliseconds()
    sus result ValidationResult = validate_array_unique(large_array, "perf_array")
    sus end_time drip = timez.now_milliseconds()
    sus duration drip = end_time - start_time
    
    test_validation_passes(result, "Large unique array should pass validation")
    testz.assert_true(duration < 1000, "Array uniqueness validation should complete in under 1 second")
    vibez.spill("Performance: Array uniqueness check took " + core.int_to_string(duration) + "ms")
}

fr fr ===== EDGE CASE TESTS =====

slay test_edge_cases() {
    testz.test_group("Edge Cases")
    
    fr fr Test Unicode strings
    sus unicode_result ValidationResult = validate_string_required("こんにちは", "unicode_field")
    test_validation_passes(unicode_result, "Unicode string should pass validation")
    
    fr fr Test very long field names
    sus long_field_name tea = stringz.repeat("field_", 100)
    sus long_field_result ValidationResult = validate_string_required("value", long_field_name)
    test_validation_passes(long_field_result, "Validation should work with long field names")
    
    fr fr Test special characters in values
    sus special_chars ValidationResult = validate_string_required("!@#$%^&*()", "special_field")
    test_validation_passes(special_chars, "Special characters should be allowed in required validation")
    
    fr fr Test null-like strings
    sus null_string ValidationResult = validate_string_required("null", "null_field")
    test_validation_passes(null_string, "String 'null' should pass required validation")
    
    sus undefined_string ValidationResult = validate_string_required("undefined", "undefined_field")
    test_validation_passes(undefined_string, "String 'undefined' should pass required validation")
}

fr fr ===== INTEGRATION TESTS =====

slay test_real_world_scenarios() {
    testz.test_group("Real World Scenarios")
    
    fr fr Test user registration form validation
    slay validate_user_registration(username tea, email tea, password tea, confirm_password tea) ValidationResult {
        sus results []ValidationResult = []
        
        fr fr Validate username
        results = arrayz.append(results, validate_string_required(username, "username"))
        results = arrayz.append(results, validate_string_length(username, 3, 20, "username"))
        
        fr fr Validate email
        results = arrayz.append(results, validate_email(email, "email"))
        
        fr fr Validate password
        results = arrayz.append(results, validate_string_required(password, "password"))
        results = arrayz.append(results, validate_string_length(password, 8, 50, "password"))
        
        fr fr Validate password confirmation
        check password != confirm_password {
            sus mismatch_result ValidationResult = new_validation_result()
            add_error(&mismatch_result, "confirm_password", "Passwords do not match", ERR_CUSTOM, confirm_password)
            results = arrayz.append(results, mismatch_result)
        }
        
        damn combine_results(results)
    }
    
    fr fr Test valid registration
    sus valid_reg ValidationResult = validate_user_registration("johndoe", "john@example.com", "password123", "password123")
    test_validation_passes(valid_reg, "Valid user registration should pass")
    
    fr fr Test invalid registration
    sus invalid_reg ValidationResult = validate_user_registration("", "invalid-email", "short", "different")
    test_validation_fails(invalid_reg, "Invalid user registration should fail")
    testz.assert_true(get_error_count(invalid_reg) >= 3, "Invalid registration should have multiple errors")
}

fr fr ===== MEMORY SAFETY TESTS =====

slay test_memory_safety() {
    testz.test_group("Memory Safety")
    
    fr fr Test validation with large number of errors
    sus stress_result ValidationResult = new_validation_result()
    
    sus k normie = 0
    bestie k < 100 {
        add_error(&stress_result, "field_" + core.int_to_string(k), "Error message " + core.int_to_string(k), ERR_CUSTOM, "value")
        k = k + 1
    }
    
    testz.assert_eq_int(get_error_count(stress_result), 100, "Should handle 100 errors without memory issues")
    test_validation_fails(stress_result, "Result with 100 errors should be invalid")
    
    fr fr Test validation chain reuse
    sus reused_chain ValidationChain = new_validation_chain("test", "value")
    
    sus n normie = 0
    bestie n < 50 {
        reused_chain = chain_required(&reused_chain)
        n = n + 1
    }
    
    sus final_result ValidationResult = chain_get_result(reused_chain)
    test_validation_passes(final_result, "Reused validation chain should work correctly")
}

fr fr ===== MAIN TEST RUNNER =====

slay run_all_validationz_tests() {
    testz.test_start("Comprehensive validationz Module Test Suite")
    
    vibez.spill("🚀 Running comprehensive validationz tests...")
    
    fr fr Core functionality tests
    test_validation_result_creation()
    test_add_error_functionality()
    test_add_warning_functionality()
    
    fr fr String validation tests
    test_string_required_validation()
    test_string_length_validation()
    test_email_validation()
    test_url_validation()
    
    fr fr Numeric validation tests
    test_int_range_validation()
    test_positive_int_validation()
    test_float_range_validation()
    
    fr fr Array validation tests
    test_array_length_validation()
    test_array_not_empty_validation()
    test_array_unique_validation()
    
    fr fr Chain validation tests
    test_validation_chains()
    
    fr fr Composite validation tests
    test_combine_results()
    
    fr fr Helper function tests
    test_pattern_matching()
    test_is_numeric()
    
    fr fr Formatting tests
    test_error_formatting()
    test_warning_formatting()
    
    fr fr Performance tests
    test_validation_performance()
    test_array_validation_performance()
    
    fr fr Edge case tests
    test_edge_cases()
    
    fr fr Integration tests
    test_real_world_scenarios()
    
    fr fr Memory safety tests
    test_memory_safety()
    
    testz.print_test_summary()
    vibez.spill("✅ validationz comprehensive test suite completed!")
}

fr fr Initialize and run tests
init_validationz()
run_all_validationz_tests()
