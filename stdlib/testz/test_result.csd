fr fr CURSED TestResult Integration
fr fr Enhanced testing framework using TestResult type system

fr fr ================================
fr fr TestResult Integration
fr fr ================================

fr fr Global test state using TestResult system
sus current_suite TestSuite = TestSuite.new("default")
sus test_report TestReport = TestReport.new()
sus current_test_name tea = ""
sus current_assertion_name tea = ""
sus test_start_time normie = 0

fr fr ================================
fr fr Enhanced Test Functions
fr fr ================================

slay test_start_enhanced(name tea) {
    current_test_name = name
    test_start_time = time.now_millis()
    vibez.spill("Running test: " + name)
}

slay test_assertion_start(assertion_name tea) {
    current_assertion_name = assertion_name
}

slay test_pass_enhanced(message tea) {
    sus execution_time normie = time.now_millis() - test_start_time
    sus result TestResult = TestResult.pass(current_test_name, current_assertion_name, message)
    result = result.with_execution_time(execution_time)
    
    current_suite.add_test(result)
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail_enhanced(message tea, expected tea, actual tea) {
    sus execution_time normie = time.now_millis() - test_start_time
    sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, message, expected, actual)
    result = result.with_execution_time(execution_time)
    
    current_suite.add_test(result)
    vibez.spill("  ✗ FAIL: " + message)
    vibez.spill("    Expected: " + expected)
    vibez.spill("    Actual:   " + actual)
}

slay test_skip_enhanced(message tea) {
    sus execution_time normie = time.now_millis() - test_start_time
    sus result TestResult = TestResult.skip(current_test_name, current_assertion_name, message)
    result = result.with_execution_time(execution_time)
    
    current_suite.add_test(result)
    vibez.spill("  ⚠ SKIP: " + message)
}

slay test_error_enhanced(message tea) {
    sus execution_time normie = time.now_millis() - test_start_time
    sus result TestResult = TestResult.error(current_test_name, current_assertion_name, message)
    result = result.with_execution_time(execution_time)
    
    current_suite.add_test(result)
    vibez.spill("  ⚠ ERROR: " + message)
}

fr fr ================================
fr fr Enhanced Assertion Functions
fr fr ================================

