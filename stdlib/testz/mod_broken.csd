# CURSED Testing Framework (testz v2.0) - Complete Enterprise Testing Suite
# Comprehensive assertion library with all critical functions for stdlib testing

# Export all test functions for visibility
vibes test_start, test_pass, test_fail, test_end
vibes assert_eq_int, assert_ne_int, assert_gt_int, assert_lt_int, assert_ge_int, assert_le_int
vibes assert_eq_string, assert_ne_string
vibes assert_true, assert_false
vibes assert_eq_float, assert_in_range, assert_nil, assert_not_nil
vibes print_test_summary, all_tests_passed, get_test_results, get_pass_count, get_total_count, reset_test_state

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

# Integer assertions
slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("int equality: " + tea(actual) + " == " + tea(expected))
    } else {
        test_fail("int equality: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_ne_int(actual normie, expected normie) {
    lowkey actual != expected {
        test_pass("int inequality: " + tea(actual) + " != " + tea(expected))
    } else {
        test_fail("int inequality: got " + tea(actual) + ", expected not " + tea(expected))
    }
}

slay assert_gt_int(actual normie, expected normie) {
    lowkey actual > expected {
        test_pass("int greater than: " + tea(actual) + " > " + tea(expected))
    } else {
        test_fail("int greater than: got " + tea(actual) + ", expected > " + tea(expected))
    }
}

slay assert_lt_int(actual normie, expected normie) {
    lowkey actual < expected {
        test_pass("int less than: " + tea(actual) + " < " + tea(expected))
    } else {
        test_fail("int less than: got " + tea(actual) + ", expected < " + tea(expected))
    }
}

slay assert_ge_int(actual normie, expected normie) {
    lowkey actual >= expected {
        test_pass("int greater equal: " + tea(actual) + " >= " + tea(expected))
    } else {
        test_fail("int greater equal: got " + tea(actual) + ", expected >= " + tea(expected))
    }
}

slay assert_le_int(actual normie, expected normie) {
    lowkey actual <= expected {
        test_pass("int less equal: " + tea(actual) + " <= " + tea(expected))
    } else {
        test_fail("int less equal: got " + tea(actual) + ", expected <= " + tea(expected))
    }
}

# String assertions
slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("string equality: \"" + actual + "\" == \"" + expected + "\"")
    } else {
        test_fail("string equality: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_ne_string(actual tea, expected tea) {
    lowkey actual != expected {
        test_pass("string inequality: \"" + actual + "\" != \"" + expected + "\"")
    } else {
        test_fail("string inequality: got \"" + actual + "\", expected not \"" + expected + "\"")
    }
}

# Boolean assertions
slay assert_true(condition lit) {
    lowkey condition == based {
        test_pass("assert_true: condition is based")
    } else {
        test_fail("assert_true: got " + tea(condition) + ", expected based")
    }
}

slay assert_false(condition lit) {
    lowkey condition == cap {
        test_pass("assert_false: condition is cap")
    } else {
        test_fail("assert_false: got " + tea(condition) + ", expected cap")
    }
}

# Float assertions (with tolerance)
slay assert_eq_float(actual meal, expected meal) {
    sus diff meal = actual - expected
    lowkey diff < 0.0001 && diff > -0.0001 {
        test_pass("float equality: " + tea(actual) + " ≈ " + tea(expected))
    } else {
        test_fail("float equality: got " + tea(actual) + ", expected " + tea(expected))
    }
}

# Range assertions
slay assert_in_range(value normie, min_val normie, max_val normie) {
    lowkey value >= min_val && value <= max_val {
        test_pass("in range: " + tea(value) + " in [" + tea(min_val) + ", " + tea(max_val) + "]")
    } else {
        test_fail("in range: " + tea(value) + " not in [" + tea(min_val) + ", " + tea(max_val) + "]")
    }
}

# Nil assertions
slay assert_nil(value tea) {
    lowkey value == "" {
        test_pass("assert_nil: value is empty/nil")
    } else {
        test_fail("assert_nil: got \"" + value + "\", expected nil")
    }
}

slay assert_not_nil(value tea) {
    lowkey value != "" {
        test_pass("assert_not_nil: value is not empty/nil")
    } else {
        test_fail("assert_not_nil: value is empty/nil")
    }
}

# Test end function
slay test_end() {
    lowkey current_test_passed == based {
        passed_tests = passed_tests + 1
        vibez.spill("✓ PASSED: " + current_test_name)
    } else {
        failed_tests = failed_tests + 1
        vibez.spill("✗ FAILED: " + current_test_name)
    }
}

# Print comprehensive test summary
slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== Test Summary ===")
    vibez.spill("Total: " + tea(total_tests))
    vibez.spill("Passed: " + tea(passed_tests))
    vibez.spill("Failed: " + tea(failed_tests))
    
    lowkey failed_tests == 0 {
        vibez.spill("🎉 ALL TESTS PASSED!")
    } else {
        vibez.spill("❌ " + tea(failed_tests) + " TESTS FAILED")
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

slay get_pass_count() normie {
    damn passed_tests
}

slay get_total_count() normie {
    damn total_tests
}

slay reset_test_state() {
    current_test_name = ""
    total_tests = 0
    passed_tests = 0
    failed_tests = 0
    current_test_passed = based
}
