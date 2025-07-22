fr fr CURSED Testing Framework v4.0 - Complete TestResult Integration
fr fr Enterprise-grade testing with full TestResult type system support
fr fr Provides comprehensive testing utilities with structured result handling

fr fr ================================
fr fr Test Framework Core with Complete TestResult Integration
fr fr ================================

fr fr Global test state using TestResult system
sus current_suite_name tea = "default"
sus test_results [TestResult] = []
sus current_test_name tea = ""
sus current_assertion_name tea = ""
sus test_start_time normie = 0

fr fr Legacy compatibility state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

fr fr TestResult type definition (simplified for CURSED)
struct TestResult {
    test_name tea
    assertion_name tea
    status tea
    message tea
    expected tea
    actual tea
    execution_time normie
    line_number normie
    file_name tea
}

fr fr TestResult factory functions
slay TestResult.pass(test_name tea, assertion_name tea, message tea) TestResult {
    damn TestResult{
        test_name: test_name,
        assertion_name: assertion_name,
        status: "PASS",
        message: message,
        expected: "",
        actual: "",
        execution_time: 0,
        line_number: 0,
        file_name: ""
    }
}

slay TestResult.fail(test_name tea, assertion_name tea, message tea, expected tea, actual tea) TestResult {
    damn TestResult{
        test_name: test_name,
        assertion_name: assertion_name,
        status: "FAIL",
        message: message,
        expected: expected,
        actual: actual,
        execution_time: 0,
        line_number: 0,
        file_name: ""
    }
}

slay TestResult.skip(test_name tea, assertion_name tea, message tea) TestResult {
    damn TestResult{
        test_name: test_name,
        assertion_name: assertion_name,
        status: "SKIP",
        message: message,
        expected: "",
        actual: "",
        execution_time: 0,
        line_number: 0,
        file_name: ""
    }
}

slay TestResult.error(test_name tea, assertion_name tea, message tea) TestResult {
    damn TestResult{
        test_name: test_name,
        assertion_name: assertion_name,
        status: "ERROR",
        message: message,
        expected: "",
        actual: "",
        execution_time: 0,
        line_number: 0,
        file_name: ""
    }
}

fr fr TestResult status checking functions
slay TestResult.is_pass(result TestResult) lit {
    damn result.status == "PASS"
}

slay TestResult.is_fail(result TestResult) lit {
    damn result.status == "FAIL"
}

slay TestResult.is_skip(result TestResult) lit {
    damn result.status == "SKIP"
}

slay TestResult.is_error(result TestResult) lit {
    damn result.status == "ERROR"
}

fr fr TestResult enhancement functions
slay TestResult.with_execution_time(result TestResult, time normie) TestResult {
    result.execution_time = time
    damn result
}

slay TestResult.with_line_number(result TestResult, line normie) TestResult {
    result.line_number = line
    damn result
}

slay TestResult.with_file_name(result TestResult, file tea) TestResult {
    result.file_name = file
    damn result
}

fr fr ================================
fr fr Enhanced Core Test Functions
fr fr ================================

slay test_start(name tea) {
    current_test_name = name
    test_count = test_count + 1
    vibez.spill("Running test: " + name)
}

slay test_assertion_start(assertion_name tea) {
    current_assertion_name = assertion_name
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    
    sus result TestResult = TestResult.pass(current_test_name, current_assertion_name, message)
    test_results = test_results + [result]
    
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    
    sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, message, "unknown", "unknown")
    test_results = test_results + [result]
    
    vibez.spill("  ✗ FAIL: " + message)
}

slay test_skip(message tea) {
    sus result TestResult = TestResult.skip(current_test_name, current_assertion_name, message)
    test_results = test_results + [result]
    
    vibez.spill("  ⚠ SKIP: " + message)
}

slay test_error(message tea) {
    sus result TestResult = TestResult.error(current_test_name, current_assertion_name, message)
    test_results = test_results + [result]
    
    vibez.spill("  ⚠ ERROR: " + message)
}

