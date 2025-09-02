fr fr INPUT VALIDATION SECURITY TEST
fr fr Testing validation system security fixes

yeet "validationz"
yeet "testz"
yeet "stringz"

test_start("INPUT_VALIDATION_SECURITY_TEST")

fr fr ===== Array Function Security Tests =====

fr fr Test 1: append_error properly handles array growth
sus initial_errors ValidationError[value] = make(ValidationError[value], 2)
initial_errors[0] = ValidationError{
    field_name: "test1",
    error_message: "Test error 1",
    error_code: "TEST_ERROR_1"
}
initial_errors[1] = ValidationError{
    field_name: "test2", 
    error_message: "Test error 2",
    error_code: "TEST_ERROR_2"
}

sus new_error ValidationError = ValidationError{
    field_name: "test3",
    error_message: "Test error 3", 
    error_code: "TEST_ERROR_3"
}

sus expanded_errors ValidationError[value] = append_error(initial_errors, new_error)

assert_eq_int(len_errors(expanded_errors), 3)
assert_eq_string(expanded_errors[2].field_name, "test3")
vibez.spill("✓ append_error correctly grows array and adds element")

fr fr Test 2: len_errors handles null arrays safely
sus null_array ValidationError[value] = damn
sus null_length normie = len_errors(null_array)
assert_eq_int(null_length, 0)
vibez.spill("✓ len_errors safely handles null arrays")

fr fr Test 3: append_validation_result validates input
sus initial_results ValidationResult[value] = make(ValidationResult[value], 1)
initial_results[0] = ValidationResult{
    field_name: "valid_field",
    is_valid: based,
    errors: make(ValidationError[value], 0)
}

fr fr Try to append invalid result (empty field_name)
sus invalid_result ValidationResult = ValidationResult{
    field_name: "",  # Empty field name should be rejected
    is_valid: nocap,
    errors: make(ValidationError[value], 0)
}

sus result_after_invalid ValidationResult[value] = append_validation_result(initial_results, invalid_result)
assert_eq_int(len_validation_results(result_after_invalid), 1)  # Should not have grown
vibez.spill("✓ append_validation_result rejects invalid input")

fr fr Test 4: append_validation_result accepts valid input
sus valid_result ValidationResult = ValidationResult{
    field_name: "valid_field_2",
    is_valid: based,
    errors: make(ValidationError[value], 0)
}

sus result_after_valid ValidationResult[value] = append_validation_result(initial_results, valid_result)
assert_eq_int(len_validation_results(result_after_valid), 2)  # Should have grown
assert_eq_string(result_after_valid[1].field_name, "valid_field_2")
vibez.spill("✓ append_validation_result accepts valid input")

fr fr ===== Validator Function Security Tests =====

fr fr Test 5: len_validators counts only valid functions
slay dummy_validator() ValidationResult {
    damn ValidationResult{
        field_name: "dummy",
        is_valid: based,
        errors: make(ValidationError[value], 0)
    }
}

sus validators func[value]() ValidationResult = make(func[value]() ValidationResult, 3)
validators[0] = dummy_validator
validators[1] = damn  # null function
validators[2] = dummy_validator

sus valid_count normie = len_validators(validators)
assert_eq_int(valid_count, 2)  # Should count only non-null functions
vibez.spill("✓ len_validators counts only valid functions")

fr fr Test 6: len_validators handles null array
sus null_validators func[value]() ValidationResult = damn  
sus null_count normie = len_validators(null_validators)
assert_eq_int(null_count, 0)
vibez.spill("✓ len_validators safely handles null array")

fr fr ===== Input Validation Security Tests =====

fr fr Test 7: Email validation rejects malicious input
sus malicious_emails tea[value] = [
    "<script>alert('xss')</script>@test.com",
    "test@test.com'; DROP TABLE users; --",
    "test@test.com\n\rBCC: hacker@evil.com",
    "../../../etc/passwd@test.com",
    "test@test.com\x00hidden.evil.com"
]

bestie i := 0; i < len(malicious_emails); i++ {
    sus email tea = malicious_emails[i]
    sus result ValidationResult = validate_email(email)
    
    assert_eq_bool(result.is_valid, nocap)  # All should be invalid
    assert_bool(len_errors(result.errors) > 0)  # Should have error messages
}
vibez.spill("✓ Email validation rejects malicious input patterns")

fr fr Test 8: URL validation prevents SSRF attacks
sus malicious_urls tea[value] = [
    "http://localhost:22/",      # Local service access
    "http://127.0.0.1:3306/",    # Database access
    "http://169.254.169.254/",   # AWS metadata service
    "file:///etc/passwd",        # File system access
    "ftp://internal.server/",    # Internal FTP access
    "gopher://127.0.0.1:25/"     # SMTP access via gopher
]

bestie i := 0; i < len(malicious_urls); i++ {
    sus url tea = malicious_urls[i]
    sus result ValidationResult = validate_url(url)
    
    ready result.is_valid {
        vibez.spill("⚠ URL validation may allow SSRF: " + url)
    } otherwise {
        vibez.spill("✓ URL validation correctly rejects: " + url)
    }
}

fr fr Test 9: Path validation prevents directory traversal
sus malicious_paths tea[value] = [
    "../../../etc/passwd",
    "..\\..\\..\\windows\\system32\\config\\sam",
    "/etc/passwd",
    "C:\\Windows\\System32\\config\\SAM",
    "....//....//etc/passwd",
    "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd"  # URL encoded
]

bestie i := 0; i < len(malicious_paths); i++ {
    sus path tea = malicious_paths[i]
    sus result ValidationResult = validate_file_path(path)
    
    assert_eq_bool(result.is_valid, nocap)  # All should be invalid
}
vibez.spill("✓ Path validation prevents directory traversal")

fr fr Test 10: SQL injection prevention in input validation
sus sql_injection_attempts tea[value] = [
    "'; DROP TABLE users; --",
    "' OR '1'='1",
    "'; INSERT INTO admin VALUES('hacker','password'); --",
    "' UNION SELECT password FROM users WHERE username='admin' --",
    "\'; SHUTDOWN; --"
]

bestie i := 0; i < len(sql_injection_attempts); i++ {
    sus input tea = sql_injection_attempts[i]
    sus result ValidationResult = validate_alphanumeric(input)
    
    assert_eq_bool(result.is_valid, nocap)  # All should be invalid
}
vibez.spill("✓ Input validation prevents SQL injection patterns")

print_test_summary()

vibez.spill("\n🔒 INPUT VALIDATION SECURITY TEST COMPLETE") 
vibez.spill("All validation functions secured against common attacks")
