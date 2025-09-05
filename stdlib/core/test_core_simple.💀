fr fr Simple Core Module Test
yeet "core"

fr fr Simple test framework
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
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: strings match")
    } highkey {
        test_fail("assert_eq_string failed: got '" + actual + "', expected '" + expected + "'")
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } highkey {
        test_fail("assert_true failed: got cap, expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_pass("assert_false: value is cap")
    } highkey {
        test_fail("assert_false failed: got based, expected cap")
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
    } highkey {
        vibez.spill("❌ Some tests failed")
    }
}

fr fr Test type conversions
test_start("Type Conversions")
assert_eq_int(int_from_bool(based), 1)
assert_eq_int(int_from_bool(cap), 0)
assert_eq_string(string_from_int(42), "42")
assert_eq_string(string_from_int(0), "0")
assert_true(lit_from_int(1))
assert_false(lit_from_int(0))

fr fr Test Option type
test_start("Option Type")
sus some_val (lit, normie) = option_some(42)
sus none_val (lit, normie) = option_none()
assert_true(option_is_some(some_val))
assert_false(option_is_some(none_val))
assert_eq_int(option_unwrap(some_val), 42)
assert_eq_int(option_unwrap_or(none_val, 100), 100)

fr fr Test Result type
test_start("Result Type")
sus ok_val (lit, normie, normie) = result_ok(123)
sus err_val (lit, normie, normie) = result_err(404)
assert_true(result_is_ok(ok_val))
assert_false(result_is_ok(err_val))
assert_eq_int(result_unwrap(ok_val), 123)
assert_eq_int(result_unwrap_or(err_val, 999), 999)

fr fr Test math utilities
test_start("Math Utilities")
assert_eq_int(max(5, 3), 5)
assert_eq_int(min(5, 3), 3)
assert_eq_int(abs(-5), 5)
assert_eq_int(pow(2, 3), 8)
assert_eq_int(sqrt(16), 4)

fr fr Test string utilities
test_start("String Utilities")
assert_eq_int(string_len("hello"), 5)
assert_eq_string(string_concat("hello", "world"), "helloworld")
assert_true(string_contains("hello world", "world"))
assert_true(string_starts_with("hello world", "hello"))
assert_true(string_ends_with("hello world", "world"))
assert_eq_string(string_trim("  hello  "), "hello")

fr fr Test boolean utilities
test_start("Boolean Utilities")
assert_true(not(cap))
assert_false(not(based))
assert_true(and(based, based))
assert_false(and(based, cap))
assert_true(or(based, cap))
assert_false(or(cap, cap))

fr fr Test compiler utilities
test_start("Compiler Utilities")
assert_eq_int(token_type_identifier(), 1)
assert_eq_int(error_code_syntax(), 1000)
assert_eq_int(hash_string("test"), 200)

print_test_summary()
vibez.spill("🚀 Core module testing complete!")
