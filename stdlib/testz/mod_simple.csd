fr fr CURSED Testing Framework
fr fr Provides comprehensive testing utilities for CURSED programs

fr fr ================================
fr fr Test Framework Core
fr fr ================================

fr fr Global test state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0
sus current_test_name tea = ""

fr fr ================================
fr fr Core Test Functions
fr fr ================================

slay test_start(name tea) {
    test_count = test_count + 1
    current_test_name = name
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

fr fr ================================
fr fr Basic Assertion Functions
fr fr ================================

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_eq_bool(actual lit, expected lit) {
    lowkey actual == expected {
        test_pass("assert_eq_bool: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_bool failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } highkey {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_pass("assert_false: value is cap")
    } highkey {
        test_fail("assert_false failed: got " + tea(value) + ", expected cap")
    }
}

fr fr ================================
fr fr Test Reporting
fr fr ================================

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ Some tests failed")
    }
}

slay run_all_tests() normie {
    print_test_summary()
    
    lowkey test_failed > 0 {
        damn 1
    } highkey {
        damn 0
    }
}

fr fr ================================
fr fr Test Utilities
fr fr ================================

slay reset_test_state() {
    test_count = 0
    test_passed = 0
    test_failed = 0
    current_test_name = ""
}

fr fr ================================
fr fr Export all testing functions
fr fr ================================

vibes test_start
vibes test_pass
vibes test_fail
vibes assert_eq_int
vibes assert_eq_string
vibes assert_eq_bool
vibes assert_true
vibes assert_false
vibes print_test_summary
vibes run_all_tests
vibes reset_test_state
