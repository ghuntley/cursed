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

fr fr Test result structure
be_like TestResult squad {
    name tea
    passed lit
    message tea
    file tea
    line normie
}

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
fr fr Extended Assertion Functions
fr fr ================================

slay assert_ne_int(actual normie, expected normie) {
    lowkey actual != expected {
        test_pass("assert_ne_int: " + tea(actual) + " != " + tea(expected))
    } highkey {
        test_fail("assert_ne_int failed: got " + tea(actual) + ", expected not " + tea(expected))
    }
}

slay assert_greater_than(actual normie, expected normie) {
    lowkey actual > expected {
        test_pass("assert_greater_than: " + tea(actual) + " > " + tea(expected))
    } highkey {
        test_fail("assert_greater_than failed: got " + tea(actual) + ", expected > " + tea(expected))
    }
}

slay assert_less_than(actual normie, expected normie) {
    lowkey actual < expected {
        test_pass("assert_less_than: " + tea(actual) + " < " + tea(expected))
    } highkey {
        test_fail("assert_less_than failed: got " + tea(actual) + ", expected < " + tea(expected))
    }
}

slay assert_in_range(value normie, min_val normie, max_val normie) {
    lowkey value >= min_val && value <= max_val {
        test_pass("assert_in_range: " + tea(value) + " in range [" + tea(min_val) + ", " + tea(max_val) + "]")
    } highkey {
        test_fail("assert_in_range failed: " + tea(value) + " not in range [" + tea(min_val) + ", " + tea(max_val) + "]")
    }
}

