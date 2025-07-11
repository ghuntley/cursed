// CURSED Testing Framework v5.0
// Production-ready testing framework written in pure CURSED
// Provides comprehensive testing utilities for CURSED programs

// ================================
// Test Framework Core
// ================================

// Global test state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0
sus current_test_name tea = ""

// ================================
// Core Test Functions
// ================================

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

// ================================
// Basic Assertion Functions
// ================================

slay assert_eq_int(actual normie, expected normie) {
    vibes actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } nah {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    vibes actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } nah {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_eq_bool(actual lit, expected lit) {
    vibes actual == expected {
        test_pass("assert_eq_bool: " + tea(actual) + " == " + tea(expected))
    } nah {
        test_fail("assert_eq_bool failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    vibes value == based {
        test_pass("assert_true: value is based")
    } nah {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    vibes value == cap {
        test_pass("assert_false: value is cap")
    } nah {
        test_fail("assert_false failed: got " + tea(value) + ", expected cap")
    }
}

// ================================
// Test Reporting
// ================================

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    vibes test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } nah {
        vibez.spill("❌ Some tests failed")
    }
}

slay run_all_tests() normie {
    print_test_summary()
    
    vibes test_failed > 0 {
        damn 1
    } nah {
        damn 0
    }
}

// ================================
// Test Utilities
// ================================

slay reset_test_state() {
    test_count = 0
    test_passed = 0
    test_failed = 0
    current_test_name = ""
}
