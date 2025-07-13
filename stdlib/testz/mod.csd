# CURSED Testing Framework (testz v2.0) - FIXED TYPE CONVERSION ISSUES
# Addresses String("") to type Lit conversion errors by simplifying type operations

# Global test state variables
sus current_test_name tea = ""
sus total_tests normie = 0
sus passed_tests normie = 0
sus failed_tests normie = 0
sus current_test_passed lit = based

# Start a new test case - simplified to avoid type conversion issues
slay test_start(name tea) {
    current_test_name = name
    total_tests = total_tests + 1
    current_test_passed = based
    vibez.spill("Test: " + name)
}

# Assert integer equality - avoid problematic type conversions
slay assert_eq_int(actual normie, expected normie) {
    bestie actual == expected {
        vibez.spill("PASS: int equality")
    } else {
        vibez.spill("FAIL: int equality")
        current_test_passed = cap
    }
}

# Assert string equality - simplified to prevent String-to-Lit conversion
slay assert_eq_string(actual tea, expected tea) {
    bestie actual == expected {
        vibez.spill("PASS: string equality")
    } else {
        vibez.spill("FAIL: string equality")
        current_test_passed = cap
    }
}

# Assert boolean true - explicit boolean type handling
slay assert_true(condition lit) {
    bestie condition == based {
        vibez.spill("PASS: assert_true")
    } else {
        vibez.spill("FAIL: assert_true")
        current_test_passed = cap
    }
}

# Assert boolean false - explicit boolean type handling
slay assert_false(condition lit) {
    bestie condition == cap {
        vibez.spill("PASS: assert_false")
    } else {
        vibez.spill("FAIL: assert_false")
        current_test_passed = cap
    }
}

# Test end function
slay test_end() {
    bestie current_test_passed == based {
        passed_tests = passed_tests + 1
        vibez.spill("PASSED")
    } else {
        failed_tests = failed_tests + 1
        vibez.spill("FAILED")
    }
}

# Print test summary - avoid complex string+int operations that cause type errors
slay print_test_summary() {
    vibez.spill("=== Test Summary ===")
    vibez.spill("Tests completed")
    
    bestie failed_tests == 0 {
        vibez.spill("ALL TESTS PASSED!")
    } else {
        vibez.spill("SOME TESTS FAILED")
    }
    vibez.spill("==================")
}

# Helper functions
slay all_tests_passed() lit {
    damn failed_tests == 0
}

slay get_test_results() normie {
    damn failed_tests
}

slay reset_test_state() {
    current_test_name = ""
    total_tests = 0
    passed_tests = 0
    failed_tests = 0
    current_test_passed = based
}
