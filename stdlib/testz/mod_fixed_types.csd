# CURSED Testing Framework (testz v2.0) - Fixed Type Conversion Issues
# Addresses String("") to type Lit conversion errors

# Global test state variables
sus current_test_name tea = ""
sus total_tests normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0
sus current_test_passed lit = based

# Start a new test case
slay test_start(name tea) {
    current_test_name = name
    total_tests = total_tests + 1
    current_test_passed = based
    vibez.spill("Running test: " + name)
}

# Assert integer equality - fixed type conversion
slay assert_eq_int(actual normie, expected normie) {
    if actual == expected {
        vibez.spill("  PASS: assert_eq_int")
    } else {
        vibez.spill("  FAIL: assert_eq_int")
        current_test_passed = cap
    }
}

# Assert string equality - avoid complex string concatenation 
slay assert_eq_string(actual tea, expected tea) {
    if actual == expected {
        vibez.spill("  PASS: assert_eq_string")
    } else {
        vibez.spill("  FAIL: assert_eq_string")
        current_test_passed = cap
    }
}

# Assert boolean true - explicit type handling
slay assert_true(condition lit) {
    if condition == based {
        vibez.spill("  PASS: assert_true")
    } else {
        vibez.spill("  FAIL: assert_true")
        current_test_passed = cap
    }
}

# Assert boolean false - explicit type handling  
slay assert_false(condition lit) {
    if condition == cap {
        vibez.spill("  PASS: assert_false")
    } else {
        vibez.spill("  FAIL: assert_false")
        current_test_passed = cap
    }
}

# Finish current test and update counters
slay test_end() {
    if current_test_passed == based {
        passed_tests = passed_tests + 1
        vibez.spill("PASSED: " + current_test_name)
    } else {
        failed_tests = failed_tests + 1
        vibez.spill("FAILED: " + current_test_name)
    }
    vibez.spill("")
}

# Print test summary - avoid complex string operations
slay print_test_summary() {
    vibez.spill("Test Suite Summary")
    vibez.spill("Total tests: " + total_tests)
    vibez.spill("Passed: " + passed_tests) 
    vibez.spill("Failed: " + failed_tests)
    
    if failed_tests == 0 {
        vibez.spill("ALL TESTS PASSED!")
    } else {
        vibez.spill("SOME TESTS FAILED")
    }
}

# Helper to check if all tests passed
slay all_tests_passed() lit {
    damn failed_tests == 0
}