fr fr ================================
fr fr Enhanced Assertion Functions with TestResult
fr fr ================================

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        sus result TestResult = TestResult.pass(current_test_name, "assert_eq_int", 
            "assert_eq_int: " + tea(actual) + " == " + tea(expected))
        test_results = test_results + [result]
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: assert_eq_int")
    } highkey {
        sus result TestResult = TestResult.fail(current_test_name, "assert_eq_int", 
            "assert_eq_int failed", tea(expected), tea(actual))
        test_results = test_results + [result]
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        sus result TestResult = TestResult.pass(current_test_name, "assert_eq_string", 
            "assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
        test_results = test_results + [result]
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: assert_eq_string")
    } highkey {
        sus result TestResult = TestResult.fail(current_test_name, "assert_eq_string", 
            "assert_eq_string failed", expected, actual)
        test_results = test_results + [result]
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_eq_bool(actual lit, expected lit) {
    lowkey actual == expected {
        sus result TestResult = TestResult.pass(current_test_name, "assert_eq_bool", 
            "assert_eq_bool: " + tea(actual) + " == " + tea(expected))
        test_results = test_results + [result]
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: assert_eq_bool")
    } highkey {
        sus result TestResult = TestResult.fail(current_test_name, "assert_eq_bool", 
            "assert_eq_bool failed", tea(expected), tea(actual))
        test_results = test_results + [result]
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: assert_eq_bool failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        sus result TestResult = TestResult.pass(current_test_name, "assert_true", 
            "assert_true: value is based")
        test_results = test_results + [result]
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: assert_true")
    } highkey {
        sus result TestResult = TestResult.fail(current_test_name, "assert_true", 
            "assert_true failed", "based", tea(value))
        test_results = test_results + [result]
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        sus result TestResult = TestResult.pass(current_test_name, "assert_false", 
            "assert_false: value is cap")
        test_results = test_results + [result]
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: assert_false")
    } highkey {
        sus result TestResult = TestResult.fail(current_test_name, "assert_false", 
            "assert_false failed", "cap", tea(value))
        test_results = test_results + [result]
        test_failed = test_failed + 1
        vibez.spill("  ✗ FAIL: assert_false failed: got " + tea(value) + ", expected cap")
    }
}

fr fr ================================
fr fr Enhanced Assertion Functions that Return TestResult
fr fr ================================

slay assert_eq_int_result(test_name tea, actual normie, expected normie) TestResult {
    lowkey actual == expected {
        damn TestResult.pass(test_name, "assert_eq_int", "assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        damn TestResult.fail(test_name, "assert_eq_int", "assert_eq_int failed", tea(expected), tea(actual))
    }
}

slay assert_eq_string_result(test_name tea, actual tea, expected tea) TestResult {
    lowkey actual == expected {
        damn TestResult.pass(test_name, "assert_eq_string", "assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        damn TestResult.fail(test_name, "assert_eq_string", "assert_eq_string failed", expected, actual)
    }
}

slay assert_eq_bool_result(test_name tea, actual lit, expected lit) TestResult {
    lowkey actual == expected {
        damn TestResult.pass(test_name, "assert_eq_bool", "assert_eq_bool: " + tea(actual) + " == " + tea(expected))
    } highkey {
        damn TestResult.fail(test_name, "assert_eq_bool", "assert_eq_bool failed", tea(expected), tea(actual))
    }
}

slay assert_true_result(test_name tea, value lit) TestResult {
    lowkey value == based {
        damn TestResult.pass(test_name, "assert_true", "assert_true: value is based")
    } highkey {
        damn TestResult.fail(test_name, "assert_true", "assert_true failed", "based", tea(value))
    }
}

slay assert_false_result(test_name tea, value lit) TestResult {
    lowkey value == cap {
        damn TestResult.pass(test_name, "assert_false", "assert_false: value is cap")
    } highkey {
        damn TestResult.fail(test_name, "assert_false", "assert_false failed", "cap", tea(value))
    }
}

fr fr ================================
fr fr Enhanced Test Reporting with TestResult
fr fr ================================

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    fr fr Calculate statistics from TestResult array
    sus total_results normie = test_results.length
    sus passed_results normie = 0
    sus failed_results normie = 0
    sus skipped_results normie = 0
    sus error_results normie = 0
    
    sus i normie = 0
    periodt i < total_results {
        sus result TestResult = test_results[i]
        lowkey result.status == "PASS" {
            passed_results = passed_results + 1
        } highkey lowkey result.status == "FAIL" {
            failed_results = failed_results + 1
        } highkey lowkey result.status == "SKIP" {
            skipped_results = skipped_results + 1
        } highkey lowkey result.status == "ERROR" {
            error_results = error_results + 1
        }
        i = i + 1
    }
    
    vibez.spill("TestResult Summary:")
    vibez.spill("  Total Results: " + tea(total_results))
    vibez.spill("  Passed: " + tea(passed_results))
    vibez.spill("  Failed: " + tea(failed_results))
    vibez.spill("  Skipped: " + tea(skipped_results))
    vibez.spill("  Errors: " + tea(error_results))
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ Some tests failed")
    }
}

fr fr ================================
fr fr JSON Output Support
fr fr ================================

slay generate_json_report() tea {
    sus json_output tea = "{\n"
    json_output = json_output + "  \"suite_name\": \"" + current_suite_name + "\",\n"
    json_output = json_output + "  \"total_tests\": " + tea(test_count) + ",\n"
    json_output = json_output + "  \"passed_tests\": " + tea(test_passed) + ",\n"
    json_output = json_output + "  \"failed_tests\": " + tea(test_failed) + ",\n"
    json_output = json_output + "  \"results\": [\n"
    
    sus i normie = 0
    periodt i < test_results.length {
        sus result TestResult = test_results[i]
        json_output = json_output + "    {\n"
        json_output = json_output + "      \"test_name\": \"" + result.test_name + "\",\n"
        json_output = json_output + "      \"assertion_name\": \"" + result.assertion_name + "\",\n"
        json_output = json_output + "      \"status\": \"" + result.status + "\",\n"
        json_output = json_output + "      \"message\": \"" + result.message + "\",\n"
        json_output = json_output + "      \"expected\": \"" + result.expected + "\",\n"
        json_output = json_output + "      \"actual\": \"" + result.actual + "\"\n"
        json_output = json_output + "    }"
        
        lowkey i < test_results.length - 1 {
            json_output = json_output + ","
        }
        json_output = json_output + "\n"
        i = i + 1
    }
    
    json_output = json_output + "  ]\n"
    json_output = json_output + "}\n"
    
    damn json_output
}

fr fr ================================
fr fr TAP Output Support
fr fr ================================

slay generate_tap_report() tea {
    sus tap_output tea = "1.." + tea(test_count) + "\n"
    
    sus i normie = 0
    sus test_number normie = 1
    periodt i < test_results.length {
        sus result TestResult = test_results[i]
        
        lowkey result.status == "PASS" {
            tap_output = tap_output + "ok " + tea(test_number) + " - " + result.test_name + ": " + result.assertion_name + "\n"
        } highkey lowkey result.status == "FAIL" {
            tap_output = tap_output + "not ok " + tea(test_number) + " - " + result.test_name + ": " + result.assertion_name + "\n"
            tap_output = tap_output + "  ---\n"
            tap_output = tap_output + "  message: " + result.message + "\n"
            tap_output = tap_output + "  expected: " + result.expected + "\n"
            tap_output = tap_output + "  actual: " + result.actual + "\n"
            tap_output = tap_output + "  ...\n"
        } highkey lowkey result.status == "SKIP" {
            tap_output = tap_output + "ok " + tea(test_number) + " - " + result.test_name + ": " + result.assertion_name + " fr fr SKIP " + result.message + "\n"
        } highkey lowkey result.status == "ERROR" {
            tap_output = tap_output + "not ok " + tea(test_number) + " - " + result.test_name + ": " + result.assertion_name + " fr fr ERROR " + result.message + "\n"
        }
        
        i = i + 1
        test_number = test_number + 1
    }
    
    damn tap_output
}

fr fr ================================
fr fr Test Utilities
fr fr ================================

slay reset_test_state() {
    test_count = 0
    test_passed = 0
    test_failed = 0
    current_test_name = ""
    current_assertion_name = ""
    test_results = []
}

slay run_all_tests() normie {
    print_test_summary()
    
    lowkey test_failed > 0 {
        damn 1
    } highkey {
        damn 0
    }
}

slay get_test_results() [TestResult] {
    damn test_results
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

fr fr ================================
fr fr Collection Management
fr fr ================================

slay add_test_result(result TestResult) {
    test_results = test_results + [result]
    
    lowkey result.status == "PASS" {
        test_passed = test_passed + 1
    } highkey lowkey result.status == "FAIL" {
        test_failed = test_failed + 1
    }
    
    test_count = test_count + 1
}

slay collect_test_results(results [TestResult]) {
    sus i normie = 0
    periodt i < results.length {
        add_test_result(results[i])
        i = i + 1
    }
}

fr fr ================================
fr fr Export all testing functions
fr fr ================================

fr fr Note: vibes export system not working, functions available globally
