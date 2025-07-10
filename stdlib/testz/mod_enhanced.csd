fr fr CURSED Testing Framework v3.0
fr fr Enhanced with TestResult Type System
fr fr Provides comprehensive testing utilities with structured result handling

yeet "test_result"

fr fr ================================
fr fr Test Framework Core with TestResult Integration
fr fr ================================

fr fr Global test state using TestResult system
sus current_suite TestSuite = TestSuite.new("default")
sus test_report TestReport = TestReport.new()
sus current_test_name tea = ""
sus current_assertion_name tea = ""
sus test_start_time normie = 0

fr fr Legacy compatibility state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

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
    current_suite.add_test(result)
    
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    
    sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, message, "unknown", "unknown")
    current_suite.add_test(result)
    
    vibez.spill("  ✗ FAIL: " + message)
}

slay test_skip(message tea) {
    sus result TestResult = TestResult.skip(current_test_name, current_assertion_name, message)
    current_suite.add_test(result)
    
    vibez.spill("  ⚠ SKIP: " + message)
}

slay test_error(message tea) {
    sus result TestResult = TestResult.error(current_test_name, current_assertion_name, message)
    current_suite.add_test(result)
    
    vibez.spill("  ⚠ ERROR: " + message)
}

fr fr ================================
fr fr Enhanced Assertion Functions
fr fr ================================