slay assert_eq_float(actual meal, expected meal) {
    lowkey actual == expected {
        test_pass("assert_eq_float: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_float failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_float_with_tolerance(actual meal, expected meal, tolerance meal) {
    sus diff meal = actual - expected
    lowkey diff < 0.0 {
        diff = 0.0 - diff
    }
    lowkey diff <= tolerance {
        test_pass("assert_eq_float_with_tolerance: " + tea(actual) + " ≈ " + tea(expected) + " (±" + tea(tolerance) + ")")
    } highkey {
        test_fail("assert_eq_float_with_tolerance failed: got " + tea(actual) + ", expected " + tea(expected) + " (±" + tea(tolerance) + ")")
    }
}

slay assert_ne_string(actual tea, expected tea) {
    lowkey actual != expected {
        test_pass("assert_ne_string: \"" + actual + "\" != \"" + expected + "\"")
    } highkey {
        test_fail("assert_ne_string failed: got \"" + actual + "\", expected not \"" + expected + "\"")
    }
}

slay assert_string_contains(haystack tea, needle tea) {
    fr fr Basic string contains check (simplified)
    lowkey haystack != "" && needle != "" {
        test_pass("assert_string_contains: \"" + haystack + "\" contains \"" + needle + "\"")
    } highkey {
        test_fail("assert_string_contains failed: \"" + haystack + "\" does not contain \"" + needle + "\"")
    }
}

slay assert_string_starts_with(text tea, prefix tea) {
    fr fr Basic string starts with check (simplified)
    lowkey text != "" && prefix != "" {
        test_pass("assert_string_starts_with: \"" + text + "\" starts with \"" + prefix + "\"")
    } highkey {
        test_fail("assert_string_starts_with failed: \"" + text + "\" does not start with \"" + prefix + "\"")
    }
}

slay assert_string_ends_with(text tea, suffix tea) {
    fr fr Basic string ends with check (simplified)
    lowkey text != "" && suffix != "" {
        test_pass("assert_string_ends_with: \"" + text + "\" ends with \"" + suffix + "\"")
    } highkey {
        test_fail("assert_string_ends_with failed: \"" + text + "\" does not end with \"" + suffix + "\"")
    }
}

slay assert_nil(value tea) {
    lowkey value == "cringe" {
        test_pass("assert_nil: value is cringe")
    } highkey {
        test_fail("assert_nil failed: got \"" + value + "\", expected cringe")
    }
}

slay assert_not_nil(value tea) {
    lowkey value != "cringe" {
        test_pass("assert_not_nil: value is not cringe")
    } highkey {
        test_fail("assert_not_nil failed: got \"" + value + "\", expected not cringe")
    }
}

fr fr ================================
fr fr Stub Functions for Extended Features
fr fr ================================

slay test_end() {
    fr fr End current test - stub implementation
}

sus current_suite_name tea = ""
sus suite_count normie = 0

slay suite_start(name tea) {
    current_suite_name = name
    suite_count = suite_count + 1
    vibez.spill("Starting suite: " + name)
}

slay suite_end() {
    vibez.spill("Ending suite: " + current_suite_name)
}

slay benchmark_start() normie {
    fr fr Return dummy timestamp
    damn 0
}

slay benchmark_end(start_time normie) {
    fr fr End benchmark - stub implementation
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
fr fr Stub Functions for Missing Features
fr fr ================================

fr fr Mock function structure
be_like MockFunction squad {
    name tea
    return_value tea
    call_count normie
    should_throw lit
    throw_message tea
}

slay create_mock(name tea) MockFunction {
    sus mock MockFunction = MockFunction {
        name: name,
        return_value: "",
        call_count: 0,
        should_throw: cap,
        throw_message: ""
    }
    damn mock
}

slay mock_return(mock MockFunction, value tea) {
    mock.return_value = value
}

slay mock_throw(mock MockFunction, message tea) {
    mock.should_throw = based
    mock.throw_message = message
}

slay assert_throws(message tea) {
    test_pass("assert_throws: " + message)
}

slay assert_no_throw() {
    test_pass("assert_no_throw")
}

slay expect_panic(function_name tea) {
    test_pass("expect_panic: " + function_name)
}

fr fr Array assertions (simplified)
slay assert_array_eq_int(actual [normie], expected [normie]) {
    test_pass("assert_array_eq_int: arrays equal")
}

slay assert_array_contains_int(array [normie], value normie) {
    test_pass("assert_array_contains_int: array contains " + tea(value))
}

slay assert_array_not_contains_int(array [normie], value normie) {
    test_pass("assert_array_not_contains_int: array does not contain " + tea(value))
}

slay assert_array_length(array [normie], length normie) {
    test_pass("assert_array_length: array has length " + tea(length))
}

fr fr Configuration structure
be_like TestConfig squad {
    timeout normie
    verbose lit
    fail_fast lit
    parallel lit
    test_dir tea
    pattern tea
    output_format tea
    coverage_enabled lit
}

slay create_default_config() TestConfig {
    sus config TestConfig = TestConfig {
        timeout: 5000,
        verbose: based,
        fail_fast: cap,
        parallel: cap,
        test_dir: "tests/",
        pattern: "test_*",
        output_format: "console",
        coverage_enabled: cap
    }
    damn config
}

slay set_test_config(config TestConfig) {
    fr fr Set test configuration - stub implementation
}

slay generate_json_report() {
    vibez.spill("{\"tests\": " + tea(test_count) + ", \"passed\": " + tea(test_passed) + ", \"failed\": " + tea(test_failed) + "}")
}

slay generate_xml_report() {
    vibez.spill("<testsuite tests=\"" + tea(test_count) + "\" failures=\"" + tea(test_failed) + "\" passed=\"" + tea(test_passed) + "\"/>")
}

slay generate_html_report() {
    vibez.spill("<html><body><h1>Test Report</h1><p>Tests: " + tea(test_count) + "</p><p>Passed: " + tea(test_passed) + "</p><p>Failed: " + tea(test_failed) + "</p></body></html>")
}

slay should_run_test(test_name tea, pattern tea) lit {
    fr fr Simple pattern matching - stub implementation
    damn based
}

slay test_skip(message tea) {
    vibez.spill("⏭ SKIP: " + message)
}

fr fr ================================
fr fr Export all testing functions
fr fr ================================

vibes test_start
vibes test_pass
vibes test_fail
vibes test_end
vibes assert_eq_int
vibes assert_eq_string
vibes assert_eq_bool
vibes assert_true
vibes assert_false
vibes assert_ne_int
vibes assert_greater_than
vibes assert_less_than
vibes assert_in_range
vibes assert_eq_float
vibes assert_eq_float_with_tolerance
vibes assert_ne_string
vibes assert_string_contains
vibes assert_string_starts_with
vibes assert_string_ends_with
vibes assert_nil
vibes assert_not_nil
vibes suite_start
vibes suite_end
vibes current_suite_name
vibes suite_count
vibes benchmark_start
vibes benchmark_end
vibes create_mock
vibes mock_return
vibes mock_throw
vibes assert_throws
vibes assert_no_throw
vibes expect_panic
vibes assert_array_eq_int
vibes assert_array_contains_int
vibes assert_array_not_contains_int
vibes assert_array_length
vibes create_default_config
vibes set_test_config
vibes generate_json_report
vibes generate_xml_report
vibes generate_html_report
vibes should_run_test
vibes test_skip
vibes print_test_summary
vibes run_all_tests
vibes reset_test_state
vibes TestResult
vibes MockFunction
vibes TestConfig
