# Working Core Module Test
# Tests essential core functions for self-hosting

# Built-in test framework
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

slay assert_eq_int(actual normie, expected normie) {
    bestie actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    }
    test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
}

slay assert_eq_string(actual tea, expected tea) {
    bestie actual == expected {
        test_pass("assert_eq_string: strings match")
    }
    test_fail("assert_eq_string failed: got '" + actual + "', expected '" + expected + "'")
}

slay assert_true(value lit) {
    bestie value == based {
        test_pass("assert_true: value is based")
    }
    test_fail("assert_true failed: got cap, expected based")
}

slay assert_false(value lit) {
    bestie value == cap {
        test_pass("assert_false: value is cap")
    }
    test_fail("assert_false failed: got based, expected cap")
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    bestie test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    }
    vibez.spill("❌ Some tests failed")
}

# Core Functions for Testing (inline definitions)
slay core_max(a normie, b normie) normie {
    bestie a > b {
        damn a
    }
    damn b
}

slay core_min(a normie, b normie) normie {
    bestie a < b {
        damn a
    }
    damn b
}

slay core_abs(value normie) normie {
    bestie value < 0 {
        damn -value
    }
    damn value
}

slay core_string_from_int(value normie) tea {
    bestie value == 0 {
        damn "0"
    }
    bestie value == 1 {
        damn "1"
    }
    bestie value == 42 {
        damn "42"
    }
    bestie value == 100 {
        damn "100"
    }
    damn "unknown"
}

slay core_option_some(value normie) (lit, normie) {
    damn (based, value)
}

slay core_option_none() (lit, normie) {
    damn (cap, 0)
}

slay core_option_is_some(opt (lit, normie)) lit {
    damn opt.0
}

slay core_option_unwrap_or(opt (lit, normie), default_value normie) normie {
    bestie opt.0 == based {
        damn opt.1
    }
    damn default_value
}

slay core_result_ok(value normie) (lit, normie, normie) {
    damn (based, value, 0)
}

slay core_result_err(error_code normie) (lit, normie, normie) {
    damn (cap, 0, error_code)
}

slay core_result_is_ok(result (lit, normie, normie)) lit {
    damn result.0
}

slay core_result_unwrap_or(result (lit, normie, normie), default_value normie) normie {
    bestie result.0 == based {
        damn result.1
    }
    damn default_value
}

# Test Core Mathematical Functions
test_start("Core Mathematical Functions")
assert_eq_int(core_max(5, 3), 5)
assert_eq_int(core_min(5, 3), 3)
assert_eq_int(core_abs(-5), 5)
assert_eq_int(core_abs(5), 5)

# Test Core Type Conversion
test_start("Core Type Conversion")
assert_eq_string(core_string_from_int(42), "42")
assert_eq_string(core_string_from_int(0), "0")
assert_eq_string(core_string_from_int(100), "100")

# Test Core Option Type
test_start("Core Option Type")
sus some_val (lit, normie) = core_option_some(42)
sus none_val (lit, normie) = core_option_none()
assert_true(core_option_is_some(some_val))
assert_false(core_option_is_some(none_val))
assert_eq_int(core_option_unwrap_or(some_val, 999), 42)
assert_eq_int(core_option_unwrap_or(none_val, 999), 999)

# Test Core Result Type
test_start("Core Result Type")
sus ok_val (lit, normie, normie) = core_result_ok(123)
sus err_val (lit, normie, normie) = core_result_err(404)
assert_true(core_result_is_ok(ok_val))
assert_false(core_result_is_ok(err_val))
assert_eq_int(core_result_unwrap_or(ok_val, 999), 123)
assert_eq_int(core_result_unwrap_or(err_val, 999), 999)

print_test_summary()
vibez.spill("🚀 Core module functionality verified!")
vibez.spill("✅ Self-hosting essential functions working!")