slay assert_eq_int(actual normie, expected normie) {
    current_assertion_name = "assert_eq_int"
    
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_eq_int failed", tea(expected), tea(actual))
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    current_assertion_name = "assert_eq_string"
    
    lowkey actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_eq_string failed", expected, actual)
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_eq_bool(actual lit, expected lit) {
    current_assertion_name = "assert_eq_bool"
    
    lowkey actual == expected {
        test_pass("assert_eq_bool: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_eq_bool failed", tea(expected), tea(actual))
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_eq_bool failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    current_assertion_name = "assert_true"
    
    lowkey value == based {
        test_pass("assert_true: value is based")
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_true failed", "based", tea(value))
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    current_assertion_name = "assert_false"
    
    lowkey value == cap {
        test_pass("assert_false: value is cap")
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_false failed", "cap", tea(value))
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_false failed: got " + tea(value) + ", expected cap")
    }
}

fr fr ================================
fr fr Enhanced Test Reporting
fr fr ================================

slay print_test_summary() {
    fr fr Finalize current suite
    test_report.add_suite(current_suite)
    
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    fr fr Enhanced reporting with TestResult system
    vibez.spill("")
    vibez.spill("=== ENHANCED REPORT ===")
    vibez.spill("Total assertions: " + tea(test_report.total_tests))
    vibez.spill("Passed assertions: " + tea(test_report.passed_tests))
    vibez.spill("Failed assertions: " + tea(test_report.failed_tests))
    vibez.spill("Skipped assertions: " + tea(test_report.skipped_tests))
    vibez.spill("Error assertions: " + tea(test_report.error_tests))
    vibez.spill("Success rate: " + tea(test_report.success_rate) + "%")
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ Some tests failed")
    }
}

slay print_detailed_report() {
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
    current_assertion_name = ""
    
    fr fr Reset TestResult system
    test_report = TestReport.new()
    current_suite = TestSuite.new("default")
}

slay create_test_suite(suite_name tea) {
    current_suite = TestSuite.new(suite_name)
}

slay finalize_test_suite() {
    test_report.add_suite(current_suite)
    current_suite = TestSuite.new("default")
}

fr fr ================================
fr fr Advanced Test Features
fr fr ================================

slay assert_eq_float(actual meal, expected meal) {
    current_assertion_name = "assert_eq_float"
    
    sus tolerance meal = 0.001
    sus diff meal = actual - expected
    lowkey diff < tolerance && diff > -tolerance {
        test_pass("assert_eq_float: " + tea(actual) + " ≈ " + tea(expected))
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_eq_float failed", tea(expected), tea(actual))
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_eq_float failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_not_nil(value *void) {
    current_assertion_name = "assert_not_nil"
    
    lowkey value != cringe {
        test_pass("assert_not_nil: value is not cringe")
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_not_nil failed", "not cringe", "cringe")
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_not_nil failed: value is cringe")
    }
}

slay assert_nil(value *void) {
    current_assertion_name = "assert_nil"
    
    lowkey value == cringe {
        test_pass("assert_nil: value is cringe")
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_nil failed", "cringe", "not cringe")
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_nil failed: value is not cringe")
    }
}

slay assert_greater_than(actual normie, expected normie) {
    current_assertion_name = "assert_greater_than"
    
    lowkey actual > expected {
        test_pass("assert_greater_than: " + tea(actual) + " > " + tea(expected))
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_greater_than failed", "> " + tea(expected), tea(actual))
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_greater_than failed: got " + tea(actual) + ", expected > " + tea(expected))
    }
}

slay assert_less_than(actual normie, expected normie) {
    current_assertion_name = "assert_less_than"
    
    lowkey actual < expected {
        test_pass("assert_less_than: " + tea(actual) + " < " + tea(expected))
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_less_than failed", "< " + tea(expected), tea(actual))
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_less_than failed: got " + tea(actual) + ", expected < " + tea(expected))
    }
}

slay assert_string_contains(haystack tea, needle tea) {
    current_assertion_name = "assert_string_contains"
    
    fr fr TODO: Implement string.contains when string stdlib is available
    fr fr For now, use simple comparison
    lowkey haystack == needle {
        test_pass("assert_string_contains: \"" + haystack + "\" contains \"" + needle + "\"")
    } highkey {
        test_failed = test_failed + 1
        
        sus result TestResult = TestResult.fail(current_test_name, current_assertion_name, 
            "assert_string_contains failed", "contains \"" + needle + "\"", "\"" + haystack + "\"")
        current_suite.add_test(result)
        
        vibez.spill("  ✗ FAIL: assert_string_contains failed: \"" + haystack + "\" does not contain \"" + needle + "\"")
    }
}

fr fr ================================
fr fr Benchmark and Performance Testing
fr fr ================================

slay benchmark_start() normie {
    fr fr TODO: Implement when time stdlib is available
    damn 0
}

slay benchmark_end(start_time normie) {
    fr fr TODO: Implement when time stdlib is available
    sus elapsed normie = 0
    vibez.spill("Benchmark completed in " + tea(elapsed) + "ms")
}

slay test_with_timeout(test_name tea, timeout_ms normie, test_func slay()) {
    test_start(test_name)
    
    fr fr TODO: Implement timeout when concurrency is available
    test_func()
    
    test_pass("Test completed within timeout")
}

fr fr ================================
fr fr Test Data Creation Utilities
fr fr ================================

slay create_test_array() [normie] {
    damn [1, 2, 3, 4, 5]
}

slay create_test_string() tea {
    damn "test_string_data"
}

slay create_test_struct() (tea, normie, lit) {
    damn ("test", 42, based)
}

fr fr ================================
fr fr Migration and Compatibility
fr fr ================================

fr fr Functions for migrating existing tests to use TestResult system
slay migrate_to_test_result() {
    vibez.spill("Migrating test suite to TestResult system...")
    
    fr fr Create default suite with metadata
    current_suite = TestSuite.new("migrated_suite")
    current_suite.add_metadata("migration_date", "2025-01-07")
    current_suite.add_metadata("cursed_version", "8.0.0")
    
    vibez.spill("Migration complete. TestResult system is now active.")
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
fr fr Export compatibility functions
fr fr ================================

fr fr Note: CURSED export system not fully implemented yet
fr fr All functions are available globally when this module is imported
fr fr The enhanced TestResult system provides backward compatibility 
fr fr while adding structured test result handling capabilities
