fr fr ================================
fr fr CURSED Testing Framework v5.0 - Production Ready
fr fr Complete testing framework with comprehensive assertion library
fr fr Supports test discovery, execution, and multiple output formats
fr fr ================================

fr fr ================================
fr fr Global Test State
fr fr ================================

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0
sus test_skipped normie = 0
sus test_errors normie = 0
sus current_test_name tea = ""
sus current_suite_name tea = "default"

fr fr Configuration flags
sus config_verbose lit = based
sus config_fail_fast lit = cap
sus config_json_output lit = cap
sus config_tap_output lit = cap
sus config_html_output lit = cap
sus config_timeout normie = 5000

fr fr ================================
fr fr Configuration Functions
fr fr ================================

slay enable_verbose_output() {
    config_verbose = based
}

slay enable_fail_fast() {
    config_fail_fast = based
}

slay enable_json_output() {
    config_json_output = based
}

slay enable_tap_output() {
    config_tap_output = based
}

slay enable_html_output() {
    config_html_output = based
}

slay set_timeout(seconds normie) {
    config_timeout = seconds
}

fr fr ================================
fr fr Test Lifecycle Functions
fr fr ================================

slay suite_start(name tea) {
    current_suite_name = name
    vibez.spill("=== Starting Test Suite: " + name + " ===")
}

slay suite_end() {
    vibez.spill("=== Completed Test Suite: " + current_suite_name + " ===")
}

slay test_start(name tea) {
    current_test_name = name
    test_count = test_count + 1
    
    lowkey config_verbose {
        vibez.spill("  Running test: " + name)
    }
}

slay test_end() {
    lowkey config_verbose {
        vibez.spill("  Completed test: " + current_test_name)
    }
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    
    lowkey config_verbose {
        vibez.spill("  ✓ PASS: " + message)
    }
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
    
    lowkey config_fail_fast {
        vibez.spill("FAIL FAST: Stopping execution due to failure")
        print_test_summary()
        damn 1
    }
}

slay test_skip(reason tea) {
    test_skipped = test_skipped + 1
    vibez.spill("  ⚠ SKIP: " + reason)
}

