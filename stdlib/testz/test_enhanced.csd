fr fr ================================
fr fr CURSED Testing Framework v5.0 - Enhanced Tests
fr fr Comprehensive test suite for the enhanced testing framework
fr fr Tests all testing primitives and advanced features
fr fr ================================

yeet "testz"

fr fr ================================
fr fr Test Suite: Basic Framework Functionality
fr fr ================================

slay test_framework_initialization() {
    test_start("test_framework_initialization")
    
    fr fr Test that the framework initializes correctly
    assert_eq_int(test_count, 1)
    assert_eq_string(current_test_name, "test_framework_initialization")
    assert_eq_string(current_suite_name, "default")
    
    test_end()
}

slay test_configuration_system() {
    test_start("test_configuration_system")
    
    fr fr Test configuration functions
    enable_verbose_output()
    enable_fail_fast()
    enable_json_output()
    enable_tap_output()
    enable_html_output()
    enable_xml_output()
    
    fr fr Test timeout and max failures
    set_timeout(10000)
    set_max_failures(50)
    
    fr fr Test that configuration doesn't break basic functionality
    assert_eq_int(42, 42)
    assert_eq_string("test", "test")
    assert_true(based)
    assert_false(cap)
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Basic Assertions
fr fr ================================

slay test_integer_assertions() {
    test_start("test_integer_assertions")
    
    fr fr Test basic integer equality
    assert_eq_int(42, 42)
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_int(1 + 1, 2)
    assert_eq_int(5 * 2, 10)
    assert_eq_int(10 / 2, 5)
    
    fr fr Test integer inequality
    assert_ne_int(42, 43)
    assert_ne_int(1, 2)
    assert_ne_int(0, 1)
    assert_ne_int(-1, 1)
    
    fr fr Test integer comparisons
    assert_greater_than(5, 3)
    assert_greater_than(100, 42)
    assert_greater_than(1, 0)
    assert_greater_than(0, -1)
    
    assert_less_than(3, 5)
    assert_less_than(42, 100)
    assert_less_than(0, 1)
    assert_less_than(-1, 0)
    
    fr fr Test range assertions
    assert_in_range(5, 1, 10)
    assert_in_range(42, 40, 50)
    assert_in_range(0, -5, 5)
    assert_in_range(-3, -5, 5)
    
    test_end()
}

slay test_string_assertions() {
    test_start("test_string_assertions")
    
    fr fr Test basic string equality
    assert_eq_string("hello", "hello")
    assert_eq_string("CURSED", "CURSED")
    assert_eq_string("", "")
    assert_eq_string("123", "123")
    assert_eq_string("hello world", "hello world")
    
    fr fr Test string inequality
    assert_ne_string("hello", "world")
    assert_ne_string("CURSED", "cursed")
    assert_ne_string("", " ")
    assert_ne_string("123", "456")
    
    fr fr Test string contains
    assert_string_contains("hello world", "world")
    assert_string_contains("CURSED programming", "CURSED")
    assert_string_contains("hello world", "hello")
    assert_string_contains("testing framework", "test")
    assert_string_contains("", "")
    
    fr fr Test string starts with
    assert_string_starts_with("hello world", "hello")
    assert_string_starts_with("CURSED", "CURSED")
    assert_string_starts_with("testing", "test")
    assert_string_starts_with("", "")
    
    fr fr Test string ends with
    assert_string_ends_with("hello world", "world")
    assert_string_ends_with("CURSED", "CURSED")
    assert_string_ends_with("testing", "ing")
    assert_string_ends_with("", "")
    
    test_end()
}

slay test_boolean_assertions() {
    test_start("test_boolean_assertions")
    
    fr fr Test basic boolean values
    assert_true(based)
    assert_false(cap)
    
    fr fr Test boolean expressions
    assert_true(5 > 3)
    assert_false(3 > 5)
    assert_true(2 == 2)
    assert_false(2 == 3)
    assert_true(1 != 2)
    assert_false(1 != 1)
    
    fr fr Test boolean equality
    assert_eq_bool(based, based)
    assert_eq_bool(cap, cap)
    assert_eq_bool(5 > 3, based)
    assert_eq_bool(3 > 5, cap)
    assert_eq_bool(2 == 2, based)
    assert_eq_bool(2 == 3, cap)
    
    test_end()
}

slay test_float_assertions() {
    test_start("test_float_assertions")
    
    fr fr Test basic float equality
    assert_eq_float(3.14159, 3.14159)
    assert_eq_float(1.0, 1.0)
    assert_eq_float(0.0, 0.0)
    assert_eq_float(-1.5, -1.5)
    
    fr fr Test float equality with tolerance
    assert_eq_float_with_tolerance(3.14, 3.141, 0.01)
    assert_eq_float_with_tolerance(1.001, 1.002, 0.01)
    assert_eq_float_with_tolerance(0.0, 0.001, 0.01)
    assert_eq_float_with_tolerance(-1.5, -1.49, 0.1)
    
    test_end()
}

slay test_nil_assertions() {
    test_start("test_nil_assertions")
    
    fr fr Test nil values
    assert_nil("cringe")
    assert_not_nil("hello")
    assert_not_nil("42")
    assert_not_nil("")
    assert_not_nil("not cringe")
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Performance Testing
fr fr ================================

slay test_performance_testing() {
    test_start("test_performance_testing")
    
    fr fr Test benchmark timing
    sus start_time normie = benchmark_start()
    
    fr fr Simulate some work
    sus result normie = 0
    bestie i := 0; i < 1000; i++ {
        result = result + i
    }
    
    benchmark_end(start_time)
    
    fr fr Test calculation result
    assert_eq_int(result, 499500)  fr fr Sum of 0 to 999
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Suite Management
fr fr ================================

slay test_suite_management() {
    suite_start("Test Suite Management")
    
    test_start("test_suite_functionality")
    
    fr fr Test suite-related functionality
    assert_eq_string(current_suite_name, "Test Suite Management")
    assert_greater_than(test_count, 0)
    
    test_end()
    
    suite_end()
}

fr fr ================================
fr fr Test Suite: Report Generation
fr fr ================================

slay test_report_generation() {
    test_start("test_report_generation")
    
    fr fr Test JSON report generation
    sus json_report tea = generate_json_report()
    assert_string_contains(json_report, "\"framework\": \"CURSED Testing Framework v5.0\"")
    assert_string_contains(json_report, "\"total_tests\":")
    
    fr fr Test TAP report generation
    sus tap_report tea = generate_tap_report()
    assert_string_starts_with(tap_report, "TAP version 13")
    assert_string_contains(tap_report, "1..")
    
    fr fr Test XML report generation
    sus xml_report tea = generate_xml_report()
    assert_string_starts_with(xml_report, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")
    assert_string_contains(xml_report, "<testsuites>")
    
    fr fr Test HTML report generation
    sus html_report tea = generate_html_report()
    assert_string_starts_with(html_report, "<!DOCTYPE html>")
    assert_string_contains(html_report, "<title>CURSED Test Results</title>")
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Edge Cases
fr fr ================================

slay test_edge_cases() {
    test_start("test_edge_cases")
    
    fr fr Test boundary conditions
    assert_eq_int(0, 0)
    assert_eq_int(-1, -1)
    assert_eq_int(2147483647, 2147483647)  fr fr Max 32-bit int
    assert_eq_int(-2147483648, -2147483648)  fr fr Min 32-bit int
    
    fr fr Test empty strings
    assert_eq_string("", "")
    assert_ne_string("", " ")
    assert_string_contains("", "")
    assert_string_starts_with("", "")
    assert_string_ends_with("", "")
    
    fr fr Test floating point edge cases
    assert_eq_float(0.0, 0.0)
    assert_eq_float(1.0, 1.0)
    assert_eq_float(-1.0, -1.0)
    
    fr fr Test boolean edge cases
    assert_true(based)
    assert_false(cap)
    
    test_end()
}

fr fr ================================
fr fr Test Suite: State Management
fr fr ================================

slay test_state_management() {
    test_start("test_state_management")
    
    fr fr Test state tracking
    sus initial_count normie = get_test_count()
    sus initial_passed normie = get_passed_count()
    
    fr fr These should increment counters
    assert_eq_int(42, 42)  fr fr Should increment passed
    assert_eq_string("test", "test")  fr fr Should increment passed
    
    fr fr Verify state changes
    assert_greater_than(get_passed_count(), initial_passed)
    assert_greater_than(get_test_count(), initial_count)
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Skip Testing
fr fr ================================

slay test_skipping() {
    test_start("test_skipping")
    
    fr fr Test skipping functionality
    test_skip("This test is intentionally skipped for demonstration")
    
    fr fr This should still run
    assert_eq_int(1, 1)
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Configuration Testing
fr fr ================================

slay test_configuration_settings() {
    test_start("test_configuration_settings")
    
    fr fr Test configuration functions
    enable_verbose_output()
    enable_fail_fast()
    enable_json_output()
    enable_tap_output()
    enable_html_output()
    enable_xml_output()
    
    fr fr Test configuration doesn't break functionality
    assert_eq_int(123, 123)
    assert_eq_string("config", "config")
    assert_true(based)
    assert_false(cap)
    
    fr fr Test timeout and max failures
    set_timeout(15000)
    set_max_failures(25)
    
    fr fr Verify configuration changes don't break basic tests
    assert_eq_int(456, 456)
    assert_ne_int(123, 456)
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Filter Testing
fr fr ================================

slay test_filtering() {
    test_start("test_filtering")
    
    fr fr Test pattern matching
    assert_true(should_run_test("test_example", "test_*"))
    assert_true(should_run_test("my_test", "*test*"))
    assert_true(should_run_test("test_basic", "test_*"))
    assert_true(should_run_test("advanced_test", "*test*"))
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Statistics Collection
fr fr ================================

slay test_statistics_collection() {
    test_start("test_statistics_collection")
    
    fr fr Test statistics collection
    sus total_tests normie = get_test_count()
    sus passed_tests normie = get_passed_count()
    sus failed_tests normie = get_failed_count()
    sus skipped_tests normie = get_skipped_count()
    sus error_tests normie = get_error_count()
    
    fr fr Verify statistics are reasonable
    assert_greater_than(total_tests, 0)
    assert_greater_than(passed_tests, 0)
    
    fr fr Test that all counts are non-negative
    assert_in_range(total_tests, 0, 10000)
    assert_in_range(passed_tests, 0, 10000)
    assert_in_range(failed_tests, 0, 10000)
    assert_in_range(skipped_tests, 0, 10000)
    assert_in_range(error_tests, 0, 10000)
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Comprehensive Validation
fr fr ================================

slay test_comprehensive_validation() {
    test_start("test_comprehensive_validation")
    
    fr fr Test all types of assertions in one test
    assert_eq_int(100, 100)
    assert_ne_int(100, 200)
    assert_greater_than(200, 100)
    assert_less_than(100, 200)
    assert_in_range(150, 100, 200)
    
    assert_eq_string("validation", "validation")
    assert_ne_string("validation", "testing")
    assert_string_contains("comprehensive validation", "validation")
    assert_string_starts_with("comprehensive validation", "comprehensive")
    assert_string_ends_with("comprehensive validation", "validation")
    
    assert_eq_bool(based, based)
    assert_eq_bool(cap, cap)
    assert_true(based)
    assert_false(cap)
    
    assert_eq_float(3.14, 3.14)
    assert_eq_float_with_tolerance(3.14, 3.141, 0.01)
    
    assert_not_nil("not nil")
    assert_nil("cringe")
    
    test_end()
}

fr fr ================================
fr fr Test Suite: Error Handling
fr fr ================================

slay test_error_handling() {
    test_start("test_error_handling")
    
    fr fr Test error reporting
    test_error("This is a test error for demonstration")
    
    fr fr Test that errors don't stop execution
    assert_eq_int(1, 1)
    assert_eq_string("after error", "after error")
    
    test_end()
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("🧪 Starting CURSED Testing Framework v5.0 Enhanced Tests...")
    vibez.spill("")
    
    fr fr Configure testing framework
    enable_verbose_output()
    enable_json_output()
    enable_tap_output()
    enable_xml_output()
    enable_html_output()
    
    fr fr Reset test state for clean run
    reset_test_state()
    
    fr fr Run all test suites
    vibez.spill("=== Running Framework Tests ===")
    test_framework_initialization()
    test_configuration_system()
    
    vibez.spill("=== Running Basic Assertion Tests ===")
    test_integer_assertions()
    test_string_assertions()
    test_boolean_assertions()
    test_float_assertions()
    test_nil_assertions()
    
    vibez.spill("=== Running Advanced Feature Tests ===")
    test_performance_testing()
    test_suite_management()
    test_report_generation()
    
    vibez.spill("=== Running Edge Case Tests ===")
    test_edge_cases()
    
    vibez.spill("=== Running State Management Tests ===")
    test_state_management()
    test_skipping()
    
    vibez.spill("=== Running Configuration Tests ===")
    test_configuration_settings()
    test_filtering()
    
    vibez.spill("=== Running Statistics Tests ===")
    test_statistics_collection()
    
    vibez.spill("=== Running Comprehensive Validation Tests ===")
    test_comprehensive_validation()
    
    vibez.spill("=== Running Error Handling Tests ===")
    test_error_handling()
    
    fr fr Print comprehensive test summary
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("🎯 CURSED Testing Framework v5.0 Enhanced Tests Complete!")
    vibez.spill("")
    
    fr fr Test all output formats
    vibez.spill("=== Alternative Output Formats ===")
    vibez.spill("JSON Report:")
    generate_json_report()
    vibez.spill("")
    
    vibez.spill("TAP Report:")
    generate_tap_report()
    vibez.spill("")
    
    vibez.spill("XML Report:")
    generate_xml_report()
    vibez.spill("")
    
    vibez.spill("HTML Report:")
    generate_html_report()
    vibez.spill("")
    
    fr fr Return appropriate exit code
    lowkey get_failed_count() > 0 || get_error_count() > 0 {
        vibez.spill("❌ Some tests failed - returning exit code 1")
        damn 1
    } highkey {
        vibez.spill("✅ All tests passed - returning exit code 0")
        damn 0
    }
}
