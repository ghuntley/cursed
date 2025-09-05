yeet "validation"

// Global test state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
    vibez.spill("Running test: " + name)
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } else {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_pass("assert_false: value is cap")
    } else {
        test_fail("assert_false failed: got " + tea(value) + ", expected cap")
    }
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } else {
        vibez.spill("❌ Some tests failed")
    }
}

slay test_basic_validation() {
    test_start("Basic Validation Functions")
    
    // Test not empty validation
    sus result1 ValidationResult = validate_not_empty("test")
    assert_true(result1.is_valid)
    
    sus result2 ValidationResult = validate_not_empty("")
    assert_false(result2.is_valid)
    
    // Test length validation
    sus result3 ValidationResult = validate_length("test", 4)
    assert_true(result3.is_valid)
    
    sus result4 ValidationResult = validate_length("test", 5)
    assert_false(result4.is_valid)
    
    // Test min length validation
    sus result5 ValidationResult = validate_min_length("test", 3)
    assert_true(result5.is_valid)
    
    sus result6 ValidationResult = validate_min_length("test", 5)
    assert_false(result6.is_valid)
}

slay test_email_validation() {
    test_start("Email Validation")
    
    // Valid emails
    sus result1 ValidationResult = validate_email("test@example.com")
    assert_true(result1.is_valid)
    
    sus result2 ValidationResult = validate_email("user.name@domain.co.uk")
    assert_true(result2.is_valid)
    
    // Invalid emails
    sus result3 ValidationResult = validate_email("")
    assert_false(result3.is_valid)
    
    sus result4 ValidationResult = validate_email("no-at-symbol")
    assert_false(result4.is_valid)
    
    sus result5 ValidationResult = validate_email("@no-local-part.com")
    assert_false(result5.is_valid)
    
    sus result6 ValidationResult = validate_email("no-domain@")
    assert_false(result6.is_valid)
}

slay test_phone_validation() {
    test_start("Phone Number Validation")
    
    // Valid phone numbers
    sus result1 ValidationResult = validate_phone_number("1234567890")
    assert_true(result1.is_valid)
    
    sus result2 ValidationResult = validate_phone_number("555-123-4567")
    assert_true(result2.is_valid)
    
    // Invalid phone numbers
    sus result3 ValidationResult = validate_phone_number("")
    assert_false(result3.is_valid)
    
    sus result4 ValidationResult = validate_phone_number("123")
    assert_false(result4.is_valid)
    
    sus result5 ValidationResult = validate_phone_number("abc-def-ghij")
    assert_false(result5.is_valid)
}

slay test_url_validation() {
    test_start("URL Validation")
    
    // Valid URLs
    sus result1 ValidationResult = validate_url("https://www.example.com")
    assert_true(result1.is_valid)
    
    sus result2 ValidationResult = validate_url("http://test.org/path")
    assert_true(result2.is_valid)
    
    sus result3 ValidationResult = validate_url("ftp://files.example.com")
    assert_true(result3.is_valid)
    
    // Invalid URLs
    sus result4 ValidationResult = validate_url("")
    assert_false(result4.is_valid)
    
    sus result5 ValidationResult = validate_url("not-a-url")
    assert_false(result5.is_valid)
    
    sus result6 ValidationResult = validate_url("http://")
    assert_false(result6.is_valid)
}

slay test_credit_card_validation() {
    test_start("Credit Card Validation")
    
    // Valid credit cards (test numbers)
    sus result1 ValidationResult = validate_credit_card("4111111111111111")
    assert_true(result1.is_valid)
    
    sus result2 ValidationResult = validate_credit_card("5555555555554444")
    assert_true(result2.is_valid)
    
    sus result3 ValidationResult = validate_credit_card("4111-1111-1111-1111")
    assert_true(result3.is_valid)
    
    // Invalid credit cards
    sus result4 ValidationResult = validate_credit_card("")
    assert_false(result4.is_valid)
    
    sus result5 ValidationResult = validate_credit_card("123")
    assert_false(result5.is_valid)
    
    sus result6 ValidationResult = validate_credit_card("4111111111111112")
    assert_false(result6.is_valid)
}

slay test_date_validation() {
    test_start("Date Format Validation")
    
    // Valid dates
    sus result1 ValidationResult = validate_date_format("2023-12-31", "YYYY-MM-DD")
    assert_true(result1.is_valid)
    
    sus result2 ValidationResult = validate_date_format("12/31/2023", "MM/DD/YYYY")
    assert_true(result2.is_valid)
    
    sus result3 ValidationResult = validate_date_format("31/12/2023", "DD/MM/YYYY")
    assert_true(result3.is_valid)
    
    // Invalid dates
    sus result4 ValidationResult = validate_date_format("", "YYYY-MM-DD")
    assert_false(result4.is_valid)
    
    sus result5 ValidationResult = validate_date_format("2023-13-01", "YYYY-MM-DD")
    assert_false(result5.is_valid)
    
    sus result6 ValidationResult = validate_date_format("2023-12-32", "YYYY-MM-DD")
    assert_false(result6.is_valid)
}

slay test_numeric_validation() {
    test_start("Numeric Validation")
    
    // Positive validation
    sus result1 ValidationResult = validate_positive(5)
    assert_true(result1.is_valid)
    
    sus result2 ValidationResult = validate_positive(0)
    assert_false(result2.is_valid)
    
    sus result3 ValidationResult = validate_positive(-5)
    assert_false(result3.is_valid)
    
    // Range validation
    sus result4 ValidationResult = validate_range(5, 1, 10)
    assert_true(result4.is_valid)
    
    sus result5 ValidationResult = validate_range(0, 1, 10)
    assert_false(result5.is_valid)
    
    sus result6 ValidationResult = validate_range(15, 1, 10)
    assert_false(result6.is_valid)
}

slay run_validation_tests() {
    vibez.spill("🔍 Running CURSED Validation Module Tests")
    vibez.spill("==========================================")
    
    test_basic_validation()
    test_email_validation()
    test_phone_validation()
    test_url_validation()
    test_credit_card_validation()
    test_date_validation()
    test_numeric_validation()
    
    print_test_summary()
    
    lowkey test_failed > 0 {
        damn 1
    } else {
        damn 0
    }
}

// Auto-run tests when this file is executed
run_validation_tests()