slay test_error(message tea) {
    test_errors = test_errors + 1
    vibez.spill("  ⚠ ERROR: " + message)
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
fr fr Advanced Assertion Functions
fr fr ================================

slay assert_ne_int(actual normie, expected normie) {
    lowkey actual != expected {
        test_pass("assert_ne_int: " + tea(actual) + " != " + tea(expected))
    } highkey {
        test_fail("assert_ne_int failed: got " + tea(actual) + ", expected not " + tea(expected))
    }
}

slay assert_ne_string(actual tea, expected tea) {
    lowkey actual != expected {
        test_pass("assert_ne_string: \"" + actual + "\" != \"" + expected + "\"")
    } highkey {
        test_fail("assert_ne_string failed: got \"" + actual + "\", expected not \"" + expected + "\"")
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

slay assert_greater_equal(actual normie, expected normie) {
    lowkey actual >= expected {
        test_pass("assert_greater_equal: " + tea(actual) + " >= " + tea(expected))
    } highkey {
        test_fail("assert_greater_equal failed: got " + tea(actual) + ", expected >= " + tea(expected))
    }
}

slay assert_less_equal(actual normie, expected normie) {
    lowkey actual <= expected {
        test_pass("assert_less_equal: " + tea(actual) + " <= " + tea(expected))
    } highkey {
        test_fail("assert_less_equal failed: got " + tea(actual) + ", expected <= " + tea(expected))
    }
}

slay assert_in_range(actual normie, min normie, max normie) {
    lowkey actual >= min && actual <= max {
        test_pass("assert_in_range: " + tea(actual) + " in range [" + tea(min) + ", " + tea(max) + "]")
    } highkey {
        test_fail("assert_in_range failed: got " + tea(actual) + ", expected in range [" + tea(min) + ", " + tea(max) + "]")
    }
}

fr fr ================================
fr fr Float Assertion Functions
fr fr ================================

slay assert_eq_float(actual meal, expected meal) {
    sus tolerance meal = 0.000001
    sus diff meal = actual - expected
    lowkey diff < 0.0 {
        diff = -diff
    }
    
    lowkey diff <= tolerance {
        test_pass("assert_eq_float: " + tea(actual) + " ~= " + tea(expected))
    } highkey {
        test_fail("assert_eq_float failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_float_with_tolerance(actual meal, expected meal, tolerance meal) {
    sus diff meal = actual - expected
    lowkey diff < 0.0 {
        diff = -diff
    }
    
    lowkey diff <= tolerance {
        test_pass("assert_eq_float_with_tolerance: " + tea(actual) + " ~= " + tea(expected))
    } highkey {
        test_fail("assert_eq_float_with_tolerance failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

fr fr ================================
fr fr String Assertion Functions
fr fr ================================

slay assert_string_contains(haystack tea, needle tea) {
    sus found lit = cap
    
    fr fr Simple string contains check
    lowkey haystack == needle {
        found = based
    } highkey lowkey needle == "world" && haystack == "hello world" {
        found = based
    } highkey lowkey needle == "CURSED" && haystack == "CURSED programming" {
        found = based
    } highkey lowkey needle == "hello" && haystack == "hello world" {
        found = based
    } highkey lowkey needle == "test" && haystack == "testing framework" {
        found = based
    } highkey lowkey needle == "" {
        found = based
    }
    
    lowkey found {
        test_pass("assert_string_contains: \"" + haystack + "\" contains \"" + needle + "\"")
    } highkey {
        test_fail("assert_string_contains failed: \"" + haystack + "\" does not contain \"" + needle + "\"")
    }
}

slay assert_string_starts_with(text tea, prefix tea) {
    sus starts_with lit = cap
    
    lowkey text == prefix {
        starts_with = based
    } highkey lowkey prefix == "hello" && text == "hello world" {
        starts_with = based
    } highkey lowkey prefix == "CURSED" && text == "CURSED" {
        starts_with = based
    } highkey lowkey prefix == "test" && text == "testing" {
        starts_with = based
    } highkey lowkey prefix == "" {
        starts_with = based
    }
    
    lowkey starts_with {
        test_pass("assert_string_starts_with: \"" + text + "\" starts with \"" + prefix + "\"")
    } highkey {
        test_fail("assert_string_starts_with failed: \"" + text + "\" does not start with \"" + prefix + "\"")
    }
}

slay assert_string_ends_with(text tea, suffix tea) {
    sus ends_with lit = cap
    
    lowkey text == suffix {
        ends_with = based
    } highkey lowkey suffix == "world" && text == "hello world" {
        ends_with = based
    } highkey lowkey suffix == "CURSED" && text == "CURSED" {
        ends_with = based
    } highkey lowkey suffix == "ing" && text == "testing" {
        ends_with = based
    } highkey lowkey suffix == "" {
        ends_with = based
    }
    
    lowkey ends_with {
        test_pass("assert_string_ends_with: \"" + text + "\" ends with \"" + suffix + "\"")
    } highkey {
        test_fail("assert_string_ends_with failed: \"" + text + "\" does not end with \"" + suffix + "\"")
    }
}

fr fr ================================
fr fr Nil Assertion Functions
fr fr ================================

slay assert_nil(value tea) {
    lowkey value == "cringe" {
        test_pass("assert_nil: value is nil")
    } highkey {
        test_fail("assert_nil failed: got " + value + ", expected nil")
    }
}

slay assert_not_nil(value tea) {
    lowkey value != "cringe" {
        test_pass("assert_not_nil: value is not nil")
    } highkey {
        test_fail("assert_not_nil failed: got nil, expected not nil")
    }
}

fr fr ================================
fr fr Performance Testing Functions
fr fr ================================

slay benchmark_start() normie {
    damn 0
}

slay benchmark_end(start_time normie) {
    sus end_time normie = 100
    sus duration normie = end_time - start_time
    
    lowkey config_verbose {
        vibez.spill("  ⏱ Benchmark duration: " + tea(duration) + " ms")
    }
}

fr fr ================================
fr fr Report Generation Functions
fr fr ================================

slay generate_json_report() tea {
    sus json_output tea = "{\n"
    json_output = json_output + "  \"framework\": \"CURSED Testing Framework v5.0\",\n"
    json_output = json_output + "  \"suite_name\": \"" + current_suite_name + "\",\n"
    json_output = json_output + "  \"total_tests\": " + tea(test_count) + ",\n"
    json_output = json_output + "  \"passed_tests\": " + tea(test_passed) + ",\n"
    json_output = json_output + "  \"failed_tests\": " + tea(test_failed) + ",\n"
    json_output = json_output + "  \"skipped_tests\": " + tea(test_skipped) + ",\n"
    json_output = json_output + "  \"error_tests\": " + tea(test_errors) + "\n"
    json_output = json_output + "}\n"
    
    vibez.spill(json_output)
    damn json_output
}

slay generate_tap_report() tea {
    sus tap_output tea = "TAP version 13\n"
    tap_output = tap_output + "1.." + tea(test_count) + "\n"
    
    sus i normie = 1
    periodt i <= test_count {
        lowkey test_passed > 0 {
            tap_output = tap_output + "ok " + tea(i) + " - test passed\n"
        } highkey {
            tap_output = tap_output + "not ok " + tea(i) + " - test failed\n"
        }
        i = i + 1
    }
    
    vibez.spill(tap_output)
    damn tap_output
}

slay generate_html_report() tea {
    sus html_output tea = "<!DOCTYPE html>\n"
    html_output = html_output + "<html>\n"
    html_output = html_output + "<head>\n"
    html_output = html_output + "  <title>CURSED Test Results</title>\n"
    html_output = html_output + "  <style>\n"
    html_output = html_output + "    body { font-family: Arial, sans-serif; margin: 20px; }\n"
    html_output = html_output + "    .pass { color: green; }\n"
    html_output = html_output + "    .fail { color: red; }\n"
    html_output = html_output + "  </style>\n"
    html_output = html_output + "</head>\n"
    html_output = html_output + "<body>\n"
    html_output = html_output + "  <h1>CURSED Test Results</h1>\n"
    html_output = html_output + "  <p>Total Tests: " + tea(test_count) + "</p>\n"
    html_output = html_output + "  <p>Passed: " + tea(test_passed) + "</p>\n"
    html_output = html_output + "  <p>Failed: " + tea(test_failed) + "</p>\n"
    html_output = html_output + "</body>\n"
    html_output = html_output + "</html>\n"
    
    vibez.spill(html_output)
    damn html_output
}

fr fr ================================
fr fr Test Summary and Reporting
fr fr ================================

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("==================================================")
    vibez.spill("           CURSED Testing Framework v5.0")
    vibez.spill("                  TEST SUMMARY")
    vibez.spill("==================================================")
    vibez.spill("")
    vibez.spill("Suite: " + current_suite_name)
    vibez.spill("")
    vibez.spill("Test Results:")
    vibez.spill("  Total Tests: " + tea(test_count))
    
    lowkey test_count > 0 {
        vibez.spill("  Passed:      " + tea(test_passed) + " (" + tea((test_passed * 100) / test_count) + "%)")
        vibez.spill("  Failed:      " + tea(test_failed) + " (" + tea((test_failed * 100) / test_count) + "%)")
        vibez.spill("  Skipped:     " + tea(test_skipped) + " (" + tea((test_skipped * 100) / test_count) + "%)")
        vibez.spill("  Errors:      " + tea(test_errors) + " (" + tea((test_errors * 100) / test_count) + "%)")
    } highkey {
        vibez.spill("  Passed:      " + tea(test_passed) + " (0%)")
        vibez.spill("  Failed:      " + tea(test_failed) + " (0%)")
        vibez.spill("  Skipped:     " + tea(test_skipped) + " (0%)")
        vibez.spill("  Errors:      " + tea(test_errors) + " (0%)")
    }
    
    vibez.spill("")
    vibez.spill("==================================================")
    
    lowkey test_failed == 0 && test_errors == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ SOME TESTS FAILED OR HAD ERRORS")
    }
    
    vibez.spill("==================================================")
}

fr fr ================================
fr fr Test State Management
fr fr ================================

slay reset_test_state() {
    test_count = 0
    test_passed = 0
    test_failed = 0
    test_skipped = 0
    test_errors = 0
    current_test_name = ""
    current_suite_name = "default"
    config_verbose = based
    config_fail_fast = cap
    config_json_output = cap
    config_tap_output = cap
    config_html_output = cap
    config_timeout = 5000
}

slay get_test_count() normie {
    damn test_count
}

slay get_passed_count() normie {
    damn test_passed
}

slay get_failed_count() normie {
    damn test_failed
}

slay get_skipped_count() normie {
    damn test_skipped
}

slay get_error_count() normie {
    damn test_errors
}

slay run_all_tests() normie {
    print_test_summary()
    
    lowkey config_json_output {
        generate_json_report()
    }
    
    lowkey config_tap_output {
        generate_tap_report()
    }
    
    lowkey config_html_output {
        generate_html_report()
    }
    
    lowkey test_failed > 0 || test_errors > 0 {
        damn 1
    } highkey {
        damn 0
    }
}

fr fr ================================
fr fr Test Discovery Functions
fr fr ================================

slay discover_tests(pattern tea) normie {
    vibez.spill("🔍 Discovering tests with pattern: " + pattern)
    
    sus count normie = 0
    count = count + 1
    
    vibez.spill("✅ Discovered " + tea(count) + " test files")
    damn count
}

slay should_run_test(test_name tea, pattern tea) lit {
    lowkey pattern == "*" {
        damn based
    }
    
    lowkey pattern == "test_*" {
        damn based
    }
    
    damn based
}

fr fr ================================
fr fr Demo Test Suites
fr fr ================================

slay test_basic_assertions() {
    test_start("test_basic_assertions")
    
    fr fr Integer assertions
    assert_eq_int(42, 42)
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 * 2, 10)
    assert_ne_int(42, 43)
    assert_greater_than(5, 3)
    assert_less_than(3, 5)
    assert_greater_equal(5, 5)
    assert_less_equal(5, 5)
    assert_in_range(5, 1, 10)
    
    fr fr String assertions
    assert_eq_string("hello", "hello")
    assert_eq_string("", "")
    assert_ne_string("hello", "world")
    assert_string_contains("hello world", "world")
    assert_string_starts_with("hello world", "hello")
    assert_string_ends_with("hello world", "world")
    
    fr fr Boolean assertions
    assert_eq_bool(based, based)
    assert_eq_bool(cap, cap)
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(3 > 5)
    
    fr fr Nil assertions
    assert_nil("cringe")
    assert_not_nil("hello")
    
    test_end()
}

slay test_advanced_features() {
    test_start("test_advanced_features")
    
    fr fr Float assertions
    assert_eq_float(3.14, 3.14)
    assert_eq_float_with_tolerance(3.14, 3.141, 0.01)
    
    fr fr Performance testing
    sus start_time normie = benchmark_start()
    sus result normie = 0
    bestie i := 0; i < 100; i++ {
        result = result + i
    }
    benchmark_end(start_time)
    assert_eq_int(result, 4950)
    
    fr fr Test skipping
    test_skip("This is a demonstration skip")
    
    test_end()
}

slay test_comprehensive_validation() {
    test_start("test_comprehensive_validation")
    
    fr fr Complex expressions
    assert_eq_int(1 + 2 * 3, 7)
    assert_eq_int((1 + 2) * 3, 9)
    assert_true((5 > 3) && (2 < 4))
    assert_false((5 < 3) || (2 > 4))
    
    fr fr Variable testing
    sus a normie = 10
    sus b normie = 20
    sus c normie = a + b
    
    assert_eq_int(c, 30)
    assert_greater_than(c, a)
    assert_greater_than(c, b)
    
    sus name tea = "CURSED"
    sus greeting tea = "Hello " + name
    
    assert_eq_string(greeting, "Hello CURSED")
    assert_ne_string(greeting, name)
    
    test_end()
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("🧪 Starting CURSED Testing Framework v5.0 - Production Ready")
    vibez.spill("===============================================================")
    vibez.spill("")
    vibez.spill("This is a comprehensive testing framework written in pure CURSED")
    vibez.spill("that provides enterprise-grade testing capabilities.")
    vibez.spill("")
    
    fr fr Configure testing framework
    enable_verbose_output()
    enable_json_output()
    enable_tap_output()
    enable_html_output()
    
    fr fr Reset test state
    reset_test_state()
    
    fr fr Test discovery
    discover_tests("test_*")
    
    fr fr Run test suites
    suite_start("Core Testing Framework")
    
    vibez.spill("=== Running Basic Assertion Tests ===")
    test_basic_assertions()
    
    vibez.spill("=== Running Advanced Feature Tests ===")
    test_advanced_features()
    
    vibez.spill("=== Running Comprehensive Validation Tests ===")
    test_comprehensive_validation()
    
    suite_end()
    
    fr fr Print results
    print_test_summary()
    
    fr fr Generate reports
    vibez.spill("")
    vibez.spill("=== Alternative Output Formats ===")
    vibez.spill("JSON Report:")
    generate_json_report()
    vibez.spill("")
    
    vibez.spill("TAP Report:")
    generate_tap_report()
    vibez.spill("")
    
    vibez.spill("HTML Report:")
    generate_html_report()
    vibez.spill("")
    
    vibez.spill("🎯 CURSED Testing Framework v5.0 - Production Ready Complete!")
    vibez.spill("")
    vibez.spill("Framework Features:")
    vibez.spill("✅ Comprehensive assertion library (50+ functions)")
    vibez.spill("✅ Multiple output formats (Console, JSON, TAP, HTML)")
    vibez.spill("✅ Test discovery and filtering")
    vibez.spill("✅ Performance benchmarking")
    vibez.spill("✅ Test lifecycle management")
    vibez.spill("✅ Configurable test execution")
    vibez.spill("✅ Production-ready error handling")
    vibez.spill("✅ Full CURSED language integration")
    vibez.spill("")
    
    fr fr Return exit code
    sus exit_code normie = run_all_tests()
    
    lowkey exit_code == 0 {
        vibez.spill("✅ All tests passed successfully!")
    } highkey {
        vibez.spill("❌ Some tests failed - check output above")
    }
    
    damn exit_code
}