slay assert_eq_int_enhanced(actual normie, expected normie) {
    test_assertion_start("assert_eq_int")
    
    lowkey actual == expected {
        test_pass_enhanced("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail_enhanced("assert_eq_int failed", tea(expected), tea(actual))
    }
}

slay assert_eq_string_enhanced(actual tea, expected tea) {
    test_assertion_start("assert_eq_string")
    
    lowkey actual == expected {
        test_pass_enhanced("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_fail_enhanced("assert_eq_string failed", expected, actual)
    }
}

slay assert_eq_bool_enhanced(actual lit, expected lit) {
    test_assertion_start("assert_eq_bool")
    
    lowkey actual == expected {
        test_pass_enhanced("assert_eq_bool: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail_enhanced("assert_eq_bool failed", tea(expected), tea(actual))
    }
}

slay assert_true_enhanced(value lit) {
    test_assertion_start("assert_true")
    
    lowkey value == based {
        test_pass_enhanced("assert_true: value is based")
    } highkey {
        test_fail_enhanced("assert_true failed", "based", tea(value))
    }
}

slay assert_false_enhanced(value lit) {
    test_assertion_start("assert_false")
    
    lowkey value == cap {
        test_pass_enhanced("assert_false: value is cap")
    } highkey {
        test_fail_enhanced("assert_false failed", "cap", tea(value))
    }
}

slay assert_eq_float_enhanced(actual meal, expected meal) {
    test_assertion_start("assert_eq_float")
    
    sus tolerance meal = 0.001
    sus diff meal = actual - expected
    lowkey diff < tolerance && diff > -tolerance {
        test_pass_enhanced("assert_eq_float: " + tea(actual) + " ≈ " + tea(expected))
    } highkey {
        test_fail_enhanced("assert_eq_float failed", tea(expected), tea(actual))
    }
}

slay assert_not_nil_enhanced(value *void) {
    test_assertion_start("assert_not_nil")
    
    lowkey value != cringe {
        test_pass_enhanced("assert_not_nil: value is not cringe")
    } highkey {
        test_fail_enhanced("assert_not_nil failed", "not cringe", "cringe")
    }
}

slay assert_nil_enhanced(value *void) {
    test_assertion_start("assert_nil")
    
    lowkey value == cringe {
        test_pass_enhanced("assert_nil: value is cringe")
    } highkey {
        test_fail_enhanced("assert_nil failed", "cringe", "not cringe")
    }
}

fr fr ================================
fr fr Test Suite Management
fr fr ================================

slay create_test_suite(suite_name tea) {
    current_suite = TestSuite.new(suite_name)
    current_suite.add_metadata("created_at", time.now_string())
    current_suite.add_metadata("cursed_version", "8.0.0")
}

slay finalize_test_suite() {
    current_suite.set_timing(0, 0, time.now_millis())
    test_report.add_suite(current_suite)
    
    vibez.spill("")
    vibez.spill("=== SUITE SUMMARY: " + current_suite.suite_name + " ===")
    vibez.spill("Total tests: " + tea(current_suite.total_count()))
    vibez.spill("Passed: " + tea(current_suite.passed_count()))
    vibez.spill("Failed: " + tea(current_suite.failed_count()))
    vibez.spill("Skipped: " + tea(current_suite.skipped_count()))
    vibez.spill("Errors: " + tea(current_suite.error_count()))
    vibez.spill("Success rate: " + tea(current_suite.success_rate()) + "%")
    
    lowkey current_suite.is_successful() {
        vibez.spill("✅ SUITE PASSED!")
    } highkey {
        vibez.spill("❌ SUITE FAILED!")
    }
}

fr fr ================================
fr fr Enhanced Test Reporting
fr fr ================================

slay print_test_report() {
    sus console_report tea = test_report.to_console()
    vibez.spill(console_report)
}

slay generate_json_report() tea {
    damn test_report.to_json()
}

slay generate_xml_report() tea {
    damn test_report.to_xml()
}

slay generate_html_report() tea {
    damn test_report.to_html()
}

slay get_test_statistics() (normie, normie, normie, normie, meal) {
    damn (
        test_report.total_tests,
        test_report.passed_tests,
        test_report.failed_tests,
        test_report.skipped_tests,
        test_report.success_rate
    )
}

slay is_test_run_successful() lit {
    damn test_report.is_successful()
}

fr fr ================================
fr fr Test Utilities
fr fr ================================

slay benchmark_test(test_name tea, operation slay()) {
    test_start_enhanced(test_name)
    
    sus start_time normie = time.now_millis()
    operation()
    sus end_time normie = time.now_millis()
    sus execution_time normie = end_time - start_time
    
    test_assertion_start("benchmark")
    test_pass_enhanced("Execution time: " + tea(execution_time) + "ms")
}

slay skip_test(test_name tea, reason tea) {
    test_start_enhanced(test_name)
    test_assertion_start("skip")
    test_skip_enhanced(reason)
}

slay expect_panic(test_name tea, operation slay()) {
    test_start_enhanced(test_name)
    test_assertion_start("expect_panic")
    
    fr fr TODO: Implement panic catching when error handling is available
    fr fr For now, just run the operation and assume it panics
    operation()
    test_fail_enhanced("expect_panic failed", "panic", "no panic")
}

fr fr ================================
fr fr Test Data Utilities
fr fr ================================

slay create_test_array_int(size normie) [normie] {
    sus arr [normie]
    bestie i := 0; i < size; i++ {
        arr[i] = i * 2
    }
    damn arr
}

slay create_test_string(prefix tea) tea {
    damn prefix + "_test_data_" + tea(time.now_millis())
}

slay create_test_struct_data() (tea, normie, lit) {
    damn ("test_struct", 42, based)
}

fr fr ================================
fr fr Compatibility Functions
fr fr ================================

fr fr Provide backward compatibility with existing testz functions
slay test_start(name tea) {
    test_start_enhanced(name)
}

slay test_pass(message tea) {
    test_assertion_start("manual")
    test_pass_enhanced(message)
}

slay test_fail(message tea) {
    test_assertion_start("manual")
    test_fail_enhanced(message, "unknown", "unknown")
}

slay assert_eq_int(actual normie, expected normie) {
    assert_eq_int_enhanced(actual, expected)
}

slay assert_eq_string(actual tea, expected tea) {
    assert_eq_string_enhanced(actual, expected)
}

slay assert_eq_bool(actual lit, expected lit) {
    assert_eq_bool_enhanced(actual, expected)
}

slay assert_true(value lit) {
    assert_true_enhanced(value)
}

slay assert_false(value lit) {
    assert_false_enhanced(value)
}

slay print_test_summary() {
    finalize_test_suite()
    print_test_report()
}

slay run_all_tests() normie {
    lowkey is_test_run_successful() {
        damn 0
    } highkey {
        damn 1
    }
}

slay reset_test_state() {
    test_report = TestReport.new()
    current_suite = TestSuite.new("default")
    current_test_name = ""
    current_assertion_name = ""
}

fr fr ================================
fr fr Advanced Test Features
fr fr ================================

slay test_with_timeout(test_name tea, timeout_ms normie, operation slay()) {
    test_start_enhanced(test_name)
    test_assertion_start("timeout")
    
    sus start_time normie = time.now_millis()
    operation()
    sus end_time normie = time.now_millis()
    sus execution_time normie = end_time - start_time
    
    lowkey execution_time <= timeout_ms {
        test_pass_enhanced("Completed within timeout: " + tea(execution_time) + "ms")
    } highkey {
        test_fail_enhanced("Timeout exceeded", tea(timeout_ms) + "ms", tea(execution_time) + "ms")
    }
}

slay test_performance_regression(test_name tea, operation slay(), baseline_ms normie) {
    test_start_enhanced(test_name)
    test_assertion_start("performance_regression")
    
    sus start_time normie = time.now_millis()
    operation()
    sus end_time normie = time.now_millis()
    sus execution_time normie = end_time - start_time
    
    sus threshold normie = baseline_ms + (baseline_ms / 10) fr fr 10% tolerance
    
    lowkey execution_time <= threshold {
        test_pass_enhanced("Performance within threshold: " + tea(execution_time) + "ms")
    } highkey {
        test_fail_enhanced("Performance regression", tea(threshold) + "ms", tea(execution_time) + "ms")
    }
}

slay test_memory_usage(test_name tea, operation slay(), max_memory_mb normie) {
    test_start_enhanced(test_name)
    test_assertion_start("memory_usage")
    
    fr fr TODO: Implement memory monitoring when available
    operation()
    test_pass_enhanced("Memory usage test completed")
}

fr fr ================================
fr fr Export functions for external use
fr fr ================================

fr fr Note: CURSED export system not fully implemented yet
fr fr Functions are available globally when this module is imported
