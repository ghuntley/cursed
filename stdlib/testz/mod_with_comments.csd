# CURSED Testing Framework (testz v2.0) - Simple Version

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
    vibez.spill("Test: " + name)
}

# Test pass helper
slay test_pass(message tea) {
    vibez.spill("PASS: " + message)
}

# Test fail helper
slay test_fail(message tea) {
    vibez.spill("FAIL: " + message)
    current_test_passed = cap
}

# Boolean assertions
slay assert_true(condition lit) {
    lowkey condition == based {
        test_pass("assert_true: condition is based")
    } else {
        test_fail("assert_true: condition is not based")
    }
}

slay assert_false(condition lit) {
    lowkey condition == cap {
        test_pass("assert_false: condition is cap")
    } else {
        test_fail("assert_false: condition is not cap")
    }
}

# Print test summary
slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== Test Summary ===")
    vibez.spill("Total tests executed")
    
    lowkey failed_tests == 0 {
        vibez.spill("🎉 ALL TESTS PASSED!")
    } else {
        vibez.spill("❌ SOME TESTS FAILED")
    }
    vibez.spill("==================")
}
